//! Regression for CMake oracle bucket 7f8795fb.
//!
//! The reference scanner's zero token is BRACKET_ARGUMENT_OPEN. Recovery can
//! therefore emit bracket content without an opener, including at EOF. An
//! inert initial token instead makes these inputs source_file trees with
//! inserted missing delimiters, rather than the reference's top-level ERROR.

use std::ops::Range;
use ts_port::{InputEdit, Language, ParseOptions, ParseState, Parser, Point};

fn assert_root_error(
    source: &[u8],
    prefix: &[(&str, u16, Range<usize>, usize)],
    end: Point,
    recovery_bytes: Range<usize>,
    expected_progress_calls: usize,
    chunk_size: usize,
) {
    let language = Language::from(ts_port_cmake::language());
    let mut parser = Parser::new();
    parser.set_language(&language).unwrap();

    // Verify initialization as well as empty-state deserialization after a
    // previous parse has scanned bracket arguments or comments at different levels.
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
                    let remaining = source.get(offset..).unwrap_or_default();
                    &remaining[..remaining.len().min(chunk_size)]
                },
                None,
                Some(ParseOptions::new().progress_callback(&mut progress)),
            )
            .unwrap();
        assert_eq!(progress_calls, expected_progress_calls);

        let root = tree.root_node();
        assert_eq!(root.kind(), "ERROR");
        assert_eq!(root.kind_id(), u16::MAX);
        assert!(root.is_error());
        assert!(root.has_error());
        assert!(root.is_named());
        assert!(!root.is_extra());
        assert!(!root.is_missing());
        assert_eq!(root.byte_range(), 0..source.len());
        assert_eq!(root.start_position(), Point::new(0, 0));
        assert_eq!(root.end_position(), end);

        // Recovery must keep the completed command (if present), identifier,
        // opening parenthesis, and argument list as direct ERROR children.
        // Inserting a missing close delimiter instead wraps them in a command.
        assert_eq!(root.child_count(), prefix.len() + 1);
        assert_eq!(root.named_child_count(), prefix.len());
        for (index, (kind, id, bytes, child_count)) in prefix.iter().enumerate() {
            let child = root.child(index).unwrap();
            assert_eq!(child.kind(), *kind);
            assert_eq!(child.kind_id(), *id);
            assert_eq!(child.byte_range(), *bytes);
            assert_eq!(child.start_position(), point_at(source, bytes.start));
            assert_eq!(child.end_position(), point_at(source, bytes.end));
            assert_eq!(child.child_count(), *child_count);
            assert_eq!(child.is_named(), *kind != "(");
            assert!(!child.is_extra());
            assert!(!child.is_error());
            assert!(!child.has_error());
            assert!(!child.is_missing());
        }

        let content = root.child(prefix.len()).unwrap();
        assert_eq!(content.kind(), "bracket_argument_content");
        assert_eq!(content.kind_id(), 37);
        assert_eq!(content.byte_range(), recovery_bytes);
        assert_eq!(
            content.start_position(),
            point_at(source, recovery_bytes.start)
        );
        assert_eq!(content.end_position(), end);
        assert_eq!(content.child_count(), 0);
        assert!(content.is_named());
        assert!(!content.is_extra());
        assert!(!content.is_error());
        assert!(!content.has_error());
        // Even zero-width content at EOF is scanner-produced, not missing.
        assert!(!content.is_missing());
    }
}

fn point_at(source: &[u8], byte: usize) -> Point {
    source[..byte].iter().fold(Point::new(0, 0), |point, &ch| {
        if ch == b'\n' {
            Point::new(point.row + 1, 0)
        } else {
            Point::new(point.row, point.column + 1)
        }
    })
}

fn assert_incremental_repair_and_undo(
    source: &[u8],
    edit_bytes: Range<usize>,
    replacement: &[u8],
    recovery_bytes: Range<usize>,
) {
    let language = Language::from(ts_port_cmake::language());
    let mut parser = Parser::new();
    parser.set_language(&language).unwrap();
    let mut tree = parser.parse(source, None).unwrap();
    let expected_sexp = tree.root_node().to_sexp();

    let mut repaired_source = source.to_vec();
    repaired_source.splice(edit_bytes.clone(), replacement.iter().copied());
    let repair = InputEdit {
        start_byte: edit_bytes.start,
        old_end_byte: edit_bytes.end,
        new_end_byte: edit_bytes.start + replacement.len(),
        start_position: point_at(source, edit_bytes.start),
        old_end_position: point_at(source, edit_bytes.end),
        new_end_position: point_at(&repaired_source, edit_bytes.start + replacement.len()),
    };
    let undo = InputEdit {
        start_byte: repair.start_byte,
        old_end_byte: repair.new_end_byte,
        new_end_byte: repair.old_end_byte,
        start_position: repair.start_position,
        old_end_position: repair.new_end_position,
        new_end_position: repair.old_end_position,
    };

    // Reuse the edited tree, not just the parser. Repair removes the recovery
    // token; undo must recreate it from the reset scanner state, including the
    // zero-width token at EOF in the unterminated calls.
    for _ in 0..2 {
        tree.edit(&repair);
        tree = parser.parse(&repaired_source, Some(&tree)).unwrap();
        assert_eq!(tree.root_node().kind(), "source_file");
        assert!(!tree.root_node().has_error());

        tree.edit(&undo);
        tree = parser.parse(source, Some(&tree)).unwrap();
        let root = tree.root_node();
        assert_eq!(root.kind(), "ERROR");
        assert!(root.is_error());
        assert_eq!(root.to_sexp(), expected_sexp);
        assert_eq!(root.byte_range(), 0..source.len());
        assert_eq!(root.end_position(), point_at(source, source.len()));
        let content = root.child(root.child_count() - 1).unwrap();
        assert_eq!(content.kind(), "bracket_argument_content");
        assert_eq!(content.byte_range(), recovery_bytes);
        assert_eq!(
            content.start_position(),
            point_at(source, recovery_bytes.start)
        );
        assert_eq!(content.end_position(), point_at(source, recovery_bytes.end));
        assert!(!content.is_missing());
        assert!(!content.has_error());
    }
}

