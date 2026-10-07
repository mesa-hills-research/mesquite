//! Ruby's external scanner, translated from `src/scanner.c`.

use ts_port_tables::{ExternalScanner, Lexer, SERIALIZATION_BUFFER_SIZE};

// Indices in the grammar's external-token array, in C TokenType order.
const LINE_BREAK: usize = 0;
const NO_LINE_BREAK: usize = 1;
const SIMPLE_SYMBOL: usize = 2;
const STRING_START: usize = 3;
const SYMBOL_START: usize = 4;
const SUBSHELL_START: usize = 5;
const REGEX_START: usize = 6;
const STRING_ARRAY_START: usize = 7;
const SYMBOL_ARRAY_START: usize = 8;
const HEREDOC_BODY_START: usize = 9;
const STRING_CONTENT: usize = 10;
const HEREDOC_CONTENT: usize = 11;
const STRING_END: usize = 12;
const HEREDOC_BODY_END: usize = 13;
const HEREDOC_START: usize = 14;
const FORWARD_SLASH: usize = 15;
const BLOCK_AMPERSAND: usize = 16;
const SPLAT_STAR: usize = 17;
const UNARY_MINUS: usize = 18;
const UNARY_MINUS_NUM: usize = 19;
const BINARY_MINUS: usize = 20;
const BINARY_STAR: usize = 21;
const SINGLETON_CLASS_LEFT_ANGLE_LEFT_ANGLE: usize = 22;
const HASH_KEY_SYMBOL: usize = 23;
const IDENTIFIER_SUFFIX: usize = 24;
const CONSTANT_SUFFIX: usize = 25;
const HASH_SPLAT_STAR_STAR: usize = 26;
const BINARY_STAR_STAR: usize = 27;
const ELEMENT_REFERENCE_BRACKET: usize = 28;
const SHORT_INTERPOLATION: usize = 29;
const NONE: usize = 30;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct Literal {
    kind: usize,
    open_delimiter: i32,
    close_delimiter: i32,
    nesting_depth: i32,
    allows_interpolation: bool,
}

#[derive(Debug, Default, PartialEq, Eq)]
struct Heredoc {
    word: Vec<u8>,
    end_word_indentation_allowed: bool,
    allows_interpolation: bool,
    started: bool,
}

/// Owned vectors replace C's arrays and their reset/destroy routines.
#[derive(Debug, Default)]
pub(crate) struct Scanner {
    has_leading_whitespace: bool,
    literal_stack: Vec<Literal>,
    open_heredocs: Vec<Heredoc>,
}

// The reference uses wctype in the default C locale, not Unicode categories.
fn is_space(c: i32) -> bool {
    matches!(c, 0x09..=0x0d | 0x20)
}

fn is_alpha(c: i32) -> bool {
    matches!(c, 0x41..=0x5a | 0x61..=0x7a)
}

fn is_upper(c: i32) -> bool {
    matches!(c, 0x41..=0x5a)
}

fn is_lower(c: i32) -> bool {
    matches!(c, 0x61..=0x7a)
}

fn is_digit(c: i32) -> bool {
    matches!(c, 0x30..=0x39)
}

fn is_alnum(c: i32) -> bool {
    is_alpha(c) || is_digit(c)
}

fn advance(lexer: &mut dyn Lexer) {
    lexer.advance(false);
}

fn emit(lexer: &mut dyn Lexer, symbol: usize) -> bool {
    lexer.set_result_symbol(symbol as u16);
    true
}

// Unlike wctype, C's is_iden_char first narrows the code point to char.
fn is_iden_char(c: u8) -> bool {
    const IDENTIFIER_BYTES: [bool; 256] = {
        let mut table = [true; 256];
        let excluded = b"\0\n\r\t :;`\"'@$#.,|^&<=>+-*/\\%?!~()[]{}";
        let mut i = 0;
        while i < excluded.len() {
            table[excluded[i] as usize] = false;
            i += 1;
        }
        table
    };
    IDENTIFIER_BYTES[usize::from(c)]
}

fn scan_operator(lexer: &mut dyn Lexer) -> bool {
    // The switches only recognize ASCII, so match the integer lookahead
    // directly instead of validating it as a Unicode scalar on every call.
    match lexer.lookahead() {
        // <, <=, <<, <=>
        0x3c => {
            // '<'
            advance(lexer);
            if lexer.lookahead() == i32::from(b'<') {
                advance(lexer);
            } else if lexer.lookahead() == i32::from(b'=') {
                advance(lexer);
                if lexer.lookahead() == i32::from(b'>') {
                    advance(lexer);
                }
            }
            true
        }
        0x3e => {
            // '>'
            advance(lexer);
            if lexer.lookahead() == i32::from(b'>') || lexer.lookahead() == i32::from(b'=') {
                advance(lexer);
            }
            true
        }
        0x3d => {
            // '='
            advance(lexer);
            if lexer.lookahead() == i32::from(b'~') {
                advance(lexer);
                return true;
            }
            if lexer.lookahead() == i32::from(b'=') {
                advance(lexer);
                if lexer.lookahead() == i32::from(b'=') {
                    advance(lexer);
                }
                return true;
            }
            false
        }
        0x2b | 0x2d | 0x7e => {
            // '+', '-', '~'
            advance(lexer);
            if lexer.lookahead() == i32::from(b'@') {
                advance(lexer);
            }
            true
        }
        0x2e => {
            // '.'
            advance(lexer);
            if lexer.lookahead() == i32::from(b'.') {
                advance(lexer);
                return true;
            }
            false
        }
        0x26 | 0x5e | 0x7c | 0x2f | 0x25 | 0x60 => {
            // '&', '^', '|', '/', '%', '`'
            advance(lexer);
            true
        }
        0x21 => {
            // '!'
            advance(lexer);
            if lexer.lookahead() == i32::from(b'=') || lexer.lookahead() == i32::from(b'~') {
                advance(lexer);
            }
            true
        }
        0x2a => {
            // '*'
            advance(lexer);
            if lexer.lookahead() == i32::from(b'*') {
                advance(lexer);
            }
            true
        }
        0x5b => {
            // '['
            advance(lexer);
            if lexer.lookahead() == i32::from(b']') {
                advance(lexer);
            } else {
                return false;
            }
            if lexer.lookahead() == i32::from(b'=') {
                advance(lexer);
            }
            true
        }
        _ => false,
    }
}

fn scan_symbol_identifier(lexer: &mut dyn Lexer) -> bool {
    if lexer.lookahead() == i32::from(b'@') {
        advance(lexer);
        if lexer.lookahead() == i32::from(b'@') {
            advance(lexer);
        }
    } else if lexer.lookahead() == i32::from(b'$') {
        advance(lexer);
    }

    if is_iden_char(lexer.lookahead() as u8) {
        advance(lexer);
    } else if !scan_operator(lexer) {
        return false;
    }
    while is_iden_char(lexer.lookahead() as u8) {
        advance(lexer);
    }
    if lexer.lookahead() == i32::from(b'?') || lexer.lookahead() == i32::from(b'!') {
        advance(lexer);
    }
    if lexer.lookahead() == i32::from(b'=') {
        lexer.mark_end();
        advance(lexer);
        if lexer.lookahead() != i32::from(b'>') {
            lexer.mark_end();
        }
    }
    true
}

