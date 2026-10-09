//! The JSDoc external scanner, translated from `src/scanner.c`.

use tree_sitter_language::{ExternalScanner, Lexer};

// External token indices, in the order of the C TokenType enum.
const TYPE_TOKEN: usize = 0;

/// The C scanner has no payload or persistent state.
pub(crate) struct Scanner;

/// Scans to the next balanced `}`, leaving the lexer on it. A newline, a NUL or the
/// end of the input first means there is no type.
fn scan_for_type(lexer: &mut dyn Lexer) -> bool {
    let mut stack = 0i32;
    loop {
        if lexer.eof() {
            return false;
        }
        match lexer.lookahead() {
            0x7b => stack = stack.wrapping_add(1),
            0x7d => {
                stack = stack.wrapping_sub(1);
                if stack == -1 {
                    return true;
                }
            }
            0x0a | 0 => return false,
            _ => {}
        }
        lexer.advance(false);
    }
}

impl ExternalScanner for Scanner {
    fn scan(&mut self, lexer: &mut dyn Lexer, valid_symbols: &[bool]) -> bool {
        if valid_symbols[TYPE_TOKEN] && scan_for_type(lexer) {
            lexer.set_result_symbol(TYPE_TOKEN as u16);
            lexer.mark_end();
            return true;
        }
        false
    }

    fn serialize(&mut self, _buffer: &mut [u8]) -> usize {
        0
    }

    fn deserialize(&mut self, _buffer: &[u8]) {}
}

/// Creates a scanner (C's `tree_sitter_jsdoc_external_scanner_create`).
pub(crate) fn create() -> Box<dyn ExternalScanner> {
    Box::new(Scanner)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A lexer over a string, recording where `mark_end` was called.
    struct TestLexer {
        input: Vec<i32>,
        position: usize,
        marked: Option<usize>,
        result: u16,
    }

    impl TestLexer {
        fn new(input: &str) -> Self {
            Self {
                input: input.chars().map(|c| c as i32).collect(),
                position: 0,
                marked: None,
                result: u16::MAX,
            }
        }
    }

    impl Lexer for TestLexer {
        fn lookahead(&self) -> i32 {
            self.input.get(self.position).copied().unwrap_or(0)
        }

        fn result_symbol(&self) -> u16 {
            self.result
        }

        fn set_result_symbol(&mut self, symbol: u16) {
            self.result = symbol;
        }

        fn advance(&mut self, _skip: bool) {
            if self.position < self.input.len() {
                self.position += 1;
            }
        }

        fn mark_end(&mut self) {
            self.marked = Some(self.position);
        }

        fn get_column(&mut self) -> u32 {
            0
        }

        fn is_at_included_range_start(&self) -> bool {
            false
        }

        fn eof(&self) -> bool {
            self.position >= self.input.len()
        }
    }

    fn scan(input: &str, valid: bool) -> (bool, TestLexer) {
        let mut lexer = TestLexer::new(input);
        let found = Scanner.scan(&mut lexer, &[valid]);
        (found, lexer)
    }

    #[test]
    fn type_ends_before_the_balancing_brace() {
        let (found, lexer) = scan("Array<{a: number}>} rest", true);
        assert!(found);
        assert_eq!(lexer.result, TYPE_TOKEN as u16);
        assert_eq!(lexer.marked, Some(18));
        assert_eq!(lexer.position, 18);
    }

    #[test]
    fn newline_nul_or_end_of_input_means_no_type() {
        assert!(!scan("string\n}", true).0);
        assert!(!scan("string\0}", true).0);
        assert!(!scan("{string}", true).0);
        assert_eq!(scan("string", true).1.marked, None);
    }

    #[test]
    fn only_scans_when_the_type_is_valid() {
        let (found, lexer) = scan("string}", false);
        assert!(!found);
        assert_eq!(lexer.position, 0);
    }

    #[test]
    fn serializes_nothing() {
        let mut scanner = Scanner;
        let mut buffer = [0u8; 4];
        assert_eq!(scanner.serialize(&mut buffer), 0);
    }
}
