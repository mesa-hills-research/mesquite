//! The R external scanner, translated from `src/scanner.c`.

use std::ffi::c_char;
use tree_sitter_language::{ExternalScanner, Lexer, SERIALIZATION_BUFFER_SIZE};

const START: usize = 0;
const NEWLINE: usize = 1;
const SEMICOLON: usize = 2;
const RAW_STRING_OPEN: usize = 3;
const RAW_STRING_CONTENT: usize = 4;
const RAW_STRING_CLOSE: usize = 5;
const ELSE: usize = 6;
const OPEN_PAREN: usize = 7;
const CLOSE_PAREN: usize = 8;
const OPEN_BRACE: usize = 9;
const CLOSE_BRACE: usize = 10;
const OPEN_BRACKET: usize = 11;
const CLOSE_BRACKET: usize = 12;
const OPEN_BRACKET2: usize = 13;
const CLOSE_BRACKET2: usize = 14;
const ERROR_SENTINEL: usize = 15;

const SCOPE_TOP_LEVEL: u8 = 0;
const SCOPE_BRACE: u8 = 1;
const SCOPE_PAREN: u8 = 2;
const SCOPE_BRACKET: u8 = 3;
const SCOPE_BRACKET2: u8 = 4;

// Three raw-string bytes, a native-endian C unsigned count, then one byte/scope.
const HEADER_SIZE: usize = 3 + size_of::<u32>();
const MAX_SCOPES_COUNT: usize = SERIALIZATION_BUFFER_SIZE - HEADER_SIZE;

#[derive(Default)]
struct RawStringState {
    closing_bracket: c_char,
    hyphen_count: u8,
    closing_quote: c_char,
}

/// The scanner's state (C's `Payload`). The implicit top-level scope is not stored.
#[derive(Default)]
pub(crate) struct Scanner {
    raw_string: RawStringState,
    scopes: Vec<u8>,
}

impl Scanner {
    fn reset(&mut self) {
        self.raw_string = RawStringState::default();
        self.scopes.clear();
    }

    fn push_scope(&mut self, scope: u8) -> bool {
        if self.scopes.len() >= MAX_SCOPES_COUNT {
            return false;
        }
        self.scopes.push(scope);
        true
    }

    fn peek_scope(&self) -> u8 {
        self.scopes.last().copied().unwrap_or(SCOPE_TOP_LEVEL)
    }

    fn pop_scope(&mut self, expected: u8) -> bool {
        // C pops even when the scope does not match and scanning will fail.
        self.scopes.pop() == Some(expected)
    }

    fn consume_whitespace_and_ignored_newlines(&self, lexer: &mut dyn Lexer) {
        while is_space(lexer.lookahead()) {
            if lexer.lookahead() != i32::from(b'\n') {
                lexer.advance(true);
                continue;
            }
            if matches!(
                self.peek_scope(),
                SCOPE_PAREN | SCOPE_BRACKET | SCOPE_BRACKET2
            ) {
                lexer.advance(true);
                continue;
            }
            break;
        }
    }

    fn scan_raw_string_open(&mut self, lexer: &mut dyn Lexer) -> bool {
        // These three locals are C chars, not full lookahead code points.
        let prefix = lexer.lookahead() as c_char;
        if prefix != b'r' as c_char && prefix != b'R' as c_char {
            return false;
        }
        lexer.advance(false);

        let closing_quote = lexer.lookahead() as c_char;
        if closing_quote != b'"' as c_char && closing_quote != b'\'' as c_char {
            return false;
        }
        lexer.advance(false);

        let mut hyphen_count = 0u8;
        while lexer.lookahead() == i32::from(b'-') {
            if hyphen_count == u8::MAX {
                return false;
            }
            lexer.advance(false);
            hyphen_count += 1;
        }

        let opening_bracket = lexer.lookahead() as c_char;
        let closing_bracket = match opening_bracket as u8 {
            b'(' => b')',
            b'[' => b']',
            b'{' => b'}',
            _ => return false,
        } as c_char;
        lexer.advance(false);

        lexer.mark_end();
        lexer.set_result_symbol(RAW_STRING_OPEN as u16);
        self.raw_string = RawStringState {
            closing_bracket,
            hyphen_count,
            closing_quote,
        };
        true
    }

