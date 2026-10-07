//! The XML external scanner, translated from `xml/src/scanner.c` and
//! `common/scanner.h` (with `TS_XML` defined).

use ts_port_tables::{ExternalScanner, Lexer, SERIALIZATION_BUFFER_SIZE};

// External token indices, in the order of common/scanner.h's TokenType.
const PI_TARGET: usize = 0;
const PI_CONTENT: usize = 1;
const COMMENT: usize = 2;
const CHAR_DATA: usize = 3;
const CDATA: usize = 4;
const XML_MODEL: usize = 5;
const XML_STYLESHEET: usize = 6;
const START_TAG_NAME: usize = 7;
const END_TAG_NAME: usize = 8;
const ERRONEOUS_END_NAME: usize = 9;
const SELF_CLOSING_TAG_DELIMITER: usize = 10;

/// C stores each name as an array of narrowed `char` bytes, not UTF-8.
#[derive(Default)]
pub(crate) struct Scanner {
    tags: Vec<Vec<u8>>,
}

fn advance_if_eq(lexer: &mut dyn Lexer, ch: u8) -> bool {
    if !lexer.eof() && lexer.lookahead() == i32::from(ch) {
        lexer.advance(false);
        true
    } else {
        false
    }
}

fn is_valid_name_start_char(ch: i32) -> bool {
    // The reference runs in the C locale: iswalpha only accepts ASCII letters.
    matches!(ch, 0x41..=0x5a | 0x61..=0x7a | 0x5f | 0x3a)
}

fn is_valid_name_char(ch: i32) -> bool {
    is_valid_name_start_char(ch) || matches!(ch, 0x30..=0x39 | 0x2e | 0x2d | 0xb7)
}

fn check_word(lexer: &mut dyn Lexer, word: &[u8]) -> bool {
    for &ch in word {
        if !advance_if_eq(lexer, ch) {
            return false;
        }
    }
    true
}

fn scan_pi_target(lexer: &mut dyn Lexer, valid_symbols: &[bool]) -> bool {
    let mut advanced_once = false;
    let mut found_x_first = false;

    if is_valid_name_start_char(lexer.lookahead()) {
        if matches!(lexer.lookahead(), 0x78 | 0x58) {
            found_x_first = true;
            lexer.mark_end();
        }
        advanced_once = true;
        lexer.advance(false);
    }

    if advanced_once {
        while is_valid_name_char(lexer.lookahead()) {
            if found_x_first && matches!(lexer.lookahead(), 0x6d | 0x4d) {
                lexer.advance(false);
                if matches!(lexer.lookahead(), 0x6c | 0x4c) {
                    lexer.advance(false);
                    if is_valid_name_char(lexer.lookahead()) {
                        // C also clears found_x_first here; the loop tail does so below.
                        let last_char_hyphen = lexer.lookahead() == i32::from(b'-');
                        lexer.advance(false);
                        if last_char_hyphen {
                            if valid_symbols[XML_MODEL] && check_word(lexer, b"model") {
                                return false;
                            }
                            // A failed check_word keeps its consumed prefix, just as C does.
                            if valid_symbols[XML_STYLESHEET] && check_word(lexer, b"stylesheet") {
                                return false;
                            }
                        }
                    } else {
                        return false;
                    }
                }
            }

            // Preserve this extra advance, even after the XML-prefix lookahead.
            found_x_first = false;
            lexer.advance(false);
        }

        lexer.mark_end();
        lexer.set_result_symbol(PI_TARGET as u16);
        return true;
    }

    false
}