fn scan_heredoc_word(lexer: &mut dyn Lexer, heredoc: &mut Heredoc) {
    let mut word = Vec::new();
    let mut quote = 0;
    match lexer.lookahead() {
        0x27 | 0x22 | 0x60 => {
            // "'", '"', '`'
            quote = lexer.lookahead();
            advance(lexer);
            while lexer.lookahead() != quote && !lexer.eof() {
                word.push(lexer.lookahead() as u8);
                advance(lexer);
            }
            advance(lexer);
        }
        _ => {
            if is_alnum(lexer.lookahead()) || lexer.lookahead() == i32::from(b'_') {
                word.push(lexer.lookahead() as u8);
                advance(lexer);
                while is_alnum(lexer.lookahead()) || lexer.lookahead() == i32::from(b'_') {
                    word.push(lexer.lookahead() as u8);
                    advance(lexer);
                }
            }
        }
    }
    heredoc.word = word;
    heredoc.allows_interpolation = quote != i32::from(b'\'');
}

fn scan_short_interpolation(
    lexer: &mut dyn Lexer,
    has_content: bool,
    content_symbol: usize,
) -> bool {
    let start = lexer.lookahead() as u8;
    if start == b'@' || start == b'$' {
        if has_content {
            return emit(lexer, content_symbol);
        }
        lexer.mark_end();
        advance(lexer);
        let mut is_short_interpolation = false;
        if start == b'$' {
            // strchr converts to char and also matches the terminating NUL.
            if b"!@&`'+~=/\\,;.<>*$?:\"\0".contains(&(lexer.lookahead() as u8)) {
                is_short_interpolation = true;
            } else if lexer.lookahead() == i32::from(b'-') {
                advance(lexer);
                is_short_interpolation =
                    is_alpha(lexer.lookahead()) || lexer.lookahead() == i32::from(b'_');
            } else {
                is_short_interpolation =
                    is_alnum(lexer.lookahead()) || lexer.lookahead() == i32::from(b'_');
            }
        }
        if start == b'@' {
            if lexer.lookahead() == i32::from(b'@') {
                advance(lexer);
            }
            is_short_interpolation =
                is_iden_char(lexer.lookahead() as u8) && !is_digit(lexer.lookahead());
        }
        if is_short_interpolation {
            return emit(lexer, SHORT_INTERPOLATION);
        }
    }
    false
}

impl Scanner {
    fn skip(&mut self, lexer: &mut dyn Lexer) {
        self.has_leading_whitespace = true;
        lexer.advance(true);
    }

    fn reset(&mut self) {
        self.literal_stack.clear();
        self.open_heredocs.clear();
    }

    fn scan_whitespace(&mut self, lexer: &mut dyn Lexer, valid_symbols: &[bool; NONE]) -> bool {
        let heredoc_body_start_is_valid = !self.open_heredocs.is_empty()
            && !self.open_heredocs[0].started
            && valid_symbols[HEREDOC_BODY_START];
        let line_break_is_valid = !valid_symbols[NO_LINE_BREAK] && valid_symbols[LINE_BREAK];
        let mut crossed_newline = false;
        loop {
            if line_break_is_valid && lexer.is_at_included_range_start() {
                lexer.mark_end();
                return emit(lexer, LINE_BREAK);
            }
            let lookahead = lexer.lookahead();
            match lookahead {
                0x20 | 0x09 => self.skip(lexer), // ' ', '\t'
                0x0d => {
                    // '\r'
                    if heredoc_body_start_is_valid {
                        lexer.set_result_symbol(HEREDOC_BODY_START as u16);
                        self.open_heredocs[0].started = true;
                        return true;
                    }
                    self.skip(lexer);
                }
                0x0a => {
                    // '\n'
                    if heredoc_body_start_is_valid {
                        lexer.set_result_symbol(HEREDOC_BODY_START as u16);
                        self.open_heredocs[0].started = true;
                        return true;
                    } else if line_break_is_valid && !crossed_newline {
                        lexer.mark_end();
                        advance(lexer);
                        crossed_newline = true;
                    } else {
                        self.skip(lexer);
                    }
                }
                0x5c => {
                    // '\\'
                    advance(lexer);
                    if lexer.lookahead() == i32::from(b'\r') {
                        self.skip(lexer);
                    }
                    if is_space(lexer.lookahead()) {
                        self.skip(lexer);
                    } else {
                        return false;
                    }
                }
                _ => {
                    if crossed_newline {
                        if lookahead != i32::from(b'.')
                            && lookahead != i32::from(b'&')
                            && lookahead != i32::from(b'#')
                        {
                            lexer.set_result_symbol(LINE_BREAK as u16);
                        } else if lookahead == i32::from(b'.') {
                            // A call operator suppresses the break; a range does not.
                            advance(lexer);
                            if !lexer.eof() && lexer.lookahead() == i32::from(b'.') {
                                lexer.set_result_symbol(LINE_BREAK as u16);
                            } else {
                                return false;
                            }
                        }
                    }
                    return true;
                }
            }
        }
    }

    fn scan_open_delimiter(
        &self,
        lexer: &mut dyn Lexer,
        literal: &mut Literal,
        valid_symbols: &[bool; NONE],
    ) -> bool {
        match lexer.lookahead() {
            0x22 | 0x27 => {
                // '"', "'"
                literal.kind = STRING_START;
                literal.open_delimiter = lexer.lookahead();
                literal.close_delimiter = lexer.lookahead();
                literal.allows_interpolation = lexer.lookahead() == i32::from(b'"');
                advance(lexer);
                true
            }
            0x60 => {
                // '`'
                if !valid_symbols[SUBSHELL_START] {
                    return false;
                }
                literal.kind = SUBSHELL_START;
                literal.open_delimiter = lexer.lookahead();
                literal.close_delimiter = lexer.lookahead();
                literal.allows_interpolation = true;
                advance(lexer);
                true
            }
            0x2f => {
                // '/'
                if !valid_symbols[REGEX_START] {
                    return false;
                }
                literal.kind = REGEX_START;
                literal.open_delimiter = lexer.lookahead();
                literal.close_delimiter = lexer.lookahead();
                literal.allows_interpolation = true;
                advance(lexer);
                if valid_symbols[FORWARD_SLASH] {
                    if !self.has_leading_whitespace {
                        return false;
                    }
                    if matches!(lexer.lookahead(), 0x20 | 0x09 | 0x0a | 0x0d) {
                        return false;
                    }
                    if lexer.lookahead() == i32::from(b'=') {
                        return false;
                    }
                }
                true
            }
            0x25 => {
                // '%'
                advance(lexer);
                let (required, kind, interpolation, has_modifier) = match lexer.lookahead() {
                    0x73 => (SIMPLE_SYMBOL, SYMBOL_START, false, true), // 's'
                    0x72 => (REGEX_START, REGEX_START, true, true),     // 'r'
                    0x78 => (SUBSHELL_START, SUBSHELL_START, true, true), // 'x'
                    0x71 => (STRING_START, STRING_START, false, true),  // 'q'
                    0x51 => (STRING_START, STRING_START, true, true),   // 'Q'
                    0x77 => (STRING_ARRAY_START, STRING_ARRAY_START, false, true), // 'w'
                    0x69 => (SYMBOL_ARRAY_START, SYMBOL_ARRAY_START, false, true), // 'i'
                    0x57 => (STRING_ARRAY_START, STRING_ARRAY_START, true, true), // 'W'
                    0x49 => (SYMBOL_ARRAY_START, SYMBOL_ARRAY_START, true, true), // 'I'
                    _ => (STRING_START, STRING_START, true, false),
                };
                if !valid_symbols[required] {
                    return false;
                }
                literal.kind = kind;
                literal.allows_interpolation = interpolation;
                if has_modifier {
                    advance(lexer);
                }
                match lexer.lookahead() {
                    0x28 => {
                        // '('
                        literal.open_delimiter = i32::from(b'(');
                        literal.close_delimiter = i32::from(b')');
                    }
                    0x5b => {
                        // '['
                        literal.open_delimiter = i32::from(b'[');
                        literal.close_delimiter = i32::from(b']');
                    }
                    0x7b => {
                        // '{'
                        literal.open_delimiter = i32::from(b'{');
                        literal.close_delimiter = i32::from(b'}');
                    }
                    0x3c => {
                        // '<'
                        literal.open_delimiter = i32::from(b'<');
                        literal.close_delimiter = i32::from(b'>');
                    }
                    0x0d | 0x0a | 0x20 | 0x09 => {
                        // '\r', '\n', ' ', '\t'
                        // Preserve C's zero delimiters in this branch. Where `/`
                        // is valid, `%` plus whitespace must be an operator.
                        if valid_symbols[FORWARD_SLASH] {
                            return false;
                        }
                    }
                    // Other ASCII punctuation allowed as a percent delimiter.
                    0x7c | 0x21 | 0x23 | 0x2f | 0x5c | 0x40 | 0x24 | 0x25 | 0x5e | 0x26 | 0x2a
                    | 0x29 | 0x5d | 0x7d | 0x3e | 0x2b | 0x2d | 0x7e | 0x60 | 0x2c | 0x2e
                    | 0x3f | 0x3a | 0x3b | 0x5f | 0x22 | 0x27 => {
                        // As in C, '=' is not a percent-literal delimiter.
                        literal.open_delimiter = lexer.lookahead();
                        literal.close_delimiter = lexer.lookahead();
                    }
                    _ => return false,
                }
                advance(lexer);
                true
            }
            _ => false,
        }
    }

