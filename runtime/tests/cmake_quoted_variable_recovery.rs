//! Regression for CMake oracle bucket 1b5a6fb7.

use ts_port::{Language, ParseOptions, ParseState, Parser, Point};

#[test]
fn whitespace_in_quoted_variable_names_does_not_nest_errors() {
    let language = Language::from(ts_port_cmake::language());
    for source in [
        "message(\"${var\twith\ttab}\")\n",
        "message(\"${var with space}\")\n",
    ] {
        let mut parser = Parser::new();
        parser.set_language(&language).unwrap();

        // Check initial state and reuse after bracket argument/comment scans.
        // C's calloc and empty-snapshot reset select BRACKET_ARGUMENT_OPEN,
        // allowing bracket content during recovery even without an opener.
        // An inert or retained token nests an extra ERROR around the quote,
        // dollar sign, and opening brace instead.
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
                    &mut |offset, _| source.as_bytes().get(offset..).unwrap_or_default(),
                    None,
                    Some(ParseOptions::new().progress_callback(&mut progress)),
                )
                .unwrap();
            assert_eq!(progress_calls, 0);

            let root = tree.root_node();
            assert_eq!(root.kind(), "source_file");
            assert_eq!(root.kind_id(), 43);
            assert_eq!(root.byte_range(), 0..source.len());
            assert_eq!(root.start_position(), Point::new(0, 0));
            assert_eq!(root.end_position(), Point::new(1, 0));
            assert!(root.has_error());
            assert_eq!(root.child_count(), 1);
            assert_eq!(root.named_child_count(), 1);

            let error = root.child(0).unwrap();
            assert!(error.is_error());
            assert!(error.is_extra());
            assert!(error.is_named());
            assert!(!error.is_missing());
            assert_eq!(error.byte_range(), 0..source.len());
            assert_eq!(error.start_position(), Point::new(0, 0));
            assert_eq!(error.end_position(), Point::new(1, 0));
            assert_eq!(error.child_count(), 6);
            assert_eq!(error.named_child_count(), 2);

            for (index, (kind, id, named, bytes)) in [
                ("identifier", 35, true, 0..7),
                ("(", 14, false, 7..8),
                ("\"", 16, false, 8..9),
                ("$", 8, false, 9..10),
                ("{", 9, false, 10..11),
                ("bracket_argument_content", 37, true, 15..source.len()),
            ]
            .into_iter()
            .enumerate()
            {
                let child = error.child(index).unwrap();
                assert_eq!(child.kind(), kind);
                assert_eq!(child.kind_id(), id);
                assert_eq!(child.is_named(), named);
                assert_eq!(child.byte_range(), bytes);
                assert_eq!(child.start_position(), Point::new(0, bytes.start));
                assert_eq!(
                    child.end_position(),
                    if bytes.end == source.len() {
                        Point::new(1, 0)
                    } else {
                        Point::new(0, bytes.end)
                    }
                );
                assert_eq!(child.child_count(), 0);
                assert!(!child.is_extra());
                assert!(!child.is_error());
                assert!(!child.has_error());
                assert!(!child.is_missing());
            }
            assert_eq!(error.named_child(0), error.child(0));
            assert_eq!(error.named_child(1), error.child(5));
        }
    }
}
