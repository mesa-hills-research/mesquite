//! Elm's external scanner, translated from `src/scanner.c`.

use ts_port_tables::{ExternalScanner, Lexer, SERIALIZATION_BUFFER_SIZE};

const VIRTUAL_END_DECL: usize = 0;
const VIRTUAL_OPEN_SECTION: usize = 1;
const VIRTUAL_END_SECTION: usize = 2;
const MINUS_WITHOUT_TRAILING_WHITESPACE: usize = 3;
const GLSL_CONTENT: usize = 4;
const BLOCK_COMMENT_CONTENT: usize = 5;
const STRING_CONTENT_MULTILINE: usize = 6;
const MAX_INDENT_DEPTH: usize = 256;

/// Columns on the stack are deliberately bytes, whereas the measured column is
/// a u32. This narrowing, and the implicit first indent in snapshots, match C.
#[derive(Default)]
pub(crate) struct Scanner {
    indent_length: u32,
    indents: Vec<u8>,
    runback: Vec<u8>,
}

fn is_elm_space(lexer: &dyn Lexer) -> bool {
    matches!(lexer.lookahead(), 0x20 | 0x0d | 0x0a)
}

/// Even a partial match advances the lexer. Only 2 denotes a complete `in`.
fn check_for_in(lexer: &mut dyn Lexer, valid_symbols: &[bool]) -> u8 {
    if valid_symbols[VIRTUAL_END_SECTION] && lexer.lookahead() == i32::from(b'i') {
        lexer.advance(true);
        if lexer.lookahead() == i32::from(b'n') {
            lexer.advance(true);
            if is_elm_space(lexer) || lexer.eof() {
                return 2;
            }
        }
        return 1;
    }
    0
}

fn check_for_section_ending_token(lexer: &dyn Lexer, valid_symbols: &[bool]) -> bool {
    valid_symbols[VIRTUAL_END_SECTION] && matches!(lexer.lookahead(), 0x29 | 0x2c | 0x7d)
}

fn scan_block_comment(lexer: &mut dyn Lexer) -> bool {
    lexer.mark_end();
    if lexer.lookahead() != i32::from(b'{') {
        return false;
    }
    lexer.advance(false);
    if lexer.lookahead() != i32::from(b'-') {
        return false;
    }
    lexer.advance(false);

    // C recurses for nested comments. A depth counter preserves every advance
    // and mark_end (including failed nested openers) without growing the call stack.
    let mut depth = 1usize;
    loop {
        match lexer.lookahead() {
            0x7b => {
                lexer.mark_end();
                lexer.advance(false);
                if lexer.lookahead() == i32::from(b'-') {
                    lexer.advance(false);
                    depth += 1;
                }
            }
            0x2d => {
                lexer.advance(false);
                if lexer.lookahead() == i32::from(b'}') {
                    lexer.advance(false);
                    depth -= 1;
                    if depth == 0 {
                        return true;
                    }
                }
            }
            0 => return true,
            _ => lexer.advance(false),
        }
    }
}

fn advance_to_line_end(lexer: &mut dyn Lexer) {
    while lexer.lookahead() != i32::from(b'\n') && !lexer.eof() {
        lexer.advance(false);
    }
}

/// Look past a comment when its column would otherwise close a section. Unlike
/// scan_block_comment, this path never marks the end and terminates on eof().
/// The opening `{-` has already been consumed.
fn advance_past_block_comment(lexer: &mut dyn Lexer) {
    let mut nesting = 1usize;
    while nesting > 0 && !lexer.eof() {
        if lexer.lookahead() == i32::from(b'{') {
            lexer.advance(false);
            if lexer.lookahead() == i32::from(b'-') {
                lexer.advance(false);
                nesting += 1;
            }
        } else if lexer.lookahead() == i32::from(b'-') {
            lexer.advance(false);
            if lexer.lookahead() == i32::from(b'}') {
                lexer.advance(false);
                nesting -= 1;
            }
        } else {
            lexer.advance(false);
        }
    }
}

impl Scanner {
    fn emit_runback(&mut self, lexer: &mut dyn Lexer, valid_symbols: &[bool]) -> bool {
        if self.runback.last() == Some(&0) && valid_symbols[VIRTUAL_END_DECL] {
            self.runback.pop();
            lexer.set_result_symbol(VIRTUAL_END_DECL as u16);
            return true;
        }
        if self.runback.last() == Some(&1) && valid_symbols[VIRTUAL_END_SECTION] {
            self.runback.pop();
            lexer.set_result_symbol(VIRTUAL_END_SECTION as u16);
            return true;
        }
        false
    }

