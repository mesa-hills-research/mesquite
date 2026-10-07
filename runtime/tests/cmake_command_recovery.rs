//! Regressions for CMake oracle bucket 6c68c51b.

use tree_sitter::{InputEdit, Language, ParseOptions, ParseState, Parser, Point};

#[test]
fn malformed_variable_references_recover_as_top_level_errors() {
    let language = Language::from(tree_sitter_cmake::language());

    // Expected shapes and ranges come from the C oracle. In particular, these
    // must not become normal_command nodes with a nested argument-list error.
    for (source, child_count, named_child_count, content_start) in [
        ("set(var \"${\")\n", 7, 3, 11),
        ("message(${var\twith\ttab})\n", 5, 2, 14),
        ("message(${var with space})\n", 5, 2, 14),
    ] {
        // Split punctuation, skipped whitespace, and the trailing newline
        // across callback chunks without changing recovery or progress counts.
        for chunk_size in [source.len(), 1] {
            let mut parser = Parser::new();
            parser.set_language(&language).unwrap();

            // Check both scanner creation and reset after a prior parse left a
            // nonzero token. Both must restore the zero-valued
            // BRACKET_ARGUMENT_OPEN state before recovery enables all symbols.
            for preceding_source in [None, Some("message([=[value]=])"), Some("#[==[comment]==]")] {
                if let Some(preceding_source) = preceding_source {
                    let tree = parser.parse(preceding_source, None).unwrap();
                    assert!(!tree.root_node().has_error());
                }

                // These short recoveries do not reach C's progress checkpoint.
                // Count callbacks as well as checking the resulting error tree:
                // a different recovery path can produce the same node shape.
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
                assert_eq!(progress_calls, 0, "{source:?}");

                let root = tree.root_node();
                assert_eq!(root.kind(), "source_file", "{source:?}");
                assert!(root.has_error());
                assert_eq!(root.child_count(), 1);
                assert_eq!(root.named_child_count(), 1);
                assert_eq!(root.byte_range(), 0..source.len());

                let error = root.child(0).unwrap();
                assert!(error.is_error(), "{source:?}: {}", root.to_sexp());
                assert!(error.is_named());
                assert!(error.is_extra());
                assert!(!error.is_missing());
                assert_eq!(error.byte_range(), 0..source.len());
                assert_eq!(error.start_position(), Point::new(0, 0));
                assert_eq!(error.end_position(), Point::new(1, 0));
                assert_eq!(error.child_count(), child_count);
                assert_eq!(error.named_child_count(), named_child_count);
                assert_eq!(error.child(0).unwrap().kind(), "identifier");

                // No opening bracket exists in the input. C's zero scanner state
                // nevertheless allows this content token during error recovery,
                // absorbing the command's closing punctuation and final newline.
                let content = error.child(child_count - 1).unwrap();
                assert_eq!(content.kind(), "bracket_argument_content");
                assert_eq!(content.byte_range(), content_start..source.len());
                assert_eq!(content.start_position(), Point::new(0, content_start));
                assert_eq!(content.end_position(), Point::new(1, 0));
                assert!(content.is_named());
                assert!(!content.is_extra());
                assert!(!content.is_missing());
                assert!(!content.has_error());
                assert_eq!(content.child_count(), 0);
            }
        }
    }
}

