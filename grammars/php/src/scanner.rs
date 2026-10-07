//! The PHP external scanner, translated from `php/common/scanner.h`.
//! Shared by the PHP and PHP-only grammars.

use ts_port_tables::{ExternalScanner, Lexer, SERIALIZATION_BUFFER_SIZE};

// External token indices, in the order of the C TokenType enum.
const AUTOMATIC_SEMICOLON: usize = 0;
const ENCAPSED_STRING_CHARS: usize = 1;
const ENCAPSED_STRING_CHARS_AFTER_VARIABLE: usize = 2;
const EXECUTION_STRING_CHARS: usize = 3;
const EXECUTION_STRING_CHARS_AFTER_VARIABLE: usize = 4;
const ENCAPSED_STRING_CHARS_HEREDOC: usize = 5;
const ENCAPSED_STRING_CHARS_AFTER_VARIABLE_HEREDOC: usize = 6;
const EOF_TOKEN: usize = 7;
const HEREDOC_START: usize = 8;
const HEREDOC_END: usize = 9;
const NOWDOC_STRING: usize = 10;
const SENTINEL_ERROR: usize = 11;

#[derive(Default)]
struct Heredoc {
    end_word_indentation_allowed: bool,
    word: Vec<i32>,
}

/// The scanner's state (C's `payload`).
#[derive(Default)]
pub(crate) struct Scanner {
    has_leading_whitespace: bool,
    heredocs: Vec<Heredoc>,
}

// The reference runs in the default C locale: its wide-character predicates
// classify ASCII, not Unicode. In particular, iswspace includes vertical tab.
fn is_space(c: i32) -> bool {
    matches!(c, 0x09..=0x0d | 0x20)
}

fn is_digit(c: i32) -> bool {
    (b'0' as i32..=b'9' as i32).contains(&c)
}

fn is_valid_name_char(c: i32) -> bool {
    is_digit(c)
        || (b'a' as i32..=b'z' as i32).contains(&c)
        || (b'A' as i32..=b'Z' as i32).contains(&c)
        || c == '_' as i32
        || c >= 0x80
}

fn scan_whitespace(lexer: &mut dyn Lexer) -> bool {
    loop {
        while is_space(lexer.lookahead()) {
            lexer.advance(false);
        }

        if lexer.lookahead() == '/' as i32 {
            lexer.advance(false);
            if lexer.lookahead() == '/' as i32 {
                lexer.advance(false);
                // The C scanner tests lookahead, not eof, here.
                while lexer.lookahead() != 0 && lexer.lookahead() != '\n' as i32 {
                    lexer.advance(false);
                }
            } else {
                return false;
            }
        } else {
            return true;
        }
    }
}

fn is_escapable_sequence(lexer: &mut dyn Lexer) -> bool {
    match char::from_u32(lexer.lookahead() as u32) {
        Some('n' | 'r' | 't' | 'v' | 'e' | 'f' | '\\' | '$' | '"') => true,
        Some('x') => {
            // Even an invalid hex escape consumes the 'x' during lookahead.
            lexer.advance(false);
            let c = lexer.lookahead();
            is_digit(c)
                || (b'a' as i32..=b'f' as i32).contains(&c)
                || (b'A' as i32..=b'F' as i32).contains(&c)
        }
        // The grammar handles invalid Unicode escapes, including "\u{$a}".
        Some('u') => true,
        Some('0'..='7') => true,
        _ => false,
    }
}

fn scan_heredoc_word(lexer: &mut dyn Lexer) -> Vec<i32> {
    let mut result = Vec::new();
    while is_valid_name_char(lexer.lookahead()) {
        result.push(lexer.lookahead());
        lexer.advance(false);
    }
    result
}

