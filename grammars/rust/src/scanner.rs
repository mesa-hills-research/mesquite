//! The Rust grammar's external scanner, translated from `scanner.c`.

use tree_sitter_language::{ExternalScanner, Lexer};

// These indices are the order of TokenType in the C scanner.
const STRING_CONTENT: usize = 0;
const STRING_CLOSE: usize = 1;
const RAW_STRING_LITERAL_START: usize = 2;
const RAW_STRING_LITERAL_CONTENT: usize = 3;
const RAW_STRING_LITERAL_END: usize = 4;
const FLOAT_LITERAL: usize = 5;
const BLOCK_OUTER_DOC_MARKER: usize = 6;
const BLOCK_INNER_DOC_MARKER: usize = 7;
const BLOCK_COMMENT_CONTENT: usize = 8;
const LINE_DOC_CONTENT: usize = 9;
const ERROR_SENTINEL: usize = 10;

/// The scanner's state (C's `payload`).
#[derive(Default)]
pub(crate) struct Scanner {
    opening_hash_count: u8,
}

// The C runtime does not call setlocale, so these wctype predicates use the
// default C locale, not Rust's Unicode alphabetic/whitespace classifications.
fn is_digit(c: i32) -> bool {
    matches!(c, 0x30..=0x39)
}

fn is_alpha(c: i32) -> bool {
    matches!(c, 0x41..=0x5a | 0x61..=0x7a)
}

fn is_space(c: i32) -> bool {
    matches!(c, 0x09..=0x0d | 0x20)
}

fn is_num_char(c: i32) -> bool {
    c == i32::from(b'_') || is_digit(c)
}

fn process_string(lexer: &mut dyn Lexer) -> bool {
    let mut has_content = false;
    loop {
        if lexer.lookahead() == i32::from(b'"') || lexer.lookahead() == i32::from(b'\\') {
            break;
        }
        if lexer.eof() {
            return false;
        }
        has_content = true;
        lexer.advance(false);
    }
    lexer.set_result_symbol(STRING_CONTENT as u16);
    lexer.mark_end();
    has_content
}

impl Scanner {
    fn scan_raw_string_start(&mut self, lexer: &mut dyn Lexer) -> bool {
        if lexer.lookahead() == i32::from(b'b') || lexer.lookahead() == i32::from(b'c') {
            lexer.advance(false);
        }
        if lexer.lookahead() != i32::from(b'r') {
            return false;
        }
        lexer.advance(false);

        let mut opening_hash_count = 0u8;
        while lexer.lookahead() == i32::from(b'#') {
            lexer.advance(false);
            opening_hash_count = opening_hash_count.wrapping_add(1);
        }

        if lexer.lookahead() != i32::from(b'"') {
            return false;
        }
        lexer.advance(false);
        self.opening_hash_count = opening_hash_count;
        lexer.set_result_symbol(RAW_STRING_LITERAL_START as u16);
        true
    }

    fn scan_raw_string_content(&self, lexer: &mut dyn Lexer) -> bool {
        loop {
            if lexer.eof() {
                return false;
            }
            if lexer.lookahead() == i32::from(b'"') {
                lexer.mark_end();
                lexer.advance(false);
                let mut hash_count = 0u32;
                while lexer.lookahead() == i32::from(b'#')
                    && hash_count < u32::from(self.opening_hash_count)
                {
                    lexer.advance(false);
                    hash_count += 1;
                }
                if hash_count == u32::from(self.opening_hash_count) {
                    lexer.set_result_symbol(RAW_STRING_LITERAL_CONTENT as u16);
                    return true;
                }
            } else {
                lexer.advance(false);
            }
        }
    }

    fn scan_raw_string_end(&self, lexer: &mut dyn Lexer) -> bool {
        lexer.advance(false);
        for _ in 0..self.opening_hash_count {
            lexer.advance(false);
        }
        lexer.set_result_symbol(RAW_STRING_LITERAL_END as u16);
        true
    }
}

