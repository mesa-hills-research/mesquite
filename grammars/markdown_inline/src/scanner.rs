//! The Markdown inline external scanner, translated from `scanner.c`.

use ts_port_tables::{ExternalScanner, Lexer};

// The order is the external token order in grammar.js and scanner.c.
#[derive(Clone, Copy)]
#[repr(u16)]
enum Token {
    Error,
    TriggerError,
    CodeSpanStart,
    CodeSpanClose,
    EmphasisOpenStar,
    EmphasisOpenUnderscore,
    EmphasisCloseStar,
    EmphasisCloseUnderscore,
    LastTokenWhitespace,
    LastTokenPunctuation,
    StrikethroughOpen,
    StrikethroughClose,
    LatexSpanStart,
    LatexSpanClose,
    UnclosedSpan,
}

const STATE_EMPHASIS_DELIMITER_IS_OPEN: u8 = 1 << 2;

/// Punctuation as defined by the Markdown spec, not Unicode punctuation.
fn is_punctuation(chr: i32) -> bool {
    matches!(chr, 0x21..=0x2f | 0x3a..=0x40 | 0x5b..=0x60 | 0x7b..=0x7e)
}

/// The scanner's state (C's `payload`).
#[derive(Default)]
pub(crate) struct Scanner {
    state: u8,
    code_span_delimiter_length: u8,
    latex_span_delimiter_length: u8,
    num_emphasis_delimiters_left: u8,
}

// Keep speculative span lookahead out of the common no-delimiter scan frame.
// Select token ids here rather than carrying them as additional scan arguments;
// both delimiter handlers then fit the register argument budget for a tail call.
#[inline(never)]
fn parse_leaf_delimiter(
    lexer: &mut dyn Lexer,
    delimiter_length: &mut u8,
    valid_symbols: &[bool; 15],
    delimiter: u8,
) -> bool {
    let (open_token, close_token) = if delimiter == b'`' {
        (Token::CodeSpanStart, Token::CodeSpanClose)
    } else {
        (Token::LatexSpanStart, Token::LatexSpanClose)
    };
    // Dispatch already checked the first delimiter. Keep one lookahead per
    // position; the closing-run search can reuse the final opening lookahead.
    let mut level = 0u8;
    let mut lookahead;
    loop {
        lexer.advance(false);
        level = level.wrapping_add(1);
        lookahead = lexer.lookahead();
        if lookahead != i32::from(delimiter) {
            break;
        }
    }
    lexer.mark_end();
    if level == *delimiter_length && valid_symbols[close_token as usize] {
        *delimiter_length = 0;
        lexer.set_result_symbol(close_token as u16);
        return true;
    }
    if valid_symbols[open_token as usize] {
        // Look for a matching closing run, but leave the token end at the end
        // of the opening run. The opening count wraps; closing runs use size_t
        // in C. A wrapped zero already matches the empty run at this position.
        if level == 0 || find_closing_run(lexer, lookahead, delimiter, usize::from(level)) {
            *delimiter_length = level;
            lexer.set_result_symbol(open_token as u16);
            return true;
        }
        if valid_symbols[Token::UnclosedSpan as usize] {
            lexer.set_result_symbol(Token::UnclosedSpan as u16);
            return true;
        }
    }
    false
}

/// Alternate ordinary content and delimiter runs, instead of resetting and
/// comparing a closing counter on every ordinary character.
fn find_closing_run(
    lexer: &mut dyn Lexer,
    mut lookahead: i32,
    delimiter: u8,
    level: usize,
) -> bool {
    loop {
        while lookahead != i32::from(delimiter) {
            // Zero can be an embedded NUL: only eof() distinguishes it from
            // end-of-input. Nonzero content needs no virtual EOF query.
            if lookahead == 0 && lexer.eof() {
                return false;
            }
            lexer.advance(false);
            lookahead = lexer.lookahead();
        }
        let mut close_level = 0usize;
        loop {
            close_level = close_level.wrapping_add(1);
            lexer.advance(false);
            lookahead = lexer.lookahead();
            if lookahead != i32::from(delimiter) {
                break;
            }
        }
        if close_level == level {
            return true;
        }
    }
}

