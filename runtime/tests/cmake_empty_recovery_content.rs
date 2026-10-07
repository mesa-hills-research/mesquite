//! Regression for CMake oracle bucket f5e2762e.

use std::ops::Range;
use tree_sitter::{InputEdit, Language, ParseOptions, ParseState, Parser, Point};

fn assert_empty_recovery_content(
    source: &str,
    prefix: &[(&str, Range<usize>, usize)],
    end: Point,
    chunk_size: usize,
) {
    let language = Language::from(tree_sitter_cmake::language());
    let mut parser = Parser::new();
    parser.set_language(&language).unwrap();

    // Check both initial scanner state and reuse after bracket argument/comment
    // tokens. C zero-initializes the scanner and resets both state fields when
    // there is no previous external token. Zero means BRACKET_ARGUMENT_OPEN,
    // so recovery can emit content at EOF without an actual opening bracket.
    for preceding_source in [None, Some("message([[value]])"), Some("#[=[comment]=]")] {
        if let Some(preceding_source) = preceding_source {
            let tree = parser.parse(preceding_source, None).unwrap();
            assert!(!tree.root_node().has_error());
        }

        let mut progress_calls = 0;
        let mut progress = |_: &ParseState| {
            progress_calls += 1;
            false
        };
        let tree = parser
            .parse_with_options(
                &mut |offset, _| {
                    let remaining = source.as_bytes().get(offset..).unwrap_or_default();
                    &remaining[..remaining.len().min(chunk_size)]
                },
                None,
                Some(ParseOptions::new().progress_callback(&mut progress)),
            )
            .unwrap();
        assert_eq!(progress_calls, 0);

        let root = tree.root_node();
        assert_eq!(root.kind(), "source_file");
        assert_eq!(root.byte_range(), 0..source.len());
        assert_eq!(root.start_position(), Point::new(0, 0));
        assert_eq!(root.end_position(), end);
        assert!(root.has_error());
        assert_eq!(root.child_count(), 1);
        assert_eq!(root.named_child_count(), 1);

        let error = root.child(0).unwrap();
        assert_eq!(error.kind(), "ERROR");
        assert!(error.is_error());
        assert!(error.is_extra());
        assert_eq!(error.byte_range(), 0..source.len());
        assert_eq!(error.start_position(), Point::new(0, 0));
        assert_eq!(error.end_position(), end);
        assert_eq!(error.child_count(), prefix.len() + 1);
        assert_eq!(error.named_child_count(), 2);

        for (index, (kind, bytes, child_count)) in prefix.iter().enumerate() {
            let child = error.child(index).unwrap();
            assert_eq!(child.kind(), *kind);
            assert_eq!(child.byte_range(), *bytes);
            assert_eq!(child.start_position(), Point::new(0, bytes.start));
            assert_eq!(child.end_position(), Point::new(0, bytes.end));
            assert_eq!(child.child_count(), *child_count);
        }

        let content = error.child(prefix.len()).unwrap();
        assert_eq!(content.kind(), "bracket_argument_content");
        assert_eq!(content.kind_id(), 37);
        assert_eq!(content.byte_range(), source.len()..source.len());
        assert_eq!(content.start_position(), end);
        assert_eq!(content.end_position(), end);
        assert_eq!(content.child_count(), 0);
        assert_eq!(error.named_child(1), Some(content));
        assert!(content.is_named());
        assert!(!content.is_extra());
        assert!(!content.is_error());
        assert!(!content.has_error());
        // This is a scanner-produced empty token, not a missing-node insertion.
        assert!(!content.is_missing());

        // Zero-width content is still a child, with a parent and predecessor,
        // and cursor traversal must reach it even though its start equals EOF.
        let previous = error.child(prefix.len() - 1).unwrap();
        assert_eq!(content.parent(), Some(error));
        assert_eq!(content.prev_sibling(), Some(previous));
        // C's node next-sibling search skips children ending at or before
        // the target's end. With no intervening whitespace it therefore does
        // not find this empty token; cursor traversal below still must.
        assert_eq!(
            previous.next_sibling(),
            (previous.end_byte() < content.end_byte()).then_some(content)
        );
        assert_eq!(content.prev_named_sibling(), error.named_child(0));
        assert_eq!(content.next_sibling(), None);
        assert_eq!(content.next_named_sibling(), None);

        let mut cursor = error.walk();
        assert!(cursor.goto_last_child());
        assert_eq!(cursor.node(), content);
        assert!(!cursor.goto_next_sibling());
        assert!(cursor.goto_previous_sibling());
        assert_eq!(cursor.node(), previous);
        assert!(cursor.goto_next_sibling());
        assert_eq!(cursor.node(), content);
    }
}

