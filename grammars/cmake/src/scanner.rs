//! The CMake external scanner, translated from `src/scanner.c`.

use ts_port_tables::{ExternalScanner, Lexer};

const BRACKET_ARGUMENT_OPEN: usize = 0;
const BRACKET_ARGUMENT_CONTENT: usize = 1;
const BRACKET_ARGUMENT_CLOSE: usize = 2;
const BRACKET_COMMENT_OPEN: usize = 3;
const BRACKET_COMMENT_CONTENT: usize = 4;
const BRACKET_COMMENT_CLOSE: usize = 5;
const LINE_COMMENT: usize = 6;

// C serializes the struct verbatim: a four-byte unsigned followed by a
// four-byte TokenType enum, both native-endian, with no padding.
const STATE_SIZE: usize = 8;

/// The scanner's state (C's `payload`), zero-initialized by `ts_calloc`.
// The initial token is BRACKET_ARGUMENT_OPEN (zero), including when error
// recovery enables content without a preceding opener.
#[derive(Default)]
pub(crate) struct Scanner {
    level: u32,
    // Keep the raw enum representation so deserialization preserves every byte.
    token: u32,
}

impl Scanner {
    fn is_open_brackets(&mut self, lexer: &mut dyn Lexer) -> bool {
        if lexer.lookahead() != i32::from(b'[') {
            return false;
        }

        lexer.advance(false);

        let mut level = 0u32;
        while lexer.lookahead() == i32::from(b'=') {
            level = level.wrapping_add(1);
            lexer.advance(false);
        }

        if lexer.lookahead() != i32::from(b'[') {
            return false;
        }

        lexer.advance(false);
        lexer.mark_end();
        self.level = level;
        true
    }

    fn parse_bracketed_content(&self, lexer: &mut dyn Lexer) {
        while !lexer.eof() {
            if lexer.lookahead() == i32::from(b']') {
                let mut level = 0u32;

                lexer.mark_end();
                lexer.advance(false);
                while lexer.lookahead() == i32::from(b'=') {
                    level = level.wrapping_add(1);
                    lexer.advance(false);
                }

                if level == self.level && lexer.lookahead() == i32::from(b']') {
                    break;
                }

                // In particular, a partial closer at EOF does not move the mark
                // past its initial ']'. A later ordinary character can do so.
                continue;
            }

            lexer.advance(false);
            lexer.mark_end();
        }
    }

    fn is_close_brackets(&mut self, lexer: &mut dyn Lexer) -> bool {
        if lexer.lookahead() != i32::from(b']') {
            return false;
        }

        let mut level = 0u32;
        lexer.advance(false);
        while lexer.lookahead() == i32::from(b'=') {
            level = level.wrapping_add(1);
            lexer.advance(false);
        }

        if level != self.level || lexer.lookahead() != i32::from(b']') {
            return false;
        }

        lexer.advance(false);
        lexer.mark_end();
        self.level = 0;
        true
    }
}

