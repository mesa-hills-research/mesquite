//! The CUDA raw-string external scanner, translated from `src/scanner.c`.
//!
//! CUDA's C scanner matches the C++ scanner apart from exported function names;
//! this translation preserves the same raw-string handling and snapshots.

use tree_sitter_language::{ExternalScanner, Lexer, SERIALIZATION_BUFFER_SIZE, Symbol};

const RAW_STRING_DELIMITER: Symbol = 0;
const RAW_STRING_CONTENT: Symbol = 1;
const MAX_DELIMITER_LENGTH: usize = 16;
// The C scanner stores native-endian, 32-bit wchar_t values.
const WCHAR_SIZE: usize = size_of::<i32>();
const _: () = assert!(MAX_DELIMITER_LENGTH * WCHAR_SIZE < SERIALIZATION_BUFFER_SIZE);

/// The scanner's state (C's `payload`).
#[derive(Default)]
pub(crate) struct Scanner {
    delimiter_length: usize,
    delimiter: [i32; MAX_DELIMITER_LENGTH],
}

impl Scanner {
    fn reset(&mut self) {
        self.delimiter_length = 0;
        self.delimiter.fill(0);
    }

    /// Scan the delimiter in `R"delimiter(content)delimiter"`.
    fn scan_raw_string_delimiter(&mut self, lexer: &mut dyn Lexer) -> bool {
        if self.delimiter_length > 0 {
            // A closing delimiter must match the recorded opening delimiter.
            // Quotes can be part of the delimiter, so they cannot terminate it.
            for &character in &self.delimiter[..self.delimiter_length] {
                if lexer.lookahead() != character {
                    return false;
                }
                lexer.advance(false);
            }
            self.reset();
            return true;
        }

        loop {
            // Preserve C's order: reaching the limit fails even if the next
            // character is '('. Do not skip leading whitespace.
            if self.delimiter_length >= MAX_DELIMITER_LENGTH
                || lexer.eof()
                || lexer.lookahead() == i32::from(b'\\')
                // iswspace in C's default locale. Rust's
                // Unicode is_whitespace would reject additional delimiters.
                || matches!(lexer.lookahead(), 0x09..=0x0d | 0x20)
            {
                return false;
            }
            if lexer.lookahead() == i32::from(b'(') {
                // Empty delimiters are handled by a delimiter-less grammar rule.
                return self.delimiter_length > 0;
            }
            self.delimiter[self.delimiter_length] = lexer.lookahead();
            self.delimiter_length += 1;
            lexer.advance(false);
        }
    }

    /// Scan content, excluding the closing `)delimiter"` sequence.
    fn scan_raw_string_content(&self, lexer: &mut dyn Lexer) -> bool {
        // None is C's -1: no candidate closing delimiter is in progress.
        let mut delimiter_index = None;
        loop {
            // Even an incomplete raw string has content extending to EOF.
            if lexer.eof() {
                lexer.mark_end();
                return true;
            }

            if let Some(index) = delimiter_index {
                if index == self.delimiter_length {
                    if lexer.lookahead() == i32::from(b'"') {
                        return true;
                    }
                    delimiter_index = None;
                } else if lexer.lookahead() == self.delimiter[index] {
                    delimiter_index = Some(index + 1);
                } else {
                    delimiter_index = None;
                }
            }

            if delimiter_index.is_none() && lexer.lookahead() == i32::from(b')') {
                // Scan through the candidate terminator, but leave it out of
                // the content token. A later candidate can replace this mark.
                lexer.mark_end();
                delimiter_index = Some(0);
            }

            lexer.advance(false);
        }
    }
}

impl ExternalScanner for Scanner {
    fn scan(&mut self, lexer: &mut dyn Lexer, valid_symbols: &[bool]) -> bool {
        if valid_symbols[RAW_STRING_DELIMITER as usize]
            && valid_symbols[RAW_STRING_CONTENT as usize]
        {
            // Both symbols are enabled during error recovery.
            return false;
        }
        if valid_symbols[RAW_STRING_DELIMITER as usize] {
            lexer.set_result_symbol(RAW_STRING_DELIMITER);
            return self.scan_raw_string_delimiter(lexer);
        }
        if valid_symbols[RAW_STRING_CONTENT as usize] {
            lexer.set_result_symbol(RAW_STRING_CONTENT);
            return self.scan_raw_string_content(lexer);
        }
        false
    }

    fn serialize(&mut self, buffer: &mut [u8]) -> usize {
        let size = self.delimiter_length * WCHAR_SIZE;
        for (&character, bytes) in self.delimiter[..self.delimiter_length]
            .iter()
            .zip(buffer[..size].as_chunks_mut::<WCHAR_SIZE>().0)
        {
            bytes.copy_from_slice(&character.to_ne_bytes());
        }
        size
    }

