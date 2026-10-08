//! The PowerShell external scanner, translated from `src/scanner.c`.

use tree_sitter_language::{ExternalScanner, Lexer};

const STATEMENT_TERMINATOR: u16 = 0;

/// The C scanner has no payload or persistent state.
pub(crate) struct Scanner;

impl ExternalScanner for Scanner {
    fn scan(&mut self, lexer: &mut dyn Lexer, valid_symbols: &[bool]) -> bool {
        if !valid_symbols[usize::from(STATEMENT_TERMINATOR)] {
            return false;
        }

        lexer.set_result_symbol(STATEMENT_TERMINATOR);
        // This token has no characters: all advancement is only lookahead.
        lexer.mark_end();

        loop {
            match lexer.lookahead() {
                // NUL (including EOF), '}', ';', ')', or newline.
                0 | 0x7d | 0x3b | 0x29 | 0x0a => return true,
                // iswspace in C's default locale. Newline returns above;
                // notably, CR alone is skipped rather than ending a statement.
                0x09..=0x0d | 0x20 => lexer.advance(true),
                _ => return false,
            }
        }
    }

    fn serialize(&mut self, _buffer: &mut [u8]) -> usize {
        0
    }

    fn deserialize(&mut self, _buffer: &[u8]) {}
}

/// Creates a scanner (C's `tree_sitter_powershell_external_scanner_create`).
pub(crate) fn create() -> Box<dyn ExternalScanner> {
    Box::new(Scanner)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Eq)]
    enum Call {
        SetSymbol(u16),
        MarkEnd(usize),
        Advance(bool),
    }

    struct TestLexer {
        input: Vec<i32>,
        position: usize,
        symbol: u16,
        calls: Vec<Call>,
    }

    impl TestLexer {
        fn new(input: &str) -> Self {
            Self {
                input: input.chars().map(|c| c as i32).collect(),
                position: 0,
                symbol: u16::MAX,
                calls: Vec::new(),
            }
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
            self.calls.push(Call::SetSymbol(symbol));
        }

        fn advance(&mut self, skip: bool) {
            assert!(self.position < self.input.len());
            self.calls.push(Call::Advance(skip));
            self.position += 1;
        }

        fn mark_end(&mut self) {
            self.calls.push(Call::MarkEnd(self.position));
        }

        fn get_column(&mut self) -> u32 {
            panic!("the C scanner never calls get_column")
        }

        fn is_at_included_range_start(&self) -> bool {
            panic!("the C scanner never checks included range starts")
        }

        fn eof(&self) -> bool {
            panic!("the C scanner checks NUL lookahead, not eof")
        }
    }

    #[test]
    fn invalid_symbol_does_not_touch_lexer() {
        let mut lexer = TestLexer::new(" \n");
        assert!(!create().scan(&mut lexer, &[false]));
        assert!(lexer.calls.is_empty());
        assert_eq!(lexer.position, 0);
        assert_eq!(lexer.result_symbol(), u16::MAX);
    }

    #[test]
    fn terminators_are_zero_width_and_not_consumed() {
        for input in ["", "\0tail", "}tail", ";tail", ")tail", "\ntail"] {
            let mut lexer = TestLexer::new(input);
            assert!(create().scan(&mut lexer, &[true]), "{input:?}");
            assert_eq!(lexer.position, 0);
            assert_eq!(lexer.calls, [Call::SetSymbol(0), Call::MarkEnd(0)]);
        }
    }

    #[test]
    fn whitespace_is_lookahead_after_marking_the_empty_token() {
        for terminator in ["", "\0tail", "}", ";", ")", "\n"] {
            let mut lexer = TestLexer::new(&format!(" \t\u{b}\u{c}\r{terminator}"));
            assert!(create().scan(&mut lexer, &[true]));
            assert_eq!(lexer.position, 5);
            assert_eq!(
                lexer.calls,
                [
                    Call::SetSymbol(0),
                    Call::MarkEnd(0),
                    Call::Advance(true),
                    Call::Advance(true),
                    Call::Advance(true),
                    Call::Advance(true),
                    Call::Advance(true),
                ]
            );
        }
    }

    #[test]
    fn nonterminators_fail_even_after_carriage_return() {
        // Unicode whitespace is not iswspace in C's default locale.
        for tail in [
            "x", "{", "(", "#", "\u{85}", "\u{a0}", "\u{2003}", "\u{2028}",
        ] {
            let mut lexer = TestLexer::new(&format!("\r{tail}\n"));
            assert!(!create().scan(&mut lexer, &[true]), "{tail:?}");
            assert_eq!(lexer.position, 1);
            assert_eq!(
                lexer.calls,
                [Call::SetSymbol(0), Call::MarkEnd(0), Call::Advance(true)]
            );
        }
    }

    #[test]
    fn scanner_has_no_serialized_state() {
        let mut scanner = create();
        let mut buffer = [0xa5; 16];
        for state in [&[][..], &[1, 2, 3][..]] {
            scanner.deserialize(state);
            assert!(scanner.scan(&mut TestLexer::new(";"), &[true]));
            assert_eq!(scanner.serialize(&mut buffer), 0);
            assert_eq!(buffer, [0xa5; 16]);
            assert_eq!(scanner.serialize(&mut []), 0);
        }
    }
}