impl Scanner {
    fn scan_nowdoc_string(&self, lexer: &mut dyn Lexer) -> bool {
        let Some(heredoc) = self.heredocs.last() else {
            return false;
        };
        let mut has_consumed_content = false;

        // Unlike heredoc content, nowdoc content can consume line breaks before
        // checking for a closing tag.
        while is_space(lexer.lookahead()) {
            lexer.advance(false);
            has_consumed_content = true;
        }

        let mut end_tag_matched = false;
        for (i, &c) in heredoc.word.iter().enumerate() {
            if lexer.lookahead() != c {
                break;
            }
            lexer.advance(false);
            has_consumed_content = true;
            end_tag_matched = i == heredoc.word.len() - 1 && !is_valid_name_char(lexer.lookahead());
        }
        if end_tag_matched {
            return false;
        }

        let mut has_content = has_consumed_content;
        loop {
            lexer.mark_end();
            match char::from_u32(lexer.lookahead() as u32) {
                Some('\n' | '\r') => return has_content,
                _ => {
                    if lexer.eof() {
                        return false;
                    }
                    lexer.advance(false);
                }
            }
            has_content = true;
        }
    }

    fn scan_encapsed_part_string(
        &self,
        lexer: &mut dyn Lexer,
        mut is_after_variable: bool,
        is_heredoc: bool,
        is_execution_string: bool,
    ) -> bool {
        let mut has_consumed_content = false;

        if is_heredoc && let Some(heredoc) = self.heredocs.last() {
            // Indentation is allowed before a closing tag, but do not consume
            // line breaks when scanning heredoc content.
            while is_space(lexer.lookahead())
                && lexer.lookahead() != '\r' as i32
                && lexer.lookahead() != '\n' as i32
            {
                lexer.advance(false);
                has_consumed_content = true;
            }

            let mut end_tag_matched = false;
            for (i, &c) in heredoc.word.iter().enumerate() {
                if lexer.lookahead() != c {
                    break;
                }
                has_consumed_content = true;
                lexer.advance(false);
                end_tag_matched =
                    i == heredoc.word.len() - 1 && !is_valid_name_char(lexer.lookahead());
            }
            if end_tag_matched {
                return false;
            }
        }

        let mut has_content = has_consumed_content;
        loop {
            lexer.mark_end();
            match char::from_u32(lexer.lookahead() as u32) {
                Some('"') => {
                    if !is_heredoc && !is_execution_string {
                        return has_content;
                    }
                    lexer.advance(false);
                }
                Some('`') => {
                    if is_execution_string {
                        return has_content;
                    }
                    lexer.advance(false);
                }
                Some('\n' | '\r') => {
                    if is_heredoc {
                        return has_content;
                    }
                    lexer.advance(false);
                }
                Some('\\') => {
                    lexer.advance(false);
                    if lexer.lookahead() == '{' as i32 {
                        // \{ is ordinary content; consume both characters so
                        // the brace cannot start an interpolation.
                        lexer.advance(false);
                    } else if is_execution_string && lexer.lookahead() == '`' as i32 {
                        return has_content;
                    } else if is_heredoc && lexer.lookahead() == '\\' as i32 {
                        lexer.advance(false);
                    } else if is_escapable_sequence(lexer) {
                        return has_content;
                    }
                }
                Some('$') => {
                    lexer.advance(false);
                    if (is_valid_name_char(lexer.lookahead()) && !is_digit(lexer.lookahead()))
                        || lexer.lookahead() == '{' as i32
                    {
                        return has_content;
                    }
                }
                Some('-') if is_after_variable => {
                    lexer.advance(false);
                    if lexer.lookahead() == '>' as i32 {
                        lexer.advance(false);
                        if is_valid_name_char(lexer.lookahead()) {
                            return has_content;
                        }
                    }
                }
                // '-' falls through to '[' in C when not after a variable.
                Some('-' | '[') => {
                    if is_after_variable {
                        return has_content;
                    }
                    lexer.advance(false);
                }
                Some('{') => {
                    lexer.advance(false);
                    if lexer.lookahead() == '$' as i32 {
                        return has_content;
                    }
                }
                _ => {
                    if lexer.eof() {
                        return false;
                    }
                    lexer.advance(false);
                }
            }
            is_after_variable = false;
            has_content = true;
        }
    }
}