    fn scan_heredoc_content(&mut self, lexer: &mut dyn Lexer) -> bool {
        let heredoc = &self.open_heredocs[0];
        let mut position_in_word = 0;
        let mut look_for_heredoc_end = true;
        let mut has_content = false;
        loop {
            if position_in_word == heredoc.word.len() {
                if !has_content {
                    lexer.mark_end();
                }
                while lexer.lookahead() == i32::from(b' ') || lexer.lookahead() == i32::from(b'\t')
                {
                    advance(lexer);
                }
                if lexer.lookahead() == i32::from(b'\n') || lexer.lookahead() == i32::from(b'\r') {
                    if has_content {
                        lexer.set_result_symbol(HEREDOC_CONTENT as u16);
                    } else {
                        self.open_heredocs.remove(0);
                        lexer.set_result_symbol(HEREDOC_BODY_END as u16);
                    }
                    return true;
                }
                has_content = true;
                position_in_word = 0;
            }
            if lexer.eof() {
                lexer.mark_end();
                if has_content {
                    lexer.set_result_symbol(HEREDOC_CONTENT as u16);
                } else {
                    self.open_heredocs.remove(0);
                    lexer.set_result_symbol(HEREDOC_BODY_END as u16);
                }
                return true;
            }
            // C stores each code point in a signed char, then promotes it back
            // to int for this comparison (it does not store UTF-8 here).
            if lexer.lookahead() == i32::from(heredoc.word[position_in_word] as i8)
                && look_for_heredoc_end
            {
                advance(lexer);
                position_in_word += 1;
            } else {
                position_in_word = 0;
                look_for_heredoc_end = false;
                if heredoc.allows_interpolation && lexer.lookahead() == i32::from(b'\\') {
                    if has_content {
                        return emit(lexer, HEREDOC_CONTENT);
                    }
                    return false;
                }
                if heredoc.allows_interpolation && lexer.lookahead() == i32::from(b'#') {
                    lexer.mark_end();
                    advance(lexer);
                    if lexer.lookahead() == i32::from(b'{') {
                        if has_content {
                            return emit(lexer, HEREDOC_CONTENT);
                        }
                        return false;
                    }
                    if scan_short_interpolation(lexer, has_content, HEREDOC_CONTENT) {
                        return true;
                    }
                } else if lexer.lookahead() == i32::from(b'\r')
                    || lexer.lookahead() == i32::from(b'\n')
                {
                    if lexer.lookahead() == i32::from(b'\r') {
                        advance(lexer);
                        if lexer.lookahead() == i32::from(b'\n') {
                            advance(lexer);
                        }
                    } else {
                        advance(lexer);
                    }
                    has_content = true;
                    look_for_heredoc_end = true;
                    while lexer.lookahead() == i32::from(b' ')
                        || lexer.lookahead() == i32::from(b'\t')
                    {
                        advance(lexer);
                        if !heredoc.end_word_indentation_allowed {
                            look_for_heredoc_end = false;
                        }
                    }
                    lexer.mark_end();
                } else {
                    has_content = true;
                    advance(lexer);
                    lexer.mark_end();
                }
            }
        }
    }

    fn scan_literal_content(&mut self, lexer: &mut dyn Lexer) -> bool {
        let literal = self.literal_stack.last_mut().unwrap();
        let mut has_content = false;
        let stop_on_space =
            literal.kind == SYMBOL_ARRAY_START || literal.kind == STRING_ARRAY_START;
        loop {
            let lookahead = lexer.lookahead();
            if stop_on_space && is_space(lookahead) {
                if has_content {
                    lexer.mark_end();
                    return emit(lexer, STRING_CONTENT);
                }
                return false;
            }
            if lookahead == literal.close_delimiter {
                lexer.mark_end();
                if literal.nesting_depth == 1 {
                    if has_content {
                        lexer.set_result_symbol(STRING_CONTENT as u16);
                    } else {
                        advance(lexer);
                        if literal.kind == REGEX_START {
                            while is_lower(lexer.lookahead()) {
                                advance(lexer);
                            }
                        }
                        self.literal_stack.pop();
                        lexer.set_result_symbol(STRING_END as u16);
                        lexer.mark_end();
                    }
                    return true;
                }
                literal.nesting_depth -= 1;
                advance(lexer);
            } else if lookahead == literal.open_delimiter {
                literal.nesting_depth += 1;
                advance(lexer);
            } else if literal.allows_interpolation && lookahead == i32::from(b'#') {
                lexer.mark_end();
                advance(lexer);
                if lexer.lookahead() == i32::from(b'{') {
                    if has_content {
                        return emit(lexer, STRING_CONTENT);
                    }
                    return false;
                }
                if scan_short_interpolation(lexer, has_content, STRING_CONTENT) {
                    return true;
                }
            } else if lookahead == i32::from(b'\\') {
                if literal.allows_interpolation {
                    if has_content {
                        lexer.mark_end();
                        return emit(lexer, STRING_CONTENT);
                    }
                    return false;
                }
                advance(lexer);
                advance(lexer);
            } else if lexer.eof() {
                advance(lexer);
                lexer.mark_end();
                return false;
            } else {
                advance(lexer);
            }
            has_content = true;
        }
    }
}

