//! The Fortran external scanner, translated from `src/scanner.c`.

use ts_port_tables::{ExternalScanner, Lexer};

// External token indices, in the order of the C TokenType enum.
const LINE_CONTINUATION: usize = 0;
const INTEGER_LITERAL: usize = 1;
const FLOAT_LITERAL: usize = 2;
const BOZ_LITERAL: usize = 3;
const STRING_LITERAL: usize = 4;
const STRING_LITERAL_KIND: usize = 5;
const END_OF_STATEMENT: usize = 6;
const PREPROC_UNARY_OPERATOR: usize = 7;
const HOLLERITH_CONSTANT: usize = 8;
const DO_LABEL: usize = 9;
const DO_LABEL_VIRTUAL: usize = 10;
const DO_LABEL_CONTINUE: usize = 11;

const MAX_LABEL_STACK: usize = 100;

#[derive(Debug, PartialEq, Eq)]
struct DoLabel {
    value: i32,
    count: i32,
}

/// The active prefix of C's labels/counts arrays is kept as one bounded stack.
#[derive(Default)]
pub(crate) struct Scanner {
    in_line_continuation: bool,
    labels: Vec<DoLabel>,
    pending_label_virtual: i32,
    is_pending_eos_virtual: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum NumberType {
    None,
    Integer,
    Float,
}

#[derive(Debug)]
struct NumberResult {
    kind: NumberType,
    value: i32,
    digit_count: i32,
}

// The reference uses the default C locale for wide character classification.
fn is_space(c: i32) -> bool {
    matches!(c, 0x09..=0x0d | 0x20)
}

fn is_blank(c: i32) -> bool {
    matches!(c, 0x09 | 0x20)
}

fn is_digit(c: i32) -> bool {
    matches!(c, 0x30..=0x39)
}

fn is_alpha(c: i32) -> bool {
    matches!(c, 0x41..=0x5a | 0x61..=0x7a)
}

fn is_alnum(c: i32) -> bool {
    is_alpha(c) || is_digit(c)
}

fn is_hex_digit(c: i32) -> bool {
    is_digit(c) || matches!(c, 0x41..=0x46 | 0x61..=0x66)
}

fn is_identifier_char(c: i32) -> bool {
    // This helper, unlike the direct iswalnum calls, takes a C char.
    let c = i32::from(c as i8);
    is_alnum(c) || c == i32::from(b'_')
}

fn is_boz_sentinel(c: i32) -> bool {
    matches!(c as u8, b'B' | b'b' | b'O' | b'o' | b'Z' | b'z')
}

fn is_exp_sentinel(c: i32) -> bool {
    matches!(c as u8, b'D' | b'd' | b'E' | b'e' | b'Q' | b'q')
}

fn skip_literal_continuation_sequence(lexer: &mut dyn Lexer) -> bool {
    if lexer.lookahead() != i32::from(b'&') {
        return true;
    }
    lexer.advance(false);
    while is_space(lexer.lookahead()) {
        lexer.advance(false);
    }
    if lexer.lookahead() == i32::from(b'&') {
        lexer.advance(false);
        return true;
    }
    false
}

fn scan_int(lexer: &mut dyn Lexer, mut result: Option<&mut NumberResult>) -> bool {
    if !is_digit(lexer.lookahead()) {
        return false;
    }
    if let Some(result) = result.as_deref_mut() {
        result.value = 0;
        result.digit_count = 0;
    }

    loop {
        while is_digit(lexer.lookahead()) {
            if let Some(result) = result.as_deref_mut()
                && result.digit_count < 7
            {
                result.value = result.value * 10 + lexer.lookahead() - i32::from(b'0');
                result.digit_count += 1;
            }
            lexer.advance(false);
        }
        lexer.mark_end();

        if lexer.lookahead() == i32::from(b'&') && skip_literal_continuation_sequence(lexer) {
            continue;
        }
        break;
    }
    true
}

fn scan_number(lexer: &mut dyn Lexer) -> NumberResult {
    let mut result = NumberResult {
        kind: NumberType::Integer,
        value: 0,
        digit_count: 0,
    };
    let mut digits = scan_int(lexer, Some(&mut result));

    if lexer.lookahead() == i32::from(b'.') {
        lexer.advance(false);
        if digits && !is_alnum(lexer.lookahead()) {
            lexer.mark_end();
        }
        result.kind = NumberType::Float;
    }

    // The second scan must run even when there were leading digits.
    digits = scan_int(lexer, None) || digits;
    if digits && is_exp_sentinel(lexer.lookahead()) {
        lexer.advance(false);
        if lexer.lookahead() == i32::from(b'+') || lexer.lookahead() == i32::from(b'-') {
            lexer.advance(false);
            lexer.mark_end();
        }
        if !scan_int(lexer, None) {
            result.kind = NumberType::Integer;
            return result;
        }
        result.kind = NumberType::Float;
    }

    if !digits {
        result.kind = NumberType::None;
    }
    result
}

fn scan_boz(lexer: &mut dyn Lexer) -> bool {
    lexer.set_result_symbol(BOZ_LITERAL as u16);
    let mut boz_prefix = false;
    if is_boz_sentinel(lexer.lookahead()) {
        lexer.advance(false);
        boz_prefix = true;
    }
    if lexer.lookahead() == i32::from(b'\'') || lexer.lookahead() == i32::from(b'"') {
        let quote = lexer.lookahead();
        lexer.advance(false);
        if !is_hex_digit(lexer.lookahead()) {
            return false;
        }
        while is_hex_digit(lexer.lookahead()) {
            lexer.advance(false);
        }
        if lexer.lookahead() != quote {
            return false;
        }
        lexer.advance(false);
        if !boz_prefix && !is_boz_sentinel(lexer.lookahead()) {
            return false;
        }
        // C checks a suffix, but does not consume it.
        lexer.mark_end();
        return true;
    }
    false
}

fn scan_hollerith_constant(lexer: &mut dyn Lexer) -> bool {
    let mut length = 0u32;
    while is_digit(lexer.lookahead()) {
        let new_length = length
            .wrapping_mul(10)
            .wrapping_add((lexer.lookahead() - i32::from(b'0')) as u32);
        // Preserve C's overflow test, rather than using checked multiplication.
        if new_length < length {
            return false;
        }
        length = new_length;
        lexer.advance(false);
        if !skip_literal_continuation_sequence(lexer) {
            return false;
        }
    }
    if length == 0 {
        return false;
    }
    if lexer.lookahead() != i32::from(b'H') && lexer.lookahead() != i32::from(b'h') {
        return false;
    }
    lexer.advance(false);

    for _ in 0..length {
        if lexer.lookahead() == 0 || lexer.eof() {
            return false;
        }
        if !skip_literal_continuation_sequence(lexer) {
            return false;
        }
        lexer.advance(false);
    }
    lexer.set_result_symbol(HOLLERITH_CONSTANT as u16);
    lexer.mark_end();
    true
}

fn scan_string_literal_kind(lexer: &mut dyn Lexer) -> bool {
    if !is_alpha(lexer.lookahead()) {
        return false;
    }
    lexer.set_result_symbol(STRING_LITERAL_KIND as u16);
    let mut current_char = 0u8;
    while is_identifier_char(lexer.lookahead()) && !lexer.eof() {
        current_char = lexer.lookahead() as u8;
        if lexer.lookahead() == i32::from(b'_') {
            lexer.mark_end();
        }
        lexer.advance(false);
    }
    current_char == b'_'
        && (lexer.lookahead() == i32::from(b'"') || lexer.lookahead() == i32::from(b'\''))
}

fn scan_string_literal(lexer: &mut dyn Lexer) -> bool {
    let opening_quote = i32::from(lexer.lookahead() as i8);
    if opening_quote != i32::from(b'"') && opening_quote != i32::from(b'\'') {
        return false;
    }
    lexer.advance(false);
    lexer.set_result_symbol(STRING_LITERAL as u16);

    while lexer.lookahead() != i32::from(b'\n') && !lexer.eof() {
        if lexer.lookahead() == i32::from(b'&') {
            lexer.advance(false);
            while is_blank(lexer.lookahead()) {
                lexer.advance(false);
            }
            if lexer.lookahead() == i32::from(b'\n') || lexer.lookahead() == i32::from(b'\r') {
                while is_space(lexer.lookahead()) {
                    lexer.advance(false);
                }
            }
            continue;
        }
        if lexer.lookahead() == opening_quote {
            lexer.advance(false);
            lexer.mark_end();
            // A failed continuation still consumes input. C ignores its result.
            skip_literal_continuation_sequence(lexer);
            if lexer.lookahead() != opening_quote {
                return true;
            }
        }
        lexer.advance(false);
    }
    false
}

fn scan_preproc_unary_operator(lexer: &mut dyn Lexer) -> bool {
    if matches!(lexer.lookahead() as u8, b'!' | b'~' | b'-' | b'+') {
        lexer.advance(false);
        lexer.set_result_symbol(PREPROC_UNARY_OPERATOR as u16);
        return true;
    }
    false
}

impl Scanner {
    fn scan_end_of_statement(&self, lexer: &mut dyn Lexer) -> bool {
        if lexer.eof() {
            lexer.advance(true);
            lexer.set_result_symbol(END_OF_STATEMENT as u16);
            return true;
        }
        if self.in_line_continuation {
            return false;
        }
        if lexer.lookahead() == i32::from(b'\r') {
            lexer.advance(true);
            if lexer.lookahead() == i32::from(b'\n') {
                lexer.advance(true);
            }
        } else if lexer.lookahead() == i32::from(b'\n') {
            lexer.advance(true);
        } else if lexer.lookahead() != i32::from(b'!') {
            return false;
        }
        lexer.set_result_symbol(END_OF_STATEMENT as u16);
        true
    }