impl ExternalScanner for Scanner {
    fn scan(&mut self, lexer: &mut dyn Lexer, valid_symbols: &[bool]) -> bool {
        if valid_symbols[SENTINEL_ERROR] {
            return false;
        }

        self.has_leading_whitespace = false;
        lexer.mark_end();

        // Preserve the C dispatch order, including the after-variable forms
        // taking precedence over their corresponding ordinary content forms.
        for (symbol, after_variable, heredoc, execution) in [
            (ENCAPSED_STRING_CHARS_AFTER_VARIABLE, true, false, false),
            (ENCAPSED_STRING_CHARS, false, false, false),
            (EXECUTION_STRING_CHARS_AFTER_VARIABLE, true, false, true),
            (EXECUTION_STRING_CHARS, false, false, true),
            (
                ENCAPSED_STRING_CHARS_AFTER_VARIABLE_HEREDOC,
                true,
                true,
                false,
            ),
            (ENCAPSED_STRING_CHARS_HEREDOC, false, true, false),
        ] {
            if valid_symbols[symbol] {
                lexer.set_result_symbol(symbol as u16);
                return self.scan_encapsed_part_string(lexer, after_variable, heredoc, execution);
            }
        }

        if valid_symbols[NOWDOC_STRING] {
            lexer.set_result_symbol(NOWDOC_STRING as u16);
            return self.scan_nowdoc_string(lexer);
        }

        if valid_symbols[HEREDOC_END] {
            lexer.set_result_symbol(HEREDOC_END as u16);
            let Some(heredoc) = self.heredocs.last() else {
                return false;
            };
            while is_space(lexer.lookahead()) {
                lexer.advance(true);
            }
            let word = scan_heredoc_word(lexer);
            if word != heredoc.word {
                return false;
            }
            lexer.mark_end();
            self.heredocs.pop();
            return true;
        }

        if !scan_whitespace(lexer) {
            return false;
        }

        if valid_symbols[EOF_TOKEN] && lexer.eof() {
            lexer.set_result_symbol(EOF_TOKEN as u16);
            return true;
        }

        if valid_symbols[HEREDOC_START] {
            lexer.set_result_symbol(HEREDOC_START as u16);
            while is_space(lexer.lookahead()) {
                lexer.advance(true);
            }
            let word = scan_heredoc_word(lexer);
            if word.is_empty() {
                return false;
            }
            lexer.mark_end();
            self.heredocs.push(Heredoc {
                word,
                ..Heredoc::default()
            });
            return true;
        }

        if valid_symbols[AUTOMATIC_SEMICOLON] {
            lexer.set_result_symbol(AUTOMATIC_SEMICOLON as u16);
            if lexer.lookahead() != '?' as i32 {
                return false;
            }
            lexer.advance(false);
            return lexer.lookahead() == '>' as i32;
        }

        false
    }

    fn serialize(&mut self, buffer: &mut [u8]) -> usize {
        let mut size = 0;
        buffer[size] = self.heredocs.len() as u8;
        size += 1;
        for heredoc in &self.heredocs {
            let word_size = heredoc.word.len() * size_of::<i32>();
            // C deliberately rejects a state that would exactly fill the buffer,
            // and leaves any already-written prefix intact on failure.
            if size + 5 + word_size >= SERIALIZATION_BUFFER_SIZE {
                return 0;
            }
            buffer[size] = u8::from(heredoc.end_word_indentation_allowed);
            size += 1;
            buffer[size..size + 4].copy_from_slice(&(heredoc.word.len() as u32).to_ne_bytes());
            size += 4;
            for &c in &heredoc.word {
                buffer[size..size + 4].copy_from_slice(&c.to_ne_bytes());
                size += 4;
            }
        }
        size
    }