impl ExternalScanner for Scanner {
    fn scan(&mut self, lexer: &mut dyn Lexer, valid_symbols: &[bool]) -> bool {
        // The grammar has NONE external tokens. Check that once instead of
        // bounds-checking each individual token lookup in the scanning paths.
        let valid_symbols: &[bool; NONE] = valid_symbols[..NONE].try_into().unwrap();
        self.has_leading_whitespace = false;
        if !valid_symbols[STRING_START] {
            if (valid_symbols[STRING_CONTENT] || valid_symbols[STRING_END])
                && !self.literal_stack.is_empty()
            {
                return self.scan_literal_content(lexer);
            }
            if (valid_symbols[HEREDOC_CONTENT] || valid_symbols[HEREDOC_BODY_END])
                && !self.open_heredocs.is_empty()
            {
                return self.scan_heredoc_content(lexer);
            }
        }

        lexer.set_result_symbol(NONE as u16);
        if !self.scan_whitespace(lexer, valid_symbols) {
            return false;
        }
        if lexer.result_symbol() != NONE as u16 {
            return true;
        }

        let lookahead = lexer.lookahead();
        match lookahead {
            0x26 => {
                // '&'
                if valid_symbols[BLOCK_AMPERSAND] {
                    advance(lexer);
                    if lexer.lookahead() != i32::from(b'&')
                        && lexer.lookahead() != i32::from(b'.')
                        && lexer.lookahead() != i32::from(b'=')
                        && !is_space(lexer.lookahead())
                    {
                        return emit(lexer, BLOCK_AMPERSAND);
                    }
                    return false;
                }
            }
            0x3c => {
                // '<'
                if valid_symbols[SINGLETON_CLASS_LEFT_ANGLE_LEFT_ANGLE] {
                    advance(lexer);
                    if lexer.lookahead() == i32::from(b'<') {
                        advance(lexer);
                        return emit(lexer, SINGLETON_CLASS_LEFT_ANGLE_LEFT_ANGLE);
                    }
                    return false;
                }
            }
            0x2a => {
                // '*'
                if valid_symbols[SPLAT_STAR]
                    || valid_symbols[BINARY_STAR]
                    || valid_symbols[HASH_SPLAT_STAR_STAR]
                    || valid_symbols[BINARY_STAR_STAR]
                {
                    advance(lexer);
                    if lexer.lookahead() == i32::from(b'=') {
                        return false;
                    }
                    if lexer.lookahead() == i32::from(b'*') {
                        if valid_symbols[HASH_SPLAT_STAR_STAR] || valid_symbols[BINARY_STAR_STAR] {
                            advance(lexer);
                            if lexer.lookahead() == i32::from(b'=') {
                                return false;
                            }
                            if valid_symbols[BINARY_STAR_STAR] && !self.has_leading_whitespace {
                                return emit(lexer, BINARY_STAR_STAR);
                            }
                            if valid_symbols[HASH_SPLAT_STAR_STAR] && !is_space(lexer.lookahead()) {
                                return emit(lexer, HASH_SPLAT_STAR_STAR);
                            }
                            if valid_symbols[BINARY_STAR_STAR] {
                                return emit(lexer, BINARY_STAR_STAR);
                            }
                            if valid_symbols[HASH_SPLAT_STAR_STAR] {
                                return emit(lexer, HASH_SPLAT_STAR_STAR);
                            }
                            return false;
                        }
                        return false;
                    }
                    if valid_symbols[BINARY_STAR] && !self.has_leading_whitespace {
                        return emit(lexer, BINARY_STAR);
                    }
                    if valid_symbols[SPLAT_STAR] && !is_space(lexer.lookahead()) {
                        return emit(lexer, SPLAT_STAR);
                    }
                    if valid_symbols[BINARY_STAR] {
                        return emit(lexer, BINARY_STAR);
                    }
                    if valid_symbols[SPLAT_STAR] {
                        return emit(lexer, SPLAT_STAR);
                    }
                    return false;
                }
            }
            0x2d => {
                // '-'
                if valid_symbols[UNARY_MINUS]
                    || valid_symbols[UNARY_MINUS_NUM]
                    || valid_symbols[BINARY_MINUS]
                {
                    advance(lexer);
                    if lexer.lookahead() != i32::from(b'=') && lexer.lookahead() != i32::from(b'>')
                    {
                        if valid_symbols[UNARY_MINUS_NUM]
                            && (!valid_symbols[BINARY_STAR] || self.has_leading_whitespace)
                            && is_digit(lexer.lookahead())
                        {
                            return emit(lexer, UNARY_MINUS_NUM);
                        }
                        if valid_symbols[UNARY_MINUS]
                            && self.has_leading_whitespace
                            && !is_space(lexer.lookahead())
                        {
                            lexer.set_result_symbol(UNARY_MINUS as u16);
                        } else if valid_symbols[BINARY_MINUS] {
                            lexer.set_result_symbol(BINARY_MINUS as u16);
                        } else {
                            lexer.set_result_symbol(UNARY_MINUS as u16);
                        }
                        return true;
                    }
                    return false;
                }
            }
            0x3a => {
                // ':'
                if valid_symbols[SYMBOL_START] {
                    let mut literal = Literal {
                        kind: SYMBOL_START,
                        nesting_depth: 1,
                        ..Literal::default()
                    };
                    advance(lexer);
                    match lexer.lookahead() {
                        0x22 => {
                            // '"'
                            advance(lexer);
                            literal.open_delimiter = i32::from(b'"');
                            literal.close_delimiter = i32::from(b'"');
                            literal.allows_interpolation = true;
                            self.literal_stack.push(literal);
                            return emit(lexer, SYMBOL_START);
                        }
                        0x27 => {
                            // "'"
                            advance(lexer);
                            literal.open_delimiter = i32::from(b'\'');
                            literal.close_delimiter = i32::from(b'\'');
                            literal.allows_interpolation = false;
                            self.literal_stack.push(literal);
                            return emit(lexer, SYMBOL_START);
                        }
                        _ => {
                            if scan_symbol_identifier(lexer) {
                                return emit(lexer, SIMPLE_SYMBOL);
                            }
                        }
                    }
                    return false;
                }
            }
            0x5b if valid_symbols[ELEMENT_REFERENCE_BRACKET]
                && (!self.has_leading_whitespace || !valid_symbols[STRING_START]) =>
            {
                advance(lexer);
                return emit(lexer, ELEMENT_REFERENCE_BRACKET);
            }
            _ => {}
        }

        if ((valid_symbols[HASH_KEY_SYMBOL] || valid_symbols[IDENTIFIER_SUFFIX])
            && (is_alpha(lookahead) || lookahead == i32::from(b'_')))
            || (valid_symbols[CONSTANT_SUFFIX] && is_upper(lookahead))
        {
            let valid_identifier_symbol = if is_upper(lookahead) {
                CONSTANT_SUFFIX
            } else {
                IDENTIFIER_SUFFIX
            };
            let mut lookahead = lookahead;
            while is_alnum(lookahead) || lookahead == i32::from(b'_') {
                advance(lexer);
                lookahead = lexer.lookahead();
            }
            if valid_symbols[HASH_KEY_SYMBOL] && lexer.lookahead() == i32::from(b':') {
                lexer.mark_end();
                advance(lexer);
                if lexer.lookahead() != i32::from(b':') {
                    return emit(lexer, HASH_KEY_SYMBOL);
                }
            } else if valid_symbols[valid_identifier_symbol] && lexer.lookahead() == i32::from(b'!')
            {
                advance(lexer);
                if lexer.lookahead() != i32::from(b'=') {
                    return emit(lexer, valid_identifier_symbol);
                }
            }
            return false;
        }

        if valid_symbols[STRING_START] {
            let mut literal = Literal {
                nesting_depth: 1,
                ..Literal::default()
            };
            if lexer.lookahead() == i32::from(b'<') {
                advance(lexer);
                if lexer.lookahead() != i32::from(b'<') {
                    return false;
                }
                advance(lexer);
                let mut heredoc = Heredoc::default();
                if lexer.lookahead() == i32::from(b'-') || lexer.lookahead() == i32::from(b'~') {
                    advance(lexer);
                    heredoc.end_word_indentation_allowed = true;
                }
                scan_heredoc_word(lexer, &mut heredoc);
                if heredoc.word.is_empty() {
                    return false;
                }
                self.open_heredocs.push(heredoc);
                return emit(lexer, HEREDOC_START);
            }
            if self.scan_open_delimiter(lexer, &mut literal, valid_symbols) {
                self.literal_stack.push(literal);
                return emit(lexer, literal.kind);
            }
            return false;
        }
        false
    }

