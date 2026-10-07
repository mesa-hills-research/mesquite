//! The JavaScript external scanner, translated from `src/scanner.c`.

use ts_port_tables::{ExternalScanner, Lexer};

// External token indices, in the order of the C TokenType enum.
const AUTOMATIC_SEMICOLON: usize = 0;
const TEMPLATE_CHARS: usize = 1;
const TERNARY_QMARK: usize = 2;
const HTML_COMMENT: usize = 3;
const LOGICAL_OR: usize = 4;
const ESCAPE_SEQUENCE: usize = 5;
const REGEX_PATTERN: usize = 6;
const JSX_TEXT: usize = 7;

/// The C scanner has no payload or persistent state.
pub(crate) struct Scanner;

// The C scanner runs in the default C locale. In particular, its wide-character
// predicates are not Unicode predicates, and iswspace includes vertical tab
// (unlike Rust's is_ascii_whitespace).
fn is_space(c: i32) -> bool {
    matches!(c, 0x09..=0x0d | 0x20)
}

fn is_digit(c: i32) -> bool {
    (b'0' as i32..=b'9' as i32).contains(&c)
}

fn is_alpha(c: i32) -> bool {
    (b'a' as i32..=b'z' as i32).contains(&c) || (b'A' as i32..=b'Z' as i32).contains(&c)
}

fn is_line_terminator(c: i32) -> bool {
    matches!(c, 0x0a | 0x2028 | 0x2029)
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
            Some('`') => return has_content,
            Some('\0') => return false,
            Some('$') => {
                advance(lexer);
                if lexer.lookahead() == '{' as i32 {
                    return has_content;
                }
            }
            Some('\\') => return has_content,
            _ => advance(lexer),
        }
        has_content = true;
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum WhitespaceResult {
    /// Semicolon is illegal: a syntax error occurred.
    Reject,
    /// Unclear if a semicolon will be legal; continue scanning.
    NoNewline,
    /// Semicolon is legal, assuming a comment was encountered.
    Accept,
}

/// If `consume` is false, only consume enough to check whether a comment
/// indicates that a semicolon is legal.
fn scan_whitespace_and_comments(
    lexer: &mut dyn Lexer,
    scanned_comment: &mut bool,
    consume: bool,
) -> WhitespaceResult {
    let mut saw_block_newline = false;

    loop {
        while is_space(lexer.lookahead()) {
            skip(lexer);
        }

        if lexer.lookahead() == '/' as i32 {
            skip(lexer);

            if lexer.lookahead() == '/' as i32 {
                skip(lexer);
                while lexer.lookahead() != 0 && !is_line_terminator(lexer.lookahead()) {
                    skip(lexer);
                }
                *scanned_comment = true;
            } else if lexer.lookahead() == '*' as i32 {
                skip(lexer);
                while lexer.lookahead() != 0 {
                    if lexer.lookahead() == '*' as i32 {
                        skip(lexer);
                        if lexer.lookahead() == '/' as i32 {
                            skip(lexer);
                            *scanned_comment = true;

                            if lexer.lookahead() != '/' as i32 && !consume {
                                return if saw_block_newline {
                                    WhitespaceResult::Accept
                                } else {
                                    WhitespaceResult::NoNewline
                                };
                            }

                            break;
                        }
                    } else if is_line_terminator(lexer.lookahead()) {
                        saw_block_newline = true;
                        skip(lexer);
                    } else {
                        skip(lexer);
                    }
                }
            } else {
                return WhitespaceResult::Reject;
            }
        } else {
            return WhitespaceResult::Accept;
        }
    }
}