#[test]
fn unterminated_call_retains_top_level_error() {
    let source = b"message(\n\n\nmessage(\"Additional message\")\n";
    // A one-byte callback exercises EOF recovery across input chunk boundaries.
    for chunk_size in [source.len(), 1] {
        assert_root_error(
            source,
            &[
                ("identifier", 35, 0..7, 0),
                ("(", 14, 7..8, 0),
                ("argument_list", 63, 8..41, 4),
            ],
            Point::new(4, 0),
            source.len()..source.len(),
            1,
            chunk_size,
        );
    }
    assert_incremental_repair_and_undo(
        source,
        source.len()..source.len(),
        b")",
        source.len()..source.len(),
    );
}

#[test]
fn unterminated_call_after_valid_command_retains_top_level_error() {
    let source = b"set(var \"\\\n\")\nmessage(\n\n\nmessage(\"Additional message\")\n";
    for chunk_size in [source.len(), 1] {
        assert_root_error(
            source,
            &[
                ("normal_command", 84, 0..13, 4),
                ("identifier", 35, 14..21, 0),
                ("(", 14, 21..22, 0),
                ("argument_list", 63, 22..55, 4),
            ],
            Point::new(6, 0),
            source.len()..source.len(),
            1,
            chunk_size,
        );
    }
    assert_incremental_repair_and_undo(
        source,
        source.len()..source.len(),
        b")",
        source.len()..source.len(),
    );
}

#[test]
fn nul_in_argument_is_recovery_content_not_eof() {
    let source = b"LIST(APPEND foo TEST\x000000000000000000000000000 )\nCMAKE_HOST_SYSTEM_INFORMATION(RESULT bar QUERY HOSTNAME)\n";
    // An isolated NUL in its own chunk must not be mistaken for end of input.
    for chunk_size in [source.len(), 1] {
        assert_root_error(
            source,
            &[
                ("identifier", 35, 0..4, 0),
                ("(", 14, 4..5, 0),
                ("argument_list", 63, 5..20, 3),
            ],
            Point::new(2, 0),
            20..source.len(),
            0,
            chunk_size,
        );
    }
    assert_incremental_repair_and_undo(source, 20..21, b" ", 20..source.len());
}

#[test]
fn canceled_unterminated_calls_resume_with_empty_recovery_content() {
    let language = Language::from(ts_port_cmake::language());
    for source in [
        &b"message(\n\n\nmessage(\"Additional message\")\n"[..],
        &b"set(var \"\\\n\")\nmessage(\n\n\nmessage(\"Additional message\")\n"[..],
    ] {
        for chunk_size in [source.len(), 1] {
            let mut parser = Parser::new();
            parser.set_language(&language).unwrap();
            let expected = parser.parse(source, None).unwrap();
            assert!(expected.root_node().is_error());

            for preceding_source in [None, Some("#[==[comment]==]")] {
                if let Some(preceding_source) = preceding_source {
                    let tree = parser.parse(preceding_source, None).unwrap();
                    assert!(!tree.root_node().has_error());
                }

                let mut read = |offset, _| {
                    let remaining = source.get(offset..).unwrap_or_default();
                    &remaining[..remaining.len().min(chunk_size)]
                };
                let mut progress_calls = 0;
                let mut cancel = |_: &ParseState| {
                    progress_calls += 1;
                    true
                };
                assert!(
                    parser
                        .parse_with_options(
                            &mut read,
                            None,
                            Some(ParseOptions::new().progress_callback(&mut cancel)),
                        )
                        .is_none()
                );
                assert_eq!(progress_calls, 1);

                // Do not reset: resume the outstanding parse, retaining its
                // stack and scanner state. It must still produce real empty
                // content at EOF rather than insert a missing close delimiter.
                let tree = parser.parse_with_options(&mut read, None, None).unwrap();
                let root = tree.root_node();
                assert_eq!(root.kind(), "ERROR");
                assert!(root.is_error());
                assert_eq!(root.to_sexp(), expected.root_node().to_sexp());
                assert_eq!(root.byte_range(), 0..source.len());
                assert_eq!(root.end_position(), point_at(source, source.len()));
                let content = root.child(root.child_count() - 1).unwrap();
                assert_eq!(content.kind(), "bracket_argument_content");
                assert_eq!(content.byte_range(), source.len()..source.len());
                assert_eq!(content.start_position(), root.end_position());
                assert_eq!(content.end_position(), root.end_position());
                assert!(!content.is_missing());
                assert!(!content.has_error());
            }
        }
    }
}
