//! The Python external scanner, translated from `src/scanner.c`.

use tree_sitter_language::{ExternalScanner, Lexer, SERIALIZATION_BUFFER_SIZE};

// Indices in the grammar's external-token array. COMMENT (7) is only a sentinel
// for the parser; this scanner does not inspect or return it.
const NEWLINE: usize = 0;
const INDENT: usize = 1;
const DEDENT: usize = 2;
const STRING_START: usize = 3;
const STRING_CONTENT: usize = 4;
const ESCAPE_INTERPOLATION: usize = 5;
const STRING_END: usize = 6;
const CLOSE_PAREN: usize = 8;
const CLOSE_BRACKET: usize = 9;
const CLOSE_BRACE: usize = 10;
const EXCEPT: usize = 11;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct Delimiter {
    flags: u8,
}

impl Delimiter {
    const SINGLE_QUOTE: u8 = 1 << 0;
    const DOUBLE_QUOTE: u8 = 1 << 1;
    const BACK_QUOTE: u8 = 1 << 2;
    const RAW: u8 = 1 << 3;
    const FORMAT: u8 = 1 << 4;
    const TRIPLE: u8 = 1 << 5;
    const BYTES: u8 = 1 << 6;

    fn is_format(self) -> bool {
        self.flags & Self::FORMAT != 0
    }

    fn is_raw(self) -> bool {
        self.flags & Self::RAW != 0
    }

    fn is_triple(self) -> bool {
        self.flags & Self::TRIPLE != 0
    }

    fn is_bytes(self) -> bool {
        self.flags & Self::BYTES != 0
    }

    fn end_character(self) -> i32 {
        if self.flags & Self::SINGLE_QUOTE != 0 {
            i32::from(b'\'')
        } else if self.flags & Self::DOUBLE_QUOTE != 0 {
            i32::from(b'"')
        } else if self.flags & Self::BACK_QUOTE != 0 {
            i32::from(b'`')
        } else {
            0
        }
    }
}

/// The scanner's state (C's `payload`). Vec ownership also replaces `destroy`.
pub(crate) struct Scanner {
    indents: Vec<u16>,
    delimiters: Vec<Delimiter>,
    inside_interpolated_string: bool,
}

impl Default for Scanner {
    fn default() -> Self {
        Self {
            indents: vec![0],
            delimiters: Vec::new(),
            inside_interpolated_string: false,
        }
    }
}

