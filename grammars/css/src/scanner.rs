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

// Most characters in a property value or pseudo selector do not affect the
// colon lookahead. A byte table lets that path bypass the delimiter dispatch.
const PSEUDO_SPECIAL: [bool; 128] = {
    let mut result = [false; 128];
    result[0] = true;
    result[b';' as usize] = true;
    result[b'}' as usize] = true;
    result[b'{' as usize] = true;
    result[b'/' as usize] = true;
    result[b'*' as usize] = true;
    result
};

impl ExternalScanner for Scanner {
    fn scan(&mut self, lexer: &mut dyn Lexer, valid_symbols: &[bool]) -> bool {
        // These immutable flags are fixed for the scan. Copy them once so
        // callback-heavy paths do not have to retain the caller's slice.
        let flags = *valid_symbols
            .first_chunk::<3>()
            .expect("CSS has three external tokens");
        if flags[ERROR_RECOVERY] {
            return false;
        }

        // Unlike C's field access, lookahead is a dynamic call. Cache it until
        // advancing, including across mark_end and result_symbol updates.
        let mut lookahead = lexer.lookahead();
        // Reuse the initial classification if descendant recognition is not
        // enabled; both token branches otherwise test the same character.
        let mut space = is_space(lookahead);
        if space && flags[DESCENDANT_OP] {
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
                // Whitespace has been consumed and ':' was handled above.
                // The pseudo-class branch cannot accept this character.
                _ => return false,
            }
        }

