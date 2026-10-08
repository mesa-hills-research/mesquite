//! CMake error recovery from whitespace in quoted variable names.

use tree_sitter::{InputEdit, Language, ParseOptions, ParseState, Parser, Point, Tree};

const SOURCES: [&str; 3] = [
    "message(\"${var\twith\ttab}\")\n",
    "message(\"${var with space}\")\n",
    // Reduced Registry-query.cmake: a valid second variable reference must
    // stay within recovery content after whitespace in the first one's name.
    "message(\"${CMAKE_ CURRENT_SOURCE_DIR}/${FILE_DIR}\")\n",
];

#[test]
fn whitespace_in_quoted_variable_names_does_not_nest_errors() {
    let language = Language::from(tree_sitter_cmake::language());
    for source in SOURCES {
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

            // Split malformed variable names and their recovery suffix at
            // every byte boundary, including the final newline before EOF.
            for chunk_size in [source.len(), 1] {
                let mut progress_calls = 0;
                let mut progress = |_: &ParseState| {
                    progress_calls += 1;
                    false
                };
                let tree = parser
                    .parse_with_options(
                        &mut |offset, _| {
                            let remaining = source.as_bytes().get(offset..).unwrap_or_default();
                            &remaining[..remaining.len().min(chunk_size)]
                        },
                        None,
                        Some(ParseOptions::new().progress_callback(&mut progress)),
                    )
                    .unwrap();
                assert_eq!(progress_calls, 0);

                assert_quoted_variable_recovery(&tree, source);
            }
        }
    }
}

#[test]
fn repairing_and_restoring_quoted_variable_names_resets_recovery_state() {
    let language = Language::from(tree_sitter_cmake::language());
    for source in SOURCES {
        let mut parser = Parser::new();
        parser.set_language(&language).unwrap();
        let mut tree = parser.parse(source, None).unwrap();
        assert_quoted_variable_recovery(&tree, source);

        // Replace the malformed variable name with a valid one of equal length,
        // then restore it. Reusing the edited trees must neither retain the old
        // recovery-content state nor introduce a nested ERROR on restoration.
        let repaired = source.replace([' ', '\t'], "_");
        let start = source.find("${").unwrap() + 2;
        let end = source.find('}').unwrap();
        let edit = InputEdit {
            start_byte: start,
            old_end_byte: end,
            new_end_byte: end,
            start_position: Point::new(0, start),
            old_end_position: Point::new(0, end),
            new_end_position: Point::new(0, end),
        };
        for _ in 0..2 {
            tree.edit(&edit);
            tree = parser.parse(&repaired, Some(&tree)).unwrap();
            assert!(!tree.root_node().has_error());
            assert_eq!(tree.root_node().byte_range(), 0..repaired.len());
            assert_eq!(tree.root_node().end_position(), Point::new(1, 0));

            tree.edit(&edit);
            tree = parser.parse(source, Some(&tree)).unwrap();
            assert_quoted_variable_recovery(&tree, source);
        }
    }
}