impl ExternalScanner for Scanner {
    fn scan(&mut self, lexer: &mut dyn Lexer, valid_symbols: &[bool]) -> bool {
        // `iswspace` in the reference's default C locale, not Unicode whitespace.
        // C skips it even when scanning bracketed content.
        while matches!(lexer.lookahead(), 0x09..=0x0d | 0x20) {
            lexer.advance(true);
        }

        if valid_symbols[BRACKET_ARGUMENT_OPEN] && self.is_open_brackets(lexer) {
            lexer.set_result_symbol(BRACKET_ARGUMENT_OPEN as u16);
            self.token = BRACKET_ARGUMENT_OPEN as u32;
            return true;
        }
        if valid_symbols[BRACKET_ARGUMENT_CONTENT] && self.token == BRACKET_ARGUMENT_OPEN as u32 {
            self.parse_bracketed_content(lexer);
            lexer.set_result_symbol(BRACKET_ARGUMENT_CONTENT as u16);
            self.token = BRACKET_ARGUMENT_CONTENT as u32;
            return true;
        }
        if valid_symbols[BRACKET_ARGUMENT_CLOSE]
            && self.token == BRACKET_ARGUMENT_CONTENT as u32
            && self.is_close_brackets(lexer)
        {
            lexer.set_result_symbol(BRACKET_ARGUMENT_CLOSE as u16);
            return true;
        }
        if lexer.lookahead() == i32::from(b'#') {
            if !valid_symbols[BRACKET_COMMENT_OPEN] && !valid_symbols[LINE_COMMENT] {
                return false;
            }

            lexer.advance(false);
            // Both kinds of comment are tried if either comment opener is valid.
            if self.is_open_brackets(lexer) {
                lexer.set_result_symbol(BRACKET_COMMENT_OPEN as u16);
                self.token = BRACKET_COMMENT_OPEN as u32;
                return true;
            }

            while !matches!(lexer.lookahead(), 0x0d | 0x0a | 0) {
                lexer.advance(false);
            }

            lexer.mark_end();
            lexer.set_result_symbol(LINE_COMMENT as u16);
            return true;
        }
        if valid_symbols[BRACKET_COMMENT_CONTENT] && self.token == BRACKET_COMMENT_OPEN as u32 {
            self.parse_bracketed_content(lexer);
            lexer.set_result_symbol(BRACKET_COMMENT_CONTENT as u16);
            self.token = BRACKET_COMMENT_CONTENT as u32;
            return true;
        }
        if valid_symbols[BRACKET_COMMENT_CLOSE]
            && self.token == BRACKET_COMMENT_CONTENT as u32
            && self.is_close_brackets(lexer)
        {
            lexer.set_result_symbol(BRACKET_COMMENT_CLOSE as u16);
            return true;
        }

        false
    }

    fn serialize(&mut self, buffer: &mut [u8]) -> usize {
        buffer[..4].copy_from_slice(&self.level.to_ne_bytes());
        buffer[4..STATE_SIZE].copy_from_slice(&self.token.to_ne_bytes());
        STATE_SIZE
    }

    fn deserialize(&mut self, buffer: &[u8]) {
        if buffer.len() == STATE_SIZE {
            self.level = u32::from_ne_bytes(buffer[..4].try_into().unwrap());
            self.token = u32::from_ne_bytes(buffer[4..STATE_SIZE].try_into().unwrap());
        } else {
            // Empty or invalid snapshots reset both fields, just like calloc.
            *self = Self::default();
        }
    }
}

