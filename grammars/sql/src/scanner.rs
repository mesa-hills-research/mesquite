//! The SQL external scanner, translated from `src/scanner.c`.

use tree_sitter_language::{ExternalScanner, Lexer, SERIALIZATION_BUFFER_SIZE};

const DOLLAR_QUOTED_STRING_START_TAG: usize = 0;
const DOLLAR_QUOTED_STRING_END_TAG: usize = 1;
const DOLLAR_QUOTED_STRING: usize = 2;

/// The C scanner stores a nullable, NUL-terminated byte string, not UTF-8.
#[derive(Default)]
pub(crate) struct Scanner {
    start_tag: Option<Vec<u8>>,
}

fn is_space(c: i32) -> bool {
    // `iswspace` in C's default locale.
    matches!(c, 0x09..=0x0d | 0x20)
}

fn skip_whitespace(lexer: &mut dyn Lexer) {
    while is_space(lexer.lookahead()) {
        lexer.advance(true);
    }
}

/// The portion used by C's `strcmp` and `strlen`. Narrowing a code point to
/// `char` can introduce an embedded NUL, even for a non-NUL input character.
fn c_string_content(bytes: &[u8]) -> &[u8] {
    &bytes[..bytes
        .iter()
        .position(|&byte| byte == 0)
        .unwrap_or(bytes.len())]
}

fn scan_dollar_string_tag(lexer: &mut dyn Lexer) -> Option<Vec<u8>> {
    if lexer.lookahead() != i32::from(b'$') {
        return None;
    }

    // Vec replaces C's add_char allocation/growth and owns the final terminator.
    let mut tag = vec![b'$'];
    lexer.advance(false);
    while lexer.lookahead() != i32::from(b'$') && !is_space(lexer.lookahead()) && !lexer.eof() {
        tag.push(lexer.lookahead() as u8);
        lexer.advance(false);
    }

    if lexer.lookahead() == i32::from(b'$') {
        tag.push(lexer.lookahead() as u8);
        tag.push(0);
        lexer.advance(false);
        Some(tag)
    } else {
        None
    }
}

impl ExternalScanner for Scanner {
    fn scan(&mut self, lexer: &mut dyn Lexer, valid_symbols: &[bool]) -> bool {
        if valid_symbols[DOLLAR_QUOTED_STRING_START_TAG] && self.start_tag.is_none() {
            skip_whitespace(lexer);
            let Some(start_tag) = scan_dollar_string_tag(lexer) else {
                return false;
            };
            self.start_tag = Some(start_tag);
            lexer.set_result_symbol(DOLLAR_QUOTED_STRING_START_TAG as u16);
            return true;
        }

        if valid_symbols[DOLLAR_QUOTED_STRING_END_TAG]
            && let Some(start_tag) = &self.start_tag
        {
            skip_whitespace(lexer);
            if let Some(end_tag) = scan_dollar_string_tag(lexer)
                && c_string_content(&end_tag) == c_string_content(start_tag)
            {
                self.start_tag = None;
                lexer.set_result_symbol(DOLLAR_QUOTED_STRING_END_TAG as u16);
                return true;
            }
            return false;
        }

        if valid_symbols[DOLLAR_QUOTED_STRING] {
            lexer.mark_end();
            skip_whitespace(lexer);
            let Some(start_tag) = scan_dollar_string_tag(lexer) else {
                return false;
            };

            if let Some(active_tag) = &self.start_tag
                && c_string_content(active_tag) == c_string_content(&start_tag)
            {
                return false;
            }

            loop {
                if lexer.eof() {
                    return false;
                }

                let Some(end_tag) = scan_dollar_string_tag(lexer) else {
                    // Even a failed tag attempt may have advanced the lexer.
                    // C always advances once more, including at EOF.
                    lexer.advance(false);
                    continue;
                };

                if c_string_content(&end_tag) == c_string_content(&start_tag) {
                    lexer.mark_end();
                    lexer.set_result_symbol(DOLLAR_QUOTED_STRING as u16);
                    return true;
                }
            }
        }

        false
    }

    fn serialize(&mut self, buffer: &mut [u8]) -> usize {
        let Some(start_tag) = &self.start_tag else {
            return 0;
        };
        let contents = c_string_content(start_tag);
        let tag_length = contents.len() + 1;
        // C rejects equality with the 1024-byte limit too. The slice-length
        // guard additionally handles callers providing a smaller buffer safely.
        if tag_length >= SERIALIZATION_BUFFER_SIZE || tag_length > buffer.len() {
            return 0;
        }

        buffer[..contents.len()].copy_from_slice(contents);
        buffer[contents.len()] = 0;
        // Serialization deliberately consumes the active tag in the C scanner.
        self.start_tag = None;
        tag_length
    }

