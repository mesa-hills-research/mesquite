//! Bash's external scanner, translated from `src/scanner.c`.
//!
//! The heredoc strings are byte arrays, not UTF-8: the C scanner truncates each
//! lookahead to `char`. State restoration intentionally neither truncates the
//! heredoc stack nor resets the glob fields when given an empty snapshot.

use tree_sitter_language::{ExternalScanner, Lexer, SERIALIZATION_BUFFER_SIZE};

const HEREDOC_START: usize = 0;
const SIMPLE_HEREDOC_BODY: usize = 1;
const HEREDOC_BODY_BEGINNING: usize = 2;
const HEREDOC_CONTENT: usize = 3;
const HEREDOC_END: usize = 4;
const FILE_DESCRIPTOR: usize = 5;
const EMPTY_VALUE: usize = 6;
const CONCAT: usize = 7;
const VARIABLE_NAME: usize = 8;
const TEST_OPERATOR: usize = 9;
const REGEX: usize = 10;
const REGEX_NO_SLASH: usize = 11;
const REGEX_NO_SPACE: usize = 12;
const EXPANSION_WORD: usize = 13;
const EXTGLOB_PATTERN: usize = 14;
const BARE_DOLLAR: usize = 15;
const BRACE_START: usize = 16;
const IMMEDIATE_DOUBLE_HASH: usize = 17;
const EXTERNAL_EXPANSION_SYM_HASH: usize = 18;
const EXTERNAL_EXPANSION_SYM_BANG: usize = 19;
const EXTERNAL_EXPANSION_SYM_EQUAL: usize = 20;
const CLOSING_BRACE: usize = 21;
const CLOSING_BRACKET: usize = 22;
const HEREDOC_ARROW: usize = 23;
const HEREDOC_ARROW_DASH: usize = 24;
const NEWLINE: usize = 25;
const OPENING_PAREN: usize = 26;
// ESAC = 27 is not used by the scanner.
const ERROR_RECOVERY: usize = 28;

type ValidSymbols = [bool; ERROR_RECOVERY + 1];
const SPACE: u8 = 1;
const ALPHA: u8 = 2;
const DIGIT: u8 = 4;
const CONCAT_END: u8 = 8;
const IDENTIFIER: u8 = 16;
const SPECIAL_VARIABLE: u8 = 32;
const TEST_PUNCTUATION: u8 = 64;

// C-locale classes shared by the token tests at each input position.
fn character_class(c: i32) -> u8 {
    const CLASSES: [u8; 128] = {
        let mut classes = [0; 128];
        let mut c = 0;
        while c < classes.len() {
            if matches!(c, 0x09..=0x0d | 0x20) {
                classes[c] |= SPACE | CONCAT_END;
            }
            if matches!(c, 0x41..=0x5a | 0x61..=0x7a) {
                classes[c] |= ALPHA | IDENTIFIER;
            }
            if matches!(c, 0x30..=0x39) {
                classes[c] |= DIGIT | IDENTIFIER;
            }
            if matches!(c, 0 | 0x3e | 0x3c | 0x29 | 0x28 | 0x3b | 0x26 | 0x7c) {
                classes[c] |= CONCAT_END;
            }
            if c == 0x5f {
                classes[c] |= IDENTIFIER;
            }
            if matches!(c, 0x2a | 0x40 | 0x3f | 0x2d | 0x30 | 0x5f) {
                classes[c] |= SPECIAL_VARIABLE;
            }
            if matches!(c, 0x5c | 0x2d | 0x0a | 0x24) {
                classes[c] |= TEST_PUNCTUATION;
            }
            c += 1;
        }
        classes
    };
    CLASSES.get(c as usize).copied().unwrap_or(0)
}

/// `lookahead` is a field in C, but a virtual call through the Rust lexer.
/// Keep a scan-local copy, refreshed only when the input position advances, so
/// the many token/character checks at each position are plain integer reads.
/// All position-changing operations go through this wrapper; token boundaries,
/// columns, and EOF queries still use the original lexer callbacks.
struct ScannerLexer<'a> {
    inner: &'a mut dyn Lexer,
    lookahead: i32,
    class: u8,
}

impl<'a> ScannerLexer<'a> {
    fn new(inner: &'a mut dyn Lexer) -> Self {
        let lookahead = inner.lookahead();
        Self {
            inner,
            lookahead,
            class: character_class(lookahead),
        }
    }

    fn lookahead(&self) -> i32 {
        self.lookahead
    }

    fn advance(&mut self, skip: bool) {
        self.inner.advance(skip);
        self.lookahead = self.inner.lookahead();
        self.class = character_class(self.lookahead);
    }

    fn is_space(&self) -> bool {
        self.class & SPACE != 0
    }
    fn is_alpha(&self) -> bool {
        self.class & ALPHA != 0
    }
    fn is_digit(&self) -> bool {
        self.class & DIGIT != 0
    }
    fn is_alnum(&self) -> bool {
        self.class & (ALPHA | DIGIT) != 0
    }

    // Called after skip_whitespace::<false>. Other characters, including a
    // newline that the grammar wants to keep, do nothing in the test block.
    #[inline(always)]
    fn starts_test_punctuation(&self, valid: &ValidSymbols) -> bool {
        self.class & TEST_PUNCTUATION != 0
            && (self.lookahead != 0x0a || !valid[NEWLINE])
            && (self.lookahead != 0x24 || (valid[BARE_DOLLAR] && !valid[ERROR_RECOVERY]))
    }

    // Commit the cache once per unconditional run, not once per character.
    fn skip_whitespace<const NEWLINES: bool>(&mut self) {
        if !self.is_space() || (!NEWLINES && self.lookahead == 0x0a) {
            return;
        }
        let c = loop {
            self.inner.advance(true);
            let c = self.inner.lookahead();
            if !matches!(c, 0x09..=0x0d | 0x20) || (!NEWLINES && c == 0x0a) {
                break c;
            }
        };
        self.lookahead = c;
        self.class = character_class(c);
    }

    // Most attempts begin with a name, not a number. Keep numeric-prefix
    // handling out of this loop so it does not carry numeric state or the
    // digit-loop setup through every ordinary identifier character.
    #[inline(always)]
    fn scan_identifier(&mut self) -> Option<bool> {
        let mut c;
        let mut class = self.class;
        if class & IDENTIFIER == 0 {
            return None;
        }
        if class & DIGIT != 0 {
            return Some(self.scan_numeric_identifier());
        }
        loop {
            self.inner.advance(false);
            c = self.inner.lookahead();
            class = character_class(c);
            if class & IDENTIFIER == 0 {
                break;
            }
        }
        self.lookahead = c;
        self.class = class;
        Some(false)
    }

    // After the numeric prefix, later digits cannot make a name numeric again.
    #[cold]
    #[inline(never)]
    fn scan_numeric_identifier(&mut self) -> bool {
        let mut c = self.lookahead;
        let mut class = self.class;
        while class & DIGIT != 0 {
            self.inner.advance(false);
            c = self.inner.lookahead();
            class = character_class(c);
        }
        let is_number = class & IDENTIFIER == 0;
        while class & IDENTIFIER != 0 {
            self.inner.advance(false);
            c = self.inner.lookahead();
            class = character_class(c);
        }
        self.lookahead = c;
        self.class = class;
        is_number
    }

    fn mark_end(&mut self) {
        self.inner.mark_end();
    }

    fn set_result_symbol(&mut self, symbol: u16) {
        self.inner.set_result_symbol(symbol);
    }

    fn result_symbol(&self) -> u16 {
        self.inner.result_symbol()
    }

    fn get_column(&mut self) -> u32 {
        self.inner.get_column()
    }

    fn eof(&self) -> bool {
        // Nonzero lookahead rules out EOF; zero still needs the NUL/EOF query.
        self.lookahead == 0 && self.inner.eof()
    }
}

// Character classes as in C's default locale, not Unicode's.
#[cfg(test)]
fn is_space(c: i32) -> bool {
    matches!(c, 0x09..=0x0d | 0x20)
}

#[cfg(test)]
fn is_alpha(c: i32) -> bool {
    matches!(c, 0x41..=0x5a | 0x61..=0x7a)
}

#[cfg(test)]
fn is_digit(c: i32) -> bool {
    matches!(c, 0x30..=0x39)
}

fn advance(lexer: &mut ScannerLexer<'_>) {
    lexer.advance(false);
}

fn skip(lexer: &mut ScannerLexer<'_>) {
    lexer.advance(true);
}

