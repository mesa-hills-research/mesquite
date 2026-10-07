//! OCaml's shared external scanner, translated from `common/scanner.h`.

use ts_port_tables::{ExternalScanner, Lexer, SERIALIZATION_BUFFER_SIZE};

const COMMENT: usize = 0;
const LEFT_QUOTED_STRING_DELIM: usize = 1;
const RIGHT_QUOTED_STRING_DELIM: usize = 2;
const STRING_DELIM: usize = 3;
const LINE_NUMBER_DIRECTIVE: usize = 4;
const NULL_CHARACTER: usize = 5;
const ERROR_SENTINEL: usize = 6;

/// The logical delimiter length is separate from its retained allocation: C's
/// deserialize overwrites only `length` BYTES of the int32_t array, then sets its
/// CODE POINT count to `length`. The remaining bytes must survive even a clear.
#[derive(Default)]
pub(crate) struct Scanner {
    in_string: bool,
    quoted_string_id_length: usize,
    quoted_string_id: Vec<i32>,
}

impl Scanner {
    fn clear_id(&mut self) {
        self.quoted_string_id_length = 0;
    }

    fn resize_id(&mut self, min_capacity: usize) {
        if self.quoted_string_id.len() >= min_capacity {
            return;
        }
        let mut capacity = self.quoted_string_id.len().max(16);
        while capacity < min_capacity {
            capacity *= 2;
        }
        // C leaves newly allocated storage uninitialized. Zero those otherwise
        // indeterminate bytes, but retain every byte previously written by C's
        // push/assign operations, including bytes outside the logical length.
        self.quoted_string_id.resize(capacity, 0);
    }

    fn assign_id(&mut self, buffer: &[u8]) {
        self.resize_id(buffer.len());
        for (word, bytes) in self.quoted_string_id.iter_mut().zip(buffer.chunks(4)) {
            let mut representation = word.to_ne_bytes();
            representation[..bytes.len()].copy_from_slice(bytes);
            *word = i32::from_ne_bytes(representation);
        }
        self.quoted_string_id_length = buffer.len();
    }

    fn push_id(&mut self, c: i32) {
        self.resize_id(self.quoted_string_id_length + 1);
        self.quoted_string_id[self.quoted_string_id_length] = c;
        self.quoted_string_id_length += 1;
    }

    fn scan_left_quoted_string_delimiter(&mut self, lexer: &mut dyn Lexer) -> bool {
        loop {
            let c = scan_quoted_string_delim_char(lexer);
            if c == 0 {
                break;
            }
            self.push_id(c);
        }
        if lexer.lookahead() == b'|' as i32 {
            lexer.advance(false);
            self.in_string = true;
            true
        } else {
            self.clear_id();
            false
        }
    }

    fn scan_right_quoted_string_delimiter(&mut self, lexer: &mut dyn Lexer) -> bool {
        for i in 0..self.quoted_string_id_length {
            if scan_quoted_string_delim_char(lexer) != self.quoted_string_id[i] {
                return false;
            }
        }
        if lexer.lookahead() == b'}' as i32 {
            // The closing brace belongs to the internal lexer, not this token.
            self.in_string = false;
            self.clear_id();
            true
        } else {
            false
        }
    }

    fn scan_quoted_string(&mut self, lexer: &mut dyn Lexer) -> bool {
        if !self.scan_left_quoted_string_delimiter(lexer) {
            return false;
        }
        loop {
            match lexer.lookahead() {
                124 => {
                    // '|'
                    lexer.advance(false);
                    if self.scan_right_quoted_string_delimiter(lexer) {
                        return true;
                    }
                }
                0 => {
                    if lexer.eof() {
                        return false;
                    }
                    lexer.advance(false);
                }
                _ => lexer.advance(false),
            }
        }
    }