    fn advance_whitespace_after_comment(&mut self, lexer: &mut dyn Lexer) {
        while matches!(lexer.lookahead(), 0x20 | 0x0a | 0x0d | 0x09) {
            if lexer.lookahead() == i32::from(b'\n') {
                lexer.advance(false);
                while lexer.lookahead() == i32::from(b' ') {
                    lexer.advance(false);
                }
                self.indent_length = lexer.get_column();
            } else {
                lexer.advance(false);
            }
        }
    }
}

impl ExternalScanner for Scanner {
    fn scan(&mut self, lexer: &mut dyn Lexer, valid_symbols: &[bool]) -> bool {
        // All tokens being valid signals error recovery.
        if valid_symbols[..=STRING_CONTENT_MULTILINE]
            .iter()
            .all(|&v| v)
        {
            return false;
        }

        if self.emit_runback(lexer, valid_symbols) {
            return true;
        }
        self.runback.clear();

        // String contents precede whitespace/comments, so `--` inside a string
        // cannot be mistaken for a line comment.
        if valid_symbols[STRING_CONTENT_MULTILINE] {
            lexer.set_result_symbol(STRING_CONTENT_MULTILINE as u16);
            let mut has_content = false;
            loop {
                match lexer.lookahead() {
                    0x22 => {
                        lexer.mark_end();
                        lexer.advance(false);
                        if lexer.lookahead() == i32::from(b'"') {
                            lexer.advance(false);
                            if lexer.lookahead() == i32::from(b'"') {
                                return has_content;
                            }
                        }
                        // One or two quotes, but not three, are content.
                        has_content = true;
                    }
                    0x5c | 0 => {
                        lexer.mark_end();
                        return has_content;
                    }
                    _ => {
                        has_content = true;
                        lexer.advance(false);
                    }
                }
            }
        }

        let mut has_newline = false;
        let mut found_in = false;
        let mut can_call_mark_end = true;
        lexer.mark_end();
        loop {
            if matches!(lexer.lookahead(), 0x20 | 0x0d) {
                lexer.advance(true);
            } else if lexer.lookahead() == i32::from(b'\n') {
                lexer.advance(true);
                has_newline = true;
                while lexer.lookahead() == i32::from(b' ') {
                    lexer.advance(true);
                }
                self.indent_length = lexer.get_column();
            } else if !valid_symbols[BLOCK_COMMENT_CONTENT] && lexer.lookahead() == i32::from(b'-')
            {
                lexer.advance(false);
                let lookahead = lexer.lookahead();
                if valid_symbols[MINUS_WITHOUT_TRAILING_WHITESPACE]
                    && (matches!(lookahead, 0x61..=0x7a | 0x41..=0x5a | 0x28) || lookahead > 127)
                {
                    if can_call_mark_end {
                        lexer.set_result_symbol(MINUS_WITHOUT_TRAILING_WHITESPACE as u16);
                        lexer.mark_end();
                        return true;
                    }
                    return false;
                }
                // Line comments after a newline are whitespace for indentation
                // purposes, but no subsequent mark_end may swallow the comment.
                if lookahead == i32::from(b'-') && has_newline {
                    can_call_mark_end = false;
                    lexer.advance(false);
                    advance_to_line_end(lexer);
                } else if valid_symbols[BLOCK_COMMENT_CONTENT]
                    && lexer.lookahead() == i32::from(b'}')
                {
                    lexer.set_result_symbol(BLOCK_COMMENT_CONTENT as u16);
                    return true;
                } else {
                    return false;
                }
            } else if lexer.eof() {
                if valid_symbols[VIRTUAL_END_SECTION] {
                    lexer.set_result_symbol(VIRTUAL_END_SECTION as u16);
                    return true;
                }
                if valid_symbols[VIRTUAL_END_DECL] {
                    lexer.set_result_symbol(VIRTUAL_END_DECL as u16);
                    return true;
                }
                break;
            } else {
                break;
            }
        }

        if check_for_in(lexer, valid_symbols) == 2 {
            if has_newline {
                found_in = true;
            } else {
                lexer.set_result_symbol(VIRTUAL_END_SECTION as u16);
                self.indents.pop();
                return true;
            }
        }

        if check_for_section_ending_token(lexer, valid_symbols) {
            lexer.set_result_symbol(VIRTUAL_END_SECTION as u16);
            self.indents.pop();
            return true;
        }

        if valid_symbols[VIRTUAL_OPEN_SECTION] && !lexer.eof() {
            if self.indents.len() >= MAX_INDENT_DEPTH {
                return false;
            }
            self.indents.push(lexer.get_column() as u8);
            lexer.set_result_symbol(VIRTUAL_OPEN_SECTION as u16);
            return true;
        }

        if valid_symbols[BLOCK_COMMENT_CONTENT] {
            if !can_call_mark_end {
                return false;
            }
            lexer.mark_end();
            loop {
                if lexer.lookahead() == 0 {
                    break;
                }
                if lexer.lookahead() != i32::from(b'{') && lexer.lookahead() != i32::from(b'-') {
                    lexer.advance(false);
                } else if lexer.lookahead() == i32::from(b'-') {
                    lexer.mark_end();
                    lexer.advance(false);
                    if lexer.lookahead() == i32::from(b'}') {
                        break;
                    }
                } else if scan_block_comment(lexer) {
                    lexer.mark_end();
                }
            }
            lexer.set_result_symbol(BLOCK_COMMENT_CONTENT as u16);
            return true;
        }

        if has_newline {
            self.runback.clear();

            // A less-indented block comment need not mean the code after it is
            // less indented. Look past it without moving the token's end.
            if lexer.lookahead() == i32::from(b'{')
                && !valid_symbols[BLOCK_COMMENT_CONTENT]
                && self
                    .indents
                    .last()
                    .is_some_and(|&indent| self.indent_length < u32::from(indent))
            {
                lexer.advance(false);
                if lexer.lookahead() == i32::from(b'-') {
                    can_call_mark_end = false;
                    lexer.advance(false);
                    advance_past_block_comment(lexer);
                    self.advance_whitespace_after_comment(lexer);
                    while lexer.lookahead() == i32::from(b'{') {
                        lexer.advance(false);
                        if lexer.lookahead() == i32::from(b'-') {
                            lexer.advance(false);
                            advance_past_block_comment(lexer);
                            self.advance_whitespace_after_comment(lexer);
                        } else {
                            break;
                        }
                    }
                }
            }

            while let Some(&indent) = self.indents.last() {
                if self.indent_length > u32::from(indent) {
                    break;
                }
                if self.indent_length == u32::from(indent) {
                    if found_in {
                        self.indents.pop();
                        self.runback.push(1);
                        found_in = false;
                        break;
                    }
                    // Incoming comments do not end the declaration. Failed
                    // comment checks still consume their first character.
                    if lexer.lookahead() == i32::from(b'-') {
                        lexer.advance(true);
                        if lexer.lookahead() == i32::from(b'-') {
                            break;
                        }
                    }
                    if lexer.lookahead() == i32::from(b'{') {
                        lexer.advance(true);
                        if lexer.lookahead() == i32::from(b'-') {
                            break;
                        }
                    }
                    self.runback.push(0);
                    break;
                }
                self.indents.pop();
                self.runback.push(1);
                if found_in
                    && self
                        .indents
                        .last()
                        .is_none_or(|&indent| self.indent_length > u32::from(indent))
                {
                    found_in = false;
                }
            }

            // A let may start on the previous line at a different indentation.
            if found_in && !self.indents.is_empty() {
                self.indents.pop();
                self.runback.push(1);
            }

            self.runback.reverse();
            if self.emit_runback(lexer, valid_symbols) {
                return true;
            }
            if lexer.eof() && valid_symbols[VIRTUAL_END_SECTION] {
                lexer.set_result_symbol(VIRTUAL_END_SECTION as u16);
                return true;
            }
        }

        if valid_symbols[GLSL_CONTENT] {
            if !can_call_mark_end {
                return false;
            }
            lexer.set_result_symbol(GLSL_CONTENT as u16);
            loop {
                match lexer.lookahead() {
                    0x7c => {
                        lexer.mark_end();
                        lexer.advance(false);
                        if lexer.lookahead() == i32::from(b']') {
                            lexer.advance(false);
                            return true;
                        }
                    }
                    0 => {
                        lexer.mark_end();
                        return true;
                    }
                    _ => lexer.advance(false),
                }
            }
        }
        false
    }