fn scan_pi_content(lexer: &mut dyn Lexer) -> bool {
    while !lexer.eof() && !matches!(lexer.lookahead(), 0x0a | 0x3f) {
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

fn scan_tag_name(lexer: &mut dyn Lexer) -> Vec<u8> {
    let mut name = Vec::new();
    if is_valid_name_start_char(lexer.lookahead()) {
        name.push(lexer.lookahead() as u8);
        lexer.advance(false);
    }
    // C still runs this loop if the first character was not a name-start character.
    while is_valid_name_char(lexer.lookahead()) {
        name.push(lexer.lookahead() as u8);
        lexer.advance(false);
    }
    name
}

fn in_char_data(lexer: &dyn Lexer) -> bool {
    !lexer.eof() && !matches!(lexer.lookahead(), 0x3c | 0x26)
}

fn scan_char_data(lexer: &mut dyn Lexer) -> bool {
    let mut advanced_once = false;
    while in_char_data(lexer) {
        if lexer.lookahead() == i32::from(b']') {
            lexer.mark_end();
            lexer.advance(false);
            if lexer.lookahead() == i32::from(b']') {
                lexer.advance(false);
                if lexer.lookahead() == i32::from(b'>') {
                    lexer.advance(false);
                    if advanced_once {
                        lexer.set_result_symbol(CHAR_DATA as u16);
                        return false;
                    }
                }
            }
        }
        advanced_once = true;
        if in_char_data(lexer) {
            lexer.advance(false);
        }
    }

    if advanced_once {
        lexer.mark_end();
        lexer.set_result_symbol(CHAR_DATA as u16);
        return true;
    }
    false
}

fn scan_cdata(lexer: &mut dyn Lexer) -> bool {
    let mut advanced_once = false;
    while !lexer.eof() {
        if lexer.lookahead() == i32::from(b']') {
            lexer.mark_end();
            lexer.advance(false);
            if lexer.lookahead() == i32::from(b']') {
                lexer.advance(false);
                if lexer.lookahead() == i32::from(b'>') && advanced_once {
                    lexer.set_result_symbol(CDATA as u16);
                    return true;
                }
            }
        }
        advanced_once = true;
        lexer.advance(false);
    }
    false
}

impl Scanner {
    fn scan_start_tag_name(&mut self, lexer: &mut dyn Lexer) -> bool {
        let name = scan_tag_name(lexer);
        if name.is_empty() {
            return false;
        }
        lexer.set_result_symbol(START_TAG_NAME as u16);
        self.tags.push(name);
        true
    }

    fn scan_end_tag_name(&mut self, lexer: &mut dyn Lexer) -> bool {
        let name = scan_tag_name(lexer);
        if name.is_empty() {
            return false;
        }
        if self.tags.last() == Some(&name) {
            self.tags.pop();
            lexer.set_result_symbol(END_TAG_NAME as u16);
        } else {
            lexer.set_result_symbol(ERRONEOUS_END_NAME as u16);
        }
        lexer.result_symbol() == END_TAG_NAME as u16
    }

    fn scan_self_closing_tag_delimiter(&mut self, lexer: &mut dyn Lexer) -> bool {
        lexer.advance(false);
        if !advance_if_eq(lexer, b'>') {
            return false;
        }
        if !self.tags.is_empty() {
            self.tags.pop();
            lexer.set_result_symbol(SELF_CLOSING_TAG_DELIMITER as u16);
        }
        true
    }
}

impl ExternalScanner for Scanner {
    fn scan(&mut self, lexer: &mut dyn Lexer, valid_symbols: &[bool]) -> bool {
        if valid_symbols[PI_TARGET]
            && valid_symbols[PI_CONTENT]
            && valid_symbols[COMMENT]
            && valid_symbols[CHAR_DATA]
            && valid_symbols[CDATA]
        {
            return false;
        }
        if valid_symbols[PI_TARGET] {
            return scan_pi_target(lexer, valid_symbols);
        }
        if valid_symbols[PI_CONTENT] {
            return scan_pi_content(lexer);
        }
        if valid_symbols[CHAR_DATA] && scan_char_data(lexer) {
            return true;
        }
        if valid_symbols[CDATA] && scan_cdata(lexer) {
            return true;
        }

        match lexer.lookahead() {
            0x3c => {
                lexer.mark_end();
                lexer.advance(false);
                if lexer.lookahead() == i32::from(b'!') {
                    lexer.advance(false);
                    return scan_comment(lexer);
                }
            }
            0x2f => {
                if valid_symbols[SELF_CLOSING_TAG_DELIMITER] {
                    return self.scan_self_closing_tag_delimiter(lexer);
                }
            }
            0 => {}
            _ => {
                if valid_symbols[START_TAG_NAME] {
                    return self.scan_start_tag_name(lexer);
                }
                if valid_symbols[END_TAG_NAME] {
                    return self.scan_end_tag_name(lexer);
                }
            }
        }
        false
    }

    fn serialize(&mut self, buffer: &mut [u8]) -> usize {
        let limit = buffer.len().min(SERIALIZATION_BUFFER_SIZE);
        if limit < 8 {
            return 0;
        }
        let tag_count = self.tags.len().min(u16::MAX as usize);
        let mut serialized_tag_count = 0u32;
        let mut size = 8;
        buffer[4..8].copy_from_slice(&(tag_count as u32).to_ne_bytes());

        for tag in self.tags.iter_mut().take(tag_count) {
            let name_length = tag.len().min(u8::MAX as usize);
            // This deliberately budgets two bytes although only one length byte is written.
            if size + 2 + name_length >= limit {
                break;
            }
            buffer[size] = name_length as u8;
            size += 1;
            buffer[size..size + name_length].copy_from_slice(&tag[..name_length]);
            // The C serializer deletes every serialized name, but retains its stack slot.
            *tag = Vec::new();
            size += name_length;
            serialized_tag_count += 1;
        }
        buffer[..4].copy_from_slice(&serialized_tag_count.to_ne_bytes());
        size
    }

    fn deserialize(&mut self, buffer: &[u8]) {
        self.tags = Vec::new();
        if buffer.is_empty() {
            return;
        }
        // Only complete snapshots come from the runtime. Reject malformed short input
        // rather than reproducing C's out-of-bounds reads.
        let Some(header) = buffer.get(..8) else {
            return;
        };
        let serialized_tag_count = u32::from_ne_bytes(header[..4].try_into().unwrap()) as usize;
        let tag_count = u32::from_ne_bytes(header[4..8].try_into().unwrap()) as usize;
        if tag_count == 0 {
            return;
        }
        self.tags.reserve(tag_count);
        let mut size = 8;
        for _ in 0..serialized_tag_count {
            let Some(&name_length) = buffer.get(size) else {
                self.tags.clear();
                return;
            };
            size += 1;
            let Some(name) = buffer.get(size..size + usize::from(name_length)) else {
                self.tags.clear();
                return;
            };
            self.tags.push(name.to_vec());
            size += usize::from(name_length);
        }
        // Unserialized tags remain on the stack as empty names.
        while self.tags.len() < tag_count {
            self.tags.push(Vec::new());
        }
    }
}

/// Creates a scanner (C's `tree_sitter_xml_external_scanner_create`).
pub(crate) fn create() -> Box<dyn ExternalScanner> {
    Box::new(Scanner::default())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Eq)]
    enum Event {
        Advance(usize),
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
                input: input.chars().map(|ch| ch as i32).collect(),
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
            assert!(!skip, "XML never skips characters via the lexer");
            self.events.push(Event::Advance(self.position));
            if self.position < self.input.len() {
                self.position += 1;
            }
        }
        fn mark_end(&mut self) {
            self.end = Some(self.position);
            self.events.push(Event::Mark(self.position));
        }
        fn get_column(&mut self) -> u32 {
            panic!("XML does not query the column")
        }
        fn is_at_included_range_start(&self) -> bool {
            panic!("XML does not query included ranges")
        }
        fn eof(&self) -> bool {
            self.position == self.input.len()
        }
    }

    fn valid(tokens: &[usize]) -> [bool; 11] {
        let mut result = [false; 11];
        for &token in tokens {
            result[token] = true;
        }
        result
    }

    #[test]
    fn names_use_c_locale_and_narrowed_bytes() {
        let mut scanner = Scanner::default();
        let mut lexer = TestLexer::new("-1:tag·name>");
        assert!(scanner.scan(&mut lexer, &valid(&[START_TAG_NAME])));
        assert_eq!(lexer.symbol, START_TAG_NAME as u16);
        assert_eq!(lexer.lookahead(), i32::from(b'>'));
        assert_eq!(lexer.end, None);
        assert_eq!(scanner.tags, [b"-1:tag\xb7name".to_vec()]);

        let mut lexer = TestLexer::new("-1:tag·name>");
        assert!(scanner.scan(&mut lexer, &valid(&[END_TAG_NAME])));
        assert_eq!(lexer.symbol, END_TAG_NAME as u16);
        assert!(scanner.tags.is_empty());

        let mut lexer = TestLexer::new("é>");
        assert!(!scanner.scan(&mut lexer, &valid(&[START_TAG_NAME])));
        assert_eq!(lexer.position, 0);
        let mut lexer = TestLexer::new("abcé>");
        assert!(scanner.scan(&mut lexer, &valid(&[START_TAG_NAME])));
        assert_eq!(lexer.position, 3);
        assert_eq!(scanner.tags, [b"abc".to_vec()]);
    }

    #[test]
    fn mismatched_end_tag_does_not_pop() {
        let mut scanner = Scanner {
            tags: vec![b"outer".to_vec(), b"inner".to_vec()],
        };
        let mut lexer = TestLexer::new("outer>");
        assert!(!scanner.scan(&mut lexer, &valid(&[END_TAG_NAME])));
        assert_eq!(lexer.symbol, ERRONEOUS_END_NAME as u16);
        assert_eq!(scanner.tags.len(), 2);
    }

    #[test]
    fn self_closing_delimiter_leaves_symbol_unchanged_on_empty_stack() {
        let mut scanner = Scanner::default();
        let mut lexer = TestLexer::new("/>");
        assert!(scanner.scan(&mut lexer, &valid(&[SELF_CLOSING_TAG_DELIMITER])));
        assert_eq!(lexer.symbol, u16::MAX);
        assert_eq!(lexer.events, [Event::Advance(0), Event::Advance(1)]);

        scanner.tags.push(b"tag".to_vec());
        let mut lexer = TestLexer::new("/x");
        assert!(!scanner.scan(&mut lexer, &valid(&[SELF_CLOSING_TAG_DELIMITER])));
        assert_eq!(lexer.position, 1);
        assert_eq!(scanner.tags.len(), 1);
        let mut lexer = TestLexer::new("/>");
        assert!(scanner.scan(&mut lexer, &valid(&[SELF_CLOSING_TAG_DELIMITER])));
        assert_eq!(lexer.symbol, SELF_CLOSING_TAG_DELIMITER as u16);
        assert!(scanner.tags.is_empty());
    }

    #[test]
    fn pi_target_preserves_xml_prefix_lookahead() {
        for input in ["xml ", "XML?", "xMl"] {
            let mut lexer = TestLexer::new(input);
            assert!(!scan_pi_target(&mut lexer, &valid(&[PI_TARGET])));
            assert_eq!(lexer.position, 3);
            assert_eq!(lexer.end, Some(0));
            assert_eq!(lexer.symbol, u16::MAX);
        }
        for (input, enabled) in [
            ("xml-model ", vec![PI_TARGET, XML_MODEL]),
            ("xml-stylesheet ", vec![PI_TARGET, XML_STYLESHEET]),
            // The second word check starts after the first check's consumed prefix.
            (
                "xml-modstylesheet ",
                vec![PI_TARGET, XML_MODEL, XML_STYLESHEET],
            ),
        ] {
            let mut lexer = TestLexer::new(input);
            assert!(!scan_pi_target(&mut lexer, &valid(&enabled)));
            assert_eq!(lexer.position, input.len() - 1);
            assert_eq!(lexer.end, Some(0));
        }
        let mut lexer = TestLexer::new("xmlx target ");
        assert!(scan_pi_target(&mut lexer, &valid(&[PI_TARGET])));
        assert_eq!(lexer.end, Some(11)); // The loop-tail advance consumes the first space.
        assert_eq!(lexer.symbol, PI_TARGET as u16);
        let mut lexer = TestLexer::new("1name ");
        assert!(!scan_pi_target(&mut lexer, &valid(&[PI_TARGET])));
        assert_eq!(lexer.position, 0);
    }

    #[test]
    fn pi_content_requires_newline_after_terminator() {
        let mut lexer = TestLexer::new("text?>  \nrest");
        assert!(scan_pi_content(&mut lexer));
        assert_eq!(lexer.position, 9);
        assert_eq!(lexer.end, Some(4));
        assert_eq!(lexer.symbol, PI_CONTENT as u16);
        for input in ["text?>", "text?>\r\n", "text?x", "text\n?>\n"] {
            let mut lexer = TestLexer::new(input);
            assert!(!scan_pi_content(&mut lexer));
            assert_eq!(lexer.symbol, u16::MAX);
        }
    }

    #[test]
    fn char_data_preserves_bracket_and_failure_behavior() {
        let mut lexer = TestLexer::new("]]>x<");
        assert!(scan_char_data(&mut lexer));
        assert_eq!(lexer.position, 4);
        assert_eq!(lexer.end, Some(4));
        assert_eq!(lexer.symbol, CHAR_DATA as u16);

        let mut lexer = TestLexer::new("a]]>x<");
        assert!(!scan_char_data(&mut lexer));
        assert_eq!(lexer.position, 4);
        assert_eq!(lexer.end, Some(1));
        assert_eq!(lexer.symbol, CHAR_DATA as u16);

        // A failed CharData attempt does not rewind before scanning a comment.
        let mut lexer = TestLexer::new("a]]><!--x-->");
        assert!(Scanner::default().scan(&mut lexer, &valid(&[CHAR_DATA])));
        assert_eq!(lexer.symbol, COMMENT as u16);
        assert_eq!(lexer.end, Some(12));
    }

    #[test]
    fn cdata_marks_before_brackets_but_leaves_greater_than_unconsumed() {
        let mut lexer = TestLexer::new("x]]>");
        assert!(scan_cdata(&mut lexer));
        assert_eq!(
            lexer.events,
            [
                Event::Advance(0),
                Event::Mark(1),
                Event::Advance(1),
                Event::Advance(2),
                Event::Symbol(CDATA as u16),
            ]
        );
        let mut lexer = TestLexer::new("]]>");
        assert!(!scan_cdata(&mut lexer));
        assert_eq!(lexer.position, 3);
        assert_eq!(lexer.end, Some(0));
        let mut lexer = TestLexer::new("]");
        assert!(!scan_cdata(&mut lexer));
        assert_eq!(
            lexer.events,
            [Event::Mark(0), Event::Advance(0), Event::Advance(1)]
        );
    }

    #[test]
    fn comments_ignore_validity_but_recovery_returns_without_lexing() {
        let mut scanner = Scanner::default();
        let mut lexer = TestLexer::new("<!--x-->");
        assert!(scanner.scan(&mut lexer, &valid(&[])));
        assert_eq!(lexer.end, Some(8));
        assert_eq!(lexer.events.first(), Some(&Event::Mark(0)));
        assert_eq!(lexer.symbol, COMMENT as u16);

        let mut lexer = TestLexer::new("<!--x-->");
        let recovering = valid(&[PI_TARGET, PI_CONTENT, COMMENT, CHAR_DATA, CDATA]);
        assert!(!scanner.scan(&mut lexer, &recovering));
        assert!(lexer.events.is_empty());

        let mut lexer = TestLexer::new("<!--x--y-->");
        assert!(!scanner.scan(&mut lexer, &valid(&[COMMENT])));
        assert_eq!(lexer.position, 7); // The first double hyphen must be followed by '>'.
        assert_eq!(lexer.end, Some(0));
    }

    #[test]
    fn serialization_uses_native_u32_counts_and_deletes_serialized_names() {
        let mut scanner = Scanner {
            tags: vec![b"root".to_vec(), b"a\xb7".to_vec()],
        };
        let mut buffer = [0xcc; SERIALIZATION_BUFFER_SIZE];
        let size = scanner.serialize(&mut buffer);
        let mut expected = Vec::new();
        expected.extend(2u32.to_ne_bytes());
        expected.extend(2u32.to_ne_bytes());
        expected.extend(b"\x04root\x02a\xb7");
        assert_eq!(&buffer[..size], expected);
        assert_eq!(buffer[size], 0xcc);
        assert_eq!(scanner.tags, [Vec::<u8>::new(), Vec::new()]);

        let mut restored = Scanner::default();
        restored.deserialize(&buffer[..size]);
        assert_eq!(restored.tags, [b"root".to_vec(), b"a\xb7".to_vec()]);
        assert_eq!(scanner.serialize(&mut buffer), 10);
        assert_eq!(&buffer[8..10], &[0, 0]);
        restored.deserialize(&[]);
        assert!(restored.tags.is_empty());
        assert_eq!(restored.serialize(&mut buffer), 8);
        assert_eq!(&buffer[..8], &[0; 8]);
    }

    #[test]
    fn serialization_truncates_names_and_pads_unserialized_tags() {
        let mut scanner = Scanner {
            tags: vec![vec![b'a'; 300]; 4],
        };
        let mut buffer = [0; SERIALIZATION_BUFFER_SIZE];
        let size = scanner.serialize(&mut buffer);
        assert_eq!(size, 8 + 3 * 256);
        assert_eq!(&buffer[..4], &3u32.to_ne_bytes());
        assert_eq!(&buffer[4..8], &4u32.to_ne_bytes());
        assert!(scanner.tags[..3].iter().all(Vec::is_empty));
        assert_eq!(scanner.tags[3].len(), 300);
        scanner.deserialize(&buffer[..size]);
        assert_eq!(
            scanner.tags,
            [vec![b'a'; 255], vec![b'a'; 255], vec![b'a'; 255], vec![]]
        );
    }

    #[test]
    fn serialization_preserves_strict_limit_and_capped_tag_count() {
        let mut scanner = Scanner {
            tags: vec![
                vec![b'a'; 255],
                vec![b'b'; 255],
                vec![b'c'; 255],
                vec![b'd'; 246],
            ],
        };
        let mut buffer = [0; SERIALIZATION_BUFFER_SIZE];
        // 776 + 2 + 246 == 1024: rejected despite the actual entry fitting.
        assert_eq!(scanner.serialize(&mut buffer), 776);
        assert_eq!(scanner.tags[3].len(), 246);

        scanner.tags = vec![Vec::new(); usize::from(u16::MAX) + 1];
        let size = scanner.serialize(&mut buffer);
        assert_eq!(size, 1022);
        assert_eq!(&buffer[..4], &1014u32.to_ne_bytes());
        assert_eq!(&buffer[4..8], &u32::from(u16::MAX).to_ne_bytes());
        scanner.deserialize(&buffer[..size]);
        assert_eq!(scanner.tags.len(), usize::from(u16::MAX));
        assert!(scanner.tags.iter().all(Vec::is_empty));
    }
}
