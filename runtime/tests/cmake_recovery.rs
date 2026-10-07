//! CMake recovery regressions from oracle buckets 1b5a6fb7, f5e2762e, and 6c68c51b.

use ts_port::{ParseOptions, ParseState, Parser, Point};

#[test]
fn quoted_variable_whitespace_keeps_error_children_flat() {
    let language = ts_port_cmake::language().into();
    for warmup in ["", "message([==[prior]==])\n", "#[=[prior]=]\n"] {
        let mut parser = Parser::new();
        parser.set_language(&language).unwrap();
        let tree = parser.parse(warmup, None).unwrap();
        assert!(!tree.root_node().has_error());

        // C's scanner starts with token zero (BRACKET_ARGUMENT_OPEN), and
        // deserializing an empty snapshot restores it even after another parse
        // emitted external tokens. Recovery therefore allows bracket content
        // without an opener. Suppressing that token used to introduce a nested
        // ERROR around the quote, dollar sign, and opening brace.
        for source in [
            "message(\"${var\twith\ttab}\")\n",
            "message(\"${var with space}\")\n",
        ] {
            let tree = parser.parse(source, None).unwrap();
            let root = tree.root_node();
            assert_eq!(root.kind(), "source_file");
            assert_eq!(root.byte_range(), 0..source.len());
            assert_eq!(root.start_position(), Point::new(0, 0));
            assert_eq!(root.end_position(), Point::new(1, 0));
            assert!(root.has_error());
            assert_eq!(root.child_count(), 1);
            assert_eq!(root.named_child_count(), 1);

            let error = root.child(0).unwrap();
            assert!(error.is_error());
            assert!(error.is_named());
            assert!(error.is_extra());
            assert!(!error.is_missing());
            assert!(error.has_error());
            assert_eq!(error.byte_range(), 0..source.len());
            assert_eq!(error.range(), root.range());
            assert_eq!(error.child_count(), 6);
            assert_eq!(error.named_child_count(), 2);

            let expected = [
                ("identifier", 35, 0..7, true),
                ("(", 14, 7..8, false),
                ("\"", 16, 8..9, false),
                ("$", 8, 9..10, false),
                ("{", 9, 10..11, false),
                ("bracket_argument_content", 37, 15..source.len(), true),
            ];
            for (index, (kind, kind_id, range, named)) in expected.into_iter().enumerate() {
                let child = error.child(index).unwrap();
                assert_eq!(child.kind(), kind, "{source:?}, child {index}");
                assert_eq!(child.kind_id(), kind_id);
                assert_eq!(child.byte_range(), range);
                assert_eq!(child.start_position(), Point::new(0, range.start));
                let end = if range.end == source.len() {
                    Point::new(1, 0)
                } else {
                    Point::new(0, range.end)
                };
                assert_eq!(child.end_position(), end);
                assert_eq!(child.is_named(), named);
                assert!(!child.is_extra());
                assert!(!child.is_missing());
                assert!(!child.is_error());
                assert!(!child.has_error());
                assert_eq!(child.child_count(), 0);
            }
        }
    }
}

#[test]
fn unfinished_commands_keep_zero_width_bracket_content_at_eof() {
    let language = ts_port_cmake::language().into();
    for warmup in ["", "message([==[prior]==])\n", "#[=[prior]=]\n"] {
        let mut parser = Parser::new();
        parser.set_language(&language).unwrap();
        let tree = parser.parse(warmup, None).unwrap();
        assert!(!tree.root_node().has_error());

        // OneCharacter.cmake and malformedInclude.cmake from bucket f5e2762e,
        // plus the incomplete message command from bucket 6c68c51b.
        // The scanner's zero token means BRACKET_ARGUMENT_OPEN even without an
        // opener. Recovery accepts its zero-width content at EOF as an ordinary
        // named leaf, NOT as a missing node. Empty-state deserialization must
        // restore that token even after a preceding parse emitted bracket tokens.
        for (source, eof, child_count, first_kind) in [
            ("a", Point::new(0, 1), 2, "identifier"),
            ("if(\n", Point::new(1, 0), 3, "if"),
            ("message(S", Point::new(0, 9), 3, "identifier"),
        ] {
            let mut progress_calls = 0;
            let mut progress = |_: &ParseState| {
                progress_calls += 1;
                false
            };
            let mut read = |byte: usize, _: Point| &source.as_bytes()[byte.min(source.len())..];
            let options = ParseOptions::new().progress_callback(&mut progress);
            let tree = parser
                .parse_with_options(&mut read, None, Some(options))
                .unwrap();
            assert_eq!(progress_calls, 0);

            let root = tree.root_node();
            assert_eq!(root.kind(), "source_file");
            assert_eq!(root.byte_range(), 0..source.len());
            assert_eq!(root.start_position(), Point::new(0, 0));
            assert_eq!(root.end_position(), eof);
            assert_eq!(root.child_count(), 1);
            assert_eq!(root.named_child_count(), 1);
            assert!(root.has_error());

            let error = root.child(0).unwrap();
            assert!(error.is_error());
            assert!(error.is_named());
            assert!(error.is_extra());
            assert!(!error.is_missing());
            assert!(error.has_error());
            assert_eq!(error.range(), root.range());
            assert_eq!(error.child_count(), child_count, "{source:?}");
            assert_eq!(error.named_child_count(), 2);
            assert_eq!(error.child(0).unwrap().kind(), first_kind);

            let content = error.child(child_count - 1).unwrap();
            assert_eq!(content.kind(), "bracket_argument_content");
            assert_eq!(content.kind_id(), 37);
            assert_eq!(content.byte_range(), source.len()..source.len());
            assert_eq!(content.start_position(), eof);
            assert_eq!(content.end_position(), eof);
            assert!(content.is_named());
            assert!(!content.is_extra());
            assert!(!content.is_missing());
            assert!(!content.is_error());
            assert!(!content.has_error());
            assert_eq!(content.child_count(), 0);
            assert_eq!(content.named_child_count(), 0);
            assert_eq!(error.named_child(1), Some(content));
            assert!(content.next_sibling().is_none());
        }
    }
}