    fn scan_start_line_continuation(&mut self, lexer: &mut dyn Lexer) -> bool {
        self.in_line_continuation = lexer.lookahead() == i32::from(b'&');
        if !self.in_line_continuation {
            return false;
        }
        lexer.advance(false);
        lexer.set_result_symbol(LINE_CONTINUATION as u16);
        true
    }

    fn scan_end_line_continuation(&mut self, lexer: &mut dyn Lexer) -> bool {
        if !self.in_line_continuation || lexer.lookahead() == i32::from(b'!') {
            return false;
        }
        self.in_line_continuation = false;
        if lexer.lookahead() == i32::from(b'&') {
            lexer.advance(false);
        }
        lexer.set_result_symbol(LINE_CONTINUATION as u16);
        true
    }

    fn track_labeled_do(&mut self, label: i32) {
        if let Some(top) = self.labels.last_mut()
            && top.value == label
        {
            top.count = top.count.wrapping_add(1);
            return;
        }
        if self.labels.len() < MAX_LABEL_STACK {
            self.labels.push(DoLabel {
                value: label,
                count: 1,
            });
        }
    }

    fn scan_do_label_eos(&mut self, lexer: &mut dyn Lexer) -> bool {
        if self.is_pending_eos_virtual {
            self.is_pending_eos_virtual = false;
            lexer.set_result_symbol(END_OF_STATEMENT as u16);
            return true;
        }
        false
    }