    fn serialize(&mut self, buffer: &mut [u8]) -> usize {
        let mut size = 0;
        if self.literal_stack.len() * 5 + 2 >= SERIALIZATION_BUFFER_SIZE {
            return 0;
        }
        // The trait supplies 1024 bytes; handle shorter caller buffers safely too.
        if self.literal_stack.len() * 5 + 2 > buffer.len() {
            return 0;
        }
        buffer[size] = self.literal_stack.len() as u8;
        size += 1;
        for literal in &self.literal_stack {
            buffer[size..size + 5].copy_from_slice(&[
                literal.kind as u8,
                literal.open_delimiter as u8,
                literal.close_delimiter as u8,
                literal.nesting_depth as u8,
                u8::from(literal.allows_interpolation),
            ]);
            size += 5;
        }
        buffer[size] = self.open_heredocs.len() as u8;
        size += 1;
        for heredoc in &self.open_heredocs {
            if size + 2 + heredoc.word.len() >= SERIALIZATION_BUFFER_SIZE {
                return 0;
            }
            // C's preceding check is two bytes short of the actual header size.
            // Do not reproduce its out-of-bounds write on oversized states.
            if size + 4 + heredoc.word.len() > buffer.len().min(SERIALIZATION_BUFFER_SIZE) {
                return 0;
            }
            buffer[size..size + 4].copy_from_slice(&[
                u8::from(heredoc.end_word_indentation_allowed),
                u8::from(heredoc.allows_interpolation),
                u8::from(heredoc.started),
                heredoc.word.len() as u8,
            ]);
            size += 4;
            buffer[size..size + heredoc.word.len()].copy_from_slice(&heredoc.word);
            size += heredoc.word.len();
        }
        size
    }

    fn deserialize(&mut self, buffer: &[u8]) {
        self.has_leading_whitespace = false;
        // Keep the literal allocation and each still-open heredoc's word buffer:
        // the parser restores a snapshot before every external scan.
        self.literal_stack.clear();
        if buffer.is_empty() {
            self.open_heredocs.clear();
            return;
        }
        let mut size = 0;
        let literal_depth = buffer[size];
        size += 1;
        for _ in 0..literal_depth {
            let Some(bytes) = buffer.get(size..size + 5) else {
                self.reset();
                return;
            };
            self.literal_stack.push(Literal {
                kind: usize::from(bytes[0]),
                open_delimiter: i32::from(bytes[1]),
                close_delimiter: i32::from(bytes[2]),
                nesting_depth: i32::from(bytes[3]),
                allows_interpolation: bytes[4] != 0,
            });
            size += 5;
        }
        let Some(&open_heredoc_count) = buffer.get(size) else {
            self.reset();
            return;
        };
        size += 1;
        self.open_heredocs
            .resize_with(usize::from(open_heredoc_count), Heredoc::default);
        for index in 0..usize::from(open_heredoc_count) {
            let Some(bytes) = buffer.get(size..size + 4) else {
                self.reset();
                return;
            };
            size += 4;
            let word_length = usize::from(bytes[3]);
            let Some(word) = buffer.get(size..size + word_length) else {
                self.reset();
                return;
            };
            let heredoc = &mut self.open_heredocs[index];
            heredoc.end_word_indentation_allowed = bytes[0] != 0;
            heredoc.allows_interpolation = bytes[1] != 0;
            heredoc.started = bytes[2] != 0;
            heredoc.word.clear();
            heredoc.word.extend_from_slice(word);
            size += word_length;
        }
        // The C scanner asserts this invariant. Invalid/truncated snapshots are
        // not produced in defined C behavior; reject them without unsafe reads.
        if size != buffer.len() {
            self.reset();
        }
    }
}

