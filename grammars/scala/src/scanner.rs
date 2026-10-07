//! Scala's external scanner, translated from scanner.c.
//!
//! Layout widths and saved positions are the C scanner's wrapping i16 values.
//! Snapshots retain the native-endian i16 representation, including case flags.

use tree_sitter_language::{ExternalScanner, Lexer, SERIALIZATION_BUFFER_SIZE};

const AUTOMATIC_SEMICOLON: usize = 0;
const INDENT: usize = 1;
const OUTDENT: usize = 2;
const COMMA_OUTDENT: usize = 3;
const SIMPLE_STRING_START: usize = 4;
const SIMPLE_STRING_MIDDLE: usize = 5;
const SIMPLE_MULTILINE_STRING_START: usize = 6;
const INTERPOLATED_STRING_MIDDLE: usize = 7;
const INTERPOLATED_MULTILINE_STRING_MIDDLE: usize = 8;
const RAW_STRING_START: usize = 9;
const RAW_STRING_MIDDLE: usize = 10;
const RAW_STRING_MULTILINE_MIDDLE: usize = 11;
const SINGLE_LINE_STRING_END: usize = 12;
const MULTILINE_STRING_END: usize = 13;
const ELSE: usize = 14;
const CATCH: usize = 15;
const FINALLY: usize = 16;
const EXTENDS: usize = 17;
const DERIVES: usize = 18;
const WITH: usize = 19;
const BLOCK_COMMENT: usize = 20;
const SUPPRESS_BLOCK_COMMENT: usize = 21;
const ERROR_SENTINEL: usize = 22;
const COLON_EOL: usize = 23;
const POSTFIX_OP: usize = 24;
const POSTFIX_STAR: usize = 25;
const FLOATING_POINT_WITH_SEPARATORS: usize = 26;
const END_KEYWORD: usize = 27;
const CONTROL_TAIL_GATE: usize = 28;
const XML_TAG_START: usize = 29;
const ERASED_MODIFIER: usize = 30;
const OPEN_MODIFIER: usize = 31;
const OPAQUE_MODIFIER: usize = 32;
const INFIX_MODIFIER: usize = 33;
const TRACKED_MODIFIER: usize = 34;
const TRANSPARENT_MODIFIER: usize = 35;
const INLINE_MODIFIER: usize = 36;
const INTO_MODIFIER: usize = 37;
const UPDATE_MODIFIER: usize = 38;
const CONSUME_MODIFIER: usize = 39;
const USES: usize = 40;
const OP_LEFT_OR: usize = 41;
const OP_LEFT_XOR: usize = 42;
const OP_LEFT_AND: usize = 43;
const OP_LEFT_EQ: usize = 44;
const OP_LEFT_REL: usize = 45;
const OP_LEFT_COLON: usize = 46;
const OP_LEFT_ADD: usize = 47;
const OP_LEFT_MUL: usize = 48;
const OP_LEFT_OTHER: usize = 49;
const OP_NAME: usize = 50;
const USING_DIRECTIVE_START: usize = 51;

const CASE_INDENT_FLAG: i16 = 0x4000;

pub(crate) struct Scanner {
    indents: Vec<i16>,
    last_indentation_size: i16,
    last_newline_count: i16,
    last_column: i16,
    last_char: i16,
    after_colon_eol: i16,
}

impl Default for Scanner {
    fn default() -> Self {
        Self {
            indents: Vec::new(),
            last_indentation_size: -1,
            last_newline_count: 0,
            last_column: -1,
            last_char: 0,
            after_colon_eol: 0,
        }
    }
}

pub(crate) fn create() -> Box<dyn ExternalScanner> {
    Box::new(Scanner::default())
}

impl ExternalScanner for Scanner {
    fn scan(&mut self, lexer: &mut dyn Lexer, valid_symbols: &[bool]) -> bool {
        let result = self.scan_impl(lexer, valid_symbols);
        // Comments preserve the fewer-braces colon flag. Failed scans do not
        // change it (the runtime restores the preceding snapshot).
        if result {
            if lexer.result_symbol() == COLON_EOL as u16 {
                self.after_colon_eol = 1;
            } else if lexer.result_symbol() != BLOCK_COMMENT as u16 {
                self.after_colon_eol = 0;
            }
        }
        result
    }

    fn serialize(&mut self, buffer: &mut [u8]) -> usize {
        let size = (self.indents.len() + 5) * 2;
        if size > SERIALIZATION_BUFFER_SIZE || size > buffer.len() {
            return 0;
        }
        let header = [
            self.last_indentation_size,
            self.last_newline_count,
            self.last_column,
            self.last_char,
            self.after_colon_eol,
        ];
        for (bytes, value) in buffer[..size]
            .as_chunks_mut::<2>()
            .0
            .iter_mut()
            .zip(header.iter().chain(&self.indents))
        {
            bytes.copy_from_slice(&value.to_ne_bytes());
        }
        size
    }

    fn deserialize(&mut self, buffer: &[u8]) {
        self.indents.clear();
        self.last_indentation_size = -1;
        self.last_column = -1;
        self.last_char = 0;
        self.last_newline_count = 0;
        self.after_colon_eol = 0;
        // The C scanner assumes all nonempty snapshots have a full header.
        // Reject malformed short snapshots rather than reading out of bounds.
        if buffer.len() < 10 {
            return;
        }
        let mut values = buffer
            .as_chunks::<2>()
            .0
            .iter()
            .map(|b| i16::from_ne_bytes([b[0], b[1]]));
        self.last_indentation_size = values.next().unwrap();
        self.last_newline_count = values.next().unwrap();
        self.last_column = values.next().unwrap();
        self.last_char = values.next().unwrap();
        self.after_colon_eol = values.next().unwrap();
        self.indents.extend(values);
    }
}

fn is_space(c: i32) -> bool {
    c == b' ' as i32 || (b'\t' as i32..=b'\r' as i32).contains(&c)
}

fn is_alpha(c: i32) -> bool {
    (b'a' as i32..=b'z' as i32).contains(&c) || (b'A' as i32..=b'Z' as i32).contains(&c)
}

fn is_digit(c: i32) -> bool {
    (b'0' as i32..=b'9' as i32).contains(&c)
}

fn is_alnum(c: i32) -> bool {
    is_alpha(c) || is_digit(c)
}

fn advance(lexer: &mut dyn Lexer) {
    lexer.advance(false);
}

fn skip(lexer: &mut dyn Lexer) {
    lexer.advance(true);
}

fn advance_past_blanks(lexer: &mut dyn Lexer) -> bool {
    let mut found = false;
    while lexer.lookahead() == b' ' as i32 || lexer.lookahead() == b'\t' as i32 {
        advance(lexer);
        found = true;
    }
    found
}

fn indent_width(entry: i16) -> i16 {
    if entry == -1 { -1 } else { entry & 0x3fff }
}

fn at_case_region_width(prev: i16, width: i16) -> bool {
    prev != -1 && prev & CASE_INDENT_FLAG != 0 && width == indent_width(prev)
}

// '/' is deliberately excluded: it may open a comment. Unicode operators
// likewise stay with the internal lexer instead of taking layout paths.
fn is_op_char(c: i32) -> bool {
    b"!#%&*+-<=>?@\\^|~:".iter().any(|&op| c == op as i32)
}

fn op_left_class(first: i32) -> usize {
    match char::from_u32(first as u32) {
        Some('|') => OP_LEFT_OR,
        Some('^') => OP_LEFT_XOR,
        Some('&') => OP_LEFT_AND,
        Some('=' | '!') => OP_LEFT_EQ,
        Some('<' | '>') => OP_LEFT_REL,
        Some(':') => OP_LEFT_COLON,
        Some('+' | '-') => OP_LEFT_ADD,
        Some('*' | '%') => OP_LEFT_MUL,
        _ => OP_LEFT_OTHER,
    }
}

