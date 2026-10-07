//! Odin's stateless external scanner, translated from `src/scanner.c`.

use tree_sitter_language::{ExternalScanner, Lexer};

const NEWLINE: usize = 0;
const BACKSLASH: usize = 1;
const NL_COMMA: usize = 2;
const FLOAT: usize = 3;
const BLOCK_COMMENT: usize = 4;
const BRACKET: usize = 5;
// External token 6 (QUOTE) is not used by the C scanner.

// The reference scanner uses iswspace in the default C locale, including VT.
fn is_space(c: i32) -> bool {
    matches!(c, 0x09..=0x0d | 0x20)
}

fn is_digit(c: i32) -> bool {
    (i32::from(b'0')..=i32::from(b'9')).contains(&c)
}

/// The C scanner has a null payload and serializes no state.
pub(crate) struct Scanner;

impl ExternalScanner for Scanner {
    fn scan(&mut self, lexer: &mut dyn Lexer, valid_symbols: &[bool]) -> bool {
        // Breaking out of this block implements C's `goto newline`. In
        // particular, failed float attempts must bypass the NL_COMMA branch.
        'newline: {
            if valid_symbols[FLOAT] {
                while is_space(lexer.lookahead()) && lexer.lookahead() != i32::from(b'\n') {
                    lexer.advance(true);
                }

                if !valid_symbols[NEWLINE] {
                    while is_space(lexer.lookahead()) {
                        lexer.advance(true);
                    }
                }

                let mut found_decimal = false;
                let mut found_exponent = false;
                let mut found_number_before_decimal = false;
                let mut found_number_after_decimal = false;
                let mut found_number_after_exponent = false;
                let mut first_character = true;
                loop {
                    match lexer.lookahead() {
                        0x2e => {
                            // '.'
                            if (found_decimal || found_exponent)
                                && (found_number_after_decimal || found_number_before_decimal)
                            {
                                lexer.set_result_symbol(FLOAT as u16);
                                lexer.mark_end();
                                return true;
                            } else {
                                lexer.mark_end();
                                found_decimal = true;
                                lexer.advance(false);
                                if lexer.lookahead() == i32::from(b'.') {
                                    lexer.advance(false);
                                    break 'newline;
                                }
                                lexer.mark_end();
                                if !is_digit(lexer.lookahead())
                                    && (found_number_after_decimal || found_number_before_decimal)
                                {
                                    lexer.set_result_symbol(FLOAT as u16);
                                    return true;
                                }
                            }
                        }
                        0x69..=0x6b => {
                            // 'i', 'j', 'k'
                            if !found_number_after_decimal {
                                break 'newline;
                            }
                            if (found_decimal || found_exponent)
                                && (found_number_after_decimal || found_number_before_decimal)
                            {
                                lexer.advance(false);
                                lexer.set_result_symbol(FLOAT as u16);
                                lexer.mark_end();
                                return true;
                            }
                            break 'newline;
                        }
                        0x65 | 0x45 => {
                            // 'e', 'E'
                            if found_exponent
                                && (found_number_after_decimal || found_number_before_decimal)
                            {
                                lexer.set_result_symbol(FLOAT as u16);
                                lexer.mark_end();
                                return true;
                            } else if found_number_before_decimal || found_number_after_decimal {
                                found_exponent = true;
                                lexer.advance(false);
                            } else {
                                break 'newline;
                            }
                        }
                        0x2b | 0x2d => {
                            // '+', '-'
                            if first_character || (found_exponent && !found_number_after_exponent) {
                                lexer.advance(false);
                            } else {
                                break 'newline;
                            }
                        }
                        _ => {
                            if lexer.lookahead() <= 255 && is_digit(lexer.lookahead()) {
                                lexer.advance(false);
                                if found_decimal {
                                    found_number_after_decimal = true;
                                } else {
                                    found_number_before_decimal = true;
                                }
                                if found_exponent && !found_number_after_exponent {
                                    found_number_after_exponent = true;
                                }
                            } else {
                                if (found_decimal || found_exponent)
                                    && (found_number_after_decimal || found_number_before_decimal)
                                {
                                    lexer.set_result_symbol(FLOAT as u16);
                                    lexer.mark_end();
                                    return true;
                                }
                                if found_number_before_decimal {
                                    return false;
                                }
                                break 'newline;
                            }
                        }
                    }
                    first_character = false;
                }
            }

            if valid_symbols[NL_COMMA] {
                while is_space(lexer.lookahead()) && lexer.lookahead() != i32::from(b'\n') {
                    lexer.advance(true);
                }

                if lexer.lookahead() == i32::from(b',') {
                    lexer.advance(false);
                    lexer.set_result_symbol(NL_COMMA as u16);
                    lexer.mark_end();
                    while is_space(lexer.lookahead()) && lexer.lookahead() != i32::from(b'\n') {
                        lexer.advance(false);
                    }

                    if lexer.lookahead() == i32::from(b'\n') {
                        while is_space(lexer.lookahead()) {
                            lexer.advance(false);
                        }
                        return lexer.lookahead() != i32::from(b'}');
                    }
                }
            }
        }

