//! Ruby's external scanner, translated from `src/scanner.c`.

use tree_sitter_language::{ExternalScanner, Lexer, SERIALIZATION_BUFFER_SIZE};

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

// All recognized source delimiters are ASCII; restored delimiters and kinds
// are bytes as well. Only nesting_depth needs C's full int width while scanning
// a token (it is deliberately narrowed only at serialization).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct Literal {
    kind: u8,
    open_delimiter: u8,
    close_delimiter: u8,
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
/// Leading whitespace is scan-local: C never includes it in a snapshot.
#[derive(Debug, Default)]
pub(crate) struct Scanner {
    literal_stack: Vec<Literal>,
    open_heredocs: Vec<Heredoc>,
}

// C reads lookahead directly from TSLexer. Keep the same value locally so
// repeated character tests do not each dispatch through the Lexer trait object.
// Only advance changes lookahead; mark_end and result-symbol updates do not.
struct CachedLexer<'a> {
    inner: &'a mut dyn Lexer,
    lookahead: i32,
}

impl<'a> CachedLexer<'a> {
    fn new(inner: &'a mut dyn Lexer) -> Self {
        let lookahead = inner.lookahead();
        Self { inner, lookahead }
    }

    fn lookahead(&self) -> i32 {
        self.lookahead
    }

    fn advance(&mut self, skip: bool) {
        self.inner.advance(skip);
        self.lookahead = self.inner.lookahead();
    }

    fn mark_end(&mut self) {
        self.inner.mark_end();
    }

    fn set_result_symbol(&mut self, symbol: u16) {
        self.inner.set_result_symbol(symbol);
    }

    fn is_at_included_range_start(&self) -> bool {
        self.inner.is_at_included_range_start()
    }

    fn eof(&self) -> bool {
        // A nonzero lookahead is never EOF, but zero can be an embedded NUL.
        self.lookahead == 0 && self.inner.eof()
    }
}

// wctype as in C's default locale, not Unicode categories.
fn is_space(c: i32) -> bool {
    matches!(c, 0x09..=0x0d | 0x20)
}

