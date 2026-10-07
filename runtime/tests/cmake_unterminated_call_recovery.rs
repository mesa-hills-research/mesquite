//! Regression for CMake oracle bucket 7f8795fb.
//!
//! The reference scanner's zero token is BRACKET_ARGUMENT_OPEN. Recovery can
//! therefore emit bracket content without an opener, including at EOF. An
//! inert initial token instead makes these inputs source_file trees with
//! inserted missing delimiters, rather than the reference's top-level ERROR.

use std::ops::Range;
use ts_port::{Language, ParseOptions, ParseState, Parser, Point};

fn assert_root_error(
    source: &[u8],
    end: Point,
    recovery_bytes: Range<usize>,
    expected_progress_calls: usize,
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
                &mut |offset, _| source.get(offset..).unwrap_or_default(),
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
        assert!(!root.is_missing());
        assert_eq!(root.byte_range(), 0..source.len());
        assert_eq!(root.start_position(), Point::new(0, 0));
        assert_eq!(root.end_position(), end);

        let content = root.child(root.child_count() - 1).unwrap();
        assert_eq!(content.kind(), "bracket_argument_content");
        assert_eq!(content.kind_id(), 37);
        assert_eq!(content.byte_range(), recovery_bytes);
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

#[test]
fn unterminated_call_retains_top_level_error() {
    let source = b"message(\n\n\nmessage(\"Additional message\")\n";
    assert_root_error(source, Point::new(4, 0), source.len()..source.len(), 1);
}

#[test]
fn unterminated_call_after_valid_command_retains_top_level_error() {
    let source = b"set(var \"\\\n\")\nmessage(\n\n\nmessage(\"Additional message\")\n";
    assert_root_error(source, Point::new(6, 0), source.len()..source.len(), 1);
}

#[test]
fn nul_in_argument_is_recovery_content_not_eof() {
    let source = b"LIST(APPEND foo TEST\x000000000000000000000000000 )\nCMAKE_HOST_SYSTEM_INFORMATION(RESULT bar QUERY HOSTNAME)\n";
    assert_root_error(source, Point::new(2, 0), 20..source.len(), 0);
}