fn is_xml_name_start(c: i32) -> bool {
    is_alpha(c) || c == b'_' as i32 || c > 127
}

fn is_close_or_separator(c: i32) -> bool {
    matches!(char::from_u32(c as u32), Some(')' | ']' | '}' | ',' | ';'))
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum StringMode {
    Simple,
    Interpolated,
    Raw,
}

fn scan_string_content(lexer: &mut dyn Lexer, is_multiline: bool, mode: StringMode) -> bool {
    let mut closing_quote_count = 0u32;
    loop {
        if lexer.lookahead() == b'"' as i32 {
            advance(lexer);
            closing_quote_count = closing_quote_count.wrapping_add(1);
            if !is_multiline {
                lexer.set_result_symbol(SINGLE_LINE_STRING_END as u16);
                lexer.mark_end();
                return true;
            }
            if closing_quote_count >= 3 && lexer.lookahead() != b'"' as i32 {
                lexer.set_result_symbol(MULTILINE_STRING_END as u16);
                lexer.mark_end();
                return true;
            }
        } else if lexer.lookahead() == b'$' as i32 && mode != StringMode::Simple {
            let symbol = match (mode, is_multiline) {
                (StringMode::Interpolated, true) => INTERPOLATED_MULTILINE_STRING_MIDDLE,
                (StringMode::Interpolated, false) => INTERPOLATED_STRING_MIDDLE,
                (StringMode::Raw, true) => RAW_STRING_MULTILINE_MIDDLE,
                (StringMode::Raw, false) => RAW_STRING_MIDDLE,
                (StringMode::Simple, _) => unreachable!(),
            };
            lexer.set_result_symbol(symbol as u16);
            lexer.mark_end();
            return true;
        } else {
            closing_quote_count = 0;
            if lexer.lookahead() == b'\\' as i32 {
                if is_multiline || mode == StringMode::Raw {
                    advance(lexer);
                    // Single-line raw strings do not translate escapes, but
                    // escaped quotes/backslashes still affect termination.
                    if !is_multiline
                        && mode == StringMode::Raw
                        && (lexer.lookahead() == b'"' as i32 || lexer.lookahead() == b'\\' as i32)
                    {
                        advance(lexer);
                    }
                } else {
                    lexer.set_result_symbol(if mode == StringMode::Simple {
                        SIMPLE_STRING_MIDDLE
                    } else {
                        INTERPOLATED_STRING_MIDDLE
                    } as u16);
                    lexer.mark_end();
                    return true;
                }
            } else if (lexer.lookahead() == b'\n' as i32 && !is_multiline) || lexer.eof() {
                return false;
            } else {
                advance(lexer);
            }
        }
    }
}

// The caller has already consumed '/*'.
fn consume_block_comment_body_ex(lexer: &mut dyn Lexer, stop_at_newline: bool) -> bool {
    let mut depth = 1u32;
    while depth > 0 {
        if lexer.eof() || (stop_at_newline && lexer.lookahead() == b'\n' as i32) {
            return false;
        }
        if lexer.lookahead() == b'/' as i32 {
            advance(lexer);
            if lexer.lookahead() == b'*' as i32 {
                advance(lexer);
                depth = depth.wrapping_add(1);
            }
        } else if lexer.lookahead() == b'*' as i32 {
            advance(lexer);
            if lexer.lookahead() == b'/' as i32 {
                advance(lexer);
                depth = depth.wrapping_sub(1);
            }
        } else {
            advance(lexer);
        }
    }
    true
}

fn consume_block_comment_body(lexer: &mut dyn Lexer) {
    consume_block_comment_body_ex(lexer, false);
}

fn consume_block_comment_body_on_line(lexer: &mut dyn Lexer) -> bool {
    consume_block_comment_body_ex(lexer, true)
}

fn finish_block_comment(lexer: &mut dyn Lexer) -> bool {
    lexer.mark_end();
    lexer.set_result_symbol(BLOCK_COMMENT as u16);
    true
}

fn lex_block_comment(lexer: &mut dyn Lexer) -> bool {
    consume_block_comment_body(lexer);
    finish_block_comment(lexer)
}

fn scan_word(lexer: &mut dyn Lexer, word: &str) -> bool {
    for byte in word.bytes() {
        if lexer.lookahead() != byte as i32 {
            return false;
        }
        advance(lexer);
    }
    !(is_alnum(lexer.lookahead())
        || lexer.lookahead() == b'_' as i32
        || lexer.lookahead() == b'$' as i32)
}

// Return a borrowed ASCII word, or None if it overflows the C buffer. Still
// consume the complete identifier on failure. The final byte stays the NUL
// terminator, just as in the source, though comparisons use slices here.
fn read_word<'a>(lexer: &mut dyn Lexer, buf: &'a mut [u8]) -> Option<&'a str> {
    let mut len = 0;
    let mut not_keyword = false;
    while is_alnum(lexer.lookahead())
        || lexer.lookahead() == b'_' as i32
        || lexer.lookahead() == b'$' as i32
    {
        if lexer.lookahead() > 127 || len >= buf.len() - 1 {
            not_keyword = true;
        } else {
            buf[len] = lexer.lookahead() as u8;
            len += 1;
        }
        advance(lexer);
    }
    buf[len] = 0;
    if not_keyword {
        None
    } else {
        Some(std::str::from_utf8(&buf[..len]).unwrap())
    }
}

fn skip_blanks_and_block_comments(lexer: &mut dyn Lexer) {
    loop {
        advance_past_blanks(lexer);
        if lexer.lookahead() != b'/' as i32 {
            return;
        }
        advance(lexer);
        if lexer.lookahead() != b'*' as i32 {
            return;
        }
        advance(lexer);
        consume_block_comment_body(lexer);
    }
}

fn name_follows_word(lexer: &mut dyn Lexer) -> bool {
    skip_blanks_and_block_comments(lexer);
    is_alpha(lexer.lookahead())
        || lexer.lookahead() == b'_' as i32
        || lexer.lookahead() == b'$' as i32
        || lexer.lookahead() == b'`' as i32
        || lexer.lookahead() > 127
}

fn word_is_expression_tail(lexer: &mut dyn Lexer) -> bool {
    const TAILS: &[&str] = &[
        "match", "catch", "finally", "else", "then", "do", "yield", "while", "with", "extends",
    ];
    read_word(lexer, &mut [0; 8]).is_some_and(|word| !word.is_empty() && TAILS.contains(&word))
}

fn modifier_word_allowed(lexer: &mut dyn Lexer) -> bool {
    !word_is_expression_tail(lexer)
}

fn operand_word_allowed(lexer: &mut dyn Lexer) -> bool {
    !word_is_expression_tail(lexer)
}

fn modifier_name_follows(lexer: &mut dyn Lexer) -> bool {
    name_follows_word(lexer) && modifier_word_allowed(lexer)
}

fn inline_modifier_follows(lexer: &mut dyn Lexer) -> bool {
    if name_follows_word(lexer) {
        return modifier_word_allowed(lexer);
    }
    if is_digit(lexer.lookahead())
        || matches!(
            char::from_u32(lexer.lookahead() as u32),
            Some('"' | '\'' | '(' | '{' | '-')
        )
    {
        return true;
    }
    if lexer.lookahead() != b'\n' as i32 && lexer.lookahead() != b'\r' as i32 {
        return false;
    }
    while is_space(lexer.lookahead()) {
        advance(lexer);
    }
    // Across a newline only reserved definition words count, not soft names.
    const STARTS: &[&str] = &[
        "def",
        "val",
        "var",
        "type",
        "given",
        "class",
        "object",
        "trait",
        "enum",
        "final",
        "lazy",
        "override",
        "private",
        "protected",
        "sealed",
        "abstract",
        "implicit",
    ];
    read_word(lexer, &mut [0; 12]).is_some_and(|word| !word.is_empty() && STARTS.contains(&word))
}

