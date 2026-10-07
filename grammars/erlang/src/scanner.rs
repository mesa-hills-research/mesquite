//! Erlang's stateless triple-quoted string scanner, translated from `scanner.c`.

use tree_sitter_language::{ExternalScanner, Lexer};

const TQ_STRING: u16 = 0;
const TQ_SIGIL_STRING: u16 = 1;
// External token 2 is ERROR_SENTINEL; the C scanner does not inspect it.

/// The C scanner has no payload.
pub(crate) struct Scanner;

fn is_whitespace(lookahead: i32) -> bool {
    // Erlang's WHITE_SPACE macro, not Unicode or C-locale whitespace.
    matches!(lookahead, 0x01..=0x20 | 0x80..=0xa0)
}

impl ExternalScanner for Scanner {
    fn scan(&mut self, lexer: &mut dyn Lexer, valid_symbols: &[bool]) -> bool {
        if !valid_symbols[TQ_STRING as usize] && !valid_symbols[TQ_SIGIL_STRING as usize] {
            return false;
        }

        while is_whitespace(lexer.lookahead()) {
            lexer.advance(true);
        }

        let mut is_sigil_string = false;
        if valid_symbols[TQ_SIGIL_STRING as usize] && lexer.lookahead() == '~' as i32 {
            is_sigil_string = true;
            lexer.advance(false);
            match lexer.lookahead() {
                0x73 | 0x53 | 0x62 | 0x42 => lexer.advance(false), // s, S, b, B
                0x22 => {}                                         // "
                _ => return false,
            }
        }

        for _ in 0..3 {
            if lexer.lookahead() != '"' as i32 {
                return false;
            }
            lexer.advance(false);
        }

        let mut delimiter_count = 3u16;
        while lexer.lookahead() == '"' as i32 {
            delimiter_count = delimiter_count.wrapping_add(1);
            lexer.advance(false);
        }
        while lexer.lookahead() != '\n' as i32 && is_whitespace(lexer.lookahead()) {
            lexer.advance(false);
        }
        if lexer.lookahead() != '\n' as i32 {
            return false;
        }
        lexer.advance(false);

        loop {
            // As in C, the opening newline has already been consumed: a closing
            // delimiter is considered only after another newline in the body.
            if lexer.lookahead() == '\n' as i32 {
                lexer.advance(false);
                while lexer.lookahead() != '\n' as i32 && is_whitespace(lexer.lookahead()) {
                    lexer.advance(false);
                }

                let mut check = delimiter_count;
                while check > 0 {
                    if lexer.lookahead() != '"' as i32 {
                        break;
                    }
                    lexer.advance(false);
                    check -= 1;
                }
                // C accepts the required number of quotes without checking the
                // following character, even if it is another quote.
                if check == 0 {
                    lexer.mark_end();
                    lexer.set_result_symbol(if is_sigil_string {
                        TQ_SIGIL_STRING
                    } else {
                        TQ_STRING
                    });
                    return true;
                }
            } else if lexer.eof() {
                return false;
            } else {
                lexer.advance(false);
            }
        }
    }

    fn serialize(&mut self, _buffer: &mut [u8]) -> usize {
        0
    }

    fn deserialize(&mut self, _buffer: &[u8]) {}
}

