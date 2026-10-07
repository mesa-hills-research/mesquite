//! The CSS external scanner, translated from `src/scanner.c`.

use ts_port_tables::{ExternalScanner, Lexer};

// External token indices, in the order of the C TokenType enum.
const DESCENDANT_OP: usize = 0;
const PSEUDO_CLASS_SELECTOR_COLON: usize = 1;
const ERROR_RECOVERY: usize = 2;

/// The C scanner has no payload or persistent state.
pub(crate) struct Scanner;

// Match wctype.h in the reference's default C locale, not Unicode character
// classes. In particular, C whitespace includes vertical tab.
fn is_space(c: i32) -> bool {
    matches!(c, 0x09..=0x0d | 0x20)
}

// The Lexer contract guarantees zero lookahead at EOF. Avoid a dynamic EOF
// call for every ordinary character, but still distinguish an embedded NUL
// from the end of input (including the end of the included ranges).
fn at_eof(lexer: &dyn Lexer, lookahead: i32) -> bool {
    lookahead == 0 && lexer.eof()
}

impl ExternalScanner for Scanner {
    fn scan(&mut self, lexer: &mut dyn Lexer, valid_symbols: &[bool]) -> bool {
        if valid_symbols[ERROR_RECOVERY] {
            return false;
        }

        // Unlike C's field access, lookahead is a dynamic call. Cache it until
        // advancing, including across mark_end and result_symbol updates.
        let mut lookahead = lexer.lookahead();
        if is_space(lookahead) && valid_symbols[DESCENDANT_OP] {
            lexer.set_result_symbol(DESCENDANT_OP as u16);

            loop {
                lexer.advance(true);
                lookahead = lexer.lookahead();
                if !is_space(lookahead) {
                    break;
                }
            }
            lexer.mark_end();

            match lookahead {
                // Selector prefixes (# . [ - *) and C-locale iswalnum.
                0x23 | 0x2e | 0x5b | 0x2d | 0x2a | 0x30..=0x39 | 0x41..=0x5a | 0x61..=0x7a => {
                    return true;
                }
                0x3a => {
                    lexer.advance(false);
                    lookahead = lexer.lookahead();
                    if is_space(lookahead) {
                        return false;
                    }
                    loop {
                        if matches!(lookahead, 0x3b | 0x7d) || at_eof(lexer, lookahead) {
                            return false;
                        }
                        if lookahead == 0x7b {
                            return true;
                        }
                        lexer.advance(false);
                        lookahead = lexer.lookahead();
                    }
                }
                _ => {}
            }
        }

        if valid_symbols[PSEUDO_CLASS_SELECTOR_COLON] {
            while is_space(lookahead) {
                lexer.advance(true);
                lookahead = lexer.lookahead();
            }
            if lookahead == 0x3a {
                lexer.advance(false);
                lookahead = lexer.lookahead();
                if lookahead == 0x3a {
                    return false;
                }
                lexer.mark_end();
                lexer.set_result_symbol(PSEUDO_CLASS_SELECTOR_COLON as u16);

                // A brace indicates a pseudo class; a semicolon indicates a
                // property. Preserve C's advance-before-inspection order and
                // its unconditional semicolon/closing-brace loop terminators,
                // even while inside a comment.
                let mut in_comment = false;
                while !matches!(lookahead, 0x3b | 0x7d) && !at_eof(lexer, lookahead) {
                    lexer.advance(false);
                    lookahead = lexer.lookahead();
                    match lookahead {
                        0x7b if !in_comment => return true,
                        0x2f if !in_comment => {
                            lexer.advance(false);
                            lookahead = lexer.lookahead();
                            if lookahead == 0x2a {
                                in_comment = true;
                            }
                        }
                        0x2a if in_comment => {
                            lexer.advance(false);
                            lookahead = lexer.lookahead();
                            if lookahead == 0x2f {
                                in_comment = false;
                            }
                        }
                        _ => {}
                    }
                }

                // At EOF, prefer an erroneous pseudo class over a property.
                return at_eof(lexer, lookahead);
            }
        }

        false
    }

    fn serialize(&mut self, _buffer: &mut [u8]) -> usize {
        0
    }

    fn deserialize(&mut self, _buffer: &[u8]) {}
}

