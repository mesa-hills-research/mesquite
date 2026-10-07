//! The stateless TypeScript/TSX external scanner, translated from
//! `typescript/src/scanner.c` and `common/scanner.h`.

use ts_port_tables::{ExternalScanner, Lexer};

// External token indices, in the order of the C TokenType enum. The final token
// (ERROR_RECOVERY, index 9) is not inspected by this scanner.
const AUTOMATIC_SEMICOLON: usize = 0;
const TEMPLATE_CHARS: usize = 1;
const TERNARY_QMARK: usize = 2;
const HTML_COMMENT: usize = 3;
const LOGICAL_OR: usize = 4;
const ESCAPE_SEQUENCE: usize = 5;
const REGEX_PATTERN: usize = 6;
const JSX_TEXT: usize = 7;
const FUNCTION_SIGNATURE_AUTOMATIC_SEMICOLON: usize = 8;

// The reference runs in the C locale. In particular, iswspace includes vertical
// tab but not Unicode whitespace, and iswalpha/iswdigit only include ASCII.
fn is_space(c: i32) -> bool {
    matches!(c, 0x09..=0x0d | 0x20)
}

fn is_alpha(c: i32) -> bool {
    matches!(c, 0x41..=0x5a | 0x61..=0x7a)
}

fn is_digit(c: i32) -> bool {
    matches!(c, 0x30..=0x39)
}

fn advance(lexer: &mut dyn Lexer) {
    lexer.advance(false);
}

fn skip(lexer: &mut dyn Lexer) {
    lexer.advance(true);
}

fn scan_template_chars(lexer: &mut dyn Lexer) -> bool {
    lexer.set_result_symbol(TEMPLATE_CHARS as u16);
    let mut has_content = false;
    loop {
        lexer.mark_end();
        match char::from_u32(lexer.lookahead() as u32) {
            Some('`' | '\\') => return has_content,
            Some('\0') => return false,
            Some('$') => {
                advance(lexer);
                if lexer.lookahead() == i32::from(b'{') {
                    return has_content;
                }
            }
            _ => advance(lexer),
        }
        has_content = true;
    }
}

fn scan_whitespace_and_comments(lexer: &mut dyn Lexer, scanned_comment: &mut bool) -> bool {
    loop {
        while is_space(lexer.lookahead()) {
            skip(lexer);
        }

        if lexer.lookahead() == i32::from(b'/') {
            skip(lexer);
            if lexer.lookahead() == i32::from(b'/') {
                skip(lexer);
                while lexer.lookahead() != 0 && lexer.lookahead() != i32::from(b'\n') {
                    skip(lexer);
                }
                // The C scanner only sets this flag for line comments, not block comments.
                *scanned_comment = true;
            } else if lexer.lookahead() == i32::from(b'*') {
                skip(lexer);
                while lexer.lookahead() != 0 {
                    if lexer.lookahead() == i32::from(b'*') {
                        skip(lexer);
                        if lexer.lookahead() == i32::from(b'/') {
                            skip(lexer);
                            break;
                        }
                    } else {
                        skip(lexer);
                    }
                }
            } else {
                return false;
            }
        } else {
            return true;
        }
    }
}

