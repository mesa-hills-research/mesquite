//! DTD's stateless external scanner, translated from `dtd/src/scanner.c`
//! and the non-`TS_XML` portions of `common/scanner.h`.

use tree_sitter_language::{ExternalScanner, Lexer};

// Indices in the grammar's external-token array, in C TokenType order.
const PI_TARGET: usize = 0;
const PI_CONTENT: usize = 1;
const COMMENT: usize = 2;

/// C's scanner has a null payload and no persistent state.
pub(crate) struct Scanner;

fn advance_if_eq(lexer: &mut dyn Lexer, ch: u8) -> bool {
    if !lexer.eof() && lexer.lookahead() == i32::from(ch) {
        lexer.advance(false);
        true
    } else {
        false
    }
}

// C's iswalpha/iswalnum follow the default C locale, not Unicode
// categories. The scanner explicitly allows middle dot only after the start.
fn is_valid_name_start_char(ch: i32) -> bool {
    matches!(ch, 0x41..=0x5a | 0x61..=0x7a | 0x5f | 0x3a)
}

fn is_valid_name_char(ch: i32) -> bool {
    is_valid_name_start_char(ch) || matches!(ch, 0x30..=0x39 | 0x2e | 0x2d | 0xb7)
}

fn scan_pi_target(lexer: &mut dyn Lexer) -> bool {
    if !is_valid_name_start_char(lexer.lookahead()) {
        return false;
    }

    let mut found_x_first = false;
    if lexer.lookahead() == i32::from(b'x') || lexer.lookahead() == i32::from(b'X') {
        found_x_first = true;
        lexer.mark_end();
    }
    lexer.advance(false);

    while is_valid_name_char(lexer.lookahead()) {
        if found_x_first
            && (lexer.lookahead() == i32::from(b'm') || lexer.lookahead() == i32::from(b'M'))
        {
            lexer.advance(false);
            if lexer.lookahead() == i32::from(b'l') || lexer.lookahead() == i32::from(b'L') {
                lexer.advance(false);
                if !is_valid_name_char(lexer.lookahead()) {
                    return false;
                }
            }
        }

        found_x_first = false;
        // C advances here even if the 'm' above was followed by a non-name
        // character (or EOF). Do not recheck the name predicate before advancing.
        lexer.advance(false);
    }

    lexer.mark_end();
    lexer.set_result_symbol(PI_TARGET as u16);
    true
}

fn scan_pi_content(lexer: &mut dyn Lexer) -> bool {
    while !lexer.eof()
        && lexer.lookahead() != i32::from(b'\n')
        && lexer.lookahead() != i32::from(b'?')
    {
        lexer.advance(false);
    }

    if lexer.lookahead() != i32::from(b'?') {
        return false;
    }

    lexer.mark_end();
    lexer.advance(false);

    if lexer.lookahead() == i32::from(b'>') {
        lexer.advance(false);
        while lexer.lookahead() == i32::from(b' ') {
            lexer.advance(false);
        }
        if !advance_if_eq(lexer, b'\n') {
            return false;
        }
        lexer.set_result_symbol(PI_CONTENT as u16);
        return true;
    }

    false
}

fn scan_comment(lexer: &mut dyn Lexer) -> bool {
    if !advance_if_eq(lexer, b'-') || !advance_if_eq(lexer, b'-') {
        return false;
    }

    while !lexer.eof() {
        if lexer.lookahead() == i32::from(b'-') {
            lexer.advance(false);
            if lexer.lookahead() == i32::from(b'-') {
                lexer.advance(false);
                break;
            }
        } else {
            lexer.advance(false);
        }
    }

    if lexer.lookahead() == i32::from(b'>') {
        lexer.advance(false);
        lexer.mark_end();
        lexer.set_result_symbol(COMMENT as u16);
        return true;
    }

    false
}

impl ExternalScanner for Scanner {
    fn scan(&mut self, lexer: &mut dyn Lexer, valid_symbols: &[bool]) -> bool {
        if valid_symbols[PI_TARGET] && valid_symbols[PI_CONTENT] && valid_symbols[COMMENT] {
            return false;
        }

        if valid_symbols[PI_TARGET] {
            return scan_pi_target(lexer);
        }

        if valid_symbols[PI_CONTENT] {
            return scan_pi_content(lexer);
        }

        if valid_symbols[COMMENT] {
            if !advance_if_eq(lexer, b'<') || !advance_if_eq(lexer, b'!') {
                return false;
            }
            return scan_comment(lexer);
        }

        false
    }