impl ExternalScanner for Scanner {
    fn scan(&mut self, lexer: &mut dyn Lexer, valid_symbols: &[bool]) -> bool {
        let error_recovery_mode = valid_symbols[STRING_CONTENT] && valid_symbols[INDENT];
        let within_brackets = valid_symbols[CLOSE_BRACE]
            || valid_symbols[CLOSE_PAREN]
            || valid_symbols[CLOSE_BRACKET];

        if valid_symbols[ESCAPE_INTERPOLATION]
            && !self.delimiters.is_empty()
            && (lexer.lookahead() == i32::from(b'{') || lexer.lookahead() == i32::from(b'}'))
            && !error_recovery_mode
        {
            let delimiter = *self.delimiters.last().unwrap();
            if delimiter.is_format() {
                lexer.mark_end();
                let is_left_brace = lexer.lookahead() == i32::from(b'{');
                lexer.advance(false);
                if (lexer.lookahead() == i32::from(b'{') && is_left_brace)
                    || (lexer.lookahead() == i32::from(b'}') && !is_left_brace)
                {
                    lexer.advance(false);
                    lexer.mark_end();
                    lexer.set_result_symbol(ESCAPE_INTERPOLATION as u16);
                    return true;
                }
                return false;
            }
        }

        if valid_symbols[STRING_CONTENT] && !self.delimiters.is_empty() && !error_recovery_mode {
            let delimiter = *self.delimiters.last().unwrap();
            let end_char = delimiter.end_character();
            // C's `advanced_once` is always false here: both paths after setting
            // it in the interpolation-escape branch return immediately.
            let mut has_content = false;
            while lexer.lookahead() != 0 {
                if (lexer.lookahead() == i32::from(b'{') || lexer.lookahead() == i32::from(b'}'))
                    && delimiter.is_format()
                {
                    lexer.mark_end();
                    lexer.set_result_symbol(STRING_CONTENT as u16);
                    return has_content;
                }
                if lexer.lookahead() == i32::from(b'\\') {
                    if delimiter.is_raw() {
                        // Raw escapes do not set has_content, just as in C.
                        lexer.advance(false);
                        if lexer.lookahead() == delimiter.end_character()
                            || lexer.lookahead() == i32::from(b'\\')
                        {
                            lexer.advance(false);
                        }
                        if lexer.lookahead() == i32::from(b'\r') {
                            lexer.advance(false);
                            if lexer.lookahead() == i32::from(b'\n') {
                                lexer.advance(false);
                            }
                        } else if lexer.lookahead() == i32::from(b'\n') {
                            lexer.advance(false);
                        }
                        continue;
                    }
                    if delimiter.is_bytes() {
                        lexer.mark_end();
                        lexer.advance(false);
                        if lexer.lookahead() == i32::from(b'N')
                            || lexer.lookahead() == i32::from(b'u')
                            || lexer.lookahead() == i32::from(b'U')
                        {
                            // These are not escape sequences in bytes strings.
                            lexer.advance(false);
                        } else {
                            lexer.set_result_symbol(STRING_CONTENT as u16);
                            return has_content;
                        }
                    } else {
                        lexer.mark_end();
                        lexer.set_result_symbol(STRING_CONTENT as u16);
                        return has_content;
                    }
                } else if lexer.lookahead() == end_char {
                    if delimiter.is_triple() {
                        lexer.mark_end();
                        lexer.advance(false);
                        if lexer.lookahead() == end_char {
                            lexer.advance(false);
                            if lexer.lookahead() == end_char {
                                if has_content {
                                    lexer.set_result_symbol(STRING_CONTENT as u16);
                                } else {
                                    lexer.advance(false);
                                    lexer.mark_end();
                                    self.delimiters.pop();
                                    lexer.set_result_symbol(STRING_END as u16);
                                    self.inside_interpolated_string = false;
                                }
                                return true;
                            }
                            lexer.mark_end();
                            lexer.set_result_symbol(STRING_CONTENT as u16);
                            return true;
                        }
                        lexer.mark_end();
                        lexer.set_result_symbol(STRING_CONTENT as u16);
                        return true;
                    }
                    if has_content {
                        lexer.set_result_symbol(STRING_CONTENT as u16);
                    } else {
                        lexer.advance(false);
                        self.delimiters.pop();
                        lexer.set_result_symbol(STRING_END as u16);
                        self.inside_interpolated_string = false;
                    }
                    lexer.mark_end();
                    return true;
                } else if lexer.lookahead() == i32::from(b'\n')
                    && has_content
                    && !delimiter.is_triple()
                {
                    return false;
                }
                lexer.advance(false);
                has_content = true;
            }
        }

        lexer.mark_end();

        let mut found_end_of_line = false;
        let mut indent_length = 0u16;
        let mut first_comment_indent_length = -1i32;
        loop {
            if lexer.lookahead() == i32::from(b'\n') {
                found_end_of_line = true;
                indent_length = 0;
                lexer.advance(true);
            } else if lexer.lookahead() == i32::from(b' ') {
                indent_length = indent_length.wrapping_add(1);
                lexer.advance(true);
            } else if lexer.lookahead() == i32::from(b'\r')
                || lexer.lookahead() == i32::from(b'\x0c')
            {
                indent_length = 0;
                lexer.advance(true);
            } else if lexer.lookahead() == i32::from(b'\t') {
                indent_length = indent_length.wrapping_add(8);
                lexer.advance(true);
            } else if lexer.lookahead() == i32::from(b'#')
                && (valid_symbols[INDENT]
                    || valid_symbols[DEDENT]
                    || valid_symbols[NEWLINE]
                    || valid_symbols[EXCEPT])
            {
                // An inline comment must not generate an indent/dedent token.
                if !found_end_of_line {
                    return false;
                }
                if first_comment_indent_length == -1 {
                    first_comment_indent_length = i32::from(indent_length);
                }
                while lexer.lookahead() != 0 && lexer.lookahead() != i32::from(b'\n') {
                    lexer.advance(true);
                }
                lexer.advance(true);
                indent_length = 0;
            } else if lexer.lookahead() == i32::from(b'\\') {
                lexer.advance(true);
                if lexer.lookahead() == i32::from(b'\r') {
                    lexer.advance(true);
                }
                if lexer.lookahead() == i32::from(b'\n') || lexer.eof() {
                    lexer.advance(true);
                } else {
                    return false;
                }
            } else if lexer.eof() {
                indent_length = 0;
                found_end_of_line = true;
                break;
            } else {
                break;
            }
        }

        if found_end_of_line {
            if let Some(&current_indent_length) = self.indents.last() {
                if valid_symbols[INDENT] && indent_length > current_indent_length {
                    self.indents.push(indent_length);
                    lexer.set_result_symbol(INDENT as u16);
                    return true;
                }

                let next_tok_is_string_start = lexer.lookahead() == i32::from(b'"')
                    || lexer.lookahead() == i32::from(b'\'')
                    || lexer.lookahead() == i32::from(b'`');

                if (valid_symbols[DEDENT]
                    || (!valid_symbols[NEWLINE]
                        && !(valid_symbols[STRING_START] && next_tok_is_string_start)
                        && !within_brackets))
                    && indent_length < current_indent_length
                    && !self.inside_interpolated_string
                    // Wait to dedent until comments indented to the current
                    // block have been consumed by the internal lexer.
                    && first_comment_indent_length < i32::from(current_indent_length)
                {
                    self.indents.pop();
                    lexer.set_result_symbol(DEDENT as u16);
                    return true;
                }
            }

            if valid_symbols[NEWLINE] && !error_recovery_mode {
                lexer.set_result_symbol(NEWLINE as u16);
                return true;
            }
        }

        if first_comment_indent_length == -1 && valid_symbols[STRING_START] {
            let mut delimiter = Delimiter::default();
            let mut has_flags = false;
            while lexer.lookahead() != 0 {
                if lexer.lookahead() == i32::from(b'f')
                    || lexer.lookahead() == i32::from(b'F')
                    || lexer.lookahead() == i32::from(b't')
                    || lexer.lookahead() == i32::from(b'T')
                {
                    delimiter.flags |= Delimiter::FORMAT;
                } else if lexer.lookahead() == i32::from(b'r')
                    || lexer.lookahead() == i32::from(b'R')
                {
                    delimiter.flags |= Delimiter::RAW;
                } else if lexer.lookahead() == i32::from(b'b')
                    || lexer.lookahead() == i32::from(b'B')
                {
                    delimiter.flags |= Delimiter::BYTES;
                } else if lexer.lookahead() != i32::from(b'u')
                    && lexer.lookahead() != i32::from(b'U')
                {
                    break;
                }
                has_flags = true;
                lexer.advance(false);
            }

            if lexer.lookahead() == i32::from(b'`') {
                delimiter.flags |= Delimiter::BACK_QUOTE;
                lexer.advance(false);
                lexer.mark_end();
            } else if lexer.lookahead() == i32::from(b'\'') || lexer.lookahead() == i32::from(b'"')
            {
                let quote = lexer.lookahead();
                delimiter.flags |= if quote == i32::from(b'\'') {
                    Delimiter::SINGLE_QUOTE
                } else {
                    Delimiter::DOUBLE_QUOTE
                };
                lexer.advance(false);
                lexer.mark_end();
                if lexer.lookahead() == quote {
                    lexer.advance(false);
                    if lexer.lookahead() == quote {
                        lexer.advance(false);
                        lexer.mark_end();
                        delimiter.flags |= Delimiter::TRIPLE;
                    }
                }
            }

            if delimiter.end_character() != 0 {
                self.delimiters.push(delimiter);
                lexer.set_result_symbol(STRING_START as u16);
                self.inside_interpolated_string = delimiter.is_format();
                return true;
            }
            if has_flags {
                return false;
            }
        }

        false
    }

