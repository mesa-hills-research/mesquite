//! The Lua external scanner, translated from `src/scanner.c`.

use std::ffi::c_char;
use ts_port_tables::{ExternalScanner, Lexer};

const BLOCK_COMMENT_START: usize = 0;
const BLOCK_COMMENT_CONTENT: usize = 1;
const BLOCK_COMMENT_END: usize = 2;
const BLOCK_STRING_START: usize = 3;
const BLOCK_STRING_CONTENT: usize = 4;
const BLOCK_STRING_END: usize = 5;

fn consume_char(c: u8, lexer: &mut dyn Lexer) -> bool {
    if lexer.lookahead() != i32::from(c) {
        return false;
    }
    lexer.advance(false);
    true
}

fn consume_and_count_char(c: u8, lexer: &mut dyn Lexer) -> u8 {
    let mut count = 0u8;
    while lexer.lookahead() == i32::from(c) {
        count = count.wrapping_add(1);
        lexer.advance(false);
    }
    count
}

fn skip_whitespaces(lexer: &mut dyn Lexer) {
    // `iswspace` in the reference's default C locale, including vertical tab.
    while matches!(lexer.lookahead(), 0x09..=0x0d | 0x20) {
        lexer.advance(true);
    }
}

/// The scanner's state (C's `payload`).
#[derive(Default)]
pub(crate) struct Scanner {
    ending_char: c_char,
    level_count: u8,
}

impl Scanner {
    fn reset_state(&mut self) {
        self.ending_char = 0;
        self.level_count = 0;
    }

    fn scan_block_start(&mut self, lexer: &mut dyn Lexer) -> bool {
        if consume_char(b'[', lexer) {
            let level = consume_and_count_char(b'=', lexer);
            if consume_char(b'[', lexer) {
                self.level_count = level;
                return true;
            }
        }
        false
    }

    fn scan_block_end(&self, lexer: &mut dyn Lexer) -> bool {
        if consume_char(b']', lexer) {
            let level = consume_and_count_char(b'=', lexer);
            if self.level_count == level && consume_char(b']', lexer) {
                return true;
            }
        }
        false
    }

    fn scan_block_content(&self, lexer: &mut dyn Lexer) -> bool {
        while lexer.lookahead() != 0 {
            if lexer.lookahead() == i32::from(b']') {
                lexer.mark_end();
                if self.scan_block_end(lexer) {
                    return true;
                }
            } else {
                lexer.advance(false);
            }
        }
        false
    }

    fn scan_comment_start(&mut self, lexer: &mut dyn Lexer) -> bool {
        if consume_char(b'-', lexer) && consume_char(b'-', lexer) {
            lexer.mark_end();
            if self.scan_block_start(lexer) {
                lexer.mark_end();
                lexer.set_result_symbol(BLOCK_COMMENT_START as u16);
                return true;
            }
        }
        false
    }

    fn scan_comment_content(&mut self, lexer: &mut dyn Lexer) -> bool {
        if self.ending_char == 0 {
            if self.scan_block_content(lexer) {
                lexer.set_result_symbol(BLOCK_COMMENT_CONTENT as u16);
                return true;
            }
            return false;
        }

        while lexer.lookahead() != 0 {
            if lexer.lookahead() == i32::from(self.ending_char) {
                self.reset_state();
                lexer.set_result_symbol(BLOCK_COMMENT_CONTENT as u16);
                return true;
            }
            lexer.advance(false);
        }
        false
    }
}

impl ExternalScanner for Scanner {
    fn scan(&mut self, lexer: &mut dyn Lexer, valid_symbols: &[bool]) -> bool {
        // Failed attempts deliberately leave the lexer advanced for the next test.
        if valid_symbols[BLOCK_STRING_END] && self.scan_block_end(lexer) {
            self.reset_state();
            lexer.set_result_symbol(BLOCK_STRING_END as u16);
            return true;
        }

        if valid_symbols[BLOCK_STRING_CONTENT] && self.scan_block_content(lexer) {
            lexer.set_result_symbol(BLOCK_STRING_CONTENT as u16);
            return true;
        }

        if valid_symbols[BLOCK_COMMENT_END] && self.ending_char == 0 && self.scan_block_end(lexer) {
            self.reset_state();
            lexer.set_result_symbol(BLOCK_COMMENT_END as u16);
            return true;
        }

        if valid_symbols[BLOCK_COMMENT_CONTENT] && self.scan_comment_content(lexer) {
            return true;
        }

        skip_whitespaces(lexer);

        if valid_symbols[BLOCK_STRING_START] && self.scan_block_start(lexer) {
            lexer.set_result_symbol(BLOCK_STRING_START as u16);
            return true;
        }

        if valid_symbols[BLOCK_COMMENT_START] && self.scan_comment_start(lexer) {
            return true;
        }

        false
    }

    fn serialize(&mut self, buffer: &mut [u8]) -> usize {
        buffer[0] = self.ending_char as u8;
        buffer[1] = self.level_count;
        2
    }

