//! The Kotlin grammar's stateless external scanner, translated from `scanner.c`.

use ts_port_tables::{ExternalScanner, Lexer};

// TokenType indices from the C scanner.
const SEMI: usize = 0;
const CLASS_MEMBER_SEMI: usize = 1;
const BLOCK_COMMENT: usize = 2;
const NOT_IS: usize = 3;
const IN: usize = 4;
const Q_DOT: usize = 5;
const MULTILINE_STRING_CONTENT: usize = 6;
const CONSTRUCTOR: usize = 7;
const GET: usize = 8;
const SET: usize = 9;
const DOLLAR: usize = 10;

const MAX_WORD_SIZE: usize = 16;

/// The C scanner reads `TSLexer.lookahead` directly. Cache that field locally so
/// repeated tests at one position do not dispatch through `dyn Lexer` each time.
/// Only `advance` changes it; marks and result-symbol updates leave it intact.
struct Cursor<'a> {
    lexer: &'a mut dyn Lexer,
    lookahead: i32,
}

impl<'a> Cursor<'a> {
    fn new(lexer: &'a mut dyn Lexer) -> Self {
        let lookahead = lexer.lookahead();
        Self { lexer, lookahead }
    }

    fn lookahead(&self) -> i32 {
        self.lookahead
    }

    fn advance(&mut self, skip: bool) {
        self.lexer.advance(skip);
        self.lookahead = self.lexer.lookahead();
    }

    fn mark_end(&mut self) {
        self.lexer.mark_end();
    }

    fn set_result_symbol(&mut self, symbol: u16) {
        self.lexer.set_result_symbol(symbol);
    }

    fn eof(&self) -> bool {
        // Nonzero lookahead cannot be EOF. A zero can also be an embedded NUL,
        // so only that case needs the runtime's end-of-input test.
        self.lookahead == 0 && self.lexer.eof()
    }
}

// The reference process uses the default C locale, not Unicode character classes.
fn is_alpha(c: i32) -> bool {
    matches!(c, 0x41..=0x5a | 0x61..=0x7a)
}

fn is_digit(c: i32) -> bool {
    matches!(c, 0x30..=0x39)
}

fn is_alnum(c: i32) -> bool {
    is_alpha(c) || is_digit(c)
}

fn is_space(c: i32) -> bool {
    matches!(c, 0x09..=0x0d | 0x20)
}

