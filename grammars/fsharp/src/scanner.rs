//! F# external scanner, translated from fsharp/common/scanner.h.
//!
//! Indentation and preprocessor stacks retain C's narrow snapshot format.
//! Lookahead probes intentionally keep their advances (and mark_end calls):
//! later fallbacks in the same scan observe the position left by failed probes.

use ts_port_tables::{ExternalScanner, Lexer, SERIALIZATION_BUFFER_SIZE};

const NEWLINE: usize = 0;
const INDENT: usize = 1;
const DEDENT: usize = 2;
const THEN: usize = 3;
const ELSE: usize = 4;
const ELIF: usize = 5;
const PREPROC_IF: usize = 6;
const PREPROC_ELSE: usize = 7;
const PREPROC_END: usize = 8;
const CLASS: usize = 9;
const BEGIN: usize = 10;
const STRUCT: usize = 11;
const INTERFACE: usize = 12;
const END: usize = 13;
const AND: usize = 14;
const WITH: usize = 15;
const TRIPLE_QUOTE_CONTENT: usize = 16;
const FORMAT_TRIPLE_QUOTE_CONTENT: usize = 17;
const BLOCK_COMMENT_CONTENT: usize = 18;
const INSIDE_STRING: usize = 19;
const NEWLINE_NO_ALIGNED: usize = 20;
const TUPLE_MARKER: usize = 21;
const QUOTED_CLOSE: usize = 22;
const UNTYPED_QUOTED_CLOSE: usize = 23;
const MULTI_DOLLAR_TRIPLE_QUOTE_START: usize = 24;
const MULTI_DOLLAR_TRIPLE_QUOTED_CONTENT: usize = 25;
const MULTI_DOLLAR_INTERP_START: usize = 26;
const MULTI_DOLLAR_INTERP_END: usize = 27;
const MULTI_DOLLAR_TRIPLE_QUOTE_END: usize = 28;
const TYAPP_OPEN: usize = 29;
const PAREN_INDENT: usize = 30;
const TYPE_APP_INDENT: usize = 31;
const TYPE_DECL_NEWLINE: usize = 32;
const IN: usize = 33;
const DO_KEYWORD: usize = 34;
const TRY_INDENT: usize = 35;
const PREPROC_INACTIVE: usize = 36;
const ELEM_SEP: usize = 37;
const BRACE_INDENT: usize = 38;
const ERROR_SENTINEL: usize = 39;

const INDENT_NORMAL: u8 = 0;
const INDENT_PAREN: u8 = 1;
const INDENT_TYPE_APP: u8 = 2;
const INDENT_TRY: u8 = 3;
const INDENT_BRACE: u8 = 4;
const INDENT_KIND_MIDLINE_FLAG: u8 = 0x80;
const INDENT_KIND_STRANDED_LINE_FLAG: u8 = 0x40;
const INDENT_KIND_FLAGS_MASK: u8 = 0xc0;
const PREPROC_STRUCTURED: u8 = 0;
const PREPROC_STRAY: u8 = 1;

#[derive(Default)]
pub(crate) struct Scanner {
    indents: Vec<u16>,
    indent_kinds: Vec<u8>,
    preprocessor_indents: Vec<u16>,
    preproc_kinds: Vec<u8>,
    line_stranded: u8,
    multi_dollar_count: u8,
    stranded_dedent: u8,
}

fn advance(lexer: &mut dyn Lexer) {
    lexer.advance(false);
}
fn skip(lexer: &mut dyn Lexer) {
    lexer.advance(true);
}
fn is_word_char(c: i32) -> bool {
    // ASCII letters/digits, underscore and apostrophe (not Unicode categories).
    matches!(c, 97..=122 | 65..=90 | 48..=57 | 95 | 39)
}

impl Scanner {
    fn push_indent(&mut self, length: u32, kind: u8) {
        self.indents.push(length as u16);
        self.indent_kinds.push(kind);
    }
    fn pop_indent(&mut self) {
        self.indents.pop();
        self.indent_kinds.pop();
    }
    // Widen the stored column for the scanner's u32 column comparisons.
    fn peek_indent_length(&self) -> u32 {
        u32::from(*self.indents.last().unwrap())
    }
    fn push_preproc_kind(&mut self, kind: u8) {
        self.preproc_kinds.push(kind);
    }
    fn pop_preproc_kind(&mut self) {
        self.preproc_kinds.pop();
    }
    fn top_preproc_is_stray(&self) -> bool {
        self.preproc_kinds.last() == Some(&PREPROC_STRAY)
    }
    fn top_preproc_is_structured(&self) -> bool {
        self.preproc_kinds.last() == Some(&PREPROC_STRUCTURED)
    }
    fn peek_indent_kind(&self) -> u8 {
        self.indent_kinds.last().copied().unwrap_or(INDENT_NORMAL) & !INDENT_KIND_FLAGS_MASK
    }
    fn top_indent_is_midline_anchor(&self) -> bool {
        self.indent_kinds
            .last()
            .is_some_and(|k| k & INDENT_KIND_MIDLINE_FLAG != 0)
    }
    fn top_indent_is_stranded_line(&self) -> bool {
        self.indent_kinds
            .last()
            .is_some_and(|k| k & INDENT_KIND_STRANDED_LINE_FLAG != 0)
    }
    fn peek_is_paren_indent(&self) -> bool {
        matches!(self.peek_indent_kind(), INDENT_PAREN | INDENT_TYPE_APP)
    }
    fn peek_is_brace_indent(&self) -> bool {
        self.peek_indent_kind() == INDENT_BRACE
    }
    fn peek_is_type_app_indent(&self) -> bool {
        self.peek_indent_kind() == INDENT_TYPE_APP
    }
    fn peek_is_try_indent(&self) -> bool {
        self.peek_indent_kind() == INDENT_TRY
    }
    fn try_dedent_for_preproc(&mut self, lexer: &mut dyn Lexer) -> bool {
        if let (Some(&indent), Some(&preproc)) =
            (self.indents.last(), self.preprocessor_indents.last())
            && preproc < indent
        {
            self.pop_indent();
            lexer.set_result_symbol(DEDENT as u16);
            return true;
        }
        false
    }
}

// Consume a stray #else through its matching #endif, leaving the final newline.
fn swallow_inactive_region(lexer: &mut dyn Lexer) {
    let mut depth = 1;
    loop {
        while lexer.lookahead() != b'\n' as i32 && !lexer.eof() {
            advance(lexer);
        }
        if lexer.eof() {
            return;
        }
        advance(lexer);
        while matches!(lexer.lookahead(), 32 | 9 | 13) {
            advance(lexer);
        }
        if lexer.lookahead() != b'#' as i32 {
            continue;
        }
        advance(lexer);
        if lexer.lookahead() == b'i' as i32 {
            advance(lexer);
            if lexer.lookahead() == b'f' as i32 {
                advance(lexer);
                if !is_word_char(lexer.lookahead()) {
                    depth += 1;
                }
            }
        } else if lexer.lookahead() == b'e' as i32 {
            advance(lexer);
            if match_keyword_rest(lexer, b"ndif") {
                depth -= 1;
                if depth == 0 {
                    while lexer.lookahead() != b'\n' as i32 && !lexer.eof() {
                        advance(lexer);
                    }
                    return;
                }
            }
        }
    }
}

// Called after '<'. The matching '>' remains the current lookahead. Newline
// state is only written on success, just like C's optional out parameter.
fn is_type_application_open_ex(lexer: &mut dyn Lexer, out_saw_newline: &mut bool) -> bool {
    let mut angle_depth = 1;
    let mut paren_depth = 0;
    let mut saw_newline = false;
    let mut prev_was_caret = false;
    while !lexer.eof() && angle_depth > 0 {
        let c = lexer.lookahead();
        let after_caret = prev_was_caret;
        prev_was_caret = false;
        if matches!(c, 10 | 13) {
            saw_newline = true;
            advance(lexer);
            continue;
        }
        // Spacing and , * . : # ^ / | { } [ ] ` % are type-argument-shaped.
        if is_word_char(c)
            || matches!(
                c,
                32 | 9 | 44 | 42 | 46 | 58 | 35 | 94 | 47 | 124 | 123 | 125 | 91 | 93 | 96 | 37
            )
        {
            prev_was_caret = c == b'^' as i32;
            advance(lexer);
            continue;
        }
        match c {
            40 => {
                paren_depth += 1;
                advance(lexer);
            }
            41 => {
                if paren_depth <= 0 {
                    return false;
                }
                paren_depth -= 1;
                advance(lexer);
            }
            60 => {
                angle_depth += 1;
                advance(lexer);
            }
            62 => {
                angle_depth -= 1;
                if angle_depth == 0 {
                    if paren_depth > 0 {
                        return false;
                    }
                    *out_saw_newline = saw_newline;
                    return true;
                }
                advance(lexer);
            }
            45 => {
                advance(lexer);
                if lexer.lookahead() == b'>' as i32 {
                    advance(lexer);
                    continue;
                }
                if (after_caret || paren_depth > 0) && matches!(lexer.lookahead(), 48..=57) {
                    continue;
                }
                return false;
            }
            _ => return false,
        }
    }
    false
}
fn is_type_application_open(lexer: &mut dyn Lexer) -> bool {
    is_type_application_open_ex(lexer, &mut false)
}
fn is_multiline_type_app_ahead(lexer: &mut dyn Lexer) -> bool {
    let mut saw_newline = false;
    is_type_application_open_ex(lexer, &mut saw_newline) && saw_newline
}

fn is_srtp_typar_group_ahead(lexer: &mut dyn Lexer) -> bool {
    let mut typars = 0u32;
    loop {
        while matches!(lexer.lookahead(), 32 | 9) {
            advance(lexer);
        }
        if !matches!(lexer.lookahead(), 94 | 39) {
            return false;
        }
        advance(lexer);
        if !is_word_char(lexer.lookahead()) {
            return false;
        }
        while is_word_char(lexer.lookahead()) {
            advance(lexer);
        }
        typars = typars.wrapping_add(1);
        while matches!(lexer.lookahead(), 32 | 9) {
            advance(lexer);
        }
        if lexer.lookahead() != b'o' as i32 {
            break;
        }
        advance(lexer);
        if lexer.lookahead() != b'r' as i32 {
            return false;
        }
        advance(lexer);
        if is_word_char(lexer.lookahead()) {
            return false;
        }
    }
    if typars < 2 || lexer.lookahead() != b')' as i32 {
        return false;
    }
    advance(lexer);
    while matches!(lexer.lookahead(), 32 | 9) {
        advance(lexer);
    }
    if lexer.lookahead() != b':' as i32 {
        return false;
    }
    advance(lexer);
    while matches!(lexer.lookahead(), 32 | 9) {
        advance(lexer);
    }
    lexer.lookahead() == b'(' as i32
}

fn scan_n_chars(lexer: &mut dyn Lexer, ch: i32, count: u8) -> bool {
    lexer.mark_end();
    for _ in 0..count {
        if lexer.lookahead() != ch {
            return false;
        }
        advance(lexer);
    }
    lexer.mark_end();
    true
}

fn scan_block_comment(lexer: &mut dyn Lexer) -> bool {
    lexer.mark_end();
    if lexer.lookahead() != b'(' as i32 {
        return false;
    }
    advance(lexer);
    if lexer.lookahead() != b'*' as i32 {
        return false;
    }
    advance(lexer);
    // Replace recursion with a depth counter, retaining each recursive call's
    // mark_end and its failed-opener advance, including on a non-comment '('.
    let mut depth = 1usize;
    loop {
        match lexer.lookahead() {
            40 => {
                lexer.mark_end();
                advance(lexer);
                if lexer.lookahead() == b'*' as i32 {
                    advance(lexer);
                    depth += 1;
                }
            }
            42 => {
                advance(lexer);
                if lexer.lookahead() == b')' as i32 {
                    advance(lexer);
                    depth -= 1;
                    if depth == 0 {
                        return true;
                    }
                }
            }
            0 => return true,
            _ => advance(lexer),
        }
    }
}