#[test]
fn multiline_command_errors_preserve_recovery_boundaries() {
    let language = Language::from(tree_sitter_cmake::language());

    // CommandError0 and ParenInVarName0 from the CMake syntax tests. These
    // oracle-verified cases cover whitespace before the recovery token and a
    // valid command preceding ERROR, unlike the single-line regressions above.
    for (source, error_index, error_start, child_count, content_start, content_column) in [
        ("message\n  (\"Example Message\")\n", 0, 0, 2, 10, 2),
        (
            "set(\"e(x)\" value)\nmessage(\"-->${e(x)}<--\")\n",
            1,
            18,
            7,
            34,
            16,
        ),
    ] {
        // Split punctuation, skipped whitespace, and the trailing newline
        // across callback chunks without changing recovery or progress counts.
        for chunk_size in [source.len(), 1] {
            let mut parser = Parser::new();
            parser.set_language(&language).unwrap();
            for preceding_source in [None, Some("message([=[value]=])"), Some("#[==[comment]==]")] {
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
                assert_eq!(root.end_position(), Point::new(2, 0));
                assert!(root.has_error());
                assert_eq!(root.child_count(), error_index + 1);
                assert_eq!(root.named_child_count(), error_index + 1);
                if error_index == 1 {
                    let command = root.child(0).unwrap();
                    assert_eq!(command.kind(), "normal_command");
                    assert_eq!(command.byte_range(), 0..17);
                    assert!(!command.has_error());
                }

                let error = root.child(error_index).unwrap();
                assert!(error.is_error(), "{source:?}: {}", root.to_sexp());
                assert!(error.is_named());
                assert!(error.is_extra());
                assert!(!error.is_missing());
                assert_eq!(error.byte_range(), error_start..source.len());
                assert_eq!(error.start_position(), Point::new(error_index, 0));
                assert_eq!(error.end_position(), Point::new(2, 0));
                assert_eq!(error.child_count(), child_count);
                assert_eq!(error.named_child_count(), 2);
                assert_eq!(error.child(0).unwrap().kind(), "identifier");

                // Leading whitespace is skipped, but all remaining punctuation
                // and the final newline belong to the bracket-content token.
                let content = error.child(child_count - 1).unwrap();
                assert_eq!(content.kind(), "bracket_argument_content");
                assert_eq!(content.byte_range(), content_start..source.len());
                assert_eq!(content.start_position(), Point::new(1, content_column));
                assert_eq!(content.end_position(), Point::new(2, 0));
                assert!(content.is_named());
                assert!(!content.is_extra());
                assert!(!content.is_missing());
                assert!(!content.has_error());
                assert_eq!(content.child_count(), 0);
            }
        }
    }
}

#[test]
fn repairing_and_restoring_variable_references_restores_top_level_errors() {
    let language = Language::from(tree_sitter_cmake::language());
    for (source, content_start) in [
        ("set(var \"${\")\n", 11),
        ("message(${var\twith\ttab})\n", 14),
        ("message(${var with space})\n", 14),
    ] {
        let mut parser = Parser::new();
        parser.set_language(&language).unwrap();
        let mut tree = parser.parse(source, None).unwrap();
        let expected_sexp = tree.root_node().to_sexp();
        assert!(tree.root_node().child(0).unwrap().is_error());

        // Unlike a fresh parse or parser reuse without an old tree, incremental
        // repair can reuse earlier reductions. Change the input length too:
        // restoring the error must recreate the recovery token at its original
        // byte/point range, not keep the repaired command or a stale snapshot.
        let start = source.find("${").unwrap() + 2;
        let (end, replacement) = match source.find('}') {
            Some(end) => (end, "value"),
            None => (start, "value}"),
        };
        let mut repaired = source.to_owned();
        repaired.replace_range(start..end, replacement);
        let repair = InputEdit {
            start_byte: start,
            old_end_byte: end,
            new_end_byte: start + replacement.len(),
            start_position: Point::new(0, start),
            old_end_position: Point::new(0, end),
            new_end_position: Point::new(0, start + replacement.len()),
        };
        let undo = InputEdit {
            start_byte: repair.start_byte,
            old_end_byte: repair.new_end_byte,
            new_end_byte: repair.old_end_byte,
            start_position: repair.start_position,
            old_end_position: repair.new_end_position,
            new_end_position: repair.old_end_position,
        };

        for _ in 0..2 {
            tree.edit(&repair);
            tree = parser.parse(&repaired, Some(&tree)).unwrap();
            let root = tree.root_node();
            assert_eq!(root.kind(), "source_file");
            assert!(!root.has_error(), "{repaired:?}: {}", root.to_sexp());
            assert_eq!(root.child(0).unwrap().kind(), "normal_command");
            assert_eq!(root.byte_range(), 0..repaired.len());

            tree.edit(&undo);
            tree = parser.parse(source, Some(&tree)).unwrap();
            let root = tree.root_node();
            assert_eq!(root.to_sexp(), expected_sexp, "{source:?}");
            assert_eq!(root.byte_range(), 0..source.len());
            assert_eq!(root.end_position(), Point::new(1, 0));
            assert_eq!(root.child_count(), 1);

            let error = root.child(0).unwrap();
            assert!(error.is_error());
            assert!(error.is_extra());
            assert_eq!(error.byte_range(), 0..source.len());
            let content = error.child(error.child_count() - 1).unwrap();
            assert_eq!(content.kind(), "bracket_argument_content");
            assert_eq!(content.byte_range(), content_start..source.len());
            assert_eq!(content.start_position(), Point::new(0, content_start));
            assert_eq!(content.end_position(), Point::new(1, 0));
            assert!(!content.is_missing());
            assert!(!content.has_error());
        }
    }
}