    fn serialize(&mut self, buffer: &mut [u8]) -> usize {
        let delimiter_count = self.delimiters.len().min(usize::from(u8::MAX));
        buffer[0] = u8::from(self.inside_interpolated_string);
        buffer[1] = delimiter_count as u8;
        for (slot, delimiter) in buffer[2..2 + delimiter_count]
            .iter_mut()
            .zip(&self.delimiters)
        {
            *slot = delimiter.flags;
        }
        let mut size = 2 + delimiter_count;
        for &indent in self.indents.iter().skip(1) {
            // C only checks size < 1024, which can write one byte past the
            // buffer with an odd delimiter count. Do not write a partial u16
            // or overflow the supplied slice in that undefined-behavior case.
            if size >= SERIALIZATION_BUFFER_SIZE || size + 2 > buffer.len() {
                break;
            }
            buffer[size..size + 2].copy_from_slice(&indent.to_le_bytes());
            size += 2;
        }
        size
    }

    fn deserialize(&mut self, buffer: &[u8]) {
        self.delimiters.clear();
        self.indents.clear();
        self.indents.push(0);

        // In C an empty buffer leaves inside_interpolated_string unchanged.
        // In particular, it is not derived from the restored delimiter stack.
        if !buffer.is_empty() {
            self.inside_interpolated_string = buffer[0] != 0;
            let delimiter_count = usize::from(buffer[1]);
            let end = 2 + delimiter_count;
            self.delimiters
                .extend(buffer[2..end].iter().map(|&flags| Delimiter { flags }));
            for &pair in buffer[end..].as_chunks::<2>().0 {
                self.indents.push(u16::from_le_bytes(pair));
            }
        }
    }
}