fn is_case_definition_word(lexer: &mut dyn Lexer) -> bool {
    advance_past_blanks(lexer);
    matches!(read_word(lexer, &mut [0; 7]), Some("class" | "object"))
}

fn is_case_clause_intro(lexer: &mut dyn Lexer) -> bool {
    scan_word(lexer, "case") && !is_case_definition_word(lexer)
}

#[derive(Clone, Copy)]
enum CommentAtLayout {
    None,
    Lexed,
    Abort,
    SameLineCode,
}

fn check_comment_at_layout(lexer: &mut dyn Lexer, valid_symbols: &[bool]) -> CommentAtLayout {
    if lexer.lookahead() != b'/' as i32 {
        return CommentAtLayout::None;
    }
    advance(lexer);
    if lexer.lookahead() == b'*' as i32 && valid_symbols[BLOCK_COMMENT] {
        advance(lexer);
        loop {
            consume_block_comment_body(lexer);
            advance_past_blanks(lexer);
            if lexer.eof() || lexer.lookahead() == b'\n' as i32 || lexer.lookahead() == b'\r' as i32
            {
                finish_block_comment(lexer);
                return CommentAtLayout::Lexed;
            }
            if lexer.lookahead() == b'/' as i32 {
                advance(lexer);
                if lexer.lookahead() == b'*' as i32 {
                    advance(lexer);
                    continue;
                }
                if lexer.lookahead() == b'/' as i32 {
                    // Cannot re-mark backwards: include the trailing line comment.
                    while !lexer.eof() && lexer.lookahead() != b'\n' as i32 {
                        advance(lexer);
                    }
                    finish_block_comment(lexer);
                    return CommentAtLayout::Lexed;
                }
                // A lone '/' is code; preserve the off-by-one column.
                return CommentAtLayout::SameLineCode;
            }
            return CommentAtLayout::SameLineCode;
        }
    }
    if lexer.lookahead() == b'/' as i32 || lexer.lookahead() == b'*' as i32 {
        return CommentAtLayout::Abort;
    }
    CommentAtLayout::None
}

fn skip_delimited(lexer: &mut dyn Lexer, close: i32, escapes: bool) {
    while !lexer.eof() && lexer.lookahead() != close && lexer.lookahead() != b'\n' as i32 {
        if escapes && lexer.lookahead() == b'\\' as i32 {
            advance(lexer);
        }
        advance(lexer);
    }
    if lexer.lookahead() == close {
        advance(lexer);
    }
}

// After an opening quote, a character literal returns zero; a Scala quote
// returns its first character, which still contributes to bracket depth.
fn skip_char_or_quote_tail(lexer: &mut dyn Lexer) -> i32 {
    if lexer.lookahead() == b'\\' as i32 {
        advance(lexer);
        if !lexer.eof() && lexer.lookahead() != b'\n' as i32 {
            advance(lexer);
        }
        if lexer.lookahead() == b'\'' as i32 {
            advance(lexer);
        }
        return 0;
    }
    if lexer.eof() || lexer.lookahead() == b'\n' as i32 {
        return 0;
    }
    let quoted = lexer.lookahead();
    advance(lexer);
    if lexer.lookahead() == b'\'' as i32 {
        advance(lexer);
        return 0;
    }
    quoted
}

fn rest_of_line_is_blank_or_comments(lexer: &mut dyn Lexer) -> bool {
    loop {
        advance_past_blanks(lexer);
        if lexer.eof() || lexer.lookahead() == b'\n' as i32 || lexer.lookahead() == b'\r' as i32 {
            return true;
        }
        if lexer.lookahead() != b'/' as i32 {
            return false;
        }
        advance(lexer);
        if lexer.lookahead() == b'/' as i32 {
            while !lexer.eof() && lexer.lookahead() != b'\n' as i32 {
                advance(lexer);
            }
            return true;
        }
        if lexer.lookahead() != b'*' as i32 {
            return false;
        }
        advance(lexer);
        if !consume_block_comment_body_on_line(lexer) {
            return true;
        }
    }
}

// Comments and newlines are transparent when searching for a right operand.
fn has_operand(lexer: &mut dyn Lexer) -> bool {
    loop {
        if lexer.eof() {
            return false;
        }
        if is_space(lexer.lookahead()) {
            advance(lexer);
            continue;
        }
        if lexer.lookahead() != b'/' as i32 {
            return !is_close_or_separator(lexer.lookahead());
        }
        advance(lexer);
        if lexer.lookahead() == b'/' as i32 {
            while !lexer.eof() && lexer.lookahead() != b'\n' as i32 {
                advance(lexer);
            }
            continue;
        }
        if lexer.lookahead() != b'*' as i32 {
            return true;
        }
        advance(lexer);
        consume_block_comment_body(lexer);
    }
}

fn operand_follows(lexer: &mut dyn Lexer) -> bool {
    advance_past_blanks(lexer) && has_operand(lexer) && operand_word_allowed(lexer)
}

fn is_leading_infix_continuation(lexer: &mut dyn Lexer) -> bool {
    if is_op_char(lexer.lookahead()) {
        advance(lexer);
        while is_op_char(lexer.lookahead()) {
            advance(lexer);
        }
        return operand_follows(lexer);
    }
    if lexer.lookahead() == b'`' as i32 {
        advance(lexer);
        while lexer.lookahead() != b'`' as i32 && !lexer.eof() {
            advance(lexer);
        }
        if lexer.lookahead() != b'`' as i32 {
            return false;
        }
        advance(lexer);
        return operand_follows(lexer);
    }
    false
}

#[derive(Default)]
struct LineScan {
    ends_conditional: bool,
    has_case_arrow: bool,
    closes_bracket: bool,
}

fn scan_rest_of_line(lexer: &mut dyn Lexer) -> LineScan {
    let mut depth = 0i32;
    let mut result = LineScan::default();
    while !lexer.eof() && lexer.lookahead() != b'\n' as i32 && lexer.lookahead() != b'\r' as i32 {
        let c = lexer.lookahead();
        if c == b' ' as i32 || c == b'\t' as i32 {
            advance(lexer);
        } else if matches!(char::from_u32(c as u32), Some('(' | '[' | '{')) {
            depth += 1;
            result.ends_conditional = false;
            advance(lexer);
        } else if matches!(char::from_u32(c as u32), Some(')' | ']' | '}')) {
            depth -= 1;
            if depth < 0 {
                result.closes_bracket = true;
            }
            result.ends_conditional = false;
            advance(lexer);
        } else if c == b'"' as i32 || c == b'`' as i32 {
            result.ends_conditional = false;
            advance(lexer);
            skip_delimited(lexer, c, c == b'"' as i32);
        } else if c == b'\'' as i32 {
            result.ends_conditional = false;
            advance(lexer);
            let quoted = skip_char_or_quote_tail(lexer);
            if matches!(char::from_u32(quoted as u32), Some('(' | '[' | '{')) {
                depth += 1;
            } else if matches!(char::from_u32(quoted as u32), Some(')' | ']' | '}')) {
                depth -= 1;
            }
        } else if c == b'/' as i32 {
            advance(lexer);
            if lexer.lookahead() == b'/' as i32 {
                break;
            }
            if lexer.lookahead() == b'*' as i32 {
                advance(lexer);
                if !consume_block_comment_body_on_line(lexer) {
                    break;
                }
            } else {
                result.ends_conditional = false;
            }
        } else if c == 0x21d2 {
            // Scala 2's spelling of =>.
            if depth == 0 {
                result.has_case_arrow = true;
            }
            result.ends_conditional = false;
            advance(lexer);
        } else if is_op_char(c) {
            let starts_eq = c == b'=' as i32;
            advance(lexer);
            let second_gt = lexer.lookahead() == b'>' as i32;
            let mut extra = 0;
            while is_op_char(lexer.lookahead()) {
                advance(lexer);
                extra += 1;
            }
            if depth == 0 && starts_eq && second_gt && extra == 1 {
                result.has_case_arrow = true;
            }
            result.ends_conditional = false;
        } else if is_alpha(c) || c == b'_' as i32 || c == b'$' as i32 {
            let mut buffer = [0; 5];
            let word = read_word(lexer, &mut buffer);
            result.ends_conditional = depth == 0 && matches!(word, Some("then" | "do"));
        } else {
            result.ends_conditional = false;
            advance(lexer);
        }
    }
    result
}

