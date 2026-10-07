//! Additional command recovery regressions from oracle bucket 6c68c51b.

use ts_port::{ParseOptions, ParseState, Parser, Point};

#[test]
fn newline_before_command_arguments_recovers_as_content() {
    assert_recovery(
        "message\n  (\"Example Message\")\n",
        1,
        &[("identifier", 0..7), ("bracket_argument_content", 10..30)],
    );
}

#[test]
fn parenthesis_in_variable_name_preserves_the_preceding_command() {
    assert_recovery(
        "set(\"e(x)\" value)\nmessage(\"-->${e(x)}<--\")\n",
        2,
        &[
            ("identifier", 18..25),
            ("(", 25..26),
            ("\"", 26..27),
            ("$", 30..31),
            ("{", 31..32),
            ("(", 33..34),
            ("bracket_argument_content", 34..43),
        ],
    );
}

// CommandError0.cmake and ParenInVarName0.cmake, with the direct ERROR children
// verified by the C oracle. C initializes/resets the scanner token to zero
// (BRACKET_ARGUMENT_OPEN), so recovery consumes bracket content without an
// opener. An inert token instead incorrectly retains a normal_command node.
fn assert_recovery(
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

        for chunk_size in [source.len(), 1] {
            let mut progress_calls = 0;
            let mut progress = |_: &ParseState| {
                progress_calls += 1;
                false
            };
            let mut read = |byte: usize, _: Point| {
                let start = byte.min(source.len());
                &source.as_bytes()[start..(start + chunk_size).min(source.len())]
            };
            let options = ParseOptions::new().progress_callback(&mut progress);
            let tree = parser
                .parse_with_options(&mut read, None, Some(options))
                .unwrap();
            assert_eq!(progress_calls, 0);

            let root = tree.root_node();
            assert_eq!(root.kind(), "source_file");
            assert_eq!(root.byte_range(), 0..source.len());
            assert_eq!(root.start_position(), Point::new(0, 0));
            assert_eq!(root.end_position(), Point::new(2, 0));
            assert_eq!(root.child_count(), root_child_count);
            assert_eq!(root.named_child_count(), root_child_count);
            assert!(root.has_error());
            if root_child_count == 2 {
                let command = root.child(0).unwrap();
                assert_eq!(command.kind(), "normal_command");
                assert_eq!(command.byte_range(), 0..17);
                assert!(!command.has_error());
            }

            let error = root.child(root_child_count - 1).unwrap();
            assert!(error.is_error());
            assert!(error.is_named());
            assert!(error.is_extra());
            assert!(!error.is_missing());
            assert!(error.has_error());
            assert_eq!(error.byte_range(), expected[0].1.start..source.len());
            assert_eq!(error.start_position(), point_at(source, error.start_byte()));
            assert_eq!(error.end_position(), root.end_position());
            assert_eq!(error.child_count(), expected.len());
            assert_eq!(error.named_child_count(), 2);

            for (i, (kind, range)) in expected.iter().enumerate() {
                let child = error.child(i).unwrap();
                assert_eq!(child.kind(), *kind, "child {i}");
                assert_eq!(child.byte_range(), *range);
                assert_eq!(child.start_position(), point_at(source, range.start));
                assert_eq!(child.end_position(), point_at(source, range.end));
                assert_eq!(child.is_named(), i == 0 || i == expected.len() - 1);
                assert!(!child.is_error());
                assert!(!child.is_extra());
                assert!(!child.is_missing());
                assert!(!child.has_error());
                assert_eq!(child.child_count(), 0);
            }
            let content = error.named_child(1).unwrap();
            assert_eq!(content.kind(), "bracket_argument_content");
            assert_eq!(content.kind_id(), 37);
            assert!(content.next_sibling().is_none());
        }
    }
}