    fn scan_comment(&mut self, lexer: &mut dyn Lexer) -> bool {
        let mut last = 0;
        if lexer.lookahead() != b'*' as i32 {
            return false;
        }
        lexer.advance(false);
        loop {
            match if last != 0 { last } else { lexer.lookahead() } {
                40 => {
                    // '('
                    consume_or_clear_last(lexer, &mut last);
                    self.scan_comment(lexer);
                }
                42 => {
                    // '*'
                    consume_or_clear_last(lexer, &mut last);
                    if lexer.lookahead() == b')' as i32 {
                        lexer.advance(false);
                        return true;
                    }
                }
                39 => {
                    // '\''
                    consume_or_clear_last(lexer, &mut last);
                    last = scan_character(lexer);
                }
                34 => {
                    // '"'
                    consume_or_clear_last(lexer, &mut last);
                    scan_string(lexer);
                }
                123 => {
                    // '{'
                    consume_or_clear_last(lexer, &mut last);
                    if lexer.lookahead() == b'%' as i32 {
                        lexer.advance(false);
                        if lexer.lookahead() == b'%' as i32 {
                            lexer.advance(false);
                        }
                        if scan_extattrident(lexer) {
                            while is_space(lexer.lookahead()) {
                                lexer.advance(false);
                            }
                        } else {
                            continue;
                        }
                    }
                    if self.scan_quoted_string(lexer) {
                        lexer.advance(false);
                    }
                }
                0 => {
                    if lexer.eof() {
                        return true;
                    }
                    consume_or_clear_last(lexer, &mut last);
                }
                _ => {
                    // Do not short-circuit scan_identifier when `last` is set.
                    if scan_identifier(lexer) || last != 0 {
                        last = 0;
                    } else {
                        lexer.advance(false);
                    }
                }
            }
        }
    }
}

fn consume_or_clear_last(lexer: &mut dyn Lexer, last: &mut i32) {
    if *last != 0 {
        *last = 0;
    } else {
        lexer.advance(false);
    }
}

// The reference uses iswspace/iswdigit/towupper in the default C locale.
fn is_space(c: i32) -> bool {
    matches!(c, 0x09..=0x0d | 0x20)
}

fn is_digit(c: i32) -> bool {
    (b'0' as i32..=b'9' as i32).contains(&c)
}

fn scan_string(lexer: &mut dyn Lexer) {
    loop {
        match lexer.lookahead() {
            92 => {
                // '\\' -- even at EOF, both advances occur.
                lexer.advance(false);
                lexer.advance(false);
            }
            34 => {
                lexer.advance(false);
                return;
            }
            0 => {
                if lexer.eof() {
                    return;
                }
                lexer.advance(false);
            }
            _ => lexer.advance(false),
        }
    }
}

const LOWER_UTF8_CHARS: &[i32] = &[
    0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee,
    0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff,
    0x153, 0x161, 0x17e,
];

const LOWER_UTF8_PAIRS: &[(i32, i32, i32)] = &[
    (b'a' as i32, 0x300, 0xe0),
    (b'a' as i32, 0x301, 0xe1),
    (b'a' as i32, 0x302, 0xe2),
    (b'a' as i32, 0x303, 0xe3),
    (b'a' as i32, 0x308, 0xe4),
    (b'a' as i32, 0x30a, 0xe5),
    (b'c' as i32, 0x327, 0xe7),
    (b'e' as i32, 0x300, 0xe8),
    (b'e' as i32, 0x301, 0xe9),
    (b'e' as i32, 0x302, 0xea),
    (b'e' as i32, 0x308, 0xeb),
    (b'i' as i32, 0x300, 0xec),
    (b'i' as i32, 0x301, 0xed),
    (b'i' as i32, 0x302, 0xee),
    (b'i' as i32, 0x308, 0xef),
    (b'n' as i32, 0x303, 0xf1),
    (b'o' as i32, 0x300, 0xf2),
    (b'o' as i32, 0x301, 0xf3),
    (b'o' as i32, 0x302, 0xf4),
    (b'o' as i32, 0x303, 0xf5),
    (b'o' as i32, 0x308, 0xf6),
    (b's' as i32, 0x30c, 0x161),
    (b'u' as i32, 0x300, 0xf9),
    (b'u' as i32, 0x301, 0xfa),
    (b'u' as i32, 0x302, 0xfb),
    (b'u' as i32, 0x308, 0xfc),
    (b'y' as i32, 0x301, 0xfd),
    (b'y' as i32, 0x308, 0xff),
    (b'z' as i32, 0x30c, 0x17e),
];