/// Creates a scanner (C's `tree_sitter_erlang_external_scanner_create`).
pub(crate) fn create() -> Box<dyn ExternalScanner> {
    Box::new(Scanner)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    #[derive(Debug, PartialEq, Eq)]
    enum Call {
        Advance(bool),
        Eof,
        MarkEnd,
        Result(u16),
    }

    struct TestLexer {
        input: Vec<char>,
        position: usize,
        end: Option<usize>,
        symbol: u16,
        calls: RefCell<Vec<Call>>,
    }

    impl TestLexer {
        fn new(input: &str) -> Self {
            Self {
                input: input.chars().collect(),
                position: 0,
                end: None,
                symbol: u16::MAX,
                calls: RefCell::new(Vec::new()),
            }
        }
    }

    impl Lexer for TestLexer {
        fn lookahead(&self) -> i32 {
            self.input.get(self.position).copied().unwrap_or('\0') as i32
        }

        fn result_symbol(&self) -> u16 {
            self.symbol
        }

        fn set_result_symbol(&mut self, symbol: u16) {
            self.calls.get_mut().push(Call::Result(symbol));
            self.symbol = symbol;
        }

        fn advance(&mut self, skip: bool) {
            self.calls.get_mut().push(Call::Advance(skip));
            assert!(self.position < self.input.len());
            self.position += 1;
        }

        fn mark_end(&mut self) {
            self.calls.get_mut().push(Call::MarkEnd);
            self.end = Some(self.position);
        }

        fn get_column(&mut self) -> u32 {
            panic!("the scanner does not request columns")
        }

        fn is_at_included_range_start(&self) -> bool {
            panic!("the scanner does not inspect included ranges")
        }

        fn eof(&self) -> bool {
            self.calls.borrow_mut().push(Call::Eof);
            self.position == self.input.len()
        }
    }

    #[test]
    fn callback_order_and_token_boundary() {
        let mut lexer = TestLexer::new("\t\"\"\"\nx\n \"\"\"\"");
        assert!(Scanner.scan(&mut lexer, &[true, false, false]));
        assert_eq!(lexer.end, Some(lexer.input.len() - 1));
        assert_eq!(lexer.lookahead(), '"' as i32);
        assert_eq!(lexer.result_symbol(), TQ_STRING);
        let mut expected = vec![Call::Advance(true)];
        expected.extend((0..4).map(|_| Call::Advance(false)));
        expected.push(Call::Eof);
        expected.extend((0..6).map(|_| Call::Advance(false)));
        expected.extend([Call::MarkEnd, Call::Result(TQ_STRING)]);
        assert_eq!(*lexer.calls.borrow(), expected);
    }

    #[test]
    fn sigil_prefixes() {
        for prefix in ["~", "~s", "~S", "~b", "~B"] {
            let input = format!("{prefix}\"\"\"\ntext\n\"\"\"");
            let mut lexer = TestLexer::new(&input);
            assert!(Scanner.scan(&mut lexer, &[false, true, false]));
            assert_eq!(lexer.result_symbol(), TQ_SIGIL_STRING);
            assert_eq!(lexer.end, Some(lexer.input.len()));
        }
        let mut lexer = TestLexer::new("~x\"\"\"\ntext\n\"\"\"");
        assert!(!Scanner.scan(&mut lexer, &[false, true, false]));
        assert_eq!(lexer.position, 1);
        assert_eq!(lexer.end, None);
    }

    #[test]
    fn validity_and_error_sentinel_match_c() {
        let input = "\"\"\"\ntext\n\"\"\"";
        for valid in [[true, false, true], [false, true, false]] {
            // The sentinel does not disable scanning, and a bare string can be
            // emitted even when only the sigil-string token is valid.
            let mut lexer = TestLexer::new(input);
            assert!(Scanner.scan(&mut lexer, &valid));
            assert_eq!(lexer.result_symbol(), TQ_STRING);
        }
        let mut lexer = TestLexer::new(input);
        assert!(!Scanner.scan(&mut lexer, &[false, false, true]));
        assert!(lexer.calls.borrow().is_empty());

        let mut lexer = TestLexer::new("~\"\"\"\ntext\n\"\"\"");
        assert!(!Scanner.scan(&mut lexer, &[true, false, false]));
        assert!(lexer.calls.borrow().is_empty());
    }

    #[test]
    fn opening_delimiter_requires_three_quotes_and_newline() {
        for (input, consumed) in [
            ("", 0),
            ("\"", 1),
            ("\"\"x", 2),
            ("\"\"\"x", 3),
            ("\"\"\" \r", 5),
        ] {
            let mut lexer = TestLexer::new(input);
            assert!(!Scanner.scan(&mut lexer, &[true, true, false]));
            assert_eq!(lexer.position, consumed);
            assert_eq!(lexer.end, None);
            assert_eq!(lexer.result_symbol(), u16::MAX);
            assert_eq!(
                *lexer.calls.borrow(),
                (0..consumed)
                    .map(|_| Call::Advance(false))
                    .collect::<Vec<_>>()
            );
        }
    }

    #[test]
    fn closing_delimiter_is_not_checked_on_first_body_line() {
        let mut lexer = TestLexer::new("\"\"\"\n\"\"\"");
        assert!(!Scanner.scan(&mut lexer, &[true, false, false]));
        assert_eq!(lexer.position, lexer.input.len());
        assert_eq!(lexer.end, None);
        assert_eq!(lexer.calls.borrow().last(), Some(&Call::Eof));

        let mut lexer = TestLexer::new("\"\"\"\n\"\"\"\n\"\"\"");
        assert!(Scanner.scan(&mut lexer, &[true, false, false]));
        assert_eq!(lexer.end, Some(lexer.input.len()));
    }

    #[test]
    fn short_closing_delimiters_are_content() {
        let mut lexer = TestLexer::new("\"\"\"\"\nbody\n\"\"\"\n\"\"\"\"\"suffix");
        assert!(Scanner.scan(&mut lexer, &[true, false, false]));
        assert_eq!(lexer.end, Some(lexer.input.len() - "\"suffix".len()));
    }

    #[test]
    fn erlang_whitespace_and_embedded_nul() {
        for codepoint in 0..=0x100 {
            assert_eq!(
                is_whitespace(codepoint),
                (1..=32).contains(&codepoint) || (128..=160).contains(&codepoint)
            );
        }
        assert!(!is_whitespace(0x2003));
        assert!(!is_whitespace(-1));
        let mut lexer = TestLexer::new("\u{1}\u{80}\u{a0}\"\"\"\r\n\0\n\u{80}\"\"\"");
        assert!(Scanner.scan(&mut lexer, &[true, false, false]));
        assert_eq!(lexer.end, Some(lexer.input.len()));
        assert_eq!(
            &lexer.calls.borrow()[..3],
            &[
                Call::Advance(true),
                Call::Advance(true),
                Call::Advance(true)
            ]
        );
    }

    #[test]
    fn delimiter_count_wraps_as_u16() {
        let input = format!("{}\nx\nrest", "\"".repeat(65536));
        let mut lexer = TestLexer::new(&input);
        assert!(Scanner.scan(&mut lexer, &[true, false, false]));
        assert_eq!(lexer.end, Some(65539));
        assert_eq!(lexer.lookahead(), 'r' as i32);
    }

    #[test]
    fn stateless_serialization_does_not_touch_buffer() {
        let mut scanner = create();
        let mut buffer = [0xaa; 1024];
        assert_eq!(scanner.serialize(&mut buffer), 0);
        assert_eq!(buffer, [0xaa; 1024]);
        scanner.deserialize(&[1, 2, 3]);
        scanner.deserialize(&[]);
        assert_eq!(scanner.serialize(&mut []), 0);
    }
}