#[derive(Default)]
struct Heredoc {
    is_raw: bool,
    started: bool,
    allows_indent: bool,
    delimiter: Vec<u8>,
    current_leading_word: Vec<u8>,
}

impl Heredoc {
    fn reset(&mut self) {
        self.is_raw = false;
        self.started = false;
        self.allows_indent = false;
        self.delimiter.clear();
        // C leaves current_leading_word alone; the next identifier scan clears it.
    }

    fn scan_start(&mut self, lexer: &mut ScannerLexer<'_>) -> bool {
        lexer.skip_whitespace::<true>();
        lexer.set_result_symbol(HEREDOC_START as u16);
        self.is_raw = matches!(lexer.lookahead(), 0x27 | 0x22 | 0x5c);
        let found_delimiter = advance_word(lexer, &mut self.delimiter);
        if !found_delimiter {
            self.delimiter.clear();
            return false;
        }
        found_delimiter
    }

    fn scan_end_identifier(&mut self, lexer: &mut ScannerLexer<'_>) -> bool {
        self.current_leading_word.clear();
        let mut size = 0;
        if !self.delimiter.is_empty() {
            while lexer.lookahead() != 0
                && lexer.lookahead() != i32::from(b'\n')
                && size < self.delimiter.len()
                // As C with a signed `char` (x86-64), including after the
                // truncation performed by advance_word.
                && i32::from(self.delimiter[size] as i8) == lexer.lookahead()
            {
                self.current_leading_word.push(lexer.lookahead() as u8);
                advance(lexer);
                size += 1;
            }
        }
        self.current_leading_word.push(0);
        !self.delimiter.is_empty()
            && self.current_leading_word.split(|&c| c == 0).next()
                == self.delimiter.split(|&c| c == 0).next()
    }
}

/// Consume a POSIX word, approximately, returning its unquoted bytes.
fn advance_word(lexer: &mut ScannerLexer<'_>, unquoted_word: &mut Vec<u8>) -> bool {
    let mut empty = true;
    let mut quote = 0;
    if matches!(lexer.lookahead(), 0x27 | 0x22) {
        quote = lexer.lookahead();
        advance(lexer);
    }
    while lexer.lookahead() != 0
        && !(if quote != 0 {
            lexer.lookahead() == quote || matches!(lexer.lookahead(), 0x0d | 0x0a)
        } else {
            lexer.is_space()
        })
    {
        if lexer.lookahead() == i32::from(b'\\') {
            advance(lexer);
            if lexer.lookahead() == 0 {
                return false;
            }
        }
        empty = false;
        unquoted_word.push(lexer.lookahead() as u8);
        advance(lexer);
    }
    unquoted_word.push(0);
    if quote != 0 && lexer.lookahead() == quote {
        advance(lexer);
    }
    !empty
}

fn scan_bare_dollar(lexer: &mut ScannerLexer<'_>) -> bool {
    while lexer.is_space() && lexer.lookahead() != i32::from(b'\n') && !lexer.eof() {
        skip(lexer);
    }
    if lexer.lookahead() == i32::from(b'$') {
        advance(lexer);
        lexer.set_result_symbol(BARE_DOLLAR as u16);
        lexer.mark_end();
        return lexer.is_space() || lexer.eof() || lexer.lookahead() == i32::from(b'"');
    }
    false
}

#[derive(Default)]
pub(crate) struct Scanner {
    last_glob_paren_depth: u8,
    ext_was_in_double_quote: bool,
    ext_saw_outside_quote: bool,
    heredocs: Vec<Heredoc>,
}

impl Scanner {
    #[cold]
    fn push_heredoc(&mut self, allows_indent: bool) {
        self.heredocs.push(Heredoc {
            allows_indent,
            ..Heredoc::default()
        });
    }

    #[cold]
    fn pop_heredoc(&mut self) {
        self.heredocs.pop();
    }

    #[cold]
    fn scan_heredoc_content(
        &mut self,
        lexer: &mut ScannerLexer<'_>,
        middle_type: usize,
        end_type: usize,
    ) -> bool {
        let mut did_advance = false;
        let heredoc = self.heredocs.last_mut().unwrap();
        loop {
            match lexer.lookahead() {
                0 => {
                    if lexer.eof() && did_advance {
                        heredoc.reset();
                        lexer.set_result_symbol(end_type as u16);
                        return true;
                    }
                    return false;
                }
                0x5c => {
                    did_advance = true;
                    advance(lexer);
                    advance(lexer);
                }
                0x24 => {
                    if heredoc.is_raw {
                        did_advance = true;
                        advance(lexer);
                        continue;
                    }
                    if did_advance {
                        lexer.mark_end();
                        lexer.set_result_symbol(middle_type as u16);
                        heredoc.started = true;
                        advance(lexer);
                        if lexer.is_alpha() || matches!(lexer.lookahead(), 0x7b | 0x28) {
                            return true;
                        }
                        continue;
                    }
                    if middle_type == HEREDOC_BODY_BEGINNING && lexer.get_column() == 0 {
                        lexer.set_result_symbol(middle_type as u16);
                        heredoc.started = true;
                        return true;
                    }
                    return false;
                }
                0x0a => {
                    if !did_advance {
                        skip(lexer);
                    } else {
                        advance(lexer);
                    }
                    did_advance = true;
                    if heredoc.allows_indent {
                        while lexer.is_space() {
                            advance(lexer);
                        }
                    }
                    lexer.set_result_symbol(if heredoc.started {
                        middle_type
                    } else {
                        end_type
                    } as u16);
                    lexer.mark_end();
                    if heredoc.scan_end_identifier(lexer) {
                        if lexer.result_symbol() == HEREDOC_END as u16 {
                            self.pop_heredoc();
                        }
                        return true;
                    }
                }
                _ => {
                    if lexer.get_column() == 0 {
                        while lexer.is_space() {
                            if did_advance {
                                advance(lexer);
                            } else {
                                skip(lexer);
                            }
                        }
                        if end_type != SIMPLE_HEREDOC_BODY {
                            lexer.set_result_symbol(middle_type as u16);
                            if heredoc.scan_end_identifier(lexer) {
                                return true;
                            }
                        }
                        if end_type == SIMPLE_HEREDOC_BODY {
                            lexer.set_result_symbol(end_type as u16);
                            lexer.mark_end();
                            if heredoc.scan_end_identifier(lexer) {
                                return true;
                            }
                        }
                    }
                    did_advance = true;
                    advance(lexer);
                }
            }
        }
    }
}