    fn deserialize(&mut self, buffer: &[u8]) {
        self.has_leading_whitespace = false;
        self.heredocs.clear();
        if buffer.is_empty() {
            return;
        }

        let open_heredoc_count = buffer[0];
        let mut size = 1;
        for _ in 0..open_heredoc_count {
            let end_word_indentation_allowed = buffer[size] != 0;
            size += 1;
            let word_length =
                u32::from_ne_bytes(buffer[size..size + 4].try_into().unwrap()) as usize;
            size += 4;
            let word_size = word_length * size_of::<i32>();
            let word = buffer[size..size + word_size]
                .as_chunks::<4>()
                .0
                .iter()
                .map(|bytes| i32::from_ne_bytes(*bytes))
                .collect();
            size += word_size;
            self.heredocs.push(Heredoc {
                end_word_indentation_allowed,
                word,
            });
        }
        assert_eq!(size, buffer.len());
    }
}

/// Creates a scanner (C's `tree_sitter_php_external_scanner_create`).
pub(crate) fn create() -> Box<dyn ExternalScanner> {
    Box::new(Scanner::default())
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use super::*;
    use ts_port_tables::Symbol;

    #[derive(Debug, PartialEq, Eq)]
    enum Event {
        Advance(usize, bool),
        MarkEnd(usize),
        Symbol(Symbol),
        Eof(usize),
    }

    struct TestLexer {
        input: Vec<i32>,
        position: usize,
        end: usize,
        symbol: Symbol,
        events: RefCell<Vec<Event>>,
    }

    impl TestLexer {
        fn new(input: &str) -> Self {
            Self {
                input: input.chars().map(|c| c as i32).collect(),
                position: 0,
                end: 0,
                symbol: Symbol::MAX,
                events: RefCell::new(Vec::new()),
            }
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
            self.events.get_mut().push(Event::Symbol(symbol));
        }

        fn advance(&mut self, skip: bool) {
            self.events
                .get_mut()
                .push(Event::Advance(self.position, skip));
            assert!(self.position < self.input.len());
            self.position += 1;
        }

        fn mark_end(&mut self) {
            self.end = self.position;
            self.events.get_mut().push(Event::MarkEnd(self.position));
        }

        fn get_column(&mut self) -> u32 {
            panic!("the PHP scanner does not request columns")
        }

        fn is_at_included_range_start(&self) -> bool {
            panic!("the PHP scanner does not query included ranges")
        }

        fn eof(&self) -> bool {
            self.events.borrow_mut().push(Event::Eof(self.position));
            self.position == self.input.len()
        }
    }

    fn scan_one(scanner: &mut Scanner, text: &str, symbol: usize) -> (bool, TestLexer) {
        let mut lexer = TestLexer::new(text);
        let mut valid = [false; SENTINEL_ERROR + 1];
        valid[symbol] = true;
        let result = scanner.scan(&mut lexer, &valid);
        (result, lexer)
    }

    fn with_heredoc(tag: &str) -> Scanner {
        let mut scanner = Scanner::default();
        assert!(scan_one(&mut scanner, tag, HEREDOC_START).0);
        scanner
    }

    #[test]
    fn serializes_native_endian_codepoints_and_resets_state() {
        let mut scanner = with_heredoc("OUTER");
        assert!(scan_one(&mut scanner, "é終", HEREDOC_START).0);
        scanner.heredocs[1].end_word_indentation_allowed = true;
        scanner.has_leading_whitespace = true;
        let mut expected = vec![2];
        for (flag, tag) in [(0, "OUTER"), (1, "é終")] {
            expected.push(flag);
            expected.extend_from_slice(&(tag.chars().count() as u32).to_ne_bytes());
            for c in tag.chars() {
                expected.extend_from_slice(&(c as i32).to_ne_bytes());
            }
        }
        let mut buffer = [0xcc; SERIALIZATION_BUFFER_SIZE];
        let size = scanner.serialize(&mut buffer);
        assert_eq!(&buffer[..size], expected);
        assert!(scanner.has_leading_whitespace);

        let mut restored = Scanner::default();
        restored.deserialize(&buffer[..size]);
        assert!(!restored.has_leading_whitespace);
        assert_eq!(restored.heredocs.len(), 2);
        assert!(restored.heredocs[1].end_word_indentation_allowed);
        assert_eq!(restored.serialize(&mut buffer), size);
        assert_eq!(&buffer[..size], expected);
        restored.deserialize(&[]);
        assert!(restored.heredocs.is_empty());
        assert_eq!(restored.serialize(&mut buffer), 1);
        assert_eq!(buffer[0], 0);

        // Even an empty tag from a snapshot is preserved by the C deserializer.
        restored.deserialize(&[1, 2, 0, 0, 0, 0]);
        assert!(restored.heredocs[0].word.is_empty());
        assert!(restored.heredocs[0].end_word_indentation_allowed);
        assert_eq!(restored.serialize(&mut buffer), 6);
        assert_eq!(&buffer[..6], &[1, 1, 0, 0, 0, 0]);
    }

    #[test]
    fn serialization_rejects_exactly_full_buffer_without_erasing_prefix() {
        let mut scanner = with_heredoc("A");
        assert!(scan_one(&mut scanner, "B", HEREDOC_START).0);
        assert!(scan_one(&mut scanner, &"C".repeat(250), HEREDOC_START).0);
        let mut buffer = [0xcc; SERIALIZATION_BUFFER_SIZE];
        // 1 + 3 * 5 + (1 + 1 + 250) * 4 == 1024 is rejected, not accepted.
        assert_eq!(scanner.serialize(&mut buffer), 0);
        assert_eq!(buffer[0], 3);
        assert_eq!(buffer[19], 0xcc);
        scanner.heredocs[2].word.pop();
        assert_eq!(scanner.serialize(&mut buffer), 1020);

        scanner.heredocs.pop();
        scanner.heredocs[1].word = vec!['B' as i32; 252];
        assert_eq!(scanner.serialize(&mut buffer), 1023);
    }

    #[test]
    fn heredoc_stack_and_whitespace_call_order() {
        let mut scanner = with_heredoc("OUTER");
        let (result, lexer) = scan_one(&mut scanner, " \nI;", HEREDOC_START);
        assert!(result);
        assert_eq!(
            lexer.events.into_inner(),
            vec![
                Event::MarkEnd(0),
                Event::Advance(0, false),
                Event::Advance(1, false),
                Event::Symbol(HEREDOC_START as u16),
                Event::Advance(2, false),
                Event::MarkEnd(3),
            ]
        );
        assert!(!scan_one(&mut scanner, "OUTER;", HEREDOC_END).0);
        assert_eq!(scanner.heredocs.len(), 2);
        let (result, lexer) = scan_one(&mut scanner, "\n\tI;", HEREDOC_END);
        assert!(result);
        assert_eq!(
            lexer.events.into_inner(),
            vec![
                Event::MarkEnd(0),
                Event::Symbol(HEREDOC_END as u16),
                Event::Advance(0, true),
                Event::Advance(1, true),
                Event::Advance(2, false),
                Event::MarkEnd(3),
            ]
        );
        assert!(scan_one(&mut scanner, "OUTER;", HEREDOC_END).0);
        assert!(scanner.heredocs.is_empty());
        assert!(!scan_one(&mut scanner, "OUTER;", HEREDOC_END).0);
        assert!(!scan_one(&mut scanner, ";", HEREDOC_START).0);
    }

    #[test]
    fn strings_stop_before_interpolations_and_escapes() {
        for (text, accepted, end, position) in [
            ("abc\"", true, 3, 3),
            ("\"", false, 0, 0),
            ("abc", false, 3, 3),
            ("a\0b\"", true, 3, 3),
            ("a\n`\"", true, 3, 3),
            ("$a", false, 0, 1),
            ("x$a", true, 1, 2),
            ("x$é", true, 1, 2),
            ("x${", true, 1, 2),
            ("x$1\"", true, 3, 3),
            ("x{$a}", true, 1, 2),
            ("x\\n", true, 1, 2),
            ("x\\x0", true, 1, 3),
            ("x\\xz\"", true, 4, 4),
            ("x\\u{$a}", true, 1, 2),
            ("x\\7", true, 1, 2),
            ("x\\8\"", true, 3, 3),
            ("x\\\\", true, 1, 2),
            ("x\\{\"", true, 3, 3),
        ] {
            let (result, lexer) = scan_one(&mut Scanner::default(), text, ENCAPSED_STRING_CHARS);
            assert_eq!(
                (result, lexer.end, lexer.position),
                (accepted, end, position),
                "{text:?}"
            );
        }
    }

    #[test]
    fn invalid_hex_escape_still_advances_over_x() {
        let (_, lexer) = scan_one(&mut Scanner::default(), "\\xz\"", ENCAPSED_STRING_CHARS);
        assert_eq!(
            lexer.events.into_inner(),
            vec![
                Event::MarkEnd(0),
                Event::Symbol(ENCAPSED_STRING_CHARS as u16),
                Event::MarkEnd(0),
                Event::Advance(0, false),
                Event::Advance(1, false),
                Event::MarkEnd(2),
                Event::Eof(2),
                Event::Advance(2, false),
                Event::MarkEnd(3),
            ]
        );
    }

    #[test]
    fn execution_strings_use_backtick_delimiters() {
        for (text, accepted, end, position) in [
            ("`", false, 0, 0),
            ("a\"`", true, 2, 2),
            ("a\\`", true, 1, 2),
            ("a\\\"", true, 1, 2),
            ("a\n`", true, 2, 2),
            ("a\\\\", true, 1, 2),
        ] {
            let (result, lexer) = scan_one(&mut Scanner::default(), text, EXECUTION_STRING_CHARS);
            assert_eq!(
                (result, lexer.end, lexer.position),
                (accepted, end, position),
                "{text:?}"
            );
        }
    }

    #[test]
    fn after_variable_checks_apply_only_at_start() {
        for symbol in [
            ENCAPSED_STRING_CHARS_AFTER_VARIABLE,
            EXECUTION_STRING_CHARS_AFTER_VARIABLE,
            ENCAPSED_STRING_CHARS_AFTER_VARIABLE_HEREDOC,
        ] {
            let terminator = match symbol {
                EXECUTION_STRING_CHARS_AFTER_VARIABLE => '`',
                ENCAPSED_STRING_CHARS_AFTER_VARIABLE_HEREDOC => '\n',
                _ => '"',
            };
            for (text, accepted, end, position) in [
                ("[0]", false, 0, 0),
                ("->foo", false, 0, 2),
                ("->1", false, 0, 2),
                ("->é", false, 0, 2),
                ("-", true, 1, 1),
                ("->", true, 2, 2),
                ("x->a[0]", true, 7, 7),
                ("-[0]", true, 4, 4),
            ] {
                let (result, lexer) = scan_one(
                    &mut Scanner::default(),
                    &format!("{text}{terminator}"),
                    symbol,
                );
                assert_eq!(
                    (result, lexer.end, lexer.position),
                    (accepted, end, position),
                    "{text:?}, {symbol}"
                );
            }
        }
    }

    #[test]
    fn heredoc_tag_boundaries_and_special_content() {
        let mut scanner = with_heredoc("END");
        for (text, accepted, end, position) in [
            (" \tEND;", false, 0, 5),
            ("END_MORE\n", true, 8, 8),
            ("ENDé\n", true, 4, 4),
            ("EN\n", true, 2, 2),
            ("\nEND", false, 0, 0),
            ("  \nEND", true, 2, 2),
            ("a\"`\n", true, 3, 3),
            ("a\\\\\n", true, 3, 3),
            ("a\\n", true, 1, 2),
            ("a{$x}", true, 1, 2),
            ("a", false, 1, 1),
        ] {
            let (result, lexer) = scan_one(&mut scanner, text, ENCAPSED_STRING_CHARS_HEREDOC);
            assert_eq!(
                (result, lexer.end, lexer.position),
                (accepted, end, position),
                "{text:?}"
            );
            assert_eq!(scanner.heredocs.len(), 1);
        }
    }

    #[test]
    fn nowdoc_skips_newlines_before_testing_tag_and_never_interpolates() {
        let mut scanner = with_heredoc("END");
        for (text, accepted, end, position) in [
            ("\n \r\nEND;", false, 0, 7),
            ("END_MORE\n", true, 8, 8),
            ("\nEN\n", true, 3, 3),
            ("$a\\n\"`\n", true, 6, 6),
            ("abc", false, 3, 3),
            ("\n\n", false, 2, 2),
        ] {
            let (result, lexer) = scan_one(&mut scanner, text, NOWDOC_STRING);
            assert_eq!(
                (result, lexer.end, lexer.position),
                (accepted, end, position),
                "{text:?}"
            );
            assert_eq!(scanner.heredocs.len(), 1);
        }
        assert!(!scan_one(&mut Scanner::default(), "text\n", NOWDOC_STRING).0);
    }

    #[test]
    fn automatic_semicolon_is_zero_width_and_skips_only_line_comments() {
        let (result, lexer) = scan_one(&mut Scanner::default(), " \n//x\n?>", AUTOMATIC_SEMICOLON);
        assert!(result);
        assert_eq!((lexer.end, lexer.position), (0, 7));
        assert_eq!(
            lexer.events.into_inner(),
            vec![
                Event::MarkEnd(0),
                Event::Advance(0, false),
                Event::Advance(1, false),
                Event::Advance(2, false),
                Event::Advance(3, false),
                Event::Advance(4, false),
                Event::Advance(5, false),
                Event::Symbol(AUTOMATIC_SEMICOLON as u16),
                Event::Advance(6, false),
            ]
        );
        for text in ["", "?", "??>", "/*x*/?>", "//?>", "\u{a0}?>"] {
            assert!(
                !scan_one(&mut Scanner::default(), text, AUTOMATIC_SEMICOLON).0,
                "{text:?}"
            );
        }
        assert!(scan_one(&mut Scanner::default(), "\u{b}?>", AUTOMATIC_SEMICOLON).0);
    }

    #[test]
    fn eof_after_comments_keeps_original_mark() {
        let (result, lexer) = scan_one(&mut Scanner::default(), "//x", EOF_TOKEN);
        assert!(result);
        assert_eq!((lexer.end, lexer.position), (0, 3));
        assert_eq!(
            lexer.events.into_inner(),
            vec![
                Event::MarkEnd(0),
                Event::Advance(0, false),
                Event::Advance(1, false),
                Event::Advance(2, false),
                Event::Eof(3),
                Event::Symbol(EOF_TOKEN as u16),
            ]
        );
        assert!(!scan_one(&mut Scanner::default(), "\0", EOF_TOKEN).0);
    }

    #[test]
    fn recovery_and_dispatch_precedence() {
        let mut scanner = with_heredoc("END");
        scanner.has_leading_whitespace = true;
        let mut lexer = TestLexer::new("a");
        let mut valid = [true; SENTINEL_ERROR + 1];
        assert!(!scanner.scan(&mut lexer, &valid));
        assert!(lexer.events.borrow().is_empty());
        assert!(scanner.has_leading_whitespace);
        assert_eq!(scanner.heredocs.len(), 1);

        valid[SENTINEL_ERROR] = false;
        assert!(!scanner.scan(&mut lexer, &valid));
        assert_eq!(lexer.symbol, ENCAPSED_STRING_CHARS_AFTER_VARIABLE as u16);
        assert!(!scanner.has_leading_whitespace);

        // EOF wins over the heredoc start when both are enabled.
        let mut lexer = TestLexer::new("");
        valid.fill(false);
        valid[EOF_TOKEN] = true;
        valid[HEREDOC_START] = true;
        assert!(scanner.scan(&mut lexer, &valid));
        assert_eq!(lexer.symbol, EOF_TOKEN as u16);
        assert_eq!(scanner.heredocs.len(), 1);
    }
}
