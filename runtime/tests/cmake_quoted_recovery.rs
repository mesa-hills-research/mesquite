//! Additional quoted-argument recovery regressions from oracle bucket 1b5a6fb7.

use ts_port::{Parser, Point};

#[test]
fn invalid_escape_keeps_the_quote_directly_under_error() {
    assert_recovery_children(
        concat!(
            "# Argument syntax error evaluated at deferred call site.\n",
            "cmake_language(DEFER CALL message \"Deferred \\X Error\")\n",
        ),
        2,
        &[
            ("identifier", 57..71),
            ("(", 71..72),
            ("argument", 72..77),
            ("argument", 78..82),
            ("argument", 83..90),
            ("\"", 91..92),
            ("bracket_argument_content", 101..112),
        ],
    );
}

#[test]
fn variable_template_keeps_quote_and_variable_prefix_directly_under_error() {
    assert_recovery_children(
        concat!(
            "enable_language(@lang@)\n\n",
            "# Make sure the compile command is not hidden.\n",
            "string(REPLACE \"${CMAKE_START_TEMP_FILE}\" \"\" CMAKE_@lang@_COMPILE_OBJECT \"${CMAKE_@lang@_COMPILE_OBJECT}\")\n",
            "string(REPLACE \"${CMAKE_END_TEMP_FILE}\" \"\" CMAKE_@lang@_COMPILE_OBJECT \"${CMAKE_@lang@_COMPILE_OBJECT}\")\n\n",
            "add_library(foo \"@RunCMake_SOURCE_DIR@/empty.@ext@\")\n",
        ),
        3,
        &[
            ("identifier", 72..78),
            ("(", 78..79),
            ("argument", 79..86),
            ("argument", 87..113),
            ("argument", 114..116),
            ("argument", 117..144),
            ("\"", 145..146),
            ("$", 146..147),
            ("{", 147..148),
            ("bracket_argument_content", 154..338),
        ],
    );
}

// The two larger bucket inputs exercise scanner reset after line comments and
// recovery after successful arguments. Check the C-oracle-matched direct ERROR
// children, including anonymous punctuation that an S-expression would omit.
fn assert_recovery_children(
    source: &str,
    root_child_count: usize,
    expected: &[(&str, std::ops::Range<usize>)],
) {
    fn point_at(source: &str, byte: usize) -> Point {
        let prefix = &source[..byte];
        Point::new(
            prefix.bytes().filter(|&b| b == b'\n').count(),
            prefix.rsplit('\n').next().unwrap().len(),
        )
    }

    let language = ts_port_cmake::language().into();
    for warmup in [
        None,
        Some("message([==[prior]==])\n"),
        Some("#[=[prior]=]\n"),
    ] {
        let mut parser = Parser::new();
        parser.set_language(&language).unwrap();
        if let Some(warmup) = warmup {
            assert!(!parser.parse(warmup, None).unwrap().root_node().has_error());
        }

        let tree = parser.parse(source, None).unwrap();
        let root = tree.root_node();
        assert_eq!(root.kind(), "source_file");
        assert_eq!(root.byte_range(), 0..source.len());
        assert_eq!(root.child_count(), root_child_count);
        assert_eq!(root.named_child_count(), root_child_count);
        assert!(root.has_error());

        let error = root.child(root_child_count - 1).unwrap();
        assert!(error.is_error());
        assert!(error.is_named());
        assert!(error.is_extra());
        assert!(!error.is_missing());
        assert_eq!(error.byte_range(), expected[0].1.start..source.len());
        assert_eq!(error.child_count(), expected.len());
        assert_eq!(error.start_position(), point_at(source, error.start_byte()));
        assert_eq!(error.end_position(), point_at(source, source.len()));

        for (i, (kind, range)) in expected.iter().enumerate() {
            let child = error.child(i).unwrap();
            assert_eq!(child.kind(), *kind, "child {i}");
            assert_eq!(child.byte_range(), *range);
            assert_eq!(child.start_position(), point_at(source, range.start));
            assert_eq!(child.end_position(), point_at(source, range.end));
            assert!(!child.has_error());
            assert!(!child.is_extra());
            assert!(!child.is_missing());
        }
    }
}
