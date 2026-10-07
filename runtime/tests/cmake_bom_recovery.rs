//! Parser-level regression for CMake oracle bucket 548436bf.

use ts_port::{Language, ParseOptions, ParseState, Parser, Point};

#[test]
fn non_utf8_boms_recover_as_a_single_bracket_content_token() {
    // These are deliberately passed to the UTF-8 parser as raw bytes, matching
    // the oracle. Invalid UTF-8 and embedded NULs are content, not EOF.
    let source = "\u{feff}message(STATUS \"message\")\n";
    let encodings: [(&str, Vec<u8>, usize); 4] = [
        (
            "UTF-16-BE",
            source.encode_utf16().flat_map(u16::to_be_bytes).collect(),
            0,
        ),
        (
            "UTF-16-LE",
            source.encode_utf16().flat_map(u16::to_le_bytes).collect(),
            1,
        ),
        (
            "UTF-32-BE",
            source
                .chars()
                .flat_map(|character| u32::from(character).to_be_bytes())
                .collect(),
            0,
        ),
        (
            "UTF-32-LE",
            source
                .chars()
                .flat_map(|character| u32::from(character).to_le_bytes())
                .collect(),
            3,
        ),
    ];
    let language = Language::from(ts_port_cmake::language());

    for (encoding, bytes, end_column) in encodings {
        let mut parser = Parser::new();
        parser.set_language(&language).unwrap();

        // Fresh scanners and scanners reset after bracket arguments/comments
        // must both restore token zero (BRACKET_ARGUMENT_OPEN). Recovery then
        // permits content without an actual opener, as in the C reference.
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
                    &mut |offset, _| bytes.get(offset..).unwrap_or_default(),
                    None,
                    Some(ParseOptions::new().progress_callback(&mut progress)),
                )
                .unwrap();
            assert_eq!(progress_calls, 0, "{encoding}");

            let root = tree.root_node();
            assert_eq!(root.kind(), "source_file", "{encoding}");
            assert_eq!(root.child_count(), 1, "{encoding}");
            let error = root.child(0).unwrap();
            assert_eq!(error.kind(), "ERROR", "{encoding}");
            assert_eq!(error.child_count(), 1, "{encoding}");
            let content = error.child(0).unwrap();
            assert_eq!(content.kind(), "bracket_argument_content", "{encoding}");
            assert_eq!(content.child_count(), 0, "{encoding}");

            for (node, is_error, is_extra, has_error) in [
                (root, false, false, true),
                (error, true, true, true),
                (content, false, false, false),
            ] {
                assert_eq!(node.byte_range(), 0..bytes.len(), "{encoding}");
                assert_eq!(node.start_position(), Point::new(0, 0), "{encoding}");
                assert_eq!(node.end_position(), Point::new(1, end_column), "{encoding}");
                assert!(node.is_named(), "{encoding}");
                assert!(!node.is_missing(), "{encoding}");
                assert_eq!(node.is_error(), is_error, "{encoding}");
                assert_eq!(node.is_extra(), is_extra, "{encoding}");
                assert_eq!(node.has_error(), has_error, "{encoding}");
                assert_eq!(node.named_child_count(), node.child_count(), "{encoding}");
            }
        }
    }
}