fn consume_digit_group(lexer: &mut dyn Lexer, mut saw_sep: Option<&mut bool>) -> bool {
    if !is_digit(lexer.lookahead()) {
        return false;
    }
    advance(lexer);
    loop {
        if is_digit(lexer.lookahead()) {
            advance(lexer);
        } else if lexer.lookahead() == b'_' as i32 {
            advance(lexer);
            if !is_digit(lexer.lookahead()) {
                return false;
            }
            if let Some(saw_sep) = saw_sep.as_mut() {
                **saw_sep = true;
            }
            advance(lexer);
        } else {
            return true;
        }
    }
}

fn scan_float_with_separator(lexer: &mut dyn Lexer) -> bool {
    let mut int_sep = false;
    if !consume_digit_group(lexer, Some(&mut int_sep)) {
        return false;
    }
    // Only an integer-part separator is this scanner's responsibility.
    if !int_sep {
        return false;
    }
    let mut is_float = false;
    if lexer.lookahead() == b'.' as i32 {
        advance(lexer);
        if !consume_digit_group(lexer, None) {
            return false;
        }
        is_float = true;
    }
    if lexer.lookahead() == b'e' as i32 || lexer.lookahead() == b'E' as i32 {
        advance(lexer);
        if lexer.lookahead() == b'+' as i32 || lexer.lookahead() == b'-' as i32 {
            advance(lexer);
        }
        if !consume_digit_group(lexer, None) {
            return false;
        }
        is_float = true;
    }
    if matches!(
        char::from_u32(lexer.lookahead() as u32),
        Some('d' | 'D' | 'f' | 'F')
    ) {
        advance(lexer);
        is_float = true;
    }
    if !is_float {
        return false;
    }
    lexer.mark_end();
    lexer.set_result_symbol(FLOATING_POINT_WITH_SEPARATORS as u16);
    true
}