    fn deserialize(&mut self, buffer: &[u8]) {
        assert!(
            buffer.len().is_multiple_of(WCHAR_SIZE),
            "Can't decode serialized delimiter!"
        );
        self.delimiter_length = buffer.len() / WCHAR_SIZE;
        for (character, bytes) in self.delimiter[..self.delimiter_length]
            .iter_mut()
            .zip(buffer.as_chunks::<WCHAR_SIZE>().0)
        {
            *character = i32::from_ne_bytes(*bytes);
        }
        // As in C, unused delimiter slots are unchanged, including on an empty
        // deserialize. Only delimiter_length determines which slots are live.
    }
}

/// Creates a scanner (C's `tree_sitter_cuda_external_scanner_create`).
pub(crate) fn create() -> Box<dyn ExternalScanner> {
    Box::new(Scanner::default())
}

#[cfg(test)]
mod tests {
    use super::*;

    const DELIMITER: &[bool] = &[true, false];
    const CONTENT: &[bool] = &[false, true];

    #[derive(Debug, PartialEq, Eq)]
    enum Event {
        Symbol(Symbol),
        Advance(usize),
        MarkEnd(usize),
    }

    struct TestLexer {
        input: Vec<i32>,
        position: usize,
        symbol: Symbol,
        events: Vec<Event>,
    }

    impl TestLexer {
        fn new(input: &str) -> Self {
            Self {
                input: input.chars().map(|c| c as i32).collect(),
                position: 0,
                symbol: Symbol::MAX,
                events: Vec::new(),
            }
        }

        fn marks(&self) -> Vec<usize> {
            self.events
                .iter()
                .filter_map(|event| match event {
                    Event::MarkEnd(position) => Some(*position),
                    _ => None,
                })
                .collect()
        }
    }

    impl Lexer for TestLexer {
        fn lookahead(&self) -> i32 {
            self.input.get(self.position).copied().unwrap_or(0)
        }

        fn result_symbol(&self) -> Symbol {
            self.symbol
        }

        fn set_result_symbol(&mut self, symbol: Symbol) {
            self.symbol = symbol;
            self.events.push(Event::Symbol(symbol));
        }

        fn advance(&mut self, skip: bool) {
            assert!(!skip, "raw strings never skip whitespace");
            assert!(!self.eof(), "scanner must not advance past EOF");
            self.events.push(Event::Advance(self.position));
            self.position += 1;
        }

        fn mark_end(&mut self) {
            self.events.push(Event::MarkEnd(self.position));
        }

        fn get_column(&mut self) -> u32 {
            panic!("the raw-string scanner must not query columns")
        }

        fn is_at_included_range_start(&self) -> bool {
            panic!("the raw-string scanner must not query included ranges")
        }

        fn eof(&self) -> bool {
            self.position == self.input.len()
        }
    }

    fn scanner_with_delimiter(delimiter: &str) -> Scanner {
        let mut scanner = Scanner::default();
        let mut lexer = TestLexer::new(&format!("{delimiter}("));
        assert!(scanner.scan(&mut lexer, DELIMITER));
        assert_eq!(lexer.lookahead(), i32::from(b'('));
        scanner
    }

    #[test]
    fn opening_and_closing_delimiters_preserve_lexer_calls() {
        let mut scanner = Scanner::default();
        let mut opening = TestLexer::new("a\"(");
        assert!(scanner.scan(&mut opening, DELIMITER));
        assert_eq!(opening.position, 2);
        assert_eq!(
            opening.events,
            [
                Event::Symbol(RAW_STRING_DELIMITER),
                Event::Advance(0),
                Event::Advance(1)
            ]
        );
        assert_eq!(scanner.delimiter_length, 2);
        assert_eq!(&scanner.delimiter[..2], &[i32::from(b'a'), i32::from(b'"')]);

        let mut mismatch = TestLexer::new("ab");
        assert!(!scanner.scan(&mut mismatch, DELIMITER));
        assert_eq!(mismatch.position, 1);
        assert_eq!(scanner.delimiter_length, 2);

        let mut closing = TestLexer::new("a\"not-a-quote");
        assert!(scanner.scan(&mut closing, DELIMITER));
        assert_eq!(closing.position, 2);
        assert_eq!(closing.events, opening.events);
        assert_eq!(scanner.delimiter_length, 0);
        assert_eq!(scanner.delimiter, [0; MAX_DELIMITER_LENGTH]);
    }