fn search_lower_utf8_char(c: i32) -> i32 {
    let mut base = 0;
    let mut range = LOWER_UTF8_CHARS.len();
    while range != 0 {
        let probe = base + (range >> 1);
        if c == LOWER_UTF8_CHARS[probe] {
            return c;
        } else if c > LOWER_UTF8_CHARS[probe] {
            base = probe + 1;
            range -= 1;
        }
        range >>= 1;
    }
    0
}

fn search_lower_utf8_pair(c: i32, diacritic: i32) -> i32 {
    let mut base = 0;
    let mut range = LOWER_UTF8_PAIRS.len();
    while range != 0 {
        let probe = base + (range >> 1);
        let (letter, mark, composed) = LOWER_UTF8_PAIRS[probe];
        if c == letter && diacritic == mark {
            return composed;
        } else if c > letter || (c == letter && diacritic > mark) {
            base = probe + 1;
            range -= 1;
        }
        range >>= 1;
    }
    0
}

fn scan_quoted_string_delim_char(lexer: &mut dyn Lexer) -> i32 {
    let c = lexer.lookahead();
    if c == b'|' as i32 {
        return 0;
    }
    if c == b'_' as i32 {
        lexer.advance(false);
        return c;
    }
    if (b'a' as i32..=b'z' as i32).contains(&c) {
        lexer.advance(false);
        if (0x300..=0x327).contains(&lexer.lookahead()) {
            let r = search_lower_utf8_pair(c, lexer.lookahead());
            if r != 0 {
                lexer.advance(false);
                return r;
            }
        }
        return c;
    }
    let r = search_lower_utf8_char(c);
    if r != 0 {
        lexer.advance(false);
        return r;
    }
    0
}

fn scan_character(lexer: &mut dyn Lexer) -> i32 {
    let mut last = 0;
    match lexer.lookahead() {
        92 => {
            // '\\'
            lexer.advance(false);
            if is_digit(lexer.lookahead()) {
                lexer.advance(false);
                for _ in 0..2 {
                    if !is_digit(lexer.lookahead()) {
                        return 0;
                    }
                    lexer.advance(false);
                }
            } else {
                match lexer.lookahead() {
                    120 => {
                        // 'x'
                        lexer.advance(false);
                        for _ in 0..2 {
                            if !is_digit(lexer.lookahead())
                                && !matches!(lexer.lookahead(), 65..=70 | 97..=102)
                            {
                                return 0;
                            }
                            lexer.advance(false);
                        }
                    }
                    111 => {
                        // 'o'
                        lexer.advance(false);
                        for _ in 0..3 {
                            if !is_digit(lexer.lookahead()) || lexer.lookahead() > b'7' as i32 {
                                return 0;
                            }
                            lexer.advance(false);
                        }
                    }
                    // Single quote, double quote, backslash, n, t, b, r, space.
                    39 | 34 | 92 | 110 | 116 | 98 | 114 | 32 => {
                        last = lexer.lookahead();
                        lexer.advance(false);
                    }
                    _ => return 0,
                }
            }
        }
        39 => {} // '\''
        13 => {
            lexer.advance(false);
            while lexer.lookahead() == b'\r' as i32 {
                lexer.advance(false);
            }
            if lexer.lookahead() != b'\n' as i32 {
                return 0;
            }
            lexer.advance(false);
        }
        0 => {
            if lexer.eof() {
                return 0;
            }
            lexer.advance(false);
        }
        _ => {
            if lexer.lookahead() < 256 {
                last = lexer.lookahead();
                lexer.advance(false);
            } else {
                return 0;
            }
        }
    }
    if lexer.lookahead() == b'\'' as i32 {
        lexer.advance(false);
        return 0;
    }
    last
}

fn is_lowercase_ext(c: i32) -> bool {
    (b'a' as i32..=b'z' as i32).contains(&c) || c == b'_' as i32 || c >= 192
}

fn is_identstart(c: i32) -> bool {
    is_lowercase_ext(c) || (b'A' as i32..=b'Z' as i32).contains(&c)
}

fn is_identchar(c: i32) -> bool {
    is_identstart(c) || is_digit(c) || c == b'\'' as i32
}

fn scan_identifier(lexer: &mut dyn Lexer) -> bool {
    if is_identstart(lexer.lookahead()) {
        lexer.advance(false);
        while is_identchar(lexer.lookahead()) {
            lexer.advance(false);
        }
        true
    } else {
        false
    }
}