#[test]
fn incomplete_commands_retain_zero_width_content_at_eof() {
    for source in ["a", "message"] {
        for chunk_size in [source.len(), 1] {
            assert_empty_recovery_content(
                source,
                &[("identifier", 0..source.len(), 0)],
                Point::new(0, source.len()),
                chunk_size,
            );
        }
    }
}

#[test]
fn incomplete_if_skips_trailing_whitespace_before_empty_content() {
    // The scanner skips whitespace before emitting its zero-width token. Its
    // byte and point ranges must start at EOF, not just after the opening `(`.
    // Only LF advances the row; CR and tabs contribute one byte per column.
    for (source, end) in [
        ("if(\n", Point::new(1, 0)),
        ("if(\r\n", Point::new(1, 0)),
        ("if(\r", Point::new(0, 4)),
        ("if( \t", Point::new(0, 5)),
        ("if( \t\r\n \t", Point::new(1, 2)),
        ("if(\n\n\t", Point::new(2, 1)),
    ] {
        // Byte-at-a-time reads split CRLF and trailing whitespace across
        // chunks: the empty token must still start at the true EOF position.
        for chunk_size in [source.len(), 1] {
            assert_empty_recovery_content(
                source,
                &[("if", 0..2, 0), ("(", 2..3, 0)],
                end,
                chunk_size,
            );
        }
    }
}

#[test]
fn missing_block_end_retains_header_and_empty_recovery_content() {
    // Unlike an unfinished command, these headers have already reduced to
    // complete command nodes. Recovery must preserve them as siblings of the
    // empty scanner token, rather than insert a missing endblock/endwhile.
    for (source, header, header_children) in [
        ("block()\n", "block_command", 3),
        ("while(a)\n", "while_command", 4),
    ] {
        for chunk_size in [source.len(), 1] {
            assert_empty_recovery_content(
                source,
                &[(header, 0..source.len() - 1, header_children)],
                Point::new(1, 0),
                chunk_size,
            );
        }
    }
}

#[test]
fn incremental_repair_and_undo_restore_empty_recovery_content() {
    let language = Language::from(tree_sitter_cmake::language());
    for (source, suffix, end, repaired_end) in [
        ("a", "()", Point::new(0, 1), Point::new(0, 3)),
        ("message", "()", Point::new(0, 7), Point::new(0, 9)),
        ("if(\n", ")\nendif()", Point::new(1, 0), Point::new(2, 7)),
        (
            "block()\n",
            "endblock()",
            Point::new(1, 0),
            Point::new(1, 10),
        ),
        (
            "while(a)\n",
            "endwhile()",
            Point::new(1, 0),
            Point::new(1, 10),
        ),
    ] {
        let mut parser = Parser::new();
        parser.set_language(&language).unwrap();
        let mut tree = parser.parse(source, None).unwrap();
        let expected_sexp = tree.root_node().to_sexp();
        let repaired_source = format!("{source}{suffix}");

        // The insertion is exactly at the zero-width external token. Repair
        // must discard it; undo must recreate it from the reset scanner state,
        // rather than retain the last scanner token or insert a missing node.
        for _ in 0..2 {
            tree.edit(&InputEdit {
                start_byte: source.len(),
                old_end_byte: source.len(),
                new_end_byte: repaired_source.len(),
                start_position: end,
                old_end_position: end,
                new_end_position: repaired_end,
            });
            tree = parser.parse(&repaired_source, Some(&tree)).unwrap();
            assert!(!tree.root_node().has_error(), "{repaired_source:?}");
            assert_eq!(tree.root_node().end_position(), repaired_end);

            tree.edit(&InputEdit {
                start_byte: source.len(),
                old_end_byte: repaired_source.len(),
                new_end_byte: source.len(),
                start_position: end,
                old_end_position: repaired_end,
                new_end_position: end,
            });
            tree = parser.parse(source, Some(&tree)).unwrap();
            let root = tree.root_node();
            assert_eq!(root.to_sexp(), expected_sexp, "{source:?}");
            assert_eq!(root.byte_range(), 0..source.len());
            assert_eq!(root.end_position(), end);
            let error = root.child(0).unwrap();
            assert!(error.is_error());
            assert!(error.is_extra());
            let content = error.child(error.child_count() - 1).unwrap();
            assert_eq!(content.kind(), "bracket_argument_content");
            assert_eq!(content.kind_id(), 37);
            assert_eq!(content.byte_range(), source.len()..source.len());
            assert_eq!(content.start_position(), end);
            assert_eq!(content.end_position(), end);
            assert!(content.is_named());
            assert!(!content.is_missing());
            assert!(!content.is_extra());
            assert!(!content.has_error());
            assert_eq!(content.child_count(), 0);
        }
    }
}