impl Scanner {
    // C's parse_star, parse_underscore, and parse_tilde have identical logic,
    // differing only in the character and the two token ids. Select the token
    // pair here so the common dispatch remains small, including its rejection
    // path. Keep this handler outlined just like speculative span scanning.
    #[inline(never)]
    fn parse_emphasis_delimiter(
        &mut self,
        lexer: &mut dyn Lexer,
        valid_symbols: &[bool; 15],
        delimiter: u8,
    ) -> bool {
        let (open_token, close_token) = match delimiter {
            b'*' => (Token::EmphasisOpenStar, Token::EmphasisCloseStar),
            b'_' => (
                Token::EmphasisOpenUnderscore,
                Token::EmphasisCloseUnderscore,
            ),
            _ => (Token::StrikethroughOpen, Token::StrikethroughClose),
        };
        lexer.advance(false);
        if self.num_emphasis_delimiters_left > 0 {
            if self.state & STATE_EMPHASIS_DELIMITER_IS_OPEN != 0
                && valid_symbols[open_token as usize]
            {
                // C clears the opening flag even when emitting an opener.
                self.state &= !STATE_EMPHASIS_DELIMITER_IS_OPEN;
                lexer.set_result_symbol(open_token as u16);
                self.num_emphasis_delimiters_left -= 1;
                return true;
            }
            if valid_symbols[close_token as usize] {
                lexer.set_result_symbol(close_token as u16);
                self.num_emphasis_delimiters_left -= 1;
                return true;
            }
        }
        lexer.mark_end();
        let mut delimiter_count = 1u8;
        let mut lookahead = lexer.lookahead();
        while lookahead == i32::from(delimiter) {
            delimiter_count = delimiter_count.wrapping_add(1);
            lexer.advance(false);
            lookahead = lexer.lookahead();
        }
        let line_end = matches!(lookahead, 10 | 13) || (lookahead == 0 && lexer.eof());
        if valid_symbols[open_token as usize] || valid_symbols[close_token as usize] {
            self.num_emphasis_delimiters_left = delimiter_count.wrapping_sub(1);
            let next_symbol_whitespace =
                line_end || lookahead == i32::from(b' ') || lookahead == i32::from(b'\t');
            let next_symbol_punctuation = is_punctuation(lookahead);
            if valid_symbols[close_token as usize]
                && !valid_symbols[Token::LastTokenWhitespace as usize]
                && (!valid_symbols[Token::LastTokenPunctuation as usize]
                    || next_symbol_punctuation
                    || next_symbol_whitespace)
            {
                // Closing delimiters take precedence.
                self.state &= !STATE_EMPHASIS_DELIMITER_IS_OPEN;
                lexer.set_result_symbol(close_token as u16);
                return true;
            }
            if !next_symbol_whitespace
                && (!next_symbol_punctuation
                    || valid_symbols[Token::LastTokenPunctuation as usize]
                    || valid_symbols[Token::LastTokenWhitespace as usize])
            {
                // Deliberately do not recheck valid_symbols[open_token]: C can
                // emit an opener here even when only the closer is valid.
                self.state |= STATE_EMPHASIS_DELIMITER_IS_OPEN;
                lexer.set_result_symbol(open_token as u16);
                return true;
            }
        }
        false
    }
}

impl ExternalScanner for Scanner {
    fn scan(&mut self, lexer: &mut dyn Lexer, valid_symbols: &[bool]) -> bool {
        let valid_symbols = valid_symbols
            .first_chunk::<15>()
            .expect("external token flags");
        if valid_symbols[Token::TriggerError as usize] {
            lexer.set_result_symbol(Token::Error as u16);
            return true;
        }
        match lexer.lookahead() {
            delimiter @ (0x60 | 0x24) => {
                let length = if delimiter == 0x60 {
                    &mut self.code_span_delimiter_length
                } else {
                    &mut self.latex_span_delimiter_length
                };
                parse_leaf_delimiter(lexer, length, valid_symbols, delimiter as u8)
            }
            delimiter @ (0x2a | 0x5f | 0x7e) => {
                self.parse_emphasis_delimiter(lexer, valid_symbols, delimiter as u8)
            }
            _ => false,
        }
    }

    fn serialize(&mut self, buffer: &mut [u8]) -> usize {
        buffer[..4].copy_from_slice(&[
            self.state,
            self.code_span_delimiter_length,
            self.latex_span_delimiter_length,
            self.num_emphasis_delimiters_left,
        ]);
        4
    }