/// Creates a scanner (C's `tree_sitter_css_external_scanner_create`).
pub(crate) fn create() -> Box<dyn ExternalScanner> {
    Box::new(Scanner)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::{Cell, RefCell};

    #[derive(Debug, PartialEq, Eq)]
    enum Event {
        Advance(usize, bool),
        MarkEnd(usize),
        Result(u16),
        Eof(usize),
    }

    struct TestLexer {
        input: Vec<i32>,
        position: usize,
        end: Option<usize>,
        symbol: u16,
        events: RefCell<Vec<Event>>,
        lookahead_calls: Cell<usize>,
    }

    impl Lexer for TestLexer {
        fn lookahead(&self) -> i32 {
            self.lookahead_calls.set(self.lookahead_calls.get() + 1);
            self.input.get(self.position).copied().unwrap_or(0)
        }

        fn result_symbol(&self) -> u16 {
            self.symbol
        }

        fn set_result_symbol(&mut self, symbol: u16) {
            self.symbol = symbol;
            self.events.borrow_mut().push(Event::Result(symbol));
        }

        fn advance(&mut self, skip: bool) {
            self.events
                .borrow_mut()
                .push(Event::Advance(self.position, skip));
            if self.position < self.input.len() {
                self.position += 1;
            }
        }

        fn mark_end(&mut self) {
            self.end = Some(self.position);
            self.events.borrow_mut().push(Event::MarkEnd(self.position));
        }

        fn get_column(&mut self) -> u32 {
            panic!("CSS scanner does not call get_column")
        }

        fn is_at_included_range_start(&self) -> bool {
            panic!("CSS scanner does not check included range starts")
        }

        fn eof(&self) -> bool {
            self.events.borrow_mut().push(Event::Eof(self.position));
            self.position == self.input.len()
        }
    }

    fn scan(input: &str, valid_symbols: [bool; 3]) -> (bool, TestLexer) {
        let mut lexer = TestLexer {
            input: input.chars().map(|c| c as i32).collect(),
            position: 0,
            end: None,
            symbol: u16::MAX,
            events: RefCell::new(Vec::new()),
            lookahead_calls: Cell::new(0),
        };
        let accepted = create().scan(&mut lexer, &valid_symbols);
        (accepted, lexer)
    }

    #[test]
    fn recovery_and_disabled_tokens_do_not_call_the_lexer() {
        for flags in [[true, true, true], [false, false, false]] {
            let (accepted, lexer) = scan(" :hover {", flags);
            assert!(!accepted);
            assert!(lexer.events.into_inner().is_empty());
        }
    }

    #[test]
    fn descendant_skips_c_whitespace_and_marks_before_the_selector() {
        for suffix in ["#id", ".class", "[attr]", "-tag", "*", "a", "Z", "0"] {
            let (accepted, lexer) = scan(&format!(" \t\r\n\x0b\x0c{suffix}"), [true, true, false]);
            assert!(accepted, "{suffix:?}");
            assert_eq!(lexer.end, Some(6));
            assert_eq!(
                lexer.events.into_inner(),
                [
                    Event::Result(DESCENDANT_OP as u16),
                    Event::Advance(0, true),
                    Event::Advance(1, true),
                    Event::Advance(2, true),
                    Event::Advance(3, true),
                    Event::Advance(4, true),
                    Event::Advance(5, true),
                    Event::MarkEnd(6),
                ]
            );
        }
        for input in ["a", " _", " é", " ١", "\u{a0}a", "\u{2028}a", " ", ""] {
            assert!(!scan(input, [true, false, false]).0, "{input:?}");
        }
    }

    #[test]
    fn descendant_colon_looks_ahead_without_extending_the_token() {
        for (input, accepted) in [
            (" :hover {", true),
            (" ::before {", true),
            (" :{", true),
            (" : {", false),
            (" :hover; {", false),
            (" :hover} {", false),
            (" :hover", false),
            // This branch deliberately does not track comments.
            (" :x/* {", true),
        ] {
            let (actual, lexer) = scan(input, [true, true, false]);
            assert_eq!(actual, accepted, "{input:?}");
            assert_eq!(lexer.end, Some(1));
            assert_eq!(lexer.symbol, DESCENDANT_OP as u16);
        }
    }

    #[test]
    fn pseudo_class_marks_only_the_colon_and_preserves_advance_order() {
        let (accepted, lexer) = scan(" \t:x{", [false, true, false]);
        assert!(accepted);
        assert_eq!(lexer.end, Some(3));
        assert_eq!(
            lexer.events.into_inner(),
            [
                Event::Advance(0, true),
                Event::Advance(1, true),
                Event::Advance(2, false),
                Event::MarkEnd(3),
                Event::Result(PSEUDO_CLASS_SELECTOR_COLON as u16),
                Event::Advance(3, false),
            ]
        );

        let (accepted, lexer) = scan("::before {", [false, true, false]);
        assert!(!accepted);
        assert_eq!(lexer.events.into_inner(), [Event::Advance(0, false)]);
    }

    #[test]
    fn pseudo_class_comment_and_eof_quirks_match_c() {
        for (input, accepted) in [
            (":hover {", true),
            (":hover", true),
            (":", true),
            (":value;", false),
            (":value}", false),
            (":x/* { */value;", false),
            (":x/* { */value {", true),
            (":x/* ; */value {", false),
            (":x/* } */value {", false),
            (":x/* unterminated", true),
            // The initial character after ':' is advanced over before testing
            // for braces or comment openers, just as in the C loop.
            (":{;", false),
            (":/* { */value;", true),
            (":x/{;", false),
            (":\0x{", true),
        ] {
            let (actual, lexer) = scan(input, [false, true, false]);
            assert_eq!(actual, accepted, "{input:?}");
            assert_eq!(lexer.end, Some(1));
            assert_eq!(lexer.symbol, PSEUDO_CLASS_SELECTOR_COLON as u16);
        }

        let (accepted, lexer) = scan(":", [false, true, false]);
        assert!(accepted);
        assert_eq!(
            lexer.events.into_inner(),
            [
                Event::Advance(0, false),
                Event::MarkEnd(1),
                Event::Result(PSEUDO_CLASS_SELECTOR_COLON as u16),
                Event::Eof(1),
                Event::Eof(1),
            ]
        );
    }

    #[test]
    fn lookahead_is_read_once_per_visited_position() {
        for input in [
            "",
            " ",
            ".class",
            "\0",
            "é",
            " \t.class",
            " :hover {",
            " :hover",
            " : value",
            " \t+value",
            "::before {",
            ":x/* { */value {",
            ":x/* ; */value {",
            ":x/* } */value {",
            ":x/* unterminated",
            ":x/",
            ":x/* *",
            ":x/**/{",
            ":\0x{",
        ] {
            for flags in [
                [false, false, false],
                [false, true, false],
                [true, false, false],
                [true, true, false],
            ] {
                let (_, lexer) = scan(input, flags);
                assert_eq!(
                    lexer.lookahead_calls.get(),
                    lexer.position + 1,
                    "{input:?} {flags:?}"
                );
            }
        }
        let (_, lexer) = scan(" :hover {", [true, true, true]);
        assert_eq!(lexer.lookahead_calls.get(), 0);
    }

    #[test]
    fn eof_is_only_queried_at_zero_lookahead() {
        for input in [
            " :hover {",
            " :hover;",
            " :hover}",
            " :hover",
            ":hover {",
            ":hover;",
            ":hover}",
            ":hover",
            ":",
            ":x/* unterminated",
            ":x/* { */value {",
            ":x/* ; */value {",
            ":x/* } */value {",
        ] {
            for flags in [[true, true, false], [false, true, false]] {
                let (_, lexer) = scan(input, flags);
                for event in lexer.events.into_inner() {
                    if let Event::Eof(position) = event {
                        assert_eq!(position, lexer.input.len(), "{input:?} {flags:?}");
                    }
                }
            }
        }
    }

    #[test]
    fn embedded_nul_is_not_eof_in_either_colon_scan() {
        for (input, accepted) in [
            (":\0x{", true),
            (":\0x;", false),
            (":\0x}", false),
            (":x/\0{;", true),
            (":x/*\0{*/;", false),
            (":x/*\0{*/{", true),
            (":x/*\0;*/{", false),
            (":x/*\0}*/{", false),
        ] {
            assert_eq!(scan(input, [false, true, false]).0, accepted, "{input:?}");
        }
        for (input, accepted) in [
            (" :\0x{", true),
            (" :\0x;", false),
            (" :\0x}", false),
            (" :\0x", false),
        ] {
            assert_eq!(scan(input, [true, true, false]).0, accepted, "{input:?}");
        }
    }

    #[test]
    fn serialization_is_empty_and_does_not_touch_the_buffer() {
        let mut scanner = create();
        let mut buffer = [0xa5; 1024];
        scanner.deserialize(&[]);
        assert_eq!(scanner.serialize(&mut buffer), 0);
        scanner.deserialize(&[1, 2, 3]);
        assert_eq!(scanner.serialize(&mut buffer), 0);
        assert_eq!(buffer, [0xa5; 1024]);
        assert_eq!(scanner.serialize(&mut []), 0);
    }
}