    fn deserialize(&mut self, buffer: &[u8]) {
        // Unlike most scanners, C leaves the previous state intact on empty input,
        // and a one-byte input replaces only ending_char.
        if let Some(&ending_char) = buffer.first() {
            self.ending_char = ending_char as c_char;
        }
        if let Some(&level_count) = buffer.get(1) {
            self.level_count = level_count;
        }
    }
}

/// Creates a scanner (C's `tree_sitter_lua_external_scanner_create`).
pub(crate) fn create() -> Box<dyn ExternalScanner> {
    Box::<Scanner>::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Eq)]
    enum Event {
        Advance(bool),
        MarkEnd(usize),
        Symbol(u16),
    }

    struct TestLexer {
        input: Vec<i32>,
        position: usize,
        symbol: u16,
        events: Vec<Event>,
    }

    impl TestLexer {
        fn new(input: &str) -> Self {
            Self {
                input: input.chars().map(|c| c as i32).collect(),
                position: 0,
                symbol: u16::MAX,
                events: Vec::new(),
            }
        }

        fn marks(&self) -> Vec<usize> {
            self.events
                .iter()
                .filter_map(|event| match event {
                    Event::MarkEnd(position) => Some(*position),
                    _ => None,
                })
                .collect()
        }
    }

    impl Lexer for TestLexer {
        fn lookahead(&self) -> i32 {
            self.input.get(self.position).copied().unwrap_or(0)
        }

        fn result_symbol(&self) -> u16 {
            self.symbol
        }

        fn set_result_symbol(&mut self, symbol: u16) {
            self.symbol = symbol;
            self.events.push(Event::Symbol(symbol));
        }

        fn advance(&mut self, skip: bool) {
            assert!(self.position < self.input.len());
            self.position += 1;
            self.events.push(Event::Advance(skip));
        }

        fn mark_end(&mut self) {
            self.events.push(Event::MarkEnd(self.position));
        }

        fn get_column(&mut self) -> u32 {
            panic!("Lua scanner must not query the column")
        }

        fn is_at_included_range_start(&self) -> bool {
            panic!("Lua scanner must not query included ranges")
        }

        fn eof(&self) -> bool {
            panic!("Lua scanner tests lookahead, not eof")
        }
    }

    fn valid(tokens: &[usize]) -> [bool; 6] {
        let mut result = [false; 6];
        for &token in tokens {
            result[token] = true;
        }
        result
    }

    fn state(scanner: &mut dyn ExternalScanner) -> [u8; 2] {
        let mut bytes = [0; 3];
        assert_eq!(scanner.serialize(&mut bytes), 2);
        assert_eq!(bytes[2], 0);
        [bytes[0], bytes[1]]
    }

    #[test]
    fn serialization_preserves_partial_state() {
        let mut scanner = create();
        assert_eq!(state(scanner.as_mut()), [0, 0]);
        scanner.deserialize(&[0xff, 0xfe]);
        assert_eq!(state(scanner.as_mut()), [0xff, 0xfe]);
        scanner.deserialize(&[]);
        assert_eq!(state(scanner.as_mut()), [0xff, 0xfe]);
        scanner.deserialize(b"\n");
        assert_eq!(state(scanner.as_mut()), [b'\n', 0xfe]);
        scanner.deserialize(&[0, 3, 7]);
        assert_eq!(state(scanner.as_mut()), [0, 3]);
    }

    #[test]
    fn comment_start_marks_both_prefix_and_delimiter() {
        let mut scanner = Scanner::default();
        let mut lexer = TestLexer::new(" \u{b}--[=[body");
        assert!(scanner.scan(&mut lexer, &valid(&[BLOCK_COMMENT_START])));
        assert_eq!(state(&mut scanner), [0, 1]);
        assert_eq!(lexer.position, 7);
        assert_eq!(
            lexer.events,
            [
                Event::Advance(true),
                Event::Advance(true),
                Event::Advance(false),
                Event::Advance(false),
                Event::MarkEnd(4),
                Event::Advance(false),
                Event::Advance(false),
                Event::Advance(false),
                Event::MarkEnd(7),
                Event::Symbol(BLOCK_COMMENT_START as u16),
            ]
        );
    }

    #[test]
    fn whitespace_uses_c_locale_even_without_valid_tokens() {
        let mut scanner = Scanner::default();
        let mut lexer = TestLexer::new("\t\n\u{b}\u{c}\r \u{a0}[[");
        assert!(!scanner.scan(&mut lexer, &valid(&[])));
        assert_eq!(lexer.position, 6);
        assert_eq!(
            lexer.events,
            (0..6).map(|_| Event::Advance(true)).collect::<Vec<_>>()
        );
        assert!(!scanner.scan(&mut lexer, &valid(&[BLOCK_STRING_START])));
        assert_eq!(lexer.position, 6);
    }

    #[test]
    fn delimiter_levels_wrap_at_one_byte() {
        let mut scanner = Scanner::default();
        let mut lexer = TestLexer::new(&format!("[{}[", "=".repeat(257)));
        assert!(scanner.scan(&mut lexer, &valid(&[BLOCK_STRING_START])));
        assert_eq!(state(&mut scanner), [0, 1]);
        assert_eq!(lexer.position, 259);
        assert!(lexer.marks().is_empty());
        assert_eq!(lexer.symbol, BLOCK_STRING_START as u16);

        let mut lexer = TestLexer::new("]=]tail");
        assert!(scanner.scan(&mut lexer, &valid(&[BLOCK_STRING_END])));
        assert_eq!(lexer.position, 3);
        assert_eq!(lexer.symbol, BLOCK_STRING_END as u16);
        assert_eq!(state(&mut scanner), [0, 0]);

        scanner.deserialize(&[0, 1]);
        let mut lexer = TestLexer::new(&format!("]{}]", "=".repeat(257)));
        assert!(scanner.scan(&mut lexer, &valid(&[BLOCK_COMMENT_END])));
        assert_eq!(lexer.position, 259);
        assert_eq!(lexer.symbol, BLOCK_COMMENT_END as u16);
        assert_eq!(state(&mut scanner), [0, 0]);
    }

    #[test]
    fn content_marks_candidates_and_leaves_closing_delimiter_outside_token() {
        for token in [BLOCK_STRING_CONTENT, BLOCK_COMMENT_CONTENT] {
            let mut scanner = Scanner::default();
            scanner.deserialize(&[0, 1]);
            let mut lexer = TestLexer::new("a]==]b]=]tail");
            assert!(scanner.scan(&mut lexer, &valid(&[token])));
            assert_eq!(lexer.position, 9);
            assert_eq!(lexer.marks(), [1, 4, 6]);
            assert_eq!(lexer.symbol, token as u16);
            assert_eq!(state(&mut scanner), [0, 1]);
        }
    }

    #[test]
    fn failed_end_attempt_does_not_rewind_before_content() {
        let mut scanner = Scanner::default();
        scanner.deserialize(&[0, 1]);
        let mut lexer = TestLexer::new("]==]text]=]");
        assert!(scanner.scan(
            &mut lexer,
            &valid(&[BLOCK_STRING_END, BLOCK_STRING_CONTENT])
        ));
        assert_eq!(lexer.symbol, BLOCK_STRING_CONTENT as u16);
        // The failed end scan consumed the first ']' and both '=' characters.
        assert_eq!(lexer.marks(), [3, 8]);
        assert_eq!(lexer.position, 11);
        assert_eq!(state(&mut scanner), [0, 1]);
    }

    #[test]
    fn end_tokens_take_precedence_and_reset_state() {
        let mut scanner = Scanner::default();
        scanner.deserialize(&[b'\n', 1]);
        let mut lexer = TestLexer::new("]=]");
        assert!(scanner.scan(&mut lexer, &[true; 6]));
        assert_eq!(lexer.symbol, BLOCK_STRING_END as u16);
        assert_eq!(state(&mut scanner), [0, 0]);
        assert!(lexer.marks().is_empty());
    }

    #[test]
    fn nonzero_comment_ending_is_not_consumed() {
        let mut scanner = Scanner::default();
        scanner.deserialize(&[b'\n', 7]);
        let mut lexer = TestLexer::new("]]body\nrest");
        assert!(scanner.scan(
            &mut lexer,
            &valid(&[BLOCK_COMMENT_END, BLOCK_COMMENT_CONTENT])
        ));
        assert_eq!(lexer.position, 6);
        assert_eq!(lexer.lookahead(), i32::from(b'\n'));
        assert_eq!(lexer.symbol, BLOCK_COMMENT_CONTENT as u16);
        assert!(lexer.marks().is_empty());
        assert_eq!(state(&mut scanner), [0, 0]);
    }

    #[test]
    fn unterminated_content_stops_at_eof_or_nul() {
        for input in ["body]", "body\0]]"] {
            let mut scanner = Scanner::default();
            let mut lexer = TestLexer::new(input);
            assert!(!scanner.scan(&mut lexer, &valid(&[BLOCK_COMMENT_CONTENT])));
            assert_eq!(lexer.lookahead(), 0);
            assert_eq!(lexer.symbol, u16::MAX);
            assert_eq!(state(&mut scanner), [0, 0]);
        }
    }

    #[test]
    fn failed_comment_start_preserves_level_and_prefix_mark() {
        let mut scanner = Scanner::default();
        scanner.deserialize(&[0, 3]);
        let mut lexer = TestLexer::new("--[==x");
        assert!(!scanner.scan(&mut lexer, &valid(&[BLOCK_COMMENT_START])));
        assert_eq!(lexer.position, 5);
        assert_eq!(lexer.marks(), [2]);
        assert_eq!(lexer.symbol, u16::MAX);
        assert_eq!(state(&mut scanner), [0, 3]);
    }
}