impl Scanner {
    // The concat-only grammar state can return immediately after the first
    // branch. Specializing this same implementation avoids both the full scan
    // frame and the remaining token tests, without duplicating C control flow.
    #[inline(never)]
    fn scan_inner<const ONLY_CONCAT: bool>(
        &mut self,
        lexer: &mut dyn Lexer,
        valid: &ValidSymbols,
    ) -> bool {
        let valid = if ONLY_CONCAT {
            &const { concat_symbols() }
        } else {
            valid
        };
        let lexer = &mut ScannerLexer::new(lexer);
        if valid[CONCAT] && !valid[ERROR_RECOVERY] {
            if !(lexer.class & CONCAT_END != 0
                || (lexer.lookahead() == i32::from(b'}') && valid[CLOSING_BRACE])
                || (lexer.lookahead() == i32::from(b']') && valid[CLOSING_BRACKET]))
            {
                lexer.set_result_symbol(CONCAT as u16);
                // a`b` concatenates if whitespace follows the second backtick.
                if lexer.lookahead() == i32::from(b'`') {
                    lexer.mark_end();
                    advance(lexer);
                    while lexer.lookahead() != i32::from(b'`') && !lexer.eof() {
                        advance(lexer);
                    }
                    if lexer.eof() {
                        return false;
                    }
                    if lexer.lookahead() == i32::from(b'`') {
                        advance(lexer);
                    }
                    return lexer.is_space() || lexer.eof();
                }
                if lexer.lookahead() == i32::from(b'\\') {
                    lexer.mark_end();
                    advance(lexer);
                    if matches!(lexer.lookahead(), 0x22 | 0x27 | 0x5c) {
                        return true;
                    }
                    if lexer.eof() {
                        return false;
                    }
                } else {
                    return true;
                }
            }
            if lexer.is_space() && valid[CLOSING_BRACE] && !valid[EXPANSION_WORD] {
                lexer.set_result_symbol(CONCAT as u16);
                return true;
            }
        }

        if ONLY_CONCAT {
            return false;
        }

        if valid[IMMEDIATE_DOUBLE_HASH]
            && !valid[ERROR_RECOVERY]
            && lexer.lookahead() == i32::from(b'#')
        {
            lexer.mark_end();
            advance(lexer);
            if lexer.lookahead() == i32::from(b'#') {
                advance(lexer);
                if lexer.lookahead() != i32::from(b'}') {
                    lexer.set_result_symbol(IMMEDIATE_DOUBLE_HASH as u16);
                    lexer.mark_end();
                    return true;
                }
            }
        }

        if valid[EXTERNAL_EXPANSION_SYM_HASH]
            && !valid[ERROR_RECOVERY]
            && matches!(lexer.lookahead(), 0x23 | 0x3d | 0x21)
        {
            lexer.set_result_symbol(match lexer.lookahead() {
                0x23 => EXTERNAL_EXPANSION_SYM_HASH,
                0x21 => EXTERNAL_EXPANSION_SYM_BANG,
                _ => EXTERNAL_EXPANSION_SYM_EQUAL,
            } as u16);
            advance(lexer);
            lexer.mark_end();
            while matches!(lexer.lookahead(), 0x23 | 0x3d | 0x21) {
                advance(lexer);
            }
            lexer.skip_whitespace::<true>();
            return lexer.lookahead() == i32::from(b'}');
        }

        if valid[EMPTY_VALUE]
            && (lexer.is_space() || lexer.eof() || matches!(lexer.lookahead(), 0x3b | 0x26))
        {
            lexer.set_result_symbol(EMPTY_VALUE as u16);
            return true;
        }

        if (valid[HEREDOC_BODY_BEGINNING] || valid[SIMPLE_HEREDOC_BODY])
            && self.heredocs.last().is_some_and(|h| !h.started)
            && !valid[ERROR_RECOVERY]
        {
            return self.scan_heredoc_content(lexer, HEREDOC_BODY_BEGINNING, SIMPLE_HEREDOC_BODY);
        }
        if valid[HEREDOC_END]
            && let Some(heredoc) = self.heredocs.last_mut()
            && heredoc.scan_end_identifier(lexer)
        {
            self.pop_heredoc();
            lexer.set_result_symbol(HEREDOC_END as u16);
            return true;
        }
        if valid[HEREDOC_CONTENT]
            && self.heredocs.last().is_some_and(|h| h.started)
            && !valid[ERROR_RECOVERY]
        {
            return self.scan_heredoc_content(lexer, HEREDOC_CONTENT, HEREDOC_END);
        }
        if valid[HEREDOC_START]
            && !valid[ERROR_RECOVERY]
            && let Some(heredoc) = self.heredocs.last_mut()
        {
            return heredoc.scan_start(lexer);
        }

        if valid[TEST_OPERATOR] && !valid[EXPANSION_WORD] {
            lexer.skip_whitespace::<false>();
            // After non-newline whitespace, only these four punctuation
            // cases can advance or emit a token. All other characters are
            // a no-op in the test-operator block.
            if lexer.starts_test_punctuation(valid)
                && let Some(result) = self.scan_test_punctuation(lexer, valid)
            {
                return result;
            }
        }

        if (valid[VARIABLE_NAME] || valid[FILE_DESCRIPTOR] || valid[HEREDOC_ARROW])
            && !valid[REGEX_NO_SLASH]
            && !valid[ERROR_RECOVERY]
        {
            loop {
                if (matches!(lexer.lookahead(), 0x20 | 0x09 | 0x0d)
                    || (lexer.lookahead() == i32::from(b'\n') && !valid[NEWLINE]))
                    && !valid[EXPANSION_WORD]
                {
                    skip(lexer);
                } else if lexer.lookahead() == i32::from(b'\\') {
                    skip(lexer);
                    if lexer.eof() {
                        lexer.mark_end();
                        lexer.set_result_symbol(VARIABLE_NAME as u16);
                        return true;
                    }
                    if lexer.lookahead() == i32::from(b'\r') {
                        skip(lexer);
                    }
                    if lexer.lookahead() == i32::from(b'\n') {
                        skip(lexer);
                    } else {
                        if lexer.lookahead() == i32::from(b'\\') && valid[EXPANSION_WORD] {
                            return self.scan_expansion_word(lexer, valid);
                        }
                        return false;
                    }
                } else {
                    break;
                }
            }

            if !valid[EXPANSION_WORD] && lexer.class & SPECIAL_VARIABLE != 0 {
                lexer.mark_end();
                advance(lexer);
                if matches!(
                    lexer.lookahead(),
                    0x3d | 0x5b | 0x3a | 0x2d | 0x25 | 0x23 | 0x2f
                ) {
                    return false;
                }
                if valid[EXTGLOB_PATTERN] && lexer.is_space() {
                    lexer.mark_end();
                    lexer.set_result_symbol(EXTGLOB_PATTERN as u16);
                    return true;
                }
            }

            if valid[HEREDOC_ARROW] && lexer.lookahead() == i32::from(b'<') {
                advance(lexer);
                if lexer.lookahead() == i32::from(b'<') {
                    advance(lexer);
                    if lexer.lookahead() == i32::from(b'-') {
                        advance(lexer);
                        self.push_heredoc(true);
                        lexer.set_result_symbol(HEREDOC_ARROW_DASH as u16);
                    } else if matches!(lexer.lookahead(), 0x3c | 0x3d) {
                        return false;
                    } else {
                        self.push_heredoc(false);
                        lexer.set_result_symbol(HEREDOC_ARROW as u16);
                    }
                    return true;
                }
                return false;
            }

            let Some(is_number) = lexer.scan_identifier() else {
                if lexer.lookahead() == i32::from(b'{') {
                    return scan_brace_start(lexer, valid);
                }
                if valid[EXPANSION_WORD] {
                    return self.scan_expansion_word(lexer, valid);
                }
                if valid[EXTGLOB_PATTERN] {
                    return self.scan_extglob_pattern(lexer, valid);
                }
                return false;
            };
            if is_number && valid[FILE_DESCRIPTOR] && matches!(lexer.lookahead(), 0x3e | 0x3c) {
                lexer.set_result_symbol(FILE_DESCRIPTOR as u16);
                return true;
            }
            if valid[VARIABLE_NAME] {
                if lexer.lookahead() == i32::from(b'+') {
                    lexer.mark_end();
                    advance(lexer);
                    if matches!(lexer.lookahead(), 0x3d | 0x3a) || valid[CLOSING_BRACE] {
                        lexer.set_result_symbol(VARIABLE_NAME as u16);
                        return true;
                    }
                    return false;
                }
                if lexer.lookahead() == i32::from(b'/') {
                    return false;
                }
                if matches!(lexer.lookahead(), 0x3d | 0x5b)
                    || (lexer.lookahead() == i32::from(b':')
                        && !valid[CLOSING_BRACE]
                        && !valid[OPENING_PAREN])
                    || lexer.lookahead() == i32::from(b'%')
                    || (lexer.lookahead() == i32::from(b'#') && !is_number)
                    || lexer.lookahead() == i32::from(b'@')
                    || (lexer.lookahead() == i32::from(b'-') && valid[CLOSING_BRACE])
                {
                    lexer.mark_end();
                    lexer.set_result_symbol(VARIABLE_NAME as u16);
                    return true;
                }
                if lexer.lookahead() == i32::from(b'?') {
                    lexer.mark_end();
                    advance(lexer);
                    lexer.set_result_symbol(VARIABLE_NAME as u16);
                    return lexer.is_alpha();
                }
            }
            return false;
        }

        if valid[BARE_DOLLAR] && !valid[ERROR_RECOVERY] && scan_bare_dollar(lexer) {
            return true;
        }
        self.scan_regex(lexer, valid)
    }
}