    fn scan_raw_string_content_or_close(&self, lexer: &mut dyn Lexer) -> bool {
        let RawStringState {
            closing_bracket,
            hyphen_count,
            closing_quote,
        } = self.raw_string;
        let mut any_content = false;

        while !lexer.eof() {
            if lexer.lookahead() != i32::from(closing_bracket) {
                lexer.advance(false);
                any_content = true;
                continue;
            }

            // Keep the content boundary before the tentative closing sequence.
            lexer.mark_end();
            lexer.advance(false);

            let mut matched_hyphens = true;
            for _ in 0..hyphen_count {
                if lexer.lookahead() != i32::from(b'-') {
                    matched_hyphens = false;
                    break;
                }
                lexer.advance(false);
            }
            if !matched_hyphens {
                any_content = true;
                continue;
            }
            if lexer.lookahead() != i32::from(closing_quote) {
                // Do not advance here: this character may begin another closer.
                any_content = true;
                continue;
            }
            lexer.advance(false);

            if any_content {
                lexer.set_result_symbol(RAW_STRING_CONTENT as u16);
            } else {
                lexer.mark_end();
                lexer.set_result_symbol(RAW_STRING_CLOSE as u16);
            }
            return true;
        }
        false
    }

    fn scan_raw_string_close(&self, lexer: &mut dyn Lexer) -> bool {
        // The content scanner already verified this sequence; do not recheck it.
        lexer.advance(false);
        for _ in 0..self.raw_string.hyphen_count {
            lexer.advance(false);
        }
        lexer.advance(false);
        lexer.mark_end();
        lexer.set_result_symbol(RAW_STRING_CLOSE as u16);
        true
    }

    fn scan_open_block(&mut self, lexer: &mut dyn Lexer, scope: u8, symbol: usize) -> bool {
        if !self.push_scope(scope) {
            return false;
        }
        scan_single_character(lexer, symbol)
    }

    fn scan_close_block(&mut self, lexer: &mut dyn Lexer, scope: u8, symbol: usize) -> bool {
        if !self.pop_scope(scope) {
            return false;
        }
        scan_single_character(lexer, symbol)
    }

    fn scan_open_bracket_or_bracket2(
        &mut self,
        lexer: &mut dyn Lexer,
        valid_symbols: &[bool],
    ) -> bool {
        lexer.advance(false);
        if valid_symbols[OPEN_BRACKET2] && lexer.lookahead() == i32::from(b'[') {
            if !self.push_scope(SCOPE_BRACKET2) {
                return false;
            }
            lexer.advance(false);
            lexer.mark_end();
            lexer.set_result_symbol(OPEN_BRACKET2 as u16);
            return true;
        }
        if valid_symbols[OPEN_BRACKET] {
            if !self.push_scope(SCOPE_BRACKET) {
                return false;
            }
            lexer.mark_end();
            lexer.set_result_symbol(OPEN_BRACKET as u16);
            return true;
        }
        false
    }

    fn scan_close_bracket2(&mut self, lexer: &mut dyn Lexer) -> bool {
        lexer.advance(false);
        if lexer.lookahead() != i32::from(b']') {
            return false;
        }
        self.scan_close_block(lexer, SCOPE_BRACKET2, CLOSE_BRACKET2)
    }
}

fn is_space(c: i32) -> bool {
    // `iswspace` in C's default locale includes vertical tab, not Unicode spaces.
    matches!(c, 0x09..=0x0d | 0x20)
}

fn is_identifier_continuation(c: i32) -> bool {
    matches!(c, 0x30..=0x39 | 0x41..=0x5a | 0x61..=0x7a | 0x5f | 0x2e) || c >= 128
}

fn scan_else(lexer: &mut dyn Lexer) -> bool {
    for c in b"else" {
        if lexer.lookahead() != i32::from(*c) {
            return false;
        }
        lexer.advance(false);
    }
    if is_identifier_continuation(lexer.lookahead()) {
        return false;
    }
    lexer.mark_end();
    lexer.set_result_symbol(ELSE as u16);
    true
}