fn is_infix_op_start(lexer: &mut dyn Lexer) -> bool {
    match lexer.lookahead() {
        43 | 45 => {
            skip(lexer);
            !matches!(lexer.lookahead(), 48..=57)
        }
        42 | 37 | 38 | 61 | 63 | 60 | 62 | 94 | 44 => true,
        47 => {
            skip(lexer);
            lexer.lookahead() != b'/' as i32
        }
        46 => {
            skip(lexer);
            lexer.lookahead() != b'.' as i32
        }
        33 => {
            skip(lexer);
            lexer.lookahead() == b'=' as i32
        }
        58 => {
            skip(lexer);
            matches!(lexer.lookahead(), 61 | 58 | 63 | 32 | 62)
        }
        111 => {
            skip(lexer);
            if lexer.lookahead() != b'r' as i32 {
                return false;
            }
            skip(lexer);
            !is_word_char(lexer.lookahead())
        }
        64 | 36 => {
            skip(lexer);
            lexer.lookahead() != b'"' as i32
        }
        _ => false,
    }
}
fn is_bracket_end(lexer: &dyn Lexer) -> bool {
    matches!(lexer.lookahead(), 41 | 93 | 125)
}
fn match_keyword_rest(lexer: &mut dyn Lexer, rest: &[u8]) -> bool {
    for &ch in rest {
        if lexer.lookahead() != i32::from(ch) {
            return false;
        }
        advance(lexer);
    }
    !is_word_char(lexer.lookahead())
}

const BLOCK_OPENERS: &[(u8, &[u8], usize)] = &[
    (b'c', b"lass", CLASS),
    (b'b', b"egin", BEGIN),
    (b's', b"truct", STRUCT),
    (b'i', b"nterface", INTERFACE),
];

