//! Julia's stateless external scanner, translated from `scanner.c`.

use ts_port_tables::{ExternalScanner, Lexer, Symbol};

// TokenType indices from the C scanner.
const BLOCK_COMMENT_REST: usize = 0;
const IMMEDIATE_PAREN: usize = 1;
const IMMEDIATE_BRACKET: usize = 2;
const IMMEDIATE_BRACE: usize = 3;
const IMMEDIATE_STRING_START: usize = 4;
const IMMEDIATE_COMMAND_START: usize = 5;
const CONTENT_CMD_1: usize = 6;
const CONTENT_CMD_1_RAW: usize = 7;
const CONTENT_CMD_3: usize = 8;
const CONTENT_CMD_3_RAW: usize = 9;
const CONTENT_STR_1: usize = 10;
const CONTENT_STR_1_RAW: usize = 11;
const CONTENT_STR_3: usize = 12;
const CONTENT_STR_3_RAW: usize = 13;
const END_CMD: usize = 14;
const END_STR: usize = 15;

fn scan_content(
    lexer: &mut dyn Lexer,
    content_symbol: Symbol,
    end_char: i32,
    n_delim: u32,
    interp: bool,
) -> bool {
    let end_symbol = if end_char == i32::from(b'"') {
        END_STR
    } else {
        END_CMD
    };
    let mut has_content = false;
    loop {
        let next = lexer.lookahead();
        if next == 0 {
            return false;
        }
        lexer.mark_end();
        if interp && (next == i32::from(b'$') || next == i32::from(b'\\')) {
            lexer.set_result_symbol(content_symbol);
            return has_content;
        } else if next == i32::from(b'\\') {
            // Raw strings still leave escaped delimiters and backslashes to
            // the internal lexer. The mark excludes this consumed backslash.
            lexer.advance(false);
            let next = lexer.lookahead();
            if next == end_char || next == i32::from(b'\\') {
                lexer.set_result_symbol(content_symbol);
                return has_content;
            }
        } else {
            let mut is_end_delimiter = true;
            for _ in 0..n_delim {
                if lexer.lookahead() == end_char {
                    lexer.advance(false);
                } else {
                    is_end_delimiter = false;
                    break;
                }
            }
            if is_end_delimiter {
                if has_content {
                    lexer.set_result_symbol(content_symbol);
                } else {
                    lexer.mark_end();
                    lexer.set_result_symbol(end_symbol as Symbol);
                }
                return true;
            }
        }
        // C also consumes the character after an incomplete triple delimiter
        // here, even if that character is '$', a backslash, or EOF.
        lexer.advance(false);
        has_content = true;
    }
}

fn scan_block_comment(lexer: &mut dyn Lexer) -> bool {
    // The internal lexer has already consumed the opening '#='.
    let mut after_eq = false;
    let mut nesting_depth = 1u32;
    loop {
        match lexer.lookahead() {
            0x3d => {
                // '='
                lexer.advance(false);
                after_eq = true;
            }
            0x23 => {
                // '#'
                lexer.advance(false);
                if after_eq {
                    after_eq = false;
                    nesting_depth = nesting_depth.wrapping_sub(1);
                    if nesting_depth == 0 {
                        lexer.set_result_symbol(BLOCK_COMMENT_REST as Symbol);
                        return true;
                    }
                } else {
                    after_eq = false;
                    if lexer.lookahead() == i32::from(b'=') {
                        lexer.advance(false);
                        nesting_depth = nesting_depth.wrapping_add(1);
                    }
                }
            }
            0 => return false,
            _ => {
                lexer.advance(false);
                after_eq = false;
            }
        }
    }
}

/// No payload is allocated by the C scanner.
pub(crate) struct Scanner;