fn scan_else_with_leading_newlines(lexer: &mut dyn Lexer) -> bool {
    while is_space(lexer.lookahead()) {
        if lexer.lookahead() != i32::from(b'\n') {
            lexer.advance(true);
            continue;
        }
        lexer.advance(true);
        lexer.mark_end();
        lexer.set_result_symbol(NEWLINE as u16);
    }
    if lexer.lookahead() == i32::from(b'#') {
        return false;
    }
    // A failed `else` attempt still leaves a NEWLINE token, with its earlier end.
    scan_else(lexer);
    true
}

fn scan_single_character(lexer: &mut dyn Lexer, symbol: usize) -> bool {
    lexer.advance(false);
    lexer.mark_end();
    lexer.set_result_symbol(symbol as u16);
    true
}

impl ExternalScanner for Scanner {
    fn scan(&mut self, lexer: &mut dyn Lexer, valid_symbols: &[bool]) -> bool {
        if valid_symbols[ERROR_SENTINEL] {
            return false;
        }
        if valid_symbols[START] {
            lexer.set_result_symbol(START as u16);
            return true;
        }
        // Raw-string whitespace belongs to its content, so these run before skipping.
        if valid_symbols[RAW_STRING_CONTENT] {
            return self.scan_raw_string_content_or_close(lexer);
        } else if valid_symbols[RAW_STRING_CLOSE] {
            return self.scan_raw_string_close(lexer);
        }

        self.consume_whitespace_and_ignored_newlines(lexer);

        // Each branch returns immediately, including when its scanner fails after advancing.
        if valid_symbols[SEMICOLON] && lexer.lookahead() == i32::from(b';') {
            scan_single_character(lexer, SEMICOLON)
        } else if valid_symbols[OPEN_PAREN] && lexer.lookahead() == i32::from(b'(') {
            self.scan_open_block(lexer, SCOPE_PAREN, OPEN_PAREN)
        } else if valid_symbols[CLOSE_PAREN] && lexer.lookahead() == i32::from(b')') {
            self.scan_close_block(lexer, SCOPE_PAREN, CLOSE_PAREN)
        } else if valid_symbols[OPEN_BRACE] && lexer.lookahead() == i32::from(b'{') {
            self.scan_open_block(lexer, SCOPE_BRACE, OPEN_BRACE)
        } else if valid_symbols[CLOSE_BRACE] && lexer.lookahead() == i32::from(b'}') {
            self.scan_close_block(lexer, SCOPE_BRACE, CLOSE_BRACE)
        } else if (valid_symbols[OPEN_BRACKET] || valid_symbols[OPEN_BRACKET2])
            && lexer.lookahead() == i32::from(b'[')
        {
            self.scan_open_bracket_or_bracket2(lexer, valid_symbols)
        } else if valid_symbols[CLOSE_BRACKET]
            && lexer.lookahead() == i32::from(b']')
            && self.peek_scope() == SCOPE_BRACKET
        {
            self.scan_close_block(lexer, SCOPE_BRACKET, CLOSE_BRACKET)
        } else if valid_symbols[CLOSE_BRACKET2]
            && lexer.lookahead() == i32::from(b']')
            && self.peek_scope() == SCOPE_BRACKET2
        {
            self.scan_close_bracket2(lexer)
        } else if valid_symbols[RAW_STRING_OPEN] && matches!(lexer.lookahead(), 0x72 | 0x52) {
            self.scan_raw_string_open(lexer)
        } else if valid_symbols[ELSE] && lexer.lookahead() == i32::from(b'e') {
            scan_else(lexer)
        } else if valid_symbols[ELSE]
            && self.peek_scope() == SCOPE_BRACE
            && lexer.lookahead() == i32::from(b'\n')
        {
            scan_else_with_leading_newlines(lexer)
        } else if valid_symbols[NEWLINE] && lexer.lookahead() == i32::from(b'\n') {
            scan_single_character(lexer, NEWLINE)
        } else {
            false
        }
    }

    fn serialize(&mut self, buffer: &mut [u8]) -> usize {
        let length = HEADER_SIZE + self.scopes.len();
        if buffer.len() < length {
            return 0;
        }
        buffer[0] = self.raw_string.closing_bracket as u8;
        buffer[1] = self.raw_string.hyphen_count;
        buffer[2] = self.raw_string.closing_quote as u8;
        buffer[3..HEADER_SIZE].copy_from_slice(&(self.scopes.len() as u32).to_ne_bytes());
        buffer[HEADER_SIZE..length].copy_from_slice(&self.scopes);
        length
    }

