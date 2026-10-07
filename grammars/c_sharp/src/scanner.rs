//! The C# grammar's external scanner, translated from `src/scanner.c`.

use ts_port_tables::{ExternalScanner, Lexer, SERIALIZATION_BUFFER_SIZE};

// External token indices, in the order of the C TokenType enum.
const OPT_SEMI: usize = 0;
const INTERPOLATION_REGULAR_START: usize = 1;
const INTERPOLATION_VERBATIM_START: usize = 2;
const INTERPOLATION_RAW_START: usize = 3;
const INTERPOLATION_START_QUOTE: usize = 4;
const INTERPOLATION_END_QUOTE: usize = 5;
const INTERPOLATION_OPEN_BRACE: usize = 6;
const INTERPOLATION_CLOSE_BRACE: usize = 7;
const INTERPOLATION_STRING_CONTENT: usize = 8;
const RAW_STRING_START: usize = 9;
const RAW_STRING_END: usize = 10;
const RAW_STRING_CONTENT: usize = 11;

// StringType is a bit mask: verbatim interpolations also carry REGULAR.
const REGULAR: u8 = 1 << 0;
const VERBATIM: u8 = 1 << 1;
const RAW: u8 = 1 << 2;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct Interpolation {
    dollar_count: u8,
    open_brace_count: u8,
    quote_count: u8,
    string_type: u8,
}

impl Interpolation {
    fn is_regular(&self) -> bool {
        self.string_type & REGULAR != 0
    }

    fn is_verbatim(&self) -> bool {
        self.string_type & VERBATIM != 0
    }

    fn is_raw(&self) -> bool {
        self.string_type & RAW != 0
    }
}

/// The scanner's state (C's `payload`). The Vec owns C's interpolation array.
#[derive(Default)]
pub(crate) struct Scanner {
    quote_count: u8,
    interpolation_stack: Vec<Interpolation>,
}

// The reference uses iswspace in the default C locale, not Unicode whitespace.
// Rust's is_ascii_whitespace also differs: it excludes vertical tab.
fn is_space(c: i32) -> bool {
    matches!(c, 0x09..=0x0d | 0x20)
}