    #[test]
    fn opening_delimiter_rejections_and_limit() {
        for (input, consumed) in [
            ("(", 0),
            ("", 0),
            ("ab", 2),
            ("ab\\(", 2),
            (" (", 0),
            ("a\t(", 1),
            ("a\n(", 1),
            ("a\r(", 1),
            ("a\u{b}(", 1),
            ("a\u{c}(", 1),
            ("abcdefghijklmnop(", 16),
            ("abcdefghijklmnopq(", 16),
        ] {
            let mut scanner = Scanner::default();
            let mut lexer = TestLexer::new(input);
            assert!(!scanner.scan(&mut lexer, DELIMITER), "{input:?}");
            assert_eq!(lexer.position, consumed, "{input:?}");
            // Failed scans retain partial state; the runtime restores it.
            assert_eq!(scanner.delimiter_length, consumed, "{input:?}");
            assert!(lexer.marks().is_empty());
        }
        assert_eq!(
            scanner_with_delimiter("abcdefghijklmno").delimiter_length,
            15
        );
        // Despite the C comment, ')' is not rejected. Non-ASCII whitespace is
        // not iswspace in the C locale either. Preserve both behaviors.
        assert_eq!(scanner_with_delimiter(")\u{2003}😀").delimiter_length, 3);
    }

    #[test]
    fn content_retries_candidates_without_consuming_final_quote() {
        let mut scanner = scanner_with_delimiter("tag");
        let mut lexer = TestLexer::new("x)ta)tagX)tag\"tail");
        assert!(scanner.scan(&mut lexer, CONTENT));
        assert_eq!(lexer.symbol, RAW_STRING_CONTENT);
        assert_eq!(lexer.marks(), [1, 4, 9]);
        assert_eq!(lexer.position, 13);
        assert_eq!(lexer.lookahead(), i32::from(b'"'));
        assert_eq!(scanner.delimiter_length, 3);
    }

    #[test]
    fn content_supports_empty_and_quote_delimiters() {
        let mut scanner = Scanner::default();
        let mut lexer = TestLexer::new("a))\"rest");
        assert!(scanner.scan(&mut lexer, CONTENT));
        assert_eq!(lexer.marks(), [1, 2]);
        assert_eq!(lexer.position, 3);

        let mut scanner = scanner_with_delimiter("\"");
        let mut lexer = TestLexer::new("\"hello\")\"\"");
        assert!(scanner.scan(&mut lexer, CONTENT));
        assert_eq!(lexer.marks(), [7]);
        assert_eq!(lexer.position, 9);

        let mut lexer = TestLexer::new(")\"\"");
        assert!(scanner.scan(&mut lexer, CONTENT));
        assert_eq!(lexer.marks(), [0]);
        assert_eq!(lexer.position, 2);
    }

    #[test]
    fn eof_ends_content_even_after_a_partial_terminator() {
        for input in ["", "text", "text)", "text)t", "text)tag", "a\0b"] {
            let mut scanner = scanner_with_delimiter("tag");
            let mut lexer = TestLexer::new(input);
            assert!(scanner.scan(&mut lexer, CONTENT));
            assert!(lexer.eof());
            assert_eq!(lexer.marks().last(), Some(&input.chars().count()));
        }
    }

    #[test]
    fn recovery_and_disabled_symbols_leave_everything_untouched() {
        let mut scanner = scanner_with_delimiter("tag");
        for valid_symbols in [&[true, true], &[false, false]] {
            let mut lexer = TestLexer::new("tag(");
            assert!(!scanner.scan(&mut lexer, valid_symbols));
            assert!(lexer.events.is_empty());
            assert_eq!(lexer.position, 0);
            assert_eq!(scanner.delimiter_length, 3);
        }
    }

    #[test]
    fn serialization_is_native_wchar_bytes_with_no_length_prefix() {
        let mut scanner = scanner_with_delimiter("a😀");
        let mut buffer = [0xab; SERIALIZATION_BUFFER_SIZE];
        let length = scanner.serialize(&mut buffer);
        assert_eq!(length, 8);
        let expected: Vec<_> = [i32::from(b'a'), 0x1f600]
            .into_iter()
            .flat_map(i32::to_ne_bytes)
            .collect();
        assert_eq!(&buffer[..length], &expected);
        assert_eq!(buffer[length], 0xab);

        let mut restored = create();
        restored.deserialize(&buffer[..length]);
        let mut closing = TestLexer::new("a😀\"");
        assert!(restored.scan(&mut closing, DELIMITER));
        assert_eq!(closing.position, 2);
        assert_eq!(restored.serialize(&mut buffer), 0);

        scanner.deserialize(&[]);
        assert_eq!(scanner.delimiter_length, 0);
        assert_eq!(scanner.delimiter[0], i32::from(b'a'));
        assert_eq!(scanner.serialize(&mut buffer), 0);
        let mut opening = TestLexer::new("new(");
        assert!(scanner.scan(&mut opening, DELIMITER));
        assert_eq!(scanner.delimiter_length, 3);
    }

    #[test]
    #[should_panic(expected = "Can't decode serialized delimiter!")]
    fn malformed_serialization_is_rejected() {
        Scanner::default().deserialize(&[0; 3]);
    }
}
