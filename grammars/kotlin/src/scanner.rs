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
const MAX_WORDS: usize = 16;

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

fn scan_word(lexer: &mut dyn Lexer, word: &[u8]) -> bool {
    for &c in word {
        if lexer.lookahead() != i32::from(c) {
            return false;
        }
        lexer.advance(true);
    }
    true
}

fn scan_words(
    lexer: &mut dyn Lexer,
    words: &[&str; MAX_WORDS],
    scanned_word: &mut [u8; MAX_WORD_SIZE],
) -> Option<usize> {
    if scanned_word[0] == 0 {
        for (i, c) in scanned_word[..MAX_WORD_SIZE - 1].iter_mut().enumerate() {
            if !is_alpha(lexer.lookahead()) {
                if i == 0 {
                    return None;
                }
                break;
            }
            *c = lexer.lookahead() as u8;
            lexer.advance(true);
        }
    }

    // The buffer is NUL-padded, just like the C strncmp operands. In particular,
    // identifiers longer than fifteen letters are only scanned up to that limit.
    let length = scanned_word
        .iter()
        .position(|&c| c == 0)
        .unwrap_or(MAX_WORD_SIZE);
    words
        .iter()
        .position(|word| word.as_bytes() == &scanned_word[..length])
}

/// `None` means that the C multiline-string loop reached EOF and fell through
/// to the other token scanners, rather than returning a string-content token.
fn scan_multiline_string(lexer: &mut dyn Lexer) -> Option<bool> {
    let mut did_advance = false;
    lexer.set_result_symbol(MULTILINE_STRING_CONTENT as u16);
    while !lexer.eof() {
        match char::from_u32(lexer.lookahead() as u32) {
            Some('$') => {
                lexer.mark_end();
                lexer.advance(false);
                if is_alpha(lexer.lookahead()) || lexer.lookahead() == i32::from(b'{') {
                    return Some(did_advance);
                }
                did_advance = true;
            }
            Some('"') => {
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
fn scan_not_is_tail(lexer: &mut dyn Lexer) -> Option<bool> {
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

fn scan_in_and_rest(lexer: &mut dyn Lexer, valid_symbols: &[bool]) -> bool {
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
fn scan_q_dot_and_comment(lexer: &mut dyn Lexer, valid_symbols: &[bool]) -> bool {
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
fn scan_comment(lexer: &mut dyn Lexer, valid_symbols: &[bool]) -> bool {
    if valid_symbols[DOLLAR] {
        return false;
    }

    if lexer.lookahead() != i32::from(b'/') {
        return false;
    }
    lexer.advance(false);
    if lexer.lookahead() != i32::from(b'*') {
        return false;
    }
    lexer.advance(false);

    let mut after_star = false;
    let mut nesting_depth = 1u32;
    loop {
        match char::from_u32(lexer.lookahead() as u32) {
            Some('\0') => return false,
            Some('*') => {
                lexer.advance(false);
                after_star = true;
            }
            Some('/') => {
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

fn scan_semi_word(lexer: &mut dyn Lexer, valid_symbols: &[bool]) -> bool {
    let mut scanned_word = [0; MAX_WORD_SIZE];
    while scan_words(
        lexer,
        &[
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
            "",
        ],
        &mut scanned_word,
    )
    .is_some()
    {
        scanned_word.fill(0);
        while is_space(lexer.lookahead()) {
            lexer.advance(true);
        }
    }

    let index = scan_words(
        lexer,
        &[
            "else",
            "in",
            "instanceof",
            "get",
            "set",
            "constructor",
            "by",
            "as",
            "where",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
        ],
        &mut scanned_word,
    );

    match index {
        Some(5) => {
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
        Some(0) => {
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
        Some(3) if !valid_symbols[GET] || lexer.lookahead() == i32::from(b'[') => {
            return true;
        }
        Some(4)
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
        Some(1) if valid_symbols[IN] => return true,
        _ => {}
    }
    index.is_none()
}

/// The switch after a newline. A loop replaces the annotation branch's goto
/// back to `_switch`, without repeating the semicolon's initial mark_end.
fn scan_after_newline(lexer: &mut dyn Lexer, valid_symbols: &[bool]) -> bool {
    loop {
        match char::from_u32(lexer.lookahead() as u32) {
            Some(',' | '.' | ':' | '*' | '%' | '>' | '<' | '=' | '{' | '[' | '|' | '&' | '/') => {
                return false;
            }
            Some('+') => {
                lexer.advance(true);
                if lexer.lookahead() == i32::from(b'+') {
                    return true;
                }
                return is_digit(lexer.lookahead());
            }
            Some('-') => {
                lexer.advance(true);
                if lexer.lookahead() == i32::from(b'-') {
                    return true;
                }
                return is_digit(lexer.lookahead());
            }
            Some('!') => {
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
            Some('?') => {
                if valid_symbols[Q_DOT] {
                    return scan_q_dot_and_comment(lexer, valid_symbols);
                }
                return true;
            }
            Some(
                'e' | 'i' | 'g' | 's' | 'p' | 'a' | 'f' | 'o' | 'l' | 'v' | 'n' | 'c' | 'b' | 'w',
            ) => {
                return scan_semi_word(lexer, valid_symbols);
            }
            Some(';') => {
                lexer.advance(false);
                lexer.mark_end();
                return true;
            }
            Some('@') => {
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

fn scan_semi(lexer: &mut dyn Lexer, valid_symbols: &[bool]) -> bool {
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
        return match char::from_u32(lexer.lookahead() as u32) {
            Some('!') => {
                lexer.advance(true);
                if let Some(result) = scan_not_is_tail(lexer) {
                    return result;
                }
                scan_in_and_rest(lexer, valid_symbols)
            }
            Some('?') if valid_symbols[Q_DOT] => scan_q_dot_and_comment(lexer, valid_symbols),
            Some('i') => scan_word(lexer, b"import"),
            Some(';') => {
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
    }

    impl<'a> TestLexer<'a> {
        fn new(input: &'a str) -> Self {
            Self {
                input,
                position: 0,
                end: None,
                symbol: u16::MAX,
                events: Vec::new(),
            }
        }
    }

    impl Lexer for TestLexer<'_> {
        fn lookahead(&self) -> i32 {
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
    fn scan_words_caches_fifteen_ascii_letters() {
        let words = [
            "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
        ];
        let mut lexer = TestLexer::new("abcdefghijklmnop");
        let mut word = [0; MAX_WORD_SIZE];
        assert_eq!(scan_words(&mut lexer, &words, &mut word), None);
        assert_eq!(&word, b"abcdefghijklmno\0");
        assert_eq!(lexer.position, 15);
        let calls = lexer.events.len();
        assert_eq!(scan_words(&mut lexer, &words, &mut word), None);
        assert_eq!(lexer.events.len(), calls);

        let mut lexer = TestLexer::new("éabc");
        assert_eq!(scan_words(&mut lexer, &words, &mut [0; 16]), None);
        assert_eq!(lexer.position, 0);
    }
}