impl Scanner {
    // Keep the full ordered token scan out of ordinary-code rejection. All
    // lookahead tests reuse the current position's code point until advance.
    #[inline(never)]
    fn scan_string(
        &mut self,
        lexer: &mut dyn Lexer,
        valid_symbols: &[bool; RAW_STRING_CONTENT + 1],
        mut lookahead: i32,
    ) -> bool {
        let mut brace_advanced = 0u8;
        let mut quote_count = 0u8;
        let mut did_advance = false;

        if valid_symbols[RAW_STRING_START] {
            while is_space(lookahead) {
                lexer.advance(true);
                lookahead = lexer.lookahead();
            }

            if lookahead == i32::from(b'"') {
                while lookahead == i32::from(b'"') {
                    lexer.advance(false);
                    lookahead = lexer.lookahead();
                    quote_count = quote_count.wrapping_add(1);
                }

                if quote_count >= 3 {
                    lexer.set_result_symbol(RAW_STRING_START as u16);
                    self.quote_count = quote_count;
                    return true;
                }
            }
        }

        if valid_symbols[RAW_STRING_END] && lookahead == i32::from(b'"') {
            while lookahead == i32::from(b'"') {
                lexer.advance(false);
                lookahead = lexer.lookahead();
                quote_count = quote_count.wrapping_add(1);
            }

            if quote_count == self.quote_count {
                lexer.set_result_symbol(RAW_STRING_END as u16);
                self.quote_count = 0;
                return true;
            }

            did_advance = quote_count > 0;
        }

        if valid_symbols[RAW_STRING_CONTENT] {
            while lookahead != 0 {
                if lookahead == i32::from(b'"') {
                    lexer.mark_end();
                    quote_count = 0;

                    while lookahead == i32::from(b'"') {
                        lexer.advance(false);
                        lookahead = lexer.lookahead();
                        quote_count = quote_count.wrapping_add(1);
                    }

                    if quote_count == self.quote_count {
                        lexer.set_result_symbol(RAW_STRING_CONTENT as u16);
                        return true;
                    }
                }
                lexer.advance(false);
                lookahead = lexer.lookahead();
                // C also assigns did_advance here, but this branch always
                // returns true without reading it again.
            }
            lexer.mark_end();
            lexer.set_result_symbol(RAW_STRING_CONTENT as u16);
            return true;
        }

        if valid_symbols[INTERPOLATION_REGULAR_START]
            || valid_symbols[INTERPOLATION_VERBATIM_START]
            || valid_symbols[INTERPOLATION_RAW_START]
        {
            while is_space(lookahead) {
                lexer.advance(true);
                lookahead = lexer.lookahead();
            }

            let mut dollar_advanced = 0u8;
            let mut is_verbatim = false;

            if lookahead == i32::from(b'@') {
                is_verbatim = true;
                lexer.advance(false);
                lookahead = lexer.lookahead();
            }

            while lookahead == i32::from(b'$') && quote_count == 0 {
                lexer.advance(false);
                lookahead = lexer.lookahead();
                dollar_advanced = dollar_advanced.wrapping_add(1);
            }

            if dollar_advanced > 0 && (lookahead == i32::from(b'"') || lookahead == i32::from(b'@'))
            {
                lexer.set_result_symbol(INTERPOLATION_REGULAR_START as u16);
                let mut interpolation = Interpolation {
                    dollar_count: dollar_advanced,
                    ..Interpolation::default()
                };

                if is_verbatim || lookahead == i32::from(b'@') {
                    if lookahead == i32::from(b'@') {
                        lexer.advance(false);
                        is_verbatim = true;
                    }
                    lexer.set_result_symbol(INTERPOLATION_VERBATIM_START as u16);
                    interpolation.string_type = VERBATIM;
                }

                lexer.mark_end();
                lexer.advance(false);
                lookahead = lexer.lookahead();

                if lookahead == i32::from(b'"') && !is_verbatim {
                    lexer.advance(false);
                    lookahead = lexer.lookahead();
                    if lookahead == i32::from(b'"') {
                        lexer.set_result_symbol(INTERPOLATION_RAW_START as u16);
                        interpolation.string_type |= RAW;
                        self.interpolation_stack.push(interpolation);
                    }
                    // One or three quotes push an interpolation; two quotes
                    // are an empty string and leave the stack unchanged.
                } else {
                    interpolation.string_type |= REGULAR;
                    self.interpolation_stack.push(interpolation);
                }

                return true;
            }
        }

        if valid_symbols[INTERPOLATION_START_QUOTE]
            && let Some(current) = self.interpolation_stack.last_mut()
        {
            if current.is_verbatim() || current.is_regular() {
                if lookahead == i32::from(b'"') {
                    lexer.advance(false);
                    current.quote_count = current.quote_count.wrapping_add(1);
                }
            } else {
                while lookahead == i32::from(b'"') {
                    lexer.advance(false);
                    lookahead = lexer.lookahead();
                    current.quote_count = current.quote_count.wrapping_add(1);
                }
            }

            lexer.set_result_symbol(INTERPOLATION_START_QUOTE as u16);
            return current.quote_count > 0;
        }

        if valid_symbols[INTERPOLATION_END_QUOTE]
            && let Some(current) = self.interpolation_stack.last()
        {
            while lookahead == i32::from(b'"') {
                lexer.advance(false);
                lookahead = lexer.lookahead();
                quote_count = quote_count.wrapping_add(1);
            }

            if quote_count == current.quote_count {
                lexer.set_result_symbol(INTERPOLATION_END_QUOTE as u16);
                self.interpolation_stack.pop();
                return true;
            }

            did_advance = quote_count > 0;
        }

        if valid_symbols[INTERPOLATION_OPEN_BRACE]
            && let Some(current) = self.interpolation_stack.last_mut()
        {
            while lookahead == i32::from(b'{') && brace_advanced < current.dollar_count {
                lexer.advance(false);
                lookahead = lexer.lookahead();
                brace_advanced = brace_advanced.wrapping_add(1);
            }

            if brace_advanced > 0
                && brace_advanced == current.dollar_count
                && (brace_advanced == 0 || lookahead != i32::from(b'{'))
            {
                current.open_brace_count = brace_advanced;
                lexer.set_result_symbol(INTERPOLATION_OPEN_BRACE as u16);
                return true;
            }
        }

        if valid_symbols[INTERPOLATION_CLOSE_BRACE]
            && let Some(current) = self.interpolation_stack.last_mut()
        {
            // This counter shadows (and does not reset) the earlier one in C.
            let mut brace_advanced = 0u8;

            while is_space(lookahead) {
                lexer.advance(false);
                lookahead = lexer.lookahead();
            }

            while lookahead == i32::from(b'}') {
                lexer.advance(false);
                lookahead = lexer.lookahead();
                brace_advanced = brace_advanced.wrapping_add(1);

                if brace_advanced == current.open_brace_count {
                    current.open_brace_count = 0;
                    lexer.set_result_symbol(INTERPOLATION_CLOSE_BRACE as u16);
                    return true;
                }
            }

            return false;
        }

        if valid_symbols[INTERPOLATION_STRING_CONTENT]
            && let Some(current) = self.interpolation_stack.last()
        {
            lexer.set_result_symbol(INTERPOLATION_STRING_CONTENT as u16);

            while lookahead != 0 {
                // Check raw before verbatim, and verbatim before regular:
                // string_type can contain more than one bit.
                if current.is_raw() {
                    if lookahead == i32::from(b'"') {
                        lexer.mark_end();
                        lexer.advance(false);
                        lookahead = lexer.lookahead();
                        if lookahead == i32::from(b'"') {
                            lexer.advance(false);
                            lookahead = lexer.lookahead();
                            let mut quote_advanced = 2u8;
                            while lookahead == i32::from(b'"') {
                                quote_advanced = quote_advanced.wrapping_add(1);
                                lexer.advance(false);
                                lookahead = lexer.lookahead();
                            }
                            if quote_advanced == current.quote_count {
                                return did_advance;
                            }
                        }
                    }

                    if lookahead == i32::from(b'{') {
                        lexer.mark_end();

                        while lookahead == i32::from(b'{')
                            && brace_advanced < current.open_brace_count
                        {
                            lexer.advance(false);
                            lookahead = lexer.lookahead();
                            brace_advanced = brace_advanced.wrapping_add(1);
                        }

                        if brace_advanced == current.open_brace_count
                            && (brace_advanced == 0 || lookahead != i32::from(b'{'))
                        {
                            return did_advance;
                        }
                    }
                } else if current.is_verbatim() {
                    if lookahead == i32::from(b'"') {
                        lexer.mark_end();
                        lexer.advance(false);
                        lookahead = lexer.lookahead();
                        if lookahead == i32::from(b'"') {
                            lexer.advance(false);
                            lookahead = lexer.lookahead();
                            continue;
                        }
                        return did_advance;
                    }

                    if lookahead == i32::from(b'{') {
                        lexer.mark_end();

                        while lookahead == i32::from(b'{')
                            && brace_advanced < current.open_brace_count
                        {
                            lexer.advance(false);
                            lookahead = lexer.lookahead();
                            brace_advanced = brace_advanced.wrapping_add(1);
                        }

                        if brace_advanced == current.open_brace_count
                            && (brace_advanced == 0 || lookahead != i32::from(b'{'))
                        {
                            return did_advance;
                        }
                    }
                } else if current.is_regular() {
                    if lookahead == i32::from(b'\\')
                        || lookahead == i32::from(b'\n')
                        || lookahead == i32::from(b'"')
                    {
                        lexer.mark_end();
                        return did_advance;
                    }

                    if lookahead == i32::from(b'{') {
                        lexer.mark_end();

                        while lookahead == i32::from(b'{')
                            && brace_advanced < current.open_brace_count
                        {
                            lexer.advance(false);
                            lookahead = lexer.lookahead();
                            brace_advanced = brace_advanced.wrapping_add(1);
                        }

                        if brace_advanced == current.open_brace_count
                            && (brace_advanced == 0 || lookahead != i32::from(b'{'))
                        {
                            return did_advance;
                        }
                    }
                }

                if lookahead != i32::from(b'{') {
                    brace_advanced = 0;
                }
                lexer.advance(false);
                lookahead = lexer.lookahead();
                did_advance = true;
            }

            lexer.mark_end();
            return did_advance;
        }

        false
    }
}