    fn serialize(&mut self, buffer: &mut [u8]) -> usize {
        if 3 + self.indents.len() + self.runback.len() >= SERIALIZATION_BUFFER_SIZE {
            return 0;
        }
        let runback_count = self.runback.len().min(u8::MAX as usize);
        let limit = buffer.len().min(SERIALIZATION_BUFFER_SIZE);
        // The runtime supplies 1024 bytes; also handle smaller callers safely.
        if runback_count + 6 > limit {
            return 0;
        }
        buffer[0] = runback_count as u8;
        buffer[1..1 + runback_count].copy_from_slice(&self.runback[..runback_count]);
        let mut size = 1 + runback_count;
        buffer[size] = 4;
        size += 1;
        buffer[size..size + 4].copy_from_slice(&self.indent_length.to_ne_bytes());
        size += 4;
        // The base indent is implicit, even when the in-memory stack is empty.
        for &indent in self.indents.iter().skip(1).take(limit - size) {
            buffer[size] = indent;
            size += 1;
        }
        size
    }

    fn deserialize(&mut self, buffer: &[u8]) {
        self.runback.clear();
        self.indents.clear();
        self.indents.push(0);
        // C intentionally retains indent_length on an empty snapshot.
        if buffer.is_empty() {
            return;
        }

        let runback_count = usize::from(buffer[0]);
        let Some(runback) = buffer.get(1..1 + runback_count) else {
            return;
        };
        self.runback.extend_from_slice(runback);
        let mut size = 1 + runback_count;
        let Some(&indent_length_length) = buffer.get(size) else {
            return;
        };
        size += 1;
        let indent_length_length = usize::from(indent_length_length);
        // C's memcpy has undefined behavior for malformed snapshots. Preserve
        // even partial native-endian copies, but reject out-of-bounds ones.
        let Some(bytes) = buffer.get(size..size + indent_length_length) else {
            return;
        };
        let mut column = self.indent_length.to_ne_bytes();
        let Some(destination) = column.get_mut(..indent_length_length) else {
            return;
        };
        destination.copy_from_slice(bytes);
        self.indent_length = u32::from_ne_bytes(column);
        size += indent_length_length;
        self.indents.extend_from_slice(&buffer[size..]);
    }
}