        if flags[PSEUDO_CLASS_SELECTOR_COLON] {
            while space {
                lexer.advance(true);
                lookahead = lexer.lookahead();
                space = is_space(lookahead);
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
                // The first character after ':' and the second character of
                // each comment delimiter are only tested for loop termination.
                'check_end: loop {
                    if matches!(lookahead, 0x3b | 0x7d) {
                        return false;
                    }
                    if at_eof(lexer, lookahead) {
                        // Like C, prefer an erroneous pseudo class at EOF.
                        return lexer.eof();
                    }
                    loop {
                        lexer.advance(false);
                        lookahead = lexer.lookahead();
                        if !PSEUDO_SPECIAL
                            .get(lookahead as usize)
                            .copied()
                            .unwrap_or(false)
                        {
                            continue;
                        }
                        // Test each newly visited character once, instead of
                        // separately dispatching at the loop's top and bottom.
                        match lookahead {
                            0x3b | 0x7d => return false,
                            0 => {
                                if lexer.eof() {
                                    return lexer.eof();
                                }
                            }
                            0x7b if !in_comment => return true,
                            0x2f if !in_comment => {
                                lexer.advance(false);
                                lookahead = lexer.lookahead();
                                in_comment = lookahead == 0x2a;
                                continue 'check_end;
                            }
                            0x2a if in_comment => {
                                lexer.advance(false);
                                lookahead = lexer.lookahead();
                                in_comment = lookahead != 0x2f;
                                continue 'check_end;
                            }
                            _ => {}
                        }
                    }
                }
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

    impl TestLexer {
        fn new(input: Vec<i32>) -> Self {
            Self {
                input,
                position: 0,
                end: None,
                symbol: u16::MAX,
                events: RefCell::new(Vec::new()),
                lookahead_calls: Cell::new(0),
            }
        }
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
        let mut lexer = TestLexer::new(input.chars().map(|c| c as i32).collect());
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

    // The C pseudo-class branch's original control flow, with cached lookahead
    // and EOF calls suppressed only where the Lexer contract implies !eof().
    // Keep this independent of the rotated loop and its ASCII dispatch table.
    fn reference_pseudo_class(lexer: &mut dyn Lexer) -> bool {
        let lookahead = lexer.lookahead();
        reference_pseudo_class_from(lexer, lookahead)
    }

    fn reference_pseudo_class_from(lexer: &mut dyn Lexer, mut lookahead: i32) -> bool {
        while is_space(lookahead) {
            lexer.advance(true);
            lookahead = lexer.lookahead();
        }
        if lookahead != i32::from(b':') {
            return false;
        }
        lexer.advance(false);
        lookahead = lexer.lookahead();
        if lookahead == i32::from(b':') {
            return false;
        }
        lexer.mark_end();
        lexer.set_result_symbol(PSEUDO_CLASS_SELECTOR_COLON as u16);
        let mut in_comment = false;
        while lookahead != i32::from(b';')
            && lookahead != i32::from(b'}')
            && !at_eof(lexer, lookahead)
        {
            lexer.advance(false);
            lookahead = lexer.lookahead();
            if lookahead == i32::from(b'{') && !in_comment {
                return true;
            }
            if lookahead == i32::from(b'/') && !in_comment {
                lexer.advance(false);
                lookahead = lexer.lookahead();
                if lookahead == i32::from(b'*') {
                    in_comment = true;
                }
            } else if lookahead == i32::from(b'*') && in_comment {
                lexer.advance(false);
                lookahead = lexer.lookahead();
                if lookahead == i32::from(b'/') {
                    in_comment = false;
                }
            }
        }
        at_eof(lexer, lookahead)
    }

    #[test]
    fn pseudo_class_dispatch_preserves_c_control_flow() {
        // Exhaust the interactions between delimiters, embedded NULs, ordinary
        // ASCII, and codepoints outside the table (including invalid values).
        let alphabet = [0, 0x20, 0x2a, 0x2f, 0x3a, 0x3b, 0x78, 0x7b, 0x7d, 0x100, -1];
        for length in 0..=5 {
            for mut index in 0..alphabet.len().pow(length) {
                let mut input = vec![i32::from(b':')];
                for _ in 0..length {
                    input.push(alphabet[index % alphabet.len()]);
                    index /= alphabet.len();
                }
                let mut actual = TestLexer::new(input.clone());
                let mut expected = TestLexer::new(input);
                let accepted = Scanner.scan(&mut actual, &[false, true, false]);
                assert_eq!(
                    accepted,
                    reference_pseudo_class(&mut expected),
                    "{:?}",
                    actual.input
                );
                assert_eq!(actual.events, expected.events, "{:?}", actual.input);
                assert_eq!(actual.lookahead_calls, expected.lookahead_calls);
                assert_eq!(actual.position, expected.position);
                assert_eq!(actual.end, expected.end);
                assert_eq!(actual.symbol, expected.symbol);
            }
        }
    }

    // Preserve the original C branch ordering and descendant-to-pseudo-class
    // fallthrough here, independently of the optimized flag/space handling.
    fn reference_scan(lexer: &mut dyn Lexer, valid: &[bool; 3]) -> bool {
        if valid[ERROR_RECOVERY] {
            return false;
        }
        let mut lookahead = lexer.lookahead();
        if is_space(lookahead) && valid[DESCENDANT_OP] {
            lexer.set_result_symbol(DESCENDANT_OP as u16);
            lexer.advance(true);
            lookahead = lexer.lookahead();
            while is_space(lookahead) {
                lexer.advance(true);
                lookahead = lexer.lookahead();
            }
            lexer.mark_end();
            if matches!(
                lookahead,
                0x23 | 0x2e | 0x5b | 0x2d | 0x2a | 0x30..=0x39 | 0x41..=0x5a | 0x61..=0x7a
            ) {
                return true;
            }
            if lookahead == 0x3a {
                lexer.advance(false);
                lookahead = lexer.lookahead();
                if is_space(lookahead) {
                    return false;
                }
                loop {
                    if lookahead == 0x3b || lookahead == 0x7d || at_eof(lexer, lookahead) {
                        return false;
                    }
                    if lookahead == 0x7b {
                        return true;
                    }
                    lexer.advance(false);
                    lookahead = lexer.lookahead();
                }
            }
        }
        valid[PSEUDO_CLASS_SELECTOR_COLON] && reference_pseudo_class_from(lexer, lookahead)
    }

    #[test]
    fn flag_and_whitespace_dispatch_preserve_c_control_flow() {
        // Exhaust short inputs for all flag combinations, including disabled
        // and recovery scans. The alphabet exercises both C whitespace and
        // non-C Unicode whitespace, selector prefixes, punctuation and NUL.
        let alphabet = [
            0, 0x09, 0x0b, 0x0d, 0x20, 0x2a, 0x2f, 0x3a, 0x3b, 0x41, 0x5f, 0x7b, 0x7d, 0xa0,
            0x2028, -1,
        ];
        for length in 0..=4 {
            for mut index in 0..alphabet.len().pow(length) {
                let mut input = Vec::new();
                for _ in 0..length {
                    input.push(alphabet[index % alphabet.len()]);
                    index /= alphabet.len();
                }
                for flags in 0..8 {
                    let valid = [flags & 1 != 0, flags & 2 != 0, flags & 4 != 0];
                    let mut actual = TestLexer::new(input.clone());
                    let mut expected = TestLexer::new(input.clone());
                    assert_eq!(
                        Scanner.scan(&mut actual, &valid),
                        reference_scan(&mut expected, &valid),
                        "{input:?} {valid:?}",
                    );
                    assert_eq!(actual.events, expected.events, "{input:?} {valid:?}");
                    assert_eq!(actual.lookahead_calls, expected.lookahead_calls);
                    assert_eq!(actual.position, expected.position);
                    assert_eq!(actual.end, expected.end);
                    assert_eq!(actual.symbol, expected.symbol);
                }
            }
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