impl ExternalScanner for Scanner {
    fn scan(&mut self, lexer: &mut dyn Lexer, valid_symbols: &[bool]) -> bool {
        for (symbol, delimiter) in [
            (IMMEDIATE_PAREN, b'('),
            (IMMEDIATE_BRACKET, b'['),
            (IMMEDIATE_BRACE, b'{'),
            (IMMEDIATE_STRING_START, b'"'),
            (IMMEDIATE_COMMAND_START, b'`'),
        ] {
            if valid_symbols[symbol] && lexer.lookahead() == i32::from(delimiter) {
                // These are zero-width tokens: do not advance or mark_end.
                lexer.set_result_symbol(symbol as Symbol);
                return true;
            }
        }

        if valid_symbols[BLOCK_COMMENT_REST] && scan_block_comment(lexer) {
            return true;
        }

        // Keep C's order, including continuing from the position and mark left
        // by a failed attempt when more than one content token is valid.
        for (symbol, delimiter, count, interp) in [
            (CONTENT_STR_1, b'"', 1, true),
            (CONTENT_STR_3, b'"', 3, true),
            (CONTENT_CMD_1, b'`', 1, true),
            (CONTENT_CMD_3, b'`', 3, true),
            (CONTENT_STR_1_RAW, b'"', 1, false),
            (CONTENT_STR_3_RAW, b'"', 3, false),
            (CONTENT_CMD_1_RAW, b'`', 1, false),
            (CONTENT_CMD_3_RAW, b'`', 3, false),
        ] {
            if valid_symbols[symbol]
                && scan_content(lexer, symbol as Symbol, i32::from(delimiter), count, interp)
            {
                return true;
            }
        }
        false
    }

    fn serialize(&mut self, _buffer: &mut [u8]) -> usize {
        0
    }

    fn deserialize(&mut self, _buffer: &[u8]) {}
}