fn scan_automatic_semicolon(
    lexer: &mut dyn Lexer,
    comment_condition: bool,
    scanned_comment: &mut bool,
) -> bool {
    lexer.set_result_symbol(AUTOMATIC_SEMICOLON as u16);
    lexer.mark_end();

    loop {
        if lexer.lookahead() == 0 {
            return true;
        }

        if lexer.lookahead() == '/' as i32 {
            let result = scan_whitespace_and_comments(lexer, scanned_comment, false);
            if result == WhitespaceResult::Reject {
                return false;
            }

            if result == WhitespaceResult::Accept
                && comment_condition
                && lexer.lookahead() != ',' as i32
                && lexer.lookahead() != '=' as i32
            {
                return true;
            }
        }

        if lexer.lookahead() == '}' as i32 {
            return true;
        }

        if lexer.is_at_included_range_start() {
            return true;
        }

        if is_line_terminator(lexer.lookahead()) {
            break;
        }

        if !is_space(lexer.lookahead()) {
            return false;
        }

        skip(lexer);
    }

    skip(lexer);

    if scan_whitespace_and_comments(lexer, scanned_comment, true) == WhitespaceResult::Reject {
        return false;
    }

    match char::from_u32(lexer.lookahead() as u32) {
        Some(
            '`' | ',' | ':' | ';' | '*' | '%' | '>' | '<' | '=' | '[' | '(' | '?' | '^' | '|' | '&'
            | '/',
        ) => return false,

        // Insert before decimal literals, but not a member-access dot.
        Some('.') => {
            skip(lexer);
            return is_digit(lexer.lookahead());
        }

        // Insert before ++ and --, but not binary + and -.
        Some('+') => {
            skip(lexer);
            return lexer.lookahead() == '+' as i32;
        }
        Some('-') => {
            skip(lexer);
            return lexer.lookahead() == '-' as i32;
        }

        // Insert before unary !, but not !=.
        Some('!') => {
            skip(lexer);
            return lexer.lookahead() != '=' as i32;
        }

        // Do not insert before in or instanceof, but do before an identifier.
        Some('i') => {
            skip(lexer);
            if lexer.lookahead() != 'n' as i32 {
                return true;
            }
            skip(lexer);

            if !is_alpha(lexer.lookahead()) {
                return false;
            }

            for c in b"stanceof" {
                if lexer.lookahead() != i32::from(*c) {
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

    if lexer.lookahead() == '?' as i32 {
        advance(lexer);

        if lexer.lookahead() == '?' as i32 {
            return false;
        }

        lexer.mark_end();
        lexer.set_result_symbol(TERNARY_QMARK as u16);

        if lexer.lookahead() == '.' as i32 {
            advance(lexer);
            return is_digit(lexer.lookahead());
        }
        return true;
    }
    false
}

fn scan_html_comment(lexer: &mut dyn Lexer) -> bool {
    while is_space(lexer.lookahead()) || matches!(lexer.lookahead(), 0x2028 | 0x2029) {
        skip(lexer);
    }

    let prefix: &[u8] = if lexer.lookahead() == '<' as i32 {
        b"<!--"
    } else if lexer.lookahead() == '-' as i32 {
        b"-->"
    } else {
        return false;
    };
    for c in prefix {
        if lexer.lookahead() != i32::from(*c) {
            return false;
        }
        advance(lexer);
    }

    while lexer.lookahead() != 0 && !is_line_terminator(lexer.lookahead()) {
        advance(lexer);
    }

    lexer.set_result_symbol(HTML_COMMENT as u16);
    lexer.mark_end();
    true
}

fn scan_jsx_text(lexer: &mut dyn Lexer) -> bool {
    // Text includes non-whitespace, and whitespace other than newlines and the
    // whitespace immediately following them.
    let mut saw_text = false;
    let mut at_newline = false;

    while !matches!(
        char::from_u32(lexer.lookahead() as u32),
        Some('\0' | '<' | '>' | '{' | '}' | '&')
    ) {
        let is_wspace = is_space(lexer.lookahead());
        if lexer.lookahead() == '\n' as i32 {
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

        if valid_symbols[AUTOMATIC_SEMICOLON] {
            let mut scanned_comment = false;
            let ret =
                scan_automatic_semicolon(lexer, !valid_symbols[LOGICAL_OR], &mut scanned_comment);
            if !ret
                && !scanned_comment
                && valid_symbols[TERNARY_QMARK]
                && lexer.lookahead() == '?' as i32
            {
                return scan_ternary_qmark(lexer);
            }
            return ret;
        }

        if valid_symbols[TERNARY_QMARK] {
            return scan_ternary_qmark(lexer);
        }

        if valid_symbols[HTML_COMMENT]
            && !valid_symbols[LOGICAL_OR]
            && !valid_symbols[ESCAPE_SEQUENCE]
            && !valid_symbols[REGEX_PATTERN]
        {
            return scan_html_comment(lexer);
        }

        false
    }

    fn serialize(&mut self, _buffer: &mut [u8]) -> usize {
        0
    }

    fn deserialize(&mut self, _buffer: &[u8]) {}
}

/// Creates the stateless scanner (C returns a null payload).
pub(crate) fn create() -> Box<dyn ExternalScanner> {
    Box::new(Scanner)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    #[derive(Debug, PartialEq, Eq)]
    enum Event {
        Advance(usize, bool),
        MarkEnd(usize),
        Result(u16),
        IncludedRangeStart(usize),
    }

    struct TestLexer {
        input: Vec<i32>,
        position: usize,
        end: Option<usize>,
        symbol: u16,
        range_start: Option<usize>,
        events: RefCell<Vec<Event>>,
    }

    impl TestLexer {
        fn new(input: &str) -> Self {
            Self {
                input: input.chars().map(|c| c as i32).collect(),
                position: 0,
                end: None,
                symbol: u16::MAX,
                range_start: None,
                events: RefCell::new(Vec::new()),
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
            self.events.get_mut().push(Event::Result(symbol));
            self.symbol = symbol;
        }

        fn advance(&mut self, skip: bool) {
            assert!(self.position < self.input.len(), "advanced past EOF");
            self.events
                .get_mut()
                .push(Event::Advance(self.position, skip));
            self.position += 1;
        }

        fn mark_end(&mut self) {
            self.events.get_mut().push(Event::MarkEnd(self.position));
            self.end = Some(self.position);
        }

        fn get_column(&mut self) -> u32 {
            panic!("the C JavaScript scanner never calls get_column")
        }

        fn is_at_included_range_start(&self) -> bool {
            self.events
                .borrow_mut()
                .push(Event::IncludedRangeStart(self.position));
            self.range_start == Some(self.position)
        }

        fn eof(&self) -> bool {
            panic!("the C JavaScript scanner checks lookahead, not eof")
        }
    }

    fn valid(tokens: &[usize]) -> [bool; 8] {
        let mut result = [false; 8];
        for &token in tokens {
            result[token] = true;
        }
        result
    }

    fn scan(input: &str, tokens: &[usize]) -> (bool, TestLexer) {
        let mut lexer = TestLexer::new(input);
        let accepted = create().scan(&mut lexer, &valid(tokens));
        (accepted, lexer)
    }

    #[test]
    fn serialization_is_empty_and_does_not_touch_the_buffer() {
        let mut scanner = create();
        let mut buffer = [0xa5; 1024];
        scanner.deserialize(&[1, 2, 3]);
        assert_eq!(scanner.serialize(&mut buffer), 0);
        assert_eq!(buffer, [0xa5; 1024]);
        scanner.deserialize(&[]);
        assert_eq!(scanner.serialize(&mut []), 0);
        assert!(scanner.scan(&mut TestLexer::new("?"), &valid(&[TERNARY_QMARK])));
    }

    #[test]
    fn template_boundaries_and_lookahead() {
        for (input, accepted, position, end) in [
            ("abc${x}", true, 4, 3),
            ("${x}", false, 1, 0),
            ("$${x}", true, 2, 1),
            ("$foo`", true, 4, 4),
            ("`", false, 0, 0),
            ("abc\\n", true, 3, 3),
            ("\\n", false, 0, 0),
            ("abc", false, 3, 3),
            ("abc\0more", false, 3, 3),
            ("", false, 0, 0),
        ] {
            let (actual, lexer) = scan(input, &[TEMPLATE_CHARS]);
            assert_eq!(actual, accepted, "{input:?}");
            assert_eq!(lexer.position, position, "{input:?}");
            assert_eq!(lexer.end, Some(end), "{input:?}");
            assert_eq!(lexer.symbol, TEMPLATE_CHARS as u16);
        }

        let (_, lexer) = scan("a${", &[TEMPLATE_CHARS]);
        assert_eq!(
            lexer.events.into_inner(),
            [
                Event::Result(TEMPLATE_CHARS as u16),
                Event::MarkEnd(0),
                Event::Advance(0, false),
                Event::MarkEnd(1),
                Event::Advance(1, false),
            ]
        );
    }

    #[test]
    fn semicolon_before_tokens_after_a_newline() {
        for (suffix, accepted) in [
            ("", true),
            ("identifier", true),
            ("}", true),
            ("`", false),
            (",", false),
            (":", false),
            (";", false),
            ("*", false),
            ("%", false),
            (">", false),
            ("<", false),
            ("=", false),
            ("[", false),
            ("(", false),
            ("?", false),
            ("^", false),
            ("|", false),
            ("&", false),
            ("/", false),
            (".0", true),
            (".x", false),
            (".\u{0660}", false),
            ("++x", true),
            ("+x", false),
            ("--x", true),
            ("-x", false),
            ("!x", true),
            ("!=x", false),
            ("in", false),
            ("in_", false),
            ("in0", false),
            ("in\u{00e9}", false),
            ("index", true),
            ("instanceof", false),
            ("instanceof_", false),
            ("instanceofA", true),
        ] {
            let input = format!("\n\t {suffix}");
            let (actual, lexer) = scan(&input, &[AUTOMATIC_SEMICOLON]);
            assert_eq!(actual, accepted, "{input:?}");
            assert_eq!(lexer.end, Some(0), "semicolon must remain zero-width");
            assert_eq!(lexer.symbol, AUTOMATIC_SEMICOLON as u16);
            assert!(
                lexer
                    .events
                    .borrow()
                    .iter()
                    .all(|event| { !matches!(event, Event::Advance(_, false)) })
            );
        }
    }

    #[test]
    fn semicolon_whitespace_and_range_boundaries() {
        for (input, accepted, position) in [
            ("", true, 0),
            ("}", true, 0),
            (" \t}", true, 2),
            (" \tname", false, 2),
            ("\rname", false, 1),
            ("\u{000b}\nname", true, 2),
            ("\u{2028}name", true, 1),
            ("\u{2029}name", true, 1),
            ("\u{00a0}\nname", false, 0),
        ] {
            let (actual, lexer) = scan(input, &[AUTOMATIC_SEMICOLON]);
            assert_eq!(actual, accepted, "{input:?}");
            assert_eq!(lexer.position, position, "{input:?}");
        }

        let mut lexer = TestLexer::new(" identifier");
        lexer.range_start = Some(1);
        assert!(Scanner.scan(&mut lexer, &valid(&[AUTOMATIC_SEMICOLON])));
        assert_eq!(
            lexer.events.into_inner(),
            [
                Event::Result(AUTOMATIC_SEMICOLON as u16),
                Event::MarkEnd(0),
                Event::IncludedRangeStart(0),
                Event::Advance(0, true),
                Event::IncludedRangeStart(1),
            ]
        );
    }

    #[test]
    fn semicolon_comments_preserve_c_early_returns() {
        for (input, condition, accepted, scanned) in [
            ("/", true, false, false),
            ("/* unterminated", true, true, false),
            ("/* unterminated", false, false, false),
            ("/**/", true, false, true),
            ("/**/ ", true, true, true),
            ("/**/x", true, false, true),
            ("/*\n*/x", true, true, true),
            ("/*\n*/x", false, false, true),
            ("/*\r*/x", true, false, true),
            ("/*\u{2028}*/x", true, true, true),
            ("/*\u{2029}*/x", true, true, true),
            ("/*\n*/,", true, false, true),
            ("/*\n*/=", true, false, true),
            ("/*\n*/ ,", true, true, true),
            ("/*\n*//**/x", true, true, true),
            ("/**//x", true, false, true),
            ("//x\nx", true, true, true),
            ("//x\nx", false, false, true),
            ("//x\u{2028}x", false, true, true),
            ("\n/**/x", false, true, true),
            ("\n/**/(", true, false, true),
        ] {
            let mut lexer = TestLexer::new(input);
            let mut scanned_comment = false;
            assert_eq!(
                scan_automatic_semicolon(&mut lexer, condition, &mut scanned_comment),
                accepted,
                "{input:?}, condition={condition}"
            );
            assert_eq!(scanned_comment, scanned, "{input:?}");
            assert_eq!(lexer.end, Some(0));
        }
    }

    #[test]
    fn ternary_qmark_distinguishes_optional_chain_and_nullish_coalescing() {
        for (input, accepted, position, end) in [
            ("?", true, 1, Some(1)),
            ("??", false, 1, None),
            ("?.x", false, 2, Some(1)),
            ("?.0", true, 2, Some(1)),
            ("?.\u{0660}", false, 2, Some(1)),
            (" \t? x", true, 3, Some(3)),
            ("\u{000b}?", true, 2, Some(2)),
            ("\u{2028}?", false, 0, None),
            ("x", false, 0, None),
        ] {
            let (actual, lexer) = scan(input, &[TERNARY_QMARK]);
            assert_eq!(actual, accepted, "{input:?}");
            assert_eq!(lexer.position, position, "{input:?}");
            assert_eq!(lexer.end, end, "{input:?}");
        }

        let (_, lexer) = scan(" ?.2", &[TERNARY_QMARK]);
        assert_eq!(
            lexer.events.into_inner(),
            [
                Event::Advance(0, true),
                Event::Advance(1, false),
                Event::MarkEnd(2),
                Event::Result(TERNARY_QMARK as u16),
                Event::Advance(2, false),
            ]
        );
    }

    #[test]
    fn semicolon_falls_back_to_ternary_only_without_a_comment() {
        let tokens = [AUTOMATIC_SEMICOLON, TERNARY_QMARK];
        let (accepted, lexer) = scan("\n?.2", &tokens);
        assert!(accepted);
        assert_eq!(lexer.symbol, TERNARY_QMARK as u16);
        assert_eq!(lexer.position, 3);
        assert_eq!(lexer.end, Some(2));

        let (accepted, lexer) = scan("\n/**/?.2", &tokens);
        assert!(!accepted);
        assert_eq!(lexer.symbol, AUTOMATIC_SEMICOLON as u16);
        assert_eq!(lexer.lookahead(), '?' as i32);
        assert_eq!(lexer.end, Some(0));
    }

    #[test]
    fn html_comments_stop_at_line_terminators_but_not_carriage_return() {
        for (input, accepted, position) in [
            ("<!--x\ny", true, 5),
            ("-->x\u{2028}y", true, 4),
            ("<!--x\u{2029}y", true, 5),
            ("<!--x\ry", true, 7),
            (" \u{2028}<!--x", true, 7),
            ("<!--x\0y", true, 5),
            ("<!--", true, 4),
            ("<!-x", false, 3),
            ("--x", false, 2),
            ("x", false, 0),
        ] {
            let (actual, lexer) = scan(input, &[HTML_COMMENT]);
            assert_eq!(actual, accepted, "{input:?}");
            assert_eq!(lexer.position, position, "{input:?}");
            if accepted {
                assert_eq!(lexer.symbol, HTML_COMMENT as u16);
                assert_eq!(lexer.end, Some(position));
            }
        }
    }

    #[test]
    fn jsx_text_whitespace_and_delimiters() {
        for (input, accepted, position) in [
            ("", false, 0),
            ("<", false, 0),
            (">", false, 0),
            ("{", false, 0),
            ("}", false, 0),
            ("&", false, 0),
            ("text<", true, 4),
            ("text&", true, 4),
            ("text\0more", true, 4),
            ("  ", true, 2),
            (" \n", true, 2),
            ("\n\t \n", false, 4),
            ("\n\t x<", true, 4),
            ("\n\u{000b}<", false, 2),
            ("\n\u{00a0}<", true, 2),
            ("\u{2028}<", true, 1),
            ("\r\n<", true, 2),
        ] {
            let (actual, lexer) = scan(input, &[JSX_TEXT]);
            assert_eq!(actual, accepted, "{input:?}");
            assert_eq!(lexer.position, position, "{input:?}");
            assert_eq!(lexer.symbol, JSX_TEXT as u16);
            assert_eq!(lexer.end, None, "JSX text never explicitly marks its end");
            assert!(
                lexer
                    .events
                    .borrow()
                    .iter()
                    .all(|event| { !matches!(event, Event::Advance(_, true)) })
            );
        }
    }

    #[test]
    fn dispatch_priority_and_disabled_tokens() {
        let (accepted, lexer) = scan("text`", &[TEMPLATE_CHARS, AUTOMATIC_SEMICOLON, JSX_TEXT]);
        assert!(!accepted);
        assert!(lexer.events.borrow().is_empty());

        let (accepted, lexer) = scan("text`", &[TEMPLATE_CHARS, JSX_TEXT]);
        assert!(accepted);
        assert_eq!(lexer.symbol, TEMPLATE_CHARS as u16);

        let (accepted, lexer) = scan("\n  }", &[JSX_TEXT, AUTOMATIC_SEMICOLON]);
        assert!(accepted);
        assert_eq!(lexer.symbol, AUTOMATIC_SEMICOLON as u16);
        assert_eq!(lexer.end, Some(3), "failed JSX scan is not rewound");

        for blocker in [LOGICAL_OR, ESCAPE_SEQUENCE, REGEX_PATTERN] {
            let (accepted, lexer) = scan("<!--x", &[HTML_COMMENT, blocker]);
            assert!(!accepted);
            assert!(lexer.events.borrow().is_empty());
        }

        let (accepted, lexer) = scan("<!--x", &[HTML_COMMENT, TERNARY_QMARK]);
        assert!(!accepted);
        assert_eq!(lexer.position, 0);

        let (accepted, lexer) = scan("anything", &[]);
        assert!(!accepted);
        assert!(lexer.events.borrow().is_empty());
    }
}
