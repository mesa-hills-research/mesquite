//! Parser-level regression for CMake oracle bucket 381f9bd4.

use tree_sitter::{Language, ParseOptions, ParseState, Parser, Point};

#[test]
fn truncated_utf32_boms_recover_as_one_bracket_content_token() {
    // Broken-BOM-UTF-32-{BE,LE}.cmake, parsed as UTF-8. Invalid FE/FF bytes
    // decode to -1 and NUL bytes decode to zero, but neither signifies EOF.
    for source in [b"\0\0\xfe", b"\xff\xfe\0"] {
        for chunk_size in 1..=source.len() {
            let mut parser = Parser::new();
            parser
                .set_language(&Language::from(tree_sitter_cmake::language()))
                .unwrap();

            // Exercise creation and reuse after both bracket token families.
            // Empty-state deserialization must reset token AND delimiter level,
            // just like the reference scanner's zero-initialized allocation.
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
                            let remaining = source.get(offset..).unwrap_or_default();
                            &remaining[..remaining.len().min(chunk_size)]
                        },
                        None,
                        Some(ParseOptions::new().progress_callback(&mut progress)),
                    )
                    .unwrap();
                assert_eq!(progress_calls, 0);

                let root = tree.root_node();
                assert_eq!(root.kind(), "source_file");
                assert_eq!(root.kind_id(), 43);
                assert_eq!(root.byte_range(), 0..3);
                assert_eq!(root.start_position(), Point::new(0, 0));
                assert_eq!(root.end_position(), Point::new(0, 3));
                assert!(root.is_named());
                assert!(!root.is_extra());
                assert!(!root.is_error());
                assert!(!root.is_missing());
                assert!(root.has_error());
                assert_eq!(root.child_count(), 1);
                assert_eq!(root.named_child_count(), 1);

                let error = root.child(0).unwrap();
                assert_eq!(root.named_child(0), Some(error));
                assert_eq!(error.kind(), "ERROR");
                assert_eq!(error.kind_id(), u16::MAX);
                assert_eq!(error.range(), root.range());
                assert!(error.is_named());
                assert!(error.is_extra());
                assert!(error.is_error());
                assert!(error.has_error());
                assert!(!error.is_missing());
                assert_eq!(error.child_count(), 1);
                assert_eq!(error.named_child_count(), 1);

                let content = error.child(0).unwrap();
                assert_eq!(error.named_child(0), Some(content));
                assert_eq!(content.kind(), "bracket_argument_content");
                assert_eq!(content.kind_id(), 37);
                assert_eq!(content.range(), root.range());
                assert!(content.is_named());
                assert!(!content.is_extra());
                assert!(!content.is_error());
                assert!(!content.has_error());
                assert!(!content.is_missing());
                assert_eq!(content.child_count(), 0);
                assert_eq!(content.named_child_count(), 0);
            }
        }
    }
}
