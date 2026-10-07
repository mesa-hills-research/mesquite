//! The Dart external scanner, translated from `src/scanner.c`.

use ts_port_tables::{ExternalScanner, Lexer};

const TEMPLATE_CHARS_SINGLE: usize = 0;
const TEMPLATE_CHARS_DOUBLE: usize = 1;
const TEMPLATE_CHARS_SINGLE_SINGLE: usize = 2;
const TEMPLATE_CHARS_DOUBLE_SINGLE: usize = 3;
const TEMPLATE_CHARS_RAW_SLASH: usize = 4;
const BLOCK_COMMENT: usize = 5;
const DOCUMENTATION_BLOCK_COMMENT: usize = 6;
const ANNOTATION_OPEN_PAREN: usize = 7;

fn scan_multiline_comments(lexer: &mut dyn Lexer) -> bool {
    lexer.advance(false);
    if lexer.lookahead() != i32::from(b'*') {
        return false;
    }
    lexer.advance(false);
    let documentation_comment = lexer.lookahead() == i32::from(b'*');

    let mut after_star = false;
    let mut nesting_depth = 1u32;
    loop {
        match lexer.lookahead() {
            0 => return false,
            c if c == i32::from(b'*') => {
                lexer.advance(false);
                after_star = true;
            }
            c if c == i32::from(b'/') => {
                if after_star {
                    lexer.advance(false);
                    after_star = false;
                    nesting_depth = nesting_depth.wrapping_sub(1);
                    if nesting_depth == 0 {
                        lexer.set_result_symbol(if documentation_comment {
                            DOCUMENTATION_BLOCK_COMMENT as u16
                        } else {
                            BLOCK_COMMENT as u16
                        });
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

fn scan_templates(lexer: &mut dyn Lexer, valid_symbols: &[bool]) -> bool {
    lexer.set_result_symbol(if valid_symbols[TEMPLATE_CHARS_DOUBLE] {
        TEMPLATE_CHARS_DOUBLE as u16
    } else if valid_symbols[TEMPLATE_CHARS_SINGLE] {
        TEMPLATE_CHARS_SINGLE as u16
    } else if valid_symbols[TEMPLATE_CHARS_SINGLE_SINGLE] {
        TEMPLATE_CHARS_SINGLE_SINGLE as u16
    } else {
        TEMPLATE_CHARS_DOUBLE_SINGLE as u16
    });

    let mut has_content = false;
    loop {
        lexer.mark_end();
        match lexer.lookahead() {
            c if c == i32::from(b'\'') || c == i32::from(b'"') => return has_content,
            c if c == i32::from(b'\n') => {
                if valid_symbols[TEMPLATE_CHARS_DOUBLE_SINGLE]
                    || valid_symbols[TEMPLATE_CHARS_SINGLE_SINGLE]
                {
                    return false;
                }
                lexer.advance(false);
            }
            0 => return false,
            c if c == i32::from(b'$') => return has_content,
            c if c == i32::from(b'\\') => {
                if valid_symbols[TEMPLATE_CHARS_RAW_SLASH] {
                    lexer.set_result_symbol(TEMPLATE_CHARS_RAW_SLASH as u16);
                    lexer.advance(false);
                } else {
                    return has_content;
                }
            }
            _ => lexer.advance(false),
        }
        has_content = true;
    }
}

/// The C scanner has no persistent state.
pub(crate) struct Scanner;

impl ExternalScanner for Scanner {
    fn scan(&mut self, lexer: &mut dyn Lexer, valid_symbols: &[bool]) -> bool {
        if valid_symbols[TEMPLATE_CHARS_DOUBLE]
            || valid_symbols[TEMPLATE_CHARS_SINGLE]
            || valid_symbols[TEMPLATE_CHARS_DOUBLE_SINGLE]
            || valid_symbols[TEMPLATE_CHARS_SINGLE_SINGLE]
        {
            return scan_templates(lexer, valid_symbols);
        }

        // Annotation parentheses must be adjacent, before any whitespace skip.
        if valid_symbols[ANNOTATION_OPEN_PAREN] && lexer.lookahead() == i32::from(b'(') {
            lexer.advance(false);
            lexer.mark_end();
            lexer.set_result_symbol(ANNOTATION_OPEN_PAREN as u16);
            return true;
        }

        // `iswspace` in the reference's C locale includes vertical tab, but no
        // non-ASCII whitespace.
        while matches!(lexer.lookahead(), 0x09..=0x0d | 0x20) {
            lexer.advance(true);
        }

        // The C scanner does not check either comment token's validity here.
        if lexer.lookahead() == i32::from(b'/') {
            return scan_multiline_comments(lexer);
        }
        false
    }

    fn serialize(&mut self, _buffer: &mut [u8]) -> usize {
        0
    }

    fn deserialize(&mut self, _buffer: &[u8]) {}
}

/// Creates a scanner (C's `tree_sitter_dart_external_scanner_create`).
pub(crate) fn create() -> Box<dyn ExternalScanner> {
    Box::new(Scanner)
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
        input: Vec<i32>,
        position: usize,
        symbol: u16,
        events: Vec<Event>,
    }

    impl TestLexer {
        fn new(input: &str) -> Self {
            Self {
                input: input.chars().map(|c| c as i32).collect(),
                position: 0,
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
            self.events.push(Event::Symbol(symbol));
            self.symbol = symbol;
        }

        fn advance(&mut self, skip: bool) {
            assert!(self.position < self.input.len());
            self.events.push(Event::Advance(self.position, skip));
            self.position += 1;
        }

        fn mark_end(&mut self) {
            self.events.push(Event::MarkEnd(self.position));
        }

        fn get_column(&mut self) -> u32 {
            panic!("the C scanner never calls get_column")
        }

        fn is_at_included_range_start(&self) -> bool {
            panic!("the C scanner never checks included ranges")
        }

        fn eof(&self) -> bool {
            panic!("the C scanner checks NUL lookahead instead of calling eof")
        }
    }

    fn scan(input: &str, symbols: &[usize]) -> (bool, TestLexer) {
        let mut valid_symbols = [false; 8];
        for &symbol in symbols {
            valid_symbols[symbol] = true;
        }
        let mut lexer = TestLexer::new(input);
        let result = Scanner.scan(&mut lexer, &valid_symbols);
        (result, lexer)
    }

    #[test]
    fn serialization_is_empty_and_leaves_buffer_unchanged() {
        let mut scanner = create();
        let mut buffer = [0xa5; 1024];
        scanner.deserialize(&[]);
        scanner.deserialize(&[1, 2, 3]);
        assert_eq!(scanner.serialize(&mut buffer), 0);
        assert_eq!(buffer, [0xa5; 1024]);
        assert_eq!(scanner.serialize(&mut []), 0);
    }

    #[test]
    fn annotation_parenthesis_must_precede_whitespace() {
        let (ok, lexer) = scan("(args)", &[ANNOTATION_OPEN_PAREN]);
        assert!(ok);
        assert_eq!(
            lexer.events,
            [
                Event::Advance(0, false),
                Event::MarkEnd(1),
                Event::Symbol(7)
            ]
        );
        let (ok, lexer) = scan(" \t(args)", &[ANNOTATION_OPEN_PAREN]);
        assert!(!ok);
        assert_eq!(
            lexer.events,
            [Event::Advance(0, true), Event::Advance(1, true)]
        );
        let (ok, lexer) = scan("(args)", &[]);
        assert!(!ok);
        assert!(lexer.events.is_empty());
    }

    #[test]
    fn nested_comments_ignore_valid_symbols_and_do_not_mark_end() {
        for (input, symbol) in [
            ("/* plain */", BLOCK_COMMENT),
            ("/* outer /** inner */ end */", BLOCK_COMMENT),
            ("/**/", DOCUMENTATION_BLOCK_COMMENT),
            ("/*** doc /* inner */ end */", DOCUMENTATION_BLOCK_COMMENT),
            ("/* **x/ still open */", BLOCK_COMMENT),
        ] {
            let (ok, lexer) = scan(&format!("{input}tail"), &[]);
            assert!(ok, "{input}");
            assert_eq!(lexer.position, input.len());
            let mut events: Vec<_> = (0..input.len()).map(|p| Event::Advance(p, false)).collect();
            events.push(Event::Symbol(symbol as u16));
            assert_eq!(lexer.events, events, "{input}");
        }
    }

    #[test]
    fn comments_reject_nul_unclosed_nesting_and_line_comments() {
        for (input, consumed) in [("/*x", 3), ("/* /* */", 8), ("/*x\0*/", 3), ("//x", 1)] {
            let (ok, lexer) = scan(input, &[BLOCK_COMMENT, DOCUMENTATION_BLOCK_COMMENT]);
            assert!(!ok, "{input}");
            assert_eq!(lexer.position, consumed, "{input}");
            assert_eq!(lexer.symbol, u16::MAX);
        }
    }

    #[test]
    fn whitespace_uses_c_locale_and_is_skipped_only_outside_templates() {
        let (ok, lexer) = scan(" \t\n\r\u{b}\u{c}/**/", &[]);
        assert!(ok);
        assert_eq!(lexer.symbol, DOCUMENTATION_BLOCK_COMMENT as u16);
        for (position, event) in lexer.events[..6].iter().enumerate() {
            assert_eq!(*event, Event::Advance(position, true));
        }
        for input in ["\u{a0}/**/", "\u{2003}/**/"] {
            let (ok, lexer) = scan(input, &[]);
            assert!(!ok);
            assert!(lexer.events.is_empty());
        }
        let (ok, lexer) = scan(" \t'", &[TEMPLATE_CHARS_SINGLE]);
        assert!(ok);
        assert_eq!(
            lexer.events,
            [
                Event::Symbol(0),
                Event::MarkEnd(0),
                Event::Advance(0, false),
                Event::MarkEnd(1),
                Event::Advance(1, false),
                Event::MarkEnd(2),
            ]
        );
    }

    #[test]
    fn templates_stop_before_either_quote_dollar_or_nonraw_backslash() {
        for symbol in 0..=3 {
            for delimiter in ['\'', '"', '$', '\\'] {
                for content in ["", "abc"] {
                    let input = format!("{content}{delimiter}tail");
                    let (ok, lexer) = scan(&input, &[symbol]);
                    assert_eq!(ok, !content.is_empty(), "{input}");
                    assert_eq!(lexer.symbol, symbol as u16);
                    assert_eq!(lexer.position, content.len());
                    assert_eq!(lexer.events.last(), Some(&Event::MarkEnd(content.len())));
                }
            }
        }
    }

    #[test]
    fn templates_preserve_symbol_priority_and_take_precedence_over_annotations() {
        for (symbols, expected) in [
            (vec![0, 1, 2, 3, 7], TEMPLATE_CHARS_DOUBLE),
            (vec![0, 2, 3, 7], TEMPLATE_CHARS_SINGLE),
            (vec![2, 3, 7], TEMPLATE_CHARS_SINGLE_SINGLE),
            (vec![3, 7], TEMPLATE_CHARS_DOUBLE_SINGLE),
        ] {
            let (ok, lexer) = scan("(text'", &symbols);
            assert!(ok);
            assert_eq!(lexer.symbol, expected as u16);
            assert_eq!(lexer.position, 5);
        }
    }

    #[test]
    fn template_newlines_and_nul_follow_c_failure_rules() {
        for symbol in 0..=3 {
            let (ok, lexer) = scan("a\nb'", &[symbol]);
            assert_eq!(ok, symbol <= 1);
            assert_eq!(lexer.position, if symbol <= 1 { 3 } else { 1 });
            for input in ["abc", "abc\0'"] {
                let (ok, lexer) = scan(input, &[symbol]);
                assert!(!ok);
                assert_eq!(lexer.position, 3);
                assert_eq!(lexer.events.last(), Some(&Event::MarkEnd(3)));
            }
        }
        // A single-line validity flag rejects newlines even when the selected
        // result symbol is the higher-priority multiline token.
        assert!(
            !scan(
                "a\nb'",
                &[TEMPLATE_CHARS_DOUBLE, TEMPLATE_CHARS_SINGLE_SINGLE]
            )
            .0
        );
    }

    #[test]
    fn raw_slash_changes_symbol_and_keeps_scanning() {
        let (ok, lexer) = scan("a\\b'", &[TEMPLATE_CHARS_SINGLE, TEMPLATE_CHARS_RAW_SLASH]);
        assert!(ok);
        assert_eq!(
            lexer.events,
            [
                Event::Symbol(0),
                Event::MarkEnd(0),
                Event::Advance(0, false),
                Event::MarkEnd(1),
                Event::Symbol(4),
                Event::Advance(1, false),
                Event::MarkEnd(2),
                Event::Advance(2, false),
                Event::MarkEnd(3),
            ]
        );
        // RAW_SLASH alone does not enable the template scanner.
        let (ok, lexer) = scan("\\'", &[TEMPLATE_CHARS_RAW_SLASH]);
        assert!(!ok);
        assert!(lexer.events.is_empty());
    }
}