fn process_float_literal(lexer: &mut dyn Lexer) -> bool {
    lexer.set_result_symbol(FLOAT_LITERAL as u16);
    lexer.advance(false);
    while is_num_char(lexer.lookahead()) {
        lexer.advance(false);
    }

    let mut has_fraction = false;
    let mut has_exponent = false;
    if lexer.lookahead() == i32::from(b'.') {
        has_fraction = true;
        lexer.advance(false);
        if is_alpha(lexer.lookahead()) {
            // `1.max(2)` is a method call, not a floating-point literal.
            return false;
        }
        if lexer.lookahead() == i32::from(b'.') {
            return false;
        }
        while is_num_char(lexer.lookahead()) {
            lexer.advance(false);
        }
    }

    lexer.mark_end();
    if lexer.lookahead() == i32::from(b'e') || lexer.lookahead() == i32::from(b'E') {
        has_exponent = true;
        lexer.advance(false);
        if lexer.lookahead() == i32::from(b'+') || lexer.lookahead() == i32::from(b'-') {
            lexer.advance(false);
        }
        if !is_num_char(lexer.lookahead()) {
            return true;
        }
        lexer.advance(false);
        while is_num_char(lexer.lookahead()) {
            lexer.advance(false);
        }
        lexer.mark_end();
    }

    if !has_exponent && !has_fraction {
        return false;
    }
    if lexer.lookahead() != i32::from(b'u')
        && lexer.lookahead() != i32::from(b'i')
        && lexer.lookahead() != i32::from(b'f')
    {
        return true;
    }
    lexer.advance(false);
    if !is_digit(lexer.lookahead()) {
        return true;
    }
    while is_digit(lexer.lookahead()) {
        lexer.advance(false);
    }
    lexer.mark_end();
    true
}

fn process_line_doc_content(lexer: &mut dyn Lexer) -> bool {
    lexer.set_result_symbol(LINE_DOC_CONTENT as u16);
    loop {
        if lexer.eof() {
            return true;
        }
        if lexer.lookahead() == i32::from(b'\n') {
            // Keep the newline for markdown injections.
            lexer.advance(false);
            return true;
        }
        lexer.advance(false);
    }
}

#[derive(Clone, Copy)]
enum BlockCommentState {
    LeftForwardSlash,
    LeftAsterisk,
    Continuing,
}

fn process_block_comment(lexer: &mut dyn Lexer, valid_symbols: &[bool]) -> bool {
    // C casts lookahead to char here and throughout comment processing. Keep
    // only the low byte, even for non-ASCII code points.
    let mut first = lexer.lookahead() as u8;
    if valid_symbols[BLOCK_INNER_DOC_MARKER] && first == b'!' {
        lexer.set_result_symbol(BLOCK_INNER_DOC_MARKER as u16);
        lexer.advance(false);
        return true;
    }
    if valid_symbols[BLOCK_OUTER_DOC_MARKER] && first == b'*' {
        lexer.advance(false);
        lexer.mark_end();
        if lexer.lookahead() == i32::from(b'/') {
            // An empty block comment, not a doc marker.
            return false;
        }
        if lexer.lookahead() != i32::from(b'*') {
            lexer.set_result_symbol(BLOCK_OUTER_DOC_MARKER as u16);
            return true;
        }
    } else {
        // All branches must consume exactly the saved first character.
        lexer.advance(false);
    }

    if valid_symbols[BLOCK_COMMENT_CONTENT] {
        let mut nesting_depth = 1u32;
        let mut state = match first {
            b'*' => {
                if lexer.lookahead() == i32::from(b'/') {
                    // Empty content, e.g. the end of `/*!*/`.
                    return false;
                }
                BlockCommentState::LeftAsterisk
            }
            b'/' => BlockCommentState::LeftForwardSlash,
            _ => BlockCommentState::Continuing,
        };

        // Like C, accept an unterminated comment for syntax highlighting.
        while !lexer.eof() && nesting_depth != 0 {
            first = lexer.lookahead() as u8;
            match state {
                BlockCommentState::LeftForwardSlash => {
                    if first == b'*' {
                        nesting_depth = nesting_depth.wrapping_add(1);
                    }
                    state = BlockCommentState::Continuing;
                }
                BlockCommentState::LeftAsterisk => {
                    if first == b'*' {
                        lexer.mark_end();
                        state = BlockCommentState::LeftAsterisk;
                    } else {
                        if first == b'/' {
                            nesting_depth = nesting_depth.wrapping_sub(1);
                        }
                        state = BlockCommentState::Continuing;
                    }
                }
                BlockCommentState::Continuing => {
                    lexer.mark_end();
                    match first {
                        b'/' => state = BlockCommentState::LeftForwardSlash,
                        b'*' => state = BlockCommentState::LeftAsterisk,
                        _ => {}
                    }
                }
            }
            lexer.advance(false);
            if first == b'/' && nesting_depth != 0 {
                lexer.mark_end();
            }
        }
        lexer.set_result_symbol(BLOCK_COMMENT_CONTENT as u16);
        return true;
    }
    false
}