pub(crate) fn create() -> Box<dyn ExternalScanner> {
    Box::<Scanner>::default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    #[derive(Debug, PartialEq, Eq)]
    enum Call {
        Advance(usize, bool),
        Mark(usize),
        Column(usize),
        Eof(usize),
        Symbol(u16),
    }

    struct TestLexer {
        input: Vec<i32>,
        position: usize,
        column: u32,
        end: usize,
        symbol: u16,
        calls: RefCell<Vec<Call>>,
    }

    impl TestLexer {
        fn new(input: &str) -> Self {
            Self {
                input: input.chars().map(|c| c as i32).collect(),
                position: 0,
                column: 0,
                end: 0,
                symbol: u16::MAX,
                calls: RefCell::new(Vec::new()),
            }
        }

        fn marks(&self) -> Vec<usize> {
            self.calls
                .borrow()
                .iter()
                .filter_map(|call| match call {
                    Call::Mark(position) => Some(*position),
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
            self.symbol = symbol;
            self.calls.borrow_mut().push(Call::Symbol(symbol));
        }

        fn advance(&mut self, skip: bool) {
            self.calls
                .borrow_mut()
                .push(Call::Advance(self.position, skip));
            assert!(self.position < self.input.len(), "advanced beyond eof");
            if self.lookahead() == i32::from(b'\n') {
                self.column = 0;
            } else {
                self.column += 1;
            }
            self.position += 1;
        }

        fn mark_end(&mut self) {
            self.end = self.position;
            self.calls.borrow_mut().push(Call::Mark(self.position));
        }

        fn get_column(&mut self) -> u32 {
            self.calls.borrow_mut().push(Call::Column(self.position));
            self.column
        }

        fn is_at_included_range_start(&self) -> bool {
            false
        }

        fn eof(&self) -> bool {
            self.calls.borrow_mut().push(Call::Eof(self.position));
            self.position == self.input.len()
        }
    }

    fn valid(tokens: &[usize]) -> [bool; 7] {
        let mut valid = [false; 7];
        for &token in tokens {
            valid[token] = true;
        }
        valid
    }

    fn scanner(indents: &[u8]) -> Scanner {
        Scanner {
            indents: indents.to_vec(),
            ..Scanner::default()
        }
    }

    #[test]
    fn snapshots_preserve_native_column_bytes_and_implicit_base_indent() {
        let mut scanner = Scanner {
            indent_length: 0x1234_abcd,
            indents: vec![0, 4, 10, 255],
            runback: vec![0, 1, 1],
        };
        let mut buffer = [0xcc; SERIALIZATION_BUFFER_SIZE];
        let size = scanner.serialize(&mut buffer);
        let mut expected = vec![3, 0, 1, 1, 4];
        expected.extend_from_slice(&0x1234_abcdu32.to_ne_bytes());
        expected.extend_from_slice(&[4, 10, 255]);
        assert_eq!(&buffer[..size], expected);
        assert_eq!(buffer[size], 0xcc);

        let mut restored = Scanner::default();
        assert!(restored.indents.is_empty()); // create does not seed the stack
        restored.deserialize(&buffer[..size]);
        assert_eq!(restored.indent_length, scanner.indent_length);
        assert_eq!(restored.runback, scanner.runback);
        assert_eq!(restored.indents, scanner.indents);
        restored.deserialize(&[]);
        assert_eq!(restored.indent_length, 0x1234_abcd);
        assert_eq!(restored.indents, [0]);
        assert!(restored.runback.is_empty());

        // A zero or partial column size preserves the other native-endian bytes.
        restored.deserialize(&[0, 0, 9]);
        assert_eq!(restored.indent_length, 0x1234_abcd);
        assert_eq!(restored.indents, [0, 9]);
        restored.deserialize(&[0, 2, 0x55, 0x66, 7]);
        let mut column = 0x1234_abcdu32.to_ne_bytes();
        column[..2].copy_from_slice(&[0x55, 0x66]);
        assert_eq!(restored.indent_length, u32::from_ne_bytes(column));
        assert_eq!(restored.indents, [0, 7]);
    }

    #[test]
    fn snapshot_limits_use_full_counts_before_clamping_runback() {
        let mut scanner = scanner(&[0, 255]);
        scanner.runback = (0..256).map(|n| n as u8).collect();
        let mut buffer = [0; SERIALIZATION_BUFFER_SIZE];
        let size = scanner.serialize(&mut buffer);
        assert_eq!(size, 262);
        assert_eq!(buffer[0], 255);
        assert_eq!(&buffer[1..256], &scanner.runback[..255]);
        assert_eq!(buffer[256], 4);
        assert_eq!(buffer[261], 255);
        scanner.runback.resize(1019, 0);
        assert_eq!(scanner.serialize(&mut buffer), 0); // 3 + 2 + 1019 == 1024
    }

    #[test]
    fn dedents_queue_sections_before_the_end_declaration() {
        let mut scanner = scanner(&[0, 4, 8]);
        let valid = valid(&[VIRTUAL_END_DECL, VIRTUAL_END_SECTION]);
        let mut lexer = TestLexer::new("\nx");
        assert!(scanner.scan(&mut lexer, &valid));
        assert_eq!(lexer.symbol, VIRTUAL_END_SECTION as u16);
        assert_eq!(lexer.end, 0);
        assert_eq!(scanner.indents, [0]);
        assert_eq!(scanner.runback, [0, 1]);

        for symbol in [VIRTUAL_END_SECTION, VIRTUAL_END_DECL] {
            let mut lexer = TestLexer::new("x");
            assert!(scanner.scan(&mut lexer, &valid));
            assert_eq!(*lexer.calls.borrow(), [Call::Symbol(symbol as u16)]);
        }
        assert!(scanner.runback.is_empty());
    }

    #[test]
    fn recovery_keeps_runback_but_an_invalid_queued_token_clears_it() {
        let mut scanner = scanner(&[0]);
        scanner.runback = vec![0, 1];
        let mut lexer = TestLexer::new("x");
        assert!(!scanner.scan(&mut lexer, &[true; 7]));
        assert_eq!(scanner.runback, [0, 1]);
        assert!(lexer.calls.borrow().is_empty());
        assert!(!scanner.scan(&mut lexer, &valid(&[VIRTUAL_END_DECL])));
        assert!(scanner.runback.is_empty());
    }

    #[test]
    fn in_and_partial_in_keep_their_skipped_lookahead() {
        let mut scanner = scanner(&[0, 4]);
        let mut lexer = TestLexer::new("in x");
        assert!(scanner.scan(&mut lexer, &valid(&[VIRTUAL_END_SECTION])));
        assert_eq!(scanner.indents, [0]);
        assert_eq!(lexer.end, 0);
        assert_eq!(lexer.position, 2);
        assert_eq!(
            *lexer.calls.borrow(),
            [
                Call::Mark(0),
                Call::Eof(0),
                Call::Advance(0, true),
                Call::Advance(1, true),
                Call::Symbol(VIRTUAL_END_SECTION as u16),
            ]
        );

        let mut lexer = TestLexer::new("index");
        assert!(scanner.scan(
            &mut lexer,
            &valid(&[VIRTUAL_END_SECTION, VIRTUAL_OPEN_SECTION])
        ));
        assert_eq!(lexer.symbol, VIRTUAL_OPEN_SECTION as u16);
        assert_eq!(lexer.position, 2);
        assert_eq!(scanner.indents, [0, 2]);

        let mut lexer = TestLexer::new("in\t"); // tab is not Elm space
        assert!(!scanner.scan(&mut lexer, &valid(&[VIRTUAL_END_SECTION])));
        assert_eq!(lexer.position, 2);
        assert_eq!(scanner.indents, [0, 2]);
    }

    #[test]
    fn newline_in_closes_both_inner_and_matching_sections() {
        let mut scanner = scanner(&[0, 4, 8]);
        let mut lexer = TestLexer::new("\n    in x");
        assert!(scanner.scan(&mut lexer, &valid(&[VIRTUAL_END_SECTION])));
        assert_eq!(scanner.indents, [0]);
        assert_eq!(scanner.runback, [1]);
        assert_eq!(lexer.end, 0);
        assert_eq!(lexer.position, 7);
    }

    #[test]
    fn punctuation_pops_one_section_but_eof_does_not_pop() {
        let mut scanner = scanner(&[0, 4]);
        for input in [")", ",", "}"] {
            let mut lexer = TestLexer::new(input);
            let count = scanner.indents.len();
            assert!(scanner.scan(&mut lexer, &valid(&[VIRTUAL_END_SECTION])));
            assert_eq!(lexer.position, 0);
            assert_eq!(scanner.indents.len(), count.saturating_sub(1));
        }
        scanner.indents = vec![0, 4];
        let mut lexer = TestLexer::new("");
        assert!(scanner.scan(&mut lexer, &valid(&[VIRTUAL_END_DECL, VIRTUAL_END_SECTION])));
        assert_eq!(lexer.symbol, VIRTUAL_END_SECTION as u16);
        assert_eq!(scanner.indents, [0, 4]);
    }

    #[test]
    fn open_section_narrows_column_and_enforces_depth_limit() {
        let mut scanner = scanner(&[0]);
        let mut lexer = TestLexer::new("x");
        lexer.column = 300;
        assert!(scanner.scan(&mut lexer, &valid(&[VIRTUAL_OPEN_SECTION])));
        assert_eq!(scanner.indents, [0, 44]);
        scanner.indents.resize(MAX_INDENT_DEPTH, 0);
        let mut lexer = TestLexer::new("x");
        assert!(!scanner.scan(&mut lexer, &valid(&[VIRTUAL_OPEN_SECTION])));
        assert_eq!(scanner.indents.len(), MAX_INDENT_DEPTH);
        assert!(!lexer.calls.borrow().contains(&Call::Column(0)));
    }

    #[test]
    fn multiline_strings_stop_before_quotes_and_escapes_not_comments() {
        let cases = [
            ("\"\"\"tail", false, 0, 2),
            ("x\"\"\"tail", true, 1, 3),
            ("x\"\"y\\n", true, 4, 4),
            ("--x\n y\"\"\"", true, 6, 8),
            ("abc\0tail", true, 3, 3),
            ("\"", true, 1, 1),
            ("\"\"", true, 2, 2),
            ("\\n", false, 0, 0),
        ];
        for (input, result, end, position) in cases {
            let mut scanner = Scanner::default();
            let mut lexer = TestLexer::new(input);
            assert_eq!(
                scanner.scan(&mut lexer, &valid(&[STRING_CONTENT_MULTILINE])),
                result,
                "{input:?}"
            );
            assert_eq!((lexer.end, lexer.position), (end, position), "{input:?}");
            assert_eq!(lexer.symbol, STRING_CONTENT_MULTILINE as u16);
            assert!(!lexer.calls.borrow().iter().any(|call| matches!(
                call,
                Call::Advance(_, true) | Call::Eof(_) | Call::Column(_)
            )));
        }
    }

    #[test]
    fn block_comment_marks_match_nested_and_unterminated_paths() {
        let mut scanner = Scanner::default();
        for (input, position, marks) in [
            ("a{-b{-c-}d-}e-}", 14, vec![0, 0, 1, 4, 12, 13]),
            ("{x-}", 3, vec![0, 0, 0, 2]),
            ("abc", 3, vec![0, 0]),
            ("abc-}", 4, vec![0, 0, 3]),
            ("a{-b", 4, vec![0, 0, 1, 4]),
        ] {
            let mut lexer = TestLexer::new(input);
            assert!(scanner.scan(&mut lexer, &valid(&[BLOCK_COMMENT_CONTENT])));
            assert_eq!(lexer.position, position, "{input:?}");
            assert_eq!(lexer.marks(), marks, "{input:?}");
            assert_eq!(lexer.symbol, BLOCK_COMMENT_CONTENT as u16);
        }

        let text = format!("{}{}", "{-".repeat(5000), "-}".repeat(5000));
        let mut lexer = TestLexer::new(&text);
        assert!(scan_block_comment(&mut lexer));
        assert_eq!(lexer.position, text.len());
        assert_eq!(lexer.marks(), (0..10000).step_by(2).collect::<Vec<_>>());
    }

    #[test]
    fn minus_accepts_unicode_but_cannot_swallow_a_preceding_comment() {
        let mut scanner = Scanner::default();
        for input in ["-x", "-X", "-(", "-α"] {
            let mut lexer = TestLexer::new(input);
            assert!(scanner.scan(&mut lexer, &valid(&[MINUS_WITHOUT_TRAILING_WHITESPACE])));
            assert_eq!(
                *lexer.calls.borrow(),
                [
                    Call::Mark(0),
                    Call::Advance(0, false),
                    Call::Symbol(MINUS_WITHOUT_TRAILING_WHITESPACE as u16),
                    Call::Mark(1),
                ]
            );
        }
        for input in ["-9", "- x", "\n--a\n-x"] {
            let mut lexer = TestLexer::new(input);
            assert!(!scanner.scan(&mut lexer, &valid(&[MINUS_WITHOUT_TRAILING_WHITESPACE])));
            assert_eq!(lexer.marks(), [0]);
        }
    }

    #[test]
    fn less_indented_comments_remeasure_code_without_moving_the_end() {
        for input in ["\n{-c-}\n    x", "\n{-a{-nested-}-}\n{-b-}\n    x"] {
            let mut scanner = scanner(&[0, 4]);
            let mut lexer = TestLexer::new(input);
            assert!(scanner.scan(&mut lexer, &valid(&[VIRTUAL_END_DECL, VIRTUAL_END_SECTION])));
            assert_eq!(lexer.symbol, VIRTUAL_END_DECL as u16);
            assert_eq!(scanner.indents, [0, 4]);
            assert_eq!(scanner.indent_length, 4);
            assert_eq!(lexer.marks(), [0]);
            // Only the first newline uses skip; looking past comments doesn't.
            let skipped: Vec<_> = lexer
                .calls
                .borrow()
                .iter()
                .filter_map(|call| match call {
                    Call::Advance(position, true) => Some(*position),
                    _ => None,
                })
                .collect();
            assert_eq!(skipped, [0]);
        }
    }

    #[test]
    fn glsl_looks_through_its_closer_but_marks_before_it() {
        let mut scanner = Scanner::default();
        let mut lexer = TestLexer::new("body|]tail");
        assert!(scanner.scan(&mut lexer, &valid(&[GLSL_CONTENT])));
        assert_eq!(lexer.symbol, GLSL_CONTENT as u16);
        assert_eq!(lexer.position, 6);
        assert_eq!(lexer.marks(), [0, 4]);
        let mut lexer = TestLexer::new("abc|x");
        assert!(scanner.scan(&mut lexer, &valid(&[GLSL_CONTENT])));
        assert_eq!(lexer.marks(), [0, 3, 5]);
        let mut lexer = TestLexer::new("\n--comment\nbody|]");
        assert!(!scanner.scan(&mut lexer, &valid(&[GLSL_CONTENT])));
        assert_eq!(lexer.marks(), [0]);
    }
}