    fn scan_do_label_pending(&mut self, lexer: &mut dyn Lexer) -> bool {
        if self.pending_label_virtual > 0 {
            if self.pending_label_virtual > 1 {
                self.pending_label_virtual -= 1;
                self.is_pending_eos_virtual = true;
                lexer.set_result_symbol(DO_LABEL_VIRTUAL as u16);
            } else {
                self.pending_label_virtual = 0;
                lexer.set_result_symbol(DO_LABEL_CONTINUE as u16);
            }
            return true;
        }
        false
    }

    fn scan_do_label(&mut self, lexer: &mut dyn Lexer, label: i32) {
        self.track_labeled_do(label);
        lexer.set_result_symbol(DO_LABEL as u16);
    }

    fn scan_do_label_continue(&mut self, lexer: &mut dyn Lexer, label: i32) -> bool {
        let mut loops_to_close = 0;
        if let Some(top) = self.labels.pop_if(|top| top.value == label) {
            loops_to_close = top.count;
        }
        if loops_to_close == 0 {
            return false;
        }
        self.pending_label_virtual = loops_to_close;
        self.is_pending_eos_virtual = false;
        self.scan_do_label_pending(lexer);
        true
    }

    fn scan_label_number_boz(&mut self, lexer: &mut dyn Lexer, valid_symbols: &[bool]) -> bool {
        let result = scan_number(lexer);
        if result.kind == NumberType::Integer && result.digit_count < 6 {
            if valid_symbols[DO_LABEL] {
                self.scan_do_label(lexer, result.value);
                return true;
            }
            if valid_symbols[DO_LABEL_CONTINUE] && self.scan_do_label_continue(lexer, result.value)
            {
                return true;
            }
        }
        match result.kind {
            NumberType::Integer => {
                lexer.set_result_symbol(INTEGER_LITERAL as u16);
                true
            }
            NumberType::Float => {
                lexer.set_result_symbol(FLOAT_LITERAL as u16);
                true
            }
            NumberType::None => scan_boz(lexer),
        }
    }
}

impl ExternalScanner for Scanner {
    fn scan(&mut self, lexer: &mut dyn Lexer, valid_symbols: &[bool]) -> bool {
        if valid_symbols[END_OF_STATEMENT] && self.scan_do_label_eos(lexer) {
            return true;
        }
        if (valid_symbols[DO_LABEL_CONTINUE] || valid_symbols[DO_LABEL_VIRTUAL])
            && self.scan_do_label_pending(lexer)
        {
            return true;
        }
        while is_blank(lexer.lookahead()) {
            lexer.advance(true);
        }
        if valid_symbols[END_OF_STATEMENT] && self.scan_end_of_statement(lexer) {
            return true;
        }
        while is_space(lexer.lookahead()) {
            lexer.advance(true);
        }
        if self.scan_end_line_continuation(lexer) {
            return true;
        }
        if valid_symbols[STRING_LITERAL] && scan_string_literal(lexer) {
            return true;
        }
        if valid_symbols[HOLLERITH_CONSTANT] && scan_hollerith_constant(lexer) {
            return true;
        }
        if (valid_symbols[INTEGER_LITERAL]
            || valid_symbols[FLOAT_LITERAL]
            || valid_symbols[BOZ_LITERAL]
            || valid_symbols[DO_LABEL]
            || valid_symbols[DO_LABEL_CONTINUE])
            && self.scan_label_number_boz(lexer, valid_symbols)
        {
            return true;
        }
        if valid_symbols[PREPROC_UNARY_OPERATOR] && scan_preproc_unary_operator(lexer) {
            return true;
        }
        if self.scan_start_line_continuation(lexer) {
            return true;
        }
        valid_symbols[STRING_LITERAL_KIND] && scan_string_literal_kind(lexer)
    }

