//! Parser-level regressions for CMake oracle bucket e0b0bff8.

use ts_port::{Language, ParseOptions, ParseState, Parser, Point};

fn assert_recovery_range(source: &str, identifier_end: usize, eof: Point) {
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
                &mut |offset, _| source.as_bytes().get(offset..).unwrap_or_default(),
                None,
                Some(ParseOptions::new().progress_callback(&mut progress)),
            )
            .unwrap();
        assert_eq!(progress_calls, 0);

        let root = tree.root_node();
        assert_eq!(root.kind(), "source_file");
        assert_eq!(root.byte_range(), 0..source.len());
        assert_eq!(root.start_position(), Point::new(0, 0));
        assert_eq!(root.end_position(), eof);
        assert!(root.has_error());
        assert_eq!(root.child_count(), 1);
        assert_eq!(root.named_child_count(), 1);

        let error = root.child(0).unwrap();
        assert!(error.is_error());
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
            assert_eq!(identifier.byte_range(), 0..identifier_end);
            assert_eq!(identifier.start_position(), Point::new(0, 0));
            assert_eq!(identifier.end_position(), Point::new(0, identifier_end));
            assert_eq!(identifier.child_count(), 0);
        }

        // The content leaf, not just its ancestors, must include the trailing
        // newline. Previously the ERROR ended before it or split into two nodes.
        let content = error.child(content_index).unwrap();
        assert_eq!(content.kind(), "bracket_argument_content");
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
    }
}

#[test]
fn error_after_identifier_includes_content_through_final_newline() {
    let source = concat!(
        r"Usage: .*/cmake -E \[command\] \[arguments \.\.\.\]",
        "\nAvailable commands:\n",
    );
    assert_eq!(source.len(), 72);
    assert_recovery_range(source, 5, Point::new(2, 0));
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
    assert_recovery_range(source, 0, Point::new(8, 0));
}