impl ExternalScanner for Scanner {
    fn scan(&mut self, lexer: &mut dyn Lexer, valid_symbols: &[bool]) -> bool {
        // A stranded-dedent flag lives for exactly one scan: capture it and clear
        // the persistent copy up front so it is consumed by the immediately
        // following scan (the NEWLINE emit below) and never leaks further.
        let prev_stranded_dedent = self.stranded_dedent != 0;
        self.stranded_dedent = 0;

        if valid_symbols[ERROR_SENTINEL] {
            // During error recovery, all valid_symbols are true. Tree-sitter's error
            // recovery mechanism cannot emit external scanner tokens, so we must still
            // produce tokens like DEDENT and PREPROC_END when we can identify them.
            // This enables partial parse tree recovery -- e.g., "match x with" needs
            // DEDENT to be recognized as a partially correct match-statement for
            // syntax highlighting purposes.

            // At EOF, emit DEDENT to drain the indent stack. This is critical for
            // closing partial parse trees at end of input.
            if lexer.eof() && self.indents.len() > 1 {
                self.pop_indent();
                lexer.set_result_symbol(DEDENT as u16);
                return true;
            }

            // For non-EOF cases, fall through to normal scanning logic below.
            // The normal path handles whitespace consumption and emits DEDENT/NEWLINE
            // based on actual indentation levels. Features that should not run during
            // error recovery (multi-dollar strings, quotation closers, etc.) are
            // already guarded by !valid_symbols[ERROR_SENTINEL] checks.
        }

        if valid_symbols[INSIDE_STRING] && !valid_symbols[ERROR_SENTINEL] {
            return false;
        }

        // `preproc_inactive` is an extra, so it is valid in nearly every state —
        // including states that had no valid external tokens before it existed and
        // therefore never invoked this scanner. The full scan logic below assumes
        // some structural token is wanted (several paths emit DEDENT/NEWLINE
        // unconditionally), so in those states handle only the stray-directive
        // fallback and otherwise stay out of the internal lexer's way.
        if !valid_symbols[ERROR_SENTINEL] {
            let mut any_structural_valid = false;
            for &valid in &valid_symbols[..PREPROC_INACTIVE] {
                if valid {
                    any_structural_valid = true;
                    break;
                }
            }
            // BRACE_INDENT sits after the PREPROC_INACTIVE/ELEM_SEP "extra" tokens in
            // the enum but is a structural indent token like INDENT, so it must count
            // here — otherwise a state where only BRACE_INDENT is valid (right after a
            // record/CE '{') bails out and the token is never emitted.
            if valid_symbols[BRACE_INDENT] {
                any_structural_valid = true;
            }
            if !any_structural_valid {
                while lexer.lookahead() == b' ' as i32
                    || lexer.lookahead() == b'\t' as i32
                    || lexer.lookahead() == b'\n' as i32
                    || lexer.lookahead() == b'\r' as i32
                    || lexer.lookahead() == b'\x0c' as i32
                {
                    skip(lexer);
                }
                if lexer.lookahead() != b'#' as i32 {
                    return false;
                }
                advance(lexer);
                if lexer.lookahead() == b'i' as i32 {
                    // #if — grammar cannot place it here
                    advance(lexer);
                    if lexer.lookahead() != b'f' as i32 {
                        return false;
                    }
                    advance(lexer);
                    if is_word_char(lexer.lookahead()) {
                        return false;
                    }
                    self.push_preproc_kind(PREPROC_STRAY);
                    while lexer.lookahead() != b'\n' as i32 && !lexer.eof() {
                        advance(lexer);
                    }
                    lexer.mark_end();
                    lexer.set_result_symbol(PREPROC_INACTIVE as u16);
                    return true;
                }
                if lexer.lookahead() != b'e' as i32 {
                    return false;
                }
                advance(lexer);
                if lexer.lookahead() == b'l' as i32 {
                    // #else of a stray directive
                    advance(lexer);
                    if lexer.lookahead() != b's' as i32 {
                        return false;
                    }
                    advance(lexer);
                    if lexer.lookahead() != b'e' as i32 {
                        return false;
                    }
                    advance(lexer);
                    if is_word_char(lexer.lookahead()) || !self.top_preproc_is_stray() {
                        return false;
                    }
                    self.pop_preproc_kind();
                    swallow_inactive_region(lexer);
                    lexer.mark_end();
                    lexer.set_result_symbol(PREPROC_INACTIVE as u16);
                    return true;
                }
                if lexer.lookahead() == b'n' as i32 {
                    // #endif of a stray directive
                    advance(lexer);
                    if lexer.lookahead() != b'd' as i32 {
                        return false;
                    }
                    advance(lexer);
                    if lexer.lookahead() != b'i' as i32 {
                        return false;
                    }
                    advance(lexer);
                    if lexer.lookahead() != b'f' as i32 {
                        return false;
                    }
                    advance(lexer);
                    if is_word_char(lexer.lookahead()) || !self.top_preproc_is_stray() {
                        return false;
                    }
                    self.pop_preproc_kind();
                    lexer.mark_end();
                    lexer.set_result_symbol(PREPROC_INACTIVE as u16);
                    return true;
                }
                return false;
            }
        }

        // Type application '<' disambiguation (F# spec Section 15.3).
        // When the grammar expects TYAPP_OPEN (i.e., a '<' immediately after an expression),
        // peek ahead to determine if the content between '<' and '>' looks like type arguments.
        // If not (e.g., it's a comparison operator like l<r), return false so the grammar
        // falls through to infix_op.
        if valid_symbols[TYAPP_OPEN]
            && !valid_symbols[ERROR_SENTINEL]
            && lexer.lookahead() == b'<' as i32
        {
            lexer.mark_end();
            advance(lexer);
            // Mark end right after '<' - this is what we want the token to contain
            lexer.mark_end();
            // Now peek ahead (advancing further) to check if content looks like type args.
            // Even though we advance past the type content, mark_end is already set to
            // just after '<', so the emitted token will be exactly '<'.
            if is_type_application_open(lexer) {
                lexer.set_result_symbol(TYAPP_OPEN as u16);
                return true;
            }
            // Not a type application - don't consume the '<', let grammar handle it as infix_op.
            // But we already advanced past '<' and potentially more. That's OK because
            // we return false and tree-sitter will reset the lexer position.
            return false;
        }

        if !valid_symbols[ERROR_SENTINEL] && self.multi_dollar_count > 1 {
            if valid_symbols[MULTI_DOLLAR_INTERP_START]
                && lexer.lookahead() == b'{' as i32
                && scan_n_chars(lexer, b'{' as i32, self.multi_dollar_count)
            {
                lexer.set_result_symbol(MULTI_DOLLAR_INTERP_START as u16);
                return true;
            }

            if valid_symbols[MULTI_DOLLAR_INTERP_END]
                && lexer.lookahead() == b'}' as i32
                && scan_n_chars(lexer, b'}' as i32, self.multi_dollar_count)
            {
                lexer.set_result_symbol(MULTI_DOLLAR_INTERP_END as u16);
                return true;
            }

            if valid_symbols[MULTI_DOLLAR_TRIPLE_QUOTE_END]
                && lexer.lookahead() == b'"' as i32
                && scan_n_chars(lexer, b'"' as i32, 3)
            {
                self.multi_dollar_count = 0;
                lexer.set_result_symbol(MULTI_DOLLAR_TRIPLE_QUOTE_END as u16);
                return true;
            }
        }

        if !valid_symbols[ERROR_SENTINEL]
            && (valid_symbols[TRIPLE_QUOTE_CONTENT]
                || valid_symbols[FORMAT_TRIPLE_QUOTE_CONTENT]
                || valid_symbols[MULTI_DOLLAR_TRIPLE_QUOTED_CONTENT])
        {
            let is_format = valid_symbols[FORMAT_TRIPLE_QUOTE_CONTENT];
            let is_multi_dollar = valid_symbols[MULTI_DOLLAR_TRIPLE_QUOTED_CONTENT];
            let mut has_content = false;
            lexer.mark_end();
            loop {
                if lexer.lookahead() == b'\0' as i32 {
                    break;
                }
                if (is_format || is_multi_dollar) && lexer.lookahead() == b'{' as i32 {
                    // In format triple-quoted strings, stop at '{' to allow interpolation.
                    // Multi-dollar interpolated strings require N braces, where N is the
                    // number of leading '$' characters.
                    let brace_count = if is_multi_dollar {
                        self.multi_dollar_count
                    } else {
                        1
                    };
                    lexer.mark_end();

                    if !is_multi_dollar {
                        advance(lexer);
                        if lexer.lookahead() == b'{' as i32 {
                            advance(lexer);
                            lexer.mark_end();
                            has_content = true;
                            continue;
                        }
                        if !has_content {
                            return false;
                        }
                        break;
                    }

                    let mut matches_interp_start = true;
                    for _ in 0..brace_count {
                        if lexer.lookahead() != b'{' as i32 {
                            matches_interp_start = false;
                            break;
                        }
                        advance(lexer);
                    }
                    if matches_interp_start {
                        if !has_content {
                            return false;
                        }
                        break;
                    }
                    has_content = true;
                    lexer.mark_end();
                    continue;
                }
                if lexer.lookahead() != b'"' as i32 {
                    advance(lexer);
                    has_content = true;
                } else {
                    if is_multi_dollar {
                        advance(lexer);
                        if lexer.lookahead() == b'"' as i32 {
                            advance(lexer);
                            if lexer.lookahead() == b'"' as i32 {
                                break;
                            }
                        }
                        lexer.mark_end();
                    } else {
                        lexer.mark_end();
                        skip(lexer);
                        if lexer.lookahead() == b'"' as i32 {
                            skip(lexer);
                            if lexer.lookahead() == b'"' as i32 {
                                skip(lexer);
                                break;
                            }
                        }
                    }
                    has_content = true;
                    lexer.mark_end();
                }
            }
            if is_multi_dollar {
                lexer.set_result_symbol(MULTI_DOLLAR_TRIPLE_QUOTED_CONTENT as u16);
            } else {
                lexer.set_result_symbol(
                    (if is_format {
                        FORMAT_TRIPLE_QUOTE_CONTENT
                    } else {
                        TRIPLE_QUOTE_CONTENT
                    }) as u16,
                );
            }
            return true;
        }

        if valid_symbols[TYPE_DECL_NEWLINE] && !valid_symbols[ERROR_SENTINEL] {
            // Only fire at EOF or newline; if the current character is something else
            // (e.g. '=' during GLR exploration), fall through to general scanning —
            // the lexer position is unchanged so this is safe.
            if lexer.eof() {
                lexer.set_result_symbol(TYPE_DECL_NEWLINE as u16);
                return true;
            }
            if lexer.lookahead() == b'\n' as i32 || lexer.lookahead() == b'\r' as i32 {
                // Emit a zero-width token: fix the token end at the newline so the newline
                // itself is NOT consumed. This matters inside module bodies, where the
                // newline is the separator between elements; if TYPE_DECL_NEWLINE ate it,
                // the module would close after a single bare type declaration.
                lexer.mark_end();
                // Peek ahead: skip newlines/whitespace to find indentation of next content.
                // If next content is NOT more indented than current scope, this is a bare
                // type declaration (e.g. [<Measure>] type Dollars).
                // If next content IS more indented, the type has a body (e.g. type CsvFile
                //   private (...) = ...) and TYPE_DECL_NEWLINE should not fire.
                let mut next_indent = 0u32;
                loop {
                    if lexer.lookahead() == b'\n' as i32 || lexer.lookahead() == b'\r' as i32 {
                        next_indent = 0;
                        skip(lexer);
                    } else if lexer.lookahead() == b' ' as i32 {
                        next_indent = next_indent.wrapping_add(1);
                        skip(lexer);
                    } else if lexer.lookahead() == b'\t' as i32 {
                        next_indent = next_indent.wrapping_add(8);
                        skip(lexer);
                    } else {
                        break;
                    }
                }
                if lexer.eof() {
                    lexer.set_result_symbol(TYPE_DECL_NEWLINE as u16);
                    return true;
                }
                let scope_indent = u32::from(self.indents.last().copied().unwrap_or(0));
                if next_indent <= scope_indent {
                    // Next line is at same or lower indentation — bare type declaration
                    lexer.set_result_symbol(TYPE_DECL_NEWLINE as u16);
                    return true;
                }
                // else: next line is more indented — type has a body, don't fire
                return false;
            }
        }

        // Block-comment content must be handled before the whitespace/offside walk
        // below. It is only ever valid immediately after a '(*' opener, so nothing
        // else needs scanning here; running the ws-walk instead lets a '#' that is
        // the first non-space char of the comment (e.g. a Markdown '(* # Heading')
        // be mis-read as a preprocessor directive, breaking the comment.
        if valid_symbols[BLOCK_COMMENT_CONTENT] && !valid_symbols[ERROR_SENTINEL] {
            // Scan position is directly after a shifted '(*'. If the very next char
            // is ')', the source text was `(*)` — F# defines that as the
            // multiplication operator reference, never a comment (matching FSC's
            // lexer). Decline so the GLR version that lexed '(*' as a comment opener
            // dies immediately instead of swallowing an arbitrary span hunting for
            // '*)' (`Constant(Checked.(*) l r, t)` arms repeated in one match were
            // compounding such zombie versions past the GLR version cap, killing the
            // correct parse — ExpressionOptimizer.fs whole-file wrap).
            if lexer.lookahead() == b')' as i32 {
                return false;
            }
            lexer.mark_end();
            loop {
                if lexer.lookahead() == b'\0' as i32 {
                    break;
                }
                if lexer.lookahead() != b'(' as i32 && lexer.lookahead() != b'*' as i32 {
                    advance(lexer);
                } else if lexer.lookahead() == b'*' as i32 {
                    lexer.mark_end();
                    advance(lexer);
                    if lexer.lookahead() == b')' as i32 {
                        break;
                    }
                } else if scan_block_comment(lexer) {
                    lexer.mark_end();
                    advance(lexer);
                    if lexer.lookahead() == b'*' as i32 {
                        break;
                    }
                }
            }
            lexer.set_result_symbol(BLOCK_COMMENT_CONTENT as u16);
            return true;
        }

        lexer.mark_end();

        let mut found_end_of_line = false;
        let mut found_end_of_line_semi_colon = false;
        let mut found_start_of_infix_op = false;
        let mut found_same_line_pipe_infix = false;
        let mut found_bracket_end = false;
        let mut found_preprocessor_end = false;
        let mut found_preproc_if = false;
        let mut found_preproc_else = false;
        let mut found_comment_start = false;
        let mut advanced_in_ws_walk = false;
        let mut skipped_open_paren = false;
        let mut indent_length = lexer.get_column();

        loop {
            if lexer.lookahead() == b'\n' as i32 {
                found_end_of_line = true;
                indent_length = 0;
                skip(lexer);
            } else if lexer.lookahead() == b' ' as i32 {
                indent_length = indent_length.wrapping_add(1);
                skip(lexer);
            } else if lexer.lookahead() == b'\r' as i32 || lexer.lookahead() == b'\x0c' as i32 {
                indent_length = 0;
                skip(lexer);
            } else if lexer.lookahead() == b'\t' as i32 {
                indent_length = indent_length.wrapping_add(8);
                skip(lexer);
            } else if lexer.eof() {
                found_end_of_line = true;
                indent_length = 0;
                break;
            } else if lexer.lookahead() == b'/' as i32 {
                skip(lexer);
                if !valid_symbols[INSIDE_STRING] && lexer.lookahead() == b'/' as i32 {
                    // Once the loop has scanned past a directive (`#endif`/`#else` that
                    // couldn't be emitted yet, or a `#if` line), declining on a trailing
                    // line comment would leave the directive — not the comment — as the
                    // next text for the internal lexer, which cannot lex it (ERROR).
                    // Skip the comment like whitespace instead, exactly as if the line
                    // were blank, so the pending NEWLINE/DEDENT decision proceeds.
                    if !found_preproc_if && !found_preprocessor_end && !found_preproc_else {
                        return false;
                    }
                    while lexer.lookahead() != b'\n' as i32 && !lexer.eof() {
                        skip(lexer);
                    }
                } else {
                    return false;
                }
            } else if lexer.lookahead() == b'#' as i32 {
                advanced_in_ws_walk = true;
                advance(lexer);
                if lexer.lookahead() == b'e' as i32 {
                    advance(lexer);
                    if lexer.lookahead() == b'n' as i32 {
                        advance(lexer);
                        if lexer.lookahead() == b'd' as i32 {
                            advance(lexer);
                            if lexer.lookahead() == b'i' as i32 {
                                advance(lexer);
                                if lexer.lookahead() == b'f' as i32 {
                                    advance(lexer);
                                    // The innermost open directive was consumed as trivia (no
                                    // grammar rule at its position), so this `#endif` closes it
                                    // textually: consume it as an inactive-trivia extra instead
                                    // of handing it to the grammar.
                                    if self.top_preproc_is_stray()
                                        && !valid_symbols[ERROR_SENTINEL]
                                        && !is_word_char(lexer.lookahead())
                                    {
                                        self.pop_preproc_kind();
                                        lexer.mark_end();
                                        lexer.set_result_symbol(PREPROC_INACTIVE as u16);
                                        return true;
                                    }
                                    found_preprocessor_end = true;
                                    if self.try_dedent_for_preproc(lexer) {
                                        return true;
                                    }
                                    if valid_symbols[PREPROC_END] {
                                        if !self.preprocessor_indents.is_empty() {
                                            self.preprocessor_indents.pop();
                                        }
                                        self.pop_preproc_kind();
                                        lexer.mark_end();
                                        lexer.set_result_symbol(PREPROC_END as u16);
                                        return true;
                                    }
                                }
                            }
                        }
                    } else if lexer.lookahead() == b'l' as i32 {
                        advance(lexer);
                        if lexer.lookahead() == b's' as i32 {
                            advance(lexer);
                            if lexer.lookahead() == b'e' as i32 {
                                advance(lexer);
                                // The innermost open directive was consumed as trivia, so its
                                // `#else` branch is inactive: swallow everything through the
                                // matching `#endif` as a single extra token. Only the active
                                // (first) branch reaches the grammar, so a directive the
                                // grammar has no rule for can never split a construct in two.
                                if self.top_preproc_is_stray()
                                    && !valid_symbols[ERROR_SENTINEL]
                                    && !is_word_char(lexer.lookahead())
                                {
                                    self.pop_preproc_kind();
                                    swallow_inactive_region(lexer);
                                    lexer.mark_end();
                                    lexer.set_result_symbol(PREPROC_INACTIVE as u16);
                                    return true;
                                }
                                if self.try_dedent_for_preproc(lexer) {
                                    return true;
                                }
                                if valid_symbols[PREPROC_ELSE] {
                                    lexer.mark_end();
                                    lexer.set_result_symbol(PREPROC_ELSE as u16);
                                    return true;
                                }
                                // Not emittable yet (an enclosing rule must close first):
                                // remember we scanned past it so a trailing line comment
                                // doesn't make the scanner decline (see the '/' case above).
                                found_preproc_else = true;
                            }
                        }
                    }
                } else if lexer.lookahead() == b'i' as i32 {
                    advance(lexer);
                    if lexer.lookahead() == b'n' as i32 {
                        // `#indent "off"` / `#indent "on"`: a legacy verbose-syntax
                        // directive with no grammar rule. Swallow the whole line as
                        // inactive trivia (adding it to the grammar instead costs ~1 MB
                        // of parser tables for two real-world occurrences).
                        if match_keyword_rest(lexer, b"ndent") && !valid_symbols[ERROR_SENTINEL] {
                            while lexer.lookahead() != b'\n' as i32 && !lexer.eof() {
                                advance(lexer);
                            }
                            lexer.mark_end();
                            lexer.set_result_symbol(PREPROC_INACTIVE as u16);
                            return true;
                        }
                        return false;
                    }
                    if lexer.lookahead() == b'f' as i32 {
                        advance(lexer);
                        found_preproc_if = true;
                        // If an indented block is still open above this line AND the first
                        // content line after the '#if' sits outside that block (less
                        // indented), close the block before treating the directive.
                        // The peek below only advances past mark_end (still at the scan
                        // start), so the emitted DEDENT is zero-width and the directive
                        // text is re-read by the next scan — nothing is consumed. Same
                        // technique as is_type_application_open above. Otherwise —
                        // otherwise the skip-line path below starves the parse that needs
                        // the '#if' token after the dedent (e.g. a record '}' followed by
                        // '#if' around a top-level binding). When the branch content stays
                        // at block depth, keep the directive indentation-transparent.
                        if found_end_of_line
                            && valid_symbols[DEDENT]
                            && self.indents.len() > 1
                            && indent_length < self.peek_indent_length()
                            && (!self.peek_is_paren_indent() || indent_length == 0)
                        {
                            while lexer.lookahead() != b'\n' as i32 && !lexer.eof() {
                                advance(lexer);
                            }
                            let mut content_indent = 0u32;
                            let mut has_content = false;
                            while !lexer.eof() {
                                if lexer.lookahead() == b'\n' as i32
                                    || lexer.lookahead() == b'\r' as i32
                                {
                                    content_indent = 0;
                                    advance(lexer);
                                } else if lexer.lookahead() == b' ' as i32 {
                                    content_indent = content_indent.wrapping_add(1);
                                    advance(lexer);
                                } else if lexer.lookahead() == b'\t' as i32 {
                                    content_indent = content_indent.wrapping_add(8);
                                    advance(lexer);
                                } else if lexer.lookahead() == b'/' as i32 {
                                    advance(lexer);
                                    if lexer.lookahead() != b'/' as i32 {
                                        has_content = true;
                                        break;
                                    }
                                    while lexer.lookahead() != b'\n' as i32 && !lexer.eof() {
                                        advance(lexer);
                                    }
                                } else {
                                    has_content = true;
                                    break;
                                }
                            }
                            if has_content && content_indent < self.peek_indent_length() {
                                self.pop_indent();
                                lexer.set_result_symbol(DEDENT as u16);
                                return true;
                            }
                            // Branch content starts with a token that can never begin an
                            // expression or declaration but *continues* the enclosing one
                            // (a fluent-chain '.', a closing bracket, a tuple ','): a
                            // structured branch could not parse it, so consume the directive
                            // line — including the newline and the content line's indent, so
                            // the content continues the previous expression exactly as if
                            // the directive were not there — as inactive trivia, and
                            // remember the stray open so `#else`/`#endif` are consumed
                            // textually too.
                            if has_content
                                && !valid_symbols[ERROR_SENTINEL]
                                && (lexer.lookahead() == b'.' as i32
                                    || lexer.lookahead() == b')' as i32
                                    || lexer.lookahead() == b']' as i32
                                    || lexer.lookahead() == b'}' as i32
                                    || lexer.lookahead() == b',' as i32)
                            {
                                self.push_preproc_kind(PREPROC_STRAY);
                                lexer.mark_end();
                                lexer.set_result_symbol(PREPROC_INACTIVE as u16);
                                return true;
                            }
                            // Branch content stays at block depth: treat the directive lines
                            // as whitespace. The peek already consumed up to the content
                            // char, so resume the whitespace loop from there.
                            found_end_of_line = true;
                            indent_length = content_indent;
                            continue;
                        }
                        if (valid_symbols[NEWLINE] || valid_symbols[INDENT])
                            && !valid_symbols[PREPROC_IF]
                        {
                            while lexer.lookahead() != b'\n' as i32 && !lexer.eof() {
                                skip(lexer);
                            }
                        } else if !valid_symbols[PREPROC_IF]
                            && !valid_symbols[ERROR_SENTINEL]
                            && !(!self.indents.is_empty() && valid_symbols[DEDENT])
                            && !is_word_char(lexer.lookahead())
                        {
                            // The grammar has no preproc rule at this position and no
                            // zero-width token (NEWLINE/INDENT/DEDENT) can change that:
                            // consume the directive line as inactive trivia and remember the
                            // stray open, so the matching `#else`/`#endif` are consumed
                            // textually too instead of reaching the grammar. Only the active
                            // (first) branch is parsed, so a directive the grammar cannot
                            // place never splits a construct in two. Previously this case
                            // emitted a token the parser could not shift, producing an ERROR.
                            self.push_preproc_kind(PREPROC_STRAY);
                            while lexer.lookahead() != b'\n' as i32 && !lexer.eof() {
                                advance(lexer);
                            }
                            lexer.mark_end();
                            lexer.set_result_symbol(PREPROC_INACTIVE as u16);
                            return true;
                        } else {
                            if !self.indents.is_empty() {
                                if valid_symbols[PREPROC_IF] {
                                    if !valid_symbols[ERROR_SENTINEL]
                                        && !is_word_char(lexer.lookahead())
                                    {
                                        // The grammar can adopt the directive here, but peek the
                                        // first branch line: if it starts with a token that can
                                        // never begin an expression or declaration (a fluent-chain
                                        // '.', a match-arm '|', a closing bracket, a tuple ','),
                                        // the structured branch could not parse and would split
                                        // the surrounding construct. Consume the directive line as
                                        // inactive trivia instead: the branch content then parses
                                        // inline as part of the enclosing construct, and a later
                                        // stray `#else` swallows the alternative branch.
                                        while lexer.lookahead() != b'\n' as i32 && !lexer.eof() {
                                            advance(lexer);
                                        }
                                        lexer.mark_end();
                                        let mut branch_start = 0;
                                        while !lexer.eof() {
                                            if lexer.lookahead() == b'\n' as i32
                                                || lexer.lookahead() == b'\r' as i32
                                                || lexer.lookahead() == b' ' as i32
                                                || lexer.lookahead() == b'\t' as i32
                                            {
                                                advance(lexer);
                                            } else if lexer.lookahead() == b'/' as i32 {
                                                advance(lexer);
                                                if lexer.lookahead() != b'/' as i32 {
                                                    branch_start = b'/' as i32;
                                                    break;
                                                }
                                                while lexer.lookahead() != b'\n' as i32
                                                    && !lexer.eof()
                                                {
                                                    advance(lexer);
                                                }
                                            } else {
                                                branch_start = lexer.lookahead();
                                                break;
                                            }
                                        }
                                        if self.top_preproc_is_structured()
                                            || branch_start == b'.' as i32
                                            || branch_start == b'|' as i32
                                            || branch_start == b')' as i32
                                            || branch_start == b']' as i32
                                            || branch_start == b'}' as i32
                                            || branch_start == b',' as i32
                                        {
                                            // A `#if` nested inside a structured directive's active
                                            // branch is consumed as stray trivia: structured-inside-
                                            // structured mispairs the OUTER `#endif` (the nested
                                            // adoption steals the close; DataContext's indented
                                            // sibling `#if VENDOR` blocks, ProvidedTypes' nested
                                            // record/class directives). Inlining the active branch
                                            // keeps the enclosing books balanced.
                                            self.push_preproc_kind(PREPROC_STRAY);
                                            lexer.set_result_symbol(PREPROC_INACTIVE as u16);
                                            return true;
                                        }
                                        // Branch content is expression-shaped: decline, so the
                                        // internal lexer adopts `#if` structurally. (The peek
                                        // advanced past mark_end only; declining returns it all.)
                                        return false;
                                    }
                                    let current_indent_length = self.peek_indent_length();
                                    self.preprocessor_indents.push(current_indent_length as u16);
                                    self.push_preproc_kind(PREPROC_STRUCTURED);
                                } else {
                                    self.pop_indent();
                                    lexer.set_result_symbol(DEDENT as u16);
                                    return true;
                                }
                            } else {
                                self.push_preproc_kind(PREPROC_STRUCTURED);
                                lexer.mark_end();
                                lexer.set_result_symbol(PREPROC_IF as u16);
                                return true;
                            }
                        }
                    }
                } else {
                    if found_end_of_line {
                        if valid_symbols[NEWLINE_NO_ALIGNED] {
                            lexer.set_result_symbol(NEWLINE_NO_ALIGNED as u16);
                            return true;
                        }
                        // '#line N' / '#N' / '#light' are extras, transparent for
                        // indentation. Returning false resets the lexer to the scan start
                        // (chars advanced past mark_end are returned to the input), so the
                        // internal lexer re-reads and consumes the directive in place.
                        if lexer.lookahead() == b'l' as i32 {
                            advance(lexer);
                            if lexer.lookahead() == b'i' as i32 {
                                return false;
                            }
                        } else if lexer.lookahead() >= b'0' as i32
                            && lexer.lookahead() <= b'9' as i32
                        {
                            return false;
                        }
                        // Other directive lines (#r, #load, #nowarn, ...) are real syntax
                        // nodes: fall through so an open indented block can DEDENT before
                        // the directive token is lexed internally. Flag like a preproc line
                        // so INDENT/NEWLINE stay suppressed (INDENT must fire at the next
                        // real line instead).
                        found_preprocessor_end = true;
                        break;
                    }
                    return false;
                }
            } else {
                break;
            }
        }

        // We crossed onto a new line: it is not (yet) known to be stranded. The
        // stranded-NEWLINE emission below re-sets the flag in the same scan when
        // this line does hang between levels.
        if found_end_of_line {
            self.line_stranded = 0;
        }

        // Top-level module-element separator, phase 1 of 2 (arm): a next line at
        // column 0 while only the base indent level is open separates module
        // elements, so an application expression cannot absorb the next element
        // (`f 1` followed by `let g ...` otherwise parses `let` as a let-in
        // argument with a MISSING body). Arming must happen here — at the end of
        // the whitespace walk, before any probe below moves the lexer — because
        // the token must CONSUME the walked newline: mark_end lands past it, so
        // an emission always makes progress. That is load-bearing: ELEM_SEP sits
        // in a repeat position, and a zero-width token emitted before the newline
        // can be shifted again at the same position forever, allocating without
        // bound. The EMISSION happens at phase 2, after the keyword probes, so
        // `and`/`then`/... keep their priority over the separator — a probe that
        // fires sets its own mark_end, and a probe that declines only advanced
        // past this mark (rolled back on emission). The next-line check is a
        // single-lookahead whitelist: only lines that begin like a fresh
        // declaration or expression (word char, '[' for attributes or lists,
        // '"') arm; anything else — a '|' union case or match arm, an infix or
        // fluent continuation, a closing bracket, a comment, a directive — takes
        // the normal paths. A blocked separator merely reverts that line pair to
        // the old absorbed-application parse; a wrong separator would split a
        // valid construct, so the whitelist errs toward blocking. Never at EOF
        // (nothing to separate), never after in-walk advances (the consumed span
        // must be pure walked whitespace), never during error recovery (recovery
        // marks every symbol valid regardless of grammar position). Accepted
        // side effect of arming: a zero-width token emitted below in an armed
        // scan (e.g. INDENT) carries this mark_end too and consumes the newline
        // as padding — same parse, slightly shifted extents.
        let mut elem_sep_armed = false;
        if valid_symbols[ELEM_SEP]
            && !valid_symbols[ERROR_SENTINEL]
            && found_end_of_line
            && indent_length == 0
            && !lexer.eof()
            && self.indents.len() == 1
            && !advanced_in_ws_walk
            && !found_preprocessor_end
            && !found_preproc_if
            && !found_preproc_else
            && (is_word_char(lexer.lookahead())
                || lexer.lookahead() == b'[' as i32
                || lexer.lookahead() == b'"' as i32)
        {
            elem_sep_armed = true;
            lexer.mark_end();
        }

        // Handle @> and @@> as external tokens to prevent them from being
        // tokenized as infix operators inside quotation expressions.
        // If a dedent is pending for a same-line expression block (e.g. fun x -> x),
        // emit the dedent first and leave the quote closer for the next scan.
        let mut failed_at_sign_match = false;
        if !valid_symbols[ERROR_SENTINEL] && lexer.lookahead() == b'@' as i32 {
            lexer.mark_end();
            advance(lexer);
            if lexer.lookahead() == b'@' as i32 {
                advance(lexer);
                if lexer.lookahead() == b'>' as i32 {
                    if valid_symbols[DEDENT] && self.indents.len() > 1 {
                        self.pop_indent();
                        lexer.set_result_symbol(DEDENT as u16);
                        return true;
                    }
                    advance(lexer);
                    lexer.mark_end();
                    lexer.set_result_symbol(UNTYPED_QUOTED_CLOSE as u16);
                    return true;
                }
                // @@ not followed by > -- this is an infix operator, not a quotation closer.
                // The lexer has advanced past both @ chars; flag so we skip the keyword chain
                // and treat this like an infix op.
                failed_at_sign_match = true;
                found_start_of_infix_op = true;
            } else if lexer.lookahead() == b'>' as i32 {
                if valid_symbols[DEDENT] && self.indents.len() > 1 {
                    self.pop_indent();
                    lexer.set_result_symbol(DEDENT as u16);
                    return true;
                }
                advance(lexer);
                lexer.mark_end();
                lexer.set_result_symbol(QUOTED_CLOSE as u16);
                return true;
            } else {
                // @ not followed by > or @ -- this is an infix operator, not a quotation closer.
                // The lexer has advanced past @; flag so we skip the keyword chain
                // and treat this like an infix op.
                failed_at_sign_match = true;
                found_start_of_infix_op = true;
            }
        }

        // Emit any pending DEDENT/NEWLINE before probing for multi-dollar strings:
        // the probe's mark_end moves the token end past the consumed whitespace, so
        // a DEDENT emitted from inside it swallows the newline and starves the
        // remaining DEDENT/NEWLINE this line break still owes (e.g. before $"...").
        if !valid_symbols[ERROR_SENTINEL]
            && lexer.lookahead() == b'$' as i32
            && found_end_of_line
            && !self.indents.is_empty()
        {
            let current_indent_length = self.peek_indent_length();
            if valid_symbols[DEDENT]
                && indent_length < current_indent_length
                && (!self.peek_is_paren_indent() || indent_length == 0 || lexer.eof())
            {
                self.pop_indent();
                lexer.set_result_symbol(DEDENT as u16);
                return true;
            }
            if valid_symbols[NEWLINE] && indent_length == current_indent_length && indent_length > 0
            {
                lexer.set_result_symbol(NEWLINE as u16);
                return true;
            }
        }

        if !valid_symbols[ERROR_SENTINEL]
            && lexer.lookahead() == b'$' as i32
            && valid_symbols[MULTI_DOLLAR_TRIPLE_QUOTE_START]
        {
            lexer.mark_end();

            {
                let mut dollar_count = 0;
                while lexer.lookahead() == b'$' as i32 && dollar_count < u8::MAX {
                    advance(lexer);
                    dollar_count += 1;
                }
                if dollar_count > 1 && scan_n_chars(lexer, b'"' as i32, 3) {
                    self.multi_dollar_count = dollar_count;
                    lexer.set_result_symbol(MULTI_DOLLAR_TRIPLE_QUOTE_START as u16);
                    return true;
                }
                // Not a multi-dollar string. Before bailing out, check if DEDENT or
                // NEWLINE should be emitted -- the '$' might be the start of an
                // interpolated string on a new, less-indented line.
                if found_end_of_line && !self.indents.is_empty() {
                    let current_indent_length = self.peek_indent_length();
                    if valid_symbols[DEDENT] && indent_length < current_indent_length {
                        let can_dedent_paren_indent =
                            !self.peek_is_paren_indent() || indent_length == 0 || lexer.eof();
                        if can_dedent_paren_indent {
                            self.pop_indent();
                            lexer.set_result_symbol(DEDENT as u16);
                            return true;
                        }
                    }
                    if valid_symbols[NEWLINE]
                        && indent_length == current_indent_length
                        && indent_length > 0
                    {
                        lexer.set_result_symbol(NEWLINE as u16);
                        return true;
                    }
                }
                return false;
            }
        }

        let mut failed_block_opener = false;
        {
            for &(first_char, rest, token) in BLOCK_OPENERS {
                if valid_symbols[token] && lexer.lookahead() == i32::from(first_char) {
                    // No mark_end before the keyword is confirmed: a failed probe must
                    // leave the token end at the scan start so a later zero-width DEDENT
                    // doesn't consume the newline (which the next scan still needs for
                    // NEWLINE/DEDENT decisions). The success path marks below.
                    indent_length = lexer.get_column();
                    advance(lexer);
                    if match_keyword_rest(lexer, rest) {
                        lexer.mark_end();
                        lexer.set_result_symbol((token) as u16);
                        return true;
                    }
                    failed_block_opener = true;
                    break;
                }
            }
        }

        // Not gated on found_preprocessor_end: when '#endif' follows, this point is
        // only reached if PREPROC_END was not a valid symbol (a valid one returns
        // above), and the pending NEWLINE_NO_ALIGNED (e.g. an fsi directive's
        // terminator) must close its rule first. It IS deferred while inner indented
        // blocks still need to close — DEDENT comes first then.
        if found_end_of_line && valid_symbols[NEWLINE_NO_ALIGNED] && !found_start_of_infix_op {
            let dedent_first = valid_symbols[DEDENT]
                && self.indents.len() > 1
                && indent_length < self.peek_indent_length()
                && (!self.peek_is_paren_indent() || indent_length == 0 || lexer.eof());
            if !dedent_first {
                lexer.set_result_symbol(NEWLINE_NO_ALIGNED as u16);
                return true;
            }
        }

        if !failed_block_opener && !failed_at_sign_match {
            if valid_symbols[NEWLINE] && lexer.lookahead() == b';' as i32 {
                advance(lexer);
                // `;;` is the fsi-style spelling of the same thing; consume both
                // semicolons as one token so the second one doesn't error.
                if lexer.lookahead() == b';' as i32 {
                    advance(lexer);
                }
                lexer.mark_end(); // Token = just ';'/';;'; chars after are returned to input
                let mut saw_newline = false;
                loop {
                    while lexer.lookahead() == b' ' as i32
                        || lexer.lookahead() == b'\n' as i32
                        || lexer.lookahead() == b'\r' as i32
                    {
                        if lexer.lookahead() == b'\n' as i32 {
                            saw_newline = true;
                            indent_length = 0;
                        } else if lexer.lookahead() == b'\r' as i32 {
                            // CRLF: skip without counting toward indentation.
                        } else if saw_newline {
                            indent_length = indent_length.wrapping_add(1);
                        }
                        advance(lexer); // Beyond mark_end: returned to input for next token
                    }
                    // A trailing line comment ("// ...") means the ';' is the last code on
                    // its line, so skip over it (lookahead only — the comment stays in the
                    // input) to reach the terminating newline. Without this, the loop would
                    // stop at '/' and treat the ';' as a mid-line separator that demands a
                    // following expression which isn't there (e.g. `then 1.; // note`).
                    if lexer.lookahead() == b'/' as i32 {
                        advance(lexer);
                        if lexer.lookahead() == b'/' as i32 {
                            while lexer.lookahead() != b'\n' as i32 && !lexer.eof() {
                                advance(lexer);
                            }
                            continue; // re-enter to consume the newline after the comment
                        }
                        break; // a lone '/' is an operator, not a comment
                    }
                    break;
                }
                if saw_newline {
                    found_end_of_line = true;
                }
                // A trailing ';' / ';;' TERMINATES a top-level element; it does not
                // separate two expressions. (';;' is just ';' here — the second semicolon
                // adds nothing but an fsi convention.) Emitting NEWLINE makes it a
                // sequential-expression separator that demands another expression on the
                // right, so `exit 0;` at end of file errors, and `exit 0;` followed by a
                // column-0 `let` absorbs that let as a let-in argument (MISSING `in`).
                // When the semicolon really does end a top-level element — only the base
                // indent is open and the next content is at column 0, or the file ends —
                // hand the position to the module-element separator instead (same
                // whitelist as the arming site above), which is what an element boundary
                // with no semicolon at all already produces; or at EOF decline so the ';'
                // extra takes the characters. Anything else (a ';' inside an indented
                // block or a bracket, mid-line, before a continuation line, before a
                // comment) keeps the separator behaviour.
                let mut semi_terminator = !found_comment_start
                    && self.indents.len() == 1
                    && (lexer.eof() || (saw_newline && indent_length == 0));
                if semi_terminator {
                    if !lexer.eof()
                        && valid_symbols[ELEM_SEP]
                        && !valid_symbols[ERROR_SENTINEL]
                        && (is_word_char(lexer.lookahead())
                            || lexer.lookahead() == b'[' as i32
                            || lexer.lookahead() == b'"' as i32)
                    {
                        // Consume the `;;` and the newline: the separator makes progress, so
                        // it cannot be re-shifted at this position (see the arming comment).
                        lexer.mark_end();
                        elem_sep_armed = true;
                    } else if lexer.eof() {
                        return false;
                    } else {
                        semi_terminator = false;
                    }
                }
                if !semi_terminator {
                    found_end_of_line_semi_colon = true;
                }
            }

            if lexer.lookahead() == b't' as i32 && (valid_symbols[THEN] || valid_symbols[DEDENT]) {
                advance(lexer);
                if lexer.lookahead() == b'h' as i32 {
                    advance(lexer);
                    if lexer.lookahead() == b'e' as i32 {
                        advance(lexer);
                        if lexer.lookahead() == b'n' as i32 {
                            advance(lexer);
                            if !is_word_char(lexer.lookahead()) {
                                // the 'THEN' token is only valid if we have popped the appropriate
                                // amount of dedent tokens.
                                // If 'THEN' is not valid we just continue to pop dedent tokens.
                                if valid_symbols[THEN] {
                                    lexer.mark_end();
                                    lexer.set_result_symbol(THEN as u16);
                                    return true;
                                } else {
                                    self.pop_indent();
                                    lexer.set_result_symbol(DEDENT as u16);
                                    return true;
                                }
                            }
                        }
                    }
                }
            } else if lexer.lookahead() == b'a' as i32
                && (valid_symbols[AND] || valid_symbols[DEDENT])
            {
                advance(lexer);
                if lexer.lookahead() == b'n' as i32 {
                    advance(lexer);
                    if lexer.lookahead() == b'd' as i32 {
                        advance(lexer);
                        if lexer.lookahead() == b' ' as i32 {
                            // the 'AND' token is only valid if we have popped the appropriate
                            // amount of dedent tokens.
                            // If 'AND' is not valid we just continue to pop dedent tokens.
                            if valid_symbols[AND] {
                                // Greedy-`and` guard: a line-leading `and` sitting LEFT of the
                                // current indent scope belongs to an outer construct (e.g.
                                // `and B() = ...` closing a type whose last member was a property
                                // accessor block) — unless it introduces another accessor
                                // (`and set (v) = ...`), which legitimately continues the
                                // accessor scope even though that scope's indent anchors at a
                                // mid-line column. Peeking past `and` moves beyond a possible
                                // mark_end position, so on the accessor path we decline and let
                                // the internal lexer produce the string-literal `and` token.
                                if found_end_of_line
                                    && valid_symbols[DEDENT]
                                    && !self.indents.is_empty()
                                    && indent_length < self.peek_indent_length()
                                {
                                    while lexer.lookahead() == b' ' as i32 {
                                        advance(lexer);
                                    }
                                    let mut word = [0u8; 9];
                                    let mut word_len = 0;
                                    while is_word_char(lexer.lookahead()) && word_len < 8 {
                                        word[word_len] = lexer.lookahead() as u8;
                                        word_len += 1;
                                        advance(lexer);
                                    }

                                    let accessor_next =
                  // `and [<Attr>] set (v) = ...`: an attribute set can only
                  // introduce another accessor here.
                  (word_len == 0 && lexer.lookahead() == b'[' as i32) ||
                  (!is_word_char(lexer.lookahead()) &&
                   ((&word[..word_len] == b"get") || (&word[..word_len] == b"set") ||
                    (&word[..word_len] == b"inline") ||
                    (&word[..word_len] == b"private") ||
                    (&word[..word_len] == b"internal") ||
                    (&word[..word_len] == b"public")));
                                    if !accessor_next {
                                        self.pop_indent();
                                        lexer.set_result_symbol(DEDENT as u16);
                                        return true;
                                    }
                                    return false;
                                }
                                lexer.mark_end();
                                lexer.set_result_symbol(AND as u16);
                                return true;
                            } else {
                                self.pop_indent();
                                lexer.set_result_symbol(DEDENT as u16);
                                return true;
                            }
                        } else if !is_word_char(lexer.lookahead()) {
                            // Handle 'and' followed by newline/EOF (not word char, not space).
                            // Mirror the space-branch: emit AND when valid; otherwise trigger
                            // a keyword-driven DEDENT to close intermediate scopes so the parser
                            // retries AND at the correct level on the next scan.
                            if valid_symbols[AND] {
                                lexer.mark_end();
                                lexer.set_result_symbol(AND as u16);
                                return true;
                            } else if valid_symbols[DEDENT] {
                                self.pop_indent();
                                lexer.set_result_symbol(DEDENT as u16);
                                return true;
                            }
                        }
                    }
                }
            } else if lexer.lookahead() == b'w' as i32
                && (valid_symbols[WITH] || valid_symbols[DEDENT])
            {
                advance(lexer);
                if lexer.lookahead() == b'i' as i32 {
                    advance(lexer);
                    if lexer.lookahead() == b't' as i32 {
                        advance(lexer);
                        if lexer.lookahead() == b'h' as i32 {
                            advance(lexer);
                            if !is_word_char(lexer.lookahead()) {
                                // Force-close a try-body scope whose terminating 'with' begins a
                                // new line at the same column as the body, so an ordinary dedent
                                // won't fire. This must take priority over the same-indent NEWLINE
                                // heuristic below: unlike a class/record body, a try body has no
                                // 'with' augmentation of its own, so the 'with' must close it.
                                if valid_symbols[DEDENT]
                                    && !valid_symbols[WITH]
                                    && self.peek_is_try_indent()
                                {
                                    self.pop_indent();
                                    lexer.set_result_symbol(DEDENT as u16);
                                    return true;
                                }
                                // If 'with' sits at the same indent as the current scope and the
                                // grammar isn't yet expecting WITH, emit NEWLINE first so the
                                // preceding statement closes before the augmentation opens.
                                if valid_symbols[NEWLINE]
                                    && found_end_of_line
                                    && !valid_symbols[WITH]
                                    && !self.indents.is_empty()
                                    && indent_length == self.peek_indent_length()
                                {
                                    lexer.set_result_symbol(NEWLINE as u16);
                                    return true;
                                }
                                // WITH only valid once the right number of DEDENTs have popped.
                                if valid_symbols[WITH] {
                                    lexer.mark_end();
                                    lexer.set_result_symbol(WITH as u16);
                                    return true;
                                } else {
                                    self.pop_indent();
                                    lexer.set_result_symbol(DEDENT as u16);
                                    return true;
                                }
                            } else if valid_symbols[WITH] && !is_word_char(lexer.lookahead()) {
                                // Handle 'with' followed by newline/EOF (not word char, not space).
                                // Only emit WITH here. Unlike 'and', do NOT force a keyword-driven
                                // DEDENT when only DEDENT is valid: `with` on a new line inside a
                                // class body is a CONTINUATION of the enclosing type (starting a
                                // type_extension_elements), so closing the enclosing scope early
                                // would truncate the type definition. Let the natural indent
                                // tracking (line ~1050) emit any needed DEDENTs and NEWLINE.
                                lexer.mark_end();
                                lexer.set_result_symbol(WITH as u16);
                                return true;
                            }
                        }
                    }
                }
            } else if lexer.lookahead() == b'd' as i32 && valid_symbols[DO_KEYWORD] {
                advance(lexer);
                if lexer.lookahead() == b'o' as i32 {
                    advance(lexer);
                    // Exclude 'do!' so computation-expression do-bang is never claimed.
                    if !is_word_char(lexer.lookahead()) && lexer.lookahead() != b'!' as i32 {
                        lexer.mark_end();
                        lexer.set_result_symbol(DO_KEYWORD as u16);
                        return true;
                    }
                }
            } else if lexer.lookahead() == b'i' as i32
                && (valid_symbols[IN] || valid_symbols[DEDENT])
            {
                advance(lexer);
                if lexer.lookahead() == b'n' as i32 {
                    advance(lexer);
                    if !is_word_char(lexer.lookahead()) {
                        if valid_symbols[IN] {
                            // Produce the IN token to close an _expression_block_for_let.
                            // Pop the indent that was pushed by the matching INDENT, since
                            // _in replaces _dedent as the block terminator.
                            self.pop_indent();
                            lexer.mark_end();
                            lexer.set_result_symbol(IN as u16);
                            return true;
                        }
                        // `let x = if c then a else b in x`: the `in` arrives while the
                        // else-branch's own block (or a match arm's, a lambda body's, a
                        // try body's) is still open, so IN is not valid yet but DEDENT
                        // is. Close that block — zero-width, the `in` is not consumed —
                        // and the next scan sees it with IN valid. The base level is
                        // never closed this way: there `in` can only be a `for … in`.
                        // Not inside a `{ … }` block: there `in` is an ordinary identifier
                        // in query clauses (`join c in cs on (…)`) and the parser lexes it.
                        let mut in_brace = false;
                        for i in 0..self.indent_kinds.len() {
                            if self.indent_kinds[i] == INDENT_BRACE {
                                in_brace = true;
                                break;
                            }
                        }
                        if self.indents.len() > 1 && !in_brace {
                            self.pop_indent();
                            lexer.set_result_symbol(DEDENT as u16);
                            return true;
                        }
                    }
                }
            } else if lexer.lookahead() == b'e' as i32
                && (valid_symbols[ELSE]
                    || valid_symbols[ELIF]
                    || valid_symbols[END]
                    || valid_symbols[DEDENT])
            {
                advance(lexer);
                let token_indent_level = i32::from(lexer.get_column() as i16);
                if lexer.lookahead() == b'l' as i32 {
                    advance(lexer);
                    if lexer.lookahead() == b's' as i32
                        && (valid_symbols[ELSE] || valid_symbols[DEDENT])
                    {
                        advance(lexer);
                        if lexer.lookahead() == b'e' as i32 {
                            advance(lexer);
                            if !is_word_char(lexer.lookahead()) {
                                if valid_symbols[ELSE] {
                                    // Don't pop paren-kind scopes: an if/else inside a call's
                                    // parens may sit at any indentation; ')' closes that scope.
                                    if !self.indents.is_empty()
                                        && !self.peek_is_paren_indent()
                                        && token_indent_level < self.peek_indent_length() as i32
                                    {
                                        self.pop_indent();
                                        lexer.set_result_symbol(DEDENT as u16);
                                        return true;
                                    } else {
                                        lexer.mark_end();
                                        // Only fold "else if" into ELIF when they share a line. If a
                                        // newline separates them this is an `else` whose block body
                                        // starts with an `if` (and may be followed by more statements),
                                        // so skip only same-line spacing here, never newlines.
                                        loop {
                                            if lexer.lookahead() == b' ' as i32
                                                || lexer.lookahead() == b'\t' as i32
                                            {
                                                advance(lexer);
                                            } else {
                                                break;
                                            }
                                        }
                                        if lexer.lookahead() == b'i' as i32 {
                                            advance(lexer);
                                            if lexer.lookahead() == b'f' as i32 {
                                                advance(lexer);
                                                if !is_word_char(lexer.lookahead()) {
                                                    lexer.mark_end();
                                                    lexer.set_result_symbol(ELIF as u16);
                                                    return true;
                                                }
                                            }
                                        }
                                        lexer.set_result_symbol(ELSE as u16);
                                        return true;
                                    }
                                } else {
                                    self.pop_indent();
                                    lexer.set_result_symbol(DEDENT as u16);
                                    return true;
                                }
                            }
                        }
                    } else if lexer.lookahead() == b'i' as i32
                        && (valid_symbols[ELIF] || valid_symbols[DEDENT])
                    {
                        advance(lexer);
                        if lexer.lookahead() == b'f' as i32 {
                            advance(lexer);
                            if !is_word_char(lexer.lookahead()) {
                                if valid_symbols[ELIF] {
                                    if !self.indents.is_empty()
                                        && !self.peek_is_paren_indent()
                                        && token_indent_level < self.peek_indent_length() as i32
                                    {
                                        self.pop_indent();
                                        lexer.set_result_symbol(DEDENT as u16);
                                        return true;
                                    } else {
                                        lexer.mark_end();
                                        lexer.set_result_symbol(ELIF as u16);
                                        return true;
                                    }
                                } else {
                                    self.pop_indent();
                                    lexer.set_result_symbol(DEDENT as u16);
                                    return true;
                                }
                            }
                        }
                    }
                } else if lexer.lookahead() == b'n' as i32
                    && (valid_symbols[END] || valid_symbols[DEDENT])
                {
                    advance(lexer);
                    if lexer.lookahead() == b'd' as i32 {
                        advance(lexer);
                        if !is_word_char(lexer.lookahead()) {
                            if valid_symbols[END] {
                                lexer.mark_end();
                                lexer.set_result_symbol(END as u16);
                                return true;
                            } else if valid_symbols[DEDENT] && !self.indents.is_empty() {
                                self.pop_indent();
                                lexer.set_result_symbol(DEDENT as u16);
                                return true;
                            }
                        }
                    }
                }
            } else if is_bracket_end(lexer) {
                found_bracket_end = true;
            } else if is_infix_op_start(lexer) {
                found_start_of_infix_op = true;
            } else if lexer.lookahead() == b'|' as i32 {
                skip(lexer);
                let after_pipe = lexer.lookahead();
                if after_pipe == b']' as i32 || after_pipe == b'}' as i32 {
                    found_bracket_end = true;
                } else if after_pipe == b'>' as i32 {
                    found_start_of_infix_op = true;
                } else if after_pipe == b' ' as i32 ||
               // A '|' glued to a pattern-start character (`|"A"`, `|_`,
               // `|(p, q)`, `|Some x`, `|[x]`, ``|``id`` ``) is a match arm
               // or union case exactly like the spaced form, not an infix
               // operator. Operator characters (`|>`, `||`, `|.`) keep the
               // infix treatment below.
               after_pipe == b'"' as i32 || after_pipe == b'(' as i32 || after_pipe == b'[' as i32 ||
               after_pipe == b'`' as i32 || is_word_char(after_pipe)
                {
                    if !found_end_of_line {
                        found_start_of_infix_op = true;
                        found_same_line_pipe_infix = true;
                    } else {
                        if indent_length == 0 {
                            indent_length = 1;
                        }
                        if !self.indents.is_empty() {
                            let current_indent_length = self.peek_indent_length();
                            if found_end_of_line
                                && indent_length == current_indent_length
                                && indent_length > 0
                                && !found_start_of_infix_op
                                && !found_bracket_end
                                && valid_symbols[NEWLINE]
                                && !found_preprocessor_end
                            {
                                lexer.set_result_symbol(NEWLINE as u16);
                                return true;
                            }
                        }
                    }
                } else {
                    found_start_of_infix_op = true;
                }
            } else if lexer.lookahead() == b'(' as i32 {
                skip(lexer);
                // Recorded for the SRTP typar-group veto below: this scan began *on* a
                // '(', so any group seen from here is inside that paren, i.e. the request
                // belongs to the enclosing expression's paren rather than to the group's.
                skipped_open_paren = true;
                if lexer.lookahead() == b'*' as i32 {
                    // `(*)` is the multiplication operator reference (F# spec 3.10), not a
                    // comment opener — peek one further so it doesn't suppress INDENT and
                    // force the comment interpretation (`let f = (*) 2 3`).
                    skip(lexer);
                    if lexer.lookahead() != b')' as i32 {
                        found_comment_start = true;
                    }
                }
            }
        } // end of !failed_block_opener && !failed_at_sign_match

        if valid_symbols[NEWLINE]
            && found_end_of_line_semi_colon
            && !found_comment_start
            && !found_bracket_end
        {
            // If semicolon was followed by newlines that drop the indentation,
            // fall through to DEDENT logic instead of emitting NEWLINE
            let mut needs_dedent = found_end_of_line && valid_symbols[DEDENT] &&
                        !self.indents.is_empty() &&
                        indent_length < self.peek_indent_length() &&
                        // Paren-kind scopes don't dedent on under-indented
                        // lines, so don't withhold the NEWLINE for them.
                        (!self.peek_is_paren_indent() ||
                         indent_length == 0 || lexer.eof());
            if needs_dedent && indent_length > 0 {
                // Only defer to DEDENT when the next line lands exactly on an open
                // indentation level. A line "between" levels (e.g. record fields
                // wrapped at an arbitrary lower indent after `A = "x";`) continues
                // the current construct instead.
                let mut has_matching_level = false;
                for lvl in 0..self.indents.len() - 1 {
                    if u32::from(self.indents[lvl]) == indent_length {
                        has_matching_level = true;
                        break;
                    }
                }
                if !has_matching_level {
                    needs_dedent = false;
                }
            }
            if !needs_dedent {
                lexer.set_result_symbol(NEWLINE as u16);
                return true;
            }
        }

        if valid_symbols[TRY_INDENT]
            && !valid_symbols[ERROR_SENTINEL]
            && !found_bracket_end
            && !found_preprocessor_end
            && !found_same_line_pipe_infix
        {
            // Like INDENT, but the scope is tagged so it can be force-closed when its
            // terminating `with`/`finally` sits at the same column as the body.
            self.push_indent(indent_length, INDENT_TRY);
            lexer.set_result_symbol(TRY_INDENT as u16);
            return true;
        }

        if valid_symbols[INDENT] && !valid_symbols[ERROR_SENTINEL] &&
      !found_bracket_end && !found_preprocessor_end &&
      !found_same_line_pipe_infix &&
      // A block comment trailing the current line must not anchor the new
      // block: without this, `let f x = (* c *)` + an indented body pushes
      // the comment's column, and the body line immediately DEDENTs it back
      // out. Decline instead; after the comment is consumed as an extra, the
      // re-scan crosses the newline and INDENT anchors to the body line.
      !(found_comment_start && !found_end_of_line)
        {
            let mut indent_flags = 0;
            if !found_end_of_line {
                indent_flags |= INDENT_KIND_MIDLINE_FLAG;
                if self.line_stranded != 0 {
                    indent_flags |= INDENT_KIND_STRANDED_LINE_FLAG;
                }
            }
            self.push_indent(indent_length, INDENT_NORMAL | indent_flags);
            lexer.set_result_symbol(INDENT as u16);
            return true;
        }

        if valid_symbols[PAREN_INDENT]
            && !valid_symbols[ERROR_SENTINEL]
            && !found_bracket_end
            && !found_preprocessor_end
            && !found_same_line_pipe_infix
        {
            // In `((^a or ^b) : (static member M : ^a -> ^b) x)` the inner '(' opens a
            // trait call's typar group, not a parenthesised expression. PAREN_INDENT is
            // zero-width and the parser takes it whenever it is valid, so emitting it
            // just inside that '(' would commit to paren_expression and kill the group
            // reading. Decline when what follows is exactly a typar group closed by
            // `) : (`, which the body of a parenthesised expression can never be.
            //
            // `skipped_open_paren` keeps the *enclosing* expression's own indent: this
            // same group is also what follows the outer '(', and that request — which
            // arrives with the scan sitting on the '(' the probe above skipped — must
            // still produce PAREN_INDENT, or the whole parenthesised expression dies.
            // No mark_end, so returning false discards the peek entirely.
            if !skipped_open_paren
                && (lexer.lookahead() == b'^' as i32 || lexer.lookahead() == b'\'' as i32)
                && !found_end_of_line
                && !advanced_in_ws_walk
                && is_srtp_typar_group_ahead(lexer)
            {
                return false;
            }
            // Like INDENT, but tracked separately as a paren indent so DEDENT/NEWLINE
            // logic can be more lenient inside parenthesized expressions, where the
            // closing ')' determines scope rather than indentation alone.
            self.push_indent(indent_length, INDENT_PAREN);
            lexer.set_result_symbol(PAREN_INDENT as u16);
            return true;
        }

        if valid_symbols[BRACE_INDENT]
            && !valid_symbols[ERROR_SENTINEL]
            && !found_bracket_end
            && !found_preprocessor_end
            && !found_same_line_pipe_infix
        {
            // Opens a '{...}' record/CE field block (see INDENT_BRACE).
            self.push_indent(indent_length, INDENT_BRACE);
            lexer.set_result_symbol(BRACE_INDENT as u16);
            return true;
        }

        // Fires after '<' if the type args span multiple lines — either we just
        // consumed a newline, or peek-ahead shows one before the matching '>' (the
        // variant where the first arg shares the line with '<').
        if valid_symbols[TYPE_APP_INDENT]
            && !valid_symbols[ERROR_SENTINEL]
            && !found_bracket_end
            && !found_preprocessor_end
            && !found_same_line_pipe_infix
        {
            let is_multiline = found_end_of_line || is_multiline_type_app_ahead(lexer);
            if is_multiline {
                self.push_indent(indent_length, INDENT_TYPE_APP);
                lexer.set_result_symbol(TYPE_APP_INDENT as u16);
                return true;
            }
        }

        if !self.indents.is_empty() {
            let is_paren_indent = self.peek_is_paren_indent();
            let is_brace_indent = self.peek_is_brace_indent();
            let current_indent_length = self.peek_indent_length();

            // '>' closes a TYPE_APP_INDENT the same way ')' closes a PAREN_INDENT —
            // gated on the type-app bit so we don't pop ordinary paren scopes here.
            let found_type_app_close = self.peek_is_type_app_indent()
                && lexer.lookahead() == b'>' as i32
                && valid_symbols[DEDENT];

            if (found_bracket_end || found_type_app_close) && valid_symbols[DEDENT] {
                self.pop_indent();
                lexer.set_result_symbol(DEDENT as u16);
                return true;
            }

            if found_end_of_line {
                // Inside a brace block, a line that is not more indented than the anchor
                // (including one left of the first item) separates items rather than
                // dedenting out — the '}' is what closes the block.
                let brace_separator = is_brace_indent && indent_length < current_indent_length;
                if (indent_length == current_indent_length || brace_separator)
                    && indent_length > 0
                    && !found_start_of_infix_op
                    && !found_bracket_end
                    && valid_symbols[NEWLINE]
                    && !found_preprocessor_end
                    && !found_comment_start
                {
                    lexer.set_result_symbol(NEWLINE as u16);
                    return true;
                }

                // Consume a pending stranded-dedent: the previous scan emitted a DEDENT
                // that closed a nested block, but this line sits above the enclosing
                // block's indent. Emit the NEWLINE the enclosing block owes so the line
                // starts a new element instead of being stranded. Guarded on a real
                // preceding stranded DEDENT so ordinary more-indented continuation lines
                // (e.g. a multi-line application argument) are untouched.
                if prev_stranded_dedent
                    && indent_length > current_indent_length
                    && valid_symbols[NEWLINE]
                    && !found_start_of_infix_op
                    && !found_bracket_end
                    && !found_preprocessor_end
                    && !found_comment_start
                {
                    // This line hangs between two open levels; scopes anchored mid-line
                    // on it must not claim following same-column lines as continuations.
                    self.line_stranded = 1;
                    lexer.set_result_symbol(NEWLINE as u16);
                    return true;
                }

                // Top-level module-element separator, phase 2 of 2 (emit). All
                // eligibility checks — and the mark_end that makes the token consume
                // the walked newline — happened at the arming site right after the
                // whitespace walk; see the comment there. Emitting this late keeps
                // the keyword probes' priority (an `and` continuation line must
                // produce AND, not a separator), and any lookahead they consumed past
                // the armed mark_end is returned to the input here.
                if elem_sep_armed {
                    lexer.set_result_symbol(ELEM_SEP as u16);
                    return true;
                }

                let can_dedent_preproc = if !self.preprocessor_indents.is_empty() {
                    let current_preproc_length =
                        u32::from(*self.preprocessor_indents.last().unwrap());
                    current_preproc_length < indent_length
                } else {
                    true
                };

                // An infix continuation belongs to the preceding expression even
                // when it sits left of the expression it continues.
                let can_dedent_infix_op = !found_start_of_infix_op;

                // Inside paren-indented blocks, avoid DEDENT on ordinary under-indented
                // continuation lines. But still allow it at true EOF / column 0 so a
                // paren indent can't get stranded forever if its closing bracket was
                // consumed by the grammar rather than seen here at the start of a line.
                let can_dedent_paren_indent =
                    !(is_paren_indent || is_brace_indent) || indent_length == 0 || lexer.eof();

                // A line that sits strictly DEEPER than the line which opened a
                // mid-line-anchored scope continues the anchored expression, even when
                // it is left of the anchor column itself: `let a = Some <|` anchors at
                // `Some` (col 12), but F#'s offside rule measures the continuation
                // against the `let` line's indent (4), so an operand at col 8 belongs
                // to the expression and must not be DEDENTed into. A line at or left
                // of the anchor line's indent is a sibling/outer construct and still
                // dedents (`let c = ()` followed by `let d = ()` at the same column).
                // All conditions must hold, each excluding a shape where the dedent is
                // legitimate: a genuine pushed enclosing level (size > 2 — a top-level
                // module body is not indent-scoped and its members MUST dedent out),
                // of line-column kind (paren/brace/type-app indents are synthetic),
                // the line strictly deeper than that enclosing level, and the anchor
                // NOT opened on a stranded line (a stranded declaration like
                // `let c = ()` at col 8 under a col-4 block is followed by sibling
                // declarations at its own column, which must still dedent).
                let mut can_dedent_midline_anchor = true;
                if self.top_indent_is_midline_anchor()
                    && !self.top_indent_is_stranded_line()
                    && self.indents.len() > 2
                {
                    let below_kind_raw = self.indent_kinds[self.indents.len() - 2];
                    let below_kind = below_kind_raw & !INDENT_KIND_FLAGS_MASK;
                    if (below_kind == INDENT_NORMAL || below_kind == INDENT_TRY)
                        && indent_length > u32::from(self.indents[self.indents.len() - 2])
                    {
                        can_dedent_midline_anchor = false;
                    }
                }

                if indent_length < current_indent_length
                    && !found_bracket_end
                    && can_dedent_preproc
                    && can_dedent_infix_op
                    && can_dedent_midline_anchor
                    && (!valid_symbols[TUPLE_MARKER] || valid_symbols[ERROR_SENTINEL])
                    && can_dedent_paren_indent
                {
                    self.pop_indent();
                    // If this line closed a nested block but still sits above the enclosing
                    // block's own indent (e.g. dedenting from a nested module body back to
                    // an outer module whose next member is indented past the outer
                    // module's offside column), the enclosing block owes an item
                    // separator. Record that so the next scan emits the NEWLINE — a bare
                    // DEDENT here would strand the line between two open levels.
                    //
                    // Require a genuine enclosing indent level (size > 1, i.e. we landed on
                    // a pushed level, not the base level 0). A top-level `module M`/namespace
                    // body is not indent-scoped, so an expression block (e.g. a `do ()` body)
                    // closing back to the base level is an ordinary dedent, not a stranded
                    // one — injecting a separator there wrongly glues sibling members into a
                    // sequential_expression.
                    if self.indents.len() > 1 && indent_length > self.peek_indent_length() {
                        self.stranded_dedent = 1;
                    }
                    lexer.set_result_symbol(DEDENT as u16);
                    return true;
                }
            }
        }

        false
    }