impl Scanner {
    fn scan_impl(&mut self, lexer: &mut dyn Lexer, valid_symbols: &[bool]) -> bool {
        // This token is immediate after //, so inspect it before skipping blanks.
        if valid_symbols[USING_DIRECTIVE_START]
            && !valid_symbols[ERROR_SENTINEL]
            && lexer.lookahead() == b'>' as i32
        {
            advance(lexer);
            lexer.mark_end();
            advance_past_blanks(lexer);
            if !scan_word(lexer, "using") {
                return false;
            }
            lexer.set_result_symbol(USING_DIRECTIVE_START as u16);
            return true;
        }

        let prev = self.indents.last().copied().unwrap_or(-1);
        let prev_width = indent_width(prev);
        let mut newline_count = 0i16;
        let mut indentation_size = 0i16;
        while is_space(lexer.lookahead()) {
            if lexer.lookahead() == b'\n' as i32 {
                newline_count = newline_count.wrapping_add(1);
                indentation_size = 0;
            } else {
                indentation_size = indentation_size.wrapping_add(1);
            }
            skip(lexer);
        }

        if valid_symbols[COMMA_OUTDENT] && lexer.lookahead() == b',' as i32 && prev != -1 {
            lexer.mark_end();
            // Error recovery keeps the eager pop without scanning the line.
            if !valid_symbols[ERROR_SENTINEL] {
                advance(lexer);
                let mut ends_line = false;
                loop {
                    advance_past_blanks(lexer);
                    if lexer.eof()
                        || lexer.lookahead() == b'\n' as i32
                        || lexer.lookahead() == b'\r' as i32
                    {
                        ends_line = true;
                        break;
                    }
                    if lexer.lookahead() != b'/' as i32 {
                        break;
                    }
                    advance(lexer);
                    if lexer.lookahead() == b'/' as i32 {
                        ends_line = true;
                        break;
                    }
                    if lexer.lookahead() != b'*' as i32 {
                        break;
                    }
                    advance(lexer);
                    consume_block_comment_body(lexer);
                }
                if !ends_line && !scan_rest_of_line(lexer).closes_bracket {
                    return false;
                }
            }
            self.indents.pop();
            lexer.set_result_symbol(COMMA_OUTDENT as u16);
            return true;
        }

        // Cascade a same-width case-region close only at the saved column.
        if valid_symbols[OUTDENT]
            && !valid_symbols[ERROR_SENTINEL]
            && self.last_indentation_size != -1
            && at_case_region_width(prev, self.last_indentation_size)
            && (if lexer.eof() {
                self.last_column == -1
            } else {
                lexer.get_column() as i16 == self.last_column
            })
        {
            lexer.mark_end();
            if is_case_clause_intro(lexer) {
                return false;
            }
            self.indents.pop();
            lexer.set_result_symbol(OUTDENT as u16);
            return true;
        }

        // Double-outdent before advancing the lexer any further.
        if valid_symbols[OUTDENT]
            && (lexer.lookahead() == 0
                || (prev != -1
                    && matches!(
                        char::from_u32(lexer.lookahead() as u32),
                        Some(')' | ']' | '}')
                    ))
                || (self.last_indentation_size != -1
                    && prev != -1
                    && self.last_indentation_size < prev_width))
        {
            self.indents.pop();
            lexer.set_result_symbol(OUTDENT as u16);
            return true;
        }
        self.last_indentation_size = -1;

        let indent_geometry = indentation_size > prev_width
            || (self.after_colon_eol != 0 && indentation_size == prev_width);
        if valid_symbols[INDENT]
            && newline_count > 0
            && lexer.lookahead() != b'}' as i32
            && lexer.lookahead() != b')' as i32
            && lexer.lookahead() != b']' as i32
            && (indent_geometry
                || (indentation_size == prev_width && lexer.lookahead() == b'/' as i32))
        {
            lexer.mark_end();
            match check_comment_at_layout(lexer, valid_symbols) {
                CommentAtLayout::Lexed => return true,
                CommentAtLayout::Abort => return false,
                CommentAtLayout::SameLineCode => {
                    let effective = lexer.get_column() as i16;
                    if effective > prev_width {
                        indentation_size = effective;
                    } else {
                        return finish_block_comment(lexer);
                    }
                }
                CommentAtLayout::None => {
                    if !indent_geometry {
                        return false;
                    }
                }
            }
            let mut entry = indentation_size;
            match char::from_u32(lexer.lookahead() as u32) {
                Some('e' | 'c' | 'f' | 'y' | 'd') => {
                    let mut buffer = [0; 8];
                    let word = read_word(lexer, &mut buffer);
                    if word
                        .is_some_and(|w| ["else", "catch", "finally", "yield", "do"].contains(&w))
                    {
                        // mark_end above makes this a zero-width gate. 'do' is
                        // deliberately ungated: it can also start a statement.
                        if valid_symbols[CONTROL_TAIL_GATE]
                            && word
                                .is_some_and(|w| ["else", "catch", "finally", "yield"].contains(&w))
                        {
                            lexer.set_result_symbol(CONTROL_TAIL_GATE as u16);
                            return true;
                        }
                        return false;
                    }
                    // An enum case without an arrow must not open a case region.
                    if self.indents.is_empty()
                        && word == Some("case")
                        && !is_case_definition_word(lexer)
                        && scan_rest_of_line(lexer).has_case_arrow
                    {
                        entry |= CASE_INDENT_FLAG;
                    }
                }
                Some('|' | '&')
                    if self.after_colon_eol == 0 && is_leading_infix_continuation(lexer) =>
                {
                    return false;
                }
                _ => {}
            }
            self.indents.push(entry);
            lexer.set_result_symbol(INDENT as u16);
            return true;
        }

        let case_region_close = newline_count > 0 && at_case_region_width(prev, indentation_size);
        if valid_symbols[OUTDENT]
            && (lexer.lookahead() == 0
                || (newline_count > 0 && prev != -1 && indentation_size < prev_width)
                || case_region_close)
        {
            lexer.mark_end();
            match check_comment_at_layout(lexer, valid_symbols) {
                CommentAtLayout::Lexed => return true,
                CommentAtLayout::Abort => return false,
                CommentAtLayout::SameLineCode => {
                    let effective = lexer.get_column() as i16;
                    if effective < prev_width {
                        indentation_size = effective;
                    } else {
                        // Carry the pending newline past the comment for the
                        // semicolon's own suppression rules on the next scan.
                        self.last_newline_count = newline_count;
                        self.last_column = effective;
                        self.last_char = (lexer.lookahead() & 0x7fff) as i16;
                        return finish_block_comment(lexer);
                    }
                }
                CommentAtLayout::None => {}
            }
            self.last_indentation_size = indentation_size;
            self.last_newline_count = newline_count;
            if lexer.eof() {
                self.last_column = -1;
                self.last_char = 0;
            } else {
                self.last_column = lexer.get_column() as i16;
                self.last_char = (lexer.lookahead() & 0x7fff) as i16;
            }
            if lexer.lookahead() != 0
                && is_leading_infix_continuation(lexer)
                && !scan_rest_of_line(lexer).ends_conditional
            {
                return false;
            }
            if case_region_close && is_case_clause_intro(lexer) {
                return false;
            }
            self.indents.pop();
            lexer.set_result_symbol(OUTDENT as u16);
            return true;
        }

        // Recover the saved newline only at its character and column, unless
        // this scan has already crossed a new newline of its own.
        if self.last_newline_count > 0 {
            let is_eof = lexer.eof();
            if (is_eof && self.last_column == -1)
                || (!is_eof
                    && newline_count == 0
                    && (lexer.lookahead() & 0x7fff) as i16 == self.last_char
                    && lexer.get_column() == self.last_column as u32)
            {
                newline_count = newline_count.wrapping_add(self.last_newline_count);
            }
        }
        self.last_newline_count = 0;

        if valid_symbols[END_KEYWORD]
            && !valid_symbols[ERROR_SENTINEL]
            && lexer.lookahead() == b'e' as i32
        {
            if scan_word(lexer, "end") && name_follows_word(lexer) {
                lexer.mark_end();
                lexer.set_result_symbol(END_KEYWORD as u16);
                return true;
            }
            return false;
        }

        if valid_symbols[AUTOMATIC_SEMICOLON] && newline_count > 0 {
            lexer.mark_end();
            lexer.set_result_symbol(AUTOMATIC_SEMICOLON as u16);
            if lexer.lookahead() == b'.' as i32 {
                return false;
            }
            if matches!(
                char::from_u32(lexer.lookahead() as u32),
                Some(')' | ']' | ',')
            ) {
                return false;
            }
            if lexer.lookahead() == b'}' as i32 {
                advance(lexer);
                loop {
                    advance_past_blanks(lexer);
                    if matches!(
                        char::from_u32(lexer.lookahead() as u32),
                        Some('}' | ')' | ']')
                    ) {
                        advance(lexer);
                        continue;
                    }
                    if lexer.lookahead() == b'/' as i32 {
                        advance(lexer);
                        if lexer.lookahead() == b'/' as i32 {
                            return false;
                        }
                        if lexer.lookahead() == b'*' as i32 {
                            advance(lexer);
                            consume_block_comment_body(lexer);
                            continue;
                        }
                    }
                    break;
                }
                if lexer.lookahead() == b'\n' as i32
                    || lexer.lookahead() == b'\r' as i32
                    || lexer.eof()
                {
                    return false;
                }
                return true;
            }
            if lexer.lookahead() == b'/' as i32 {
                advance(lexer);
                if lexer.lookahead() == b'/' as i32 {
                    return false;
                }
                if lexer.lookahead() == b'*' as i32 && valid_symbols[BLOCK_COMMENT] {
                    advance(lexer);
                    consume_block_comment_body(lexer);
                    advance_past_blanks(lexer);
                    if !(lexer.lookahead() == b'\n' as i32
                        || lexer.lookahead() == b'\r' as i32
                        || lexer.eof())
                    {
                        self.last_newline_count = newline_count;
                        self.last_column = lexer.get_column() as i16;
                        self.last_char = (lexer.lookahead() & 0x7fff) as i16;
                    }
                    return finish_block_comment(lexer);
                }
                // A lone slash falls through with the lexer still advanced.
            }
            if is_op_char(lexer.lookahead()) || lexer.lookahead() == b'`' as i32 {
                if newline_count == 1 && is_leading_infix_continuation(lexer) {
                    return false;
                }
                return true;
            }
            match char::from_u32(lexer.lookahead() as u32) {
                Some('e') => {
                    if valid_symbols[ELSE]
                        || valid_symbols[EXTENDS]
                        || valid_symbols[CONTROL_TAIL_GATE]
                    {
                        advance(lexer);
                        if (valid_symbols[ELSE] || valid_symbols[CONTROL_TAIL_GATE])
                            && scan_word(lexer, "lse")
                        {
                            if valid_symbols[CONTROL_TAIL_GATE] {
                                lexer.set_result_symbol(CONTROL_TAIL_GATE as u16);
                                return true;
                            }
                            return false;
                        }
                        if valid_symbols[EXTENDS] && scan_word(lexer, "xtends") {
                            return false;
                        }
                    }
                }
                Some('c') => {
                    let mut buffer = [0; 6];
                    if let Some(word) = read_word(lexer, &mut buffer).filter(|w| !w.is_empty()) {
                        if (valid_symbols[CATCH] || valid_symbols[CONTROL_TAIL_GATE])
                            && word == "catch"
                        {
                            if valid_symbols[CONTROL_TAIL_GATE] {
                                lexer.set_result_symbol(CONTROL_TAIL_GATE as u16);
                                return true;
                            }
                            return false;
                        }
                        if word == "case" && !is_case_definition_word(lexer) {
                            let line = scan_rest_of_line(lexer);
                            if line.has_case_arrow && !line.closes_bracket {
                                return false;
                            }
                        }
                    }
                }
                Some('f') => {
                    if (valid_symbols[FINALLY] || valid_symbols[CONTROL_TAIL_GATE])
                        && scan_word(lexer, "finally")
                    {
                        if valid_symbols[CONTROL_TAIL_GATE] {
                            lexer.set_result_symbol(CONTROL_TAIL_GATE as u16);
                            return true;
                        }
                        return false;
                    }
                }
                Some('w') => {
                    if valid_symbols[WITH] && scan_word(lexer, "with") {
                        return false;
                    }
                }
                Some('d') => {
                    if valid_symbols[DERIVES] && scan_word(lexer, "derives") {
                        return false;
                    }
                }
                Some('u') => {
                    if valid_symbols[USES] && scan_word(lexer, "uses") {
                        return false;
                    }
                }
                Some('m') if scan_word(lexer, "match") => {
                    return false;
                }
                _ => {}
            }
            return true;
        }

        // Mid-line closing keywords and soft modifiers share one word read.
        let outdent_arm = valid_symbols[OUTDENT]
            && !valid_symbols[CONTROL_TAIL_GATE]
            && newline_count == 0
            && prev != -1
            && ((lexer.lookahead() == b'e' as i32 && !valid_symbols[ELSE])
                || (lexer.lookahead() == b'c' as i32 && !valid_symbols[CATCH])
                || (lexer.lookahead() == b'f' as i32 && !valid_symbols[FINALLY]));
        const SOFT_MODIFIERS: &[(u8, &str, usize)] = &[
            (b'e', "erased", ERASED_MODIFIER),
            (b'o', "open", OPEN_MODIFIER),
            (b'o', "opaque", OPAQUE_MODIFIER),
            (b'i', "infix", INFIX_MODIFIER),
            (b't', "tracked", TRACKED_MODIFIER),
            (b't', "transparent", TRANSPARENT_MODIFIER),
            (b'i', "inline", INLINE_MODIFIER),
            (b'i', "into", INTO_MODIFIER),
            (b'u', "update", UPDATE_MODIFIER),
            (b'c', "consume", CONSUME_MODIFIER),
        ];
        let modifier_arm = SOFT_MODIFIERS
            .iter()
            .any(|&(first, _, symbol)| lexer.lookahead() == first as i32 && valid_symbols[symbol]);
        if !valid_symbols[ERROR_SENTINEL] && (outdent_arm || modifier_arm) {
            if outdent_arm {
                lexer.mark_end();
            }
            let mut buffer = [0; 12];
            if let Some(word) = read_word(lexer, &mut buffer).filter(|w| !w.is_empty()) {
                let modifier = SOFT_MODIFIERS
                    .iter()
                    .find(|&&(_, name, symbol)| valid_symbols[symbol] && word == name)
                    .map(|&(_, _, symbol)| symbol);
                if let Some(modifier) = modifier {
                    lexer.mark_end();
                    let follows = if modifier == INLINE_MODIFIER {
                        inline_modifier_follows(lexer)
                    } else {
                        modifier_name_follows(lexer)
                    };
                    if follows {
                        lexer.set_result_symbol(modifier as u16);
                        return true;
                    }
                    return false;
                }
                if outdent_arm && ["else", "catch", "finally"].contains(&word) {
                    self.indents.pop();
                    lexer.set_result_symbol(OUTDENT as u16);
                    return true;
                }
            }
            return false;
        }

        while is_space(lexer.lookahead()) {
            if lexer.lookahead() == b'\n' as i32 {
                newline_count = newline_count.wrapping_add(1);
            }
            skip(lexer);
        }

        if valid_symbols[XML_TAG_START]
            && !valid_symbols[ERROR_SENTINEL]
            && lexer.lookahead() == b'<' as i32
        {
            advance(lexer);
            if is_xml_name_start(lexer.lookahead()) {
                lexer.mark_end();
                lexer.set_result_symbol(XML_TAG_START as u16);
                return true;
            }
            return false;
        }
        if valid_symbols[FLOATING_POINT_WITH_SEPARATORS]
            && !valid_symbols[ERROR_SENTINEL]
            && is_digit(lexer.lookahead())
        {
            return scan_float_with_separator(lexer);
        }

        // The internal lexer handles infix operators except ones ending in '/'.
        // This branch handles those, postfix operators, and fewer-braces colons.
        if !valid_symbols[ERROR_SENTINEL]
            && is_op_char(lexer.lookahead())
            && ((valid_symbols[COLON_EOL] && lexer.lookahead() == b':' as i32)
                || valid_symbols[POSTFIX_OP]
                || valid_symbols[POSTFIX_STAR]
                || valid_symbols[op_left_class(lexer.lookahead())]
                || valid_symbols[OP_NAME])
        {
            let mut op = [0u8; 4];
            let mut op_len = 0;
            loop {
                if op_len < 3 {
                    op[op_len] = lexer.lookahead() as u8;
                }
                op_len += 1;
                advance(lexer);
                if !is_op_char(lexer.lookahead()) {
                    break;
                }
            }
            lexer.mark_end();
            if op_len == 1 && op[0] == b':' && lexer.lookahead() != b'/' as i32 {
                if valid_symbols[COLON_EOL] && rest_of_line_is_blank_or_comments(lexer) {
                    lexer.set_result_symbol(COLON_EOL as u16);
                    return true;
                }
                return false;
            }
            let shebang = op_len >= 2 && op[0] == b'#' && op[1] == b'!';
            let op_class = op_left_class(op[0] as i32);
            if lexer.lookahead() == b'/' as i32
                && !shebang
                && (valid_symbols[op_class] || valid_symbols[OP_NAME])
            {
                let mut ends_in_slash = false;
                let mut at_comment = false;
                while !at_comment {
                    let c = lexer.lookahead();
                    if c == b'/' as i32 {
                        advance(lexer);
                        at_comment =
                            lexer.lookahead() == b'/' as i32 || lexer.lookahead() == b'*' as i32;
                    } else if is_op_char(c) {
                        advance(lexer);
                    } else {
                        break;
                    }
                    ends_in_slash = !at_comment && c == b'/' as i32;
                }
                if ends_in_slash && lexer.lookahead() < 0x80 {
                    lexer.mark_end();
                    lexer.set_result_symbol(if valid_symbols[op_class] {
                        op_class
                    } else {
                        OP_NAME
                    } as u16);
                    return true;
                }
                return false;
            }
            let postfix_sym = if op_len == 1 && op[0] == b'*' {
                POSTFIX_STAR
            } else {
                POSTFIX_OP
            };
            if !valid_symbols[postfix_sym] {
                return false;
            }
            if matches!(op[0], b'=' | b'<' | b'>' | b'#' | b'@' | b'?') {
                const RESERVED: &[&[u8]] = &[
                    b"=", b"#", b"@", b"=>", b"<-", b"<:", b">:", b"<%", b"?=>", b"=>>",
                ];
                if op_len <= 3 && RESERVED.contains(&&op[..op_len]) {
                    return false;
                }
            }
            if !rest_of_line_is_blank_or_comments(lexer) {
                if is_close_or_separator(lexer.lookahead()) {
                    lexer.set_result_symbol(postfix_sym as u16);
                    return true;
                }
                return false;
            }
            if !has_operand(lexer) {
                lexer.set_result_symbol(postfix_sym as u16);
                return true;
            }
            if lexer.lookahead() == b'.' as i32 || lexer.lookahead() == b'=' as i32 {
                return false;
            }
            if lexer.lookahead() == b'@' as i32 {
                lexer.set_result_symbol(postfix_sym as u16);
                return true;
            }
            if (b'a' as i32..=b'z' as i32).contains(&lexer.lookahead()) {
                // Only hard definition keywords, not the soft modifier names.
                const DEFINITIONS: &[&str] = &[
                    "abstract",
                    "class",
                    "def",
                    "enum",
                    "export",
                    "final",
                    "given",
                    "import",
                    "implicit",
                    "lazy",
                    "object",
                    "override",
                    "package",
                    "private",
                    "protected",
                    "sealed",
                    "trait",
                    "type",
                    "val",
                    "var",
                ];
                if read_word(lexer, &mut [0; 10])
                    .is_some_and(|w| !w.is_empty() && DEFINITIONS.contains(&w))
                {
                    lexer.set_result_symbol(postfix_sym as u16);
                    return true;
                }
            }
            return false;
        }

        if valid_symbols[BLOCK_COMMENT]
            && lexer.lookahead() == b'/' as i32
            && (valid_symbols[ERROR_SENTINEL]
                || !(valid_symbols[SUPPRESS_BLOCK_COMMENT]
                    || valid_symbols[SIMPLE_STRING_MIDDLE]
                    || valid_symbols[INTERPOLATED_STRING_MIDDLE]
                    || valid_symbols[RAW_STRING_MIDDLE]
                    || valid_symbols[RAW_STRING_MULTILINE_MIDDLE]
                    || valid_symbols[INTERPOLATED_MULTILINE_STRING_MIDDLE]
                    || valid_symbols[MULTILINE_STRING_END]))
        {
            advance(lexer);
            if lexer.lookahead() == b'*' as i32 {
                advance(lexer);
                return lex_block_comment(lexer);
            }
            return false;
        }

        if valid_symbols[SIMPLE_STRING_START] && lexer.lookahead() == b'"' as i32 {
            advance(lexer);
            lexer.mark_end();
            if lexer.lookahead() == b'"' as i32 {
                advance(lexer);
                if lexer.lookahead() == b'"' as i32 {
                    advance(lexer);
                    lexer.set_result_symbol(SIMPLE_MULTILINE_STRING_START as u16);
                    lexer.mark_end();
                    return true;
                }
            }
            lexer.set_result_symbol(SIMPLE_STRING_START as u16);
            return true;
        }
        if valid_symbols[RAW_STRING_START] && lexer.lookahead() == b'r' as i32 {
            advance(lexer);
            if lexer.lookahead() == b'a' as i32 {
                advance(lexer);
                if lexer.lookahead() == b'w' as i32 {
                    advance(lexer);
                    if lexer.lookahead() == b'"' as i32 {
                        lexer.mark_end();
                        lexer.set_result_symbol(RAW_STRING_START as u16);
                        return true;
                    }
                }
            }
        }
        if valid_symbols[SIMPLE_STRING_MIDDLE] {
            return scan_string_content(lexer, false, StringMode::Simple);
        }
        if valid_symbols[INTERPOLATED_STRING_MIDDLE] {
            return scan_string_content(lexer, false, StringMode::Interpolated);
        }
        if valid_symbols[RAW_STRING_MIDDLE] {
            return scan_string_content(lexer, false, StringMode::Raw);
        }
        if valid_symbols[RAW_STRING_MULTILINE_MIDDLE] {
            return scan_string_content(lexer, true, StringMode::Raw);
        }
        if valid_symbols[INTERPOLATED_MULTILINE_STRING_MIDDLE] {
            return scan_string_content(lexer, true, StringMode::Interpolated);
        }
        if valid_symbols[MULTILINE_STRING_END] {
            return scan_string_content(lexer, true, StringMode::Simple);
        }

        // Same-width Scala 3 cases open a flagged region. Keep the partial
        // scan_word failure position for the catch gate's 'tch' suffix.
        if valid_symbols[INDENT]
            && !valid_symbols[ERROR_SENTINEL]
            && newline_count > 0
            && lexer.lookahead() == b'c' as i32
            && prev != -1
            && indentation_size == prev_width
        {
            lexer.mark_end();
            if is_case_clause_intro(lexer) {
                self.indents.push(indentation_size | CASE_INDENT_FLAG);
                lexer.set_result_symbol(INDENT as u16);
                return true;
            }
            if valid_symbols[CONTROL_TAIL_GATE] && scan_word(lexer, "tch") {
                lexer.set_result_symbol(CONTROL_TAIL_GATE as u16);
                return true;
            }
            return false;
        }
        if valid_symbols[CONTROL_TAIL_GATE]
            && !valid_symbols[ERROR_SENTINEL]
            && matches!(
                char::from_u32(lexer.lookahead() as u32),
                Some('c' | 'e' | 'f' | 't' | 'y')
            )
        {
            lexer.mark_end();
            if read_word(lexer, &mut [0; 8])
                .is_some_and(|w| ["catch", "else", "finally", "then", "yield"].contains(&w))
            {
                lexer.set_result_symbol(CONTROL_TAIL_GATE as u16);
                return true;
            }
            return false;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Eq)]
    enum Event {
        Advance(bool),
        Mark(usize),
        Symbol(u16),
        Column,
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
            self.events.push(Event::Advance(skip));
            if self.position < self.input.len() {
                self.position += 1;
            }
        }
        fn mark_end(&mut self) {
            self.end = self.position;
            self.events.push(Event::Mark(self.position));
        }
        fn get_column(&mut self) -> u32 {
            self.events.push(Event::Column);
            self.input[..self.position]
                .iter()
                .rev()
                .take_while(|&&c| c != b'\n' as i32)
                .count() as u32
        }
        fn is_at_included_range_start(&self) -> bool {
            false
        }
        fn eof(&self) -> bool {
            self.position == self.input.len()
        }
    }

    fn valid(symbols: &[usize]) -> [bool; USING_DIRECTIVE_START + 1] {
        let mut valid = [false; USING_DIRECTIVE_START + 1];
        for &symbol in symbols {
            valid[symbol] = true;
        }
        valid
    }

    fn token(
        scanner: &mut Scanner,
        input: &str,
        symbols: &[usize],
        expected: usize,
        end: usize,
    ) -> TestLexer {
        let mut lexer = TestLexer::new(input);
        assert!(
            scanner.scan(&mut lexer, &valid(symbols)),
            "input: {input:?}"
        );
        assert_eq!(lexer.symbol, expected as u16, "input: {input:?}");
        assert_eq!(lexer.end, end, "input: {input:?}");
        lexer
    }

    #[test]
    fn snapshots_are_native_endian_i16_and_keep_case_flags() {
        let mut scanner = Scanner {
            indents: vec![0, 4 | CASE_INDENT_FLAG, -1],
            last_indentation_size: 3,
            last_newline_count: 2,
            last_column: 6,
            last_char: 0x7fff,
            after_colon_eol: 1,
        };
        let values: [i16; 8] = [3, 2, 6, 0x7fff, 1, 0, 4 | CASE_INDENT_FLAG, -1];
        let expected: Vec<_> = values.into_iter().flat_map(i16::to_ne_bytes).collect();
        let mut buffer = [0; SERIALIZATION_BUFFER_SIZE];
        let size = scanner.serialize(&mut buffer);
        assert_eq!(&buffer[..size], expected);
        let mut restored = Scanner::default();
        restored.deserialize(&buffer[..size]);
        assert_eq!(restored.indents, scanner.indents);
        assert_eq!(restored.serialize(&mut buffer), size);
        assert_eq!(&buffer[..size], expected);
        restored.deserialize(&[]);
        assert!(restored.indents.is_empty());
        assert_eq!(restored.serialize(&mut buffer), 10);
        let empty: Vec<_> = [-1i16, 0, -1, 0, 0]
            .into_iter()
            .flat_map(i16::to_ne_bytes)
            .collect();
        assert_eq!(&buffer[..10], empty);
    }

    #[test]
    fn snapshot_limit_includes_all_five_header_fields() {
        let mut scanner = Scanner::default();
        scanner.indents.resize(507, 1);
        let mut buffer = [0; SERIALIZATION_BUFFER_SIZE];
        assert_eq!(scanner.serialize(&mut buffer), SERIALIZATION_BUFFER_SIZE);
        scanner.indents.push(2);
        buffer.fill(0x7f);
        assert_eq!(scanner.serialize(&mut buffer), 0);
        assert!(buffer.iter().all(|&b| b == 0x7f));
    }

    #[test]
    fn string_openers_mark_before_lookahead_and_preserve_call_order() {
        let mut scanner = Scanner::default();
        let lexer = token(
            &mut scanner,
            "\"\"x",
            &[SIMPLE_STRING_START],
            SIMPLE_STRING_START,
            1,
        );
        assert_eq!(lexer.position, 2);
        assert_eq!(
            lexer.events,
            [
                Event::Advance(false),
                Event::Mark(1),
                Event::Advance(false),
                Event::Symbol(SIMPLE_STRING_START as u16),
            ]
        );
        let lexer = token(
            &mut scanner,
            "\"\"\"x",
            &[SIMPLE_STRING_START],
            SIMPLE_MULTILINE_STRING_START,
            3,
        );
        assert_eq!(
            lexer.events,
            [
                Event::Advance(false),
                Event::Mark(1),
                Event::Advance(false),
                Event::Advance(false),
                Event::Symbol(SIMPLE_MULTILINE_STRING_START as u16),
                Event::Mark(3),
            ]
        );
        token(
            &mut scanner,
            "raw\"",
            &[RAW_STRING_START],
            RAW_STRING_START,
            3,
        );
    }

    #[test]
    fn string_modes_handle_escapes_interpolation_and_quote_runs() {
        for (input, multiline, mode, expected, end) in [
            (
                "a\\\"b\"",
                false,
                StringMode::Raw,
                SINGLE_LINE_STRING_END,
                5,
            ),
            (
                "a\\\"b\"",
                false,
                StringMode::Simple,
                SIMPLE_STRING_MIDDLE,
                1,
            ),
            (
                "abc$tail",
                false,
                StringMode::Interpolated,
                INTERPOLATED_STRING_MIDDLE,
                3,
            ),
            (
                "a\"\"\"\"\"z",
                true,
                StringMode::Simple,
                MULTILINE_STRING_END,
                6,
            ),
            (
                "a\"\"$tail",
                true,
                StringMode::Raw,
                RAW_STRING_MULTILINE_MIDDLE,
                3,
            ),
        ] {
            let mut lexer = TestLexer::new(input);
            assert!(scan_string_content(&mut lexer, multiline, mode));
            assert_eq!(lexer.symbol, expected as u16);
            assert_eq!(lexer.end, end);
        }
        assert!(!scan_string_content(
            &mut TestLexer::new("abc\n"),
            false,
            StringMode::Simple
        ));
        assert!(!scan_string_content(
            &mut TestLexer::new("abc"),
            true,
            StringMode::Raw
        ));
    }

    #[test]
    fn same_width_case_regions_and_partial_catch_probe() {
        let mut scanner = Scanner::default();
        token(&mut scanner, "\ncase x => x", &[INDENT], INDENT, 1);
        assert_eq!(scanner.indents, [CASE_INDENT_FLAG]);
        token(&mut scanner, "\nval next = 1", &[OUTDENT], OUTDENT, 1);
        assert!(scanner.indents.is_empty());

        scanner.indents.push(2);
        token(&mut scanner, "\n  case x => x", &[INDENT], INDENT, 3);
        assert_eq!(scanner.indents, [2, 2 | CASE_INDENT_FLAG]);
        let mut lexer = TestLexer::new("\n  case y => y");
        assert!(!scanner.scan(&mut lexer, &valid(&[OUTDENT])));
        assert_eq!(scanner.indents, [2, 2 | CASE_INDENT_FLAG]);

        let mut scanner = Scanner::default();
        scanner.indents.push(2);
        let lexer = token(
            &mut scanner,
            "\n  catch",
            &[INDENT, CONTROL_TAIL_GATE],
            CONTROL_TAIL_GATE,
            3,
        );
        assert_eq!(lexer.position, 8);
        assert_eq!(scanner.indents, [2]);
    }

    #[test]
    fn comment_keeps_pending_newline_and_colon_flag() {
        let mut scanner = Scanner::default();
        token(&mut scanner, ": // c\n", &[COLON_EOL], COLON_EOL, 1);
        assert_eq!(scanner.after_colon_eol, 1);
        token(
            &mut scanner,
            "/* a /* b */ c */",
            &[BLOCK_COMMENT],
            BLOCK_COMMENT,
            17,
        );
        assert_eq!(scanner.after_colon_eol, 1);
        let mut lexer = token(
            &mut scanner,
            "\n/* c */ else 2",
            &[AUTOMATIC_SEMICOLON, BLOCK_COMMENT, CONTROL_TAIL_GATE],
            BLOCK_COMMENT,
            9,
        );
        assert_eq!(scanner.last_newline_count, 1);
        assert_eq!(scanner.last_column, 8);
        assert_eq!(scanner.last_char, b'e' as i16);
        lexer.position = lexer.end;
        assert!(scanner.scan(
            &mut lexer,
            &valid(&[AUTOMATIC_SEMICOLON, CONTROL_TAIL_GATE])
        ));
        assert_eq!(lexer.symbol, CONTROL_TAIL_GATE as u16);
        assert_eq!(lexer.end, 9);
        assert_eq!(scanner.last_newline_count, 0);
        assert_eq!(scanner.after_colon_eol, 0);
    }

    #[test]
    fn directives_modifiers_and_end_markers_have_different_boundaries() {
        let mut scanner = Scanner::default();
        let lexer = token(
            &mut scanner,
            "> using dep",
            &[USING_DIRECTIVE_START],
            USING_DIRECTIVE_START,
            1,
        );
        assert_eq!(lexer.position, 7);
        assert!(!scanner.scan(
            &mut TestLexer::new(" > using dep"),
            &valid(&[USING_DIRECTIVE_START])
        ));
        token(
            &mut scanner,
            "open /* c */ class C",
            &[OPEN_MODIFIER],
            OPEN_MODIFIER,
            4,
        );
        token(
            &mut scanner,
            "inline\n  val n = 1",
            &[INLINE_MODIFIER],
            INLINE_MODIFIER,
            6,
        );
        assert!(!scanner.scan(
            &mut TestLexer::new("inline\nopen"),
            &valid(&[INLINE_MODIFIER])
        ));
        // C's end keyword mark is after skipped blanks/comments, unlike a modifier.
        token(
            &mut scanner,
            "end /* c */ name",
            &[END_KEYWORD],
            END_KEYWORD,
            12,
        );
    }

    #[test]
    fn numeric_separators_and_operator_slash_boundaries() {
        let mut scanner = Scanner::default();
        token(
            &mut scanner,
            "1_000.5e-1F x",
            &[FLOATING_POINT_WITH_SEPARATORS],
            FLOATING_POINT_WITH_SEPARATORS,
            11,
        );
        for input in ["1000.5", "1_000", "1_000.", "1__0.2"] {
            assert!(!scanner.scan(
                &mut TestLexer::new(input),
                &valid(&[FLOATING_POINT_WITH_SEPARATORS])
            ));
        }
        token(&mut scanner, "+/ x", &[OP_LEFT_ADD], OP_LEFT_ADD, 2);
        token(&mut scanner, "+/ x", &[OP_NAME], OP_NAME, 2);
        for input in ["+/* x */", "+// x", "+/⇒"] {
            assert!(!scanner.scan(&mut TestLexer::new(input), &valid(&[OP_LEFT_ADD])));
        }
        token(&mut scanner, "*)", &[POSTFIX_STAR], POSTFIX_STAR, 1);
        token(&mut scanner, "++ // c", &[POSTFIX_OP], POSTFIX_OP, 2);
        assert!(!scanner.scan(&mut TestLexer::new("=>\nval x"), &valid(&[POSTFIX_OP])));
    }

    #[test]
    fn line_scan_skips_literals_and_comments_but_counts_scala_quotes() {
        let mut lexer = TestLexer::new("(a => b) /* => */ then");
        let line = scan_rest_of_line(&mut lexer);
        assert!(line.ends_conditional);
        assert!(!line.has_case_arrow);
        assert!(!line.closes_bracket);
        let line = scan_rest_of_line(&mut TestLexer::new("'{' => \"}\" '`' ⇒ }"));
        assert!(line.has_case_arrow);
        assert!(line.closes_bracket);
        let line = scan_rest_of_line(&mut TestLexer::new("'{ => } then"));
        assert!(!line.has_case_arrow);
        assert!(!line.closes_bracket);
        assert!(line.ends_conditional);
    }
}