impl ExternalScanner for Scanner {
    fn scan(&mut self, lexer: &mut dyn Lexer, valid_symbols: &[bool]) -> bool {
        let valid_symbols = valid_symbols
            .first_chunk::<{ RAW_STRING_CONTENT + 1 }>()
            .expect("C# external token flags");
        if valid_symbols[OPT_SEMI] {
            // The C scanner suppresses all external tokens during recovery.
            if valid_symbols[INTERPOLATION_REGULAR_START] {
                return false;
            }
            lexer.set_result_symbol(OPT_SEMI as u16);
            if lexer.lookahead() == i32::from(b';') {
                lexer.advance(false);
            }
            return true;
        }

        let mut lookahead = lexer.lookahead();
        // With no active interpolation, only raw strings and interpolation
        // prefixes can consume input. Reject ordinary code without entering
        // the string scanner, but retain the C scanner's whitespace advances.
        if self.interpolation_stack.is_empty()
            && !valid_symbols[RAW_STRING_END]
            && !valid_symbols[RAW_STRING_CONTENT]
        {
            if !(valid_symbols[RAW_STRING_START]
                || valid_symbols[INTERPOLATION_REGULAR_START]
                || valid_symbols[INTERPOLATION_VERBATIM_START]
                || valid_symbols[INTERPOLATION_RAW_START])
            {
                return false;
            }
            while is_space(lookahead) {
                lexer.advance(true);
                lookahead = lexer.lookahead();
            }
            if !matches!(lookahead, 34 | 36 | 64) {
                return false;
            }
        }
        self.scan_string(lexer, valid_symbols, lookahead)
    }

    fn serialize(&mut self, buffer: &mut [u8]) -> usize {
        if self.interpolation_stack.len() * 4 + 2 > SERIALIZATION_BUFFER_SIZE {
            return 0;
        }

        buffer[0] = self.quote_count;
        buffer[1] = self.interpolation_stack.len() as u8;
        let mut size = 2;
        for interpolation in &self.interpolation_stack {
            buffer[size] = interpolation.dollar_count;
            buffer[size + 1] = interpolation.open_brace_count;
            buffer[size + 2] = interpolation.quote_count;
            buffer[size + 3] = interpolation.string_type;
            size += 4;
        }
        size
    }

    fn deserialize(&mut self, buffer: &[u8]) {
        self.quote_count = 0;
        self.interpolation_stack.clear();
        let mut size = 0;

        if !buffer.is_empty() {
            self.quote_count = buffer[0];
            let interpolation_count = usize::from(buffer[1]);
            size = 2;
            self.interpolation_stack.reserve(interpolation_count);
            for _ in 0..interpolation_count {
                self.interpolation_stack.push(Interpolation {
                    dollar_count: buffer[size],
                    open_brace_count: buffer[size + 1],
                    quote_count: buffer[size + 2],
                    string_type: buffer[size + 3],
                });
                size += 4;
            }
        }

        assert_eq!(size, buffer.len());
    }
}