fn scan_extattrident(lexer: &mut dyn Lexer) -> bool {
    while scan_identifier(lexer) {
        if lexer.lookahead() != b'.' as i32 {
            return true;
        }
        lexer.advance(false);
    }
    false
}

impl ExternalScanner for Scanner {
    fn scan(&mut self, lexer: &mut dyn Lexer, valid_symbols: &[bool]) -> bool {
        if !valid_symbols[ERROR_SENTINEL]
            && valid_symbols[LEFT_QUOTED_STRING_DELIM]
            && (is_lowercase_ext(lexer.lookahead()) || lexer.lookahead() == b'|' as i32)
        {
            lexer.set_result_symbol(LEFT_QUOTED_STRING_DELIM as u16);
            return self.scan_left_quoted_string_delimiter(lexer);
        }
        if !valid_symbols[ERROR_SENTINEL]
            && valid_symbols[RIGHT_QUOTED_STRING_DELIM]
            && lexer.lookahead() == b'|' as i32
        {
            lexer.advance(false);
            lexer.set_result_symbol(RIGHT_QUOTED_STRING_DELIM as u16);
            return self.scan_right_quoted_string_delimiter(lexer);
        }
        if self.in_string && valid_symbols[STRING_DELIM] && lexer.lookahead() == b'"' as i32 {
            lexer.advance(false);
            self.in_string = false;
            lexer.set_result_symbol(STRING_DELIM as u16);
            return true;
        }
        while is_space(lexer.lookahead()) {
            lexer.advance(true);
        }
        if !self.in_string && valid_symbols[STRING_DELIM] && lexer.lookahead() == b'"' as i32 {
            lexer.advance(false);
            self.in_string = true;
            lexer.set_result_symbol(STRING_DELIM as u16);
            return true;
        }
        if !self.in_string
            && valid_symbols[LINE_NUMBER_DIRECTIVE]
            && lexer.lookahead() == b'#' as i32
            && lexer.get_column() == 0
        {
            lexer.advance(false);
            while matches!(lexer.lookahead(), 32 | 9) {
                lexer.advance(false);
            }
            if !is_digit(lexer.lookahead()) {
                return false;
            }
            while is_digit(lexer.lookahead()) {
                lexer.advance(false);
            }
            while matches!(lexer.lookahead(), 32 | 9) {
                lexer.advance(false);
            }
            if lexer.lookahead() != b'"' as i32 {
                return false;
            }
            lexer.advance(false);
            while !matches!(lexer.lookahead(), 10 | 13 | 34) && !lexer.eof() {
                lexer.advance(false);
            }
            if lexer.lookahead() != b'"' as i32 {
                return false;
            }
            lexer.advance(false);
            while !matches!(lexer.lookahead(), 10 | 13) && !lexer.eof() {
                lexer.advance(false);
            }
            lexer.set_result_symbol(LINE_NUMBER_DIRECTIVE as u16);
            return true;
        }
        if !self.in_string && valid_symbols[COMMENT] && lexer.lookahead() == b'(' as i32 {
            lexer.advance(false);
            lexer.set_result_symbol(COMMENT as u16);
            return self.scan_comment(lexer);
        }
        if valid_symbols[NULL_CHARACTER] && lexer.lookahead() == 0 && !lexer.eof() {
            lexer.advance(false);
            lexer.set_result_symbol(NULL_CHARACTER as u16);
            return true;
        }
        false
    }

    fn serialize(&mut self, buffer: &mut [u8]) -> usize {
        let Some(first) = buffer.first_mut() else {
            return 0;
        };
        *first = u8::from(self.in_string);
        let length = self.quoted_string_id_length;
        if length < SERIALIZATION_BUFFER_SIZE && length < buffer.len() {
            for (bytes, word) in buffer[1..1 + length]
                .chunks_mut(4)
                .zip(&self.quoted_string_id)
            {
                bytes.copy_from_slice(&word.to_ne_bytes()[..bytes.len()]);
            }
            length + 1
        } else {
            1
        }
    }

    fn deserialize(&mut self, buffer: &[u8]) {
        if let Some((&first, id)) = buffer.split_first() {
            self.in_string = first != 0;
            self.assign_id(id);
        } else {
            self.in_string = false;
            self.clear_id();
        }
    }
}

