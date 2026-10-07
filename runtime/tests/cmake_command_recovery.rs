//! Regressions for CMake oracle bucket 6c68c51b.

use ts_port::{Language, Parser, Point};

#[test]
fn malformed_variable_references_recover_as_top_level_errors() {
    let language = Language::from(ts_port_cmake::language());

    // Expected shapes and ranges come from the C oracle. In particular, these
    // must not become normal_command nodes with a nested argument-list error.
    for (source, child_count, named_child_count, content_start) in [
        ("set(var \"${\")\n", 7, 3, 11),
        ("message(${var\twith\ttab})\n", 5, 2, 14),
        ("message(${var with space})\n", 5, 2, 14),
    ] {
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

            let tree = parser.parse(source, None).unwrap();
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
