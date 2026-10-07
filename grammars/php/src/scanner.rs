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

/// The scanner's state (C's `payload`). C's unused, unserialized
/// `has_leading_whitespace` flag has no bearing on token recognition.
#[derive(Default)]
pub(crate) struct Scanner {
    // Inactive slots retain their delimiter capacity across deserialize/pop.
    // Only the prefix ending at heredoc_count is semantic scanner state.
    heredoc_slots: Vec<Heredoc>,
    heredoc_count: usize,
}

// The reference runs in the default C locale: its wide-character predicates
// classify ASCII, not Unicode. In particular, iswspace includes vertical tab.
fn is_space(c: i32) -> bool {
    c == 0x20 || matches!(c, 0x09..=0x0d)
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

fn scan_whitespace(lexer: &mut dyn Lexer, mut c: i32) -> Option<i32> {
    loop {
        while is_space(c) {
            lexer.advance(false);
            c = lexer.lookahead();
        }

        if c != '/' as i32 {
            return Some(c);
        }
        lexer.advance(false);
        if lexer.lookahead() != '/' as i32 {
            return None;
        }
        lexer.advance(false);
        c = lexer.lookahead();
        // The C scanner tests lookahead, not eof, here.
        while c != 0 && c != '\n' as i32 {
            lexer.advance(false);
            c = lexer.lookahead();
        }
    }
}

// Tail-called from the small scanner dispatcher; do not pull the whitespace
// loop's saved registers into error-recovery and string dispatch.
#[inline(never)]
fn scan_automatic_semicolon(lexer: &mut dyn Lexer) -> bool {
    let c = lexer.lookahead();
    // With no whitespace/comment to consume, only '?' can begin a match.
    // A failed scan with no advancement needs no boundary mark: the runtime
    // finishes at this same position before trying the generated lexer.
    if !matches!(c, 0x09..=0x0d | 0x20 | 0x2f | 0x3f) {
        return false;
    }
    lexer.mark_end();
    let Some(c) = scan_whitespace(lexer, c) else {
        return false;
    };
    if c != '?' as i32 {
        return false;
    }
    lexer.set_result_symbol(AUTOMATIC_SEMICOLON as u16);
    lexer.advance(false);
    lexer.lookahead() == '>' as i32
}

fn is_escapable_sequence(lexer: &mut dyn Lexer, c: i32) -> bool {
    match c {
        // n r t v e f \ $ " u, and octal digits. The grammar handles
        // invalid Unicode escapes, including "\u{$a}".
        0x6e | 0x72 | 0x74 | 0x76 | 0x65 | 0x66 | 0x5c | 0x24 | 0x22 | 0x75 | 0x30..=0x37 => true,
        0x78 => {
            // Even an invalid hex escape consumes the 'x' during lookahead.
            lexer.advance(false);
            matches!(lexer.lookahead(), 0x30..=0x39 | 0x61..=0x66 | 0x41..=0x46)
        }
        _ => false,
    }
}

fn scan_heredoc_word(lexer: &mut dyn Lexer, word: &mut Vec<i32>) {
    let mut c = lexer.lookahead();
    while is_valid_name_char(c) {
        word.push(c);
        lexer.advance(false);
        c = lexer.lookahead();
    }
}

impl Scanner {
    // Keep string/heredoc dispatch, its loop state and allocation paths out of
    // the common semicolon-only scanner's stack frame and instruction stream.
    #[cold]
    #[inline(never)]
    fn scan_general(&mut self, lexer: &mut dyn Lexer, valid_symbols: &[bool]) -> bool {
        // Preserve the C dispatch order, including the after-variable forms
        // taking precedence over their corresponding ordinary content forms.
        for &(symbol, after_variable, heredoc, execution) in &[
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
                // The string loop marks every possible token boundary. Only
                // heredoc recognition can return before entering that loop.
                if heredoc {
                    lexer.mark_end();
                }
                lexer.set_result_symbol(symbol as u16);
                return self.scan_encapsed_part_string(lexer, after_variable, heredoc, execution);
            }
        }

        lexer.mark_end();
        if valid_symbols[NOWDOC_STRING] {
            lexer.set_result_symbol(NOWDOC_STRING as u16);
            return self.scan_nowdoc_string(lexer);
        }

        if valid_symbols[HEREDOC_END] {
            lexer.set_result_symbol(HEREDOC_END as u16);
            let Some(heredoc) = self.heredocs().last() else {
                return false;
            };
            while is_space(lexer.lookahead()) {
                lexer.advance(true);
            }
            // Compare in place instead of allocating a temporary closing
            // word. Still consume the complete name on mismatch, as C does.
            let mut matched = true;
            let mut length = 0;
            let mut c = lexer.lookahead();
            while is_valid_name_char(c) {
                matched &= heredoc.word.get(length) == Some(&c);
                length += 1;
                lexer.advance(false);
                c = lexer.lookahead();
            }
            if !matched || length != heredoc.word.len() {
                return false;
            }
            lexer.mark_end();
            self.heredoc_count -= 1;
            return true;
        }

        let c = lexer.lookahead();
        let Some(c) = scan_whitespace(lexer, c) else {
            return false;
        };

        if valid_symbols[EOF_TOKEN] && c == 0 && lexer.eof() {
            lexer.set_result_symbol(EOF_TOKEN as u16);
            return true;
        }

        if valid_symbols[HEREDOC_START] {
            lexer.set_result_symbol(HEREDOC_START as u16);
            while is_space(lexer.lookahead()) {
                lexer.advance(true);
            }
            let heredoc = self.push_heredoc();
            scan_heredoc_word(lexer, &mut heredoc.word);
            if heredoc.word.is_empty() {
                self.heredoc_count -= 1;
                return false;
            }
            lexer.mark_end();
            return true;
        }

        if valid_symbols[AUTOMATIC_SEMICOLON] {
            if c != '?' as i32 {
                return false;
            }
            // Failed scans do not publish a result symbol. Most calls stop
            // above, so avoid a dynamic setter for ordinary PHP tokens.
            lexer.set_result_symbol(AUTOMATIC_SEMICOLON as u16);
            lexer.advance(false);
            return lexer.lookahead() == '>' as i32;
        }

        false
    }

    #[cold]
    fn serialize_heredocs(&self, buffer: &mut [u8]) -> usize {
        let mut size = 1;
        for heredoc in self.heredocs() {
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

    #[cold]
    fn deserialize_heredocs(&mut self, buffer: &[u8], open_heredoc_count: u8) {
        let mut size = 1;
        for _ in 0..open_heredoc_count {
            let end_word_indentation_allowed = buffer[size] != 0;
            size += 1;
            let word_length =
                u32::from_ne_bytes(buffer[size..size + 4].try_into().unwrap()) as usize;
            size += 4;
            let word_size = word_length * size_of::<i32>();
            let heredoc = self.push_heredoc();
            heredoc.end_word_indentation_allowed = end_word_indentation_allowed;
            heredoc.word.extend(
                buffer[size..size + word_size]
                    .as_chunks::<4>()
                    .0
                    .iter()
                    .map(|bytes| i32::from_ne_bytes(*bytes)),
            );
            size += word_size;
        }
        assert_eq!(size, buffer.len());
    }

    fn heredocs(&self) -> &[Heredoc] {
        &self.heredoc_slots[..self.heredoc_count]
    }

    fn push_heredoc(&mut self) -> &mut Heredoc {
        if self.heredoc_count == self.heredoc_slots.len() {
            self.heredoc_slots.push(Heredoc::default());
        }
        let heredoc = &mut self.heredoc_slots[self.heredoc_count];
        self.heredoc_count += 1;
        heredoc.end_word_indentation_allowed = false;
        heredoc.word.clear();
        heredoc
    }

    fn scan_nowdoc_string(&self, lexer: &mut dyn Lexer) -> bool {
        let Some(heredoc) = self.heredocs().last() else {
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
        // Ordinary content cannot end the token. Defer C's per-character
        // mark_end until a possible boundary, before any lookahead advancement.
        // EOF has lookahead 0; an embedded NUL still needs the eof query.
        loop {
            let c = lexer.lookahead();
            match c {
                0x0a | 0x0d => {
                    lexer.mark_end();
                    return has_content;
                }
                _ => {
                    if c == 0 {
                        lexer.mark_end();
                        if lexer.eof() {
                            return false;
                        }
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

        if is_heredoc && let Some(heredoc) = self.heredocs().last() {
            // Indentation is allowed before a closing tag, but do not consume
            // line breaks when scanning heredoc content.
            while matches!(lexer.lookahead(), 0x09 | 0x0b | 0x0c | 0x20) {
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
        // Ordinary content cannot end the token. Defer C's per-character
        // mark_end until a possible boundary, before any lookahead advancement.
        // EOF has lookahead 0; an embedded NUL still needs the eof query.
        loop {
            let c = lexer.lookahead();
            match c {
                0x22 => {
                    if !is_heredoc && !is_execution_string {
                        lexer.mark_end();
                        return has_content;
                    }
                    lexer.advance(false);
                }
                0x60 => {
                    if is_execution_string {
                        lexer.mark_end();
                        return has_content;
                    }
                    lexer.advance(false);
                }
                0x0a | 0x0d => {
                    if is_heredoc {
                        lexer.mark_end();
                        return has_content;
                    }
                    lexer.advance(false);
                }
                0x5c => {
                    lexer.mark_end();
                    lexer.advance(false);
                    let c = lexer.lookahead();
                    if c == '{' as i32 {
                        // \{ is ordinary content; consume both characters so
                        // the brace cannot start an interpolation.
                        lexer.advance(false);
                    } else if is_execution_string && c == '`' as i32 {
                        return has_content;
                    } else if is_heredoc && c == '\\' as i32 {
                        lexer.advance(false);
                    } else if is_escapable_sequence(lexer, c) {
                        return has_content;
                    }
                }
                0x24 => {
                    lexer.mark_end();
                    lexer.advance(false);
                    let c = lexer.lookahead();
                    if (is_valid_name_char(c) && !is_digit(c)) || c == '{' as i32 {
                        return has_content;
                    }
                }
                0x2d if is_after_variable => {
                    lexer.mark_end();
                    lexer.advance(false);
                    if lexer.lookahead() == '>' as i32 {
                        lexer.advance(false);
                        if is_valid_name_char(lexer.lookahead()) {
                            return has_content;
                        }
                    }
                }
                // '-' falls through to '[' in C when not after a variable.
                0x2d | 0x5b => {
                    if is_after_variable {
                        lexer.mark_end();
                        return has_content;
                    }
                    lexer.advance(false);
                }
                0x7b => {
                    lexer.mark_end();
                    lexer.advance(false);
                    if lexer.lookahead() == '$' as i32 {
                        return has_content;
                    }
                }
                _ => {
                    if c == 0 {
                        lexer.mark_end();
                        if lexer.eof() {
                            return false;
                        }
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

        // Almost all non-string external lex states accept just the automatic
        // semicolon. Comparing the flag row as a whole avoids the full token
        // dispatch on every ordinary PHP token, without assuming that callers
        // cannot enable several tokens (whose C priority is handled below).
        if valid_symbols[..=SENTINEL_ERROR]
            == [
                true, false, false, false, false, false, false, false, false, false, false, false,
            ]
        {
            return scan_automatic_semicolon(lexer);
        }

        self.scan_general(lexer, valid_symbols)
    }

    fn serialize(&mut self, buffer: &mut [u8]) -> usize {
        buffer[0] = self.heredoc_count as u8;
        if self.heredoc_count == 0 {
            return 1;
        }
        self.serialize_heredocs(buffer)
    }

    fn deserialize(&mut self, buffer: &[u8]) {
        self.heredoc_count = 0;
        if let Some(&count) = buffer.first() {
            if count == 0 {
                assert_eq!(buffer.len(), 1);
            } else {
                self.deserialize_heredocs(buffer, count);
            }
        }
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
        scanner.heredoc_slots[1].end_word_indentation_allowed = true;
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

        let mut restored = Scanner::default();
        restored.deserialize(&buffer[..size]);
        assert_eq!(restored.heredoc_count, 2);
        assert!(restored.heredoc_slots[1].end_word_indentation_allowed);
        assert_eq!(restored.serialize(&mut buffer), size);
        assert_eq!(&buffer[..size], expected);
        restored.deserialize(&[]);
        assert!(restored.heredocs().is_empty());
        assert_eq!(restored.serialize(&mut buffer), 1);
        assert_eq!(buffer[0], 0);

        // Even an empty tag from a snapshot is preserved by the C deserializer.
        restored.deserialize(&[1, 2, 0, 0, 0, 0]);
        assert!(restored.heredoc_slots[0].word.is_empty());
        assert!(restored.heredoc_slots[0].end_word_indentation_allowed);
        assert_eq!(restored.serialize(&mut buffer), 6);
        assert_eq!(&buffer[..6], &[1, 1, 0, 0, 0, 0]);
    }

    #[test]
    fn heredoc_slots_reuse_words_across_restore_pop_and_reset() {
        let mut scanner = with_heredoc("OUTER");
        assert!(scan_one(&mut scanner, "é終", HEREDOC_START).0);
        let pointers = [
            scanner.heredoc_slots[0].word.as_ptr(),
            scanner.heredoc_slots[1].word.as_ptr(),
        ];
        let mut buffer = [0; SERIALIZATION_BUFFER_SIZE];
        let size = scanner.serialize(&mut buffer);

        for _ in 0..4 {
            scanner.deserialize(&buffer[..size]);
            assert_eq!(scanner.heredoc_count, 2);
            assert_eq!(scanner.heredoc_slots[0].word.as_ptr(), pointers[0]);
            assert_eq!(scanner.heredoc_slots[1].word.as_ptr(), pointers[1]);
            assert!(scan_one(&mut scanner, "é終;", HEREDOC_END).0);
            assert!(scan_one(&mut scanner, "OUTER;", HEREDOC_END).0);
            assert!(scanner.heredocs().is_empty());
            // The retained slots are storage only, not serialized state.
            let mut empty = [0xcc; SERIALIZATION_BUFFER_SIZE];
            assert_eq!(scanner.serialize(&mut empty), 1);
            assert_eq!(empty[0], 0);
            scanner.deserialize(&[]);
        }

        // An empty snapshot and a failed opener must not expose stale slots.
        assert!(!scan_one(&mut scanner, ";", HEREDOC_START).0);
        assert!(scanner.heredocs().is_empty());
        assert!(scan_one(&mut scanner, "NEW", HEREDOC_START).0);
        assert_eq!(scanner.heredoc_slots[0].word.as_ptr(), pointers[0]);
        assert_eq!(scanner.heredoc_slots[0].word, [78, 69, 87]);
        assert!(!scan_one(&mut scanner, "OUTER;", HEREDOC_END).0);
        assert!(scan_one(&mut scanner, "NEW;", HEREDOC_END).0);
    }

    #[test]
    fn closing_tag_mismatch_still_consumes_the_whole_name() {
        let mut scanner = with_heredoc("END");
        for word in ["EN", "ENDMORE", "ENDé", "OTHER"] {
            let (result, lexer) = scan_one(&mut scanner, &format!("{word};"), HEREDOC_END);
            assert!(!result, "{word}");
            assert_eq!(lexer.position, word.chars().count());
            assert_eq!(lexer.end, 0);
            assert_eq!(scanner.heredoc_count, 1);
        }
        assert!(scan_one(&mut scanner, "END;", HEREDOC_END).0);
    }

    #[test]
    fn ordinary_string_characters_need_no_intermediate_marks_or_eof_queries() {
        let (result, lexer) = scan_one(&mut Scanner::default(), "abc$x", ENCAPSED_STRING_CHARS);
        assert!(result);
        assert_eq!((lexer.end, lexer.position), (3, 4));
        assert_eq!(
            lexer.events.into_inner(),
            vec![
                Event::Symbol(ENCAPSED_STRING_CHARS as u16),
                Event::Advance(0, false),
                Event::Advance(1, false),
                Event::Advance(2, false),
                Event::MarkEnd(3),
                Event::Advance(3, false),
            ]
        );

        let (result, lexer) = scan_one(&mut with_heredoc("END"), "abc\n", NOWDOC_STRING);
        assert!(result);
        assert_eq!((lexer.end, lexer.position), (3, 3));
        assert_eq!(
            lexer.events.into_inner(),
            vec![
                Event::MarkEnd(0),
                Event::Symbol(NOWDOC_STRING as u16),
                Event::Advance(0, false),
                Event::Advance(1, false),
                Event::Advance(2, false),
                Event::MarkEnd(3),
            ]
        );
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
        scanner.heredoc_slots[2].word.pop();
        assert_eq!(scanner.serialize(&mut buffer), 1020);

        scanner.heredoc_count -= 1;
        scanner.heredoc_slots[1].word = vec!['B' as i32; 252];
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
        assert_eq!(scanner.heredoc_count, 2);
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
        assert!(scanner.heredocs().is_empty());
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
                Event::Symbol(ENCAPSED_STRING_CHARS as u16),
                Event::MarkEnd(0),
                Event::Advance(0, false),
                Event::Advance(1, false),
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
            assert_eq!(scanner.heredoc_count, 1);
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
            assert_eq!(scanner.heredoc_count, 1);
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
    fn semicolon_fast_path_preserves_general_scanner_lookahead() {
        let mut fast = with_heredoc("OUTER");
        let mut general = with_heredoc("OUTER");
        let mut valid = [false; SENTINEL_ERROR + 1];
        valid[AUTOMATIC_SEMICOLON] = true;
        let mut inputs = (0..=127)
            .map(|c| format!("{}?>", char::from_u32(c).unwrap()))
            .collect::<Vec<_>>();
        inputs.extend([
            "".into(),
            "  \r\n// comment\n?>".into(),
            "/* comment */?>".into(),
            "// comment\0?>".into(),
            "// comment".into(),
            "é?>".into(),
        ]);
        for input in inputs {
            let mut fast_lexer = TestLexer::new(&input);
            let mut general_lexer = TestLexer::new(&input);
            let fast_result = fast.scan(&mut fast_lexer, &valid);
            let general_result = general.scan_general(&mut general_lexer, &valid);
            assert_eq!(fast_result, general_result, "{input:?}");
            assert_eq!(fast_lexer.position, general_lexer.position, "{input:?}");
            if fast_result {
                assert_eq!(fast_lexer.end, general_lexer.end, "{input:?}");
                assert_eq!(fast_lexer.symbol, general_lexer.symbol, "{input:?}");
            }
            assert_eq!(fast.heredoc_count, 1);
            assert_eq!(general.heredoc_count, 1);
        }
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
        let mut lexer = TestLexer::new("a");
        let mut valid = [true; SENTINEL_ERROR + 1];
        assert!(!scanner.scan(&mut lexer, &valid));
        assert!(lexer.events.borrow().is_empty());
        assert_eq!(scanner.heredoc_count, 1);

        valid[SENTINEL_ERROR] = false;
        assert!(!scanner.scan(&mut lexer, &valid));
        assert_eq!(lexer.symbol, ENCAPSED_STRING_CHARS_AFTER_VARIABLE as u16);

        // EOF wins over the heredoc start when both are enabled.
        let mut lexer = TestLexer::new("");
        valid.fill(false);
        valid[EOF_TOKEN] = true;
        valid[HEREDOC_START] = true;
        assert!(scanner.scan(&mut lexer, &valid));
        assert_eq!(lexer.symbol, EOF_TOKEN as u16);
        assert_eq!(scanner.heredoc_count, 1);
    }
}