fn scan_automatic_semicolon(
    lexer: &mut dyn Lexer,
    valid_symbols: &[bool],
    scanned_comment: &mut bool,
) -> bool {
    lexer.set_result_symbol(AUTOMATIC_SEMICOLON as u16);
    lexer.mark_end();

    loop {
        if lexer.lookahead() == 0 {
            return true;
        }
        if lexer.lookahead() == i32::from(b'}') {
            // Do not insert before a typed object pattern, unless a valid binary
            // operator indicates that this is a ternary expression instead.
            loop {
                skip(lexer);
                if !is_space(lexer.lookahead()) {
                    break;
                }
            }
            if lexer.lookahead() == i32::from(b':') {
                return valid_symbols[LOGICAL_OR];
            }
            return true;
        }
        if !is_space(lexer.lookahead()) {
            return false;
        }
        if lexer.lookahead() == i32::from(b'\n') {
            break;
        }
        skip(lexer);
    }

    skip(lexer);
    if !scan_whitespace_and_comments(lexer, scanned_comment) {
        return false;
    }

    match char::from_u32(lexer.lookahead() as u32) {
        Some(
            '`' | ',' | '.' | ';' | '*' | '%' | '>' | '<' | '=' | '?' | '^' | '|' | '&' | '/' | ':',
        ) => {
            return false;
        }
        Some('{') => {
            if valid_symbols[FUNCTION_SIGNATURE_AUTOMATIC_SEMICOLON] {
                return false;
            }
        }
        Some('(' | '[') => {
            // These continue expressions, but not types.
            if valid_symbols[LOGICAL_OR] {
                return false;
            }
        }
        Some('+') => {
            skip(lexer);
            return lexer.lookahead() == i32::from(b'+');
        }
        Some('-') => {
            skip(lexer);
            return lexer.lookahead() == i32::from(b'-');
        }
        Some('!') => {
            skip(lexer);
            return lexer.lookahead() != i32::from(b'=');
        }
        Some('i') => {
            skip(lexer);
            if lexer.lookahead() != i32::from(b'n') {
                return true;
            }
            skip(lexer);
            if !is_alpha(lexer.lookahead()) {
                return false;
            }
            for &c in b"stanceof" {
                if lexer.lookahead() != i32::from(c) {
                    return true;
                }
                skip(lexer);
            }
            if !is_alpha(lexer.lookahead()) {
                return false;
            }
        }
        _ => {}
    }
    true
}

fn scan_ternary_qmark(lexer: &mut dyn Lexer) -> bool {
    while is_space(lexer.lookahead()) {
        skip(lexer);
    }

    if lexer.lookahead() != i32::from(b'?') {
        return false;
    }
    advance(lexer);

    // Nullish coalescing and optional chaining are not ternary operators.
    if lexer.lookahead() == i32::from(b'?') || lexer.lookahead() == i32::from(b'.') {
        return false;
    }

    lexer.mark_end();
    lexer.set_result_symbol(TERNARY_QMARK as u16);

    // Optional arguments can contain `?:`, including intervening whitespace.
    while is_space(lexer.lookahead()) {
        advance(lexer);
    }
    match char::from_u32(lexer.lookahead() as u32) {
        Some(':' | ')' | ',') => false,
        Some('.') => {
            advance(lexer);
            is_digit(lexer.lookahead())
        }
        _ => true,
    }
}

fn scan_closing_comment(lexer: &mut dyn Lexer) -> bool {
    while is_space(lexer.lookahead()) || matches!(lexer.lookahead(), 0x2028 | 0x2029) {
        skip(lexer);
    }

    let delimiter: &[u8] = match char::from_u32(lexer.lookahead() as u32) {
        Some('<') => b"<!--",
        Some('-') => b"-->",
        _ => return false,
    };
    for &c in delimiter {
        if lexer.lookahead() != i32::from(c) {
            return false;
        }
        advance(lexer);
    }

    while !matches!(lexer.lookahead(), 0 | 0x0a | 0x2028 | 0x2029) {
        advance(lexer);
    }
    lexer.set_result_symbol(HTML_COMMENT as u16);
    lexer.mark_end();
    true
}

fn scan_jsx_text(lexer: &mut dyn Lexer) -> bool {
    // Newlines and whitespace immediately following them do not count as text.
    let mut saw_text = false;
    let mut at_newline = false;
    while !matches!(
        char::from_u32(lexer.lookahead() as u32),
        Some('\0' | '<' | '>' | '{' | '}' | '&')
    ) {
        let is_wspace = is_space(lexer.lookahead());
        if lexer.lookahead() == i32::from(b'\n') {
            at_newline = true;
        } else {
            at_newline &= is_wspace;
            if !at_newline {
                saw_text = true;
            }
        }
        advance(lexer);
    }
    lexer.set_result_symbol(JSX_TEXT as u16);
    saw_text
}

/// The C scanner has a null payload and no serialized state.
pub(crate) struct Scanner;