    fn deserialize(&mut self, buffer: &[u8]) {
        // Valid snapshots have four bytes; an empty slice resets the state.
        // C reads past the buffer on malformed short snapshots. Reset those
        // safely as well, using the same single fixed-width check.
        let [state, code_span_delimiter_length, latex_span_delimiter_length, num_emphasis_delimiters_left] =
            buffer.first_chunk::<4>().copied().unwrap_or_default();
        *self = Self {
            state,
            code_span_delimiter_length,
            latex_span_delimiter_length,
            num_emphasis_delimiters_left,
        };
    }
}

/// Creates a scanner (C's `tree_sitter_markdown_inline_external_scanner_create`).
pub(crate) fn create() -> Box<dyn ExternalScanner> {
    Box::new(Scanner::default())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::{Cell, RefCell};

    #[derive(Debug, PartialEq, Eq)]
    enum Call {
        Advance(usize),
        MarkEnd(usize),
        Eof(usize),
        Symbol(u16),
    }

    struct TestLexer {
        input: Vec<i32>,
        position: usize,
        end: Option<usize>,
        symbol: u16,
        calls: RefCell<Vec<Call>>,
        lookahead_calls: Cell<usize>,
    }

    impl TestLexer {
        fn new(input: &str) -> Self {
            Self {
                input: input.chars().map(|ch| ch as i32).collect(),
                position: 0,
                end: None,
                symbol: u16::MAX,
                calls: RefCell::default(),
                lookahead_calls: Cell::new(0),
            }
        }
    }

    impl Lexer for TestLexer {
        fn lookahead(&self) -> i32 {
            self.lookahead_calls.set(self.lookahead_calls.get() + 1);
            self.input.get(self.position).copied().unwrap_or(0)
        }

        fn result_symbol(&self) -> u16 {
            self.symbol
        }

        fn set_result_symbol(&mut self, symbol: u16) {
            self.symbol = symbol;
            self.calls.borrow_mut().push(Call::Symbol(symbol));
        }

        fn advance(&mut self, skip: bool) {
            assert!(!skip, "the inline scanner must never skip characters");
            assert!(self.position < self.input.len());
            self.calls.borrow_mut().push(Call::Advance(self.position));
            self.position += 1;
        }

        fn mark_end(&mut self) {
            self.end = Some(self.position);
            self.calls.borrow_mut().push(Call::MarkEnd(self.position));
        }

        fn get_column(&mut self) -> u32 {
            panic!("the inline scanner must not request a column")
        }

        fn is_at_included_range_start(&self) -> bool {
            panic!("the inline scanner must not request included-range status")
        }

        fn eof(&self) -> bool {
            self.calls.borrow_mut().push(Call::Eof(self.position));
            self.position == self.input.len()
        }
    }

    fn valid(tokens: &[Token]) -> [bool; 15] {
        let mut result = [false; 15];
        for &token in tokens {
            result[token as usize] = true;
        }
        result
    }

    fn snapshot(scanner: &mut dyn ExternalScanner) -> [u8; 4] {
        let mut buffer = [0; 4];
        assert_eq!(scanner.serialize(&mut buffer), 4);
        buffer
    }

    const EMPHASIS: [(char, Token, Token); 3] = [
        ('*', Token::EmphasisOpenStar, Token::EmphasisCloseStar),
        (
            '_',
            Token::EmphasisOpenUnderscore,
            Token::EmphasisCloseUnderscore,
        ),
        ('~', Token::StrikethroughOpen, Token::StrikethroughClose),
    ];

    #[test]
    fn serialization_preserves_all_four_bytes_and_short_input_resets() {
        let mut scanner = create();
        assert_eq!(snapshot(&mut *scanner), [0; 4]);
        let state = [0xfd, 0xfe, 0xff, 0x80];
        scanner.deserialize(&state);
        let mut buffer = [0xaa; 8];
        assert_eq!(scanner.serialize(&mut buffer), 4);
        assert_eq!(&buffer[..4], &state);
        assert_eq!(&buffer[4..], &[0xaa; 4]);
        for length in 0..4 {
            scanner.deserialize(&state);
            scanner.deserialize(&state[..length]);
            assert_eq!(snapshot(&mut *scanner), [0; 4]);
        }
    }

    #[test]
    fn error_takes_precedence_and_other_characters_are_not_consumed() {
        let mut scanner = Scanner::default();
        scanner.deserialize(&[3, 2, 1, 4]);
        let mut lexer = TestLexer::new("`text`");
        assert!(scanner.scan(&mut lexer, &[true; 15]));
        assert_eq!(*lexer.calls.borrow(), [Call::Symbol(Token::Error as u16)]);
        assert_eq!(snapshot(&mut scanner), [3, 2, 1, 4]);

        for input in ["", " *text*", "\\*", "+", "é", "\u{12a}"] {
            let mut lexer = TestLexer::new(input);
            assert!(!scanner.scan(&mut lexer, &valid(&[Token::CodeSpanStart])));
            assert!(lexer.calls.borrow().is_empty());
        }
    }

    #[test]
    fn grouped_dispatch_rejects_non_delimiters_without_state_or_lexer_effects() {
        let mut scanner = Scanner::default();
        let state = [0xfd, 7, 8, 3];
        scanner.deserialize(&state);
        let mut valid_symbols = [true; 15];
        valid_symbols[Token::TriggerError as usize] = false;
        // Include low-byte aliases of all delimiters, NUL, invalid scalar
        // values, and non-ASCII scalars. Dispatch must compare full codepoints
        // before narrowing the matched delimiter to a byte for its handler.
        for codepoint in (0..=0x2ff).chain([i32::MIN, -1, 0xd800, 0x10ffff, i32::MAX]) {
            if matches!(codepoint, 0x24 | 0x2a | 0x5f | 0x60 | 0x7e) {
                continue;
            }
            let mut lexer = TestLexer::new("");
            lexer.input = vec![codepoint];
            assert!(!scanner.scan(&mut lexer, &valid_symbols), "{codepoint:x}");
            assert!(lexer.calls.borrow().is_empty(), "{codepoint:x}");
            assert_eq!(snapshot(&mut scanner), state, "{codepoint:x}");
        }
    }

    #[test]
    fn leaf_spans_preserve_lexer_call_order_and_independent_lengths() {
        for (delimiter, open, close, state_index) in [
            ('`', Token::CodeSpanStart, Token::CodeSpanClose, 1),
            ('$', Token::LatexSpanStart, Token::LatexSpanClose, 2),
        ] {
            let mut scanner = Scanner::default();
            let mut state = [0, 7, 8, 0];
            scanner.deserialize(&state);
            let mut lexer = TestLexer::new(&format!("{delimiter}x{delimiter}!"));
            assert!(scanner.scan(&mut lexer, &valid(&[open, close])));
            assert_eq!(lexer.end, Some(1));
            assert_eq!(lexer.position, 3);
            assert_eq!(
                *lexer.calls.borrow(),
                [
                    Call::Advance(0),
                    Call::MarkEnd(1),
                    Call::Advance(1),
                    Call::Advance(2),
                    Call::Symbol(open as u16),
                ]
            );
            state[state_index] = 1;
            assert_eq!(snapshot(&mut scanner), state);

            let mut lexer = TestLexer::new(&format!("{delimiter}tail"));
            assert!(scanner.scan(&mut lexer, &valid(&[open, close])));
            assert_eq!(
                *lexer.calls.borrow(),
                [
                    Call::Advance(0),
                    Call::MarkEnd(1),
                    Call::Symbol(close as u16)
                ]
            );
            state[state_index] = 0;
            assert_eq!(snapshot(&mut scanner), state);
        }
    }

    #[test]
    fn leaf_matching_requires_a_whole_run_and_unclosed_requires_an_opener() {
        let mut scanner = Scanner::default();
        let mut lexer = TestLexer::new("``x```y``!");
        assert!(scanner.scan(&mut lexer, &valid(&[Token::CodeSpanStart])));
        assert_eq!(lexer.symbol, Token::CodeSpanStart as u16);
        assert_eq!((lexer.end, lexer.position), (Some(2), 9));
        assert_eq!(snapshot(&mut scanner), [0, 2, 0, 0]);

        for (tokens, accepted, expected_position) in [
            (vec![Token::CodeSpanStart, Token::UnclosedSpan], true, 5),
            (vec![Token::CodeSpanStart], false, 5),
            (vec![Token::UnclosedSpan], false, 2),
        ] {
            scanner.deserialize(&[0, 7, 8, 0]);
            let mut lexer = TestLexer::new("``x`");
            // Add one more non-delimiter so the nonmatching run is reset.
            lexer.input.push(i32::from(b'!'));
            assert_eq!(scanner.scan(&mut lexer, &valid(&tokens)), accepted);
            assert_eq!((lexer.end, lexer.position), (Some(2), expected_position));
            if accepted {
                assert_eq!(lexer.symbol, Token::UnclosedSpan as u16);
            }
            assert_eq!(snapshot(&mut scanner), [0, 7, 8, 0]);
        }
    }

    #[test]
    fn leaf_opening_count_wraps_but_closing_lookahead_count_does_not() {
        let mut scanner = Scanner::default();
        let mut lexer = TestLexer::new(&format!("{}x", "`".repeat(256)));
        assert!(scanner.scan(&mut lexer, &valid(&[Token::CodeSpanClose])));
        assert_eq!(lexer.symbol, Token::CodeSpanClose as u16);
        assert_eq!((lexer.end, lexer.position), (Some(256), 256));

        for (closing_count, accepted) in [(1, true), (257, false)] {
            scanner.deserialize(&[]);
            let mut lexer = TestLexer::new(&format!(
                "{}x{}",
                "`".repeat(257),
                "`".repeat(closing_count)
            ));
            assert_eq!(
                scanner.scan(&mut lexer, &valid(&[Token::CodeSpanStart])),
                accepted
            );
            assert_eq!(lexer.end, Some(257));
            assert_eq!(scanner.code_span_delimiter_length, u8::from(accepted));
        }

        // A wrapped zero count can match the zero closing count immediately.
        let mut lexer = TestLexer::new(&format!("{}text", "$".repeat(256)));
        assert!(scanner.scan(&mut lexer, &valid(&[Token::LatexSpanStart])));
        assert_eq!(lexer.symbol, Token::LatexSpanStart as u16);
        assert_eq!(lexer.position, 256);
    }

    #[test]
    fn emphasis_run_reuses_state_and_clears_the_opening_flag() {
        for (delimiter, open, close) in EMPHASIS {
            let mut scanner = Scanner::default();
            let mut lexer = TestLexer::new(&format!("{delimiter}{delimiter}{delimiter}x"));
            let valid_symbols = valid(&[open, close, Token::LastTokenWhitespace]);
            assert!(scanner.scan(&mut lexer, &valid_symbols));
            assert_eq!(snapshot(&mut scanner), [4, 0, 0, 2]);
            assert_eq!(
                *lexer.calls.borrow(),
                [
                    Call::Advance(0),
                    Call::MarkEnd(1),
                    Call::Advance(1),
                    Call::Advance(2),
                    Call::Symbol(open as u16),
                ]
            );

            // The parser resumes at the marked end, not the lookahead position.
            lexer.position = 1;
            lexer.calls.borrow_mut().clear();
            assert!(scanner.scan(&mut lexer, &valid_symbols));
            assert_eq!(
                *lexer.calls.borrow(),
                [Call::Advance(1), Call::Symbol(open as u16)]
            );
            assert_eq!(snapshot(&mut scanner), [0, 0, 0, 1]);

            lexer.calls.borrow_mut().clear();
            assert!(scanner.scan(&mut lexer, &valid_symbols));
            assert_eq!(
                *lexer.calls.borrow(),
                [Call::Advance(2), Call::Symbol(close as u16)]
            );
            assert_eq!(snapshot(&mut scanner), [0; 4]);
        }
    }

    #[test]
    fn emphasis_flanking_and_open_token_selection_match_c() {
        for (delimiter, open, close) in EMPHASIS {
            for (suffix, previous, expected) in [
                ("word", None, Some(close)),
                ("word", Some(Token::LastTokenWhitespace), Some(open)),
                ("word", Some(Token::LastTokenPunctuation), Some(open)),
                ("!", Some(Token::LastTokenWhitespace), Some(open)),
                ("!", Some(Token::LastTokenPunctuation), Some(close)),
                (" ", Some(Token::LastTokenWhitespace), None),
                ("\t", Some(Token::LastTokenWhitespace), None),
                ("\n", Some(Token::LastTokenWhitespace), None),
                ("\r", Some(Token::LastTokenWhitespace), None),
                ("", Some(Token::LastTokenWhitespace), None),
                // Only space/tab/CR/LF/EOF count as whitespace. Punctuation
                // is ASCII-only, so the curly quote is an ordinary character.
                ("\u{b}", Some(Token::LastTokenWhitespace), Some(open)),
                ("\u{a0}", Some(Token::LastTokenWhitespace), Some(open)),
                ("“", Some(Token::LastTokenWhitespace), Some(open)),
            ] {
                let mut scanner = Scanner::default();
                let mut lexer = TestLexer::new(&format!("{delimiter}{suffix}"));
                let mut valid_symbols = valid(&[open, close]);
                if let Some(previous) = previous {
                    valid_symbols[previous as usize] = true;
                }
                assert_eq!(scanner.scan(&mut lexer, &valid_symbols), expected.is_some());
                if let Some(token) = expected {
                    assert_eq!(lexer.symbol, token as u16);
                }
                if matches!(suffix, "\n" | "\r") {
                    assert!(!lexer
                        .calls
                        .borrow()
                        .iter()
                        .any(|call| matches!(call, Call::Eof(_))));
                }
            }

            let mut scanner = Scanner::default();
            let mut lexer = TestLexer::new(&format!("{delimiter}word"));
            // C emits an opener even though it is not marked valid here.
            assert!(scanner.scan(&mut lexer, &valid(&[close, Token::LastTokenWhitespace])));
            assert_eq!(lexer.symbol, open as u16);
        }
    }

    #[test]
    fn emphasis_count_wraps_even_when_scan_fails() {
        for (delimiter, open, _) in EMPHASIS {
            for (count, remaining) in [(256, 255), (257, 0)] {
                let run = delimiter.to_string().repeat(count);
                let mut scanner = Scanner::default();
                let mut lexer = TestLexer::new(&format!("{run} "));
                assert!(!scanner.scan(&mut lexer, &valid(&[open])));
                assert_eq!(scanner.num_emphasis_delimiters_left, remaining);
                assert_eq!((lexer.end, lexer.position), (Some(1), count));
            }
        }
    }

    // Original C control flow: compare all advances, marks, results and state,
    // including failed scans. EOF queries themselves are pure and can be elided.
    fn reference_leaf_delimiter(
        lexer: &mut dyn Lexer,
        delimiter_length: &mut u8,
        valid_symbols: &[bool],
        delimiter: u8,
        open_token: Token,
        close_token: Token,
    ) -> bool {
        let mut level = 0u8;
        while lexer.lookahead() == i32::from(delimiter) {
            lexer.advance(false);
            level = level.wrapping_add(1);
        }
        lexer.mark_end();
        if level == *delimiter_length && valid_symbols[close_token as usize] {
            *delimiter_length = 0;
            lexer.set_result_symbol(close_token as u16);
            return true;
        }
        if valid_symbols[open_token as usize] {
            // Look for a matching closing run, but leave the token end at the end
            // of the opening run. C uses size_t here, unlike the opening u8 count.
            let mut close_level = 0usize;
            while !lexer.eof() {
                if lexer.lookahead() == i32::from(delimiter) {
                    close_level = close_level.wrapping_add(1);
                } else {
                    if close_level == usize::from(level) {
                        break;
                    }
                    close_level = 0;
                }
                lexer.advance(false);
            }
            if close_level == usize::from(level) {
                *delimiter_length = level;
                lexer.set_result_symbol(open_token as u16);
                return true;
            }
            if valid_symbols[Token::UnclosedSpan as usize] {
                lexer.set_result_symbol(Token::UnclosedSpan as u16);
                return true;
            }
        }
        false
    }

    fn compare_leaf_with_reference(input: &[i32], delimiter: u8, length: u8, flags: u8) {
        let (open, close, length_index) = if delimiter == b'`' {
            (Token::CodeSpanStart, Token::CodeSpanClose, 1)
        } else {
            (Token::LatexSpanStart, Token::LatexSpanClose, 2)
        };
        let mut valid_symbols = [false; 15];
        valid_symbols[open as usize] = flags & 1 != 0;
        valid_symbols[close as usize] = flags & 2 != 0;
        valid_symbols[Token::UnclosedSpan as usize] = flags & 4 != 0;
        let mut scanner = Scanner::default();
        let mut state = [0xa5, 7, 8, 0xfe];
        state[length_index] = length;
        scanner.deserialize(&state);
        let mut actual = TestLexer::new("");
        actual.input.extend_from_slice(input);
        let mut expected = TestLexer::new("");
        expected.input.extend_from_slice(input);
        let expected_result = reference_leaf_delimiter(
            &mut expected,
            &mut state[length_index],
            &valid_symbols,
            delimiter,
            open,
            close,
        );
        assert_eq!(scanner.scan(&mut actual, &valid_symbols), expected_result);
        assert_eq!(snapshot(&mut scanner), state);
        assert_eq!(actual.position, expected.position);
        assert_eq!(actual.end, expected.end);
        assert_eq!(actual.symbol, expected.symbol);
        assert!(actual
            .calls
            .borrow()
            .iter()
            .filter(|call| !matches!(call, Call::Eof(_)))
            .eq(expected
                .calls
                .borrow()
                .iter()
                .filter(|call| !matches!(call, Call::Eof(_)))));
    }

    #[test]
    fn leaf_run_search_matches_c_for_all_short_suffixes_and_token_sets() {
        for delimiter in *b"`$" {
            // Exhaustive short suffixes include embedded NUL, which is not EOF.
            for suffix_length in 0..=5 {
                for mut pattern in 0..3usize.pow(suffix_length) {
                    let suffix: Vec<_> = (0..suffix_length)
                        .map(|_| {
                            let character = [i32::from(delimiter), i32::from(b'x'), 0][pattern % 3];
                            pattern /= 3;
                            character
                        })
                        .collect();
                    for opening_count in [1, 2] {
                        let mut input = vec![i32::from(delimiter); opening_count];
                        input.extend_from_slice(&suffix);
                        for previous_length in [0, 1, 2] {
                            for flags in 0..8 {
                                compare_leaf_with_reference(
                                    &input,
                                    delimiter,
                                    previous_length,
                                    flags,
                                );
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn wrapped_opening_counts_and_long_closing_runs_match_c() {
        for delimiter in *b"`$" {
            for opening_count in [255, 256, 257, 512, 513] {
                for closing_count in [0, 1, 2, 255, 256, 257, 512, 513] {
                    for following in [None, Some(0), Some(i32::from(b'!'))] {
                        let mut input = vec![i32::from(delimiter); opening_count];
                        input.push(0);
                        input.extend(std::iter::repeat_n(i32::from(delimiter), closing_count));
                        input.extend(following);
                        for flags in 0..8 {
                            compare_leaf_with_reference(&input, delimiter, 0, flags);
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn ordinary_span_content_uses_one_lookahead_per_position_and_no_eof_queries() {
        let mut scanner = Scanner::default();
        let mut lexer = TestLexer::new(&format!("`{}`!", "content".repeat(32)));
        assert!(scanner.scan(&mut lexer, &valid(&[Token::CodeSpanStart])));
        assert_eq!(lexer.lookahead_calls.get(), lexer.position + 1);
        assert!(!lexer
            .calls
            .borrow()
            .iter()
            .any(|call| matches!(call, Call::Eof(_))));
        assert_eq!(lexer.end, Some(1));
    }

    #[test]
    fn emphasis_treats_nul_as_content_but_eof_as_whitespace() {
        for (delimiter, open, _) in EMPHASIS {
            let valid_symbols = valid(&[open, Token::LastTokenWhitespace]);
            for (suffix, accepted) in [("\0", true), ("", false)] {
                let mut scanner = Scanner::default();
                let mut lexer = TestLexer::new(&format!("{delimiter}{suffix}"));
                assert_eq!(scanner.scan(&mut lexer, &valid_symbols), accepted);
                assert_eq!(lexer.lookahead_calls.get(), 2);
                assert_eq!(lexer.position, 1);
                assert_eq!(lexer.end, Some(1));
                assert_eq!(
                    lexer
                        .calls
                        .borrow()
                        .iter()
                        .filter(|call| matches!(call, Call::Eof(_)))
                        .count(),
                    1
                );
                if accepted {
                    assert_eq!(lexer.symbol, open as u16);
                }
            }
        }
    }
}