impl Scanner {
    // None falls through to the following token handlers, including after
    // speculative advances. Some(false) must still stop the entire scan.
    #[cold]
    fn scan_test_punctuation(
        &mut self,
        lexer: &mut ScannerLexer<'_>,
        valid: &ValidSymbols,
    ) -> Option<bool> {
        if lexer.lookahead() == i32::from(b'\\') {
            if valid[EXTGLOB_PATTERN] {
                return Some(self.scan_extglob_pattern(lexer, valid));
            }
            if valid[REGEX_NO_SPACE] {
                return Some(self.scan_regex(lexer, valid));
            }
            skip(lexer);
            if lexer.eof() {
                return Some(false);
            }
            if lexer.lookahead() == i32::from(b'\r') {
                skip(lexer);
                if lexer.lookahead() == i32::from(b'\n') {
                    skip(lexer);
                }
            } else if lexer.lookahead() == i32::from(b'\n') {
                skip(lexer);
            } else {
                return Some(false);
            }
            lexer.skip_whitespace::<true>();
        }
        if lexer.lookahead() == i32::from(b'\n') && !valid[NEWLINE] {
            skip(lexer);
            lexer.skip_whitespace::<true>();
        }
        if lexer.lookahead() == i32::from(b'-') {
            advance(lexer);
            let mut advanced_once = false;
            while lexer.is_alpha() {
                advanced_once = true;
                advance(lexer);
            }
            if lexer.is_space() && advanced_once {
                lexer.mark_end();
                advance(lexer);
                if lexer.lookahead() == i32::from(b'}') && valid[CLOSING_BRACE] {
                    if valid[EXPANSION_WORD] {
                        lexer.mark_end();
                        lexer.set_result_symbol(EXPANSION_WORD as u16);
                        return Some(true);
                    }
                    return Some(false);
                }
                lexer.set_result_symbol(TEST_OPERATOR as u16);
                return Some(true);
            }
            if lexer.is_space() && valid[EXTGLOB_PATTERN] {
                lexer.set_result_symbol(EXTGLOB_PATTERN as u16);
                return Some(true);
            }
        }
        if valid[BARE_DOLLAR] && !valid[ERROR_RECOVERY] && scan_bare_dollar(lexer) {
            return Some(true);
        }
        None
    }
}

const fn concat_symbols() -> ValidSymbols {
    let mut symbols = [false; ERROR_RECOVERY + 1];
    symbols[CONCAT] = true;
    symbols
}

impl ExternalScanner for Scanner {
    fn scan(&mut self, lexer: &mut dyn Lexer, valid: &[bool]) -> bool {
        let valid: &ValidSymbols = valid.first_chunk().unwrap();
        if valid[CONCAT] && !valid[TEST_OPERATOR] && valid == &const { concat_symbols() } {
            self.scan_inner::<true>(lexer, valid)
        } else {
            self.scan_inner::<false>(lexer, valid)
        }
    }

    fn serialize(&mut self, buffer: &mut [u8]) -> usize {
        buffer[..4].copy_from_slice(&[
            self.last_glob_paren_depth,
            u8::from(self.ext_was_in_double_quote),
            u8::from(self.ext_saw_outside_quote),
            self.heredocs.len() as u8,
        ]);
        let mut size = 4;
        for heredoc in &self.heredocs {
            if size + 3 + 4 + heredoc.delimiter.len() >= SERIALIZATION_BUFFER_SIZE {
                return 0;
            }
            buffer[size] = u8::from(heredoc.is_raw);
            buffer[size + 1] = u8::from(heredoc.started);
            buffer[size + 2] = u8::from(heredoc.allows_indent);
            size += 3;
            buffer[size..size + 4].copy_from_slice(&(heredoc.delimiter.len() as u32).to_ne_bytes());
            size += 4;
            buffer[size..size + heredoc.delimiter.len()].copy_from_slice(&heredoc.delimiter);
            size += heredoc.delimiter.len();
        }
        size
    }

    // Header-only snapshots are common; keep the stack restoration outlined.
    fn deserialize(&mut self, buffer: &[u8]) {
        if let [depth, in_quote, outside_quote, 0] = buffer {
            self.last_glob_paren_depth = *depth;
            self.ext_was_in_double_quote = *in_quote != 0;
            self.ext_saw_outside_quote = *outside_quote != 0;
        } else if buffer.is_empty() && self.heredocs.is_empty() {
            // C leaves the glob fields alone on an empty snapshot.
        } else {
            self.deserialize_heredocs(buffer);
        }
    }
}

impl Scanner {
    #[inline(never)]
    #[cold]
    fn deserialize_heredocs(&mut self, buffer: &[u8]) {
        if buffer.is_empty() {
            for heredoc in &mut self.heredocs {
                heredoc.reset();
            }
        } else {
            self.last_glob_paren_depth = buffer[0];
            self.ext_was_in_double_quote = buffer[1] != 0;
            self.ext_saw_outside_quote = buffer[2] != 0;
            let count = usize::from(buffer[3]);
            let mut size = 4;
            for i in 0..count {
                if i >= self.heredocs.len() {
                    self.heredocs.push(Heredoc::default());
                }
                let heredoc = &mut self.heredocs[i];
                heredoc.is_raw = buffer[size] != 0;
                heredoc.started = buffer[size + 1] != 0;
                heredoc.allows_indent = buffer[size + 2] != 0;
                size += 3;
                let length =
                    u32::from_ne_bytes(buffer[size..size + 4].try_into().unwrap()) as usize;
                size += 4;
                heredoc.delimiter.clear();
                heredoc
                    .delimiter
                    .extend_from_slice(&buffer[size..size + length]);
                size += length;
            }
            assert_eq!(size, buffer.len());
        }
    }
}

impl Scanner {
    // The C labels regex, extglob_pattern, expansion_word and brace_start are
    // forward-only jumps. These helpers preserve both jumps and fallthrough.
    fn scan_regex(&mut self, lexer: &mut ScannerLexer<'_>, valid: &ValidSymbols) -> bool {
        if (valid[REGEX] || valid[REGEX_NO_SLASH] || valid[REGEX_NO_SPACE])
            && !valid[ERROR_RECOVERY]
        {
            if valid[REGEX] || valid[REGEX_NO_SPACE] {
                lexer.skip_whitespace::<true>();
            }
            if !matches!(lexer.lookahead(), 0x22 | 0x27)
                || (matches!(lexer.lookahead(), 0x24 | 0x27) && valid[REGEX_NO_SLASH])
                || (lexer.lookahead() == i32::from(b'\'') && valid[REGEX_NO_SPACE])
            {
                #[derive(Default)]
                struct State {
                    done: bool,
                    advanced_once: bool,
                    found_non_alnumdollarunderdash: bool,
                    last_was_escape: bool,
                    in_single_quote: bool,
                    paren_depth: u32,
                    bracket_depth: u32,
                    brace_depth: u32,
                }
                if lexer.lookahead() == i32::from(b'$') && valid[REGEX_NO_SLASH] {
                    lexer.mark_end();
                    advance(lexer);
                    if lexer.lookahead() == i32::from(b'(') {
                        return false;
                    }
                }
                lexer.mark_end();
                let mut state = State::default();
                while !state.done {
                    if state.in_single_quote && lexer.lookahead() == i32::from(b'\'') {
                        state.in_single_quote = false;
                        advance(lexer);
                        lexer.mark_end();
                    }
                    match lexer.lookahead() {
                        0x5c => state.last_was_escape = true,
                        0 => return false,
                        0x28 => {
                            state.paren_depth = state.paren_depth.wrapping_add(1);
                            state.last_was_escape = false;
                        }
                        0x5b => {
                            state.bracket_depth = state.bracket_depth.wrapping_add(1);
                            state.last_was_escape = false;
                        }
                        0x7b => {
                            if !state.last_was_escape {
                                state.brace_depth = state.brace_depth.wrapping_add(1);
                            }
                            state.last_was_escape = false;
                        }
                        0x29 => {
                            if state.paren_depth == 0 {
                                state.done = true;
                            }
                            state.paren_depth = state.paren_depth.wrapping_sub(1);
                            state.last_was_escape = false;
                        }
                        0x5d => {
                            if state.bracket_depth == 0 {
                                state.done = true;
                            }
                            state.bracket_depth = state.bracket_depth.wrapping_sub(1);
                            state.last_was_escape = false;
                        }
                        0x7d => {
                            if state.brace_depth == 0 {
                                state.done = true;
                            }
                            state.brace_depth = state.brace_depth.wrapping_sub(1);
                            state.last_was_escape = false;
                        }
                        0x27 => {
                            state.in_single_quote = !state.in_single_quote;
                            advance(lexer);
                            state.advanced_once = true;
                            state.last_was_escape = false;
                            continue;
                        }
                        _ => state.last_was_escape = false,
                    }
                    if !state.done {
                        if valid[REGEX] {
                            let was_space = !state.in_single_quote && lexer.is_space();
                            advance(lexer);
                            state.advanced_once = true;
                            if !was_space || state.paren_depth > 0 {
                                lexer.mark_end();
                            }
                        } else if valid[REGEX_NO_SLASH] {
                            if lexer.lookahead() == i32::from(b'/') {
                                lexer.mark_end();
                                lexer.set_result_symbol(REGEX_NO_SLASH as u16);
                                return state.advanced_once;
                            }
                            if lexer.lookahead() == i32::from(b'\\') {
                                advance(lexer);
                                state.advanced_once = true;
                                if !lexer.eof() && !matches!(lexer.lookahead(), 0x5b | 0x2f) {
                                    advance(lexer);
                                    lexer.mark_end();
                                }
                            } else {
                                let was_space = !state.in_single_quote && lexer.is_space();
                                advance(lexer);
                                state.advanced_once = true;
                                if !was_space {
                                    lexer.mark_end();
                                }
                            }
                        } else if valid[REGEX_NO_SPACE] {
                            if lexer.lookahead() == i32::from(b'\\') {
                                state.found_non_alnumdollarunderdash = true;
                                advance(lexer);
                                if !lexer.eof() {
                                    advance(lexer);
                                }
                            } else if lexer.lookahead() == i32::from(b'$') {
                                lexer.mark_end();
                                advance(lexer);
                                // Do not parse a command substitution.
                                if lexer.lookahead() == i32::from(b'(') {
                                    return false;
                                }
                                if lexer.is_space() {
                                    lexer.set_result_symbol(REGEX_NO_SPACE as u16);
                                    lexer.mark_end();
                                    return true;
                                }
                            } else {
                                let was_space = !state.in_single_quote && lexer.is_space();
                                if was_space && state.paren_depth == 0 {
                                    lexer.mark_end();
                                    lexer.set_result_symbol(REGEX_NO_SPACE as u16);
                                    return state.found_non_alnumdollarunderdash;
                                }
                                if !lexer.is_alnum()
                                    && !matches!(lexer.lookahead(), 0x24 | 0x2d | 0x5f)
                                {
                                    state.found_non_alnumdollarunderdash = true;
                                }
                                advance(lexer);
                            }
                        }
                    }
                }
                lexer.set_result_symbol(if valid[REGEX_NO_SLASH] {
                    REGEX_NO_SLASH
                } else if valid[REGEX_NO_SPACE] {
                    REGEX_NO_SPACE
                } else {
                    REGEX
                } as u16);
                if valid[REGEX] && !state.advanced_once {
                    return false;
                }
                return true;
            }
        }
        self.scan_extglob_pattern(lexer, valid)
    }