fn is_alpha(c: i32) -> bool {
    matches!(c, 0x41..=0x5a | 0x61..=0x7a)
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

// Classify the C-locale identifier alphabet once per source character.
// Digits continue a word; letters and '_' start one; uppercase letters choose
// CONSTANT_SUFFIX instead of IDENTIFIER_SUFFIX.
fn word_char_kind(c: i32) -> u8 {
    const WORD_CHARS: [u8; 128] = {
        let mut table = [0; 128];
        let mut i = 0;
        while i < table.len() {
            table[i] = match i {
                0x30..=0x39 => 1,
                0x61..=0x7a | 0x5f => 2,
                0x41..=0x5a => 3,
                _ => 0,
            };
            i += 1;
        }
        table
    };
    if (c as u32) < WORD_CHARS.len() as u32 {
        WORD_CHARS[c as usize]
    } else {
        0
    }
}

fn is_word_char(c: i32) -> bool {
    word_char_kind(c) != 0
}

fn advance(lexer: &mut CachedLexer<'_>) {
    lexer.advance(false);
}

fn emit(lexer: &mut CachedLexer<'_>, symbol: usize) -> bool {
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

fn scan_operator(lexer: &mut CachedLexer<'_>) -> bool {
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

fn scan_symbol_identifier(lexer: &mut CachedLexer<'_>) -> bool {
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

fn scan_heredoc_word(lexer: &mut CachedLexer<'_>, heredoc: &mut Heredoc) {
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
    lexer: &mut CachedLexer<'_>,
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

#[derive(Debug, PartialEq, Eq)]
enum WhitespaceResult {
    Failed,
    // Whether skipped whitespace affects the following operator/literal.
    Skipped(bool),
    LineBreak,
    HeredocStart,
}

impl Scanner {
    fn clear_heredocs(&mut self) {
        if !self.open_heredocs.is_empty() {
            self.clear_heredocs_nonempty();
        }
    }

    // Keep heredoc-word deallocation off the empty/single-literal restore path.
    #[cold]
    #[inline(never)]
    fn clear_heredocs_nonempty(&mut self) {
        self.open_heredocs.clear();
    }

    fn reset(&mut self) {
        self.literal_stack.clear();
        self.clear_heredocs();
    }

    // Only CR/LF can start a heredoc body. Inspect the queue at those positions
    // rather than loading and retaining its header on every scanner attempt.
    fn start_heredoc(&mut self, valid_symbols: &[bool; NONE]) -> bool {
        if valid_symbols[HEREDOC_BODY_START]
            && let Some(heredoc) = self.open_heredocs.first_mut()
            && !heredoc.started
        {
            heredoc.started = true;
            true
        } else {
            false
        }
    }

    fn scan_whitespace(
        &mut self,
        lexer: &mut CachedLexer<'_>,
        valid_symbols: &[bool; NONE],
    ) -> WhitespaceResult {
        let mut has_leading_whitespace = false;
        let line_break_is_valid = !valid_symbols[NO_LINE_BREAK] && valid_symbols[LINE_BREAK];
        loop {
            if line_break_is_valid && lexer.is_at_included_range_start() {
                lexer.mark_end();
                return WhitespaceResult::LineBreak;
            }
            let lookahead = lexer.lookahead();
            match lookahead {
                0x20 | 0x09 => {
                    has_leading_whitespace = true;
                    lexer.advance(true);
                } // ' ', '\t'
                0x0d => {
                    // '\r'
                    if self.start_heredoc(valid_symbols) {
                        return WhitespaceResult::HeredocStart;
                    }
                    has_leading_whitespace = true;
                    lexer.advance(true);
                }
                0x0a => {
                    // '\n'
                    if self.start_heredoc(valid_symbols) {
                        return WhitespaceResult::HeredocStart;
                    } else if line_break_is_valid {
                        lexer.mark_end();
                        advance(lexer);
                        return Self::scan_whitespace_after_newline(lexer, has_leading_whitespace);
                    } else {
                        has_leading_whitespace = true;
                        lexer.advance(true);
                    }
                }
                0x5c => {
                    // '\\'
                    advance(lexer);
                    if lexer.lookahead() == i32::from(b'\r') {
                        lexer.advance(true);
                    }
                    if is_space(lexer.lookahead()) {
                        has_leading_whitespace = true;
                        lexer.advance(true);
                    } else {
                        return WhitespaceResult::Failed;
                    }
                }
                _ => {
                    return WhitespaceResult::Skipped(has_leading_whitespace);
                }
            }
        }
    }

    // Once a significant newline has been consumed, line breaks are known
    // to be valid and no heredoc body can start in this scan. Keep these facts
    // out of the ordinary whitespace loop's per-character state.
    #[inline(always)]
    fn scan_whitespace_after_newline(
        lexer: &mut CachedLexer<'_>,
        mut has_leading_whitespace: bool,
    ) -> WhitespaceResult {
        loop {
            if lexer.is_at_included_range_start() {
                lexer.mark_end();
                return WhitespaceResult::LineBreak;
            }
            match lexer.lookahead() {
                0x20 | 0x09 | 0x0d | 0x0a => {
                    has_leading_whitespace = true;
                    lexer.advance(true);
                }
                0x5c => {
                    advance(lexer);
                    if lexer.lookahead() == i32::from(b'\r') {
                        lexer.advance(true);
                    }
                    if is_space(lexer.lookahead()) {
                        has_leading_whitespace = true;
                        lexer.advance(true);
                    } else {
                        return WhitespaceResult::Failed;
                    }
                }
                0x2e => {
                    advance(lexer);
                    return if !lexer.eof() && lexer.lookahead() == i32::from(b'.') {
                        WhitespaceResult::LineBreak
                    } else {
                        WhitespaceResult::Failed
                    };
                }
                0x26 | 0x23 => return WhitespaceResult::Skipped(has_leading_whitespace),
                _ => return WhitespaceResult::LineBreak,
            }
        }
    }

    fn scan_open_delimiter(
        has_leading_whitespace: bool,
        lexer: &mut CachedLexer<'_>,
        literal: &mut Literal,
        valid_symbols: &[bool; NONE],
    ) -> bool {
        match lexer.lookahead() {
            0x22 | 0x27 => {
                // '"', "'"
                literal.kind = STRING_START as u8;
                literal.open_delimiter = lexer.lookahead() as u8;
                literal.close_delimiter = lexer.lookahead() as u8;
                literal.allows_interpolation = lexer.lookahead() == i32::from(b'"');
                advance(lexer);
                true
            }
            0x60 => {
                // '`'
                if !valid_symbols[SUBSHELL_START] {
                    return false;
                }
                literal.kind = SUBSHELL_START as u8;
                literal.open_delimiter = lexer.lookahead() as u8;
                literal.close_delimiter = lexer.lookahead() as u8;
                literal.allows_interpolation = true;
                advance(lexer);
                true
            }
            0x2f => {
                // '/'
                if !valid_symbols[REGEX_START] {
                    return false;
                }
                literal.kind = REGEX_START as u8;
                literal.open_delimiter = lexer.lookahead() as u8;
                literal.close_delimiter = lexer.lookahead() as u8;
                literal.allows_interpolation = true;
                advance(lexer);
                if valid_symbols[FORWARD_SLASH] {
                    if !has_leading_whitespace {
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
                literal.kind = kind as u8;
                literal.allows_interpolation = interpolation;
                if has_modifier {
                    advance(lexer);
                }
                match lexer.lookahead() {
                    0x28 => {
                        // '('
                        literal.open_delimiter = b'(';
                        literal.close_delimiter = b')';
                    }
                    0x5b => {
                        // '['
                        literal.open_delimiter = b'[';
                        literal.close_delimiter = b']';
                    }
                    0x7b => {
                        // '{'
                        literal.open_delimiter = b'{';
                        literal.close_delimiter = b'}';
                    }
                    0x3c => {
                        // '<'
                        literal.open_delimiter = b'<';
                        literal.close_delimiter = b'>';
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
                        literal.open_delimiter = lexer.lookahead() as u8;
                        literal.close_delimiter = lexer.lookahead() as u8;
                    }
                    _ => return false,
                }
                advance(lexer);
                true
            }
            _ => false,
        }
    }

    fn scan_heredoc_content(&mut self, lexer: &mut CachedLexer<'_>) -> bool {
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

    fn scan_literal_content(&mut self, lexer: &mut CachedLexer<'_>) -> bool {
        let literal = self.literal_stack.last_mut().unwrap();
        let mut has_content = false;
        let stop_on_space = usize::from(literal.kind) == SYMBOL_ARRAY_START
            || usize::from(literal.kind) == STRING_ARRAY_START;
        loop {
            let lookahead = lexer.lookahead();
            if stop_on_space && is_space(lookahead) {
                if has_content {
                    lexer.mark_end();
                    return emit(lexer, STRING_CONTENT);
                }
                return false;
            }
            if lookahead == i32::from(literal.close_delimiter) {
                lexer.mark_end();
                if literal.nesting_depth == 1 {
                    if has_content {
                        lexer.set_result_symbol(STRING_CONTENT as u16);
                    } else {
                        advance(lexer);
                        if usize::from(literal.kind) == REGEX_START {
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
            } else if lookahead == i32::from(literal.open_delimiter) {
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

    fn scan_cached(&mut self, lexer: &mut CachedLexer<'_>, valid_symbols: &[bool]) -> bool {
        // The grammar has NONE external tokens. Check that once instead of
        // bounds-checking each individual token lookup in the scanning paths.
        let valid_symbols: &[bool; NONE] = valid_symbols[..NONE].try_into().unwrap();
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

        // NONE is only a local sentinel in C's whitespace scanner, not an
        // emitted token. Return an explicit outcome instead of passing that
        // sentinel through dynamic result-symbol calls on every attempt.
        // As with C, a failed scan's result symbol is ignored by the parser.
        let has_leading_whitespace = match self.scan_whitespace(lexer, valid_symbols) {
            WhitespaceResult::Failed => return false,
            WhitespaceResult::Skipped(has_leading_whitespace) => has_leading_whitespace,
            WhitespaceResult::LineBreak => return emit(lexer, LINE_BREAK),
            WhitespaceResult::HeredocStart => return emit(lexer, HEREDOC_BODY_START),
        };

        let lookahead = lexer.lookahead();
        // Identifier characters are disjoint from every operator/delimiter
        // below. Handle this common case without walking their dispatch first.
        let word_kind = word_char_kind(lookahead);
        if word_kind >= 2
            && (valid_symbols[HASH_KEY_SYMBOL]
                | valid_symbols[IDENTIFIER_SUFFIX]
                | (valid_symbols[CONSTANT_SUFFIX] & (word_kind == 3)))
        {
            let valid_identifier_symbol = if word_kind == 3 {
                CONSTANT_SUFFIX
            } else {
                IDENTIFIER_SUFFIX
            };
            // The first character was already validated above. Keep lookahead
            // local throughout the run: neither advance nor lookahead needs to
            // read the wrapper, so only synchronize it at the suffix boundary.
            let mut lookahead;
            loop {
                lexer.inner.advance(false);
                lookahead = lexer.inner.lookahead();
                if !is_word_char(lookahead) {
                    break;
                }
            }
            lexer.lookahead = lookahead;
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
                            if valid_symbols[BINARY_STAR_STAR] && !has_leading_whitespace {
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
                    if valid_symbols[BINARY_STAR] && !has_leading_whitespace {
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
                            && (!valid_symbols[BINARY_STAR] || has_leading_whitespace)
                            && is_digit(lexer.lookahead())
                        {
                            return emit(lexer, UNARY_MINUS_NUM);
                        }
                        if valid_symbols[UNARY_MINUS]
                            && has_leading_whitespace
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
                    return self.scan_symbol_start(lexer);
                }
            }
            0x5b if valid_symbols[ELEMENT_REFERENCE_BRACKET]
                && (!has_leading_whitespace || !valid_symbols[STRING_START]) =>
            {
                advance(lexer);
                return emit(lexer, ELEMENT_REFERENCE_BRACKET);
            }
            _ => {}
        }

        if valid_symbols[STRING_START] {
            return self.scan_literal_start(lexer, valid_symbols, has_leading_whitespace);
        }
        false
    }

    #[inline(never)]
    fn scan_symbol_start(&mut self, lexer: &mut CachedLexer<'_>) -> bool {
        let mut literal = Literal {
            kind: SYMBOL_START as u8,
            nesting_depth: 1,
            ..Literal::default()
        };
        advance(lexer);
        match lexer.lookahead() {
            0x22 => {
                // '"'
                advance(lexer);
                literal.open_delimiter = b'"';
                literal.close_delimiter = b'"';
                literal.allows_interpolation = true;
                self.literal_stack.push(literal);
                return emit(lexer, SYMBOL_START);
            }
            0x27 => {
                // "'"
                advance(lexer);
                literal.open_delimiter = b'\'';
                literal.close_delimiter = b'\'';
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
        false
    }

    // Literal/heredoc construction needs scratch state and allocation paths,
    // unlike the common whitespace and identifier attempts.
    #[inline(never)]
    fn scan_literal_start(
        &mut self,
        lexer: &mut CachedLexer<'_>,
        valid_symbols: &[bool; NONE],
        has_leading_whitespace: bool,
    ) -> bool {
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
        if Self::scan_open_delimiter(has_leading_whitespace, lexer, &mut literal, valid_symbols) {
            self.literal_stack.push(literal);
            return emit(lexer, usize::from(literal.kind));
        }
        false
    }
}

impl ExternalScanner for Scanner {
    fn scan(&mut self, lexer: &mut dyn Lexer, valid_symbols: &[bool]) -> bool {
        self.scan_cached(&mut CachedLexer::new(lexer), valid_symbols)
    }

    fn serialize(&mut self, buffer: &mut [u8]) -> usize {
        // Ordinary tokens and unnested strings have fixed-width snapshots.
        // Keep their copies independent of the variable-depth writer.
        if self.open_heredocs.is_empty() {
            match self.literal_stack.as_slice() {
                [] => {
                    let Some(bytes) = buffer.get_mut(..2) else {
                        return 0;
                    };
                    bytes.copy_from_slice(&[0, 0]);
                    return 2;
                }
                [literal] => {
                    let Some(bytes) = buffer.get_mut(..7) else {
                        return 0;
                    };
                    bytes.copy_from_slice(&[
                        1,
                        literal.kind,
                        literal.open_delimiter,
                        literal.close_delimiter,
                        literal.nesting_depth as u8,
                        u8::from(literal.allows_interpolation),
                        0,
                    ]);
                    return 7;
                }
                _ => {}
            }
        }
        self.serialize_nonempty(buffer)
    }

    fn deserialize(&mut self, buffer: &[u8]) {
        if buffer.is_empty() || buffer == [0, 0] {
            self.reset();
        } else if let &[
            1,
            kind,
            open_delimiter,
            close_delimiter,
            nesting_depth,
            interpolation,
            0,
        ] = buffer
            && let Some(top) = self.literal_stack.first_mut()
        {
            // Restore an existing one-literal stack in place. This is the common
            // state between a string's start, content and end tokens. Matching
            // the entire snapshot also excludes truncated heredoc states.
            *top = Literal {
                kind,
                open_delimiter,
                close_delimiter,
                nesting_depth: i32::from(nesting_depth),
                allows_interpolation: interpolation != 0,
            };
            self.literal_stack.truncate(1);
            self.clear_heredocs();
        } else {
            self.deserialize_nonempty(buffer);
        }
    }
}

impl Scanner {
    #[inline(never)]
    fn serialize_nonempty(&mut self, buffer: &mut [u8]) -> usize {
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
                literal.kind,
                literal.open_delimiter,
                literal.close_delimiter,
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

    #[inline(never)]
    fn deserialize_nonempty(&mut self, buffer: &[u8]) {
        // Keep the literal allocation and each still-open heredoc's word buffer:
        // the parser restores a snapshot before every external scan.
        self.literal_stack.clear();
        let mut size = 0;
        let literal_depth = buffer[size];
        size += 1;
        for _ in 0..literal_depth {
            let Some(bytes) = buffer.get(size..size + 5) else {
                self.reset();
                return;
            };
            self.literal_stack.push(Literal {
                kind: bytes[0],
                open_delimiter: bytes[1],
                close_delimiter: bytes[2],
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
        // Usually both counts are zero. Avoid calling the general resize
        // routine unless the number of open heredocs actually changed.
        if self.open_heredocs.len() != usize::from(open_heredoc_count) {
            self.open_heredocs
                .resize_with(usize::from(open_heredoc_count), Heredoc::default);
        }
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
    use std::cell::{Cell, RefCell};

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
        range_starts: Vec<usize>,
        range_calls: RefCell<Vec<usize>>,
        calls: Vec<Call>,
        lookahead_calls: Cell<usize>,
        eof_calls: Cell<usize>,
    }

    impl TestLexer {
        fn new(input: &str) -> Self {
            Self {
                input: input.chars().map(|c| c as i32).collect(),
                position: 0,
                end: None,
                symbol: u16::MAX,
                range_start: false,
                range_starts: Vec::new(),
                range_calls: RefCell::new(Vec::new()),
                calls: Vec::new(),
                lookahead_calls: Cell::new(0),
                eof_calls: Cell::new(0),
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
            self.calls.push(Call::Symbol(symbol));
        }
        fn advance(&mut self, skip: bool) {
            self.calls.push(Call::Advance(self.position, skip));
            if self.position < self.input.len() {
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
            self.range_calls.borrow_mut().push(self.position);
            self.range_start || self.range_starts.contains(&self.position)
        }
        fn eof(&self) -> bool {
            self.eof_calls.set(self.eof_calls.get() + 1);
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
            kind: kind as u8,
            open_delimiter: open,
            close_delimiter: close,
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
    fn cached_lookahead_refreshes_only_after_advance_and_distinguishes_nul() {
        let mut inner = TestLexer::new("a\0b");
        {
            let mut lexer = CachedLexer::new(&mut inner);
            for _ in 0..4 {
                assert_eq!(lexer.lookahead(), i32::from(b'a'));
                assert!(!lexer.eof());
            }
            lexer.mark_end();
            lexer.set_result_symbol(STRING_CONTENT as u16);
            assert_eq!(lexer.lookahead(), i32::from(b'a'));
        }
        assert_eq!(inner.lookahead_calls.get(), 1);
        assert_eq!(inner.eof_calls.get(), 0);
        {
            let mut lexer = CachedLexer::new(&mut inner);
            lexer.advance(false);
            assert_eq!(lexer.lookahead(), 0);
            assert!(!lexer.eof()); // Embedded NUL is still input.
            lexer.advance(true);
            assert_eq!(lexer.lookahead(), i32::from(b'b'));
            assert!(!lexer.eof());
            lexer.advance(false);
            assert_eq!(lexer.lookahead(), 0);
            assert!(lexer.eof());
        }
        assert_eq!(inner.lookahead_calls.get(), 5);
        assert_eq!(inner.eof_calls.get(), 2);
    }

    #[test]
    fn whitespace_result_is_local_until_a_token_is_emitted() {
        let mut scanner = Scanner::default();
        let (ok, lexer) = scan(&mut scanner, " \tword", &[]);
        assert!(!ok);
        assert_eq!(lexer.position, 2);
        assert_eq!(lexer.symbol, u16::MAX);
        assert_eq!(
            lexer.calls,
            [Call::Advance(0, true), Call::Advance(1, true)]
        );

        let (ok, lexer) = scan(&mut scanner, " \n a", &[LINE_BREAK]);
        assert!(ok);
        assert_eq!(lexer.symbol, LINE_BREAK as u16);
        assert_eq!(lexer.position, 3);
        assert_eq!(lexer.end, Some(1));
        assert_eq!(
            lexer.calls,
            [
                Call::Advance(0, true),
                Call::MarkEnd(1),
                Call::Advance(1, false),
                Call::Advance(2, true),
                Call::Symbol(LINE_BREAK as u16),
            ]
        );
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
    fn compact_literals_preserve_all_snapshot_bytes_and_full_live_depth() {
        let mut scanner = Scanner::default();
        let mut buffer = [0; SERIALIZATION_BUFFER_SIZE];
        for byte in 0..=u8::MAX {
            let snapshot = [1, STRING_START as u8, byte, byte, byte, 1, 0];
            scanner.deserialize(&snapshot);
            assert_eq!(scanner.serialize(&mut buffer), snapshot.len());
            assert_eq!(&buffer[..snapshot.len()], &snapshot);
        }

        scanner.deserialize(&[1, STRING_START as u8, b'[', b']', 1, 1, 0]);
        let (ok, lexer) = scan(&mut scanner, &"[".repeat(300), &[STRING_CONTENT]);
        assert!(!ok);
        assert_eq!(lexer.position, 300);
        assert_eq!(scanner.literal_stack[0].nesting_depth, 301);
        assert_eq!(scanner.serialize(&mut buffer), 7);
        assert_eq!(&buffer[..7], &[1, STRING_START as u8, b'[', b']', 45, 1, 0]);
        scanner.deserialize(&buffer[..7]);
        assert_eq!(scanner.literal_stack[0].nesting_depth, 45);

        scanner.open_heredocs.push(heredoc(b"END", false, true));
        scanner.deserialize(&[0, 0]);
        assert!(scanner.literal_stack.is_empty());
        assert!(scanner.open_heredocs.is_empty());
        assert_eq!(scanner.serialize(&mut buffer), 2);
        assert_eq!(&buffer[..2], &[0, 0]);
    }

    #[test]
    fn single_literal_snapshot_fast_writer_matches_general_writer() {
        let mut scanner = Scanner::default();
        scanner.literal_stack.push(Literal::default());
        for byte in 0..=u8::MAX {
            for depth in [i32::MIN, -1, 0, 1, 255, 256, 257, i32::MAX] {
                scanner.literal_stack[0] = Literal {
                    kind: byte,
                    open_delimiter: byte.wrapping_add(1),
                    close_delimiter: byte.wrapping_add(2),
                    nesting_depth: depth,
                    allows_interpolation: byte % 2 == 0,
                };
                let mut fast = [0xab; 9];
                let mut general = fast;
                assert_eq!(scanner.serialize(&mut fast), 7);
                assert_eq!(scanner.serialize_nonempty(&mut general), 7);
                assert_eq!(fast, general);
                assert_eq!(&fast[7..], &[0xab; 2]);
                for length in 0..7 {
                    let mut short = [0xab; 7];
                    assert_eq!(scanner.serialize(&mut short[..length]), 0);
                    assert_eq!(short, [0xab; 7]);
                }
            }
        }
    }

    #[test]
    fn single_literal_snapshot_restore_discards_stale_state_and_reuses_capacity() {
        let mut fast = Scanner::default();
        fast.literal_stack.reserve(16);
        let capacity = fast.literal_stack.capacity();
        let mut general = Scanner::default();
        for byte in 0..=u8::MAX {
            fast.literal_stack.resize(3, Literal::default());
            fast.open_heredocs.push(heredoc(b"STALE", true, true));
            let snapshot = [
                1,
                byte,
                byte.wrapping_add(1),
                byte.wrapping_add(2),
                byte,
                byte,
                0,
            ];
            fast.deserialize(&snapshot);
            general.deserialize_nonempty(&snapshot);
            assert_eq!(fast.literal_stack, general.literal_stack);
            assert_eq!(fast.open_heredocs, general.open_heredocs);
            assert_eq!(fast.literal_stack.capacity(), capacity);
            let mut canonical = [0; 7];
            assert_eq!(fast.serialize(&mut canonical), 7);
            assert_eq!(&canonical[..5], &snapshot[..5]);
            assert_eq!(canonical[5], u8::from(byte != 0));
            assert_eq!(canonical[6], 0);
        }

        // With no existing entry, use the allocating/general restore path.
        fast.deserialize(&[]);
        fast.deserialize(&[1, STRING_START as u8, b'"', b'"', 255, 1, 0]);
        assert_eq!(fast.literal_stack.len(), 1);
        assert_eq!(fast.literal_stack[0].nesting_depth, 255);
        assert_eq!(fast.literal_stack.capacity(), capacity);

        // A seven-byte prefix is not sufficient: both header counts and the
        // full snapshot length must match before the fixed-size path is used.
        for snapshot in [
            &[1, 3, b'"', b'"', 1, 1, 1][..],
            &[2, 3, b'"', b'"', 1, 1, 0][..],
            &[1, 3, b'"', b'"', 1, 1, 0, 0][..],
        ] {
            fast.literal_stack.push(Literal::default());
            fast.deserialize(snapshot);
            assert!(fast.literal_stack.is_empty());
            assert!(fast.open_heredocs.is_empty());
        }
    }

    #[test]
    fn identifier_word_table_matches_c_locale_without_narrowing() {
        for codepoint in (-256..=1024).chain([i32::MIN, i32::MAX]) {
            assert_eq!(
                is_word_char(codepoint),
                is_alnum(codepoint) || codepoint == i32::from(b'_'),
                "{codepoint}"
            );
        }
    }

    #[test]
    fn serialization_layout_and_unsigned_byte_restoration() {
        let mut scanner = Scanner {
            literal_stack: vec![
                literal(STRING_START, b'{', b'}', true),
                Literal {
                    kind: STRING_ARRAY_START as u8,
                    open_delimiter: 0x28,
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

    // The C loop before splitting at its crossed_newline transition. Keep a
    // separate implementation to check callback order as well as token results.
    fn unsplit_whitespace(
        scanner: &mut Scanner,
        lexer: &mut CachedLexer<'_>,
        valid_symbols: &[bool; NONE],
    ) -> WhitespaceResult {
        let mut has_leading_whitespace = false;
        let heredoc_body_start_is_valid = !scanner.open_heredocs.is_empty()
            && !scanner.open_heredocs[0].started
            && valid_symbols[HEREDOC_BODY_START];
        let line_break_is_valid = !valid_symbols[NO_LINE_BREAK] && valid_symbols[LINE_BREAK];
        let mut crossed_newline = false;
        loop {
            if line_break_is_valid && lexer.is_at_included_range_start() {
                lexer.mark_end();
                return WhitespaceResult::LineBreak;
            }
            let lookahead = lexer.lookahead();
            match lookahead {
                0x20 | 0x09 => {
                    has_leading_whitespace = true;
                    lexer.advance(true);
                } // ' ', '\t'
                0x0d => {
                    // '\r'
                    if heredoc_body_start_is_valid {
                        scanner.open_heredocs[0].started = true;
                        return WhitespaceResult::HeredocStart;
                    }
                    {
                        has_leading_whitespace = true;
                        lexer.advance(true);
                    };
                }
                0x0a => {
                    // '\n'
                    if heredoc_body_start_is_valid {
                        scanner.open_heredocs[0].started = true;
                        return WhitespaceResult::HeredocStart;
                    } else if line_break_is_valid && !crossed_newline {
                        lexer.mark_end();
                        advance(lexer);
                        crossed_newline = true;
                    } else {
                        {
                            has_leading_whitespace = true;
                            lexer.advance(true);
                        };
                    }
                }
                0x5c => {
                    // '\\'
                    advance(lexer);
                    if lexer.lookahead() == i32::from(b'\r') {
                        lexer.advance(true);
                    }
                    if is_space(lexer.lookahead()) {
                        {
                            has_leading_whitespace = true;
                            lexer.advance(true);
                        };
                    } else {
                        return WhitespaceResult::Failed;
                    }
                }
                _ => {
                    if crossed_newline {
                        if lookahead != i32::from(b'.')
                            && lookahead != i32::from(b'&')
                            && lookahead != i32::from(b'#')
                        {
                            return WhitespaceResult::LineBreak;
                        } else if lookahead == i32::from(b'.') {
                            // A call operator suppresses the break; a range does not.
                            advance(lexer);
                            if !lexer.eof() && lexer.lookahead() == i32::from(b'.') {
                                return WhitespaceResult::LineBreak;
                            } else {
                                return WhitespaceResult::Failed;
                            }
                        }
                    }
                    return WhitespaceResult::Skipped(has_leading_whitespace);
                }
            }
        }
    }

    #[test]
    fn whitespace_phases_preserve_unsplit_c_control_flow() {
        let cases = [
            "",
            " ",
            "\t\t",
            "\r",
            "\n",
            "\r\n\n",
            "\\\r x",
            "\\\r\nx",
            "\\\nx",
            "\n\\\r\n&x",
            "\n .x",
            "\n ..x",
            "\n ...x",
            "\n&x",
            "\n#x",
            "\n\\x",
            "\n\0",
            "\n\u{b}",
            "\n\\\u{b} ",
            " \t\n\r &x",
            " \n\\\r\u{c}#x",
        ];
        for source in cases {
            for flags in 0..8 {
                let mut valid = [false; NONE];
                valid[LINE_BREAK] = flags & 1 != 0;
                valid[NO_LINE_BREAK] = flags & 2 != 0;
                valid[HEREDOC_BODY_START] = flags & 4 != 0;
                for heredoc_state in 0..3 {
                    for boundary in 0..=source.chars().count() + 1 {
                        let mut fast = Scanner::default();
                        let mut reference = Scanner::default();
                        if heredoc_state != 0 {
                            let make_heredoc = || Heredoc {
                                word: b"END".to_vec(),
                                started: heredoc_state == 2,
                                ..Heredoc::default()
                            };
                            fast.open_heredocs.push(make_heredoc());
                            reference.open_heredocs.push(make_heredoc());
                        }
                        let mut actual = TestLexer::new(source);
                        let mut expected = TestLexer::new(source);
                        actual.range_starts.push(boundary);
                        expected.range_starts.push(boundary);
                        assert_eq!(
                            fast.scan_whitespace(&mut CachedLexer::new(&mut actual), &valid),
                            unsplit_whitespace(
                                &mut reference,
                                &mut CachedLexer::new(&mut expected),
                                &valid,
                            ),
                            "source={source:?} flags={flags} heredoc={heredoc_state} boundary={boundary}",
                        );
                        assert_eq!(actual.position, expected.position);
                        assert_eq!(actual.end, expected.end);
                        assert_eq!(actual.calls, expected.calls);
                        assert_eq!(actual.range_calls, expected.range_calls);
                        assert_eq!(actual.lookahead_calls, expected.lookahead_calls);
                        assert_eq!(actual.eof_calls, expected.eof_calls);
                        assert_eq!(fast.open_heredocs, reference.open_heredocs);
                    }
                }
            }
        }
    }

    #[test]
    fn identifier_runs_synchronize_lookahead_before_suffix_dispatch() {
        for (source, token, position, end) in [
            ("_abc123: x", Some(HASH_KEY_SYMBOL), 8, Some(7)),
            ("a0::B", None, 3, Some(2)),
            ("name! ", Some(IDENTIFIER_SUFFIX), 5, None),
            ("Name! ", Some(CONSTANT_SUFFIX), 5, None),
            ("name!=x", None, 5, None),
            ("name\0!", None, 4, None),
            ("nameé!", None, 4, None),
            ("name", None, 4, None),
        ] {
            let mut scanner = Scanner::default();
            let mut inner = TestLexer::new(source);
            {
                let mut lexer = CachedLexer::new(&mut inner);
                assert_eq!(
                    scanner.scan_cached(
                        &mut lexer,
                        &symbols(&[HASH_KEY_SYMBOL, IDENTIFIER_SUFFIX, CONSTANT_SUFFIX]),
                    ),
                    token.is_some(),
                    "{source:?}",
                );
                assert_eq!(
                    lexer.lookahead(),
                    source.chars().nth(position).map_or(0, |c| c as i32),
                    "{source:?}",
                );
            }
            assert_eq!(inner.position, position, "{source:?}");
            assert_eq!(inner.end, end, "{source:?}");
            assert_eq!(inner.lookahead_calls.get(), position + 1, "{source:?}");
            assert_eq!(inner.symbol, token.map_or(u16::MAX, |t| t as u16));
            // Failed attempts must retain C's full lookahead distance too.
            let advances: Vec<_> = inner
                .calls
                .iter()
                .filter_map(|call| match call {
                    Call::Advance(at, skip) => Some((*at, *skip)),
                    _ => None,
                })
                .collect();
            assert_eq!(
                advances,
                (0..position).map(|i| (i, false)).collect::<Vec<_>>()
            );
        }
    }

    #[test]
    fn identifier_kinds_match_c_locale_start_and_suffix_rules() {
        for codepoint in (-256..=1024).chain([i32::MIN, i32::MAX]) {
            let expected = match codepoint {
                0x30..=0x39 => 1,
                0x61..=0x7a | 0x5f => 2,
                0x41..=0x5a => 3,
                _ => 0,
            };
            assert_eq!(word_char_kind(codepoint), expected, "{codepoint}");
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
            [Call::MarkEnd(0), Call::Symbol(LINE_BREAK as u16),]
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
        assert!(scan_short_interpolation(
            &mut CachedLexer::new(&mut lexer),
            false,
            STRING_CONTENT
        ));
        assert_eq!(usize::from(lexer.symbol), SHORT_INTERPOLATION);
        assert_eq!(lexer.end, Some(0));
        let mut lexer = TestLexer::new("$İ"); // low byte is '0'
        assert!(!scan_short_interpolation(
            &mut CachedLexer::new(&mut lexer),
            false,
            STRING_CONTENT
        ));
        let mut lexer = TestLexer::new("$ā"); // low byte is 1, not punctuation
        assert!(!scan_short_interpolation(
            &mut CachedLexer::new(&mut lexer),
            false,
            STRING_CONTENT
        ));
        let mut lexer = TestLexer::new("$Ā"); // strchr matches low-byte NUL
        assert!(scan_short_interpolation(
            &mut CachedLexer::new(&mut lexer),
            false,
            STRING_CONTENT
        ));
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
        assert_eq!(lexer.calls, [Call::Symbol(HEREDOC_BODY_START as u16),]);
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