impl ExternalScanner for Scanner {
    fn scan(&mut self, lexer: &mut dyn Lexer, valid_symbols: &[bool]) -> bool {
        if valid_symbols[TEMPLATE_CHARS] {
            if valid_symbols[AUTOMATIC_SEMICOLON] {
                return false;
            }
            return scan_template_chars(lexer);
        }

        if valid_symbols[JSX_TEXT] && scan_jsx_text(lexer) {
            return true;
        }

        if valid_symbols[AUTOMATIC_SEMICOLON]
            || valid_symbols[FUNCTION_SIGNATURE_AUTOMATIC_SEMICOLON]
        {
            let mut scanned_comment = false;
            let result = scan_automatic_semicolon(lexer, valid_symbols, &mut scanned_comment);
            if !result
                && !scanned_comment
                && valid_symbols[TERNARY_QMARK]
                && lexer.lookahead() == i32::from(b'?')
            {
                return scan_ternary_qmark(lexer);
            }
            return result;
        }
        if valid_symbols[TERNARY_QMARK] {
            return scan_ternary_qmark(lexer);
        }
        if valid_symbols[HTML_COMMENT]
            && !valid_symbols[LOGICAL_OR]
            && !valid_symbols[ESCAPE_SEQUENCE]
            && !valid_symbols[REGEX_PATTERN]
        {
            return scan_closing_comment(lexer);
        }
        false
    }

    fn serialize(&mut self, _buffer: &mut [u8]) -> usize {
        0
    }

    fn deserialize(&mut self, _buffer: &[u8]) {}
}

