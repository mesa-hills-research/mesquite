//! CMake error recovery from a malformed `endif`.

use tree_sitter::{Language, ParseOptions, ParseState, Parser, Point};

#[test]
fn malformed_endif_retains_identifier_and_recovery_content() {
    let source = b"if ( cond )\nen]dif()\n";
    let language = Language::from(tree_sitter_cmake::language());
    let mut parser = Parser::new();
    parser.set_language(&language).unwrap();

    // Exercise both the newly created scanner and reuse after bracket scans
    // leave nonzero token state. The scanner resets both fields to zero, and
    // zero is BRACKET_ARGUMENT_OPEN, allowing content during recovery without
    // an actual opening bracket.
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
                &mut |offset, _| source.get(offset..).unwrap_or_default(),
                None,
                Some(ParseOptions::new().progress_callback(&mut progress)),
            )
            .unwrap();
        assert_eq!(progress_calls, 0);

        let root = tree.root_node();
        assert_eq!(root.kind(), "source_file");
        assert_eq!(root.child_count(), 1);
        let error = root.child(0).unwrap();
        assert!(error.is_error());
        assert_eq!(error.child_count(), 3);
        assert_eq!(error.named_child_count(), 3);
        assert_eq!(error.child(0).unwrap().kind(), "if_command");

        // C retains these two leaves directly under ERROR, rather than making
        // a normal_command containing a nested ERROR and the parentheses.
        for (index, kind, bytes, start, end) in [
            (1, "identifier", 12..14, Point::new(1, 0), Point::new(1, 2)),
            (
                2,
                "bracket_argument_content",
                14..21,
                Point::new(1, 2),
                Point::new(2, 0),
            ),
        ] {
            let child = error.child(index).unwrap();
            assert_eq!(child.kind(), kind);
            assert_eq!(child.byte_range(), bytes);
            assert_eq!(child.start_position(), start);
            assert_eq!(child.end_position(), end);
            assert_eq!(child.child_count(), 0);
            assert!(child.is_named());
            assert!(!child.is_extra());
            assert!(!child.is_error());
            assert!(!child.has_error());
            assert!(!child.is_missing());
        }
    }
}