    fn serialize(&mut self, _buffer: &mut [u8]) -> usize {
        0
    }

    fn deserialize(&mut self, _buffer: &[u8]) {}
}

/// Creates a scanner (C's `tree_sitter_dtd_external_scanner_create`).
pub(crate) fn create() -> Box<dyn ExternalScanner> {
    Box::new(Scanner)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    #[derive(Debug, PartialEq, Eq)]
    enum Event {
        Advance(usize),
        Mark(usize),
        Eof(usize),
        Symbol(u16),
    }

    struct TestLexer {
        input: Vec<i32>,
        position: usize,
        symbol: u16,
        events: RefCell<Vec<Event>>,
    }

    impl TestLexer {
        fn new(input: &str) -> Self {
            Self {
                input: input.chars().map(|ch| ch as i32).collect(),
                position: 0,
                symbol: u16::MAX,
                events: RefCell::new(Vec::new()),
            }
        }

        fn marks(&self) -> Vec<usize> {
            self.events
                .borrow()
                .iter()
                .filter_map(|event| match event {
                    Event::Mark(position) => Some(*position),
                    _ => None,
                })
                .collect()
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
            self.events.get_mut().push(Event::Symbol(symbol));
            self.symbol = symbol;
        }

        fn advance(&mut self, skip: bool) {
            assert!(!skip, "DTD's scanner never skips whitespace");
            self.events.get_mut().push(Event::Advance(self.position));
            if self.position < self.input.len() {
                self.position += 1;
            }
        }

        fn mark_end(&mut self) {
            self.events.get_mut().push(Event::Mark(self.position));
        }

        fn get_column(&mut self) -> u32 {
            panic!("DTD's scanner never queries the column")
        }

        fn is_at_included_range_start(&self) -> bool {
            panic!("DTD's scanner never queries included-range starts")
        }

        fn eof(&self) -> bool {
            self.events.borrow_mut().push(Event::Eof(self.position));
            self.position == self.input.len()
        }
    }

    fn scan(input: &str, token: usize) -> (bool, TestLexer) {
        let mut lexer = TestLexer::new(input);
        let mut valid = [false; 3];
        valid[token] = true;
        let accepted = Scanner.scan(&mut lexer, &valid);
        (accepted, lexer)
    }

    #[test]
    fn pi_names_and_c_locale_character_classes() {
        for name in [
            "target",
            "_a",
            ":a",
            "a0_.-:\u{b7}",
            "xml-model",
            "xml-stylesheet",
        ] {
            let (accepted, lexer) = scan(&format!("{name} "), PI_TARGET);
            assert!(accepted, "{name}");
            assert_eq!(lexer.position, name.chars().count());
            assert_eq!(lexer.symbol, PI_TARGET as u16);
            assert_eq!(lexer.marks().last(), Some(&lexer.position));
        }
        for name in ["", "0abc", ".abc", "-abc", "\u{b7}abc", "éabc", "\0abc"] {
            let (accepted, lexer) = scan(name, PI_TARGET);
            assert!(!accepted, "{name}");
            assert_eq!(lexer.position, 0);
            assert!(lexer.events.borrow().is_empty());
        }
        let (accepted, lexer) = scan("aé ", PI_TARGET);
        assert!(accepted);
        assert_eq!(lexer.position, 1);
    }

    #[test]
    fn xml_target_is_reserved_case_insensitively() {
        for name in ["xml", "xmL", "xMl", "xML", "Xml", "XmL", "XMl", "XML"] {
            for suffix in ["", " ", "?>", "é"] {
                let (accepted, lexer) = scan(&format!("{name}{suffix}"), PI_TARGET);
                assert!(!accepted, "{name}{suffix}");
                assert_eq!(lexer.position, 3);
                assert_eq!(lexer.marks(), [0]);
                assert_eq!(lexer.symbol, u16::MAX);
            }
        }
        let (accepted, lexer) = scan("xxml ", PI_TARGET);
        assert!(accepted);
        assert_eq!(lexer.marks(), [0, 4]);
    }

    #[test]
    fn xm_prefix_preserves_unconditional_advance() {
        let (accepted, lexer) = scan("xm?>", PI_TARGET);
        assert!(accepted);
        assert_eq!(lexer.position, 3);
        assert_eq!(lexer.marks(), [0, 3]);

        let (accepted, lexer) = scan("xm", PI_TARGET);
        assert!(accepted);
        assert_eq!(
            *lexer.events.borrow(),
            [
                Event::Mark(0),
                Event::Advance(0),
                Event::Advance(1),
                Event::Advance(2), // The C scanner also calls advance at EOF.
                Event::Mark(2),
                Event::Symbol(PI_TARGET as u16),
            ]
        );
    }

    #[test]
    fn pi_content_marks_before_terminator_but_looks_through_newline() {
        let (accepted, lexer) = scan("a?> \nrest", PI_CONTENT);
        assert!(accepted);
        assert_eq!(lexer.position, 5);
        assert_eq!(
            *lexer.events.borrow(),
            [
                Event::Eof(0),
                Event::Advance(0),
                Event::Eof(1),
                Event::Mark(1),
                Event::Advance(1),
                Event::Advance(2),
                Event::Advance(3),
                Event::Eof(4),
                Event::Advance(4),
                Event::Symbol(PI_CONTENT as u16),
            ]
        );
        for input in ["?>\n", "text?>    \n", "a\0b?>\n"] {
            let (accepted, lexer) = scan(input, PI_CONTENT);
            assert!(accepted, "{input:?}");
            assert_eq!(lexer.marks(), [input.find('?').unwrap()]);
        }
    }

    #[test]
    fn pi_content_rejects_missing_newline_and_stops_at_first_question_mark() {
        for (input, position, mark) in [
            ("text", 4, None),
            ("text\n?>\n", 4, None),
            ("text?x?>\n", 5, Some(4)),
            ("text?>", 6, Some(4)),
            ("text?> \t\n", 7, Some(4)),
            ("text?>\r\n", 6, Some(4)),
        ] {
            let (accepted, lexer) = scan(input, PI_CONTENT);
            assert!(!accepted, "{input:?}");
            assert_eq!(lexer.position, position);
            assert_eq!(lexer.marks(), mark.into_iter().collect::<Vec<_>>());
            assert_eq!(lexer.symbol, u16::MAX);
        }
    }

    #[test]
    fn comments_end_at_first_double_hyphen() {
        for comment in ["<!---->", "<!--a-b-->", "<!--a\nb-->", "<!--a\0b-->"] {
            let (accepted, lexer) = scan(&format!("{comment}tail"), COMMENT);
            assert!(accepted, "{comment:?}");
            assert_eq!(lexer.position, comment.len());
            assert_eq!(lexer.marks(), [comment.len()]);
            assert_eq!(lexer.symbol, COMMENT as u16);
        }
        for (input, position) in [
            ("<!--a--x-->", 7),
            ("<!--a--->", 7),
            ("<!--a-", 6),
            ("<!--a>", 6),
            ("<!-", 3),
            ("<x", 1),
            ("x<!--a-->", 0),
        ] {
            let (accepted, lexer) = scan(input, COMMENT);
            assert!(!accepted, "{input:?}");
            assert_eq!(lexer.position, position);
            assert!(lexer.marks().is_empty());
        }
    }

    #[test]
    fn dispatch_has_priority_and_no_fallback_on_failure() {
        for mask in 0u8..8 {
            let valid = std::array::from_fn::<_, 3, _>(|i| mask & (1 << i) != 0);
            let mut lexer = TestLexer::new("<!--ok-->");
            assert_eq!(Scanner.scan(&mut lexer, &valid), mask == 4);
            if mask == 0 || mask == 7 || valid[PI_TARGET] {
                assert!(lexer.events.borrow().is_empty());
            }
        }
    }

    #[test]
    fn serialization_is_empty_and_deserialization_is_inert() {
        let mut scanner = create();
        let mut buffer = [0xab; 1024];
        assert_eq!(scanner.serialize(&mut buffer), 0);
        assert_eq!(buffer, [0xab; 1024]);
        for bytes in [&[][..], &[1, 2, 3][..]] {
            scanner.deserialize(bytes);
            let mut lexer = TestLexer::new("<!---->");
            assert!(scanner.scan(&mut lexer, &[false, false, true]));
            assert_eq!(scanner.serialize(&mut []), 0);
        }
    }
}