        'backslash: {
            if valid_symbols[NEWLINE] {
                while is_space(lexer.lookahead()) && lexer.lookahead() != i32::from(b'\n') {
                    lexer.advance(true);
                }

                if lexer.lookahead() == i32::from(b'\n') {
                    lexer.advance(false);
                    lexer.set_result_symbol(NEWLINE as u16);
                    lexer.mark_end();

                    let mut nl_count = 0u32;
                    while is_space(lexer.lookahead()) {
                        if lexer.lookahead() == i32::from(b'\n') {
                            nl_count = nl_count.wrapping_add(1);
                        }
                        lexer.advance(true);
                    }

                    let mut next_word = [0u8; 6];
                    for c in &mut next_word[..5] {
                        if is_space(lexer.lookahead()) {
                            break;
                        }
                        *c = lexer.lookahead() as u8;
                        lexer.advance(false);
                    }
                    // C narrows lookahead to char, then strcmp stops at the
                    // first NUL, even if more lookahead was consumed after it.
                    let length = next_word.iter().position(|&c| c == 0).unwrap_or(6);
                    let next_word = &next_word[..length];
                    if next_word == b"where" || next_word == b"else" {
                        if !is_space(lexer.lookahead()) {
                            return true;
                        }
                        break 'backslash;
                    }

                    if next_word == b"{" && nl_count == 0 && valid_symbols[BRACKET] {
                        return false;
                    }

                    return true;
                }
            }
        }

        if valid_symbols[BACKSLASH] && lexer.lookahead() == i32::from(b'\\') {
            lexer.advance(false);
            if lexer.lookahead() == i32::from(b'\n') {
                lexer.advance(false);
                while is_space(lexer.lookahead()) {
                    lexer.advance(false);
                }
                lexer.set_result_symbol(BACKSLASH as u16);
                return true;
            }
        }

        while is_space(lexer.lookahead()) {
            lexer.advance(true);
        }

        if valid_symbols[BLOCK_COMMENT] && lexer.lookahead() == i32::from(b'/') {
            lexer.advance(false);
            if lexer.lookahead() != i32::from(b'*') {
                return false;
            }
            lexer.advance(false);

            if lexer.lookahead() == i32::from(b'"') {
                return false;
            }

            let mut after_star = false;
            let mut nesting_depth = 1u32;
            loop {
                match lexer.lookahead() {
                    0 => return false,
                    0x2a => {
                        // '*'
                        lexer.advance(false);
                        after_star = true;
                    }
                    0x2f => {
                        // '/'
                        if after_star {
                            lexer.advance(false);
                            after_star = false;
                            nesting_depth = nesting_depth.wrapping_sub(1);
                            if nesting_depth == 0 {
                                lexer.set_result_symbol(BLOCK_COMMENT as u16);
                                return true;
                            }
                        } else {
                            lexer.advance(false);
                            after_star = false;
                            if lexer.lookahead() == i32::from(b'*') {
                                nesting_depth = nesting_depth.wrapping_add(1);
                                lexer.advance(false);
                            }
                        }
                    }
                    _ => {
                        lexer.advance(false);
                        after_star = false;
                    }
                }
            }
        }

        false
    }

    fn serialize(&mut self, _buffer: &mut [u8]) -> usize {
        0
    }

    fn deserialize(&mut self, _buffer: &[u8]) {}
}