/// Creates a scanner (C's `tree_sitter_typescript_external_scanner_create`).
pub(crate) fn create() -> Box<dyn ExternalScanner> {
    Box::new(Scanner)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Eq)]
    enum Event {
        Advance(usize, bool),
        MarkEnd(usize),
        Symbol(u16),
    }

    struct TestLexer {
        input: Vec<i32>,
        position: usize,
        end: Option<usize>,
        symbol: u16,
        events: Vec<Event>,
    }

    impl TestLexer {
        fn new(input: &str) -> Self {
            Self {
                input: input.chars().map(|c| c as i32).collect(),
                position: 0,
                end: None,
                symbol: u16::MAX,
                events: Vec::new(),
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
            self.events.push(Event::Symbol(symbol));
        }

        fn advance(&mut self, skip: bool) {
            assert!(self.position < self.input.len(), "advance past EOF");
            self.events.push(Event::Advance(self.position, skip));
            self.position += 1;
        }

        fn mark_end(&mut self) {
            self.end = Some(self.position);
            self.events.push(Event::MarkEnd(self.position));
        }

        fn get_column(&mut self) -> u32 {
            panic!("scanner does not use get_column")
        }

        fn is_at_included_range_start(&self) -> bool {
            panic!("scanner does not inspect included range starts")
        }

        fn eof(&self) -> bool {
            panic!("scanner tests lookahead, not eof")
        }
    }

    fn scan(input: &str, tokens: &[usize]) -> (bool, TestLexer) {
        let mut valid_symbols = [false; 10];
        for &token in tokens {
            valid_symbols[token] = true;
        }
        let mut lexer = TestLexer::new(input);
        let result = create().scan(&mut lexer, &valid_symbols);
        (result, lexer)
    }

    #[test]
    fn template_interpolation_preserves_mark_before_dollar() {
        let (accepted, lexer) = scan("a${x}", &[TEMPLATE_CHARS]);
        assert!(accepted);
        assert_eq!(lexer.position, 2);
        assert_eq!(lexer.end, Some(1));
        assert_eq!(
            lexer.events,
            [
                Event::Symbol(TEMPLATE_CHARS as u16),
                Event::MarkEnd(0),
                Event::Advance(0, false),
                Event::MarkEnd(1),
                Event::Advance(1, false),
            ]
        );
    }

    #[test]
    fn template_boundaries_and_unterminated_content() {
        for (input, accepted, position, end) in [
            ("`", false, 0, 0),
            ("\\n", false, 0, 0),
            ("${x}", false, 1, 0),
            ("a`", true, 1, 1),
            ("a\\n", true, 1, 1),
            ("$x`", true, 2, 2),
            ("$$${x}", true, 3, 2),
            ("abc", false, 3, 3),
            ("a\0b", false, 1, 1),
        ] {
            let (result, lexer) = scan(input, &[TEMPLATE_CHARS]);
            assert_eq!(
                (result, lexer.position, lexer.end),
                (accepted, position, Some(end)),
                "{input:?}"
            );
        }
    }

    #[test]
    fn template_dispatch_precedes_all_other_tokens() {
        let mut lexer = TestLexer::new("anything");
        assert!(!Scanner.scan(&mut lexer, &[true; 10]));
        assert!(lexer.events.is_empty());
        // Only AUTOMATIC_SEMICOLON suppresses template scanning, not its
        // function-signature variant.
        let (accepted, lexer) = scan(
            "a`",
            &[TEMPLATE_CHARS, FUNCTION_SIGNATURE_AUTOMATIC_SEMICOLON],
        );
        assert!(accepted);
        assert_eq!(lexer.symbol, TEMPLATE_CHARS as u16);
    }

    #[test]
    fn semicolon_decisions_and_lookahead() {
        for (input, accepted, position) in [
            ("", true, 0),
            (" \t", true, 2),
            ("x", false, 0),
            ("\rx", false, 1),
            ("\u{2028}x", false, 0),
            ("\nx", true, 1),
            ("}", true, 1),
            ("} \t:", false, 3),
            ("\n}", true, 1),
            ("\n++", true, 2),
            ("\n+x", false, 2),
            ("\n--", true, 2),
            ("\n-x", false, 2),
            ("\n!x", true, 2),
            ("\n!=", false, 2),
            ("\nin ", false, 3),
            ("\nin_", false, 3),
            ("\ninstanceof ", false, 11),
            ("\ninstanceofx", true, 11),
            ("\ninside", true, 4),
            ("\niné", false, 3),
            ("\n/*c*/x", true, 6),
            ("\n/*unterminated", true, 15),
            ("\n/x", false, 2),
            ("\n//c\nx", true, 5),
        ] {
            let (result, lexer) = scan(input, &[AUTOMATIC_SEMICOLON]);
            assert_eq!((result, lexer.position), (accepted, position), "{input:?}");
            assert_eq!(lexer.symbol, AUTOMATIC_SEMICOLON as u16);
            assert_eq!(lexer.end, Some(0));
            assert!(
                lexer
                    .events
                    .iter()
                    .all(|event| !matches!(event, Event::Advance(_, false)))
            );
        }
        for c in "`,.;*%><=?^|&:".chars() {
            let (accepted, lexer) = scan(&format!("\n{c}"), &[AUTOMATIC_SEMICOLON]);
            assert!(!accepted, "{c}");
            assert_eq!(lexer.position, 1);
        }
    }

    #[test]
    fn semicolon_distinguishes_types_and_expressions() {
        for input in ["\n(", "\n["] {
            assert!(scan(input, &[AUTOMATIC_SEMICOLON]).0);
            assert!(!scan(input, &[AUTOMATIC_SEMICOLON, LOGICAL_OR]).0);
        }
        assert!(!scan("} :", &[AUTOMATIC_SEMICOLON]).0);
        assert!(scan("} :", &[AUTOMATIC_SEMICOLON, LOGICAL_OR]).0);
        assert!(scan("\n{", &[AUTOMATIC_SEMICOLON]).0);
        assert!(!scan("\n{", &[FUNCTION_SIGNATURE_AUTOMATIC_SEMICOLON]).0);
        let (accepted, lexer) = scan("\nx", &[FUNCTION_SIGNATURE_AUTOMATIC_SEMICOLON]);
        assert!(accepted);
        assert_eq!(lexer.symbol, AUTOMATIC_SEMICOLON as u16);
    }

    #[test]
    fn ternary_disambiguates_optional_arguments_and_chaining() {
        for (input, accepted, position, end) in [
            ("?x", true, 1, Some(1)),
            ("?", true, 1, Some(1)),
            ("??", false, 1, None),
            ("?.1", false, 1, None),
            ("? :", false, 2, Some(1)),
            ("? )", false, 2, Some(1)),
            ("? ,", false, 2, Some(1)),
            ("? .1", true, 3, Some(1)),
            ("? .x", false, 3, Some(1)),
            ("? ?", true, 2, Some(1)),
            ("? \u{a0}:", true, 2, Some(1)),
            ("\u{b}?x", true, 2, Some(2)),
        ] {
            let (result, lexer) = scan(input, &[TERNARY_QMARK]);
            assert_eq!(
                (result, lexer.position, lexer.end),
                (accepted, position, end),
                "{input:?}"
            );
        }
        let (accepted, lexer) = scan(" ? x", &[TERNARY_QMARK]);
        assert!(accepted);
        assert_eq!(
            lexer.events,
            [
                Event::Advance(0, true),
                Event::Advance(1, false),
                Event::MarkEnd(2),
                Event::Symbol(TERNARY_QMARK as u16),
                Event::Advance(2, false),
            ]
        );
    }

    #[test]
    fn only_line_comments_prevent_semicolon_fallback_to_ternary() {
        let tokens = [AUTOMATIC_SEMICOLON, TERNARY_QMARK];
        let (accepted, lexer) = scan("\n/*c*/?x", &tokens);
        assert!(accepted);
        assert_eq!(lexer.symbol, TERNARY_QMARK as u16);
        assert_eq!(lexer.end, Some(7));
        assert_eq!(
            lexer.events.first(),
            Some(&Event::Symbol(AUTOMATIC_SEMICOLON as u16))
        );
        let (accepted, lexer) = scan("\n//c\n?x", &tokens);
        assert!(!accepted);
        assert_eq!(lexer.position, 5);
        assert_eq!(lexer.symbol, AUTOMATIC_SEMICOLON as u16);
        assert_eq!(lexer.end, Some(0));
    }

    #[test]
    fn html_comments_have_explicit_unicode_line_boundaries() {
        for (input, accepted, position) in [
            ("<!--x\n", true, 5),
            ("-->x\u{2028}", true, 4),
            ("\u{2029} -->x\u{2029}", true, 6),
            ("<!--x\ry", true, 7),
            ("<!-x", false, 3),
            ("--x", false, 2),
            ("\u{a0}<!--x", false, 0),
        ] {
            let (result, lexer) = scan(input, &[HTML_COMMENT]);
            assert_eq!((result, lexer.position), (accepted, position), "{input:?}");
            if accepted {
                assert_eq!(lexer.end, Some(position));
                assert_eq!(lexer.symbol, HTML_COMMENT as u16);
            }
        }
        for conflicting in [LOGICAL_OR, ESCAPE_SEQUENCE, REGEX_PATTERN] {
            let (accepted, lexer) = scan("<!--x", &[HTML_COMMENT, conflicting]);
            assert!(!accepted);
            assert!(lexer.events.is_empty());
        }
    }

    #[test]
    fn jsx_ignores_only_newlines_and_their_following_whitespace() {
        for (input, accepted, position) in [
            ("", false, 0),
            ("\n \t<", false, 3),
            (" \n<", true, 2),
            ("\r\n<", true, 2),
            ("\n x<", true, 3),
            ("\n\u{a0}<", true, 2),
            ("\u{2028}<", true, 1),
            ("abc&", true, 3),
            ("abc}", true, 3),
            ("abc{", true, 3),
            ("abc>", true, 3),
        ] {
            let (result, lexer) = scan(input, &[JSX_TEXT]);
            assert_eq!((result, lexer.position), (accepted, position), "{input:?}");
            assert_eq!(lexer.symbol, JSX_TEXT as u16);
            assert_eq!(lexer.end, None);
        }
        // A failed JSX scan does not rewind before the next scanner runs.
        let (accepted, lexer) = scan("\n }", &[JSX_TEXT, AUTOMATIC_SEMICOLON]);
        assert!(accepted);
        assert_eq!(lexer.end, Some(2));
        assert_eq!(lexer.position, 3);
    }

    #[test]
    fn serialization_is_empty_and_does_not_touch_the_buffer() {
        let mut scanner = create();
        let mut buffer = [0x55; 1024];
        assert_eq!(scanner.serialize(&mut buffer), 0);
        assert_eq!(buffer, [0x55; 1024]);
        scanner.deserialize(&[1, 2, 3]);
        assert_eq!(scanner.serialize(&mut []), 0);
        scanner.deserialize(&[]);
    }
}