impl ExternalScanner for Scanner {
    fn scan(&mut self, lexer: &mut dyn Lexer, valid_symbols: &[bool]) -> bool {
        // The sentinel detects error recovery, when all tokens are valid.
        if valid_symbols[ERROR_SENTINEL] {
            return false;
        }
        if valid_symbols[BLOCK_COMMENT_CONTENT]
            || valid_symbols[BLOCK_INNER_DOC_MARKER]
            || valid_symbols[BLOCK_OUTER_DOC_MARKER]
        {
            return process_block_comment(lexer, valid_symbols);
        }
        if valid_symbols[STRING_CONTENT] && !valid_symbols[FLOAT_LITERAL] && process_string(lexer) {
            return true;
        }
        // Empty content before a quote falls through to STRING_CLOSE.
        if valid_symbols[STRING_CLOSE] && lexer.lookahead() == i32::from(b'"') {
            lexer.advance(false);
            lexer.set_result_symbol(STRING_CLOSE as u16);
            lexer.mark_end();
            return true;
        }
        if valid_symbols[LINE_DOC_CONTENT] {
            return process_line_doc_content(lexer);
        }
        while is_space(lexer.lookahead()) {
            lexer.advance(true);
        }
        if valid_symbols[RAW_STRING_LITERAL_START]
            && (lexer.lookahead() == i32::from(b'r')
                || lexer.lookahead() == i32::from(b'b')
                || lexer.lookahead() == i32::from(b'c'))
        {
            return self.scan_raw_string_start(lexer);
        }
        if valid_symbols[RAW_STRING_LITERAL_CONTENT] {
            return self.scan_raw_string_content(lexer);
        }
        if valid_symbols[RAW_STRING_LITERAL_END] && lexer.lookahead() == i32::from(b'"') {
            return self.scan_raw_string_end(lexer);
        }
        if valid_symbols[FLOAT_LITERAL] && is_digit(lexer.lookahead()) {
            return process_float_literal(lexer);
        }
        false
    }

    fn serialize(&mut self, buffer: &mut [u8]) -> usize {
        buffer[0] = self.opening_hash_count;
        1
    }

    fn deserialize(&mut self, buffer: &[u8]) {
        self.opening_hash_count = if buffer.len() == 1 { buffer[0] } else { 0 };
    }
}