/// Creates a scanner (C's `tree_sitter_cmake_external_scanner_create`).
pub(crate) fn create() -> Box<dyn ExternalScanner> {
    Box::<Scanner>::default()
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
        input: Vec<char>,
        position: usize,
        end: Option<usize>,
        symbol: u16,
        events: Vec<Event>,
    }

    impl TestLexer {
        fn new(input: &str) -> Self {
            Self {
                input: input.chars().collect(),
                position: 0,
                end: None,
                symbol: u16::MAX,
                events: Vec::new(),
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
            self.symbol = symbol;
            self.events.push(Event::Symbol(symbol));
        }

        fn advance(&mut self, skip: bool) {
            self.events.push(Event::Advance(self.position, skip));
            assert!(self.position < self.input.len());
            self.position += 1;
        }

        fn mark_end(&mut self) {
            self.end = Some(self.position);
            self.events.push(Event::MarkEnd(self.position));
        }

        fn get_column(&mut self) -> u32 {
            panic!("CMake scanner does not request columns")
        }

        fn is_at_included_range_start(&self) -> bool {
            panic!("CMake scanner does not query included ranges")
        }

        fn eof(&self) -> bool {
            self.position == self.input.len()
        }
    }

    fn valid(tokens: &[usize]) -> [bool; 7] {
        let mut result = [false; 7];
        for &token in tokens {
            result[token] = true;
        }
        result
    }

    #[test]
    fn fresh_and_reset_scanners_allow_bracket_content_during_recovery() {
        let mut scanner = create();
        let mut snapshot = [0xff; STATE_SIZE];
        assert_eq!(scanner.serialize(&mut snapshot), STATE_SIZE);
        assert_eq!(snapshot, [0; STATE_SIZE]);

        for input in ["text", "", ")\n", "text"] {
            let mut lexer = TestLexer::new(input);
            assert!(scanner.scan(&mut lexer, &[true; 7]));
            assert_eq!(lexer.symbol, BRACKET_ARGUMENT_CONTENT as u16);
            assert_eq!(lexer.position, input.len());
            assert_eq!(lexer.end, (!input.is_empty()).then_some(input.len()));

            // Content changed the token; an empty snapshot must reset it so
            // that the next scan can emit content again, even at EOF.
            scanner.deserialize(&[]);
            assert_eq!(scanner.serialize(&mut snapshot), STATE_SIZE);
            assert_eq!(snapshot, [0; STATE_SIZE]);
        }
    }

    #[test]
    fn bracket_argument_sequence_and_callback_order() {
        let mut scanner = Scanner::default();
        let mut open = TestLexer::new(" \t[==[tail");
        assert!(scanner.scan(&mut open, &valid(&[BRACKET_ARGUMENT_OPEN])));
        assert_eq!(scanner.level, 2);
        assert_eq!(scanner.token, BRACKET_ARGUMENT_OPEN as u32);
        assert_eq!(open.end, Some(6));
        assert_eq!(
            open.events,
            [
                Event::Advance(0, true),
                Event::Advance(1, true),
                Event::Advance(2, false),
                Event::Advance(3, false),
                Event::Advance(4, false),
                Event::Advance(5, false),
                Event::MarkEnd(6),
                Event::Symbol(BRACKET_ARGUMENT_OPEN as u16),
            ]
        );

        let mut content = TestLexer::new("\na]===x]===]b]==]tail");
        assert!(scanner.scan(&mut content, &valid(&[BRACKET_ARGUMENT_CONTENT])));
        assert_eq!(scanner.token, BRACKET_ARGUMENT_CONTENT as u32);
        assert_eq!(content.end, Some(13));
        assert_eq!(content.position, 16); // The final ']' was only inspected.
        assert_eq!(content.events[0], Event::Advance(0, true));

        let mut close = TestLexer::new("]==]tail");
        assert!(scanner.scan(&mut close, &valid(&[BRACKET_ARGUMENT_CLOSE])));
        assert_eq!(close.end, Some(4));
        assert_eq!(scanner.level, 0);
        // Closing a bracket does not set state->token to the closing symbol.
        assert_eq!(scanner.token, BRACKET_ARGUMENT_CONTENT as u32);
    }

    #[test]
    fn content_at_eof_preserves_partial_closer_mark() {
        for (input, end) in [
            ("abc]=", Some(3)),
            ("abc]===", Some(3)),
            ("abc", Some(3)),
            ("", None),
        ] {
            let mut scanner = Scanner {
                level: 2,
                token: BRACKET_ARGUMENT_OPEN as u32,
            };
            let mut lexer = TestLexer::new(input);
            assert!(scanner.scan(&mut lexer, &valid(&[BRACKET_ARGUMENT_CONTENT])));
            assert_eq!(lexer.end, end, "{input:?}");
            assert_eq!(lexer.position, input.len());
        }
    }

    #[test]
    fn failed_bracket_attempts_keep_their_advances() {
        let mut scanner = Scanner {
            level: 7,
            token: BRACKET_ARGUMENT_OPEN as u32,
        };
        let mut lexer = TestLexer::new("[=oops");
        assert!(scanner.scan(
            &mut lexer,
            &valid(&[BRACKET_ARGUMENT_OPEN, BRACKET_ARGUMENT_CONTENT])
        ));
        assert_eq!(scanner.level, 7);
        // The content scan starts after the failed opener, not at its '['.
        assert_eq!(
            &lexer.events[..4],
            &[
                Event::Advance(0, false),
                Event::Advance(1, false),
                Event::Advance(2, false),
                Event::MarkEnd(3),
            ]
        );

        let mut close = TestLexer::new("]=]");
        assert!(!scanner.scan(&mut close, &valid(&[BRACKET_ARGUMENT_CLOSE])));
        assert_eq!(close.position, 2);
        assert_eq!(close.end, None);
        assert_eq!(scanner.level, 7);
    }

    #[test]
    fn comments_accept_either_kind_when_either_opener_is_valid() {
        let mut scanner = Scanner::default();
        let mut open = TestLexer::new("#[=[body");
        assert!(scanner.scan(&mut open, &valid(&[LINE_COMMENT])));
        assert_eq!(open.symbol, BRACKET_COMMENT_OPEN as u16);
        assert_eq!(scanner.level, 1);

        let mut content = TestLexer::new(" body]=]");
        assert!(scanner.scan(&mut content, &valid(&[BRACKET_COMMENT_CONTENT])));
        assert_eq!(content.end, Some(5));
        assert_eq!(content.symbol, BRACKET_COMMENT_CONTENT as u16);
        let mut close = TestLexer::new("]=]");
        assert!(scanner.scan(&mut close, &valid(&[BRACKET_COMMENT_CLOSE])));
        assert_eq!(close.end, Some(3));
        assert_eq!(scanner.token, BRACKET_COMMENT_CONTENT as u32);
        assert_eq!(scanner.level, 0);

        for terminator in ["\r", "\n", "\0", ""] {
            let mut line = TestLexer::new(&format!("#[=oops{terminator}"));
            assert!(scanner.scan(&mut line, &valid(&[BRACKET_COMMENT_OPEN])));
            assert_eq!(line.symbol, LINE_COMMENT as u16);
            assert_eq!(line.end, Some(7));
            assert_eq!(scanner.token, BRACKET_COMMENT_CONTENT as u32);
        }
    }

    #[test]
    fn hash_dispatch_precedes_bracket_comment_content() {
        let mut scanner = Scanner {
            level: 1,
            token: BRACKET_COMMENT_OPEN as u32,
        };
        let mut lexer = TestLexer::new("#text]=]");
        assert!(!scanner.scan(&mut lexer, &valid(&[BRACKET_COMMENT_CONTENT])));
        assert!(lexer.events.is_empty());

        // Argument content, in contrast, is checked before the '#' dispatch.
        scanner.token = BRACKET_ARGUMENT_OPEN as u32;
        assert!(scanner.scan(&mut lexer, &valid(&[BRACKET_ARGUMENT_CONTENT])));
        assert_eq!(lexer.end, Some(5));
    }

    #[test]
    fn only_c_locale_whitespace_is_skipped() {
        let mut scanner = Scanner::default();
        let mut lexer = TestLexer::new("\t\n\u{b}\u{c}\r [[");
        assert!(scanner.scan(&mut lexer, &valid(&[BRACKET_ARGUMENT_OPEN])));
        assert!(
            lexer.events[..6]
                .iter()
                .enumerate()
                .all(|(i, event)| *event == Event::Advance(i, true))
        );

        for whitespace in ['\u{85}', '\u{a0}', '\u{2003}'] {
            let mut lexer = TestLexer::new(&format!("{whitespace}[["));
            assert!(!scanner.scan(&mut lexer, &valid(&[BRACKET_ARGUMENT_OPEN])));
            assert!(lexer.events.is_empty());
        }
    }

    #[test]
    fn snapshots_are_native_endian_and_bad_lengths_reset_both_fields() {
        let mut scanner = Scanner {
            level: 0x12345678,
            token: 0xa1b2c3d4,
        };
        let mut buffer = [0xff; 16];
        assert_eq!(scanner.serialize(&mut buffer), STATE_SIZE);
        assert_eq!(&buffer[..4], &scanner.level.to_ne_bytes());
        assert_eq!(&buffer[4..8], &scanner.token.to_ne_bytes());
        assert_eq!(&buffer[8..], &[0xff; 8]);

        let mut restored = Scanner::default();
        restored.deserialize(&buffer[..8]);
        assert_eq!(restored.level, scanner.level);
        assert_eq!(restored.token, scanner.token);
        for length in [0, 1, 7, 9, 16] {
            restored.level = 42;
            restored.token = scanner.token;
            restored.deserialize(&buffer[..length]);
            assert_eq!(restored.level, 0);
            assert_eq!(restored.token, BRACKET_ARGUMENT_OPEN as u32);

            let mut lexer = TestLexer::new("text");
            assert!(restored.scan(&mut lexer, &[true; 7]));
            assert_eq!(lexer.symbol, BRACKET_ARGUMENT_CONTENT as u16);
            assert_eq!(lexer.end, Some(4));
        }
    }
}