fn scan_word(lexer: &mut Cursor<'_>, word: &[u8]) -> bool {
    for &c in word {
        if lexer.lookahead() != i32::from(c) {
            return false;
        }
        lexer.advance(true);
    }
    true
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SemiWord {
    Modifier,
    Else,
    In,
    Get,
    Set,
    Constructor,
    Suppress,
    Other,
}

fn scan_semi_keyword(lexer: &mut Cursor<'_>) -> SemiWord {
    let mut word = [0; MAX_WORD_SIZE - 1];
    let mut length = 0;
    // Like C's scan_words, inspect at most fifteen ASCII letters, including
    // for an unknown identifier. The inspected extent matters on backtracking.
    while length < word.len() && is_alpha(lexer.lookahead()) {
        word[length] = lexer.lookahead() as u8;
        length += 1;
        lexer.advance(true);
    }
    // Match once by length and bytes instead of searching both NUL-padded
    // sixteen-entry tables (and rediscovering the word length for each table).
    match &word[..length] {
        b"public" | b"private" | b"protected" | b"internal" | b"abstract" | b"final" | b"open"
        | b"override" | b"lateinit" | b"vararg" | b"noinline" | b"crossinline" | b"external"
        | b"suspend" | b"inline" => SemiWord::Modifier,
        b"else" => SemiWord::Else,
        b"in" => SemiWord::In,
        b"get" => SemiWord::Get,
        b"set" => SemiWord::Set,
        b"constructor" => SemiWord::Constructor,
        b"instanceof" | b"by" | b"as" | b"where" => SemiWord::Suppress,
        _ => SemiWord::Other,
    }
}

/// `None` means that the C multiline-string loop reached EOF and fell through
/// to the other token scanners, rather than returning a string-content token.
fn scan_multiline_string(lexer: &mut Cursor<'_>) -> Option<bool> {
    let mut did_advance = false;
    lexer.set_result_symbol(MULTILINE_STRING_CONTENT as u16);
    while !lexer.eof() {
        match lexer.lookahead() {
            0x24 => {
                // '$'
                lexer.mark_end();
                lexer.advance(false);
                if is_alpha(lexer.lookahead()) || lexer.lookahead() == i32::from(b'{') {
                    return Some(did_advance);
                }
                did_advance = true;
            }
            0x22 => {
                // '"'
                lexer.mark_end();
                // Three or four quotes end the content. They are examined but
                // excluded from it by the mark before the first quote.
                lexer.advance(false);
                if lexer.lookahead() == i32::from(b'"') {
                    lexer.advance(false);
                    if lexer.lookahead() == i32::from(b'"') {
                        lexer.advance(false);
                        if lexer.lookahead() == i32::from(b'"') {
                            lexer.advance(false);
                        }
                        return Some(did_advance);
                    }
                }
                did_advance = true;
            }
            _ => {
                lexer.advance(false);
                did_advance = true;
            }
        }
    }
    None
}

/// Enter at C's `continue_not_is_from_semi` label, after consuming `!`. This
/// deliberately does not check NOT_IS validity: the semicolon path bypasses it.
fn scan_not_is_tail(lexer: &mut Cursor<'_>) -> Option<bool> {
    if lexer.lookahead() == i32::from(b'i') {
        lexer.advance(false);
        if lexer.lookahead() == i32::from(b's') {
            lexer.advance(false);
            lexer.set_result_symbol(NOT_IS as u16);
            lexer.mark_end();
            return Some(!is_alnum(lexer.lookahead()));
        }
    }
    None
}

// These short fallthrough checks mirror C's goto labels. Keep them in the
// caller: most external scans reject a token without reaching a scanning loop.
#[inline(always)]
fn scan_in_and_rest(lexer: &mut Cursor<'_>, valid_symbols: &[bool]) -> bool {
    if valid_symbols[IN] && lexer.lookahead() == i32::from(b'i') {
        lexer.advance(false);
        if lexer.lookahead() == i32::from(b'n') {
            lexer.advance(false);
            lexer.set_result_symbol(IN as u16);
            lexer.mark_end();
            return !is_alnum(lexer.lookahead());
        }
    }
    scan_q_dot_and_comment(lexer, valid_symbols)
}

/// C's `q_dot_from_semi` label, including fallthrough into `comment`.
#[inline(always)]
fn scan_q_dot_and_comment(lexer: &mut Cursor<'_>, valid_symbols: &[bool]) -> bool {
    if valid_symbols[Q_DOT] {
        while is_space(lexer.lookahead()) {
            lexer.advance(true);
        }
        if lexer.lookahead() == i32::from(b'?') {
            lexer.advance(false);
            while is_space(lexer.lookahead()) {
                lexer.advance(true);
            }
            if lexer.lookahead() == i32::from(b'.') {
                lexer.advance(false);
                lexer.set_result_symbol(Q_DOT as u16);
                lexer.mark_end();
                return true;
            }
        }
    }
    scan_comment(lexer, valid_symbols)
}

/// C's `comment` label. BLOCK_COMMENT validity is intentionally not tested.
#[inline(always)]
fn scan_comment(lexer: &mut Cursor<'_>, valid_symbols: &[bool]) -> bool {
    if valid_symbols[DOLLAR] {
        return false;
    }

    if lexer.lookahead() != i32::from(b'/') {
        return false;
    }
    // This is terminal: every caller immediately returns the result, so the
    // body can own its lookahead cache instead of writing back into this one.
    scan_comment_body(lexer.lexer)
}

// Outline only the actual comment scan, not the common rejection checks. The
// local cursor starts after consuming '/', with no duplicate lookahead read.
#[inline(never)]
fn scan_comment_body(lexer: &mut dyn Lexer) -> bool {
    lexer.advance(false);
    let mut lexer = Cursor::new(lexer);
    if lexer.lookahead() != i32::from(b'*') {
        return false;
    }
    lexer.advance(false);

    let mut after_star = false;
    let mut nesting_depth = 1u32;
    loop {
        match lexer.lookahead() {
            0x00 => return false,
            0x2a => {
                // '*'
                lexer.advance(false);
                after_star = true;
            }
            0x2f => {
                // '/'
                if after_star {
                    lexer.advance(false);
                    after_star = false;
                    nesting_depth = nesting_depth.wrapping_sub(1);
                    if nesting_depth == 0 {
                        lexer.set_result_symbol(BLOCK_COMMENT as u16);
                        lexer.mark_end();
                        return true;
                    }
                } else {
                    lexer.advance(false);
                    after_star = false;
                    if lexer.lookahead() == i32::from(b'*') {
                        nesting_depth = nesting_depth.wrapping_add(1);
                        lexer.advance(false);
                    }
                }
            }
            _ => {
                lexer.advance(false);
                after_star = false;
            }
        }
    }
}

fn scan_semi_word(lexer: &mut Cursor<'_>, valid_symbols: &[bool]) -> bool {
    let mut word = scan_semi_keyword(lexer);
    while word == SemiWord::Modifier {
        while is_space(lexer.lookahead()) {
            lexer.advance(true);
        }
        word = scan_semi_keyword(lexer);
    }

    match word {
        SemiWord::Constructor => {
            // A secondary constructor, or a variable named `constructor`.
            while is_space(lexer.lookahead()) {
                lexer.advance(true);
            }
            if valid_symbols[CLASS_MEMBER_SEMI]
                || lexer.lookahead() == i32::from(b'.')
                || lexer.lookahead() == i32::from(b'=')
            {
                return true;
            }
        }
        SemiWord::Else => {
            // `else` suppresses a semi, except before a `when` entry's arrow.
            while is_space(lexer.lookahead()) {
                lexer.advance(true);
            }
            if lexer.lookahead() == i32::from(b'-') {
                lexer.advance(true);
                if lexer.lookahead() == i32::from(b'>') {
                    return true;
                }
            }
        }
        SemiWord::Get if !valid_symbols[GET] || lexer.lookahead() == i32::from(b'[') => {
            return true;
        }
        SemiWord::Set
            if !valid_symbols[SET]
                || lexer.lookahead() == i32::from(b'[')
                || lexer.lookahead() == i32::from(b'(')
                || lexer.lookahead() == i32::from(b'.') =>
        {
            if lexer.lookahead() == i32::from(b'(') && valid_symbols[SET] {
                while lexer.lookahead() != i32::from(b')') && !lexer.eof() {
                    lexer.advance(true);
                }
                // This skip also occurs at EOF in the C scanner.
                lexer.advance(true);
                while is_space(lexer.lookahead()) {
                    if lexer.lookahead() == i32::from(b'\n') {
                        return true;
                    }
                    lexer.advance(true);
                }
                return false;
            }
            return true;
        }
        SemiWord::In if valid_symbols[IN] => return true,
        _ => {}
    }
    word == SemiWord::Other
}

/// The switch after a newline. A loop replaces the annotation branch's goto
/// back to `_switch`, without repeating the semicolon's initial mark_end.
fn scan_after_newline(lexer: &mut Cursor<'_>, valid_symbols: &[bool]) -> bool {
    loop {
        match lexer.lookahead() {
            // , . : * % > < = { [ | & /
            0x2c | 0x2e | 0x3a | 0x2a | 0x25 | 0x3e | 0x3c | 0x3d | 0x7b | 0x5b | 0x7c | 0x26
            | 0x2f => {
                return false;
            }
            0x2b => {
                // '+'
                lexer.advance(true);
                if lexer.lookahead() == i32::from(b'+') {
                    return true;
                }
                return is_digit(lexer.lookahead());
            }
            0x2d => {
                // '-'
                lexer.advance(true);
                if lexer.lookahead() == i32::from(b'-') {
                    return true;
                }
                return is_digit(lexer.lookahead());
            }
            0x21 => {
                // '!'
                lexer.advance(true);
                if lexer.lookahead() == i32::from(b'i') && valid_symbols[NOT_IS] {
                    lexer.advance(true);
                    if lexer.lookahead() == i32::from(b's') {
                        lexer.advance(true);
                        if !is_alnum(lexer.lookahead()) {
                            return true;
                        }
                    }
                }
                return lexer.lookahead() != i32::from(b'=');
            }
            0x3f => {
                // '?'
                if valid_symbols[Q_DOT] {
                    return scan_q_dot_and_comment(lexer, valid_symbols);
                }
                return true;
            }
            // e i g s p a f o l v n c b w
            0x65 | 0x69 | 0x67 | 0x73 | 0x70 | 0x61 | 0x66 | 0x6f | 0x6c | 0x76 | 0x6e | 0x63
            | 0x62 | 0x77 => {
                return scan_semi_word(lexer, valid_symbols);
            }
            0x3b => {
                // ';'
                lexer.advance(false);
                lexer.mark_end();
                return true;
            }
            0x40 => {
                // '@'
                if valid_symbols[CONSTRUCTOR] {
                    while !is_space(lexer.lookahead()) {
                        lexer.advance(true);
                    }
                    while is_space(lexer.lookahead()) {
                        lexer.advance(true);
                    }
                    return !scan_word(lexer, b"constructor");
                }
                if valid_symbols[GET] || valid_symbols[SET] {
                    let mut saw_paren = false;
                    while if saw_paren {
                        lexer.lookahead() != i32::from(b'\n')
                    } else {
                        !is_space(lexer.lookahead())
                    } {
                        lexer.advance(true);
                        if lexer.lookahead() == i32::from(b'(') {
                            saw_paren = true;
                        }
                        if lexer.lookahead() == i32::from(b')') {
                            saw_paren = false;
                        }
                    }
                    while is_space(lexer.lookahead()) {
                        lexer.advance(true);
                    }
                    if lexer.lookahead() == i32::from(b'/') {
                        return true;
                    }
                    continue;
                }
                return true;
            }
            _ => return true,
        }
    }
}

fn scan_semi(lexer: &mut Cursor<'_>, valid_symbols: &[bool]) -> bool {
    lexer.set_result_symbol(if valid_symbols[SEMI] {
        SEMI as u16
    } else {
        CLASS_MEMBER_SEMI as u16
    });
    lexer.mark_end();
    let mut saw_newline = false;
    loop {
        if lexer.eof() {
            return true;
        }
        if lexer.lookahead() == i32::from(b';') {
            lexer.advance(false);
            lexer.mark_end();
            return true;
        }
        if !is_space(lexer.lookahead()) {
            break;
        }
        if lexer.lookahead() == i32::from(b'\n') {
            lexer.advance(true);
            saw_newline = true;
            break;
        }
        if lexer.lookahead() == i32::from(b'\r') {
            lexer.advance(true);
            if lexer.lookahead() == i32::from(b'\n') {
                lexer.advance(true);
            }
            saw_newline = true;
            break;
        }
        lexer.advance(true);
    }

    while is_space(lexer.lookahead()) {
        lexer.advance(true);
    }
    if lexer.lookahead() == i32::from(b'/') {
        return scan_comment(lexer, valid_symbols);
    }

    if !saw_newline {
        return match lexer.lookahead() {
            0x21 => {
                // '!'
                lexer.advance(true);
                if let Some(result) = scan_not_is_tail(lexer) {
                    return result;
                }
                scan_in_and_rest(lexer, valid_symbols)
            }
            0x3f if valid_symbols[Q_DOT] => scan_q_dot_and_comment(lexer, valid_symbols),
            0x69 => scan_word(lexer, b"import"), // 'i'
            0x3b => {
                // ';'
                lexer.advance(false);
                lexer.mark_end();
                true
            }
            _ => false,
        };
    }
    scan_after_newline(lexer, valid_symbols)
}

/// C's payload is NULL; no state is retained between calls.
pub(crate) struct Scanner;

impl ExternalScanner for Scanner {
    fn scan(&mut self, lexer: &mut dyn Lexer, valid_symbols: &[bool]) -> bool {
        let lexer = &mut Cursor::new(lexer);
        if valid_symbols[MULTILINE_STRING_CONTENT]
            && let Some(result) = scan_multiline_string(lexer)
        {
            return result;
        }
        if valid_symbols[SEMI] || valid_symbols[CLASS_MEMBER_SEMI] {
            return scan_semi(lexer, valid_symbols);
        }
        while is_space(lexer.lookahead()) {
            lexer.advance(true);
        }
        if valid_symbols[NOT_IS] && lexer.lookahead() == i32::from(b'!') {
            lexer.advance(false);
            if let Some(result) = scan_not_is_tail(lexer) {
                return result;
            }
        }
        scan_in_and_rest(lexer, valid_symbols)
    }

    fn serialize(&mut self, _buffer: &mut [u8]) -> usize {
        0
    }

    fn deserialize(&mut self, _buffer: &[u8]) {}
}

/// Creates a scanner (C's `tree_sitter_kotlin_external_scanner_create`).
pub(crate) fn create() -> Box<dyn ExternalScanner> {
    Box::new(Scanner)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    #[derive(Debug, PartialEq, Eq)]
    enum Event {
        Advance(usize, bool),
        Mark(usize),
        Symbol(u16),
    }

    struct TestLexer<'a> {
        input: &'a str,
        position: usize,
        end: Option<usize>,
        symbol: u16,
        events: Vec<Event>,
        lookahead_calls: Cell<usize>,
        eof_calls: Cell<usize>,
    }

    impl<'a> TestLexer<'a> {
        fn new(input: &'a str) -> Self {
            Self {
                input,
                position: 0,
                end: None,
                symbol: u16::MAX,
                events: Vec::new(),
                lookahead_calls: Cell::new(0),
                eof_calls: Cell::new(0),
            }
        }
    }

    impl Lexer for TestLexer<'_> {
        fn lookahead(&self) -> i32 {
            self.lookahead_calls.set(self.lookahead_calls.get() + 1);
            self.input[self.position..]
                .chars()
                .next()
                .map_or(0, |c| c as i32)
        }

        fn result_symbol(&self) -> u16 {
            self.symbol
        }

        fn set_result_symbol(&mut self, symbol: u16) {
            self.symbol = symbol;
            self.events.push(Event::Symbol(symbol));
        }

        fn advance(&mut self, skip: bool) {
            self.events.push(Event::Advance(self.position, skip));
            if let Some(c) = self.input[self.position..].chars().next() {
                self.position += c.len_utf8();
            }
        }

        fn mark_end(&mut self) {
            self.end = Some(self.position);
            self.events.push(Event::Mark(self.position));
        }

        fn get_column(&mut self) -> u32 {
            panic!("Kotlin's scanner does not call get_column")
        }

        fn is_at_included_range_start(&self) -> bool {
            panic!("Kotlin's scanner does not test included-range starts")
        }

        fn eof(&self) -> bool {
            self.eof_calls.set(self.eof_calls.get() + 1);
            self.position == self.input.len()
        }
    }

    fn scan<'a>(input: &'a str, tokens: &[usize]) -> (bool, TestLexer<'a>) {
        let mut valid = [false; DOLLAR + 1];
        for &token in tokens {
            valid[token] = true;
        }
        let mut lexer = TestLexer::new(input);
        let result = Scanner.scan(&mut lexer, &valid);
        (result, lexer)
    }

    #[test]
    fn cursor_caches_lookahead_but_distinguishes_nul_from_eof() {
        let mut lexer = TestLexer::new("a\0é");
        {
            let mut cursor = Cursor::new(&mut lexer);
            assert_eq!(cursor.lookahead(), i32::from(b'a'));
            assert!(!cursor.eof());
            cursor.mark_end();
            cursor.set_result_symbol(SEMI as u16);
            assert_eq!(cursor.lookahead(), i32::from(b'a'));
            cursor.advance(true);
            assert_eq!(cursor.lookahead(), 0);
            assert!(!cursor.eof());
            cursor.advance(false);
            assert_eq!(cursor.lookahead(), 'é' as i32);
            assert!(!cursor.eof());
            cursor.advance(false);
            assert_eq!(cursor.lookahead(), 0);
            assert!(cursor.eof());
        }
        // One initial read and one per advance, rather than one per test.
        assert_eq!(lexer.lookahead_calls.get(), 4);
        // Only the embedded NUL and the actual end need virtual EOF checks.
        assert_eq!(lexer.eof_calls.get(), 2);
    }

    #[test]
    fn nul_and_non_ascii_are_not_ascii_delimiters() {
        let (result, lexer) = scan("\0", &[SEMI]);
        assert!(!result);
        assert_eq!((lexer.position, lexer.end), (0, Some(0)));
        let (result, lexer) = scan("\n\0", &[SEMI]);
        assert!(result);
        assert_eq!((lexer.position, lexer.end), (1, Some(0)));

        // NUL is ordinary raw-string content, unlike in a block comment. The
        // following Unicode characters have the same low byte as ASCII quotes
        // and '$', but must also remain ordinary content.
        let content = "a\0\u{122}\u{124}";
        let input = format!("{content}$foo");
        let (result, lexer) = scan(&input, &[MULTILINE_STRING_CONTENT]);
        assert!(result);
        assert_eq!(
            (lexer.position, lexer.end),
            (content.len() + 1, Some(content.len()))
        );

        for c in ['\u{121}', '\u{13b}', '\u{12f}', '\u{165}'] {
            let input = format!("\n{c}else");
            let (result, lexer) = scan(&input, &[SEMI]);
            assert!(result, "{input:?}");
            assert_eq!((lexer.position, lexer.end), (1, Some(0)), "{input:?}");
        }
    }

    #[test]
    fn stateless_serialization() {
        let mut scanner = create();
        let mut buffer = [0xa5; 32];
        assert_eq!(scanner.serialize(&mut buffer), 0);
        assert_eq!(buffer, [0xa5; 32]);
        scanner.deserialize(&[1, 2, 3]);
        assert_eq!(scanner.serialize(&mut []), 0);
        scanner.deserialize(&[]);
        assert_eq!(scanner.serialize(&mut buffer), 0);
    }

    #[test]
    fn multiline_string_boundaries() {
        for (input, success, position, end) in [
            ("abc$foo", true, 4, Some(3)),
            ("$foo", false, 1, Some(0)),
            ("abc${foo}", true, 4, Some(3)),
            ("$1$foo", true, 3, Some(2)),
            ("$_$foo", true, 3, Some(2)),
            ("$é$foo", true, 4, Some(3)),
            ("abc\"\"\"tail", true, 6, Some(3)),
            ("abc\"\"\"\"tail", true, 7, Some(3)),
            ("abc\"\"\"\"\"tail", true, 7, Some(3)),
            ("\"\"\"tail", false, 3, Some(0)),
            ("abc\"x$foo", true, 6, Some(5)),
            ("abc\"\"x$foo", true, 7, Some(6)),
            ("unterminated", false, 12, None),
            ("abc\"", false, 4, Some(3)),
        ] {
            let (result, lexer) = scan(input, &[MULTILINE_STRING_CONTENT]);
            assert_eq!(
                (result, lexer.symbol, lexer.position, lexer.end),
                (success, MULTILINE_STRING_CONTENT as u16, position, end),
                "{input:?}"
            );
        }

        // Reaching EOF falls through, even if that changes the result symbol.
        let (result, lexer) = scan("abc", &[MULTILINE_STRING_CONTENT, SEMI]);
        assert!(result);
        assert_eq!((lexer.symbol, lexer.end), (SEMI as u16, Some(3)));
    }

    #[test]
    fn goto_not_is_preserves_skip_flags_and_bypasses_validity() {
        use Event::{Advance, Mark, Symbol};
        let (result, lexer) = scan("!is_", &[SEMI]);
        assert!(result);
        assert_eq!(
            lexer.events,
            [
                Symbol(SEMI as u16),
                Mark(0),
                Advance(0, true),
                Advance(1, false),
                Advance(2, false),
                Symbol(NOT_IS as u16),
                Mark(3),
            ]
        );

        let (result, lexer) = scan("!is_", &[NOT_IS]);
        assert!(result);
        assert_eq!(
            lexer.events,
            [
                Advance(0, false),
                Advance(1, false),
                Advance(2, false),
                Symbol(NOT_IS as u16),
                Mark(3),
            ]
        );

        let (result, lexer) = scan("\n!is=", &[SEMI, NOT_IS]);
        assert!(result);
        assert_eq!(
            lexer.events,
            [
                Symbol(SEMI as u16),
                Mark(0),
                Advance(0, true),
                Advance(1, true),
                Advance(2, true),
                Advance(3, true),
            ]
        );
    }

    #[test]
    fn keyword_boundaries_and_partial_match_fallthrough() {
        for (input, valid, success, symbol, position, end) in [
            ("!is_", vec![NOT_IS], true, NOT_IS as u16, 3, Some(3)),
            ("!is9", vec![NOT_IS], false, NOT_IS as u16, 3, Some(3)),
            ("!isλ", vec![NOT_IS], true, NOT_IS as u16, 3, Some(3)),
            ("in_", vec![IN], true, IN as u16, 2, Some(2)),
            ("inside", vec![IN], false, IN as u16, 2, Some(2)),
            ("!in_", vec![SEMI, IN], false, SEMI as u16, 2, Some(0)),
            ("!iin_", vec![SEMI, IN], true, IN as u16, 4, Some(4)),
            ("imported", vec![SEMI], true, SEMI as u16, 6, Some(0)),
            ("important", vec![SEMI], true, SEMI as u16, 6, Some(0)),
            ("imp", vec![SEMI], false, SEMI as u16, 3, Some(0)),
        ] {
            let (result, lexer) = scan(input, &valid);
            assert_eq!(
                (result, lexer.symbol, lexer.position, lexer.end),
                (success, symbol, position, end),
                "{input:?} with {valid:?}"
            );
        }
    }

    #[test]
    fn semicolon_whitespace_operators_and_end_of_input() {
        for (input, success, position, end) in [
            ("", true, 0, 0),
            (" \t", true, 2, 0),
            (" ;", true, 2, 2),
            ("\n;", true, 2, 2),
            ("\nfoo", true, 4, 0),
            ("\r\nfoo", true, 5, 0),
            ("\rfoo", true, 4, 0),
            ("\u{b}\u{c}foo", false, 2, 0),
            ("\u{a0}foo", false, 0, 0),
            ("\n,", false, 1, 0),
            ("\n++x", true, 2, 0),
            ("\n--x", true, 2, 0),
            ("\n+2.0", true, 2, 0),
            ("\n-2.0", true, 2, 0),
            ("\n+ 2", false, 2, 0),
            ("\n- x", false, 2, 0),
            ("\n!=", false, 2, 0),
            ("\n!x", true, 2, 0),
            ("\n?x", true, 1, 0),
            ("?x", false, 0, 0),
        ] {
            let (result, lexer) = scan(input, &[SEMI]);
            assert_eq!(
                (result, lexer.symbol, lexer.position, lexer.end),
                (success, SEMI as u16, position, Some(end)),
                "{input:?}"
            );
        }
        for token in [SEMI, CLASS_MEMBER_SEMI] {
            let (result, lexer) = scan("\nfoo", &[token]);
            assert!(result);
            assert_eq!(lexer.symbol, token as u16);
        }
        let (_, lexer) = scan("\nfoo", &[SEMI, CLASS_MEMBER_SEMI]);
        assert_eq!(lexer.symbol, SEMI as u16);
    }

    #[test]
    fn semicolon_words_modifiers_and_accessors() {
        // The prefix is exactly what is inspected; the token's mark remains 0.
        for (input, extra_valid, success, prefix) in [
            ("\nelse x", vec![], false, "\nelse "),
            ("\nelse -> x", vec![], true, "\nelse -"),
            ("\nelse_", vec![], false, "\nelse"),
            ("\nelsewhere", vec![], true, "\nelsewhere"),
            ("\ninstanceof X", vec![], false, "\ninstanceof"),
            ("\nget()", vec![], true, "\nget"),
            ("\nget()", vec![GET], false, "\nget"),
            ("\nget[0]", vec![GET], true, "\nget"),
            ("\nget.foo", vec![GET], false, "\nget"),
            ("\nset(x)", vec![], true, "\nset"),
            ("\nset(x) \r\n", vec![SET], true, "\nset(x) \r"),
            ("\nset(x) { }", vec![SET], false, "\nset(x) "),
            ("\nset.foo", vec![SET], true, "\nset"),
            ("\nset (x)", vec![SET], false, "\nset"),
            ("\nin range", vec![], false, "\nin"),
            ("\nin range", vec![IN], true, "\nin"),
            ("\nconstructor ()", vec![], false, "\nconstructor "),
            (
                "\nconstructor ()",
                vec![CLASS_MEMBER_SEMI],
                true,
                "\nconstructor ",
            ),
            ("\nconstructor .x", vec![], true, "\nconstructor "),
            ("\nconstructor =x", vec![], true, "\nconstructor "),
            (
                "\npublic inline get()",
                vec![GET],
                false,
                "\npublic inline get",
            ),
            ("\nprivate val x", vec![], true, "\nprivate val"),
            ("\npublic ;", vec![], true, "\npublic "),
        ] {
            let mut valid = extra_valid;
            valid.push(SEMI);
            let (result, lexer) = scan(input, &valid);
            assert_eq!(
                (result, lexer.symbol, lexer.position, lexer.end),
                (success, SEMI as u16, prefix.len(), Some(0)),
                "{input:?} with {valid:?}"
            );
        }

        let (result, lexer) = scan("\nset(", &[SEMI, SET]);
        assert!(!result);
        assert_eq!(lexer.events.last(), Some(&Event::Advance(5, true)));
    }

    #[test]
    fn annotations_resume_the_newline_switch() {
        for (input, extra_valid, success, prefix) in [
            ("\n@A constructor", CONSTRUCTOR, false, "\n@A constructor"),
            ("\n@A constructorX", CONSTRUCTOR, false, "\n@A constructor"),
            ("\n@A custom", CONSTRUCTOR, true, "\n@A c"),
            ("\n@A get()", GET, false, "\n@A get"),
            (
                "\n@A(value = 1)\nset(x)\n",
                SET,
                true,
                "\n@A(value = 1)\nset(x)",
            ),
            ("\n@A\n@B\nfoo", GET, true, "\n@A\n@B\nfoo"),
            ("\n@A\n/*x*/", GET, true, "\n@A\n"),
            ("\n@A\n+", GET, false, "\n@A\n+"),
        ] {
            let (result, lexer) = scan(input, &[SEMI, extra_valid]);
            assert_eq!(
                (result, lexer.symbol, lexer.position, lexer.end),
                (success, SEMI as u16, prefix.len(), Some(0)),
                "{input:?}"
            );
        }
    }

    #[test]
    fn safe_navigation_and_comment_fallthrough() {
        use Event::{Advance, Mark, Symbol};
        let (result, lexer) = scan("? \n.", &[Q_DOT]);
        assert!(result);
        assert_eq!(
            lexer.events,
            [
                Advance(0, false),
                Advance(1, true),
                Advance(2, true),
                Advance(3, false),
                Symbol(Q_DOT as u16),
                Mark(4),
            ]
        );
        for input in ["? .", "\n? ."] {
            let (result, lexer) = scan(input, &[SEMI, Q_DOT]);
            assert!(result);
            assert_eq!(lexer.symbol, Q_DOT as u16);
            assert_eq!(lexer.end, Some(input.len()));
        }
        let (result, lexer) = scan("? /*x*/", &[Q_DOT]);
        assert!(result);
        assert_eq!(lexer.symbol, BLOCK_COMMENT as u16);
        assert_eq!(lexer.end, Some(7));
    }

    #[test]
    fn block_comments_nest_and_dollar_suppresses_them() {
        // No validity check is made for BLOCK_COMMENT itself.
        for input in [
            "/**/",
            "/* a /* b */ c */",
            "/*/**/*/",
            "/* **/",
            "/* /x */",
        ] {
            let (result, lexer) = scan(input, &[]);
            assert!(result, "{input:?}");
            assert_eq!(lexer.symbol, BLOCK_COMMENT as u16);
            assert_eq!(lexer.end, Some(input.len()));
        }
        for input in ["//comment", "/*unterminated", "/*x\0*/"] {
            let (result, lexer) = scan(input, &[BLOCK_COMMENT]);
            assert!(!result, "{input:?}");
            assert_eq!(lexer.end, None);
        }
        let (result, lexer) = scan("/*x*/", &[DOLLAR, BLOCK_COMMENT]);
        assert!(!result);
        assert!(lexer.events.is_empty());

        let (result, lexer) = scan("\n/*x*/", &[SEMI]);
        assert!(result);
        assert_eq!((lexer.symbol, lexer.end), (BLOCK_COMMENT as u16, Some(6)));
        let (result, lexer) = scan("\n/*x*/", &[SEMI, DOLLAR]);
        assert!(!result);
        assert_eq!((lexer.position, lexer.end), (1, Some(0)));
    }

    #[test]
    fn comment_fallthrough_keeps_one_lookahead_read_per_position() {
        use Event::{Advance, Mark, Symbol};
        for prefix in ["/**/", "/* é /* λ */ 🦀 */"] {
            let input = format!("{prefix}tail");
            for tokens in [vec![], vec![BLOCK_COMMENT], vec![NOT_IS, IN, Q_DOT]] {
                let (success, lexer) = scan(&input, &tokens);
                assert!(success);
                assert_eq!(lexer.position, prefix.len());
                assert_eq!(lexer.lookahead_calls.get(), prefix.chars().count() + 1);
                assert_eq!(lexer.eof_calls.get(), 0);
                let mut expected: Vec<_> = prefix
                    .char_indices()
                    .map(|(position, _)| Advance(position, false))
                    .collect();
                expected.extend([Symbol(BLOCK_COMMENT as u16), Mark(prefix.len())]);
                assert_eq!(lexer.events, expected);
            }
        }
        // The outlined body is not entered if either rejection check fires.
        for (input, tokens) in [
            ("name", vec![NOT_IS, IN, Q_DOT]),
            ("/* comment */", vec![DOLLAR, BLOCK_COMMENT]),
        ] {
            let (success, lexer) = scan(input, &tokens);
            assert!(!success);
            assert_eq!(lexer.position, 0);
            assert!(lexer.events.is_empty());
            assert_eq!(lexer.lookahead_calls.get(), 1);
        }
    }

    #[test]
    fn semicolon_keyword_scan_is_limited_to_fifteen_ascii_letters() {
        for (input, position) in [
            ("abcdefghijklmnop", 15),
            ("éabc", 0),
            ("", 0),
            (" abc", 0),
            ("elsewhere", 9),
            ("getaway", 7),
            ("PUBLIC", 6),
        ] {
            let mut lexer = TestLexer::new(input);
            assert_eq!(
                scan_semi_keyword(&mut Cursor::new(&mut lexer)),
                SemiWord::Other,
                "{input:?}"
            );
            assert_eq!(lexer.position, position, "{input:?}");
        }
        let (success, lexer) = scan("\nabcdefghijklmnop", &[SEMI]);
        assert!(success);
        assert_eq!((lexer.position, lexer.end), (16, Some(0)));
    }

    #[test]
    fn modifiers_resume_word_scanning_even_after_non_switch_letters() {
        for modifier in [
            "public",
            "private",
            "protected",
            "internal",
            "abstract",
            "final",
            "open",
            "override",
            "lateinit",
            "vararg",
            "noinline",
            "crossinline",
            "external",
            "suspend",
            "inline",
        ] {
            for (tail, expected, inspected) in [
                ("get()", false, 3),
                ("where T", false, 5),
                ("unknown", true, 7),
                (";", true, 0),
                ("é", true, 0),
                ("", true, 0),
            ] {
                let input = format!("\n{modifier} {tail}");
                let (success, lexer) = scan(&input, &[SEMI, GET]);
                assert_eq!(success, expected, "{input:?}");
                assert_eq!(lexer.position, modifier.len() + 2 + inspected, "{input:?}");
                assert_eq!(lexer.end, Some(0), "{input:?}");
            }
        }
    }
}