/// Creates a scanner (C's `tree_sitter_rust_external_scanner_create`).
pub(crate) fn create() -> Box<dyn ExternalScanner> {
    Box::new(Scanner::default())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Eq)]
    enum Event {
        Advance { at: usize, skip: bool },
        Mark(usize),
        Symbol(u16),
    }

    struct TestLexer<'a> {
        input: &'a str,
        position: usize,
        start: usize,
        end: Option<usize>,
        symbol: u16,
        events: Vec<Event>,
    }

    impl<'a> TestLexer<'a> {
        fn new(input: &'a str) -> Self {
            Self {
                input,
                position: 0,
                start: 0,
                end: None,
                symbol: u16::MAX,
                events: Vec::new(),
            }
        }

        fn token(&self) -> &str {
            &self.input[self.start..self.end.unwrap_or(self.position)]
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
            self.events.push(Event::Advance {
                at: self.position,
                skip,
            });
            if let Some(c) = self.input[self.position..].chars().next() {
                self.position += c.len_utf8();
            }
            if skip {
                self.start = self.position;
            }
        }

        fn mark_end(&mut self) {
            self.end = Some(self.position);
            self.events.push(Event::Mark(self.position));
        }

        fn get_column(&mut self) -> u32 {
            panic!("the Rust scanner does not query columns")
        }

        fn is_at_included_range_start(&self) -> bool {
            panic!("the Rust scanner does not query included ranges")
        }

        fn eof(&self) -> bool {
            self.position == self.input.len()
        }
    }

    fn scan<'a>(
        scanner: &mut dyn ExternalScanner,
        input: &'a str,
        tokens: &[usize],
    ) -> (bool, TestLexer<'a>) {
        let mut valid = [false; ERROR_SENTINEL + 1];
        for &token in tokens {
            valid[token] = true;
        }
        let mut lexer = TestLexer::new(input);
        let accepted = scanner.scan(&mut lexer, &valid);
        (accepted, lexer)
    }

    #[test]
    fn serialization_is_exactly_one_byte() {
        let mut scanner = create();
        let mut buffer = [0xaa; 4];
        assert_eq!(scanner.serialize(&mut buffer), 1);
        assert_eq!(buffer, [0, 0xaa, 0xaa, 0xaa]);
        for hash_count in 0..=u8::MAX {
            scanner.deserialize(&[hash_count]);
            assert_eq!(scanner.serialize(&mut buffer), 1);
            assert_eq!(buffer, [hash_count, 0xaa, 0xaa, 0xaa]);
        }
        for invalid in [&[][..], &[7, 8][..]] {
            scanner.deserialize(&[9]);
            scanner.deserialize(invalid);
            scanner.serialize(&mut buffer);
            assert_eq!(buffer[0], 0);
        }
    }

    #[test]
    fn recovery_does_not_touch_the_lexer_or_state() {
        let mut scanner = Scanner {
            opening_hash_count: 7,
        };
        let mut lexer = TestLexer::new(" /* comment */");
        assert!(!scanner.scan(&mut lexer, &[true; ERROR_SENTINEL + 1]));
        assert!(lexer.events.is_empty());
        assert_eq!(scanner.opening_hash_count, 7);
    }

    #[test]
    fn string_content_preserves_whitespace_and_embedded_nul() {
        let mut scanner = Scanner::default();
        let (accepted, lexer) = scan(&mut scanner, " \0中\\n", &[STRING_CONTENT]);
        assert!(accepted);
        assert_eq!(lexer.token(), " \0中");
        assert_eq!(lexer.result_symbol(), STRING_CONTENT as u16);
        assert!(
            !lexer
                .events
                .iter()
                .any(|event| matches!(event, Event::Advance { skip: true, .. }))
        );
        let (accepted, lexer) = scan(&mut scanner, "unterminated", &[STRING_CONTENT]);
        assert!(!accepted);
        assert!(lexer.eof());
        assert_eq!(lexer.end, None);
        assert_eq!(lexer.result_symbol(), u16::MAX);
    }

    #[test]
    fn empty_string_content_falls_through_to_string_close() {
        let mut scanner = Scanner::default();
        let (accepted, lexer) = scan(&mut scanner, "\"", &[STRING_CONTENT, STRING_CLOSE]);
        assert!(accepted);
        assert_eq!(lexer.token(), "\"");
        assert_eq!(
            lexer.events,
            [
                Event::Symbol(STRING_CONTENT as u16),
                Event::Mark(0),
                Event::Advance { at: 0, skip: false },
                Event::Symbol(STRING_CLOSE as u16),
                Event::Mark(1),
            ]
        );
        let (accepted, lexer) = scan(&mut scanner, "\\", &[STRING_CONTENT, STRING_CLOSE]);
        assert!(!accepted);
        assert_eq!(lexer.position, 0);
        assert_eq!(lexer.end, Some(0));
        // Closing quotes are checked before, not after, whitespace skipping.
        let (accepted, lexer) = scan(&mut scanner, " \"", &[STRING_CLOSE]);
        assert!(!accepted);
        assert_eq!(lexer.position, 1);
    }

    #[test]
    fn raw_string_prefixes_and_wrapping_hash_count() {
        let mut scanner = Scanner::default();
        for prefix in ["r", "br", "cr"] {
            for count in [0usize, 1, 2, 255, 256, 257] {
                let input = format!(" \t{prefix}{}\"content", "#".repeat(count));
                let (accepted, lexer) = scan(&mut scanner, &input, &[RAW_STRING_LITERAL_START]);
                assert!(accepted);
                assert_eq!(scanner.opening_hash_count, count as u8);
                assert_eq!(lexer.result_symbol(), RAW_STRING_LITERAL_START as u16);
                assert_eq!(lexer.position, 2 + prefix.len() + count + 1);
                assert_eq!(lexer.start, 2);
                assert_eq!(lexer.end, None);
            }
        }
        for input in ["b\"", "c\"", "r###x", "br##x"] {
            scanner.opening_hash_count = 9;
            assert!(!scan(&mut scanner, input, &[RAW_STRING_LITERAL_START]).0);
            assert_eq!(scanner.opening_hash_count, 9);
        }
    }

    #[test]
    fn raw_content_marks_before_the_matching_delimiter() {
        let mut scanner = Scanner {
            opening_hash_count: 2,
        };
        let (accepted, lexer) = scan(
            &mut scanner,
            " \talpha\"#beta\"###tail",
            &[RAW_STRING_LITERAL_CONTENT],
        );
        assert!(accepted);
        assert_eq!(lexer.result_symbol(), RAW_STRING_LITERAL_CONTENT as u16);
        assert_eq!(lexer.token(), "alpha\"#beta");
        assert_eq!(lexer.start, 2);
        assert_eq!(lexer.end, Some(13));
        assert_eq!(lexer.position, 16);
        assert_eq!(lexer.lookahead(), i32::from(b'#'));

        let (accepted, lexer) = scan(&mut scanner, "\"##tail", &[RAW_STRING_LITERAL_END]);
        assert!(accepted);
        assert_eq!(lexer.token(), "\"##");
        assert_eq!(lexer.end, None);
        // Closing is blind consumption: content scanning already checked it.
        let (accepted, lexer) = scan(&mut scanner, "\"xy", &[RAW_STRING_LITERAL_END]);
        assert!(accepted);
        assert_eq!(lexer.token(), "\"xy");

        let (accepted, lexer) = scan(&mut scanner, "text\"#", &[RAW_STRING_LITERAL_CONTENT]);
        assert!(!accepted);
        assert!(lexer.eof());
        assert_eq!(lexer.end, Some(4));
        let (accepted, lexer) = scan(&mut scanner, "\"##", &[RAW_STRING_LITERAL_CONTENT]);
        assert!(accepted);
        assert_eq!(lexer.token(), "");
    }

    #[test]
    fn floats_preserve_speculative_exponent_and_suffix_boundaries() {
        let mut scanner = Scanner::default();
        for (input, accepted, end, position) in [
            ("1", false, Some(1), 1),
            ("1f32", false, Some(1), 1),
            ("1.max(2)", false, None, 2),
            ("1..2", false, None, 2),
            ("1.e2", false, None, 2),
            ("1.", true, Some(2), 2),
            ("1._", true, Some(3), 3),
            ("1.25", true, Some(4), 4),
            ("1e+", true, Some(1), 3),
            ("1.2e-x", true, Some(3), 5),
            ("1e_", true, Some(3), 3),
            ("1e+2f32", true, Some(7), 7),
            ("1.0i16", true, Some(6), 6),
            ("1.0u8", true, Some(5), 5),
            ("1.0foo", true, Some(3), 4),
            ("1.0f32_", true, Some(6), 6),
            ("1_2.3_4E-5_6", true, Some(12), 12),
        ] {
            let (actual, lexer) = scan(&mut scanner, input, &[FLOAT_LITERAL]);
            assert_eq!(actual, accepted, "{input}");
            assert_eq!(lexer.end, end, "{input}");
            assert_eq!(lexer.position, position, "{input}");
            assert_eq!(lexer.result_symbol(), FLOAT_LITERAL as u16, "{input}");
        }
        let (accepted, lexer) = scan(&mut scanner, "1.0", &[FLOAT_LITERAL, STRING_CONTENT]);
        assert!(accepted);
        assert_eq!(lexer.result_symbol(), FLOAT_LITERAL as u16);
    }

    #[test]
    fn character_classes_use_the_c_locale() {
        let mut scanner = Scanner::default();
        for input in ["\u{a0}1.0", "\u{2003}1.0", "١.٠"] {
            let (accepted, lexer) = scan(&mut scanner, input, &[FLOAT_LITERAL]);
            assert!(!accepted);
            assert!(lexer.events.is_empty());
        }
        // Non-ASCII letters do not prevent a trailing-dot float in the C locale.
        let (accepted, lexer) = scan(&mut scanner, "1.é", &[FLOAT_LITERAL]);
        assert!(accepted);
        assert_eq!(lexer.token(), "1.");
        let (accepted, lexer) = scan(&mut scanner, "\t\n\r\u{b}\u{c} 1.0", &[FLOAT_LITERAL]);
        assert!(accepted);
        assert_eq!(lexer.start, 6);
        assert_eq!(lexer.token(), "1.0");
    }

    #[test]
    fn line_doc_content_includes_newline_without_skipping() {
        let mut scanner = Scanner::default();
        for (input, token) in [
            (" doc\r\nnext", " doc\r\n"),
            (" doc", " doc"),
            ("\nnext", "\n"),
            ("", ""),
        ] {
            let (accepted, lexer) = scan(&mut scanner, input, &[LINE_DOC_CONTENT]);
            assert!(accepted);
            assert_eq!(lexer.token(), token);
            assert_eq!(lexer.start, 0);
            assert_eq!(lexer.end, None);
            assert_eq!(lexer.result_symbol(), LINE_DOC_CONTENT as u16);
        }
    }

    #[test]
    fn block_doc_markers_and_empty_comments() {
        let mut scanner = Scanner::default();
        let valid = [
            BLOCK_INNER_DOC_MARKER,
            BLOCK_OUTER_DOC_MARKER,
            BLOCK_COMMENT_CONTENT,
        ];
        for (input, symbol, end) in [
            ("!doc", BLOCK_INNER_DOC_MARKER, None),
            ("*doc", BLOCK_OUTER_DOC_MARKER, Some(1)),
        ] {
            let (accepted, lexer) = scan(&mut scanner, input, &valid);
            assert!(accepted);
            assert_eq!(lexer.position, 1);
            assert_eq!(lexer.end, end);
            assert_eq!(lexer.result_symbol(), symbol as u16);
        }
        let (accepted, lexer) = scan(&mut scanner, "*/", &valid);
        assert!(!accepted);
        assert_eq!(lexer.position, 1);
        assert_eq!(lexer.end, Some(1));
        let (accepted, lexer) = scan(&mut scanner, "*/", &[BLOCK_COMMENT_CONTENT]);
        assert!(!accepted);
        assert_eq!(lexer.position, 1);
        assert_eq!(lexer.end, None);
        let (accepted, lexer) = scan(&mut scanner, "*** doc */rest", &valid);
        assert!(accepted);
        assert_eq!(lexer.token(), "*** doc ");
        assert_eq!(lexer.result_symbol(), BLOCK_COMMENT_CONTENT as u16);
    }

    #[test]
    fn block_comments_keep_nested_delimiters_but_exclude_the_outer_close() {
        let mut scanner = Scanner::default();
        for input in [
            " text */rest",
            " /* nested */ outer */rest",
            "/* nested /* deeper */ */ outer */rest",
            "a//*/rest",
            "a***b***/rest",
        ] {
            let (accepted, lexer) = scan(&mut scanner, input, &[BLOCK_COMMENT_CONTENT]);
            let closing = input.rfind("*/").unwrap();
            assert!(accepted, "{input}");
            assert_eq!(lexer.token(), &input[..closing], "{input}");
            assert_eq!(lexer.position, closing + 2, "{input}");
            assert_eq!(lexer.result_symbol(), BLOCK_COMMENT_CONTENT as u16);
        }
    }

    #[test]
    fn unterminated_block_comments_keep_the_last_c_mark() {
        let mut scanner = Scanner::default();
        for (input, token, end) in [
            ("", "", None),
            ("a", "a", None),
            ("ab", "a", Some(1)),
            ("abc", "ab", Some(2)),
            ("a/", "a/", Some(2)),
            ("a*", "a", Some(1)),
        ] {
            let (accepted, lexer) = scan(&mut scanner, input, &[BLOCK_COMMENT_CONTENT]);
            assert!(accepted);
            assert_eq!(lexer.token(), token, "{input}");
            assert_eq!(lexer.end, end, "{input}");
        }
    }

    #[test]
    fn block_comments_preserve_c_char_truncation() {
        let mut scanner = Scanner::default();
        // U+0121 has the same low byte as '!'.
        let (accepted, lexer) = scan(&mut scanner, "ġdoc", &[BLOCK_INNER_DOC_MARKER]);
        assert!(accepted);
        assert_eq!(lexer.token(), "ġ");
        // U+012A/U+012F have the low bytes of '*' and '/', respectively.
        let (accepted, lexer) = scan(&mut scanner, "aĪįrest", &[BLOCK_COMMENT_CONTENT]);
        assert!(accepted);
        assert_eq!(lexer.token(), "a");
        assert_eq!(lexer.position, 5);
    }
}