    fn deserialize(&mut self, buffer: &[u8]) {
        // C resets the entire payload on empty input or any incomplete field.
        if buffer.len() < HEADER_SIZE {
            self.reset();
            return;
        }
        let count = u32::from_ne_bytes(buffer[3..HEADER_SIZE].try_into().unwrap()) as usize;
        // Snapshots emitted by C always fit the scope array. Reject oversized corrupt
        // snapshots as well, rather than reproducing C's out-of-bounds memcpy.
        if count > MAX_SCOPES_COUNT || buffer.len() - HEADER_SIZE < count {
            self.reset();
            return;
        }
        self.raw_string = RawStringState {
            closing_bracket: buffer[0] as c_char,
            hyphen_count: buffer[1],
            closing_quote: buffer[2] as c_char,
        };
        self.scopes.clear();
        self.scopes
            .extend_from_slice(&buffer[HEADER_SIZE..HEADER_SIZE + count]);
    }
}

/// Creates a scanner (C's `tree_sitter_r_external_scanner_create`).
pub(crate) fn create() -> Box<dyn ExternalScanner> {
    Box::new(Scanner {
        raw_string: RawStringState::default(),
        scopes: Vec::with_capacity(MAX_SCOPES_COUNT),
    })
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
            self.events.push(Event::Symbol(symbol));
            self.symbol = symbol;
        }
        fn advance(&mut self, skip: bool) {
            self.events.push(Event::Advance(self.position, skip));
            if !self.eof() {
                self.position += 1;
            }
        }
        fn mark_end(&mut self) {
            self.events.push(Event::MarkEnd(self.position));
            self.end = Some(self.position);
        }
        fn get_column(&mut self) -> u32 {
            panic!("R's scanner does not call get_column")
        }
        fn is_at_included_range_start(&self) -> bool {
            panic!("R's scanner does not inspect included ranges")
        }
        fn eof(&self) -> bool {
            self.position == self.input.len()
        }
    }

    fn symbols(enabled: &[usize]) -> [bool; 16] {
        let mut result = [false; 16];
        for &symbol in enabled {
            result[symbol] = true;
        }
        result
    }

    fn scan(scanner: &mut Scanner, input: &str, enabled: &[usize]) -> (bool, TestLexer) {
        let mut lexer = TestLexer::new(input);
        let accepted = scanner.scan(&mut lexer, &symbols(enabled));
        (accepted, lexer)
    }

    fn snapshot(scanner: &mut Scanner) -> Vec<u8> {
        let mut buffer = [0; SERIALIZATION_BUFFER_SIZE];
        let length = scanner.serialize(&mut buffer);
        buffer[..length].to_vec()
    }

    #[test]
    fn start_is_zero_width_and_error_sentinel_takes_precedence() {
        let mut scanner = Scanner::default();
        let (accepted, lexer) = scan(&mut scanner, " \r\n ", &[START, NEWLINE]);
        assert!(accepted);
        assert_eq!(lexer.events, [Event::Symbol(START as u16)]);
        let (accepted, lexer) = scan(&mut scanner, " \r\n ", &[START, ERROR_SENTINEL]);
        assert!(!accepted);
        assert!(lexer.events.is_empty());
    }

    #[test]
    fn snapshots_preserve_native_count_and_stale_raw_string_state() {
        let mut scanner = Scanner::default();
        assert!(scan(&mut scanner, "{", &[OPEN_BRACE]).0);
        assert!(scan(&mut scanner, "[[", &[OPEN_BRACKET2]).0);
        assert!(scan(&mut scanner, "r'---{", &[RAW_STRING_OPEN]).0);
        let mut expected = vec![b'}', 3, b'\''];
        expected.extend_from_slice(&2u32.to_ne_bytes());
        expected.extend_from_slice(&[SCOPE_BRACE, SCOPE_BRACKET2]);
        assert_eq!(snapshot(&mut scanner), expected);

        let mut restored = Scanner::default();
        restored.deserialize(&expected);
        assert_eq!(snapshot(&mut restored), expected);
        assert!(
            scan(
                &mut restored,
                "}---'",
                &[RAW_STRING_CONTENT, RAW_STRING_CLOSE]
            )
            .0
        );
        // Closing a raw string does not reset its serialized delimiter fields.
        assert_eq!(snapshot(&mut restored), expected);

        restored.deserialize(&[]);
        assert_eq!(snapshot(&mut restored), [0; HEADER_SIZE]);
        for incomplete in 1..expected.len() {
            restored.deserialize(&expected);
            restored.deserialize(&expected[..incomplete]);
            assert_eq!(snapshot(&mut restored), [0; HEADER_SIZE]);
        }
        // Trailing bytes are ignored, just as in C's deserialize helper.
        let mut extra = expected.clone();
        extra.extend_from_slice(&[99, 98]);
        restored.deserialize(&extra);
        assert_eq!(snapshot(&mut restored), expected);
    }

    #[test]
    fn scope_limit_and_failed_pops_preserve_c_mutation_order() {
        let mut scanner = Scanner::default();
        for _ in 0..MAX_SCOPES_COUNT {
            assert!(scan(&mut scanner, "(", &[OPEN_PAREN]).0);
        }
        assert_eq!(snapshot(&mut scanner).len(), SERIALIZATION_BUFFER_SIZE);
        let (accepted, lexer) = scan(&mut scanner, "(", &[OPEN_PAREN]);
        assert!(!accepted);
        assert!(lexer.events.is_empty());
        let (accepted, lexer) = scan(&mut scanner, "[[", &[OPEN_BRACKET2]);
        assert!(!accepted);
        // Unlike (, the first [ is consumed before checking the stack capacity.
        assert_eq!(lexer.events, [Event::Advance(0, false)]);
        let (accepted, lexer) = scan(&mut scanner, "}", &[CLOSE_BRACE]);
        assert!(!accepted);
        assert!(lexer.events.is_empty());
        assert_eq!(scanner.scopes.len(), MAX_SCOPES_COUNT - 1);
        scanner.reset();
        assert!(!scan(&mut scanner, ")", &[CLOSE_PAREN]).0);
    }

    #[test]
    fn bracket_width_is_greedy_on_open_and_scope_dependent_on_close() {
        let mut scanner = Scanner::default();
        let both_open = [OPEN_BRACKET, OPEN_BRACKET2];
        let both_close = [CLOSE_BRACKET, CLOSE_BRACKET2];
        let (accepted, lexer) = scan(&mut scanner, "[[", &both_open);
        assert!(accepted);
        assert_eq!(lexer.symbol, OPEN_BRACKET2 as u16);
        assert_eq!(lexer.end, Some(2));
        assert!(scan(&mut scanner, "[a", &both_open).0);
        let (accepted, lexer) = scan(&mut scanner, "]]", &both_close);
        assert!(accepted);
        assert_eq!(lexer.symbol, CLOSE_BRACKET as u16);
        assert_eq!(lexer.end, Some(1));
        let (accepted, lexer) = scan(&mut scanner, "]x", &both_close);
        assert!(!accepted);
        assert_eq!(lexer.events, [Event::Advance(0, false)]);
        assert_eq!(scanner.peek_scope(), SCOPE_BRACKET2);
        let (accepted, lexer) = scan(&mut scanner, "]]", &both_close);
        assert!(accepted);
        assert_eq!(lexer.symbol, CLOSE_BRACKET2 as u16);
        assert_eq!(lexer.end, Some(2));
        assert_eq!(scanner.peek_scope(), SCOPE_TOP_LEVEL);
        let (accepted, lexer) = scan(&mut scanner, "[[", &[OPEN_BRACKET]);
        assert!(accepted);
        assert_eq!(lexer.symbol, OPEN_BRACKET as u16);
        assert_eq!(lexer.end, Some(1));
    }

    #[test]
    fn whitespace_skips_newlines_only_in_parens_and_brackets() {
        for scope in [
            SCOPE_TOP_LEVEL,
            SCOPE_BRACE,
            SCOPE_PAREN,
            SCOPE_BRACKET,
            SCOPE_BRACKET2,
        ] {
            let mut scanner = Scanner::default();
            if scope != SCOPE_TOP_LEVEL {
                scanner.push_scope(scope);
            }
            let (accepted, lexer) = scan(&mut scanner, " \t\x0b\x0c\r\n;", &[NEWLINE, SEMICOLON]);
            assert!(accepted);
            for i in 0..5 {
                assert_eq!(lexer.events[i], Event::Advance(i, true));
            }
            if matches!(scope, SCOPE_TOP_LEVEL | SCOPE_BRACE) {
                assert_eq!(lexer.symbol, NEWLINE as u16);
                assert_eq!(lexer.events[5], Event::Advance(5, false));
                assert_eq!(lexer.end, Some(6));
            } else {
                assert_eq!(lexer.symbol, SEMICOLON as u16);
                assert_eq!(lexer.events[5], Event::Advance(5, true));
                assert_eq!(lexer.end, Some(7));
            }
        }
        let mut scanner = Scanner::default();
        let (accepted, lexer) = scan(&mut scanner, "\u{a0};", &[SEMICOLON]);
        assert!(!accepted);
        assert!(lexer.events.is_empty());
    }

    #[test]
    fn else_after_newlines_marks_each_newline_and_can_fail_after_marking() {
        let mut scanner = Scanner::default();
        scanner.push_scope(SCOPE_BRACE);
        let (accepted, lexer) = scan(&mut scanner, "\n \n else!", &[ELSE, NEWLINE]);
        assert!(accepted);
        assert_eq!(
            lexer.events,
            [
                Event::Advance(0, true),
                Event::MarkEnd(1),
                Event::Symbol(NEWLINE as u16),
                Event::Advance(1, true),
                Event::Advance(2, true),
                Event::MarkEnd(3),
                Event::Symbol(NEWLINE as u16),
                Event::Advance(3, true),
                Event::Advance(4, false),
                Event::Advance(5, false),
                Event::Advance(6, false),
                Event::Advance(7, false),
                Event::MarkEnd(8),
                Event::Symbol(ELSE as u16),
            ]
        );
        let (accepted, lexer) = scan(&mut scanner, "\n # comment", &[ELSE, NEWLINE]);
        assert!(!accepted);
        assert_eq!(lexer.end, Some(1));
        assert_eq!(lexer.symbol, NEWLINE as u16);
        assert_eq!(lexer.position, 2);
        // Even when NEWLINE is not enabled, the special ELSE branch can emit it.
        let (accepted, lexer) = scan(&mut scanner, "\n elsewhere", &[ELSE]);
        assert!(accepted);
        assert_eq!(lexer.end, Some(1));
        assert_eq!(lexer.symbol, NEWLINE as u16);
        assert_eq!(lexer.position, 6);
        scanner.reset();
        let (accepted, lexer) = scan(&mut scanner, "\n else!", &[ELSE, NEWLINE]);
        assert!(accepted);
        assert_eq!(lexer.symbol, NEWLINE as u16);
        assert_eq!(lexer.position, 1);
    }

    #[test]
    fn else_continuations_include_all_non_ascii_codepoints() {
        let mut scanner = Scanner::default();
        for input in [
            "else0", "elseA", "elsez", "else_", "else.", "elseμ", "else·",
        ] {
            let (accepted, lexer) = scan(&mut scanner, input, &[ELSE]);
            assert!(!accepted, "{input}");
            assert_eq!(lexer.position, 4);
            assert_eq!(lexer.end, None);
        }
        for input in ["else", "else!", "else#", "else\n"] {
            let (accepted, lexer) = scan(&mut scanner, input, &[ELSE]);
            assert!(accepted, "{input}");
            assert_eq!(lexer.end, Some(4));
        }
    }

    #[test]
    fn raw_open_supports_all_delimiters_and_rejects_256_hyphens() {
        let mut scanner = Scanner::default();
        for prefix in ['r', 'R'] {
            for quote in ['\'', '"'] {
                for (open, close) in [('(', ')'), ('[', ']'), ('{', '}')] {
                    let input = format!("{prefix}{quote}--{open}");
                    let (accepted, lexer) = scan(&mut scanner, &input, &[RAW_STRING_OPEN]);
                    assert!(accepted);
                    assert_eq!(lexer.end, Some(5));
                    assert_eq!(scanner.raw_string.closing_bracket as u8, close as u8);
                    assert_eq!(scanner.raw_string.closing_quote as u8, quote as u8);
                    assert_eq!(scanner.raw_string.hyphen_count, 2);
                }
            }
        }
        let input = format!("r\"{}(", "-".repeat(255));
        assert!(scan(&mut scanner, &input, &[RAW_STRING_OPEN]).0);
        let before = snapshot(&mut scanner);
        let input = format!("r\"{}(", "-".repeat(256));
        let (accepted, lexer) = scan(&mut scanner, &input, &[RAW_STRING_OPEN]);
        assert!(!accepted);
        assert_eq!(lexer.position, 257);
        assert_eq!(lexer.end, None);
        assert_eq!(snapshot(&mut scanner), before);
    }

    #[test]
    fn raw_open_narrows_quote_and_bracket_but_not_dispatch_prefix() {
        let mut scanner = Scanner::default();
        // Low bytes of U+0122 and U+0128 are the ASCII double quote and (.
        assert!(scan(&mut scanner, "r\u{122}\u{128}", &[RAW_STRING_OPEN]).0);
        assert_eq!(&snapshot(&mut scanner)[..3], &[b')', 0, b'"']);
        let (accepted, lexer) = scan(&mut scanner, "\u{172}\"(", &[RAW_STRING_OPEN]);
        assert!(!accepted);
        assert!(lexer.events.is_empty());
    }

    #[test]
    fn raw_content_retains_whitespace_and_retries_overlapping_closers() {
        let mut scanner = Scanner::default();
        assert!(scan(&mut scanner, "r\"(", &[RAW_STRING_OPEN]).0);
        let (accepted, lexer) = scan(
            &mut scanner,
            " \n())\"tail",
            &[RAW_STRING_CONTENT, RAW_STRING_CLOSE],
        );
        assert!(accepted);
        assert_eq!(lexer.symbol, RAW_STRING_CONTENT as u16);
        assert_eq!(lexer.end, Some(4));
        assert_eq!(lexer.position, 6);
        assert!(lexer.events.contains(&Event::MarkEnd(3)));
        assert!(lexer.events.contains(&Event::MarkEnd(4)));
        assert!(
            lexer
                .events
                .iter()
                .all(|event| !matches!(event, Event::Advance(_, true)))
        );
        assert!(scan(&mut scanner, "r\"-(", &[RAW_STRING_OPEN]).0);
        let (accepted, lexer) = scan(
            &mut scanner,
            "))-\"",
            &[RAW_STRING_CONTENT, RAW_STRING_CLOSE],
        );
        assert!(accepted);
        assert_eq!(lexer.symbol, RAW_STRING_CONTENT as u16);
        assert_eq!(lexer.end, Some(1));
        let (accepted, lexer) = scan(
            &mut scanner,
            "unterminated)",
            &[RAW_STRING_CONTENT, RAW_STRING_CLOSE],
        );
        assert!(!accepted);
        assert!(lexer.eof());
    }

    #[test]
    fn raw_empty_content_emits_close_and_close_only_trusts_prior_validation() {
        let mut scanner = Scanner::default();
        assert!(scan(&mut scanner, "r\"--(", &[RAW_STRING_OPEN]).0);
        let before = snapshot(&mut scanner);
        let (accepted, lexer) = scan(
            &mut scanner,
            ")--\"",
            &[RAW_STRING_CONTENT, RAW_STRING_CLOSE],
        );
        assert!(accepted);
        assert_eq!(
            lexer.events,
            [
                Event::MarkEnd(0),
                Event::Advance(0, false),
                Event::Advance(1, false),
                Event::Advance(2, false),
                Event::Advance(3, false),
                Event::MarkEnd(4),
                Event::Symbol(RAW_STRING_CLOSE as u16),
            ]
        );
        assert_eq!(snapshot(&mut scanner), before);
        let (accepted, lexer) = scan(&mut scanner, "abcd", &[RAW_STRING_CLOSE]);
        assert!(accepted);
        assert_eq!(lexer.symbol, RAW_STRING_CLOSE as u16);
        assert_eq!(lexer.end, Some(4));
        assert_eq!(snapshot(&mut scanner), before);
    }
}