/// Creates a scanner (C's `tree_sitter_ocaml_external_scanner_create`).
pub(crate) fn create() -> Box<dyn ExternalScanner> {
    Box::<Scanner>::default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    #[derive(Debug, PartialEq, Eq)]
    enum Call {
        Advance(bool),
        Symbol(u16),
        Column,
        Eof,
    }

    struct TestLexer {
        input: Vec<i32>,
        position: usize,
        symbol: u16,
        calls: RefCell<Vec<Call>>,
    }

    impl TestLexer {
        fn new(input: &str) -> Self {
            Self {
                input: input.chars().map(|c| c as i32).collect(),
                position: 0,
                symbol: u16::MAX,
                calls: RefCell::default(),
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
            self.symbol = symbol;
            self.calls.get_mut().push(Call::Symbol(symbol));
        }

        fn advance(&mut self, skip: bool) {
            self.calls.get_mut().push(Call::Advance(skip));
            if self.position < self.input.len() {
                self.position += 1;
            }
        }

        fn mark_end(&mut self) {
            panic!("the OCaml scanner never calls mark_end");
        }

        fn get_column(&mut self) -> u32 {
            self.calls.get_mut().push(Call::Column);
            self.input[..self.position]
                .iter()
                .rev()
                .take_while(|&&c| c != b'\n' as i32)
                .map(|&c| char::from_u32(c as u32).unwrap().len_utf8() as u32)
                .sum()
        }

        fn is_at_included_range_start(&self) -> bool {
            panic!("the OCaml scanner never checks included range starts");
        }

        fn eof(&self) -> bool {
            self.calls.borrow_mut().push(Call::Eof);
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
    fn quoted_delimiters_normalize_and_leave_the_closing_brace() {
        let mut scanner = Scanner::default();
        let mut left = TestLexer::new("a\u{300}œ_s\u{30c}|body");
        assert!(scanner.scan(&mut left, &valid(&[LEFT_QUOTED_STRING_DELIM])));
        assert_eq!(left.position, 7);
        assert_eq!(&scanner.quoted_string_id[..4], &[0xe0, 0x153, 95, 0x161]);
        assert!(scanner.in_string);
        assert_eq!(
            left.calls.borrow()[0],
            Call::Symbol(LEFT_QUOTED_STRING_DELIM as u16)
        );

        let mut right = TestLexer::new("|àoops}");
        assert!(!scanner.scan(&mut right, &valid(&[RIGHT_QUOTED_STRING_DELIM])));
        assert_eq!(right.position, 3); // The mismatching character was consumed.
        assert_eq!(scanner.quoted_string_id_length, 4);
        assert!(scanner.in_string);

        let mut right = TestLexer::new("|àœ_š}tail");
        assert!(scanner.scan(&mut right, &valid(&[RIGHT_QUOTED_STRING_DELIM])));
        assert_eq!(right.lookahead(), b'}' as i32);
        assert_eq!(right.position, 5);
        assert!(!scanner.in_string);
        assert_eq!(scanner.quoted_string_id_length, 0);
        assert_eq!(
            right.calls.borrow()[..2],
            [Call::Advance(false), Call::Symbol(2)]
        );
    }

    #[test]
    fn normalization_tables_and_rejected_characters() {
        for &c in LOWER_UTF8_CHARS {
            assert_eq!(search_lower_utf8_char(c), c);
        }
        for &(c, diacritic, result) in LOWER_UTF8_PAIRS {
            assert_eq!(search_lower_utf8_pair(c, diacritic), result);
        }
        assert_eq!(search_lower_utf8_char(0xf7), 0);
        assert_eq!(search_lower_utf8_char(0x162), 0);
        assert_eq!(search_lower_utf8_pair(b'b' as i32, 0x300), 0);
        let mut lexer = TestLexer::new("b\u{300}");
        assert_eq!(scan_quoted_string_delim_char(&mut lexer), b'b' as i32);
        assert_eq!(lexer.position, 1);
        assert_eq!(scan_quoted_string_delim_char(&mut lexer), 0);
        assert_eq!(lexer.position, 1);
    }

    #[test]
    fn left_delimiter_appends_and_failure_clears_only_its_length() {
        let mut scanner = Scanner::default();
        for text in ["a|", "b|"] {
            assert!(scanner.scan(
                &mut TestLexer::new(text),
                &valid(&[LEFT_QUOTED_STRING_DELIM])
            ));
        }
        assert_eq!(&scanner.quoted_string_id[..2], &[97, 98]);
        assert_eq!(scanner.quoted_string_id_length, 2);
        let mut lexer = TestLexer::new("cA|");
        assert!(!scanner.scan(&mut lexer, &valid(&[LEFT_QUOTED_STRING_DELIM])));
        assert_eq!(lexer.position, 1);
        assert_eq!(scanner.quoted_string_id_length, 0);
        assert_eq!(&scanner.quoted_string_id[..3], &[97, 98, 99]);
        assert!(scanner.in_string); // Failure does not reset in_string.
    }

    #[test]
    fn snapshots_copy_codepoint_count_bytes_and_retain_the_rest() {
        let mut scanner = Scanner::default();
        let mut lexer = TestLexer::new("abcde|");
        assert!(scanner.scan(&mut lexer, &valid(&[LEFT_QUOTED_STRING_DELIM])));
        let mut buffer = [0xcc; SERIALIZATION_BUFFER_SIZE];
        assert_eq!(scanner.serialize(&mut buffer), 6);
        let mut expected = vec![1];
        expected.extend_from_slice(&97i32.to_ne_bytes());
        expected.push(98i32.to_ne_bytes()[0]);
        assert_eq!(&buffer[..6], expected);
        assert_eq!(buffer[6], 0xcc);
        scanner.deserialize(&[]);
        assert!(!scanner.in_string);
        assert_eq!(scanner.quoted_string_id_length, 0);
        scanner.deserialize(&buffer[..6]);
        assert!(scanner.in_string);
        assert_eq!(scanner.quoted_string_id_length, 5);
        assert_eq!(&scanner.quoted_string_id[..5], &[97, 98, 99, 100, 101]);

        // Partial native-endian words are assigned byte-for-byte, not decoded
        // as one byte per code point, and untouched words remain in storage.
        scanner.deserialize(&[1, 0x12, 0x34]);
        let mut word = 97i32.to_ne_bytes();
        word[..2].copy_from_slice(&[0x12, 0x34]);
        assert_eq!(scanner.quoted_string_id[0], i32::from_ne_bytes(word));
        assert_eq!(&scanner.quoted_string_id[1..5], &[98, 99, 100, 101]);
        assert_eq!(scanner.quoted_string_id_length, 2);
    }

    #[test]
    fn snapshot_limit_counts_bytes_not_int32_storage() {
        let mut scanner = Scanner::default();
        for _ in 0..SERIALIZATION_BUFFER_SIZE - 1 {
            scanner.push_id(b'a' as i32);
        }
        let mut buffer = [0; SERIALIZATION_BUFFER_SIZE];
        assert_eq!(scanner.serialize(&mut buffer), SERIALIZATION_BUFFER_SIZE);
        scanner.push_id(b'b' as i32);
        assert_eq!(scanner.serialize(&mut buffer), 1);
    }

    #[test]
    fn string_closing_precedes_whitespace_skipping() {
        let mut scanner = Scanner::default();
        let mut lexer = TestLexer::new(" \t\"");
        assert!(scanner.scan(&mut lexer, &valid(&[STRING_DELIM])));
        assert!(scanner.in_string);
        assert_eq!(
            *lexer.calls.borrow(),
            [
                Call::Advance(true),
                Call::Advance(true),
                Call::Advance(false),
                Call::Symbol(3)
            ]
        );
        let mut lexer = TestLexer::new(" \"");
        assert!(!scanner.scan(&mut lexer, &valid(&[STRING_DELIM])));
        assert_eq!(lexer.position, 1);
        assert!(scanner.in_string);
        assert!(scanner.scan(&mut lexer, &valid(&[STRING_DELIM])));
        assert!(!scanner.in_string);
    }

    #[test]
    fn error_sentinel_only_suppresses_quoted_delimiters() {
        let mut scanner = Scanner::default();
        let mut lexer = TestLexer::new("abc|");
        assert!(!scanner.scan(
            &mut lexer,
            &valid(&[LEFT_QUOTED_STRING_DELIM, ERROR_SENTINEL])
        ));
        assert_eq!(lexer.position, 0);
        let mut lexer = TestLexer::new("\"");
        assert!(scanner.scan(&mut lexer, &valid(&[STRING_DELIM, ERROR_SENTINEL])));
        let mut lexer = TestLexer::new("\0");
        assert!(scanner.scan(&mut lexer, &valid(&[NULL_CHARACTER, ERROR_SENTINEL])));
        assert_eq!(
            *lexer.calls.borrow(),
            [Call::Eof, Call::Advance(false), Call::Symbol(5)]
        );
        assert!(!scanner.scan(&mut lexer, &valid(&[NULL_CHARACTER])));
    }

    #[test]
    fn line_directives_require_column_zero_and_leave_newline() {
        let mut scanner = Scanner::default();
        let mut lexer = TestLexer::new("\n# \t123 \"a.ml\" trailing\r\n");
        assert!(scanner.scan(&mut lexer, &valid(&[LINE_NUMBER_DIRECTIVE])));
        assert_eq!(lexer.lookahead(), b'\r' as i32);
        assert_eq!(lexer.result_symbol(), LINE_NUMBER_DIRECTIVE as u16);
        assert_eq!(
            lexer.calls.borrow()[..3],
            [Call::Advance(true), Call::Column, Call::Advance(false)]
        );
        let mut lexer = TestLexer::new(" #1\"a.ml\"");
        assert!(!scanner.scan(&mut lexer, &valid(&[LINE_NUMBER_DIRECTIVE])));
        assert_eq!(lexer.position, 1);
        let mut lexer = TestLexer::new("#1\"unterminated\n");
        assert!(!scanner.scan(&mut lexer, &valid(&[LINE_NUMBER_DIRECTIVE])));
        assert_eq!(lexer.lookahead(), b'\n' as i32);
        assert!(!is_space(0xa0));
    }

    #[test]
    fn comments_handle_nesting_strings_characters_extensions_and_nuls() {
        for text in [
            "(* outer (* nested *) end *)tail",
            "(* \"*)\" end *)tail",
            "(* '\"' '\\039' '\\x2a' '\\o052' *)tail",
            "(* {a|*)|a} *)tail",
            "(* {%foo.bar a|*)|a} *)tail",
            "(* {%%foo.bar |*)|} *)tail",
            "(* nul\0 and identifier' *)tail",
            "(* '*)tail", // scan_character returns the consumed '*' via last.
        ] {
            let mut scanner = Scanner::default();
            let mut lexer = TestLexer::new(text);
            assert!(scanner.scan(&mut lexer, &valid(&[COMMENT])), "{text}");
            assert_eq!(lexer.lookahead(), b't' as i32, "{text}");
            assert!(!scanner.in_string, "{text}");
            assert_eq!(scanner.quoted_string_id_length, 0, "{text}");
        }
    }

    #[test]
    fn unterminated_comments_are_tokens_and_keep_quoted_string_state() {
        let mut scanner = Scanner::default();
        let mut lexer = TestLexer::new("(* {abc|unterminated");
        assert!(scanner.scan(&mut lexer, &valid(&[COMMENT])));
        assert!(lexer.eof());
        assert!(scanner.in_string);
        assert_eq!(scanner.quoted_string_id_length, 3);
        let mut lexer = TestLexer::new("\\");
        scan_string(&mut lexer);
        assert_eq!(
            *lexer.calls.borrow(),
            [Call::Advance(false), Call::Advance(false), Call::Eof]
        );
    }

    #[test]
    fn character_scanner_preserves_partial_escapes_and_cr_runs() {
        for (text, position, result) in [
            ("\\123'!", 5, 0),
            ("\\12x", 3, 0),
            ("\\xAf'!", 5, 0),
            ("\\xAg", 3, 0),
            ("\\o777'!", 6, 0),
            ("\\o780", 3, 0),
            ("\\n!", 2, b'n' as i32),
            ("\r\r\n'!", 4, 0),
            ("\r\r!", 2, 0),
            ("''", 1, 0),
            ("é'!", 2, 0),
            ("œ'!", 0, 0),
            ("*!", 1, b'*' as i32),
            ("\0'!", 2, 0),
        ] {
            let mut lexer = TestLexer::new(text);
            assert_eq!(scan_character(&mut lexer), result, "{text:?}");
            assert_eq!(lexer.position, position, "{text:?}");
        }
    }
}