/// Creates a scanner (C's `tree_sitter_odin_external_scanner_create`).
pub(crate) fn create() -> Box<dyn ExternalScanner> {
    Box::new(Scanner)
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
        end: Option<usize>,
        symbol: u16,
        events: Vec<Event>,
    }

    impl TestLexer {
        fn new(input: &str) -> Self {
            Self {
                input: input.chars().map(|c| c as i32).collect(),
                position: 0,
                end: None,
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
            if self.position < self.input.len() {
                self.position += 1;
            }
        }

        fn mark_end(&mut self) {
            self.end = Some(self.position);
            self.events.push(Event::Mark(self.position));
        }

        fn get_column(&mut self) -> u32 {
            panic!("the Odin scanner does not request columns")
        }

        fn is_at_included_range_start(&self) -> bool {
            panic!("the Odin scanner does not query included ranges")
        }

        fn eof(&self) -> bool {
            panic!("the Odin scanner tests lookahead, not eof")
        }
    }

    fn scan(input: &str, tokens: &[usize]) -> (bool, TestLexer) {
        let mut valid = [false; 7];
        for &token in tokens {
            valid[token] = true;
        }
        let mut lexer = TestLexer::new(input);
        let result = Scanner.scan(&mut lexer, &valid);
        (result, lexer)
    }

    #[test]
    fn floats_preserve_permissive_exponents_and_imaginary_suffix_rules() {
        for (input, accepted, position, end) in [
            ("1.", true, 2, Some(2)),
            (".1", true, 2, Some(2)),
            ("-1.2i", true, 5, Some(5)),
            ("+.2j", true, 4, Some(4)),
            ("0.3k", true, 4, Some(4)),
            ("1e", true, 2, Some(2)),
            ("1e++-", true, 5, Some(5)),
            ("1e2i", false, 3, None),
            ("1.i", true, 2, Some(2)),
            ("1.2e2j", true, 6, Some(6)),
            ("1e.2", true, 2, Some(2)),
            ("1.2.3", true, 3, Some(3)),
            ("1.2eE", true, 4, Some(4)),
            ("1.2+3", false, 3, Some(2)),
            ("123", false, 3, None),
            ("1..<", false, 3, Some(1)),
            ("..", false, 2, Some(0)),
            ("i", false, 0, None),
            ("e", false, 0, None),
            (".", false, 1, Some(1)),
            (".e", false, 1, Some(1)),
        ] {
            let (result, lexer) = scan(input, &[FLOAT]);
            assert_eq!(result, accepted, "{input:?}");
            assert_eq!(lexer.position, position, "{input:?}");
            assert_eq!(lexer.end, end, "{input:?}");
            if accepted {
                assert_eq!(lexer.symbol, FLOAT as u16, "{input:?}");
            }
        }
    }

    #[test]
    fn float_marks_and_newline_skipping_match_c() {
        let (result, lexer) = scan("\t1. ", &[FLOAT]);
        assert!(result);
        assert_eq!(
            lexer.events,
            [
                Event::Advance(0, true),
                Event::Advance(1, false),
                Event::Mark(2),
                Event::Advance(2, false),
                Event::Mark(3),
                Event::Symbol(FLOAT as u16),
            ]
        );
        let (result, lexer) = scan("\n1.0", &[FLOAT]);
        assert!(result);
        assert_eq!(lexer.symbol, FLOAT as u16);
        assert_eq!(lexer.events[0], Event::Advance(0, true));
        let (result, lexer) = scan("\n1.0", &[FLOAT, NEWLINE]);
        assert!(result);
        assert_eq!(lexer.symbol, NEWLINE as u16);
        assert_eq!(lexer.end, Some(1));
        assert_eq!(lexer.events[0], Event::Advance(0, false));
    }

    #[test]
    fn float_fallback_skips_comma_without_rewinding() {
        let (result, lexer) = scan(",\nx", &[FLOAT, NL_COMMA]);
        assert!(!result);
        assert!(lexer.events.is_empty());
        let (result, lexer) = scan("1..\nx", &[FLOAT, NEWLINE]);
        assert!(result);
        assert_eq!(lexer.symbol, NEWLINE as u16);
        assert_eq!(lexer.end, Some(4));
        let (result, lexer) = scan("1/*x*/", &[FLOAT, BLOCK_COMMENT]);
        assert!(!result);
        assert_eq!(lexer.position, 1);
        let (result, lexer) = scan("+/*x*/", &[FLOAT, BLOCK_COMMENT]);
        assert!(result);
        assert_eq!(lexer.symbol, BLOCK_COMMENT as u16);
        assert_eq!(lexer.position, 6);
        assert_eq!(lexer.end, None);
    }

    #[test]
    fn comma_keeps_its_mark_while_looking_past_whitespace() {
        let (result, lexer) = scan(" , \t\n x", &[NL_COMMA]);
        assert!(result);
        assert_eq!(lexer.position, 6);
        assert_eq!(
            lexer.events,
            [
                Event::Advance(0, true),
                Event::Advance(1, false),
                Event::Symbol(NL_COMMA as u16),
                Event::Mark(2),
                Event::Advance(2, false),
                Event::Advance(3, false),
                Event::Advance(4, false),
                Event::Advance(5, false),
            ]
        );
        let (result, lexer) = scan(",\n}", &[NL_COMMA, NEWLINE]);
        assert!(!result);
        assert_eq!(lexer.position, 2);
        assert_eq!(lexer.end, Some(1));
        let (result, lexer) = scan(",/*x*/", &[NL_COMMA, BLOCK_COMMENT]);
        assert!(result);
        assert_eq!(lexer.symbol, BLOCK_COMMENT as u16);
        assert_eq!(lexer.position, 6);
        assert_eq!(lexer.end, Some(1));
    }

    #[test]
    fn newline_looks_ahead_five_narrowed_characters() {
        for (input, accepted) in [
            ("\nwhere x", false),
            ("\nelse x", false),
            ("\nwhereX", true),
            ("\nelseX", true),
            ("\nwhere", true),
            ("\nelse", true),
            ("\n{ ", false),
            ("\n{x", true),
            ("\n\n{ ", true),
            ("\n{\0abc", false),
            // C's char cast and strcmp see a one-byte brace followed by NUL.
            ("\n\u{17b}\u{100}abc", false),
        ] {
            let (result, lexer) = scan(input, &[NEWLINE, BRACKET]);
            assert_eq!(result, accepted, "{input:?}");
            assert_eq!(lexer.end, Some(1), "{input:?}");
            assert_eq!(lexer.symbol, NEWLINE as u16, "{input:?}");
        }
        let (result, lexer) = scan("\n", &[NEWLINE]);
        assert!(result);
        assert_eq!(lexer.events.len(), 8); // advance, symbol, mark, five EOF advances
        assert!(
            lexer.events[3..]
                .iter()
                .all(|e| *e == Event::Advance(1, false))
        );
        assert!(scan("\n{ ", &[NEWLINE]).0);
    }

    #[test]
    fn newline_keyword_falls_through_to_comment_without_remarking() {
        let (result, lexer) = scan("\nelse /*x*/", &[NEWLINE, BLOCK_COMMENT]);
        assert!(result);
        assert_eq!(lexer.symbol, BLOCK_COMMENT as u16);
        assert_eq!(lexer.end, Some(1));
        assert_eq!(lexer.position, 11);
        // The space is skipped only after the backslash test, so this does
        // not recognize a continuation even though BACKSLASH is valid.
        assert!(!scan("\nwhere \\\nx", &[NEWLINE, BACKSLASH]).0);
    }

    #[test]
    fn continuation_and_nested_comments_do_not_mark_end() {
        let (result, lexer) = scan("\\\n \t\nx", &[BACKSLASH]);
        assert!(result);
        assert_eq!(lexer.symbol, BACKSLASH as u16);
        assert_eq!(lexer.position, 5);
        assert_eq!(lexer.end, None);
        assert!(
            lexer.events[..5]
                .iter()
                .all(|e| matches!(e, Event::Advance(_, false)))
        );
        for (input, accepted, position) in [
            ("/* a /* b */ c */x", true, 17),
            ("/***/x", true, 5),
            ("/*\"x*/", false, 2),
            ("/* x", false, 4),
            ("/*\0*/", false, 2),
            ("/x", false, 1),
        ] {
            let (result, lexer) = scan(input, &[BLOCK_COMMENT]);
            assert_eq!(result, accepted, "{input:?}");
            assert_eq!(lexer.position, position, "{input:?}");
            assert_eq!(lexer.end, None, "{input:?}");
        }
    }

    #[test]
    fn whitespace_uses_c_locale_and_serialization_is_empty() {
        let (result, lexer) = scan("\u{b}1.0", &[FLOAT]);
        assert!(result);
        assert_eq!(lexer.events[0], Event::Advance(0, true));
        let (result, lexer) = scan("\u{a0}1.0", &[FLOAT]);
        assert!(!result);
        assert!(lexer.events.is_empty());
        let mut scanner = create();
        let mut buffer = [0xa5; 1024];
        assert_eq!(scanner.serialize(&mut buffer), 0);
        assert_eq!(buffer, [0xa5; 1024]);
        scanner.deserialize(&[1, 2, 3]);
        scanner.deserialize(&[]);
        assert_eq!(scanner.serialize(&mut []), 0);
    }
}
