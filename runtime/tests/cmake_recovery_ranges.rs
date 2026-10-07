//! CMake error-range regression from oracle bucket e0b0bff8.

use ts_port::{ParseOptions, ParseState, Parser, Point};

#[test]
fn recovery_content_extends_error_through_the_final_newline() {
    // The two smallest cases from bucket e0b0bff8. With an inert initial scanner
    // token instead of C's zero (BRACKET_ARGUMENT_OPEN), recovery lost the content
    // child and final newline, and sometimes split the ERROR into multiple nodes.
    let usage = r"Usage: .*/cmake -E \[command\] \[arguments \.\.\.\]
Available commands:
";
    let properties = r#".*Properties for TARGET rot13:.*
.*rot13.SOURCES = \"rot13.c;rot13.h\".*
.*rot13.POSITION_INDEPENDENT_CODE = \"True\".*
+
.*--.*
.*Properties for SOURCE rot13.c:.*
.*rot13.c.LOCATION = \"[^\"]*/PrintHelpers/rot13.c\".*
.*rot13.c.LANGUAGE = \"C\".*
"#;
    assert_eq!(usage.len(), 72);
    assert_eq!(properties.len(), 248);

    let language = ts_port_cmake::language().into();
    for (source, content_start, end_row) in [(usage, 5, 2), (properties, 0, 8)] {
        for warmup in [
            None,
            Some("message([==[prior]==])\n"),
            Some("#[=[prior]=]\n"),
        ] {
            let mut parser = Parser::new();
            parser.set_language(&language).unwrap();
            if let Some(warmup) = warmup {
                let tree = parser.parse(warmup, None).unwrap();
                assert!(!tree.root_node().has_error());
            }

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
            assert_eq!(root.end_position(), Point::new(end_row, 0));
            assert!(root.has_error());
            assert_eq!(root.child_count(), 1);
            assert_eq!(root.named_child_count(), 1);

            let error = root.child(0).unwrap();
            assert!(error.is_error());
            assert!(error.is_named());
            assert!(error.is_extra());
            assert!(!error.is_missing());
            assert!(error.has_error());
            assert_eq!(error.range(), root.range());
            let content_index = usize::from(content_start != 0);
            assert_eq!(error.child_count(), content_index + 1);
            assert_eq!(error.named_child_count(), content_index + 1);

            let expected = [
                ("identifier", 35, 0..content_start),
                ("bracket_argument_content", 37, content_start..source.len()),
            ];
            for (index, (kind, kind_id, range)) in
                expected.into_iter().skip(1 - content_index).enumerate()
            {
                let child = error.child(index).unwrap();
                assert_eq!(child.kind(), kind);
                assert_eq!(child.kind_id(), kind_id);
                assert_eq!(child.byte_range(), range);
                assert_eq!(child.start_position(), Point::new(0, range.start));
                let end = if range.end == source.len() {
                    Point::new(end_row, 0)
                } else {
                    Point::new(0, range.end)
                };
                assert_eq!(child.end_position(), end);
                assert!(child.is_named());
                assert!(!child.is_extra());
                assert!(!child.is_missing());
                assert!(!child.is_error());
                assert!(!child.has_error());
                assert_eq!(child.child_count(), 0);
                assert_eq!(error.named_child(index), Some(child));
            }
        }
    }
}
