//! A lex function or external scanner that keeps advancing at the end of the input
//! never returns: the C library hangs. The port ends that parse with no tree, and the
//! parser stays usable.

use std::time::{Duration, Instant};
use tree_sitter::{Language, ParseOptions, ParseState, Parser};
use tree_sitter_language::{ExternalScanner, LanguageTables, Lexer, StateId};

/// Like the Kotlin scanner bug: skip to the next space, which never comes at the end.
struct SkipToSpace;

impl ExternalScanner for SkipToSpace {
    fn scan(&mut self, lexer: &mut dyn Lexer, _valid_symbols: &[bool]) -> bool {
        while lexer.lookahead() != i32::from(b' ') {
            lexer.advance(true);
        }
        false
    }
    fn serialize(&mut self, _buffer: &mut [u8]) -> usize {
        0
    }
    fn deserialize(&mut self, _buffer: &[u8]) {}
}

/// Advances at the end of the input a thousand times, then gives up.
struct Bounded;

impl ExternalScanner for Bounded {
    fn scan(&mut self, lexer: &mut dyn Lexer, _valid_symbols: &[bool]) -> bool {
        for _ in 0..1000 {
            lexer.advance(true);
        }
        false
    }
    fn serialize(&mut self, _buffer: &mut [u8]) -> usize {
        0
    }
    fn deserialize(&mut self, _buffer: &[u8]) {}
}

fn stuck_lex(lexer: &mut dyn Lexer, _state: StateId) -> bool {
    loop {
        lexer.advance(false);
    }
}

/// The CMake grammar with its scanner or its lex function replaced.
fn cmake_with(change: impl FnOnce(&mut LanguageTables)) -> Language {
    let mut tables = tree_sitter_cmake::language().clone();
    change(&mut tables);
    Language::from(&*Box::leak(Box::new(tables)))
}

fn set_scanner(tables: &mut LanguageTables, create: fn() -> Box<dyn ExternalScanner>) {
    tables.external_scanner.as_mut().unwrap().create = create;
}

#[test]
fn a_stalled_scanner_or_lexer_ends_the_parse_with_no_tree() {
    let cmake = Language::from(tree_sitter_cmake::language());
    let stalled = [
        cmake_with(|t| set_scanner(t, || Box::new(SkipToSpace))),
        cmake_with(|t| t.lex_fn = stuck_lex),
    ];
    let mut parser = Parser::new();
    for language in &stalled {
        for source in ["", "message(hello)", "message(hello world)\n"] {
            parser.set_language(language).unwrap();
            let start = Instant::now();
            assert!(parser.parse(source, None).is_none(), "{source:?}");

            let mut progress_calls = 0;
            let mut progress = |_: &ParseState| {
                progress_calls += 1;
                false
            };
            let parsed = parser.parse_with_options(
                &mut |offset, _| source.as_bytes().get(offset..).unwrap_or_default(),
                None,
                Some(ParseOptions::new().progress_callback(&mut progress)),
            );
            assert!(parsed.is_none(), "{source:?}");
            assert!(start.elapsed() < Duration::from_secs(10));

            // The parse was reset: the next one starts over.
            parser.set_language(&cmake).unwrap();
            let tree = parser.parse(source, None).unwrap();
            assert!(!tree.root_node().has_error(), "{source:?}");
        }
    }
}

#[test]
fn a_scanner_may_advance_at_the_end_of_the_input_a_few_times() {
    let cmake = Language::from(tree_sitter_cmake::language());
    let bounded = cmake_with(|t| set_scanner(t, || Box::new(Bounded)));
    let mut parser = Parser::new();
    for source in ["", "message(hello)", "message(hello world)\n"] {
        parser.set_language(&cmake).unwrap();
        let expected = parser.parse(source, None).unwrap().root_node().to_sexp();
        parser.set_language(&bounded).unwrap();
        let tree = parser.parse(source, None).unwrap();
        assert_eq!(tree.root_node().to_sexp(), expected, "{source:?}");
    }
}