/// Creates the state corresponding to C's zero-initialized scanner payload.
pub(crate) fn create() -> Box<dyn ExternalScanner> {
    Box::<Scanner>::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Eq)]
    enum Call {
        Advance(usize, bool),
        MarkEnd(usize),
        Symbol(u16),
    }

    struct TestLexer {
        input: Vec<i32>,
        position: usize,
        end: Option<usize>,
        symbol: u16,
        range_start: bool,
        calls: Vec<Call>,
    }

    impl TestLexer {
        fn new(input: &str) -> Self {
            Self {
                input: input.chars().map(|c| c as i32).collect(),
                position: 0,
                end: None,
                symbol: u16::MAX,
                range_start: false,
                calls: Vec::new(),
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
            self.calls.push(Call::Symbol(symbol));
        }
        fn advance(&mut self, skip: bool) {
            self.calls.push(Call::Advance(self.position, skip));
            if !self.eof() {
                self.position += 1;
            }
        }
        fn mark_end(&mut self) {
            self.end = Some(self.position);
            self.calls.push(Call::MarkEnd(self.position));
        }
        fn get_column(&mut self) -> u32 {
            panic!("Ruby's scanner does not call get_column")
        }
        fn is_at_included_range_start(&self) -> bool {
            self.range_start
        }
        fn eof(&self) -> bool {
            self.position == self.input.len()
        }
    }

    fn symbols(tokens: &[usize]) -> [bool; NONE] {
        let mut result = [false; NONE];
        for &token in tokens {
            result[token] = true;
        }
        result
    }

    fn scan(scanner: &mut Scanner, input: &str, valid: &[usize]) -> (bool, TestLexer) {
        let mut lexer = TestLexer::new(input);
        let result = scanner.scan(&mut lexer, &symbols(valid));
        (result, lexer)
    }

    fn literal(kind: usize, open: u8, close: u8, interpolation: bool) -> Literal {
        Literal {
            kind,
            open_delimiter: i32::from(open),
            close_delimiter: i32::from(close),
            nesting_depth: 1,
            allows_interpolation: interpolation,
        }
    }

    fn heredoc(word: &[u8], indentation: bool, interpolation: bool) -> Heredoc {
        Heredoc {
            word: word.to_vec(),
            end_word_indentation_allowed: indentation,
            allows_interpolation: interpolation,
            started: true,
        }
    }

    #[test]
    fn nul_does_not_end_literal_or_heredoc_content() {
        let mut scanner = Scanner::default();
        scanner
            .literal_stack
            .push(literal(STRING_START, b'"', b'"', true));
        let (ok, lexer) = scan(&mut scanner, "x\0y\"", &[STRING_CONTENT]);
        assert!(ok);
        assert_eq!(lexer.symbol, STRING_CONTENT as u16);
        assert_eq!(lexer.position, 3);
        assert_eq!(lexer.end, Some(3));

        scanner.open_heredocs.push(heredoc(b"END", false, true));
        let (ok, lexer) = scan(&mut scanner, "x\0y\nEND\n", &[HEREDOC_CONTENT]);
        assert!(ok);
        assert_eq!(lexer.symbol, HEREDOC_CONTENT as u16);
        assert_eq!(lexer.position, 7);
        assert_eq!(lexer.end, Some(4));
    }

    #[test]
    fn serialization_layout_and_unsigned_byte_restoration() {
        let mut scanner = Scanner {
            has_leading_whitespace: true,
            literal_stack: vec![
                literal(STRING_START, b'{', b'}', true),
                Literal {
                    kind: STRING_ARRAY_START,
                    open_delimiter: 0x128,
                    close_delimiter: 0xff,
                    nesting_depth: 258,
                    allows_interpolation: false,
                },
            ],
            open_heredocs: vec![
                heredoc(b"END", true, false),
                Heredoc {
                    word: vec![0xe9],
                    allows_interpolation: true,
                    ..Heredoc::default()
                },
            ],
        };
        let mut buffer = [0; SERIALIZATION_BUFFER_SIZE];
        let size = scanner.serialize(&mut buffer);
        assert_eq!(
            &buffer[..size],
            &[
                2, 3, b'{', b'}', 1, 1, 7, b'(', 0xff, 2, 0, 2, 1, 0, 1, 3, b'E', b'N', b'D', 0, 1,
                0, 1, 0xe9,
            ]
        );
        scanner.deserialize(&buffer[..size]);
        assert!(!scanner.has_leading_whitespace);
        assert_eq!(scanner.literal_stack[1].open_delimiter, 40);
        assert_eq!(scanner.literal_stack[1].close_delimiter, 255);
        assert_eq!(scanner.literal_stack[1].nesting_depth, 2);
        assert_eq!(scanner.open_heredocs[1].word, [0xe9]);
        let mut restored = [0; SERIALIZATION_BUFFER_SIZE];
        assert_eq!(scanner.serialize(&mut restored), size);
        assert_eq!(&restored[..size], &buffer[..size]);
        scanner.deserialize(&[]);
        assert!(scanner.literal_stack.is_empty());
        assert!(scanner.open_heredocs.is_empty());
        assert_eq!(scanner.serialize(&mut buffer), 2);
        assert_eq!(&buffer[..2], &[0, 0]);
    }

    #[test]
    fn restoring_snapshots_reuses_literal_and_heredoc_allocations() {
        let mut scanner = Scanner {
            literal_stack: vec![literal(STRING_START, b'{', b'}', true)],
            open_heredocs: vec![
                heredoc(b"FIRST", true, false),
                heredoc(b"LAST", false, true),
            ],
            ..Scanner::default()
        };
        scanner.literal_stack.reserve(32);
        scanner.open_heredocs.reserve(8);
        for heredoc in &mut scanner.open_heredocs {
            heredoc.word.reserve(64);
        }
        let literal_capacity = scanner.literal_stack.capacity();
        let heredoc_capacity = scanner.open_heredocs.capacity();
        let word_capacities = [
            scanner.open_heredocs[0].word.capacity(),
            scanner.open_heredocs[1].word.capacity(),
        ];
        let mut buffer = [0; SERIALIZATION_BUFFER_SIZE];
        let size = scanner.serialize(&mut buffer);
        for _ in 0..8 {
            // Simulate a scan that changed state before the parser backtracked.
            scanner.literal_stack.clear();
            for heredoc in &mut scanner.open_heredocs {
                heredoc.word.clear();
                heredoc.started = false;
                heredoc.allows_interpolation = false;
            }
            scanner.deserialize(&buffer[..size]);
            assert_eq!(scanner.literal_stack.capacity(), literal_capacity);
            assert_eq!(scanner.open_heredocs.capacity(), heredoc_capacity);
            for (heredoc, capacity) in scanner.open_heredocs.iter().zip(word_capacities) {
                assert_eq!(heredoc.word.capacity(), capacity);
            }
            let mut restored = [0; SERIALIZATION_BUFFER_SIZE];
            assert_eq!(scanner.serialize(&mut restored), size);
            assert_eq!(&restored[..size], &buffer[..size]);
        }
        // A shorter snapshot must discard stale entries, not serialize them.
        scanner.deserialize(&[0, 1, 0, 1, 0, 1, b'X']);
        assert!(scanner.literal_stack.is_empty());
        assert_eq!(scanner.open_heredocs.len(), 1);
        assert_eq!(scanner.open_heredocs[0].word, b"X");
        assert_eq!(scanner.open_heredocs[0].word.capacity(), word_capacities[0]);
        assert!(!scanner.open_heredocs[0].started);
        scanner.deserialize(&[]);
        assert!(scanner.open_heredocs.is_empty());
        assert_eq!(scanner.literal_stack.capacity(), literal_capacity);
        assert_eq!(scanner.open_heredocs.capacity(), heredoc_capacity);
    }

    #[test]
    fn serialization_limits_and_safe_rejection_of_invalid_buffers() {
        let mut scanner = Scanner {
            literal_stack: vec![literal(STRING_START, b'"', b'"', true); 204],
            ..Scanner::default()
        };
        let mut buffer = [0; SERIALIZATION_BUFFER_SIZE];
        assert_eq!(scanner.serialize(&mut buffer), 1022);
        scanner.literal_stack.push(scanner.literal_stack[0]);
        assert_eq!(scanner.serialize(&mut buffer), 0);
        scanner.literal_stack.clear();
        for _ in 0..4 {
            scanner
                .open_heredocs
                .push(heredoc(&[b'x'; 250], false, true));
        }
        scanner.open_heredocs.push(heredoc(b"XX", false, true));
        assert_eq!(scanner.serialize(&mut buffer), 1024);
        scanner.deserialize(&buffer);
        assert_eq!(scanner.open_heredocs.len(), 5);
        scanner.open_heredocs[4].word.push(b'X');
        // The C scanner writes one byte past its buffer for this state.
        assert_eq!(scanner.serialize(&mut buffer), 0);
        assert_eq!(scanner.serialize(&mut [0; 1]), 0);
        for bytes in [&[1][..], &[0, 1, 0, 0, 0, 5], &[0, 0, 0]] {
            scanner.deserialize(bytes);
            assert!(scanner.literal_stack.is_empty());
            assert!(scanner.open_heredocs.is_empty());
        }
    }

    #[test]
    fn newline_range_operator_and_call_operator_lookahead() {
        let mut scanner = Scanner::default();
        let (ok, lexer) = scan(&mut scanner, "\n ..x", &[LINE_BREAK]);
        assert!(ok);
        assert_eq!(lexer.end, Some(0));
        assert_eq!(lexer.position, 3);
        assert_eq!(
            lexer.calls,
            [
                Call::Symbol(NONE as u16),
                Call::MarkEnd(0),
                Call::Advance(0, false),
                Call::Advance(1, true),
                Call::Advance(2, false),
                Call::Symbol(LINE_BREAK as u16),
            ]
        );
        let (ok, lexer) = scan(&mut scanner, "\n .method", &[LINE_BREAK]);
        assert!(!ok);
        assert_eq!(lexer.position, 3);
        assert_eq!(lexer.end, Some(0));
        let (ok, lexer) = scan(&mut scanner, "\n x", &[LINE_BREAK, NO_LINE_BREAK]);
        assert!(!ok);
        assert_eq!(lexer.end, None);
        assert_eq!(lexer.position, 2);
        let mut lexer = TestLexer::new("x");
        lexer.range_start = true;
        assert!(scanner.scan(&mut lexer, &symbols(&[LINE_BREAK])));
        assert_eq!(
            lexer.calls,
            [
                Call::Symbol(NONE as u16),
                Call::MarkEnd(0),
                Call::Symbol(LINE_BREAK as u16),
            ]
        );
    }

    #[test]
    fn whitespace_sensitive_operator_choices() {
        let mut scanner = Scanner::default();
        let valid = [
            SPLAT_STAR,
            BINARY_STAR,
            HASH_SPLAT_STAR_STAR,
            BINARY_STAR_STAR,
            UNARY_MINUS,
            UNARY_MINUS_NUM,
            BINARY_MINUS,
            BLOCK_AMPERSAND,
        ];
        for (input, token) in [
            ("*x", BINARY_STAR),
            (" *x", SPLAT_STAR),
            (" * x", BINARY_STAR),
            ("**x", BINARY_STAR_STAR),
            (" **x", HASH_SPLAT_STAR_STAR),
            (" ** x", BINARY_STAR_STAR),
            ("-x", BINARY_MINUS),
            (" -x", UNARY_MINUS),
            (" - x", BINARY_MINUS),
            (" -1", UNARY_MINUS_NUM),
            ("-1", BINARY_MINUS),
            ("&x", BLOCK_AMPERSAND),
        ] {
            let (ok, lexer) = scan(&mut scanner, input, &valid);
            assert!(ok, "{input:?}");
            assert_eq!(usize::from(lexer.symbol), token, "{input:?}");
        }
        for input in ["*=", "**=", "-=", "->", "&&", "&.", "&=", "& "] {
            assert!(!scan(&mut scanner, input, &valid).0, "{input:?}");
        }
        // Numeric-minus specifically consults BINARY_STAR, not BINARY_MINUS.
        let (ok, lexer) = scan(&mut scanner, "-1", &[UNARY_MINUS_NUM, BINARY_MINUS]);
        assert!(ok);
        assert_eq!(usize::from(lexer.symbol), UNARY_MINUS_NUM);
    }

    #[test]
    fn percent_literal_modifiers_and_zero_delimiters() {
        let mut scanner = Scanner::default();
        let valid = [
            STRING_START,
            SIMPLE_SYMBOL,
            SUBSHELL_START,
            REGEX_START,
            STRING_ARRAY_START,
            SYMBOL_ARRAY_START,
        ];
        for (input, kind, interpolation) in [
            ("%s{", SYMBOL_START, false),
            ("%r{", REGEX_START, true),
            ("%x{", SUBSHELL_START, true),
            ("%q{", STRING_START, false),
            ("%Q{", STRING_START, true),
            ("%w{", STRING_ARRAY_START, false),
            ("%W{", STRING_ARRAY_START, true),
            ("%i{", SYMBOL_ARRAY_START, false),
            ("%I{", SYMBOL_ARRAY_START, true),
            ("%{", STRING_START, true),
        ] {
            let (ok, lexer) = scan(&mut scanner, input, &valid);
            assert!(ok, "{input:?}");
            assert_eq!(usize::from(lexer.symbol), kind);
            assert_eq!(
                scanner.literal_stack.pop(),
                Some(literal(kind, b'{', b'}', interpolation))
            );
        }
        assert!(!scan(&mut scanner, "%s{", &[STRING_START, SYMBOL_START]).0);
        assert!(!scan(&mut scanner, "%=", &valid).0);
        assert!(!scan(&mut scanner, "% ", &[STRING_START, FORWARD_SLASH]).0);
        assert!(scan(&mut scanner, "% ", &[STRING_START]).0);
        assert_eq!(scanner.literal_stack[0], literal(STRING_START, 0, 0, true));
        let (ok, lexer) = scan(&mut scanner, "", &[STRING_END]);
        assert!(ok);
        assert_eq!(
            lexer.calls,
            [
                Call::MarkEnd(0),
                Call::Advance(0, false),
                Call::Symbol(STRING_END as u16),
                Call::MarkEnd(0),
            ]
        );
        assert!(scanner.literal_stack.is_empty());
    }

    #[test]
    fn regex_slash_ambiguity_and_option_suffix() {
        let mut scanner = Scanner::default();
        let valid = [STRING_START, REGEX_START, FORWARD_SLASH];
        for input in ["/a", " / ", " /=", " /\n"] {
            assert!(!scan(&mut scanner, input, &valid).0, "{input:?}");
        }
        assert!(scan(&mut scanner, " /a", &valid).0);
        let (ok, lexer) = scan(&mut scanner, "/imX", &[STRING_END]);
        assert!(ok);
        assert_eq!(usize::from(lexer.symbol), STRING_END);
        assert_eq!(lexer.position, 3);
        assert_eq!(lexer.end, Some(3));
        assert!(scanner.literal_stack.is_empty());
    }

    #[test]
    fn nested_literal_content_and_interpolation_boundaries() {
        let mut scanner = Scanner::default();
        assert!(scan(&mut scanner, "%Q{", &[STRING_START]).0);
        let (ok, lexer) = scan(&mut scanner, "a{b}#{x}", &[STRING_CONTENT]);
        assert!(ok);
        assert_eq!(usize::from(lexer.symbol), STRING_CONTENT);
        assert_eq!(lexer.end, Some(4));
        assert_eq!(lexer.position, 5);
        assert_eq!(scanner.literal_stack[0].nesting_depth, 1);
        let (ok, lexer) = scan(&mut scanner, "#{x}", &[STRING_CONTENT]);
        assert!(!ok);
        assert_eq!(lexer.calls, [Call::MarkEnd(0), Call::Advance(0, false)]);
        let (ok, lexer) = scan(&mut scanner, "#@name}", &[STRING_CONTENT]);
        assert!(ok);
        assert_eq!(
            lexer.calls,
            [
                Call::MarkEnd(0),
                Call::Advance(0, false),
                Call::MarkEnd(1),
                Call::Advance(1, false),
                Call::Symbol(SHORT_INTERPOLATION as u16),
            ]
        );
        let (ok, lexer) = scan(&mut scanner, "x#@name}", &[STRING_CONTENT]);
        assert!(ok);
        assert_eq!(lexer.end, Some(1));
        assert_eq!(lexer.position, 2);
        assert_eq!(usize::from(lexer.symbol), STRING_CONTENT);
        let (ok, lexer) = scan(&mut scanner, "abc", &[STRING_CONTENT]);
        assert!(!ok);
        assert_eq!(
            &lexer.calls[lexer.calls.len() - 2..],
            &[Call::Advance(3, false), Call::MarkEnd(3),]
        );
    }

    #[test]
    fn integer_dispatch_does_not_narrow_non_ascii_lookahead() {
        let mut scanner = Scanner::default();
        for lookahead in [
            -1,
            0xd800,
            0x110000,
            0x100 + i32::from(b'['),
            0x100 + i32::from(b'"'),
        ] {
            let mut lexer = TestLexer::new("");
            lexer.input.push(lookahead);
            assert!(!scanner.scan(
                &mut lexer,
                &symbols(&[STRING_START, ELEMENT_REFERENCE_BRACKET, BLOCK_AMPERSAND]),
            ));
            assert_eq!(lexer.position, 0);
            assert!(scanner.literal_stack.is_empty());
        }
        // Only is_iden_char uses the narrowed byte, including bytes >= 128.
        for byte in 0..=u8::MAX {
            assert_eq!(
                is_iden_char(byte),
                !b"\0\n\r\t :;`\"'@$#.,|^&<=>+-*/\\%?!~()[]{}".contains(&byte),
            );
        }
    }

    #[test]
    fn byte_narrowing_and_c_locale_character_classes() {
        assert!(is_space(0x0b));
        assert!(!is_space(0xa0));
        assert!(!is_alpha('é' as i32));
        assert!(!is_iden_char('Ā' as u32 as u8));
        assert!(is_iden_char('é' as u32 as u8));
        let mut lexer = TestLexer::new("$\0");
        assert!(scan_short_interpolation(&mut lexer, false, STRING_CONTENT));
        assert_eq!(usize::from(lexer.symbol), SHORT_INTERPOLATION);
        assert_eq!(lexer.end, Some(0));
        let mut lexer = TestLexer::new("$İ"); // low byte is '0'
        assert!(!scan_short_interpolation(&mut lexer, false, STRING_CONTENT));
        let mut lexer = TestLexer::new("$ā"); // low byte is 1, not punctuation
        assert!(!scan_short_interpolation(&mut lexer, false, STRING_CONTENT));
        let mut lexer = TestLexer::new("$Ā"); // strchr matches low-byte NUL
        assert!(scan_short_interpolation(&mut lexer, false, STRING_CONTENT));
    }

    #[test]
    fn array_whitespace_and_noninterpolated_escapes() {
        let mut scanner = Scanner::default();
        scanner
            .literal_stack
            .push(literal(STRING_ARRAY_START, b'[', b']', false));
        let (ok, lexer) = scan(&mut scanner, "x\u{b}", &[STRING_CONTENT]);
        assert!(ok);
        assert_eq!(lexer.position, 1);
        assert_eq!(lexer.end, Some(1));
        let (ok, lexer) = scan(&mut scanner, "\u{b}", &[STRING_CONTENT]);
        assert!(!ok);
        assert!(lexer.calls.is_empty());
        let (ok, lexer) = scan(&mut scanner, "\\]x]", &[STRING_CONTENT]);
        assert!(ok);
        assert_eq!(lexer.position, 3);
        assert_eq!(lexer.end, Some(3));
        assert_eq!(scanner.literal_stack[0].nesting_depth, 1);
    }

    #[test]
    fn symbol_assignment_hash_keys_and_identifier_suffixes() {
        let mut scanner = Scanner::default();
        let (ok, lexer) = scan(&mut scanner, ":foo=>", &[SYMBOL_START]);
        assert!(ok);
        assert_eq!(usize::from(lexer.symbol), SIMPLE_SYMBOL);
        assert_eq!(lexer.end, Some(4));
        assert_eq!(lexer.position, 5);
        let (ok, lexer) = scan(&mut scanner, ":foo=x", &[SYMBOL_START]);
        assert!(ok);
        assert_eq!(lexer.end, Some(5));
        for input in [":[]=", ":<=>", ":@@name", ":$foo", ":é"] {
            let (ok, lexer) = scan(&mut scanner, input, &[SYMBOL_START]);
            assert!(ok, "{input:?}");
            assert!(lexer.eof());
        }
        assert!(!scan(&mut scanner, ":Ā", &[SYMBOL_START]).0);
        let (ok, lexer) = scan(&mut scanner, "key:val", &[HASH_KEY_SYMBOL]);
        assert!(ok);
        assert_eq!(lexer.end, Some(3));
        assert_eq!(lexer.position, 4);
        assert!(!scan(&mut scanner, "key::val", &[HASH_KEY_SYMBOL]).0);
        for (input, token) in [("name!", IDENTIFIER_SUFFIX), ("Const!", CONSTANT_SUFFIX)] {
            let (ok, lexer) = scan(&mut scanner, input, &[IDENTIFIER_SUFFIX, CONSTANT_SUFFIX]);
            assert!(ok);
            assert_eq!(usize::from(lexer.symbol), token);
        }
        assert!(!scan(&mut scanner, "name!=", &[IDENTIFIER_SUFFIX]).0);
    }

    #[test]
    fn heredoc_body_start_indentation_and_terminator_boundaries() {
        let mut scanner = Scanner::default();
        let (ok, lexer) = scan(&mut scanner, "<<~'END'", &[STRING_START]);
        assert!(ok);
        assert_eq!(usize::from(lexer.symbol), HEREDOC_START);
        assert!(!scanner.open_heredocs[0].allows_interpolation);
        assert!(scanner.open_heredocs[0].end_word_indentation_allowed);
        assert!(!scanner.open_heredocs[0].started);
        let (ok, lexer) = scan(&mut scanner, "\n", &[HEREDOC_BODY_START]);
        assert!(ok);
        assert_eq!(lexer.position, 0);
        assert_eq!(
            lexer.calls,
            [
                Call::Symbol(NONE as u16),
                Call::Symbol(HEREDOC_BODY_START as u16),
            ]
        );
        assert!(scanner.open_heredocs[0].started);
        let (ok, lexer) = scan(&mut scanner, "\n  abc\n  END\n", &[HEREDOC_CONTENT]);
        assert!(ok);
        assert_eq!(usize::from(lexer.symbol), HEREDOC_CONTENT);
        assert_eq!(lexer.position, 12);
        assert_eq!(lexer.end, Some(9));
        assert_eq!(scanner.open_heredocs.len(), 1);
        let (ok, lexer) = scan(&mut scanner, "END  \n", &[HEREDOC_BODY_END]);
        assert!(ok);
        assert_eq!(usize::from(lexer.symbol), HEREDOC_BODY_END);
        assert_eq!(lexer.position, 5);
        assert_eq!(lexer.end, Some(3));
        assert!(scanner.open_heredocs.is_empty());
    }

    #[test]
    fn heredoc_eof_interpolation_and_signed_word_bytes() {
        let mut scanner = Scanner::default();
        scanner.open_heredocs.push(heredoc(b"END", false, true));
        let (ok, lexer) = scan(&mut scanner, "EN", &[HEREDOC_CONTENT]);
        assert!(ok);
        assert_eq!(usize::from(lexer.symbol), HEREDOC_BODY_END);
        assert_eq!(lexer.end, Some(2));
        assert!(scanner.open_heredocs.is_empty());
        scanner.open_heredocs.push(heredoc(b"END", false, true));
        let (ok, lexer) = scan(&mut scanner, "#", &[HEREDOC_CONTENT]);
        assert!(ok);
        // C does not set has_content for an uninterpolated '#'.
        assert_eq!(usize::from(lexer.symbol), HEREDOC_BODY_END);
        assert_eq!(lexer.end, Some(1));
        scanner.open_heredocs.push(heredoc(b"END", false, true));
        let (ok, lexer) = scan(&mut scanner, "#@name", &[HEREDOC_CONTENT]);
        assert!(ok);
        assert_eq!(usize::from(lexer.symbol), SHORT_INTERPOLATION);
        assert_eq!(lexer.end, Some(1));
        let (ok, lexer) = scan(&mut scanner, "\\x", &[HEREDOC_CONTENT]);
        assert!(!ok);
        assert!(lexer.calls.is_empty());
        scanner.open_heredocs[0].word = vec![0xe9];
        let (ok, lexer) = scan(&mut scanner, "é\n", &[HEREDOC_CONTENT]);
        assert!(ok);
        assert_eq!(usize::from(lexer.symbol), HEREDOC_CONTENT);
        assert_eq!(scanner.open_heredocs.len(), 1);
    }
}