    fn scan_extglob_pattern(&mut self, lexer: &mut ScannerLexer<'_>, valid: &ValidSymbols) -> bool {
        if valid[EXTGLOB_PATTERN] && !valid[ERROR_RECOVERY] {
            lexer.skip_whitespace::<true>();
            if matches!(
                lexer.lookahead(),
                0x3f | 0x2a | 0x2b | 0x40 | 0x21 | 0x2d | 0x29 | 0x5c | 0x2e | 0x5b
            ) || lexer.is_alpha()
            {
                if lexer.lookahead() == i32::from(b'\\') {
                    advance(lexer);
                    if (lexer.is_space() || lexer.lookahead() == i32::from(b'"'))
                        && !matches!(lexer.lookahead(), 0x0d | 0x0a)
                    {
                        advance(lexer);
                    } else {
                        return false;
                    }
                }
                if lexer.lookahead() == i32::from(b')') && self.last_glob_paren_depth == 0 {
                    lexer.mark_end();
                    advance(lexer);
                    if lexer.is_space() {
                        return false;
                    }
                }
                lexer.mark_end();
                let was_non_alpha = !lexer.is_alpha();
                if lexer.lookahead() != i32::from(b'[') {
                    // Do not accept esac followed by whitespace.
                    if lexer.lookahead() == i32::from(b'e') {
                        lexer.mark_end();
                        advance(lexer);
                        if lexer.lookahead() == i32::from(b's') {
                            advance(lexer);
                            if lexer.lookahead() == i32::from(b'a') {
                                advance(lexer);
                                if lexer.lookahead() == i32::from(b'c') {
                                    advance(lexer);
                                    if lexer.is_space() {
                                        return false;
                                    }
                                }
                            }
                        }
                    } else {
                        advance(lexer);
                    }
                }
                if lexer.lookahead() == i32::from(b'-') {
                    lexer.mark_end();
                    advance(lexer);
                    while lexer.is_alnum() {
                        advance(lexer);
                    }
                    if matches!(lexer.lookahead(), 0x29 | 0x5c | 0x2e) {
                        return false;
                    }
                    lexer.mark_end();
                }
                // Case item -) or *).
                if lexer.lookahead() == i32::from(b')') && self.last_glob_paren_depth == 0 {
                    lexer.mark_end();
                    advance(lexer);
                    if lexer.is_space() {
                        lexer.set_result_symbol(EXTGLOB_PATTERN as u16);
                        return was_non_alpha;
                    }
                }
                if lexer.is_space() {
                    lexer.mark_end();
                    lexer.set_result_symbol(EXTGLOB_PATTERN as u16);
                    self.last_glob_paren_depth = 0;
                    return true;
                }
                if lexer.lookahead() == i32::from(b'$') {
                    lexer.mark_end();
                    advance(lexer);
                    if matches!(lexer.lookahead(), 0x7b | 0x28) {
                        lexer.set_result_symbol(EXTGLOB_PATTERN as u16);
                        return true;
                    }
                }
                if lexer.lookahead() == i32::from(b'|') {
                    lexer.mark_end();
                    advance(lexer);
                    lexer.set_result_symbol(EXTGLOB_PATTERN as u16);
                    return true;
                }
                if !lexer.is_alnum()
                    && !matches!(
                        lexer.lookahead(),
                        0x28 | 0x22 | 0x5b | 0x3f | 0x2f | 0x5c | 0x5f | 0x2a
                    )
                {
                    return false;
                }
                struct State {
                    done: bool,
                    saw_non_alphadot: bool,
                    paren_depth: u32,
                    bracket_depth: u32,
                    brace_depth: u32,
                }
                let mut state = State {
                    done: false,
                    saw_non_alphadot: was_non_alpha,
                    paren_depth: u32::from(self.last_glob_paren_depth),
                    bracket_depth: 0,
                    brace_depth: 0,
                };
                while !state.done {
                    match lexer.lookahead() {
                        0 => return false,
                        0x28 => state.paren_depth = state.paren_depth.wrapping_add(1),
                        0x5b => state.bracket_depth = state.bracket_depth.wrapping_add(1),
                        0x7b => state.brace_depth = state.brace_depth.wrapping_add(1),
                        0x29 => {
                            if state.paren_depth == 0 {
                                state.done = true;
                            }
                            state.paren_depth = state.paren_depth.wrapping_sub(1);
                        }
                        0x5d => {
                            if state.bracket_depth == 0 {
                                state.done = true;
                            }
                            state.bracket_depth = state.bracket_depth.wrapping_sub(1);
                        }
                        0x7d => {
                            if state.brace_depth == 0 {
                                state.done = true;
                            }
                            state.brace_depth = state.brace_depth.wrapping_sub(1);
                        }
                        _ => {}
                    }
                    if lexer.lookahead() == i32::from(b'|') {
                        lexer.mark_end();
                        advance(lexer);
                        if state.paren_depth == 0
                            && state.bracket_depth == 0
                            && state.brace_depth == 0
                        {
                            lexer.set_result_symbol(EXTGLOB_PATTERN as u16);
                            return true;
                        }
                    }
                    if !state.done {
                        let was_space = lexer.is_space();
                        if lexer.lookahead() == i32::from(b'$') {
                            lexer.mark_end();
                            if !lexer.is_alpha() && !matches!(lexer.lookahead(), 0x2e | 0x5c) {
                                state.saw_non_alphadot = true;
                            }
                            advance(lexer);
                            if matches!(lexer.lookahead(), 0x28 | 0x7b) {
                                lexer.set_result_symbol(EXTGLOB_PATTERN as u16);
                                self.last_glob_paren_depth = state.paren_depth as u8;
                                return state.saw_non_alphadot;
                            }
                        }
                        if was_space {
                            lexer.mark_end();
                            lexer.set_result_symbol(EXTGLOB_PATTERN as u16);
                            self.last_glob_paren_depth = 0;
                            return state.saw_non_alphadot;
                        }
                        if lexer.lookahead() == i32::from(b'"') {
                            lexer.mark_end();
                            lexer.set_result_symbol(EXTGLOB_PATTERN as u16);
                            self.last_glob_paren_depth = 0;
                            return state.saw_non_alphadot;
                        }
                        if lexer.lookahead() == i32::from(b'\\') {
                            if !lexer.is_alpha() && !matches!(lexer.lookahead(), 0x2e | 0x5c) {
                                state.saw_non_alphadot = true;
                            }
                            advance(lexer);
                            if lexer.is_space() || lexer.lookahead() == i32::from(b'"') {
                                advance(lexer);
                            }
                        } else {
                            if !lexer.is_alpha() && !matches!(lexer.lookahead(), 0x2e | 0x5c) {
                                state.saw_non_alphadot = true;
                            }
                            advance(lexer);
                        }
                        if !was_space {
                            lexer.mark_end();
                        }
                    }
                }
                lexer.set_result_symbol(EXTGLOB_PATTERN as u16);
                self.last_glob_paren_depth = 0;
                return state.saw_non_alphadot;
            }
            self.last_glob_paren_depth = 0;
            return false;
        }
        self.scan_expansion_word(lexer, valid)
    }