    fn serialize(&mut self, buffer: &mut [u8]) -> usize {
        // Each stack count is capped at 255. The final kind stack can be
        // truncated at 1024, while its count still records the capped depth.
        let limit = buffer.len().min(SERIALIZATION_BUFFER_SIZE);
        let mut size = 0;
        let mut write = |byte: u8| {
            if size < limit {
                buffer[size] = byte;
                size += 1;
            }
        };
        write(self.multi_dollar_count);
        write(self.stranded_dedent);
        write(self.line_stranded);
        let preprocessor_count = self.preprocessor_indents.len().min(255);
        write(preprocessor_count as u8);
        for &indent in &self.preprocessor_indents[..preprocessor_count] {
            write(indent as u8);
        }
        let indent_count = self.indents.len().saturating_sub(1).min(255);
        write(indent_count as u8);
        for &indent in self.indents.iter().skip(1).take(indent_count) {
            write(indent as u8);
        }
        for &kind in self.indent_kinds.iter().skip(1).take(indent_count) {
            write(kind);
        }
        let preproc_kind_count = self.preproc_kinds.len().min(255);
        write(preproc_kind_count as u8);
        for &kind in &self.preproc_kinds[..preproc_kind_count] {
            write(kind);
        }
        size
    }

    fn deserialize(&mut self, buffer: &[u8]) {
        // Retain allocations: the parser restores snapshots frequently under GLR.
        self.indents.clear();
        self.indents.push(0);
        self.indent_kinds.clear();
        self.indent_kinds.push(INDENT_NORMAL);
        self.preprocessor_indents.clear();
        self.preproc_kinds.clear();
        self.multi_dollar_count = 0;
        self.stranded_dedent = 0;
        self.line_stranded = 0;
        let mut bytes = buffer.iter().copied();
        let Some(byte) = bytes.next() else {
            return;
        };
        self.multi_dollar_count = byte;
        let Some(byte) = bytes.next() else {
            return;
        };
        self.stranded_dedent = byte;
        let Some(byte) = bytes.next() else {
            return;
        };
        self.line_stranded = byte;
        let Some(count) = bytes.next() else {
            return;
        };
        self.preprocessor_indents
            .extend(bytes.by_ref().take(count as usize).map(u16::from));
        let Some(count) = bytes.next() else {
            return;
        };
        self.indents
            .extend(bytes.by_ref().take(count as usize).map(u16::from));
        for _ in 1..self.indents.len() {
            self.indent_kinds
                .push(bytes.next().unwrap_or(INDENT_NORMAL));
        }
        let Some(count) = bytes.next() else {
            return;
        };
        self.preproc_kinds.extend(bytes.take(count as usize));
    }
}

