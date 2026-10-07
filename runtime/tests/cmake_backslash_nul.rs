//! Parser-level regression for CMake oracle bucket 9a8ee9c6.

use ts_port::{Language, ParseOptions, ParseState, Parser, Point};

#[test]
fn backslash_nul_recovers_as_one_bracket_content_token() {
    // NullAfterBackslash.cmake: the escape before an embedded NUL fails normal
    // lexing. Recovery must consume the entire suffix, including the next line,
    // as bracket content rather than reducing an argument and a nested ERROR.
    let source = format!("A({}\\\0\n({}\n", "A".repeat(52), "A".repeat(54)).into_bytes();
    assert_eq!(source.len(), 113);
    assert_eq!(&source[54..57], b"\\\0\n");

    let mut parser = Parser::new();
    parser
        .set_language(&Language::from(ts_port_cmake::language()))
        .unwrap();

    // Check the initial scanner and reuse after tokens that change both scanner
    // fields. C's calloc and empty-state deserialization both restore token zero
    // (BRACKET_ARGUMENT_OPEN), enabling content without an opener in recovery.
    for preceding_source in [None, Some("message([[value]])"), Some("#[=[comment]=]")] {
        for chunk_size in [source.len(), 1] {
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
            assert_eq!(progress_calls, 1);

            let root = tree.root_node();
            assert_eq!(root.kind(), "source_file");
            assert_eq!(root.byte_range(), 0..113);
            assert_eq!(root.start_position(), Point::new(0, 0));
            assert_eq!(root.end_position(), Point::new(2, 0));
            assert!(root.has_error());
            assert_eq!(root.child_count(), 1);
            assert_eq!(root.named_child_count(), 1);

            let error = root.child(0).unwrap();
            assert!(error.is_error());
            assert!(error.is_extra());
            assert!(error.is_named());
            assert!(!error.is_missing());
            assert_eq!(error.range(), root.range());
            assert_eq!(error.child_count(), 3);
            assert_eq!(error.named_child_count(), 2);

            for (index, kind, bytes, start, end, named) in [
                (
                    0,
                    "identifier",
                    0..1,
                    Point::new(0, 0),
                    Point::new(0, 1),
                    true,
                ),
                (1, "(", 1..2, Point::new(0, 1), Point::new(0, 2), false),
                (
                    2,
                    "bracket_argument_content",
                    54..113,
                    Point::new(0, 54),
                    Point::new(2, 0),
                    true,
                ),
            ] {
                let child = error.child(index).unwrap();
                assert_eq!(child.kind(), kind);
                assert_eq!(child.byte_range(), bytes);
                assert_eq!(child.start_position(), start);
                assert_eq!(child.end_position(), end);
                assert_eq!(child.is_named(), named);
                assert_eq!(child.child_count(), 0);
                assert!(!child.is_extra());
                assert!(!child.is_error());
                assert!(!child.has_error());
                assert!(!child.is_missing());
            }
        }
    }
}