/// Creates a scanner (C's `tree_sitter_c_sharp_external_scanner_create`).
/// Dropping the scanner also drops its interpolation stack.
pub(crate) fn create() -> Box<dyn ExternalScanner> {
    Box::new(Scanner::default())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

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
        lookahead_calls: Cell<usize>,
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
                lookahead_calls: Cell::new(0),
            }
        }

        fn token(&self) -> &'a str {
            &self.input[self.start..self.end.unwrap_or(self.position)]
        }
    }

    impl Lexer for TestLexer<'_> {
        fn lookahead(&self) -> i32 {
            self.lookahead_calls.set(self.lookahead_calls.get() + 1);
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
            panic!("the C# scanner does not query columns")
        }

        fn is_at_included_range_start(&self) -> bool {
            panic!("the C# scanner does not query included ranges")
        }

        fn eof(&self) -> bool {
            panic!("the C# scanner tests lookahead, not eof")
        }
    }

    fn scan<'a>(
        scanner: &mut dyn ExternalScanner,
        input: &'a str,
        tokens: &[usize],
    ) -> (bool, TestLexer<'a>) {
        let mut valid = [false; RAW_STRING_CONTENT + 1];
        for &token in tokens {
            valid[token] = true;
        }
        let mut lexer = TestLexer::new(input);
        let accepted = scanner.scan(&mut lexer, &valid);
        (accepted, lexer)
    }

    fn snapshot(scanner: &mut dyn ExternalScanner) -> Vec<u8> {
        let mut buffer = [0; SERIALIZATION_BUFFER_SIZE];
        let length = scanner.serialize(&mut buffer);
        buffer[..length].to_vec()
    }

    fn interpolation(string_type: u8, quote_count: u8, open_brace_count: u8) -> Scanner {
        Scanner {
            quote_count: 0,
            interpolation_stack: vec![Interpolation {
                dollar_count: 2,
                open_brace_count,
                quote_count,
                string_type,
            }],
        }
    }

    #[test]
    fn empty_stack_rejection_preserves_whitespace_for_every_flag_set() {
        let input = " \tname";
        for mask in 0..(1 << (RAW_STRING_CONTENT + 1)) {
            let valid = std::array::from_fn::<_, { RAW_STRING_CONTENT + 1 }, _>(|bit| {
                mask & (1 << bit) != 0
            });
            let mut scanner = Scanner {
                quote_count: 3,
                ..Scanner::default()
            };
            let mut lexer = TestLexer::new(input);
            let accepted = scanner.scan(&mut lexer, &valid);
            let mut events = Vec::new();
            let mut start = 0;
            let mut position = 0;
            let expected = if valid[OPT_SEMI] {
                if valid[INTERPOLATION_REGULAR_START] {
                    false
                } else {
                    events.push(Event::Symbol(OPT_SEMI as u16));
                    true
                }
            } else if valid[RAW_STRING_CONTENT] {
                for at in 0..input.len() {
                    events.push(Event::Advance {
                        at,
                        skip: valid[RAW_STRING_START] && at < 2,
                    });
                }
                start = if valid[RAW_STRING_START] { 2 } else { 0 };
                position = input.len();
                events.push(Event::Mark(position));
                events.push(Event::Symbol(RAW_STRING_CONTENT as u16));
                true
            } else {
                if valid[RAW_STRING_START]
                    || valid[INTERPOLATION_REGULAR_START]
                    || valid[INTERPOLATION_VERBATIM_START]
                    || valid[INTERPOLATION_RAW_START]
                {
                    for at in 0..2 {
                        events.push(Event::Advance { at, skip: true });
                    }
                    start = 2;
                    position = 2;
                }
                false
            };
            assert_eq!(accepted, expected, "mask={mask}");
            assert_eq!(lexer.events, events, "mask={mask}");
            assert_eq!(lexer.start, start, "mask={mask}");
            assert_eq!(lexer.position, position, "mask={mask}");
            assert_eq!(snapshot(&mut scanner), [3, 0]);
        }
    }

    #[test]
    fn failed_prefix_probe_reads_lookahead_once_per_position() {
        for input in ["identifier", " \t\nidentifier", "\u{2003}identifier"] {
            let (accepted, lexer) = scan(
                &mut Scanner::default(),
                input,
                &[
                    RAW_STRING_START,
                    INTERPOLATION_REGULAR_START,
                    INTERPOLATION_VERBATIM_START,
                    INTERPOLATION_RAW_START,
                ],
            );
            assert!(!accepted);
            assert_eq!(lexer.lookahead_calls.get(), lexer.position + 1);
        }
    }

    #[test]
    fn wrapping_raw_probe_preserves_interpolation_fallthrough() {
        for count in [0usize, 1, 2, 3, 255, 256, 257, 258, 259] {
            let mut scanner = Scanner::default();
            let input = format!("{} \t@$\"x", "\"".repeat(count));
            let (accepted, lexer) = scan(
                &mut scanner,
                &input,
                &[RAW_STRING_START, INTERPOLATION_REGULAR_START],
            );
            match count as u8 {
                0 => {
                    assert!(accepted);
                    assert_eq!(lexer.symbol, INTERPOLATION_VERBATIM_START as u16);
                    assert_eq!(lexer.position, count + 5);
                    assert_eq!(lexer.end, Some(count + 4));
                    assert_eq!(snapshot(&mut scanner), [0, 1, 1, 0, 0, VERBATIM | REGULAR]);
                }
                1 | 2 => {
                    assert!(!accepted);
                    // Whitespace and @ are still consumed, but the previous
                    // quote count prevents the following $ from advancing.
                    assert_eq!(lexer.position, count + 3);
                    assert_eq!(snapshot(&mut scanner), [0, 0]);
                }
                _ => {
                    assert!(accepted);
                    assert_eq!(lexer.symbol, RAW_STRING_START as u16);
                    assert_eq!(lexer.position, count);
                    assert_eq!(snapshot(&mut scanner), [count as u8, 0]);
                }
            }
        }
    }

    #[test]
    fn serialization_layout_roundtrip_and_reset() {
        let mut scanner = create();
        assert_eq!(snapshot(scanner.as_mut()), [0, 0]);
        let bytes = [
            255,
            2,
            128,
            129,
            130,
            RAW,
            254,
            253,
            252,
            VERBATIM | REGULAR,
        ];
        scanner.deserialize(&bytes);
        assert_eq!(snapshot(scanner.as_mut()), bytes);
        scanner.deserialize(&[7, 1, 3, 2, 1, REGULAR]);
        assert_eq!(snapshot(scanner.as_mut()), [7, 1, 3, 2, 1, REGULAR]);
        scanner.deserialize(&[]);
        assert_eq!(snapshot(scanner.as_mut()), [0, 0]);
    }

    #[test]
    fn serialization_limit_is_checked_before_writing() {
        let mut scanner = Scanner {
            quote_count: 3,
            interpolation_stack: vec![Interpolation::default(); 255],
        };
        let mut buffer = [0xaa; SERIALIZATION_BUFFER_SIZE];
        assert_eq!(scanner.serialize(&mut buffer), 1022);
        assert_eq!(&buffer[..2], &[3, 255]);
        assert_eq!(&buffer[1022..], &[0xaa, 0xaa]);
        let mut restored = Scanner::default();
        restored.deserialize(&buffer[..1022]);
        assert_eq!(restored.interpolation_stack, scanner.interpolation_stack);

        scanner.interpolation_stack.push(Interpolation::default());
        buffer.fill(0xaa);
        assert_eq!(scanner.serialize(&mut buffer), 0);
        assert_eq!(buffer, [0xaa; SERIALIZATION_BUFFER_SIZE]);
    }

    #[test]
    fn error_recovery_and_optional_semicolon() {
        let mut scanner = interpolation(RAW, 3, 2);
        let before = snapshot(&mut scanner);
        let mut lexer = TestLexer::new(";");
        assert!(!scanner.scan(&mut lexer, &[true; RAW_STRING_CONTENT + 1]));
        assert!(lexer.events.is_empty());
        assert_eq!(snapshot(&mut scanner), before);

        for (input, expected) in [(";x", ";"), (" ;", ""), ("", ""), ("x", "")] {
            let (accepted, lexer) = scan(&mut scanner, input, &[OPT_SEMI]);
            assert!(accepted);
            assert_eq!(lexer.token(), expected);
            assert_eq!(lexer.events.first(), Some(&Event::Symbol(OPT_SEMI as u16)));
            assert_eq!(lexer.end, None);
            assert_eq!(snapshot(&mut scanner), before);
        }
    }

    #[test]
    fn raw_string_start_and_end_use_wrapping_counts_and_c_whitespace() {
        let whitespace = " \t\n\r\u{b}\u{c}";
        for count in [1usize, 2, 3, 4, 255, 256, 257, 258, 259] {
            let mut scanner = Scanner::default();
            let quotes = "\"".repeat(count);
            let input = format!("{whitespace}{quotes}x");
            let (accepted, lexer) = scan(&mut scanner, &input, &[RAW_STRING_START]);
            assert_eq!(accepted, count as u8 >= 3);
            assert_eq!(lexer.start, whitespace.len());
            assert_eq!(lexer.position, whitespace.len() + count);
            assert_eq!(lexer.end, None);
            if accepted {
                assert_eq!(scanner.quote_count, count as u8);
                assert_eq!(lexer.result_symbol(), RAW_STRING_START as u16);
                let (accepted, lexer) = scan(&mut scanner, &quotes, &[RAW_STRING_END]);
                assert!(accepted);
                assert_eq!(lexer.token(), quotes);
                assert_eq!(lexer.result_symbol(), RAW_STRING_END as u16);
                assert_eq!(scanner.quote_count, 0);
            }
        }
        for space in ['\u{85}', '\u{a0}', '\u{2003}', '\u{2028}'] {
            let input = format!("{space}\"\"\"");
            let (accepted, lexer) = scan(&mut Scanner::default(), &input, &[RAW_STRING_START]);
            assert!(!accepted);
            assert!(lexer.events.is_empty());
        }
    }

    #[test]
    fn raw_content_marks_before_terminator_but_advances_past_it() {
        let mut scanner = Scanner {
            quote_count: 3,
            ..Scanner::default()
        };
        let (accepted, lexer) = scan(&mut scanner, "a\"\"\"x", &[RAW_STRING_CONTENT]);
        assert!(accepted);
        assert_eq!(lexer.token(), "a");
        assert_eq!(lexer.position, 4);
        assert_eq!(
            lexer.events,
            [
                Event::Advance { at: 0, skip: false },
                Event::Mark(1),
                Event::Advance { at: 1, skip: false },
                Event::Advance { at: 2, skip: false },
                Event::Advance { at: 3, skip: false },
                Event::Symbol(RAW_STRING_CONTENT as u16),
            ]
        );
        // The content branch accepts even an empty token.
        for input in ["", "\"\"\"", "\0unread"] {
            let (accepted, lexer) = scan(&mut scanner, input, &[RAW_STRING_CONTENT]);
            assert!(accepted);
            assert_eq!(lexer.token(), "");
        }
    }

    #[test]
    fn unmatched_raw_quotes_fall_through_and_advance_even_at_eof() {
        let mut scanner = Scanner {
            quote_count: 3,
            ..Scanner::default()
        };
        let (accepted, lexer) = scan(&mut scanner, "\"\"", &[RAW_STRING_CONTENT]);
        assert!(accepted);
        assert_eq!(lexer.token(), "\"\"");
        assert_eq!(
            lexer.events,
            [
                Event::Mark(0),
                Event::Advance { at: 0, skip: false },
                Event::Advance { at: 1, skip: false },
                Event::Advance { at: 2, skip: false },
                Event::Mark(2),
                Event::Symbol(RAW_STRING_CONTENT as u16),
            ]
        );
        let (accepted, lexer) = scan(
            &mut scanner,
            "\"\"abc\"\"\"",
            &[RAW_STRING_END, RAW_STRING_CONTENT],
        );
        assert!(accepted);
        assert_eq!(lexer.token(), "\"\"abc");
        assert_eq!(scanner.quote_count, 3);
    }

    #[test]
    fn interpolation_prefixes_mark_before_quotes_and_classify_the_stack() {
        for (input, token, dollars, flags, end, advanced) in [
            ("$\"x", INTERPOLATION_REGULAR_START, 1, REGULAR, 1, 2),
            ("$\"\"", INTERPOLATION_REGULAR_START, 1, 0, 1, 3),
            ("$\"\"\"x", INTERPOLATION_RAW_START, 1, RAW, 1, 3),
            ("$$\"\"\"x", INTERPOLATION_RAW_START, 2, RAW, 2, 4),
            (
                "$@\"x",
                INTERPOLATION_VERBATIM_START,
                1,
                VERBATIM | REGULAR,
                2,
                3,
            ),
            (
                "@$\"x",
                INTERPOLATION_VERBATIM_START,
                1,
                VERBATIM | REGULAR,
                2,
                3,
            ),
            (
                "$@x",
                INTERPOLATION_VERBATIM_START,
                1,
                VERBATIM | REGULAR,
                2,
                3,
            ),
        ] {
            let mut scanner = Scanner::default();
            // C can return a start kind other than the one flagged valid.
            let (accepted, lexer) = scan(&mut scanner, input, &[INTERPOLATION_REGULAR_START]);
            assert!(accepted, "{input:?}");
            assert_eq!(lexer.result_symbol(), token as u16);
            assert_eq!(lexer.end, Some(end));
            assert_eq!(lexer.position, advanced);
            if flags == 0 {
                assert!(scanner.interpolation_stack.is_empty());
            } else {
                assert_eq!(snapshot(&mut scanner), [0, 1, dollars, 0, 0, flags]);
            }
        }
    }

    #[test]
    fn interpolation_prefix_call_order_and_dollar_wrap() {
        let mut scanner = Scanner::default();
        let (accepted, lexer) = scan(&mut scanner, " $@\"x", &[INTERPOLATION_VERBATIM_START]);
        assert!(accepted);
        assert_eq!(lexer.token(), "$@");
        assert_eq!(
            lexer.events,
            [
                Event::Advance { at: 0, skip: true },
                Event::Advance { at: 1, skip: false },
                Event::Symbol(INTERPOLATION_REGULAR_START as u16),
                Event::Advance { at: 2, skip: false },
                Event::Symbol(INTERPOLATION_VERBATIM_START as u16),
                Event::Mark(3),
                Event::Advance { at: 3, skip: false },
            ]
        );
        for count in [255usize, 256, 257] {
            let mut scanner = Scanner::default();
            let input = format!("{}\"x", "$".repeat(count));
            let (accepted, lexer) = scan(&mut scanner, &input, &[INTERPOLATION_REGULAR_START]);
            assert_eq!(accepted, count as u8 > 0);
            if accepted {
                assert_eq!(scanner.interpolation_stack[0].dollar_count, count as u8);
                assert_eq!(lexer.end, Some(count));
            } else {
                assert_eq!(lexer.position, count);
                assert!(scanner.interpolation_stack.is_empty());
            }
        }
    }

    #[test]
    fn starting_quotes_update_the_existing_wrapping_count() {
        for flags in [REGULAR, VERBATIM, VERBATIM | REGULAR, RAW, 0] {
            let mut scanner = interpolation(flags, 0, 0);
            let (accepted, lexer) = scan(&mut scanner, "\"\"\"x", &[INTERPOLATION_START_QUOTE]);
            assert!(accepted);
            let count = if flags & (REGULAR | VERBATIM) != 0 {
                1
            } else {
                3
            };
            assert_eq!(lexer.position, count as usize);
            assert_eq!(scanner.interpolation_stack[0].quote_count, count);
            let (accepted, lexer) = scan(&mut scanner, "x", &[INTERPOLATION_START_QUOTE]);
            assert!(accepted); // A previous nonzero count is sufficient.
            assert_eq!(lexer.position, 0);
        }
        let mut scanner = interpolation(REGULAR, 255, 0);
        let (accepted, lexer) = scan(&mut scanner, "\"", &[INTERPOLATION_START_QUOTE]);
        assert!(!accepted);
        assert_eq!(scanner.interpolation_stack[0].quote_count, 0);
        assert_eq!(lexer.result_symbol(), INTERPOLATION_START_QUOTE as u16);
        let mut scanner = interpolation(RAW, 255, 0);
        let (accepted, _) = scan(&mut scanner, "\"\"\"", &[INTERPOLATION_START_QUOTE]);
        assert!(accepted);
        assert_eq!(scanner.interpolation_stack[0].quote_count, 2);
    }

    #[test]
    fn nested_interpolations_pop_only_after_matching_quotes() {
        let mut scanner = Scanner::default();
        assert!(scan(&mut scanner, "$$\"\"\"", &[INTERPOLATION_RAW_START]).0);
        assert!(scan(&mut scanner, "\"\"\"", &[INTERPOLATION_START_QUOTE]).0);
        assert!(scan(&mut scanner, "{{x", &[INTERPOLATION_OPEN_BRACE]).0);
        assert_eq!(snapshot(&mut scanner), [0, 1, 2, 2, 3, RAW]);
        assert!(scan(&mut scanner, "$@\"", &[INTERPOLATION_VERBATIM_START]).0);
        assert!(scan(&mut scanner, "\"", &[INTERPOLATION_START_QUOTE]).0);
        assert_eq!(
            snapshot(&mut scanner),
            [0, 2, 2, 2, 3, RAW, 1, 0, 1, VERBATIM | REGULAR]
        );
        assert!(!scan(&mut scanner, "\"\"", &[INTERPOLATION_END_QUOTE]).0);
        assert_eq!(scanner.interpolation_stack.len(), 2);
        assert!(scan(&mut scanner, "\"", &[INTERPOLATION_END_QUOTE]).0);
        assert_eq!(snapshot(&mut scanner), [0, 1, 2, 2, 3, RAW]);
        let (accepted, lexer) = scan(&mut scanner, " \t}}}x", &[INTERPOLATION_CLOSE_BRACE]);
        assert!(accepted);
        assert_eq!(lexer.token(), " \t}}");
        assert_eq!(lexer.start, 0); // Closing-brace whitespace is not skipped.
        assert_eq!(snapshot(&mut scanner), [0, 1, 2, 0, 3, RAW]);
        assert!(scan(&mut scanner, "\"\"\"", &[INTERPOLATION_END_QUOTE]).0);
        assert_eq!(snapshot(&mut scanner), [0, 0]);
    }

    #[test]
    fn braces_require_exact_open_count_and_closing_count_wraps() {
        for (input, accepted, advanced) in [("{x", false, 1), ("{{x", true, 2), ("{{{x", false, 2)]
        {
            let mut scanner = interpolation(RAW, 3, 0);
            let (result, lexer) = scan(&mut scanner, input, &[INTERPOLATION_OPEN_BRACE]);
            assert_eq!(result, accepted);
            assert_eq!(lexer.position, advanced);
            assert_eq!(
                scanner.interpolation_stack[0].open_brace_count,
                if accepted { 2 } else { 0 }
            );
        }
        let mut scanner = interpolation(RAW, 3, 0);
        let input = "}".repeat(256);
        let (accepted, lexer) = scan(&mut scanner, &input, &[INTERPOLATION_CLOSE_BRACE]);
        assert!(accepted);
        assert_eq!(lexer.position, 256);
        let (accepted, lexer) = scan(
            &mut scanner,
            " text",
            &[INTERPOLATION_CLOSE_BRACE, INTERPOLATION_STRING_CONTENT],
        );
        assert!(!accepted); // Closing braces return false without trying content.
        assert_eq!(lexer.position, 1);
        assert_eq!(lexer.start, 0);
    }

    #[test]
    fn regular_content_boundaries_and_nul() {
        for boundary in ["\\", "\n", "\"", "{"] {
            let mut scanner = interpolation(REGULAR, 1, 0);
            let (accepted, lexer) = scan(&mut scanner, boundary, &[INTERPOLATION_STRING_CONTENT]);
            assert!(!accepted);
            assert_eq!(lexer.position, 0);
            assert_eq!(lexer.end, Some(0));
            let input = format!("a{boundary}b");
            let (accepted, lexer) = scan(&mut scanner, &input, &[INTERPOLATION_STRING_CONTENT]);
            assert!(accepted);
            assert_eq!(lexer.token(), "a");
            assert_eq!(lexer.position, 1);
        }
        let (accepted, lexer) = scan(
            &mut interpolation(REGULAR, 1, 0),
            "a\0unread",
            &[INTERPOLATION_STRING_CONTENT],
        );
        assert!(accepted);
        assert_eq!(lexer.token(), "a");
        assert_eq!(lexer.position, 1);
    }

    #[test]
    fn verbatim_escaped_quotes_do_not_set_did_advance() {
        let mut scanner = interpolation(VERBATIM | REGULAR, 1, 0);
        let (accepted, lexer) = scan(&mut scanner, "\"\"{x", &[INTERPOLATION_STRING_CONTENT]);
        assert!(!accepted);
        assert_eq!(
            lexer.events,
            [
                Event::Symbol(INTERPOLATION_STRING_CONTENT as u16),
                Event::Mark(0),
                Event::Advance { at: 0, skip: false },
                Event::Advance { at: 1, skip: false },
                Event::Mark(2),
            ]
        );
        let (accepted, lexer) = scan(&mut scanner, "a\"\"\"x", &[INTERPOLATION_STRING_CONTENT]);
        assert!(accepted);
        assert_eq!(lexer.token(), "a\"\"");
        assert_eq!(lexer.position, 4);
        let (accepted, lexer) = scan(&mut scanner, "\\\n\"", &[INTERPOLATION_STRING_CONTENT]);
        assert!(accepted);
        assert_eq!(lexer.token(), "\\\n");
    }

    #[test]
    fn raw_interpolation_content_quote_boundary_and_wrap() {
        for flags in [RAW, RAW | VERBATIM | REGULAR] {
            let mut scanner = interpolation(flags, 3, 0);
            for prefix in ["", "abc"] {
                for count in [3, 259] {
                    let input = format!("{prefix}{}x", "\"".repeat(count));
                    let (accepted, lexer) =
                        scan(&mut scanner, &input, &[INTERPOLATION_STRING_CONTENT]);
                    assert_eq!(accepted, !prefix.is_empty());
                    assert_eq!(lexer.token(), prefix);
                    assert_eq!(lexer.position, prefix.len() + count);
                }
            }
            let (accepted, lexer) = scan(&mut scanner, "a\"{x", &[INTERPOLATION_STRING_CONTENT]);
            assert!(accepted);
            assert_eq!(lexer.token(), "a\"");
            assert_eq!(lexer.position, 2);
        }
    }

    #[test]
    fn content_brace_limit_uses_open_count_not_dollar_count() {
        for flags in [REGULAR, VERBATIM | REGULAR, RAW] {
            let mut scanner = interpolation(flags, 3, 1);
            let (accepted, lexer) = scan(&mut scanner, "a{x", &[INTERPOLATION_STRING_CONTENT]);
            assert!(accepted);
            assert_eq!(lexer.token(), "a");
            assert_eq!(lexer.position, 2);
            let (accepted, lexer) = scan(&mut scanner, "{{x", &[INTERPOLATION_STRING_CONTENT]);
            assert!(accepted);
            assert_eq!(lexer.token(), "{{x");
        }
    }

    #[test]
    fn failed_earlier_branches_preserve_local_counts() {
        let mut scanner = interpolation(REGULAR, 1, 0);
        // A quote consumed by raw-start still counts toward interpolation-end.
        let (accepted, lexer) = scan(
            &mut scanner,
            "\"x",
            &[RAW_STRING_START, INTERPOLATION_END_QUOTE],
        );
        assert!(accepted);
        assert_eq!(lexer.result_symbol(), INTERPOLATION_END_QUOTE as u16);
        assert!(scanner.interpolation_stack.is_empty());

        // The nonzero raw-start quote count prevents consuming the dollar.
        let (accepted, lexer) = scan(
            &mut scanner,
            "\"$\"x",
            &[RAW_STRING_START, INTERPOLATION_REGULAR_START],
        );
        assert!(!accepted);
        assert_eq!(lexer.position, 1);

        // Failed end quotes set did_advance, so immediate content boundaries
        // can succeed without any further advance.
        let mut scanner = interpolation(REGULAR, 1, 0);
        let (accepted, lexer) = scan(
            &mut scanner,
            "\"\"\\",
            &[INTERPOLATION_END_QUOTE, INTERPOLATION_STRING_CONTENT],
        );
        assert!(accepted);
        assert_eq!(lexer.token(), "\"\"");
        assert_eq!(lexer.position, 2);

        // Failed open braces leave their local counter set for content.
        let mut scanner = interpolation(RAW, 3, 0);
        let (accepted, lexer) = scan(
            &mut scanner,
            "{{{x",
            &[INTERPOLATION_OPEN_BRACE, INTERPOLATION_STRING_CONTENT],
        );
        assert!(accepted);
        assert_eq!(lexer.token(), "{{{x");
    }
}