pub(crate) fn create() -> Box<dyn ExternalScanner> {
    let mut scanner = Scanner::default();
    scanner.deserialize(&[]);
    Box::new(scanner)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Eq)]
    enum Call {
        Advance(usize, bool),
        Mark(usize),
        Column(usize),
    }

    struct TestLexer {
        input: Vec<i32>,
        position: usize,
        end: usize,
        symbol: u16,
        calls: Vec<Call>,
    }

    impl TestLexer {
        fn new(input: &str) -> Self {
            Self {
                input: input.chars().map(|c| c as i32).collect(),
                position: 0,
                end: 0,
                symbol: u16::MAX,
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
        }
        fn advance(&mut self, skip: bool) {
            assert!(!self.eof(), "unexpected advance at EOF");
            self.calls.push(Call::Advance(self.position, skip));
            self.position += 1;
        }
        fn mark_end(&mut self) {
            self.calls.push(Call::Mark(self.position));
            self.end = self.position;
        }
        fn get_column(&mut self) -> u32 {
            self.calls.push(Call::Column(self.position));
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

    fn fresh() -> Scanner {
        let mut scanner = Scanner::default();
        scanner.deserialize(&[]);
        scanner
    }

    fn valid(tokens: &[usize]) -> [bool; ERROR_SENTINEL + 1] {
        let mut valid = [false; ERROR_SENTINEL + 1];
        for &token in tokens {
            valid[token] = true;
        }
        valid
    }

    fn snapshot(scanner: &mut Scanner) -> Vec<u8> {
        let mut buffer = [0u8; SERIALIZATION_BUFFER_SIZE];
        let length = scanner.serialize(&mut buffer);
        buffer[..length].to_vec()
    }

    #[test]
    fn snapshot_bytes_narrow_columns_and_preserve_kind_flags() {
        let mut scanner = fresh();
        assert_eq!(snapshot(&mut scanner), [0; 6]);
        scanner.multi_dollar_count = 3;
        scanner.stranded_dedent = 1;
        scanner.line_stranded = 1;
        scanner.preprocessor_indents = vec![260, 255];
        scanner.push_indent(8, INDENT_KIND_MIDLINE_FLAG);
        scanner.push_indent(300, INDENT_BRACE);
        scanner.preproc_kinds = vec![PREPROC_STRUCTURED, PREPROC_STRAY];
        let bytes = snapshot(&mut scanner);
        assert_eq!(bytes, [3, 1, 1, 2, 4, 255, 2, 8, 44, 128, 4, 2, 0, 1]);
        scanner.deserialize(&bytes);
        assert_eq!(snapshot(&mut scanner), bytes);
        assert_eq!(scanner.indents, [0, 8, 44]);
        assert_eq!(scanner.preprocessor_indents, [4, 255]);

        // Missing kind bytes default to NORMAL; noncanonical flag bytes are
        // retained as bytes rather than normalized to bool in snapshots.
        scanner.deserialize(&[4, 2, 3, 0, 2, 8, 12, INDENT_PAREN]);
        assert_eq!(
            scanner.indent_kinds,
            [INDENT_NORMAL, INDENT_PAREN, INDENT_NORMAL]
        );
        assert_eq!(snapshot(&mut scanner), [4, 2, 3, 0, 2, 8, 12, 1, 0, 0]);
        scanner.deserialize(&[]);
        assert_eq!(snapshot(&mut scanner), [0; 6]);
    }

    #[test]
    fn snapshot_caps_counts_and_truncates_final_stack_at_buffer_limit() {
        let mut scanner = fresh();
        for _ in 0..300 {
            scanner.push_indent(257, INDENT_KIND_MIDLINE_FLAG);
            scanner.preprocessor_indents.push(258);
            scanner.push_preproc_kind(PREPROC_STRAY);
        }
        let bytes = snapshot(&mut scanner);
        assert_eq!(bytes.len(), 1024);
        assert_eq!(&bytes[..4], [0, 0, 0, 255]);
        assert!(bytes[4..259].iter().all(|&b| b == 2));
        assert_eq!(bytes[259], 255);
        assert!(bytes[260..515].iter().all(|&b| b == 1));
        assert!(bytes[515..770].iter().all(|&b| b == 128));
        assert_eq!(bytes[770], 255);
        assert!(bytes[771..].iter().all(|&b| b == PREPROC_STRAY));
        scanner.deserialize(&bytes);
        assert_eq!(scanner.indents.len(), 256);
        assert_eq!(scanner.preproc_kinds.len(), 253);
        assert_eq!(snapshot(&mut scanner)[770], 253);
    }

    #[test]
    fn type_application_probe_marks_only_the_open_angle() {
        let mut scanner = fresh();
        let mut lexer = TestLexer::new("<int *\n string>rest");
        assert!(scanner.scan(&mut lexer, &valid(&[TYAPP_OPEN])));
        assert_eq!(lexer.symbol, TYAPP_OPEN as u16);
        assert_eq!(lexer.end, 1);
        assert_eq!(lexer.lookahead(), b'>' as i32);
        assert_eq!(
            &lexer.calls[..3],
            [Call::Mark(0), Call::Advance(0, false), Call::Mark(1)]
        );
        assert!(
            lexer.calls[3..]
                .iter()
                .all(|c| matches!(c, Call::Advance(_, false)))
        );

        for (body, expected) in [
            ("kg^(-12345/123)>", true),
            ("m s^-1>", true),
            ("``Risk %``>", true),
            ("int -> string>", true),
            ("b-1>c", false),
            ("b^2 - 1>c", false),
            ("|(inst :> IFoo).M", false),
            ("r)", false),
            ("int", false),
        ] {
            let mut lexer = TestLexer::new(body);
            assert_eq!(is_type_application_open(&mut lexer), expected, "{body}");
        }
        let mut lexer = TestLexer::new("<a-1>c");
        assert!(!scanner.scan(&mut lexer, &valid(&[TYAPP_OPEN])));
        assert_eq!(lexer.position, 3); // A failed probe does not rewind itself.
        assert_eq!(lexer.end, 1);
    }

    #[test]
    fn srtp_group_requires_multiple_typars_and_colon_paren_suffix() {
        for (body, expected) in [
            ("^a or ^b) : (", true),
            ("'T1 or 'T2) : (", true),
            ("^a) : (", false),
            ("^a or ^b)", false),
            ("^a order ^b) : (", false),
        ] {
            assert_eq!(
                is_srtp_typar_group_ahead(&mut TestLexer::new(body)),
                expected
            );
        }
    }

    #[test]
    fn stranded_dedent_owes_one_newline_and_sets_durable_line_flag() {
        let mut scanner = fresh();
        scanner.push_indent(4, INDENT_NORMAL);
        scanner.push_indent(12, INDENT_NORMAL);
        let symbols = valid(&[DEDENT, NEWLINE]);
        let mut lexer = TestLexer::new("\n        value");
        assert!(scanner.scan(&mut lexer, &symbols));
        assert_eq!(lexer.symbol, DEDENT as u16);
        assert_eq!(lexer.end, 0);
        assert_eq!(scanner.stranded_dedent, 1);
        assert_eq!(scanner.indents, [0, 4]);

        let bytes = snapshot(&mut scanner);
        scanner.deserialize(&bytes);
        let mut lexer = TestLexer::new("\n        value");
        assert!(scanner.scan(&mut lexer, &symbols));
        assert_eq!(lexer.symbol, NEWLINE as u16);
        assert_eq!(lexer.end, 0);
        assert_eq!(scanner.stranded_dedent, 0);
        assert_eq!(scanner.line_stranded, 1);
        let mut lexer = TestLexer::new("\n        value");
        assert!(!scanner.scan(&mut lexer, &symbols));
        assert_eq!(scanner.line_stranded, 0);
    }

    #[test]
    fn underindent_treatment_depends_on_scope_kind_and_midline_flags() {
        for (kind, symbol) in [
            (INDENT_NORMAL, Some(DEDENT)),
            (INDENT_PAREN, None),
            (INDENT_TYPE_APP, None),
            (INDENT_BRACE, Some(NEWLINE)),
            (INDENT_TRY, Some(DEDENT)),
            (INDENT_KIND_MIDLINE_FLAG, None),
            (
                INDENT_KIND_MIDLINE_FLAG | INDENT_KIND_STRANDED_LINE_FLAG,
                Some(DEDENT),
            ),
        ] {
            let mut scanner = fresh();
            scanner.push_indent(4, INDENT_NORMAL);
            scanner.push_indent(12, kind);
            let mut lexer = TestLexer::new("\n        value");
            assert_eq!(
                scanner.scan(&mut lexer, &valid(&[DEDENT, NEWLINE])),
                symbol.is_some()
            );
            if let Some(symbol) = symbol {
                assert_eq!(lexer.symbol, symbol as u16);
                assert_eq!(lexer.end, 0);
            }
        }
    }

    #[test]
    fn failed_keyword_probe_keeps_zero_width_dedent() {
        let mut scanner = fresh();
        scanner.push_indent(12, INDENT_NORMAL);
        let mut lexer = TestLexer::new("\n    classy");
        assert!(scanner.scan(&mut lexer, &valid(&[CLASS, DEDENT, NEWLINE])));
        assert_eq!(lexer.symbol, DEDENT as u16);
        assert_eq!(lexer.end, 0);
        assert_eq!(lexer.lookahead(), b'y' as i32);
    }

    #[test]
    fn top_level_separator_consumes_newline_but_keyword_has_priority() {
        let mut scanner = fresh();
        let mut lexer = TestLexer::new("\nnext");
        assert!(scanner.scan(&mut lexer, &valid(&[NEWLINE, ELEM_SEP])));
        assert_eq!(lexer.symbol, ELEM_SEP as u16);
        assert_eq!(lexer.end, 1);
        let mut lexer = TestLexer::new("\nand next");
        assert!(scanner.scan(&mut lexer, &valid(&[AND, ELEM_SEP])));
        assert_eq!(lexer.symbol, AND as u16);
        assert_eq!(lexer.end, 4);
    }

    #[test]
    fn triple_quote_modes_preserve_their_different_advance_and_mark_order() {
        let mut scanner = fresh();
        let mut lexer = TestLexer::new("abc\"\"\"tail");
        assert!(scanner.scan(&mut lexer, &valid(&[TRIPLE_QUOTE_CONTENT])));
        assert_eq!(lexer.end, 3);
        assert_eq!(lexer.position, 6);
        assert_eq!(
            lexer.calls,
            [
                Call::Mark(0),
                Call::Advance(0, false),
                Call::Advance(1, false),
                Call::Advance(2, false),
                Call::Mark(3),
                Call::Advance(3, true),
                Call::Advance(4, true),
                Call::Advance(5, true),
            ]
        );

        scanner.multi_dollar_count = 2;
        let mut lexer = TestLexer::new("abc\"\"\"tail");
        assert!(scanner.scan(&mut lexer, &valid(&[MULTI_DOLLAR_TRIPLE_QUOTED_CONTENT])));
        // In C the multi-dollar branch does NOT mark before probing quotes.
        assert_eq!(lexer.end, 0);
        assert_eq!(lexer.position, 5);
        assert_eq!(
            lexer.calls,
            [
                Call::Mark(0),
                Call::Advance(0, false),
                Call::Advance(1, false),
                Call::Advance(2, false),
                Call::Advance(3, false),
                Call::Advance(4, false),
            ]
        );
        let mut lexer = TestLexer::new("abc{{expr");
        assert!(scanner.scan(&mut lexer, &valid(&[MULTI_DOLLAR_TRIPLE_QUOTED_CONTENT])));
        assert_eq!(lexer.end, 3);
        assert_eq!(lexer.position, 5);
    }

    #[test]
    fn nested_comment_probe_retains_marks_even_for_non_comment_parens() {
        let mut lexer = TestLexer::new("(*a(b (*c*)d*)z");
        assert!(scan_block_comment(&mut lexer));
        assert_eq!(lexer.lookahead(), b'z' as i32);
        let marks: Vec<_> = lexer
            .calls
            .iter()
            .filter_map(|call| match call {
                Call::Mark(pos) => Some(*pos),
                _ => None,
            })
            .collect();
        assert_eq!(marks, [0, 3, 6]);
        assert_eq!(lexer.end, 6);
        // The comment-body token explicitly declines the multiplication operator.
        let mut scanner = fresh();
        let mut lexer = TestLexer::new(")");
        assert!(!scanner.scan(&mut lexer, &valid(&[BLOCK_COMMENT_CONTENT])));
        assert!(lexer.calls.is_empty());
    }

    #[test]
    fn stray_preprocessor_else_swallows_nested_inactive_directives() {
        let mut scanner = fresh();
        let symbols = valid(&[PREPROC_INACTIVE]);
        let mut lexer = TestLexer::new("#if DEBUG\nactive");
        assert!(scanner.scan(&mut lexer, &symbols));
        assert_eq!(lexer.symbol, PREPROC_INACTIVE as u16);
        assert_eq!(lexer.end, 9);
        assert!(scanner.top_preproc_is_stray());
        let source = "#else\n#if NESTED\nignored\n#endif\n#elsewhere\n#endif // note\nnext";
        let mut lexer = TestLexer::new(source);
        assert!(scanner.scan(&mut lexer, &symbols));
        assert_eq!(lexer.symbol, PREPROC_INACTIVE as u16);
        assert_eq!(lexer.end, source.find("\nnext").unwrap());
        assert_eq!(lexer.lookahead(), b'\n' as i32);
        assert!(scanner.preproc_kinds.is_empty());
    }
}