/// Creates a scanner (C's `tree_sitter_python_external_scanner_create`).
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
        MarkEnd(usize),
        Eof(usize),
        Result(u16),
    }

    struct TestLexer {
        input: Vec<i32>,
        position: usize,
        end: Option<usize>,
        symbol: u16,
        calls: RefCell<Vec<Call>>,
    }

    impl TestLexer {
        fn new(input: &str) -> Self {
            Self {
                input: input.chars().map(|c| c as i32).collect(),
                position: 0,
                end: None,
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
            self.calls.borrow_mut().push(Call::Result(symbol));
        }

        fn advance(&mut self, skip: bool) {
            self.calls
                .borrow_mut()
                .push(Call::Advance(self.position, skip));
            if self.position < self.input.len() {
                self.position += 1;
            }
        }

        fn mark_end(&mut self) {
            self.end = Some(self.position);
            self.calls.borrow_mut().push(Call::MarkEnd(self.position));
        }

        fn get_column(&mut self) -> u32 {
            panic!("the Python scanner does not call get_column")
        }

        fn is_at_included_range_start(&self) -> bool {
            panic!("the Python scanner does not check included ranges")
        }

        fn eof(&self) -> bool {
            self.calls.borrow_mut().push(Call::Eof(self.position));
            self.position == self.input.len()
        }
    }

    fn scan(scanner: &mut Scanner, input: &str, symbols: &[usize]) -> (bool, TestLexer) {
        let mut lexer = TestLexer::new(input);
        let mut valid = [false; 12];
        for &symbol in symbols {
            valid[symbol] = true;
        }
        let accepted = scanner.scan(&mut lexer, &valid);
        (accepted, lexer)
    }

    fn in_string(start: &str) -> Scanner {
        let mut scanner = Scanner::default();
        let (accepted, lexer) = scan(&mut scanner, start, &[STRING_START]);
        assert!(accepted);
        assert_eq!(lexer.symbol, STRING_START as u16);
        scanner
    }

    fn serialized(scanner: &mut Scanner) -> Vec<u8> {
        let mut buffer = [0; SERIALIZATION_BUFFER_SIZE];
        let length = scanner.serialize(&mut buffer);
        buffer[..length].to_vec()
    }

    #[test]
    fn string_prefixes_and_opening_quote_lookahead() {
        for (input, flags, end, position) in [
            ("'", 1, 1, 1),
            ("\"\"", 2, 1, 2),
            ("`", 4, 1, 1),
            ("'''x", 1 | 32, 3, 3),
            ("Fr\"\"\"x", 2 | 8 | 16 | 32, 5, 5),
            ("t'", 1 | 16, 2, 2),
            ("T`", 4 | 16, 2, 2),
            ("uUbBrRfFtT\"\"", 2 | 8 | 16 | 64, 11, 12),
        ] {
            let mut scanner = Scanner::default();
            let (accepted, lexer) = scan(&mut scanner, input, &[STRING_START]);
            assert!(accepted, "{input:?}");
            assert_eq!(
                (lexer.end, lexer.position),
                (Some(end), position),
                "{input:?}"
            );
            assert_eq!(scanner.delimiters, [Delimiter { flags }]);
            assert_eq!(scanner.inside_interpolated_string, flags & 16 != 0);
        }
        let mut scanner = Scanner::default();
        let (accepted, lexer) = scan(&mut scanner, "rubbish", &[STRING_START]);
        assert!(!accepted);
        assert_eq!((lexer.end, lexer.position), (Some(0), 4));
        assert!(scanner.delimiters.is_empty());
    }

    #[test]
    fn interpolation_escape_calls_and_failed_probe() {
        let mut scanner = in_string("f'");
        for input in ["{{", "}}"] {
            let (accepted, lexer) =
                scan(&mut scanner, input, &[ESCAPE_INTERPOLATION, STRING_CONTENT]);
            assert!(accepted);
            assert_eq!(
                *lexer.calls.borrow(),
                [
                    Call::MarkEnd(0),
                    Call::Advance(0, false),
                    Call::Advance(1, false),
                    Call::MarkEnd(2),
                    Call::Result(ESCAPE_INTERPOLATION as u16),
                ]
            );
        }
        for input in ["{x", "}x", "{}", "}{"] {
            let (accepted, lexer) =
                scan(&mut scanner, input, &[ESCAPE_INTERPOLATION, STRING_CONTENT]);
            assert!(!accepted);
            assert_eq!(
                *lexer.calls.borrow(),
                [Call::MarkEnd(0), Call::Advance(0, false)]
            );
        }
        let (accepted, lexer) = scan(&mut scanner, "hello{world}", &[STRING_CONTENT]);
        assert!(accepted);
        assert_eq!((lexer.symbol, lexer.end), (STRING_CONTENT as u16, Some(5)));
    }

    #[test]
    fn triple_quote_probes_mark_before_the_closing_delimiter() {
        let mut scanner = in_string("''' ");
        let (accepted, lexer) = scan(&mut scanner, "a'''", &[STRING_CONTENT]);
        assert!(accepted);
        assert_eq!(
            *lexer.calls.borrow(),
            [
                Call::Advance(0, false),
                Call::MarkEnd(1),
                Call::Advance(1, false),
                Call::Advance(2, false),
                Call::Result(STRING_CONTENT as u16),
            ]
        );
        for input in ["'x", "''x"] {
            let (accepted, lexer) = scan(&mut scanner, input, &[STRING_CONTENT]);
            assert!(accepted);
            assert_eq!(lexer.symbol, STRING_CONTENT as u16);
            assert_eq!(lexer.end, Some(input.len() - 1));
        }
        let (accepted, lexer) = scan(&mut scanner, "'''", &[STRING_CONTENT]);
        assert!(accepted);
        assert_eq!((lexer.symbol, lexer.end), (STRING_END as u16, Some(3)));
        assert!(scanner.delimiters.is_empty());
    }

    #[test]
    fn raw_and_bytes_escape_behavior() {
        let mut scanner = in_string("'");
        let (accepted, lexer) = scan(&mut scanner, "abc\\n'", &[STRING_CONTENT]);
        assert!(accepted);
        assert_eq!((lexer.position, lexer.end), (3, Some(3)));

        let mut scanner = in_string("b'");
        let (accepted, lexer) = scan(&mut scanner, "\\u1234'", &[STRING_CONTENT]);
        assert!(accepted);
        assert_eq!((lexer.symbol, lexer.end), (STRING_CONTENT as u16, Some(6)));
        let (accepted, lexer) = scan(&mut scanner, "\\n'", &[STRING_CONTENT]);
        assert!(!accepted);
        assert_eq!((lexer.position, lexer.end), (1, Some(0)));
        // C advances over the character following N/u/U even when it is a quote.
        let (accepted, lexer) = scan(&mut scanner, "\\u'", &[STRING_CONTENT]);
        assert!(!accepted);
        assert_eq!(lexer.position, 3);

        // Raw escapes do not set has_content: the next quote can end the token.
        for input in ["\\''", "\\\r\n'", "\\\n'", "\\\\'"] {
            let mut scanner = in_string("r'");
            let (accepted, lexer) = scan(&mut scanner, input, &[STRING_CONTENT]);
            assert!(accepted, "{input:?}");
            assert_eq!(lexer.symbol, STRING_END as u16);
            assert_eq!(lexer.end, Some(input.len()));
            assert!(scanner.delimiters.is_empty());
        }
        let mut scanner = in_string("rf'");
        let (accepted, lexer) = scan(&mut scanner, "\\\\{", &[STRING_CONTENT]);
        assert!(!accepted);
        assert_eq!(lexer.end, Some(2));
    }

    #[test]
    fn newlines_and_eof_in_string_content() {
        let mut scanner = in_string("'");
        let (accepted, lexer) = scan(&mut scanner, "a\n'", &[STRING_CONTENT]);
        assert!(!accepted);
        assert_eq!((lexer.position, lexer.end), (1, None));
        let (accepted, lexer) = scan(&mut scanner, "\n'", &[STRING_CONTENT]);
        assert!(accepted);
        assert_eq!((lexer.symbol, lexer.end), (STRING_CONTENT as u16, Some(1)));
        let (accepted, lexer) = scan(&mut scanner, "abc", &[STRING_CONTENT, NEWLINE]);
        assert!(accepted);
        assert_eq!((lexer.symbol, lexer.end), (NEWLINE as u16, Some(3)));

        let mut scanner = in_string("'''");
        let (accepted, lexer) = scan(&mut scanner, "a\n'''", &[STRING_CONTENT]);
        assert!(accepted);
        assert_eq!((lexer.position, lexer.end), (4, Some(2)));
    }

    #[test]
    fn indentation_is_zero_width_and_tabs_add_eight() {
        let mut scanner = Scanner::default();
        let (accepted, lexer) = scan(&mut scanner, "\n \tx", &[INDENT]);
        assert!(accepted);
        assert_eq!(scanner.indents, [0, 9]);
        assert_eq!(lexer.end, Some(0));
        assert_eq!(
            *lexer.calls.borrow(),
            [
                Call::MarkEnd(0),
                Call::Advance(0, true),
                Call::Advance(1, true),
                Call::Advance(2, true),
                Call::Eof(3),
                Call::Result(INDENT as u16),
            ]
        );
        let (accepted, lexer) = scan(&mut scanner, "\n  x", &[]);
        assert!(accepted); // DEDENT can be returned even when not valid.
        assert_eq!(lexer.symbol, DEDENT as u16);
        assert_eq!(scanner.indents, [0]);

        for input in ["\n    \x0c x", "\n    \r x"] {
            let mut scanner = Scanner::default();
            assert!(scan(&mut scanner, input, &[INDENT]).0);
            assert_eq!(scanner.indents, [0, 1]);
        }
        let input = format!("\n{}\tx", " ".repeat(65536));
        let mut scanner = Scanner::default();
        assert!(scan(&mut scanner, &input, &[INDENT]).0);
        assert_eq!(scanner.indents, [0, 8]);
    }

    #[test]
    fn comments_brackets_and_interpolation_delay_dedent() {
        let mut scanner = Scanner {
            indents: vec![0, 4],
            ..Scanner::default()
        };
        for bracket in [CLOSE_PAREN, CLOSE_BRACE, CLOSE_BRACKET] {
            assert!(!scan(&mut scanner, "\nx", &[bracket]).0);
        }
        assert!(!scan(&mut scanner, " # inline\nx", &[DEDENT]).0);
        assert!(!scan(&mut scanner, "\n    # same block\nx", &[DEDENT]).0);
        assert_eq!(scanner.indents, [0, 4]);
        assert!(scan(&mut scanner, "\n  # outer block\nx", &[DEDENT]).0);
        assert_eq!(scanner.indents, [0]);

        scanner.indents.push(4);
        let (accepted, lexer) = scan(&mut scanner, "\n'", &[STRING_START]);
        assert!(accepted);
        assert_eq!(lexer.symbol, STRING_START as u16);
        assert_eq!(scanner.indents, [0, 4]);
        scanner.inside_interpolated_string = true;
        assert!(!scan(&mut scanner, "\nx", &[DEDENT]).0);
        scanner.inside_interpolated_string = false;
        assert!(scan(&mut scanner, "\nx", &[DEDENT]).0);
    }

    #[test]
    fn line_continuation_does_not_set_found_end_of_line() {
        let mut scanner = Scanner::default();
        assert!(!scan(&mut scanner, "\\\r\n    x", &[INDENT, NEWLINE]).0);
        assert!(!scan(&mut scanner, "\\x", &[INDENT, NEWLINE]).0);
        assert_eq!(scanner.indents, [0]);
        let (accepted, lexer) = scan(&mut scanner, "\\", &[NEWLINE]);
        assert!(accepted);
        assert_eq!(
            *lexer.calls.borrow(),
            [
                Call::MarkEnd(0),
                Call::Advance(0, true),
                Call::Eof(1),
                Call::Advance(1, true),
                Call::Eof(1),
                Call::Result(NEWLINE as u16),
            ]
        );
    }

    #[test]
    fn recovery_disables_string_content_and_newline_but_not_indent() {
        let mut scanner = in_string("f'");
        let valid = [INDENT, STRING_CONTENT, ESCAPE_INTERPOLATION, NEWLINE];
        let (accepted, lexer) = scan(&mut scanner, "{{", &valid);
        assert!(!accepted);
        assert_eq!(lexer.position, 0);
        assert!(!scan(&mut scanner, "\nx", &valid).0);
        let (accepted, lexer) = scan(&mut scanner, "\n x", &valid);
        assert!(accepted);
        assert_eq!(lexer.symbol, INDENT as u16);
    }

    #[test]
    fn serialization_layout_and_empty_reset_preserve_c_state() {
        let mut scanner = Scanner::default();
        assert_eq!(serialized(&mut scanner), [0, 0]);
        scanner = in_string("fr\"\"\"");
        scanner.indents.extend([4, 0x1234]);
        let expected = [1, 1, 2 | 8 | 16 | 32, 4, 0, 0x34, 0x12];
        assert_eq!(serialized(&mut scanner), expected);
        let mut restored = Scanner::default();
        restored.deserialize(&expected);
        assert_eq!(serialized(&mut restored), expected);
        restored.deserialize(&[]);
        assert_eq!(serialized(&mut restored), [1, 0]);
        assert_eq!(restored.indents, [0]);

        // A nested non-format string clears the flag despite the outer f-string.
        assert!(scan(&mut scanner, "'", &[STRING_START]).0);
        assert!(!scanner.inside_interpolated_string);
        assert!(scan(&mut scanner, "'", &[STRING_CONTENT]).0);
        assert_eq!(scanner.delimiters.len(), 1);
        assert!(!scanner.inside_interpolated_string);
    }

    #[test]
    fn serialization_limits_delimiters_and_stops_on_whole_indents() {
        let mut scanner = Scanner::default();
        for i in 0..260 {
            scanner.delimiters.push(Delimiter { flags: i as u8 });
        }
        let bytes = serialized(&mut scanner);
        assert_eq!(&bytes[..2], &[0, 255]);
        assert_eq!(bytes[2..], (0..255).collect::<Vec<u8>>());

        scanner.indents.resize(600, 0x1234);
        let bytes = serialized(&mut scanner);
        assert_eq!(bytes.len(), 1023); // C overflows a 1024-byte buffer here.
        assert_eq!(&bytes[1021..], &[0x34, 0x12]);
        scanner.delimiters.clear();
        assert_eq!(serialized(&mut scanner).len(), 1024);
    }
}