    fn serialize(&mut self, buffer: &mut [u8]) -> usize {
        let depth = self.labels.len();
        let size = 10 + 8 * depth;
        if depth > MAX_LABEL_STACK || buffer.len() < size {
            return 0;
        }
        buffer[0] = u8::from(self.in_line_continuation);
        buffer[1..5].copy_from_slice(&(depth as i32).to_ne_bytes());
        let mut offset = 5;
        // The C snapshot has all labels first, followed by all counts.
        for label in &self.labels {
            buffer[offset..offset + 4].copy_from_slice(&label.value.to_ne_bytes());
            offset += 4;
        }
        for label in &self.labels {
            buffer[offset..offset + 4].copy_from_slice(&label.count.to_ne_bytes());
            offset += 4;
        }
        buffer[offset..offset + 4].copy_from_slice(&self.pending_label_virtual.to_ne_bytes());
        buffer[offset + 4] = u8::from(self.is_pending_eos_virtual);
        size
    }

    fn deserialize(&mut self, buffer: &[u8]) {
        self.in_line_continuation = buffer.first().is_some_and(|&b| b != 0);
        self.labels.clear();
        self.pending_label_virtual = 0;
        self.is_pending_eos_virtual = false;
        if buffer.is_empty() {
            return;
        }
        // Truncated/negative-depth snapshots are undefined in C. Reject them
        // safely, using the same reset as C's over-limit depth check.
        if buffer.len() < 5 {
            return;
        }
        let depth = i32::from_ne_bytes(buffer[1..5].try_into().unwrap());
        if !(0..=MAX_LABEL_STACK as i32).contains(&depth) {
            return;
        }
        let depth = depth as usize;
        if buffer.len() < 10 + 8 * depth {
            return;
        }
        for i in 0..depth {
            let value_offset = 5 + 4 * i;
            let count_offset = value_offset + 4 * depth;
            self.labels.push(DoLabel {
                value: i32::from_ne_bytes(
                    buffer[value_offset..value_offset + 4].try_into().unwrap(),
                ),
                count: i32::from_ne_bytes(
                    buffer[count_offset..count_offset + 4].try_into().unwrap(),
                ),
            });
        }
        let offset = 5 + 8 * depth;
        self.pending_label_virtual =
            i32::from_ne_bytes(buffer[offset..offset + 4].try_into().unwrap());
        self.is_pending_eos_virtual = buffer[offset + 4] != 0;
    }
}

pub(crate) fn create() -> Box<dyn ExternalScanner> {
    Box::new(Scanner::default())
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

    struct TestLexer {
        input: Vec<i32>,
        position: usize,
        end: usize,
        symbol: u16,
        events: Vec<Event>,
    }

    impl TestLexer {
        fn new(input: &str) -> Self {
            Self {
                input: input.chars().map(|c| c as i32).collect(),
                position: 0,
                end: 0,
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
            self.symbol = symbol;
            self.events.push(Event::Symbol(symbol));
        }
        fn advance(&mut self, skip: bool) {
            self.events.push(Event::Advance(self.position, skip));
            if !self.eof() {
                self.position += 1;
            }
        }
        fn mark_end(&mut self) {
            self.end = self.position;
            self.events.push(Event::Mark(self.position));
        }
        fn get_column(&mut self) -> u32 {
            panic!("the Fortran scanner does not request columns")
        }
        fn is_at_included_range_start(&self) -> bool {
            false
        }
        fn eof(&self) -> bool {
            self.position == self.input.len()
        }
    }

    fn valid(tokens: &[usize]) -> [bool; 12] {
        let mut result = [false; 12];
        for &token in tokens {
            result[token] = true;
        }
        result
    }

    #[test]
    fn numbers_preserve_lookahead_marks_and_digit_limit() {
        for (text, kind, value, digit_count, position, end) in [
            ("123456789x", NumberType::Integer, 1_234_567, 7, 9, 9),
            ("12&\n &34e+5!", NumberType::Float, 1234, 4, 11, 11),
            ("1.abc", NumberType::Float, 1, 1, 2, 1),
            ("1.e+", NumberType::Integer, 1, 1, 4, 4),
            ("1.efoo", NumberType::Integer, 1, 1, 3, 1),
            (".e10", NumberType::None, 0, 0, 1, 0),
            ("1&\nX", NumberType::Integer, 1, 1, 3, 1),
            ("1&\n&!", NumberType::Integer, 1, 1, 4, 4),
        ] {
            let mut lexer = TestLexer::new(text);
            let result = scan_number(&mut lexer);
            assert_eq!(
                (result.kind, result.value, result.digit_count),
                (kind, value, digit_count),
                "{text:?}"
            );
            assert_eq!((lexer.position, lexer.end), (position, end), "{text:?}");
        }

        let mut lexer = TestLexer::new("1e+");
        assert_eq!(scan_number(&mut lexer).kind, NumberType::Integer);
        assert_eq!(
            lexer.events,
            [
                Event::Advance(0, false),
                Event::Mark(1),
                Event::Advance(1, false),
                Event::Advance(2, false),
                Event::Mark(3),
            ]
        );
    }

    #[test]
    fn boz_suffix_is_checked_but_not_consumed() {
        let mut lexer = TestLexer::new("'AB'z");
        assert!(scan_boz(&mut lexer));
        assert_eq!(lexer.symbol, BOZ_LITERAL as u16);
        assert_eq!((lexer.position, lexer.end), (4, 4));
        assert_eq!(lexer.lookahead(), i32::from(b'z'));

        // C accepts hex digits even for the binary prefix.
        let mut lexer = TestLexer::new("b'FE'");
        assert!(scan_boz(&mut lexer));
        assert_eq!(lexer.end, 5);
    }

    #[test]
    fn hollerith_continuations_and_overflow_follow_c() {
        let mut lexer = TestLexer::new("2H&\n&aZ!");
        assert!(scan_hollerith_constant(&mut lexer));
        assert_eq!(lexer.symbol, HOLLERITH_CONSTANT as u16);
        assert_eq!((lexer.position, lexer.end), (7, 7));

        let mut lexer = TestLexer::new("0Hx");
        assert!(!scan_hollerith_constant(&mut lexer));
        assert_eq!(lexer.position, 1);

        let mut lexer = TestLexer::new("4294967296Hx");
        assert!(!scan_hollerith_constant(&mut lexer));
        assert_eq!(lexer.position, 9);

        // Multiplication can wrap without producing a decrease. C accepts this
        // prefix (705032704 after wrapping), advances over H, then fails at EOF.
        let mut lexer = TestLexer::new("5000000000H");
        assert!(!scan_hollerith_constant(&mut lexer));
        assert_eq!(lexer.position, 11);
    }

    #[test]
    fn strings_keep_marks_before_failed_continuation_lookahead() {
        let mut lexer = TestLexer::new("'a'& \nx");
        assert!(scan_string_literal(&mut lexer));
        assert_eq!((lexer.position, lexer.end), (6, 3));
        assert_eq!(
            lexer.events,
            [
                Event::Advance(0, false),
                Event::Symbol(STRING_LITERAL as u16),
                Event::Advance(1, false),
                Event::Advance(2, false),
                Event::Mark(3),
                Event::Advance(3, false),
                Event::Advance(4, false),
                Event::Advance(5, false),
            ]
        );

        let mut lexer = TestLexer::new("'a'&\n&'b'");
        assert!(scan_string_literal(&mut lexer));
        assert_eq!((lexer.position, lexer.end), (9, 9));
        let mut lexer = TestLexer::new("'a\nb'");
        assert!(!scan_string_literal(&mut lexer));
        assert_eq!(lexer.position, 2);
    }

    #[test]
    fn kind_excludes_only_the_final_underscore() {
        let mut lexer = TestLexer::new("kind_x_'text'");
        assert!(scan_string_literal_kind(&mut lexer));
        assert_eq!(lexer.symbol, STRING_LITERAL_KIND as u16);
        assert_eq!((lexer.position, lexer.end), (7, 6));
        assert_eq!(
            lexer
                .events
                .iter()
                .filter(|e| matches!(e, Event::Mark(_)))
                .collect::<Vec<_>>(),
            [&Event::Mark(4), &Event::Mark(6)]
        );
    }

    #[test]
    fn c_locale_and_char_narrowing_are_distinct() {
        assert!(is_space(0x0b));
        assert!(!is_space(0x00a0));
        assert!(!is_alpha(0x0141));
        assert!(is_identifier_char(0x0141));
        assert!(is_boz_sentinel(0x0142));
        assert!(is_exp_sentinel(0x0145));
        let mut lexer = TestLexer::new("\u{0121}");
        assert!(scan_preproc_unary_operator(&mut lexer));
        let mut lexer = TestLexer::new("\u{0122}x\"");
        assert!(scan_string_literal(&mut lexer));
        assert_eq!(lexer.end, 3);
    }

    #[test]
    fn failed_scanners_do_not_rewind_before_the_next_attempt() {
        let mut scanner = Scanner::default();
        let mut lexer = TestLexer::new("12Z'AB'");
        assert!(scanner.scan(&mut lexer, &valid(&[HOLLERITH_CONSTANT, INTEGER_LITERAL])));
        // Hollerith consumes 12 and fails, then the number/BOZ path sees Z.
        assert_eq!(lexer.symbol, BOZ_LITERAL as u16);
        assert_eq!(lexer.end, 7);
        let mut lexer = TestLexer::new("12");
        assert!(!scanner.scan(&mut lexer, &valid(&[HOLLERITH_CONSTANT, INTEGER_LITERAL])));
        assert_eq!(lexer.position, 2);
    }

    #[test]
    fn shared_do_labels_emit_virtual_tokens_before_any_whitespace() {
        let mut scanner = Scanner::default();
        for _ in 0..3 {
            let mut lexer = TestLexer::new("10");
            assert!(scanner.scan(&mut lexer, &valid(&[DO_LABEL])));
            assert_eq!(lexer.symbol, DO_LABEL as u16);
        }
        assert_eq!(
            scanner.labels,
            [DoLabel {
                value: 10,
                count: 3
            }]
        );
        let mut lexer = TestLexer::new(" 10 continue");
        assert!(scanner.scan(&mut lexer, &valid(&[DO_LABEL_CONTINUE])));
        assert_eq!(lexer.symbol, DO_LABEL_VIRTUAL as u16);
        assert_eq!(lexer.end, 3);
        assert!(scanner.labels.is_empty());

        let mut lexer = TestLexer::new("  next");
        for token in [
            END_OF_STATEMENT,
            DO_LABEL_VIRTUAL,
            END_OF_STATEMENT,
            DO_LABEL_CONTINUE,
        ] {
            assert!(scanner.scan(&mut lexer, &valid(&[END_OF_STATEMENT, DO_LABEL_VIRTUAL])));
            assert_eq!(lexer.symbol, token as u16);
            assert_eq!(lexer.events, [Event::Symbol(token as u16)]);
            lexer.events.clear();
        }
        assert_eq!(scanner.pending_label_virtual, 0);
        assert!(!scanner.is_pending_eos_virtual);
    }

    #[test]
    fn do_labels_only_match_the_top_and_have_five_digits() {
        let mut scanner = Scanner::default();
        scanner.track_labeled_do(10);
        scanner.track_labeled_do(20);
        scanner.track_labeled_do(10);
        assert_eq!(scanner.labels.len(), 3);
        let mut lexer = TestLexer::new("20");
        assert!(scanner.scan(&mut lexer, &valid(&[DO_LABEL_CONTINUE])));
        assert_eq!(lexer.symbol, INTEGER_LITERAL as u16);
        assert_eq!(scanner.labels.len(), 3);
        let mut lexer = TestLexer::new("123456");
        assert!(scanner.scan(&mut lexer, &valid(&[DO_LABEL])));
        assert_eq!(lexer.symbol, INTEGER_LITERAL as u16);
        assert_eq!(scanner.labels.len(), 3);

        scanner.labels.clear();
        for label in 0..=MAX_LABEL_STACK {
            scanner.track_labeled_do(label as i32);
        }
        assert_eq!(scanner.labels.len(), MAX_LABEL_STACK);
        scanner.track_labeled_do((MAX_LABEL_STACK - 1) as i32);
        assert_eq!(scanner.labels.last().unwrap().count, 2);
    }

    #[test]
    fn continuations_and_statement_end_callback_order() {
        let mut scanner = Scanner::default();
        let mut lexer = TestLexer::new("&");
        assert!(scanner.scan(&mut lexer, &valid(&[])));
        assert!(scanner.in_line_continuation);
        assert_eq!(lexer.symbol, LINE_CONTINUATION as u16);

        let mut lexer = TestLexer::new("\n  &x");
        assert!(scanner.scan(&mut lexer, &valid(&[END_OF_STATEMENT])));
        assert!(!scanner.in_line_continuation);
        assert_eq!(
            lexer.events,
            [
                Event::Advance(0, true),
                Event::Advance(1, true),
                Event::Advance(2, true),
                Event::Advance(3, false),
                Event::Symbol(LINE_CONTINUATION as u16),
            ]
        );
        let mut lexer = TestLexer::new(" \r\n!");
        assert!(scanner.scan(&mut lexer, &valid(&[END_OF_STATEMENT])));
        assert_eq!(
            lexer.events,
            [
                Event::Advance(0, true),
                Event::Advance(1, true),
                Event::Advance(2, true),
                Event::Symbol(END_OF_STATEMENT as u16),
            ]
        );
        let mut lexer = TestLexer::new("! comment");
        assert!(scanner.scan(&mut lexer, &valid(&[END_OF_STATEMENT])));
        assert_eq!(lexer.events, [Event::Symbol(END_OF_STATEMENT as u16)]);

        // EOF ends a statement even when a continuation is active, and still
        // invokes advance(skip=true) at EOF, without clearing that state.
        scanner.in_line_continuation = true;
        let mut lexer = TestLexer::new("");
        assert!(scanner.scan(&mut lexer, &valid(&[END_OF_STATEMENT])));
        assert!(scanner.in_line_continuation);
        assert_eq!(
            lexer.events,
            [
                Event::Advance(0, true),
                Event::Symbol(END_OF_STATEMENT as u16)
            ]
        );
    }

    #[test]
    fn snapshots_match_native_endian_c_layout_and_reset_rules() {
        let mut scanner = Scanner {
            in_line_continuation: true,
            pending_label_virtual: 4,
            is_pending_eos_virtual: true,
            ..Scanner::default()
        };
        scanner.track_labeled_do(10);
        scanner.track_labeled_do(10);
        scanner.track_labeled_do(20);
        let mut expected = vec![1];
        for value in [2_i32, 10, 20, 2, 1, 4] {
            expected.extend_from_slice(&value.to_ne_bytes());
        }
        expected.push(1);
        let mut buffer = [0; 1024];
        let size = scanner.serialize(&mut buffer);
        assert_eq!(size, 26);
        assert_eq!(&buffer[..size], expected);
        let mut restored = Scanner::default();
        restored.deserialize(&buffer[..size]);
        assert_eq!(restored.serialize(&mut buffer), size);
        assert_eq!(&buffer[..size], expected);

        // Over-limit depth resets labels/pending tokens but keeps continuation.
        let mut invalid = vec![1];
        invalid.extend_from_slice(&101_i32.to_ne_bytes());
        restored.deserialize(&invalid);
        assert!(restored.in_line_continuation);
        assert!(restored.labels.is_empty());
        assert_eq!(restored.pending_label_virtual, 0);
        assert!(!restored.is_pending_eos_virtual);
        restored.deserialize(&[]);
        assert_eq!(restored.serialize(&mut buffer), 10);
        assert_eq!(&buffer[..10], &[0; 10]);

        for label in 0..MAX_LABEL_STACK {
            restored.track_labeled_do(label as i32);
        }
        assert_eq!(restored.serialize(&mut buffer), 810);
        scanner.deserialize(&buffer[..810]);
        assert_eq!(scanner.labels, restored.labels);
    }
}