/// Creates a scanner (C's `tree_sitter_julia_external_scanner_create`).
pub(crate) fn create() -> Box<dyn ExternalScanner> {
    Box::new(Scanner)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Eq)]
    enum Call {
        Advance(usize),
        Mark(usize),
        Symbol(Symbol),
    }

    struct TestLexer {
        input: Vec<i32>,
        position: usize,
        symbol: Symbol,
        calls: Vec<Call>,
    }

    impl TestLexer {
        fn new(input: &str) -> Self {
            Self {
                input: input.chars().map(|c| c as i32).collect(),
                position: 0,
                symbol: Symbol::MAX,
                calls: Vec::new(),
            }
        }
    }

    impl Lexer for TestLexer {
        fn lookahead(&self) -> i32 {
            self.input.get(self.position).copied().unwrap_or(0)
        }

        fn result_symbol(&self) -> Symbol {
            self.symbol
        }

        fn set_result_symbol(&mut self, symbol: Symbol) {
            self.symbol = symbol;
            self.calls.push(Call::Symbol(symbol));
        }

        fn advance(&mut self, skip: bool) {
            assert!(!skip, "Julia's scanner never skips whitespace");
            self.calls.push(Call::Advance(self.position));
            if self.position < self.input.len() {
                self.position += 1;
            }
        }

        fn mark_end(&mut self) {
            self.calls.push(Call::Mark(self.position));
        }

        fn get_column(&mut self) -> u32 {
            panic!("the C scanner does not call get_column")
        }

        fn is_at_included_range_start(&self) -> bool {
            panic!("the C scanner does not inspect included ranges")
        }

        fn eof(&self) -> bool {
            panic!("the C scanner tests lookahead == 0, not eof")
        }
    }

    fn scan(input: &str, valid: &[usize]) -> (bool, TestLexer) {
        let mut lexer = TestLexer::new(input);
        let mut valid_symbols = [false; 16];
        for &symbol in valid {
            valid_symbols[symbol] = true;
        }
        let accepted = Scanner.scan(&mut lexer, &valid_symbols);
        (accepted, lexer)
    }

    #[test]
    fn immediate_tokens_are_zero_width_and_take_priority() {
        for (symbol, input) in [
            (IMMEDIATE_PAREN, "("),
            (IMMEDIATE_BRACKET, "["),
            (IMMEDIATE_BRACE, "{"),
            (IMMEDIATE_STRING_START, "\""),
            (IMMEDIATE_COMMAND_START, "`"),
        ] {
            let mut lexer = TestLexer::new(input);
            assert!(Scanner.scan(&mut lexer, &[true; 16]));
            assert_eq!(lexer.calls, [Call::Symbol(symbol as Symbol)]);
            assert_eq!(lexer.position, 0);

            let (accepted, lexer) = scan(&format!(" {input}"), &[symbol]);
            assert!(!accepted);
            assert!(lexer.calls.is_empty());
        }
    }

    #[test]
    fn all_content_variants_preserve_delimiter_marks() {
        for (symbol, delimiter, count, end_symbol) in [
            (CONTENT_STR_1, '"', 1, END_STR),
            (CONTENT_STR_3, '"', 3, END_STR),
            (CONTENT_CMD_1, '`', 1, END_CMD),
            (CONTENT_CMD_3, '`', 3, END_CMD),
            (CONTENT_STR_1_RAW, '"', 1, END_STR),
            (CONTENT_STR_3_RAW, '"', 3, END_STR),
            (CONTENT_CMD_1_RAW, '`', 1, END_CMD),
            (CONTENT_CMD_3_RAW, '`', 3, END_CMD),
        ] {
            let delimiter = delimiter.to_string().repeat(count);
            // End-token validity is deliberately not required by C.
            let (accepted, lexer) = scan(&delimiter, &[symbol]);
            assert!(accepted);
            let mut expected = vec![Call::Mark(0)];
            expected.extend((0..count).map(Call::Advance));
            expected.push(Call::Mark(count));
            expected.push(Call::Symbol(end_symbol as Symbol));
            assert_eq!(lexer.calls, expected);

            let (accepted, lexer) = scan(&format!("α{delimiter}"), &[symbol]);
            assert!(accepted);
            let mut expected = vec![Call::Mark(0), Call::Advance(0), Call::Mark(1)];
            expected.extend((1..=count).map(Call::Advance));
            expected.push(Call::Symbol(symbol as Symbol));
            assert_eq!(lexer.calls, expected);
        }
    }

    #[test]
    fn interpolation_and_escape_stop_before_the_special_character() {
        for symbol in [CONTENT_STR_1, CONTENT_STR_3, CONTENT_CMD_1, CONTENT_CMD_3] {
            for special in ["$", "\\"] {
                let (accepted, lexer) = scan(special, &[symbol]);
                assert!(!accepted);
                assert_eq!(lexer.calls, [Call::Mark(0), Call::Symbol(symbol as Symbol)]);

                let (accepted, lexer) = scan(&format!("x{special}"), &[symbol]);
                assert!(accepted);
                assert_eq!(
                    lexer.calls,
                    [
                        Call::Mark(0),
                        Call::Advance(0),
                        Call::Mark(1),
                        Call::Symbol(symbol as Symbol),
                    ]
                );
            }
        }
    }

    #[test]
    fn raw_escape_inspects_one_character_past_the_mark() {
        for (symbol, delimiter) in [
            (CONTENT_STR_1_RAW, '"'),
            (CONTENT_STR_3_RAW, '"'),
            (CONTENT_CMD_1_RAW, '`'),
            (CONTENT_CMD_3_RAW, '`'),
        ] {
            for escaped in [delimiter, '\\'] {
                let (accepted, lexer) = scan(&format!("\\{escaped}"), &[symbol]);
                assert!(!accepted);
                assert_eq!(
                    lexer.calls,
                    [
                        Call::Mark(0),
                        Call::Advance(0),
                        Call::Symbol(symbol as Symbol)
                    ]
                );
                let (accepted, lexer) = scan(&format!("x\\{escaped}"), &[symbol]);
                assert!(accepted);
                assert_eq!(
                    lexer.calls,
                    [
                        Call::Mark(0),
                        Call::Advance(0),
                        Call::Mark(1),
                        Call::Advance(1),
                        Call::Symbol(symbol as Symbol),
                    ]
                );
            }
        }
    }

    #[test]
    fn raw_unrecognized_escapes_and_dollars_are_content() {
        let (accepted, lexer) = scan("\\q$\"", &[CONTENT_STR_1_RAW]);
        assert!(accepted);
        assert_eq!(
            lexer.calls,
            [
                Call::Mark(0),
                Call::Advance(0),
                Call::Advance(1),
                Call::Mark(2),
                Call::Advance(2),
                Call::Mark(3),
                Call::Advance(3),
                Call::Symbol(CONTENT_STR_1_RAW as Symbol),
            ]
        );
    }

    #[test]
    fn incomplete_triple_delimiter_consumes_the_next_character() {
        for special in ['$', '\\'] {
            let (accepted, lexer) = scan(&format!("\"\"{special}\"\"\""), &[CONTENT_STR_3]);
            assert!(accepted);
            assert_eq!(
                lexer.calls,
                [
                    Call::Mark(0),
                    Call::Advance(0),
                    Call::Advance(1),
                    Call::Advance(2),
                    Call::Mark(3),
                    Call::Advance(3),
                    Call::Advance(4),
                    Call::Advance(5),
                    Call::Symbol(CONTENT_STR_3 as Symbol),
                ]
            );
        }
        let (accepted, lexer) = scan("\"\"", &[CONTENT_STR_3]);
        assert!(!accepted);
        assert_eq!(
            lexer.calls,
            [
                Call::Mark(0),
                Call::Advance(0),
                Call::Advance(1),
                Call::Advance(2)
            ]
        );
    }

    #[test]
    fn failed_attempts_keep_their_position_and_symbol() {
        let (accepted, lexer) = scan("\\q\\\"", &[CONTENT_STR_1, CONTENT_STR_1_RAW]);
        assert!(accepted);
        assert_eq!(
            lexer.calls,
            [
                Call::Mark(0),
                Call::Symbol(CONTENT_STR_1 as Symbol),
                Call::Mark(0),
                Call::Advance(0),
                Call::Advance(1),
                Call::Mark(2),
                Call::Advance(2),
                Call::Symbol(CONTENT_STR_1_RAW as Symbol),
            ]
        );

        // The failed string scan consumes to EOF before command scanning runs.
        let (accepted, lexer) = scan("`", &[CONTENT_STR_1, CONTENT_CMD_1]);
        assert!(!accepted);
        assert_eq!(lexer.calls, [Call::Mark(0), Call::Advance(0)]);

        // A failed comment likewise prevents a content scan from starting over.
        let (accepted, lexer) = scan("\"", &[BLOCK_COMMENT_REST, CONTENT_STR_1]);
        assert!(!accepted);
        assert_eq!(lexer.calls, [Call::Advance(0)]);
    }

    #[test]
    fn nested_comments_have_no_explicit_end_mark() {
        for input in [
            "=#tail",
            "#=nested=#=#tail",
            "==#tail",
            "=x#=nested=#=#tail",
        ] {
            let (accepted, lexer) = scan(input, &[BLOCK_COMMENT_REST]);
            assert!(accepted, "{input}");
            let end = input.len() - "tail".len();
            assert_eq!(lexer.position, end);
            let mut expected: Vec<_> = (0..end).map(Call::Advance).collect();
            expected.push(Call::Symbol(BLOCK_COMMENT_REST as Symbol));
            assert_eq!(lexer.calls, expected);
        }
    }

    #[test]
    fn nul_and_unterminated_content_do_not_emit_tokens() {
        for (input, symbol, end) in [
            ("abc", CONTENT_STR_1, 3),
            ("x\0\"", CONTENT_STR_1, 1),
            ("abc", BLOCK_COMMENT_REST, 3),
            ("#==#", BLOCK_COMMENT_REST, 4),
            ("x\0=#", BLOCK_COMMENT_REST, 1),
        ] {
            let (accepted, lexer) = scan(input, &[symbol]);
            assert!(!accepted, "{input:?}");
            assert_eq!(lexer.position, end);
            assert_eq!(lexer.symbol, Symbol::MAX);
        }
    }

    #[test]
    fn scanner_has_no_serialized_state() {
        let mut scanner = create();
        let mut buffer = [0xa5; 1024];
        assert_eq!(scanner.serialize(&mut buffer), 0);
        assert_eq!(buffer, [0xa5; 1024]);
        scanner.deserialize(&[1, 2, 3]);
        scanner.deserialize(&[]);
        assert_eq!(scanner.serialize(&mut []), 0);
    }
}