    fn scan_expansion_word(&mut self, lexer: &mut ScannerLexer<'_>, valid: &ValidSymbols) -> bool {
        if valid[EXPANSION_WORD] {
            let mut advanced_once = false;
            let mut advance_once_space = false;
            loop {
                if lexer.lookahead() == i32::from(b'"') {
                    return false;
                }
                if lexer.lookahead() == i32::from(b'$') {
                    lexer.mark_end();
                    advance(lexer);
                    if matches!(lexer.lookahead(), 0x7b | 0x28 | 0x27) || lexer.is_alnum() {
                        lexer.set_result_symbol(EXPANSION_WORD as u16);
                        return advanced_once;
                    }
                    advanced_once = true;
                }
                if lexer.lookahead() == i32::from(b'}') {
                    lexer.mark_end();
                    lexer.set_result_symbol(EXPANSION_WORD as u16);
                    return advanced_once || advance_once_space;
                }
                if lexer.lookahead() == i32::from(b'(') && !(advanced_once || advance_once_space) {
                    lexer.mark_end();
                    advance(lexer);
                    while lexer.lookahead() != i32::from(b')') && !lexer.eof() {
                        if lexer.lookahead() == i32::from(b'$') {
                            lexer.mark_end();
                            advance(lexer);
                            if matches!(lexer.lookahead(), 0x7b | 0x28 | 0x27) || lexer.is_alnum() {
                                lexer.set_result_symbol(EXPANSION_WORD as u16);
                                return advanced_once;
                            }
                            advanced_once = true;
                        } else {
                            advanced_once = advanced_once || !lexer.is_space();
                            advance_once_space = advance_once_space || lexer.is_space();
                            advance(lexer);
                        }
                    }
                    lexer.mark_end();
                    if lexer.lookahead() == i32::from(b')') {
                        advanced_once = true;
                        advance(lexer);
                        lexer.mark_end();
                        if lexer.lookahead() == i32::from(b'}') {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                if lexer.lookahead() == i32::from(b'\'') {
                    return false;
                }
                if lexer.eof() {
                    return false;
                }
                advanced_once = advanced_once || !lexer.is_space();
                advance_once_space = advance_once_space || lexer.is_space();
                advance(lexer);
            }
        }
        scan_brace_start(lexer, valid)
    }
}

fn scan_brace_start(lexer: &mut ScannerLexer<'_>, valid: &ValidSymbols) -> bool {
    if valid[BRACE_START] && !valid[ERROR_RECOVERY] {
        lexer.skip_whitespace::<true>();
        if lexer.lookahead() != i32::from(b'{') {
            return false;
        }
        advance(lexer);
        lexer.mark_end();
        while lexer.is_digit() {
            advance(lexer);
        }
        if lexer.lookahead() != i32::from(b'.') {
            return false;
        }
        advance(lexer);
        if lexer.lookahead() != i32::from(b'.') {
            return false;
        }
        advance(lexer);
        while lexer.is_digit() {
            advance(lexer);
        }
        if lexer.lookahead() != i32::from(b'}') {
            return false;
        }
        lexer.set_result_symbol(BRACE_START as u16);
        return true;
    }
    false
}

pub(crate) fn create() -> Box<dyn ExternalScanner> {
    Box::<Scanner>::default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    #[derive(Debug, PartialEq, Eq)]
    enum Event {
        Advance(bool),
        MarkEnd,
        Column,
        Symbol(u16),
    }

    struct TestLexer {
        input: Vec<i32>,
        position: usize,
        end: Option<usize>,
        symbol: u16,
        events: Vec<Event>,
        lookahead_calls: Cell<usize>,
    }

    impl TestLexer {
        fn new(input: &str) -> Self {
            Self {
                input: input.chars().map(|c| c as i32).collect(),
                position: 0,
                end: None,
                symbol: u16::MAX,
                events: Vec::new(),
                lookahead_calls: Cell::new(0),
            }
        }

        fn token_end(&self) -> usize {
            self.end.unwrap_or(self.position)
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
            self.events.push(Event::Symbol(symbol));
        }

        fn advance(&mut self, skip: bool) {
            self.events.push(Event::Advance(skip));
            if !self.eof() {
                self.position += 1;
            }
        }

        fn mark_end(&mut self) {
            self.events.push(Event::MarkEnd);
            self.end = Some(self.position);
        }

        fn get_column(&mut self) -> u32 {
            self.events.push(Event::Column);
            self.input[..self.position]
                .iter()
                .rev()
                .take_while(|&&c| c != i32::from(b'\n'))
                .count() as u32
        }

        fn is_at_included_range_start(&self) -> bool {
            false
        }

        fn eof(&self) -> bool {
            self.position == self.input.len()
        }
    }

    fn valid(tokens: &[usize]) -> [bool; ERROR_RECOVERY + 1] {
        let mut result = [false; ERROR_RECOVERY + 1];
        for &token in tokens {
            result[token] = true;
        }
        result
    }

    fn snapshot(scanner: &mut Scanner) -> Vec<u8> {
        let mut buffer = [0; SERIALIZATION_BUFFER_SIZE];
        let size = scanner.serialize(&mut buffer);
        buffer[..size].to_vec()
    }

    #[test]
    fn cached_lookahead_refreshes_and_keeps_nul_distinct_from_eof() {
        let mut inner = TestLexer::new(" \0é?");
        // The runtime also uses negative lookahead values for decoding errors.
        inner.input[3] = -1;
        {
            let mut lexer = ScannerLexer::new(&mut inner);
            assert_eq!(lexer.lookahead(), i32::from(b' '));
            assert_eq!(lexer.lookahead(), i32::from(b' '));
            skip(&mut lexer);
            assert_eq!(lexer.lookahead(), 0);
            assert!(!lexer.eof());
            lexer.mark_end();
            lexer.set_result_symbol(CONCAT as u16);
            assert_eq!(lexer.result_symbol(), CONCAT as u16);
            assert_eq!(lexer.get_column(), 1);
            assert_eq!(lexer.lookahead(), 0);
            advance(&mut lexer);
            assert_eq!(lexer.lookahead(), 'é' as i32);
            advance(&mut lexer);
            assert_eq!(lexer.lookahead(), -1);
            assert!(!lexer.eof());
            advance(&mut lexer);
            assert_eq!(lexer.lookahead(), 0);
            assert!(lexer.eof());
        }
        assert_eq!(inner.lookahead_calls.get(), 5);
        assert_eq!(inner.token_end(), 1);

        // Each scan has a fresh cache, including after the runtime rewinds.
        inner.position = 2;
        let lexer = ScannerLexer::new(&mut inner);
        assert_eq!(lexer.lookahead(), 'é' as i32);
        assert_eq!(inner.lookahead_calls.get(), 6);
    }

    #[test]
    fn cached_character_classes_match_c_locale_for_all_codepoints() {
        for c in -1..=0x10ffff {
            let class = character_class(c);
            assert_eq!(class & SPACE != 0, is_space(c));
            assert_eq!(class & ALPHA != 0, is_alpha(c));
            assert_eq!(class & DIGIT != 0, is_digit(c));
            assert_eq!(
                class & IDENTIFIER != 0,
                is_alpha(c) || is_digit(c) || c == 0x5f
            );
            assert_eq!(
                class & TEST_PUNCTUATION != 0,
                matches!(c, 0x5c | 0x2d | 0x0a | 0x24)
            );
            assert_eq!(
                class & SPECIAL_VARIABLE != 0,
                matches!(c, 0x2a | 0x40 | 0x3f | 0x2d | 0x30 | 0x5f)
            );
            assert_eq!(
                class & CONCAT_END != 0,
                is_space(c) || matches!(c, 0 | 0x3e | 0x3c | 0x29 | 0x28 | 0x3b | 0x26 | 0x7c)
            );
        }
        assert_eq!(character_class(i32::MIN), 0);
        assert_eq!(character_class(i32::MAX), 0);
    }

    #[test]
    fn identifier_runs_preserve_numeric_status_and_cache_boundary() {
        for (input, numeric, end) in [
            ("123>", Some(true), 3),
            ("12a3=", Some(false), 4),
            ("abc123_+", Some(false), 7),
            ("_12=", Some(false), 3),
            ("12é", Some(true), 2),
            ("é", None, 0),
            ("12\0", Some(true), 2),
            ("", None, 0),
        ] {
            let mut inner = TestLexer::new(input);
            let mut lexer = ScannerLexer::new(&mut inner);
            assert_eq!(lexer.scan_identifier(), numeric, "{input:?}");
            assert_eq!(lexer.class, character_class(lexer.lookahead));
            assert_eq!(inner.position, end);
            assert_eq!(inner.lookahead_calls.get(), end + 1);
            assert_eq!(
                inner.events,
                (0..end).map(|_| Event::Advance(false)).collect::<Vec<_>>()
            );
        }
    }

    #[test]
    fn identifier_runs_match_c_control_flow() {
        // scanner.c tests the first character separately, then keeps testing
        // both digit and name characters throughout the remaining run. Compare
        // that control flow with the port's split numeric/name loops, including
        // their speculative advances when the surrounding token is rejected.
        fn reference(lexer: &mut TestLexer) -> Option<bool> {
            let mut is_number = true;
            if is_digit(lexer.lookahead()) {
                lexer.advance(false);
            } else if is_alpha(lexer.lookahead()) || lexer.lookahead() == 0x5f {
                is_number = false;
                lexer.advance(false);
            } else {
                return None;
            }
            loop {
                if is_digit(lexer.lookahead()) {
                    lexer.advance(false);
                } else if is_alpha(lexer.lookahead()) || lexer.lookahead() == 0x5f {
                    is_number = false;
                    lexer.advance(false);
                } else {
                    return Some(is_number);
                }
            }
        }

        let characters: Vec<i32> = (0..128)
            .chain([-1, 128, 233, 0x100, 0x10ffff, i32::MAX])
            .collect();
        for &first in &characters {
            for &second in &characters {
                for prefix in ["", "09", "a_"] {
                    let mut expected = TestLexer::new(prefix);
                    expected.input.extend([first, second]);
                    expected.input.extend("8A_0=".chars().map(|c| c as i32));
                    let mut actual = TestLexer::new("");
                    actual.input.clone_from(&expected.input);
                    let expected_result = reference(&mut expected);
                    let mut cursor = ScannerLexer::new(&mut actual);
                    assert_eq!(
                        cursor.scan_identifier(),
                        expected_result,
                        "prefix={prefix:?}, first={first}, second={second}"
                    );
                    assert_eq!(cursor.lookahead(), expected.lookahead());
                    assert_eq!(cursor.class, character_class(expected.lookahead()));
                    assert_eq!(actual.position, expected.position);
                    assert_eq!(actual.events, expected.events);
                    assert_eq!(actual.end, expected.end);
                    assert_eq!(actual.symbol, expected.symbol);
                    assert_eq!(actual.lookahead_calls.get(), actual.position + 1);
                }
            }
        }
    }

    #[test]
    fn punctuation_filter_matches_unfiltered_control_flow() {
        let rows = &crate::language().external_scanner.as_ref().unwrap().states;
        let characters: Vec<i32> = (0..128)
            .chain([-1, 128, 233, 0x100, 0x10ffff, i32::MAX])
            .collect();
        for symbols in rows.as_chunks::<{ ERROR_RECOVERY + 1 }>().0 {
            for prefix in ["", " \t", " \n"] {
                for &c in &characters {
                    for suffix in ["", "-n $\n", "\\\n $x}"] {
                        let mut expected = TestLexer::new(prefix);
                        expected.input.push(c);
                        expected.input.extend(suffix.chars().map(|c| c as i32));
                        let mut actual = TestLexer::new("");
                        actual.input.clone_from(&expected.input);
                        let mut reference = Scanner::default();
                        let mut filtered = Scanner::default();
                        let mut lexer = ScannerLexer::new(&mut expected);
                        lexer.skip_whitespace::<false>();
                        let expected_result = reference.scan_test_punctuation(&mut lexer, symbols);
                        let mut lexer = ScannerLexer::new(&mut actual);
                        lexer.skip_whitespace::<false>();
                        let result = if lexer.starts_test_punctuation(symbols) {
                            filtered.scan_test_punctuation(&mut lexer, symbols)
                        } else {
                            None
                        };
                        assert_eq!(result, expected_result);
                        assert_eq!(actual.position, expected.position);
                        assert_eq!(actual.end, expected.end);
                        assert_eq!(actual.symbol, expected.symbol);
                        assert_eq!(actual.events, expected.events);
                        assert_eq!(actual.lookahead_calls.get(), expected.lookahead_calls.get());
                        assert_eq!(snapshot(&mut filtered), snapshot(&mut reference));
                    }
                }
            }
        }
    }

    #[test]
    fn whitespace_runs_preserve_skip_flags_and_newline_boundary() {
        let mut inner = TestLexer::new(" \t\r\n x");
        let mut lexer = ScannerLexer::new(&mut inner);
        lexer.skip_whitespace::<false>();
        assert_eq!(lexer.lookahead(), 0x0a);
        assert!(lexer.is_space());
        lexer.skip_whitespace::<true>();
        assert_eq!(lexer.lookahead(), i32::from(b'x'));
        assert!(lexer.is_alpha());
        assert_eq!(inner.position, 5);
        assert_eq!(inner.lookahead_calls.get(), 6);
        assert_eq!(
            inner.events,
            (0..5).map(|_| Event::Advance(true)).collect::<Vec<_>>()
        );
    }

    #[test]
    fn concat_only_specialization_matches_full_scanner_callbacks() {
        let valid = valid(&[CONCAT]);
        let mut scanner = Scanner::default();
        let mut check = |input: &str| {
            let mut full = TestLexer::new(input);
            let mut specialized = TestLexer::new(input);
            let full_result = scanner.scan_inner::<false>(&mut full, &valid);
            let specialized_result = scanner.scan_inner::<true>(&mut specialized, &valid);
            assert_eq!(full_result, specialized_result, "{input:?}");
            assert_eq!(full.position, specialized.position, "{input:?}");
            assert_eq!(full.end, specialized.end, "{input:?}");
            assert_eq!(full.symbol, specialized.symbol, "{input:?}");
            assert_eq!(full.events, specialized.events, "{input:?}");
            assert_eq!(
                full.lookahead_calls.get(),
                specialized.lookahead_calls.get()
            );
        };
        for c in 0..=127 {
            check(&char::from(c).to_string());
        }
        for input in [
            "é", "Ł", "`b` ", "`b`x", "`b", "`\0` ", "\\", "\\x", "\\\n", "\\\\", "\\\"", "\\'",
        ] {
            check(input);
        }
    }

    #[test]
    fn snapshot_bytes_and_nontruncating_reset() {
        let mut scanner = Scanner {
            last_glob_paren_depth: 201,
            ext_was_in_double_quote: true,
            ext_saw_outside_quote: true,
            heredocs: vec![Heredoc {
                is_raw: true,
                started: true,
                allows_indent: true,
                delimiter: b"END\0".to_vec(),
                current_leading_word: b"scratch\0".to_vec(),
            }],
        };
        let mut expected = vec![201, 1, 1, 1, 1, 1, 1];
        expected.extend_from_slice(&4_u32.to_ne_bytes());
        expected.extend_from_slice(b"END\0");
        assert_eq!(snapshot(&mut scanner), expected);

        let mut restored = Scanner::default();
        restored.deserialize(&expected);
        assert_eq!(snapshot(&mut restored), expected);

        // Restoring fewer heredocs does not shrink the existing stack in C.
        scanner.deserialize(&[7, 0, 1, 0]);
        assert_eq!(scanner.heredocs.len(), 1);
        assert_eq!(scanner.heredocs[0].delimiter, b"END\0");
        assert_eq!(scanner.heredocs[0].current_leading_word, b"scratch\0");
        scanner.deserialize(&[]);
        assert_eq!(snapshot(&mut scanner), [7, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(scanner.heredocs[0].current_leading_word, b"scratch\0");
    }

    #[test]
    fn serialization_requires_one_unused_byte() {
        let mut scanner = Scanner {
            heredocs: vec![Heredoc {
                delimiter: vec![b'x'; SERIALIZATION_BUFFER_SIZE - 12],
                ..Heredoc::default()
            }],
            ..Scanner::default()
        };
        let mut buffer = [0; SERIALIZATION_BUFFER_SIZE];
        assert_eq!(
            scanner.serialize(&mut buffer),
            SERIALIZATION_BUFFER_SIZE - 1
        );
        scanner.heredocs[0].delimiter.push(0);
        assert_eq!(scanner.serialize(&mut buffer), 0);
    }

    #[test]
    fn raw_indented_heredoc_retains_delimiter_until_end_token() {
        let mut scanner = Scanner::default();
        let mut lexer = TestLexer::new("<<-");
        assert!(scanner.scan(&mut lexer, &valid(&[HEREDOC_ARROW])));
        assert_eq!(lexer.symbol, HEREDOC_ARROW_DASH as u16);
        assert_eq!(lexer.token_end(), 3);
        assert!(scanner.heredocs[0].allows_indent);

        let mut lexer = TestLexer::new(" 'E\\ND'\n");
        assert!(scanner.scan(&mut lexer, &valid(&[HEREDOC_START])));
        assert_eq!(scanner.heredocs[0].delimiter, b"END\0");
        assert!(scanner.heredocs[0].is_raw);
        let mut lexer = TestLexer::new("\n\t$value\n\tEND\n");
        assert!(scanner.scan(&mut lexer, &valid(&[SIMPLE_HEREDOC_BODY])));
        assert_eq!(lexer.symbol, SIMPLE_HEREDOC_BODY as u16);
        assert_eq!(lexer.token_end(), 10);
        assert_eq!(lexer.position, 13);
        assert_eq!(scanner.heredocs.len(), 1);
        assert!(!scanner.heredocs[0].started);

        let mut lexer = TestLexer::new("END\n");
        assert!(scanner.scan(&mut lexer, &valid(&[HEREDOC_END])));
        assert_eq!(lexer.token_end(), 3);
        assert!(scanner.heredocs.is_empty());
    }

    #[test]
    fn heredoc_expansion_leaves_dollar_outside_token() {
        let mut scanner = Scanner {
            heredocs: vec![Heredoc {
                delimiter: b"END\0".to_vec(),
                ..Heredoc::default()
            }],
            ..Scanner::default()
        };
        let mut lexer = TestLexer::new("\nhello $name\nEND\n");
        assert!(scanner.scan(&mut lexer, &valid(&[HEREDOC_BODY_BEGINNING])));
        assert_eq!(lexer.symbol, HEREDOC_BODY_BEGINNING as u16);
        assert_eq!(lexer.token_end(), 7);
        assert_eq!(lexer.position, 8);
        assert!(scanner.heredocs[0].started);

        let mut lexer = TestLexer::new("$name");
        scanner.heredocs[0].started = false;
        assert!(scanner.scan(&mut lexer, &valid(&[HEREDOC_BODY_BEGINNING])));
        assert_eq!(lexer.token_end(), 0);
        assert_eq!(
            lexer.events,
            [Event::Column, Event::Symbol(HEREDOC_BODY_BEGINNING as u16)]
        );
    }

    #[test]
    fn heredoc_words_truncate_codepoints_and_compare_as_signed_char() {
        let mut heredoc = Heredoc::default();
        assert!(heredoc.scan_start(&mut ScannerLexer::new(&mut TestLexer::new("é\n"))));
        assert_eq!(heredoc.delimiter, [0xe9, 0]);
        let mut lexer = TestLexer::new("é\n");
        assert!(!heredoc.scan_end_identifier(&mut ScannerLexer::new(&mut lexer)));
        assert_eq!(lexer.position, 0);

        // U+0141 truncates to ASCII A; the original C char buffer behaves this way.
        heredoc.delimiter.clear();
        assert!(heredoc.scan_start(&mut ScannerLexer::new(&mut TestLexer::new("Ł\n"))));
        assert_eq!(heredoc.delimiter, b"A\0");
        assert!(heredoc.scan_end_identifier(&mut ScannerLexer::new(&mut TestLexer::new("A\n"))));
    }

    #[test]
    fn bare_dollar_and_concat_callback_order() {
        let mut scanner = Scanner::default();
        let mut lexer = TestLexer::new("\t$ ");
        assert!(scanner.scan(&mut lexer, &valid(&[BARE_DOLLAR])));
        assert_eq!(
            lexer.events,
            [
                Event::Advance(true),
                Event::Advance(false),
                Event::Symbol(BARE_DOLLAR as u16),
                Event::MarkEnd,
            ]
        );
        let mut lexer = TestLexer::new("`b` ");
        assert!(scanner.scan(&mut lexer, &valid(&[CONCAT])));
        assert_eq!(lexer.token_end(), 0);
        assert_eq!(
            lexer.events,
            [
                Event::Symbol(CONCAT as u16),
                Event::MarkEnd,
                Event::Advance(false),
                Event::Advance(false),
                Event::Advance(false),
            ]
        );
    }

    #[test]
    fn lookahead_boundaries_and_forward_jumps() {
        for (input, tokens, accepted, symbol, end) in [
            (
                "##x",
                vec![IMMEDIATE_DOUBLE_HASH],
                true,
                IMMEDIATE_DOUBLE_HASH,
                2,
            ),
            (
                "#!= }",
                vec![EXTERNAL_EXPANSION_SYM_HASH],
                true,
                EXTERNAL_EXPANSION_SYM_HASH,
                1,
            ),
            ("12>", vec![FILE_DESCRIPTOR], true, FILE_DESCRIPTOR, 2),
            ("foo+=", vec![VARIABLE_NAME], true, VARIABLE_NAME, 3),
            (
                "{12..34}",
                vec![VARIABLE_NAME, BRACE_START],
                true,
                BRACE_START,
                1,
            ),
            ("word$var}", vec![EXPANSION_WORD], true, EXPANSION_WORD, 4),
            (" }", vec![EXPANSION_WORD], true, EXPANSION_WORD, 1),
            ("(word)}", vec![EXPANSION_WORD], false, 0, 0),
            (
                "\\. ",
                vec![TEST_OPERATOR, REGEX_NO_SPACE],
                true,
                REGEX_NO_SPACE,
                2,
            ),
            ("abc/", vec![REGEX_NO_SLASH], true, REGEX_NO_SLASH, 3),
            ("(a b) ]", vec![REGEX], true, REGEX, 5),
            ("abc ", vec![REGEX_NO_SPACE], false, 0, 0),
            ("a.* ", vec![REGEX_NO_SPACE], true, REGEX_NO_SPACE, 3),
            ("esac ", vec![EXTGLOB_PATTERN], false, 0, 0),
            ("*) ", vec![EXTGLOB_PATTERN], true, EXTGLOB_PATTERN, 1),
            (
                "\\\"* ",
                vec![TEST_OPERATOR, EXTGLOB_PATTERN],
                true,
                EXTGLOB_PATTERN,
                3,
            ),
        ] {
            let mut scanner = Scanner::default();
            let mut lexer = TestLexer::new(input);
            assert_eq!(
                scanner.scan(&mut lexer, &valid(&tokens)),
                accepted,
                "{input:?}"
            );
            assert_eq!(
                lexer.lookahead_calls.get(),
                1 + lexer
                    .events
                    .iter()
                    .filter(|event| matches!(event, Event::Advance(_)))
                    .count(),
                "one lookahead per advance, plus scan entry: {input:?}"
            );
            if accepted {
                assert_eq!(lexer.symbol, symbol as u16, "{input:?}");
                assert_eq!(lexer.token_end(), end, "{input:?}");
            }
        }
    }

    #[test]
    fn glob_depth_is_truncated_to_one_byte_at_expansion() {
        let mut scanner = Scanner::default();
        let mut lexer = TestLexer::new(&format!("*{}${{name}}", "(".repeat(257)));
        assert!(scanner.scan(&mut lexer, &valid(&[EXTGLOB_PATTERN])));
        assert_eq!(lexer.symbol, EXTGLOB_PATTERN as u16);
        assert_eq!(lexer.token_end(), 258);
        assert_eq!(snapshot(&mut scanner), [1, 0, 0, 0]);
    }

    #[test]
    fn character_classes_use_c_locale() {
        assert!(is_space(0x0b));
        assert!(!is_space(0xa0));
        assert!(!is_alpha('é' as i32));
        assert!(!is_digit('١' as i32));
        assert!(!is_space(-1));
    }
}