    fn deserialize(&mut self, buffer: &[u8]) {
        self.start_tag = if buffer.len() > 1 {
            Some(buffer.to_vec())
        } else {
            None
        };
    }
}

/// Creates a scanner (C's `tree_sitter_sql_external_scanner_create`).
pub(crate) fn create() -> Box<dyn ExternalScanner> {
    Box::new(Scanner::default())
}

#[cfg(test)]
mod tests {
    use super::*;

    const START: &[bool] = &[true, false, false];
    const END: &[bool] = &[false, true, false];
    const STRING: &[bool] = &[false, false, true];

    #[derive(Debug, PartialEq, Eq)]
    enum Event {
        Advance(usize, bool),
        MarkEnd(usize),
        Symbol(u16),
    }

    struct TestLexer {
        input: Vec<char>,
        position: usize,
        symbol: u16,
        events: Vec<Event>,
    }

    impl TestLexer {
        fn new(input: &str) -> Self {
            Self {
                input: input.chars().collect(),
                position: 0,
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
            if self.position < self.input.len() {
                self.position += 1;
            }
        }
        fn mark_end(&mut self) {
            self.events.push(Event::MarkEnd(self.position));
        }
        fn get_column(&mut self) -> u32 {
            panic!("SQL scanner does not query the column")
        }
        fn is_at_included_range_start(&self) -> bool {
            panic!("SQL scanner does not query included ranges")
        }
        fn eof(&self) -> bool {
            self.position == self.input.len()
        }
    }

    #[test]
    fn start_tag_skips_whitespace_and_serialization_consumes_state() {
        let mut scanner = create();
        let mut lexer = TestLexer::new(" \t$$rest");
        assert!(scanner.scan(&mut lexer, START));
        assert_eq!(
            lexer.events,
            [
                Event::Advance(0, true),
                Event::Advance(1, true),
                Event::Advance(2, false),
                Event::Advance(3, false),
                Event::Symbol(0),
            ]
        );
        let mut buffer = [0xff; SERIALIZATION_BUFFER_SIZE];
        assert_eq!(scanner.serialize(&mut buffer), 3);
        assert_eq!(&buffer[..4], b"$$\0\xff");
        assert_eq!(scanner.serialize(&mut buffer), 0);
        scanner.deserialize(&buffer[..3]);
        let mut lexer = TestLexer::new("\n$$");
        assert!(scanner.scan(&mut lexer, END));
        assert_eq!(lexer.result_symbol(), 1);
        assert_eq!(scanner.serialize(&mut buffer), 0);
    }

    #[test]
    fn end_tag_mismatch_preserves_state_and_does_not_try_a_whole_string() {
        let mut scanner = Scanner::default();
        scanner.deserialize(b"$outer$\0");
        let mut lexer = TestLexer::new(" $inner$text$inner$");
        assert!(!scanner.scan(&mut lexer, &[true, true, true]));
        assert_eq!(lexer.position, 8);
        assert_eq!(scanner.start_tag.as_deref(), Some(b"$outer$\0".as_slice()));
        assert!(lexer.events.iter().all(|e| matches!(e, Event::Advance(..))));
        let mut lexer = TestLexer::new("$outer$");
        assert!(scanner.scan(&mut lexer, END));
        assert!(scanner.start_tag.is_none());
    }

    #[test]
    fn whole_string_marks_before_whitespace_and_after_matching_tag() {
        let mut scanner = Scanner::default();
        scanner.deserialize(b"$outer$\0");
        let input = " \n$a$body$other$more$a$tail";
        let mut lexer = TestLexer::new(input);
        assert!(scanner.scan(&mut lexer, STRING));
        let end = input.len() - 4;
        assert_eq!(lexer.position, end);
        assert_eq!(lexer.events.first(), Some(&Event::MarkEnd(0)));
        assert_eq!(lexer.events[1], Event::Advance(0, true));
        assert_eq!(lexer.events[2], Event::Advance(1, true));
        for position in 2..end {
            assert_eq!(lexer.events[position + 1], Event::Advance(position, false));
        }
        assert_eq!(lexer.events[lexer.events.len() - 2], Event::MarkEnd(end));
        assert_eq!(lexer.events.last(), Some(&Event::Symbol(2)));
        assert_eq!(scanner.start_tag.as_deref(), Some(b"$outer$\0".as_slice()));
    }

