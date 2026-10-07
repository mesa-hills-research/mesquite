//! Parser-level regressions for CMake oracle bucket e0b0bff8.

use ts_port::{InputEdit, Language, ParseOptions, ParseState, Parser, Point, Tree};

fn assert_recovery_range(source: &str, identifier_end: usize, eof: Point, chunk_size: usize) {
    let mut parser = Parser::new();
    parser
        .set_language(&Language::from(ts_port_cmake::language()))
        .unwrap();

    // Cover both fresh scanner creation and parser reuse after scans that leave
    // nonzero token state. C resets both scanner fields before the next parse;
    // token zero permits bracket content during recovery without an opener.
    for preceding_source in [None, Some("message([[value]])"), Some("#[=[comment]=]")] {
        if let Some(preceding_source) = preceding_source {
            let preceding_tree = parser.parse(preceding_source, None).unwrap();
            assert!(!preceding_tree.root_node().has_error());
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

        assert_tree_range(&tree, source, identifier_end, eof);

        // Delete and restore the trailing newline using the old tree. Its
        // content token and enclosing ERROR must track the edited EOF instead
        // of retaining the old range or dropping the recovery-content child.
        let mut tree = tree;
        let without_newline = source.strip_suffix('\n').unwrap();
        let preceding_eof = Point::new(
            eof.row - 1,
            without_newline.rsplit('\n').next().unwrap().len(),
        );
        let mut previous_source = source;
        let mut previous_eof = eof;
        for (next_source, next_eof) in [
            (without_newline, preceding_eof),
            (source, eof),
            (without_newline, preceding_eof),
            (source, eof),
        ] {
            tree.edit(&InputEdit {
                start_byte: without_newline.len(),
                old_end_byte: previous_source.len(),
                new_end_byte: next_source.len(),
                start_position: preceding_eof,
                old_end_position: previous_eof,
                new_end_position: next_eof,
            });
            let mut progress_calls = 0;
            let mut progress = |_: &ParseState| {
                progress_calls += 1;
                false
            };
            tree = parser
                .parse_with_options(
                    &mut |offset, _| {
                        let remaining = next_source.as_bytes().get(offset..).unwrap_or_default();
                        &remaining[..remaining.len().min(chunk_size)]
                    },
                    Some(&tree),
                    Some(ParseOptions::new().progress_callback(&mut progress)),
                )
                .unwrap();
            assert_eq!(progress_calls, 0);
            assert_tree_range(&tree, next_source, identifier_end, next_eof);
            previous_source = next_source;
            previous_eof = next_eof;
        }
    }
}

fn assert_tree_range(tree: &Tree, source: &str, identifier_end: usize, eof: Point) {
    let root = tree.root_node();
    assert_eq!(root.kind(), "source_file");
    assert_eq!(root.kind_id(), 43);
    assert_eq!(root.byte_range(), 0..source.len());
    assert_eq!(root.start_position(), Point::new(0, 0));
    assert_eq!(root.end_position(), eof);
    assert!(root.has_error());
    assert_eq!(root.child_count(), 1);
    assert_eq!(root.named_child_count(), 1);

    let error = root.child(0).unwrap();
    assert!(error.is_error());
    assert_eq!(error.kind_id(), u16::MAX);
    assert!(error.is_named());
    assert!(error.is_extra());
    assert!(error.has_error());
    assert!(!error.is_missing());
    assert_eq!(error.byte_range(), 0..source.len());
    assert_eq!(error.start_position(), Point::new(0, 0));
    assert_eq!(error.end_position(), eof);
    let content_index = usize::from(identifier_end != 0);
    assert_eq!(error.child_count(), content_index + 1);
    assert_eq!(error.named_child_count(), content_index + 1);

    if identifier_end != 0 {
        let identifier = error.child(0).unwrap();
        assert_eq!(identifier.kind(), "identifier");
        assert_eq!(identifier.kind_id(), 35);
        assert_eq!(identifier.byte_range(), 0..identifier_end);
        assert_eq!(identifier.start_position(), Point::new(0, 0));
        assert_eq!(identifier.end_position(), Point::new(0, identifier_end));
        assert_eq!(identifier.child_count(), 0);
    }

    // The content leaf, not just its ancestors, must include the trailing
    // newline. Previously the ERROR ended before it or split into two nodes.
    let content = error.child(content_index).unwrap();
    assert_eq!(content.kind(), "bracket_argument_content");
    assert_eq!(content.kind_id(), 37);
    assert_eq!(content.byte_range(), identifier_end..source.len());
    assert_eq!(content.start_position(), Point::new(0, identifier_end));
    assert_eq!(content.end_position(), eof);
    assert_eq!(content.child_count(), 0);
    assert!(content.is_named());
    assert!(!content.is_extra());
    assert!(!content.is_error());
    assert!(!content.has_error());
    assert!(!content.is_missing());
    assert_eq!(
        content.utf8_text(source.as_bytes()).unwrap(),
        &source[identifier_end..]
    );

    // Range-based navigation must also include the content's first and last
    // bytes. In particular, the final newline must resolve to the content leaf,
    // not merely the source_file whose extent already reached EOF before the
    // scanner fix. Repeat these lookups on every incrementally edited tree.
    for byte in [identifier_end, source.len() - 1] {
        assert_eq!(root.first_child_for_byte(byte), Some(error));
        assert_eq!(error.first_child_for_byte(byte), Some(content));
        assert_eq!(
            root.descendant_for_byte_range(byte, byte + 1),
            Some(content)
        );
        assert_eq!(
            root.named_descendant_for_byte_range(byte, byte + 1),
            Some(content)
        );
    }
}

#[test]
fn error_after_identifier_includes_content_through_final_newline() {
    let source = concat!(
        r"Usage: .*/cmake -E \[command\] \[arguments \.\.\.\]",
        "\nAvailable commands:\n",
    );
    assert_eq!(source.len(), 72);
    // Fragment escaped bracket sequences and the final newline across input
    // callbacks. Recovery content must retain the same range and progress count.
    for chunk_size in [source.len(), 7, 1] {
        assert_recovery_range(source, 5, Point::new(2, 0), chunk_size);
    }
}

#[test]
fn error_at_start_keeps_multiline_content_in_one_node() {
    let source = concat!(
        ".*Properties for TARGET rot13:.*\n",
        r#".*rot13.SOURCES = \"rot13.c;rot13.h\".*"#,
        "\n",
        r#".*rot13.POSITION_INDEPENDENT_CODE = \"True\".*"#,
        "\n+\n.*--.*\n.*Properties for SOURCE rot13.c:.*\n",
        r#".*rot13.c.LOCATION = \"[^\"]*/PrintHelpers/rot13.c\".*"#,
        "\n",
        r#".*rot13.c.LANGUAGE = \"C\".*"#,
        "\n",
    );
    assert_eq!(source.len(), 248);
    for chunk_size in [source.len(), 7, 1] {
        assert_recovery_range(source, 0, Point::new(8, 0), chunk_size);
    }
}