#[test]
fn malformed_quotes_after_valid_arguments_keep_flat_recovery_children() {
    // Two larger files from CMake's tests exercise recovery after valid arguments
    // and comments, rather than immediately after a command's opening parenthesis.
    let deferred_call = concat!(
        "# Argument syntax error evaluated at deferred call site.\n",
        "cmake_language(DEFER CALL message \"Deferred \\X Error\")\n",
    );
    let compile_features = concat!(
        "enable_language(@lang@)\n\n",
        "# Make sure the compile command is not hidden.\n",
        "string(REPLACE \"${CMAKE_START_TEMP_FILE}\" \"\" ",
        "CMAKE_@lang@_COMPILE_OBJECT \"${CMAKE_@lang@_COMPILE_OBJECT}\")\n",
        "string(REPLACE \"${CMAKE_END_TEMP_FILE}\" \"\" ",
        "CMAKE_@lang@_COMPILE_OBJECT \"${CMAKE_@lang@_COMPILE_OBJECT}\")\n\n",
        "add_library(foo \"@RunCMake_SOURCE_DIR@/empty.@ext@\")\n",
    );
    let deferred_children = [
        ("identifier", 35, 57..71),
        ("(", 14, 71..72),
        ("argument", 54, 72..77),
        ("argument", 54, 78..82),
        ("argument", 54, 83..90),
        ("\"", 16, 91..92),
        ("bracket_argument_content", 37, 101..112),
    ];
    let compile_children = [
        ("identifier", 35, 72..78),
        ("(", 14, 78..79),
        ("argument", 54, 79..86),
        ("argument", 54, 87..113),
        ("argument", 54, 114..116),
        ("argument", 54, 117..144),
        ("\"", 16, 145..146),
        ("$", 8, 146..147),
        ("{", 9, 147..148),
        ("bracket_argument_content", 37, 154..338),
    ];
    let language = Language::from(tree_sitter_cmake::language());
    for (source, error_index, expected_children, expected_progress_calls) in [
        (deferred_call, 1, deferred_children.as_slice(), 1),
        (compile_features, 2, compile_children.as_slice(), 2),
    ] {
        let mut parser = Parser::new();
        parser.set_language(&language).unwrap();
        for preceding_source in [None, Some("message([[value]])"), Some("#[=[comment]=]")] {
            if let Some(preceding_source) = preceding_source {
                let tree = parser.parse(preceding_source, None).unwrap();
                assert!(!tree.root_node().has_error());
            }
            for chunk_size in [source.len(), 1] {
                let mut progress_calls = 0;
                let mut progress = |_: &ParseState| {
                    progress_calls += 1;
                    false
                };
                let tree = parser
                    .parse_with_options(
                        &mut |offset, _| {
                            let remaining = source.as_bytes().get(offset..).unwrap_or_default();
                            &remaining[..remaining.len().min(chunk_size)]
                        },
                        None,
                        Some(ParseOptions::new().progress_callback(&mut progress)),
                    )
                    .unwrap();
                assert_eq!(progress_calls, expected_progress_calls);
                let root = tree.root_node();
                assert_eq!(root.kind(), "source_file");
                assert_eq!(root.byte_range(), 0..source.len());
                assert_eq!(root.end_position(), point_at(source, source.len()));
                assert_eq!(root.child_count(), error_index + 1);
                assert_eq!(root.named_child_count(), error_index + 1);
                for index in 0..error_index {
                    assert!(!root.child(index).unwrap().has_error());
                }

                let error = root.child(error_index).unwrap();
                assert!(error.is_error());
                assert!(error.is_extra());
                assert!(error.is_named());
                assert!(!error.is_missing());
                assert_eq!(
                    error.byte_range(),
                    expected_children[0].2.start..source.len()
                );
                assert_eq!(error.start_position(), point_at(source, error.start_byte()));
                assert_eq!(error.end_position(), root.end_position());
                assert_eq!(error.child_count(), expected_children.len());
                let mut named_count = 0;
                for (index, (kind, id, bytes)) in expected_children.iter().enumerate() {
                    let child = error.child(index).unwrap();
                    let named = matches!(
                        *kind,
                        "identifier" | "argument" | "bracket_argument_content"
                    );
                    assert_eq!(child.kind(), *kind);
                    assert_eq!(child.kind_id(), *id);
                    assert_eq!(child.byte_range(), *bytes);
                    assert_eq!(child.start_position(), point_at(source, bytes.start));
                    assert_eq!(child.end_position(), point_at(source, bytes.end));
                    assert_eq!(child.is_named(), named);
                    assert_eq!(child.child_count(), usize::from(*kind == "argument"));
                    assert!(!child.is_extra());
                    assert!(!child.is_error());
                    assert!(!child.has_error());
                    assert!(!child.is_missing());
                    if named {
                        assert_eq!(error.named_child(named_count), Some(child));
                        named_count += 1;
                    }
                }
                assert_eq!(error.named_child_count(), named_count);
            }
        }
    }
}

fn point_at(source: &str, byte: usize) -> Point {
    source.as_bytes()[..byte]
        .iter()
        .fold(Point::new(0, 0), |point, &ch| {
            if ch == b'\n' {
                Point::new(point.row + 1, 0)
            } else {
                Point::new(point.row, point.column + 1)
            }
        })
}

fn assert_quoted_variable_recovery(tree: &Tree, source: &str) {
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

    let content_start = source.find([' ', '\t']).unwrap() + 1;
    for (index, (kind, id, named, bytes)) in [
        ("identifier", 35, true, 0..7),
        ("(", 14, false, 7..8),
        ("\"", 16, false, 8..9),
        ("$", 8, false, 9..10),
        ("{", 9, false, 10..11),
        (
            "bracket_argument_content",
            37,
            true,
            content_start..source.len(),
        ),
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
