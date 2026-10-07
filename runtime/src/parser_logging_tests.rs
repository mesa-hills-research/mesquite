//! Regression coverage for selecting the quiet/logged parse specialization.
use super::*;
use std::sync::{LazyLock, Mutex};

// Use the same immutable C fixture tables as the reduction tests, with their
// generated lexer so diagnostics also exercise tokenization and error recovery.
#[path = "../../grammars/c/src/lex.rs"]
mod c_lexer;

fn parser() -> Parser {
    static TABLES: LazyLock<tree_sitter_language::LanguageTables> = LazyLock::new(|| {
        tree_sitter_language::LanguageTables::decode(
            include_bytes!("../../grammars/c/src/tables.bin"),
            c_lexer::ts_lex,
            Some(c_lexer::ts_lex_keywords),
            None,
        )
    });
    let mut parser = ts_parser_new();
    assert!(ts_parser_set_language(
        &mut parser,
        Some(&Language::from(&*TABLES))
    ));
    parser
}

fn install_logger(parser: &mut Parser) -> Arc<Mutex<Vec<(LogType, String)>>> {
    let messages = Arc::new(Mutex::new(Vec::new()));
    let output = messages.clone();
    ts_parser_set_logger(
        parser,
        Some(Box::new(move |kind, message| {
            output.lock().unwrap().push((kind, message.to_owned()));
        })),
    );
    messages
}

struct DotOutput(Arc<Mutex<Vec<u8>>>);

impl std::io::Write for DotOutput {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        self.0.lock().unwrap().extend_from_slice(bytes);
        Ok(bytes.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

fn parse(
    parser: &mut Parser,
    source: &[u8],
    old_tree: Option<&Tree>,
    cancel: bool,
) -> (Option<Tree>, Vec<(usize, bool)>) {
    let mut input = SliceInput {
        bytes: source,
        chunk_start: 0,
    };
    let mut progress = Vec::new();
    let tree = ts_parser_parse_with_options(
        parser,
        old_tree,
        &mut input,
        ParseOptions::new().progress_callback(&mut |state: &ParseState| {
            progress.push((state.current_byte_offset(), state.has_error()));
            cancel
        }),
    );
    (tree, progress)
}

fn snapshot(tree: &Tree) -> String {
    // Includes hidden nodes and all subtree metadata, not just the S-expression.
    format!("{:?}", tree.root)
}

#[test]
fn quiet_logger_and_dot_paths_match_fresh_and_incremental_parses() {
    for statement in ["int value = 42; /* comment */\n", "int value = ; @\n"] {
        let source = statement.repeat(40);
        let mut expected = Vec::new();
        for mode in 0..4 {
            let mut parser = parser();
            let messages = (mode & 1 != 0).then(|| install_logger(&mut parser));
            let dot = Arc::new(Mutex::new(Vec::new()));
            if mode & 2 != 0 {
                ts_parser_print_dot_graphs(&mut parser, Some(Box::new(DotOutput(dot.clone()))));
            }
            let (tree, progress) = parse(&mut parser, source.as_bytes(), None, false);
            let mut tree = tree.unwrap();
            assert!(!progress.is_empty());
            let mut actual = vec![(snapshot(&tree), progress)];

            crate::tree::ts_tree_edit(
                &mut tree,
                &InputEdit {
                    start_byte: 0,
                    old_end_byte: 3,
                    new_end_byte: 4,
                    start_point: Point { row: 0, column: 0 },
                    old_end_point: Point { row: 0, column: 3 },
                    new_end_point: Point { row: 0, column: 4 },
                },
            );
            let edited = source.replacen("int", "long", 1);
            let (tree, progress) = parse(&mut parser, edited.as_bytes(), Some(&tree), false);
            actual.push((snapshot(&tree.unwrap()), progress));
            if mode == 0 {
                expected = actual;
            } else {
                assert_eq!(actual, expected, "diagnostic mode {mode}: {statement}");
            }
            if let Some(messages) = messages {
                let messages = messages.lock().unwrap();
                assert!(messages.iter().any(|(kind, _)| *kind == LogType::Lex));
                for prefix in [
                    "new_parse",
                    "lexed_lookahead",
                    "parse_after_edit",
                    "reuse_node",
                    "done",
                ] {
                    assert!(
                        messages.iter().any(|(_, text)| text.starts_with(prefix)),
                        "{prefix}"
                    );
                }
            }
            let dot = dot.lock().unwrap();
            assert_eq!(dot.is_empty(), mode & 2 == 0);
            if !dot.is_empty() {
                assert!(dot.windows(7).any(|bytes| bytes == b"digraph"));
            }
        }
    }
}

#[test]
fn logging_configuration_is_reselected_after_cancellation() {
    let source = "int value = 42;\n".repeat(100);
    let mut expected = None;
    // All four quiet/logged combinations on the first and resumed calls.
    for (first_logged, resumed_logged) in
        [(false, false), (false, true), (true, false), (true, true)]
    {
        let mut parser = parser();
        let first_messages = first_logged.then(|| install_logger(&mut parser));
        let (tree, mut progress) = parse(&mut parser, source.as_bytes(), None, true);
        assert!(tree.is_none());
        assert_eq!(progress.len(), 1);
        assert!(ts_parser_has_outstanding_parse(&mut parser));
        let first_message_count = first_messages
            .as_ref()
            .map(|messages| messages.lock().unwrap().len());

        ts_parser_set_logger(&mut parser, None);
        let resumed_messages = resumed_logged.then(|| install_logger(&mut parser));
        let (tree, resumed_progress) = parse(&mut parser, source.as_bytes(), None, false);
        progress.extend(resumed_progress);
        let actual = (snapshot(&tree.unwrap()), progress);
        if let Some(expected) = &expected {
            assert_eq!(&actual, expected);
        } else {
            expected = Some(actual);
        }
        if let Some(messages) = first_messages {
            assert_eq!(Some(messages.lock().unwrap().len()), first_message_count);
        }
        if let Some(messages) = resumed_messages {
            let messages = messages.lock().unwrap();
            assert!(messages.iter().any(|(_, text)| text == "resume_parsing"));
            assert!(messages.iter().any(|(_, text)| text == "done"));
        }
    }
}
