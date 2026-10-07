//! CMake recovery regressions from oracle buckets
//! 1b5a6fb7, f5e2762e, 548436bf, and 6c68c51b.

use ts_port::{InputEdit, ParseOptions, ParseState, Parser, Point};

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

#[test]
fn non_utf8_bom_input_recovers_as_one_bracket_content_leaf() {
    let language = ts_port_cmake::language().into();
    for (width, little_endian, end_column) in
        [(2, false, 0), (2, true, 1), (4, false, 0), (4, true, 3)]
    {
        // Recreate the four BOM-UTF-{16,32}-{BE,LE}.cmake fixtures without
        // depending on the external corpus. These are deliberately passed to
        // the UTF-8 parser as raw bytes, not decoded into UTF-8 first.
        let mut source = Vec::new();
        for character in "\u{feff}message(STATUS \"message\")\n".chars() {
            let bytes = if little_endian {
                (character as u32).to_le_bytes()
            } else {
                (character as u32).to_be_bytes()
            };
            source.extend_from_slice(if little_endian {
                &bytes[..width]
            } else {
                &bytes[4 - width..]
            });
        }
        assert_eq!(source.len(), 27 * width);

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

            for chunk_size in [source.len(), 1] {
                let mut progress_calls = 0;
                let mut progress = |_: &ParseState| {
                    progress_calls += 1;
                    false
                };
                let mut read = |byte: usize, _: Point| {
                    let start = byte.min(source.len());
                    let end = (start + chunk_size).min(source.len());
                    &source[start..end]
                };
                let options = ParseOptions::new().progress_callback(&mut progress);
                let tree = parser
                    .parse_with_options(&mut read, None, Some(options))
                    .unwrap();
                assert_eq!(progress_calls, 0);

                // C's zero scanner token enables content during recovery even
                // without an opener. Content checks eof(), not lookahead == 0,
                // and therefore consumes the embedded NULs and malformed UTF-8.
                let root = tree.root_node();
                assert_eq!(root.kind(), "source_file");
                assert_eq!(root.kind_id(), 43);
                assert_eq!(root.byte_range(), 0..source.len());
                assert_eq!(root.start_position(), Point::new(0, 0));
                assert_eq!(root.end_position(), Point::new(1, end_column));
                assert!(root.is_named());
                assert!(!root.is_error());
                assert!(!root.is_extra());
                assert!(!root.is_missing());
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
                assert_eq!(error.child_count(), 1);
                assert_eq!(error.named_child_count(), 1);

                let content = error.child(0).unwrap();
                assert_eq!(content.kind(), "bracket_argument_content");
                assert_eq!(content.kind_id(), 37);
                assert_eq!(content.range(), root.range());
                assert!(content.is_named());
                assert!(!content.is_error());
                assert!(!content.is_extra());
                assert!(!content.is_missing());
                assert!(!content.has_error());
                assert_eq!(content.child_count(), 0);
                assert_eq!(content.named_child_count(), 0);
            }
        }
    }
}

#[test]
fn malformed_variable_commands_recover_as_top_level_errors() {
    let language = ts_port_cmake::language().into();
    for (source, child_count, named_child_count, content_start) in [
        ("set(var \"${\")\n", 7, 3, 11),
        ("message(${var\twith\ttab})\n", 5, 2, 14),
        ("message(${var with space})\n", 5, 2, 14),
    ] {
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

                // UnterminatedBrace1.cmake, NameWithTabs.cmake, and
                // NameWithSpaces.cmake from bucket 6c68c51b. With an inert
                // initial scanner token these incorrectly became normal_command
                // nodes. C's zero token instead permits bracket content during
                // recovery, consuming the command's remainder through EOF.
                let root = tree.root_node();
                assert_eq!(root.kind(), "source_file");
                assert_eq!(root.byte_range(), 0..source.len());
                assert_eq!(root.start_position(), Point::new(0, 0));
                assert_eq!(root.end_position(), Point::new(1, 0));
                assert!(root.has_error());
                assert_eq!(root.child_count(), 1);
                assert_eq!(root.named_child_count(), 1);

                let error = root.child(0).unwrap();
                assert!(error.is_error(), "{source:?}");
                assert!(error.is_named());
                assert!(error.is_extra());
                assert!(!error.is_missing());
                assert!(error.has_error());
                assert_eq!(error.range(), root.range());
                assert_eq!(error.child_count(), child_count);
                assert_eq!(error.named_child_count(), named_child_count);
                assert_eq!(error.child(0).unwrap().kind(), "identifier");

                let content = error.child(child_count - 1).unwrap();
                assert_eq!(content.kind(), "bracket_argument_content");
                assert_eq!(content.kind_id(), 37);
                assert_eq!(content.byte_range(), content_start..source.len());
                assert_eq!(content.start_position(), Point::new(0, content_start));
                assert_eq!(content.end_position(), Point::new(1, 0));
                assert!(content.is_named());
                assert!(!content.is_error());
                assert!(!content.is_extra());
                assert!(!content.is_missing());
                assert!(!content.has_error());
                assert_eq!(content.child_count(), 0);
                assert_eq!(content.named_child_count(), 0);
                assert_eq!(error.named_child(named_child_count - 1), Some(content));
                assert!(content.next_sibling().is_none());
            }
        }
    }
}

#[test]
fn eof_content_tracks_incremental_whitespace_edits() {
    let language = ts_port_cmake::language().into();
    let mut parser = Parser::new();
    parser.set_language(&language).unwrap();

    for (source, eof) in [("a", Point::new(0, 1)), ("if(\n", Point::new(1, 0))] {
        let mut tree = parser.parse(source, None).unwrap();
        let original_sexp = tree.root_node().to_sexp();
        let extended = format!("{source}\n");
        let extended_eof = Point::new(eof.row + 1, 0);

        // Inserting and removing whitespace at EOF must move the ordinary
        // zero-width content leaf, not discard it or turn it into a missing
        // node. Parsing with an old tree also exercises scanner-state restore.
        for (text, old_end, new_end) in [
            (extended.as_str(), eof, extended_eof),
            (source, extended_eof, eof),
        ] {
            tree.edit(&InputEdit {
                start_byte: source.len(),
                old_end_byte: tree.root_node().end_byte(),
                new_end_byte: text.len(),
                start_position: eof,
                old_end_position: old_end,
                new_end_position: new_end,
            });
            tree = parser.parse(text, Some(&tree)).unwrap();
            let root = tree.root_node();
            assert_eq!(root.to_sexp(), original_sexp);
            assert_eq!(root.byte_range(), 0..text.len());
            assert_eq!(root.end_position(), new_end);

            let error = root.child(0).unwrap();
            assert!(error.is_error());
            let content = error.named_child(1).unwrap();
            assert_eq!(content.kind(), "bracket_argument_content");
            assert_eq!(content.byte_range(), text.len()..text.len());
            assert_eq!(content.start_position(), new_end);
            assert_eq!(content.end_position(), new_end);
            assert!(!content.is_missing());
            assert!(!content.is_extra());
            assert!(!content.has_error());
            assert_eq!(content.child_count(), 0);
            assert!(content.next_sibling().is_none());
        }
    }
}