    #[test]
    fn whole_string_rejects_the_active_tag_without_consuming_the_body() {
        let mut scanner = Scanner::default();
        scanner.deserialize(b"$a$\0");
        let mut lexer = TestLexer::new("$a$text$a$");
        assert!(!scanner.scan(&mut lexer, STRING));
        assert_eq!(lexer.position, 3);
        assert_eq!(lexer.events.first(), Some(&Event::MarkEnd(0)));
        assert_eq!(lexer.events.last(), Some(&Event::Advance(2, false)));
        assert_eq!(scanner.start_tag.as_deref(), Some(b"$a$\0".as_slice()));
    }

    #[test]
    fn failed_tag_attempts_keep_their_advances_including_at_eof() {
        let mut scanner = Scanner::default();
        let mut lexer = TestLexer::new("$$x$bad tag$$");
        assert!(scanner.scan(&mut lexer, STRING));
        // The space ending the failed "$bad" attempt is consumed, not skipped.
        assert!(lexer.events.contains(&Event::Advance(7, false)));
        let mut lexer = TestLexer::new("$$x$bad");
        assert!(!scanner.scan(&mut lexer, STRING));
        assert_eq!(lexer.events.last(), Some(&Event::Advance(7, false)));
        assert_eq!(lexer.position, 7);
    }

    #[test]
    fn tag_characters_are_permissive_and_use_c_locale_whitespace() {
        let mut scanner = Scanner::default();
        let mut lexer = TestLexer::new("\u{a0}$$");
        assert!(!scanner.scan(&mut lexer, START));
        assert!(lexer.events.is_empty());
        let mut lexer = TestLexer::new("\t\n\u{b}\u{c}\r $!\u{a0}$");
        assert!(scanner.scan(&mut lexer, START));
        assert_eq!(scanner.start_tag.as_deref(), Some(b"$!\xa0$\0".as_slice()));
        scanner.deserialize(&[]);
        let mut lexer = TestLexer::new("$abc def$");
        assert!(!scanner.scan(&mut lexer, START));
        assert_eq!(lexer.position, 4);
        assert!(scanner.start_tag.is_none());
    }

    #[test]
    fn codepoints_are_narrowed_and_comparisons_stop_at_embedded_nuls() {
        let mut scanner = Scanner::default();
        let mut lexer = TestLexer::new("$\u{161}$");
        assert!(scanner.scan(&mut lexer, START));
        let mut lexer = TestLexer::new("$a$");
        assert!(scanner.scan(&mut lexer, END));

        let mut lexer = TestLexer::new("$\u{100}ignored$");
        assert!(scanner.scan(&mut lexer, START));
        let mut buffer = [0xff; SERIALIZATION_BUFFER_SIZE];
        assert_eq!(scanner.serialize(&mut buffer), 2);
        assert_eq!(&buffer[..3], b"$\0\xff");
        scanner.deserialize(&buffer[..2]);
        let mut lexer = TestLexer::new("$\0different$");
        assert!(scanner.scan(&mut lexer, END));

        let mut lexer = TestLexer::new("$\u{100}first$body$\0second$");
        assert!(scanner.scan(&mut lexer, STRING));
        assert!(lexer.eof());
    }

    #[test]
    fn serialization_limit_is_strict_and_failure_keeps_the_tag() {
        let mut scanner = Scanner::default();
        let mut buffer = [0xff; SERIALIZATION_BUFFER_SIZE];
        for content_length in [1020, 1021, 2200] {
            let tag = format!("${}$", "a".repeat(content_length));
            assert!(scanner.scan(&mut TestLexer::new(&tag), START));
            if content_length == 1020 {
                assert_eq!(scanner.serialize(&mut buffer), 1023);
                assert_eq!(&buffer[..1022], tag.as_bytes());
                assert_eq!(buffer[1022], 0);
                assert!(scanner.start_tag.is_none());
            } else {
                assert_eq!(scanner.serialize(&mut buffer), 0);
                assert!(scanner.start_tag.is_some());
                assert!(scanner.scan(&mut TestLexer::new(&tag), END));
            }
        }
    }

    #[test]
    fn deserialization_resets_state_for_empty_and_one_byte_snapshots() {
        let mut scanner = Scanner::default();
        for buffer in [b"".as_slice(), b"x".as_slice()] {
            scanner.deserialize(b"$active$\0");
            scanner.deserialize(buffer);
            assert!(scanner.start_tag.is_none());
        }
        scanner.deserialize(b"$$\0");
        scanner.deserialize(b"$new$\0");
        assert_eq!(scanner.start_tag.as_deref(), Some(b"$new$\0".as_slice()));
    }
}
