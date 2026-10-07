//! Bash's external scanner, translated from `src/scanner.c`.
//!
//! The heredoc strings are byte arrays, not UTF-8: the C scanner truncates each
//! lookahead to `char`. State restoration intentionally neither truncates the
//! heredoc stack nor resets the glob fields when given an empty snapshot.

use ts_port_tables::{ExternalScanner, Lexer, SERIALIZATION_BUFFER_SIZE};

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

// The reference uses the default C locale, not Unicode character classes.
fn is_space(c: i32) -> bool {
    matches!(c, 0x09..=0x0d | 0x20)
}

fn is_alpha(c: i32) -> bool {
    matches!(c, 0x41..=0x5a | 0x61..=0x7a)
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

fn skip(lexer: &mut dyn Lexer) {
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

    fn scan_start(&mut self, lexer: &mut dyn Lexer) -> bool {
        while is_space(lexer.lookahead()) {
            skip(lexer);
        }
        lexer.set_result_symbol(HEREDOC_START as u16);
        self.is_raw = matches!(lexer.lookahead(), 0x27 | 0x22 | 0x5c);
        let found_delimiter = advance_word(lexer, &mut self.delimiter);
        if !found_delimiter {
            self.delimiter.clear();
            return false;
        }
        found_delimiter
    }

    fn scan_end_identifier(&mut self, lexer: &mut dyn Lexer) -> bool {
        self.current_leading_word.clear();
        let mut size = 0;
        if !self.delimiter.is_empty() {
            while lexer.lookahead() != 0
                && lexer.lookahead() != i32::from(b'\n')
                && size < self.delimiter.len()
                // The reference target has signed char, including after the
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
fn advance_word(lexer: &mut dyn Lexer, unquoted_word: &mut Vec<u8>) -> bool {
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
            is_space(lexer.lookahead())
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

fn scan_bare_dollar(lexer: &mut dyn Lexer) -> bool {
    while is_space(lexer.lookahead()) && lexer.lookahead() != i32::from(b'\n') && !lexer.eof() {
        skip(lexer);
    }
    if lexer.lookahead() == i32::from(b'$') {
        advance(lexer);
        lexer.set_result_symbol(BARE_DOLLAR as u16);
        lexer.mark_end();
        return is_space(lexer.lookahead()) || lexer.eof() || lexer.lookahead() == i32::from(b'"');
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
    fn scan_heredoc_content(
        &mut self,
        lexer: &mut dyn Lexer,
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
                        if is_alpha(lexer.lookahead()) || matches!(lexer.lookahead(), 0x7b | 0x28) {
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
                        while is_space(lexer.lookahead()) {
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
                            self.heredocs.pop();
                        }
                        return true;
                    }
                }
                _ => {
                    if lexer.get_column() == 0 {
                        while is_space(lexer.lookahead()) {
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

impl ExternalScanner for Scanner {
    fn scan(&mut self, lexer: &mut dyn Lexer, valid: &[bool]) -> bool {
        if valid[CONCAT] && !valid[ERROR_RECOVERY] {
            if !(lexer.lookahead() == 0
                || is_space(lexer.lookahead())
                || matches!(
                    lexer.lookahead(),
                    0x3e | 0x3c | 0x29 | 0x28 | 0x3b | 0x26 | 0x7c
                )
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
                    return is_space(lexer.lookahead()) || lexer.eof();
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
            if is_space(lexer.lookahead()) && valid[CLOSING_BRACE] && !valid[EXPANSION_WORD] {
                lexer.set_result_symbol(CONCAT as u16);
                return true;
            }
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
            while is_space(lexer.lookahead()) {
                skip(lexer);
            }
            return lexer.lookahead() == i32::from(b'}');
        }

        if valid[EMPTY_VALUE]
            && (is_space(lexer.lookahead())
                || lexer.eof()
                || matches!(lexer.lookahead(), 0x3b | 0x26))
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
            self.heredocs.pop();
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
            while is_space(lexer.lookahead()) && lexer.lookahead() != i32::from(b'\n') {
                skip(lexer);
            }
            if lexer.lookahead() == i32::from(b'\\') {
                if valid[EXTGLOB_PATTERN] {
                    return self.scan_extglob_pattern(lexer, valid);
                }
                if valid[REGEX_NO_SPACE] {
                    return self.scan_regex(lexer, valid);
                }
                skip(lexer);
                if lexer.eof() {
                    return false;
                }
                if lexer.lookahead() == i32::from(b'\r') {
                    skip(lexer);
                    if lexer.lookahead() == i32::from(b'\n') {
                        skip(lexer);
                    }
                } else if lexer.lookahead() == i32::from(b'\n') {
                    skip(lexer);
                } else {
                    return false;
                }
                while is_space(lexer.lookahead()) {
                    skip(lexer);
                }
            }
            if lexer.lookahead() == i32::from(b'\n') && !valid[NEWLINE] {
                skip(lexer);
                while is_space(lexer.lookahead()) {
                    skip(lexer);
                }
            }
            if lexer.lookahead() == i32::from(b'-') {
                advance(lexer);
                let mut advanced_once = false;
                while is_alpha(lexer.lookahead()) {
                    advanced_once = true;
                    advance(lexer);
                }
                if is_space(lexer.lookahead()) && advanced_once {
                    lexer.mark_end();
                    advance(lexer);
                    if lexer.lookahead() == i32::from(b'}') && valid[CLOSING_BRACE] {
                        if valid[EXPANSION_WORD] {
                            lexer.mark_end();
                            lexer.set_result_symbol(EXPANSION_WORD as u16);
                            return true;
                        }
                        return false;
                    }
                    lexer.set_result_symbol(TEST_OPERATOR as u16);
                    return true;
                }
                if is_space(lexer.lookahead()) && valid[EXTGLOB_PATTERN] {
                    lexer.set_result_symbol(EXTGLOB_PATTERN as u16);
                    return true;
                }
            }
            if valid[BARE_DOLLAR] && !valid[ERROR_RECOVERY] && scan_bare_dollar(lexer) {
                return true;
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

            if !valid[EXPANSION_WORD]
                && matches!(lexer.lookahead(), 0x2a | 0x40 | 0x3f | 0x2d | 0x30 | 0x5f)
            {
                lexer.mark_end();
                advance(lexer);
                if matches!(
                    lexer.lookahead(),
                    0x3d | 0x5b | 0x3a | 0x2d | 0x25 | 0x23 | 0x2f
                ) {
                    return false;
                }
                if valid[EXTGLOB_PATTERN] && is_space(lexer.lookahead()) {
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
                        self.heredocs.push(Heredoc {
                            allows_indent: true,
                            ..Heredoc::default()
                        });
                        lexer.set_result_symbol(HEREDOC_ARROW_DASH as u16);
                    } else if matches!(lexer.lookahead(), 0x3c | 0x3d) {
                        return false;
                    } else {
                        self.heredocs.push(Heredoc::default());
                        lexer.set_result_symbol(HEREDOC_ARROW as u16);
                    }
                    return true;
                }
                return false;
            }

            let mut is_number = true;
            if is_digit(lexer.lookahead()) {
                advance(lexer);
            } else if is_alpha(lexer.lookahead()) || lexer.lookahead() == i32::from(b'_') {
                is_number = false;
                advance(lexer);
            } else {
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
            }
            loop {
                if is_digit(lexer.lookahead()) {
                    advance(lexer);
                } else if is_alpha(lexer.lookahead()) || lexer.lookahead() == i32::from(b'_') {
                    is_number = false;
                    advance(lexer);
                } else {
                    break;
                }
            }
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
                    return is_alpha(lexer.lookahead());
                }
            }
            return false;
        }

        if valid[BARE_DOLLAR] && !valid[ERROR_RECOVERY] && scan_bare_dollar(lexer) {
            return true;
        }
        self.scan_regex(lexer, valid)
    }

    fn serialize(&mut self, buffer: &mut [u8]) -> usize {
        buffer[0] = self.last_glob_paren_depth;
        buffer[1] = u8::from(self.ext_was_in_double_quote);
        buffer[2] = u8::from(self.ext_saw_outside_quote);
        buffer[3] = self.heredocs.len() as u8;
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

    fn deserialize(&mut self, buffer: &[u8]) {
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
    fn scan_regex(&mut self, lexer: &mut dyn Lexer, valid: &[bool]) -> bool {
        if (valid[REGEX] || valid[REGEX_NO_SLASH] || valid[REGEX_NO_SPACE])
            && !valid[ERROR_RECOVERY]
        {
            if valid[REGEX] || valid[REGEX_NO_SPACE] {
                while is_space(lexer.lookahead()) {
                    skip(lexer);
                }
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
                            let was_space = !state.in_single_quote && is_space(lexer.lookahead());
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
                                let was_space =
                                    !state.in_single_quote && is_space(lexer.lookahead());
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
                                if is_space(lexer.lookahead()) {
                                    lexer.set_result_symbol(REGEX_NO_SPACE as u16);
                                    lexer.mark_end();
                                    return true;
                                }
                            } else {
                                let was_space =
                                    !state.in_single_quote && is_space(lexer.lookahead());
                                if was_space && state.paren_depth == 0 {
                                    lexer.mark_end();
                                    lexer.set_result_symbol(REGEX_NO_SPACE as u16);
                                    return state.found_non_alnumdollarunderdash;
                                }
                                if !is_alnum(lexer.lookahead())
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

    fn scan_extglob_pattern(&mut self, lexer: &mut dyn Lexer, valid: &[bool]) -> bool {
        if valid[EXTGLOB_PATTERN] && !valid[ERROR_RECOVERY] {
            while is_space(lexer.lookahead()) {
                skip(lexer);
            }
            if matches!(
                lexer.lookahead(),
                0x3f | 0x2a | 0x2b | 0x40 | 0x21 | 0x2d | 0x29 | 0x5c | 0x2e | 0x5b
            ) || is_alpha(lexer.lookahead())
            {
                if lexer.lookahead() == i32::from(b'\\') {
                    advance(lexer);
                    if (is_space(lexer.lookahead()) || lexer.lookahead() == i32::from(b'"'))
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
                    if is_space(lexer.lookahead()) {
                        return false;
                    }
                }
                lexer.mark_end();
                let was_non_alpha = !is_alpha(lexer.lookahead());
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
                                    if is_space(lexer.lookahead()) {
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
                    while is_alnum(lexer.lookahead()) {
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
                    if is_space(lexer.lookahead()) {
                        lexer.set_result_symbol(EXTGLOB_PATTERN as u16);
                        return was_non_alpha;
                    }
                }
                if is_space(lexer.lookahead()) {
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
                if !is_alnum(lexer.lookahead())
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
                        let was_space = is_space(lexer.lookahead());
                        if lexer.lookahead() == i32::from(b'$') {
                            lexer.mark_end();
                            if !is_alpha(lexer.lookahead())
                                && !matches!(lexer.lookahead(), 0x2e | 0x5c)
                            {
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
                            if !is_alpha(lexer.lookahead())
                                && !matches!(lexer.lookahead(), 0x2e | 0x5c)
                            {
                                state.saw_non_alphadot = true;
                            }
                            advance(lexer);
                            if is_space(lexer.lookahead()) || lexer.lookahead() == i32::from(b'"') {
                                advance(lexer);
                            }
                        } else {
                            if !is_alpha(lexer.lookahead())
                                && !matches!(lexer.lookahead(), 0x2e | 0x5c)
                            {
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

    fn scan_expansion_word(&mut self, lexer: &mut dyn Lexer, valid: &[bool]) -> bool {
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
                    if matches!(lexer.lookahead(), 0x7b | 0x28 | 0x27)
                        || is_alnum(lexer.lookahead())
                    {
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
                            if matches!(lexer.lookahead(), 0x7b | 0x28 | 0x27)
                                || is_alnum(lexer.lookahead())
                            {
                                lexer.set_result_symbol(EXPANSION_WORD as u16);
                                return advanced_once;
                            }
                            advanced_once = true;
                        } else {
                            advanced_once = advanced_once || !is_space(lexer.lookahead());
                            advance_once_space = advance_once_space || is_space(lexer.lookahead());
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
                advanced_once = advanced_once || !is_space(lexer.lookahead());
                advance_once_space = advance_once_space || is_space(lexer.lookahead());
                advance(lexer);
            }
        }
        scan_brace_start(lexer, valid)
    }
}

fn scan_brace_start(lexer: &mut dyn Lexer, valid: &[bool]) -> bool {
    if valid[BRACE_START] && !valid[ERROR_RECOVERY] {
        while is_space(lexer.lookahead()) {
            skip(lexer);
        }
        if lexer.lookahead() != i32::from(b'{') {
            return false;
        }
        advance(lexer);
        lexer.mark_end();
        while is_digit(lexer.lookahead()) {
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
        while is_digit(lexer.lookahead()) {
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

        fn token_end(&self) -> usize {
            self.end.unwrap_or(self.position)
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
        assert!(heredoc.scan_start(&mut TestLexer::new("é\n")));
        assert_eq!(heredoc.delimiter, [0xe9, 0]);
        let mut lexer = TestLexer::new("é\n");
        assert!(!heredoc.scan_end_identifier(&mut lexer));
        assert_eq!(lexer.position, 0);

        // U+0141 truncates to ASCII A; the original C char buffer behaves this way.
        heredoc.delimiter.clear();
        assert!(heredoc.scan_start(&mut TestLexer::new("Ł\n")));
        assert_eq!(heredoc.delimiter, b"A\0");
        assert!(heredoc.scan_end_identifier(&mut TestLexer::new("A\n")));
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
