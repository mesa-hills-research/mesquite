//! Faithful safe-Rust translation of Perl's external scanner.
//! Quote nesting and the bounded heredoc FIFO survive scanner snapshots; all
//! speculative lookahead keeps the C scanner's advance/mark ordering.

mod intuit;
mod keywords;
mod unicode;

use intuit::{intuit_more, is_fileglob};
use ts_port_tables::{ExternalScanner, Lexer};
use unicode::{is_id_continue, is_id_start, is_whitespace};

#[derive(Clone, Copy)]
#[repr(u16)]
enum Token {
    Apostrophe,
    DoubleQuote,
    Backtick,
    SearchSlash,
    NoTokenSearchSlashPlz,
    OpenReadlineBracket,
    OpenFileglobBracket,
    PerlySemicolon,
    PerlyHeredoc,
    CtrlZ,
    QuotelikeBegin,
    QuotelikeMiddleClose,
    QuotelikeMiddleSkip,
    QuotelikeEndZw,
    QuotelikeEnd,
    QStringContent,
    QqStringContent,
    EscapeSequence,
    EscapedDelimiter,
    DollarInRegexp,
    RegexpOpenBracket,
    RegexpOpenBrace,
    Pod,
    GobbledContent,
    AttributeValueBegin,
    AttributeValue,
    Prototype,
    SignatureStart,
    HeredocDelim,
    CommandHeredocDelim,
    HeredocStart,
    HeredocMiddle,
    HeredocEnd,
    FatCommaAutoquoted,
    FatCommaAutoquotedAhead,
    Filetest,
    BraceAutoquoted,
    BraceEndZw,
    DollarIdentZw,
    NoInterpWhitespaceZw,
    Nonassoc,
    RecoverParenClose,
    RecoverBracketClose,
    RecoverBraceClose,
    RecoverArrow,
    RecoverBlockClose,
    FormatContent,
    XOp,
    KwClass,
    KwRole,
    KwMethod,
    KwAsync,
    KwTry,
    Error,
}
use Token::*;

fn emit(lexer: &mut dyn Lexer, token: Token) -> bool {
    lexer.set_result_symbol(token as u16);
    true
}

#[derive(Clone, Copy, Debug, Default)]
struct TspString {
    length: i32,
    contents: [i32; 8],
}
impl TspString {
    fn push(&mut self, c: i32) {
        if self.length < 8 {
            self.contents[self.length as usize] = c;
        }
        self.length += 1;
    }
    fn same(&self, other: &Self) -> bool {
        self.length == other.length
            && self.contents[..self.length.min(8) as usize]
                == other.contents[..other.length.min(8) as usize]
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct Quote {
    open: i32,
    close: i32,
    count: i32,
    body_leads_with_delim: bool,
}
#[derive(Clone, Copy, Debug, Default)]
struct Heredoc {
    delim: TspString,
    interpolates: bool,
    indents: bool,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(u8)]
enum HeredocState {
    #[default]
    None,
    Start,
    Unknown,
    Continue,
    End,
}

/// The payload. Inactive FIFO entries deliberately survive deserialize/reset,
/// just as in C; only live entries are serialized.
#[derive(Default)]
pub(crate) struct Scanner {
    quotes: Vec<Quote>,
    heredocs: [Heredoc; 8],
    heredoc_count: usize,
    heredoc_state: HeredocState,
    recovery_emitted: bool,
}

fn close_for_open(c: i32) -> i32 {
    match c {
        40 => 41,
        91 => 93,
        123 => 125,
        60 => 62,
        _ => 0,
    }
}

impl Scanner {
    fn push_quote(&mut self, opener: i32) {
        let closer = close_for_open(opener);
        self.quotes.push(Quote {
            open: if closer != 0 { opener } else { 0 },
            close: if closer != 0 { closer } else { opener },
            ..Quote::default()
        });
    }
    fn quote_opener(&self, c: i32) -> Option<usize> {
        self.quotes.iter().rposition(|q| q.open != 0 && q.open == c)
    }
    fn quote_closer(&self, c: i32) -> Option<usize> {
        self.quotes
            .iter()
            .rposition(|q| q.close != 0 && q.close == c)
    }
    fn take_body_lead(&mut self, c: i32) -> bool {
        if let Some(q) = self.quotes.last_mut()
            && q.body_leads_with_delim
            && c == q.open
        {
            q.body_leads_with_delim = false;
            return true;
        }
        false
    }
    fn add_heredoc(&mut self, delim: TspString, interpolates: bool, indents: bool) {
        let index = self.heredoc_count.min(7);
        if self.heredoc_count < 8 {
            self.heredoc_count += 1;
        }
        self.heredocs[index] = Heredoc {
            delim,
            interpolates,
            indents,
        };
        if self.heredoc_count == 1 {
            self.heredoc_state = HeredocState::Start;
        }
    }
    fn finish_heredoc(&mut self) {
        if self.heredoc_count > 0 {
            self.heredoc_count -= 1;
            for i in 0..self.heredoc_count {
                self.heredocs[i] = self.heredocs[i + 1];
            }
        }
        self.heredocs[self.heredoc_count] = Heredoc::default();
        self.heredoc_state = if self.heredoc_count > 0 {
            HeredocState::Start
        } else {
            HeredocState::None
        };
    }
    fn recovery(&mut self, lexer: &mut dyn Lexer, valid: &[bool], brace_ok: bool) -> bool {
        for token in [
            RecoverArrow,
            RecoverParenClose,
            RecoverBracketClose,
            RecoverBraceClose,
        ] {
            if matches!(token, RecoverBraceClose) && !brace_ok {
                continue;
            }
            if valid[token as usize] {
                self.recovery_emitted = true;
                return emit(lexer, token);
            }
        }
        false
    }
}

/// Stack-backed scratch space, capped exactly like the C lookahead buffers.
struct Lookahead<const N: usize> {
    bytes: [u8; N],
    len: usize,
}
impl<const N: usize> Lookahead<N> {
    fn new() -> Self {
        Self {
            bytes: [0; N],
            len: 0,
        }
    }
    fn len(&self) -> usize {
        self.len
    }
    fn push(&mut self, c: u8) {
        if self.len < N {
            self.bytes[self.len] = c;
            self.len += 1;
        }
    }
    fn as_slice(&self) -> &[u8] {
        &self.bytes[..self.len]
    }
    fn as_cstr(&self) -> &[u8] {
        let bytes = self.as_slice();
        &bytes[..bytes.iter().position(|&c| c == 0).unwrap_or(bytes.len())]
    }
}

// Unlike libc strchr, the source's tsp_strchr compares the full codepoint to
// each promoted ASCII byte, and also matches the string's terminating NUL.
fn strchr(s: &[u8], c: i32) -> bool {
    c == 0 || s.iter().any(|&b| i32::from(b) == c)
}
fn isidfirst(c: i32) -> bool {
    c == b'_' as i32 || is_id_start(c)
}
fn isidcont(c: i32) -> bool {
    c == b'_' as i32 || is_id_continue(c)
}
fn is_interpolation_escape(c: i32) -> bool {
    c < 256 && strchr(b"$@-[{\\", c)
}
fn skip_whitespace(lexer: &mut dyn Lexer) {
    while lexer.lookahead() != 0 && is_whitespace(lexer.lookahead()) {
        lexer.advance(true);
    }
}
fn skip_ws_to_eol(lexer: &mut dyn Lexer) -> bool {
    loop {
        let c = lexer.lookahead();
        if c == 0 || !is_whitespace(c) {
            return false;
        }
        lexer.advance(true);
        if c == b'\n' as i32 {
            return true;
        }
    }
}
fn skip_chars(lexer: &mut dyn Lexer, mut maxlen: i32, allow: &[u8]) {
    while maxlen != 0 && lexer.lookahead() != 0 && strchr(allow, lexer.lookahead()) {
        lexer.advance(false);
        if maxlen > 0 {
            maxlen -= 1;
        }
    }
}
fn skip_braced(lexer: &mut dyn Lexer) {
    if lexer.lookahead() != b'{' as i32 {
        return;
    }
    lexer.advance(false);
    while lexer.lookahead() != 0 && lexer.lookahead() != b'}' as i32 {
        lexer.advance(false);
    }
    // C also advances when the braced escape reached EOF without a closer.
    lexer.advance(false);
}

fn keyword_starter(c: i32) -> bool {
    c != 0 && strchr(b"cmnprsu", c)
}
#[derive(PartialEq, Eq)]
enum Peek {
    NoMatch,
    Keyword,
    FatComma,
    AutoquoteKey,
    NotKeyword,
}
fn peek_statement_keyword(lexer: &mut dyn Lexer) -> (Peek, bool) {
    let mut la = lexer.lookahead();
    if !keyword_starter(la) {
        return (Peek::NoMatch, false);
    }
    let mut word = Lookahead::<15>::new();
    while la != 0 && strchr(b"abcdeghklmnoprstu", la) {
        if word.len() < 15 {
            word.push(la as u8);
        }
        lexer.advance(false);
        la = lexer.lookahead();
    }
    let always = !isidcont(la)
        && matches!(
            word.as_slice(),
            b"package" | b"use" | b"no" | b"class" | b"role"
        );
    let needs_name = !isidcont(la) && matches!(word.as_slice(), b"sub" | b"method");
    if !always && !needs_name {
        while isidcont(la) {
            lexer.advance(false);
            la = lexer.lookahead();
        }
        lexer.mark_end();
        while is_whitespace(la) {
            lexer.advance(false);
            la = lexer.lookahead();
        }
        return (
            if la == b'=' as i32 {
                Peek::AutoquoteKey
            } else {
                Peek::NotKeyword
            },
            false,
        );
    }
    let structural = matches!(word.as_slice(), b"method" | b"class" | b"role");
    while is_whitespace(la) {
        lexer.advance(true);
        la = lexer.lookahead();
    }
    let peek = if la == b'=' as i32 {
        Peek::FatComma
    } else if needs_name && !isidfirst(la) {
        Peek::NotKeyword
    } else {
        Peek::Keyword
    };
    (peek, structural)
}

// Shared tails replace C's forward gotos, without restoring lexer position or
// token marks. `c` can deliberately be stale relative to lexer.lookahead().
fn fat_comma_check(lexer: &mut dyn Lexer, valid: &[bool]) -> bool {
    let c1 = lexer.lookahead();
    lexer.advance(false);
    let c = lexer.lookahead();
    if valid[FatCommaAutoquoted as usize] && c1 == b'=' as i32 && c == b'>' as i32 {
        return emit(lexer, FatCommaAutoquoted);
    }
    if valid[BraceAutoquoted as usize] && c1 == b'}' as i32 {
        return emit(lexer, BraceAutoquoted);
    }
    false
}
fn kw_autoquote(lexer: &mut dyn Lexer, valid: &[bool], mut c: i32) -> bool {
    while is_whitespace(c) || c == b'#' as i32 {
        while is_whitespace(c) {
            lexer.advance(false);
            c = lexer.lookahead();
        }
        if c == b'#' as i32 {
            lexer.advance(false);
            c = lexer.lookahead();
            while lexer.get_column() != 0 && !lexer.eof() {
                lexer.advance(false);
                c = lexer.lookahead();
            }
        }
        if lexer.eof() {
            return false;
        }
    }
    fat_comma_check(lexer, valid)
}
fn heredoc_token_handling(lexer: &mut dyn Lexer) -> bool {
    lexer.advance(false);
    lexer.mark_end();
    let c = lexer.lookahead();
    if c == b'\\' as i32 || c == b'~' as i32 || isidfirst(c) {
        return emit(lexer, PerlyHeredoc);
    }
    skip_whitespace(lexer);
    if strchr(b"'\"`", lexer.lookahead()) && lexer.lookahead() != 0 {
        return emit(lexer, PerlyHeredoc);
    }
    false
}

// C ABI snapshot sizes: TSPString is 36 bytes; TSPQuote is 16 (three
// native-endian i32s, one bool, three zero padding bytes).
const MAX_SERIALIZED_QUOTES: usize = (1024 - (1 + 1 + 1 + 8 * (2 + 36) + 1)) / 16;

impl ExternalScanner for Scanner {
    fn serialize(&mut self, buffer: &mut [u8]) -> usize {
        let count = self.quotes.len().min(MAX_SERIALIZED_QUOTES);
        let size = 1 + count * 16 + 2 + self.heredoc_count * 38 + 1;
        if buffer.len() < size {
            return 0;
        }
        buffer[..size].fill(0);
        buffer[0] = count as u8;
        let mut p = 1;
        for q in &self.quotes[..count] {
            buffer[p..p + 4].copy_from_slice(&q.open.to_ne_bytes());
            buffer[p + 4..p + 8].copy_from_slice(&q.close.to_ne_bytes());
            buffer[p + 8..p + 12].copy_from_slice(&q.count.to_ne_bytes());
            buffer[p + 12] = u8::from(q.body_leads_with_delim);
            p += 16;
        }
        buffer[p] = self.heredoc_state as u8;
        buffer[p + 1] = self.heredoc_count as u8;
        p += 2;
        for h in &self.heredocs[..self.heredoc_count] {
            buffer[p] = u8::from(h.interpolates);
            buffer[p + 1] = u8::from(h.indents);
            buffer[p + 2..p + 6].copy_from_slice(&h.delim.length.to_ne_bytes());
            p += 6;
            for c in h.delim.contents {
                buffer[p..p + 4].copy_from_slice(&c.to_ne_bytes());
                p += 4;
            }
        }
        buffer[p] = u8::from(self.recovery_emitted);
        size
    }
    fn deserialize(&mut self, buffer: &[u8]) {
        self.quotes.clear();
        self.heredoc_count = 0;
        self.heredoc_state = HeredocState::None;
        self.recovery_emitted = false;
        if buffer.is_empty() {
            return;
        }
        let count = usize::from(buffer[0]).min(MAX_SERIALIZED_QUOTES);
        let mut p = 1 + count * 16;
        // The C scanner assumes well-formed snapshots. Reject truncated ones
        // safely instead of reading outside the supplied buffer.
        if buffer.len() < p + 3 {
            return;
        }
        let hc = usize::from(buffer[p + 1]).min(8);
        if buffer.len() < p + 3 + hc * 38 {
            return;
        }
        let read_i32 = |p| i32::from_ne_bytes(buffer[p..p + 4].try_into().unwrap());
        for i in 0..count {
            let q = 1 + i * 16;
            self.quotes.push(Quote {
                open: read_i32(q),
                close: read_i32(q + 4),
                count: read_i32(q + 8),
                body_leads_with_delim: buffer[q + 12] != 0,
            });
        }
        self.heredoc_state = match buffer[p] {
            1 => HeredocState::Start,
            2 => HeredocState::Unknown,
            3 => HeredocState::Continue,
            4 => HeredocState::End,
            _ => HeredocState::None,
        };
        self.heredoc_count = hc;
        p += 2;
        for h in &mut self.heredocs[..hc] {
            h.interpolates = buffer[p] != 0;
            h.indents = buffer[p + 1] != 0;
            h.delim.length = read_i32(p + 2);
            p += 6;
            for c in &mut h.delim.contents {
                *c = read_i32(p);
                p += 4;
            }
        }
        self.recovery_emitted = buffer[p] != 0;
    }

    // The cached lookahead and its refreshes deliberately mirror ADVANCE_C,
    // including refreshes just before returning a token.
    #[allow(unused_assignments)]
    fn scan(&mut self, lexer: &mut dyn Lexer, valid: &[bool]) -> bool {
        let v = |token: Token| valid[token as usize];
        let is_error = v(Error);
        let mut skipped_whitespace = false;
        let recovery_emitted = self.recovery_emitted;
        self.recovery_emitted = false;
        let mut c = lexer.lookahead();
        macro_rules! advance {
            () => {{
                lexer.advance(false);
                c = lexer.lookahead();
            }};
        }
        macro_rules! token {
            ($t:expr) => {
                return emit(lexer, $t)
            };
        }

        if !is_error && v(GobbledContent) {
            while !lexer.eof() {
                advance!();
            }
            token!(GobbledContent);
        }
        if !is_error && v(FormatContent) {
            while c != b'\n' as i32 && !lexer.eof() {
                advance!();
            }
            if c == b'\n' as i32 {
                advance!();
            }
            while !lexer.eof() {
                let mut only_dot = false;
                if c == b'.' as i32 {
                    advance!();
                    only_dot = true;
                    while matches!(c, 32 | 9 | 13) {
                        advance!();
                    }
                    if c != b'\n' as i32 && !lexer.eof() {
                        only_dot = false;
                    }
                }
                if only_dot {
                    if c == b'\n' as i32 {
                        advance!();
                    }
                    break;
                }
                while c != b'\n' as i32 && !lexer.eof() {
                    advance!();
                }
                if c == b'\n' as i32 {
                    advance!();
                }
            }
            lexer.mark_end();
            token!(FormatContent);
        }
        if !is_error && v(Nonassoc) {
            token!(Nonassoc);
        }
        if !is_error && v(XOp) && c == b'x' as i32 {
            advance!();
            if (b'0' as i32..=b'9' as i32).contains(&c) {
                lexer.mark_end();
                token!(XOp);
            }
            return false;
        }
        if v(HeredocMiddle) && !is_error && self.heredoc_count > 0 {
            let front = self.heredocs[0];
            if self.heredoc_state != HeredocState::Continue {
                let mut line = TspString::default();
                while !lexer.eof() {
                    line.length = 0;
                    let valid_start =
                        self.heredoc_state == HeredocState::End || lexer.get_column() == 0;
                    let mut saw_escape = false;
                    if valid_start && front.indents {
                        skip_whitespace(lexer);
                        c = lexer.lookahead();
                    }
                    lexer.mark_end();
                    while c != b'\n' as i32 && !lexer.eof() {
                        if c == b'\r' as i32 {
                            advance!();
                            if c == b'\n' as i32 {
                                break;
                            }
                            line.push(b'\r' as i32);
                        }
                        line.push(c);
                        if matches!(c, 36 | 64 | 92) {
                            saw_escape = true;
                        }
                        advance!();
                    }
                    if valid_start && line.same(&front.delim) {
                        if self.heredoc_state != HeredocState::End {
                            self.heredoc_state = HeredocState::End;
                            token!(HeredocMiddle);
                        }
                        lexer.mark_end();
                        self.finish_heredoc();
                        token!(HeredocEnd);
                    }
                    if saw_escape && front.interpolates {
                        self.heredoc_state = HeredocState::Continue;
                        token!(HeredocMiddle);
                    }
                    advance!();
                }
            } else {
                let mut saw_chars = false;
                loop {
                    if is_interpolation_escape(c) {
                        lexer.mark_end();
                        break;
                    }
                    if c == b'\n' as i32 {
                        lexer.mark_end();
                        self.heredoc_state = HeredocState::Unknown;
                        token!(HeredocMiddle);
                    }
                    saw_chars = true;
                    advance!();
                }
                if saw_chars {
                    token!(HeredocMiddle);
                }
            }
        }
        // iswspace in the reference's C locale (not the Unicode whitespace table).
        if !is_error && matches!(c, 9..=13 | 32) && v(NoInterpWhitespaceZw) {
            token!(NoInterpWhitespaceZw);
        }
        let crossed_newline = skip_ws_to_eol(lexer);
        // Do NOT refresh c here. The attribute handlers in C see the cached
        // character from before skip_ws_to_eol.
        if v(HeredocStart) && self.heredoc_state == HeredocState::Start && lexer.get_column() == 0 {
            self.heredoc_state = HeredocState::Unknown;
            token!(HeredocStart);
        }
        if !is_error && v(AttributeValueBegin) && c == b'(' as i32 {
            token!(AttributeValueBegin);
        }
        if !is_error && v(AttributeValue) {
            let mut delimcount = 0;
            while !lexer.eof() {
                if c == b'\\' as i32 {
                    advance!();
                } else if c == b'(' as i32 {
                    delimcount += 1;
                } else if c == b')' as i32 {
                    if delimcount != 0 {
                        delimcount -= 1;
                    } else {
                        break;
                    }
                }
                advance!();
            }
            token!(AttributeValue);
        }
        if is_whitespace(c) {
            skipped_whitespace = true;
            skip_whitespace(lexer);
            c = lexer.lookahead();
        }
        if c == 26 && v(CtrlZ) {
            token!(CtrlZ);
        }
        let any_recovery_valid = [
            RecoverArrow,
            RecoverParenClose,
            RecoverBracketClose,
            RecoverBraceClose,
            RecoverBlockClose,
            PerlySemicolon,
        ]
        .into_iter()
        .any(v);
        let oo_kw_valid = v(KwClass) || v(KwRole) || v(KwMethod);
        let asynctry_valid = v(KwAsync) || v(KwTry);
        let recovery_pending = any_recovery_valid && (crossed_newline || recovery_emitted);
        let enter_oo = oo_kw_valid && !recovery_pending && matches!(c, 99 | 114 | 109);
        let enter_asynctry = asynctry_valid && matches!(c, 97 | 116);
        if !is_error && (enter_oo || enter_asynctry) {
            let mut word = Lookahead::<7>::new();
            while isidcont(c) {
                if word.len() < 7 {
                    word.push(c as u8);
                }
                advance!();
            }
            lexer.mark_end();
            while is_whitespace(c) {
                advance!();
            }
            // The C word buffers narrow codepoints to char and strcmp stops at
            // the first NUL, including NULs produced by that narrowing.
            let word = word.as_cstr();
            let is_class = !recovery_pending && v(KwClass) && word == b"class";
            let is_role = !recovery_pending && v(KwRole) && word == b"role";
            let is_method = !recovery_pending && v(KwMethod) && word == b"method";
            let is_async = v(KwAsync) && word == b"async";
            let is_try = v(KwTry) && word == b"try";
            if is_try && c == b'{' as i32 {
                token!(KwTry);
            }
            if is_async {
                if c == b'{' as i32 {
                    token!(KwAsync);
                }
                if matches!(c, 115 | 109 | 101) {
                    let mut nword = Lookahead::<8>::new();
                    while isidcont(c) {
                        if nword.len() < 8 {
                            nword.push(c as u8);
                        }
                        advance!();
                    }
                    let nword = nword.as_cstr();
                    if matches!(nword, b"sub" | b"method" | b"extended") {
                        token!(KwAsync);
                    }
                    return false;
                }
            }
            if (is_class || is_role) && isidfirst(c) {
                token!(if is_class { KwClass } else { KwRole });
            }
            if is_method {
                if c == b'{' as i32 || c == b'(' as i32 {
                    token!(KwMethod);
                }
                if isidfirst(c) {
                    while isidcont(c) {
                        advance!();
                    }
                    while is_whitespace(c) {
                        advance!();
                    }
                    if c != b'=' as i32 {
                        token!(KwMethod);
                    }
                    return false;
                }
            }
            return kw_autoquote(lexer, valid, c);
        }
        if !is_error
            && (c == b'}' as i32 || c == b';' as i32 || lexer.eof())
            && self.recovery(lexer, valid, c != b'}' as i32)
        {
            return true;
        }
        if v(PerlySemicolon) && (c == b'}' as i32 || lexer.eof()) && (is_error || !v(BraceEndZw)) {
            token!(PerlySemicolon);
        }
        if lexer.eof() {
            return false;
        }
        if (crossed_newline || recovery_emitted)
            && !is_error
            && any_recovery_valid
            && keyword_starter(c)
        {
            lexer.mark_end();
            let (peek, structural) = peek_statement_keyword(lexer);
            if peek == Peek::Keyword {
                if self.recovery(lexer, valid, true) {
                    return true;
                }
                if structural && v(RecoverBlockClose) {
                    self.recovery_emitted = true;
                    token!(RecoverBlockClose);
                }
                if v(PerlySemicolon) {
                    self.recovery_emitted = true;
                    token!(PerlySemicolon);
                }
            }
            if peek == Peek::FatComma {
                if v(FatCommaAutoquotedAhead) {
                    token!(FatCommaAutoquotedAhead);
                }
                lexer.mark_end();
                c = lexer.lookahead();
                return fat_comma_check(lexer, valid);
            }
            if peek == Peek::AutoquoteKey {
                c = lexer.lookahead();
                return fat_comma_check(lexer, valid);
            }
            if peek == Peek::NotKeyword {
                return false;
            }
        }
        if (v(OpenFileglobBracket) || v(OpenReadlineBracket) || v(PerlyHeredoc)) && c == b'<' as i32
        {
            advance!();
            lexer.mark_end();
            if c == b'<' as i32 {
                return heredoc_token_handling(lexer);
            }
            let mut content = Lookahead::<256>::new();
            if c == b'$' as i32 {
                content.push(b'$');
                advance!();
            }
            while isidcont(c) {
                if content.len() < 256 {
                    content.push(if c < 0x80 { c as u8 } else { 0x7f });
                }
                advance!();
            }
            if c == b'>' as i32 {
                token!(OpenReadlineBracket);
            }
            if v(OpenFileglobBracket) {
                while !matches!(c, 62 | 60 | 59 | 10) && !lexer.eof() {
                    if content.len() < 256 {
                        content.push(if c < 0x80 { c as u8 } else { 0x7f });
                    }
                    advance!();
                }
                if c == b'>' as i32 {
                    advance!();
                    let mut after = Lookahead::<256>::new();
                    while after.len() < 256 && c != b'\n' as i32 && !lexer.eof() {
                        after.push(if c < 0x80 { c as u8 } else { 0x7f });
                        advance!();
                    }
                    if is_fileglob(content.as_slice(), after.as_slice()) {
                        self.push_quote(b'<' as i32);
                        token!(OpenFileglobBracket);
                    }
                }
            }
            return false;
        }
        if !is_error
            && v(DollarIdentZw)
            && !strchr(b"${", c)
            && (skipped_whitespace || !isidcont(c))
        {
            if c == b':' as i32 {
                lexer.mark_end();
                advance!();
                if c == b':' as i32 {
                    return false;
                }
            }
            token!(DollarIdentZw);
        }
        if v(SearchSlash) && c == b'/' as i32 && !v(NoTokenSearchSlashPlz) {
            advance!();
            lexer.mark_end();
            if c != b'/' as i32 {
                self.push_quote(b'/' as i32);
                token!(SearchSlash);
            }
            return false;
        }
        if v(Apostrophe) && c == b'\'' as i32 {
            advance!();
            self.push_quote(b'\'' as i32);
            token!(Apostrophe);
        }
        if v(DoubleQuote) && c == b'"' as i32 {
            advance!();
            self.push_quote(b'"' as i32);
            token!(DoubleQuote);
        }
        if v(Backtick) && c == b'`' as i32 {
            advance!();
            self.push_quote(b'`' as i32);
            token!(Backtick);
        }
        if v(DollarInRegexp) && c == b'$' as i32 {
            advance!();
            if self.quote_closer(c).is_some() || matches!(c, 40 | 41 | 124) {
                token!(DollarInRegexp);
            }
            return false;
        }
        if ((v(RegexpOpenBracket) && c == b'[' as i32) || (v(RegexpOpenBrace) && c == b'{' as i32))
            && !self.take_body_lead(c)
        {
            let open = c;
            let close = if open == b'[' as i32 {
                b']' as i32
            } else {
                b'}' as i32
            };
            let mut buf = Lookahead::<256>::new();
            buf.push(open as u8);
            advance!();
            lexer.mark_end();
            while buf.len() < 256 && c != 0 && !lexer.eof() {
                buf.push(if c < 0x80 { c as u8 } else { 0x7f });
                if c == close {
                    break;
                }
                advance!();
            }
            if !intuit_more(buf.as_slice()) {
                if let Some(qi) = self.quote_opener(open) {
                    self.quotes[qi].count += 1;
                }
                token!(if open == b'[' as i32 {
                    RegexpOpenBracket
                } else {
                    RegexpOpenBrace
                });
            }
            return false;
        }
        if v(Pod) {
            let column = lexer.get_column();
            if column == 0 && c == b'=' as i32 {
                let mut stage: i32 = -1;
                while !lexer.eof() {
                    if c == b'\r' as i32 { /* ignored */
                    } else if stage < 1 && c == b'\n' as i32 {
                        stage = 0;
                    } else if (0..4).contains(&stage) && c == i32::from(b"=cut"[stage as usize]) {
                        stage += 1;
                    } else if stage == 4 && matches!(c, 32 | 9) {
                        stage = 5;
                    } else if stage == 4 && c == b'\n' as i32 {
                        stage = 6;
                    } else {
                        stage = -1;
                    }
                    if stage > 4 {
                        break;
                    }
                    advance!();
                }
                if stage < 6 {
                    while !lexer.eof() {
                        if c == b'\n' as i32 {
                            break;
                        }
                        advance!();
                    }
                }
                token!(Pod);
            }
        }
        if is_error {
            return false;
        }
        if v(HeredocDelim) || v(CommandHeredocDelim) {
            let mut should_indent = false;
            let mut should_interpolate = true;
            let mut delim = TspString::default();
            if !skipped_whitespace {
                if c == b'~' as i32 {
                    advance!();
                    should_indent = true;
                }
                if c == b'\\' as i32 {
                    advance!();
                    should_interpolate = false;
                }
                if isidfirst(c) {
                    while isidcont(c) {
                        delim.push(c);
                        advance!();
                    }
                    self.add_heredoc(delim, should_interpolate, should_indent);
                    token!(HeredocDelim);
                }
            }
            if should_indent {
                skip_whitespace(lexer);
                c = lexer.lookahead();
            }
            if should_interpolate && matches!(c, 39 | 34 | 96) {
                let delim_open = c;
                should_interpolate = c != b'\'' as i32;
                advance!();
                while c != delim_open && !lexer.eof() {
                    if c == b'\\' as i32 {
                        let mut to_add = c;
                        advance!();
                        if c == delim_open {
                            to_add = delim_open;
                            advance!();
                        }
                        delim.push(to_add);
                    } else {
                        delim.push(c);
                        advance!();
                    }
                }
                if c == delim_open {
                    advance!();
                    self.add_heredoc(delim, should_interpolate, should_indent);
                    if delim_open == b'`' as i32 {
                        token!(CommandHeredocDelim);
                    }
                    token!(HeredocDelim);
                }
            }
        }
        if v(QuotelikeMiddleSkip) && self.quotes.last().is_some_and(|q| q.open == 0) {
            token!(QuotelikeMiddleSkip);
        }
        if v(QuotelikeBegin) {
            let delim = c;
            if skipped_whitespace && c == b'#' as i32 {
                return false;
            }
            lexer.mark_end();
            advance!();
            if v(BraceEndZw) && delim == b'}' as i32 {
                token!(BraceEndZw);
            }
            lexer.mark_end();
            if v(QuotelikeMiddleSkip) {
                self.quotes.pop();
            }
            self.push_quote(delim);
            if close_for_open(delim) != 0 {
                while is_whitespace(c) {
                    advance!();
                }
                if c == delim {
                    self.quotes.last_mut().unwrap().body_leads_with_delim = true;
                }
            }
            token!(QuotelikeBegin);
        }
        if c == b'\\' as i32 && !(v(QuotelikeEnd) && self.quote_closer(b'\\' as i32).is_some()) {
            advance!();
            let esc_c = c;
            if !is_whitespace(c) {
                advance!();
            }
            if v(EscapedDelimiter)
                && (self.quote_opener(esc_c).is_some() || self.quote_closer(esc_c).is_some())
            {
                lexer.mark_end();
                token!(EscapedDelimiter);
            }
            if v(EscapeSequence) {
                lexer.mark_end();
                if esc_c == b'\\' as i32 {
                    token!(EscapeSequence);
                }
                if v(QStringContent) {
                    token!(QStringContent);
                }
                match esc_c {
                    120 => {
                        if c == b'{' as i32 {
                            skip_braced(lexer);
                        } else {
                            skip_chars(lexer, 2, b"0123456789ABCDEFabcdef");
                        }
                    }
                    78 | 111 => skip_braced(lexer),
                    48 => skip_chars(lexer, 3, b"01234567"),
                    _ => {}
                }
                token!(EscapeSequence);
            }
        }
        if v(QStringContent) || v(QqStringContent) {
            let is_qq = v(QqStringContent);
            let mut has_content = false;
            while c != 0 {
                if c == b'\\' as i32 {
                    break;
                }
                if let Some(qi) = self.quote_opener(c) {
                    self.quotes[qi].count += 1;
                } else if let Some(qi) = self.quote_closer(c) {
                    if self.quotes[qi].count == 0 {
                        break;
                    }
                    self.quotes[qi].count -= 1;
                } else if is_qq && is_interpolation_escape(c) {
                    break;
                }
                has_content = true;
                advance!();
            }
            if has_content {
                token!(if is_qq {
                    QqStringContent
                } else {
                    QStringContent
                });
            }
        }
        if v(QuotelikeMiddleClose)
            && let Some(qi) = self.quote_closer(c)
            && self.quotes[qi].count == 0
        {
            advance!();
            token!(QuotelikeMiddleClose);
        }
        if v(QuotelikeEnd)
            && let Some(qi) = self.quote_closer(c)
        {
            if v(QuotelikeEndZw) {
                token!(QuotelikeEndZw);
            }
            advance!();
            self.quotes.remove(qi);
            token!(QuotelikeEnd);
        }
        if c == b'(' as i32 && (v(Prototype) || v(SignatureStart)) {
            advance!();
            lexer.mark_end();
            let mut count = 0;
            while !lexer.eof() {
                if c == b')' as i32 && count == 0 {
                    advance!();
                    break;
                } else if c == b')' as i32 {
                    count -= 1;
                } else if c == b'(' as i32 {
                    count += 1;
                } else if is_id_continue(c) {
                    token!(SignatureStart);
                }
                advance!();
            }
            lexer.mark_end();
            token!(Prototype);
        }
        let c1 = c;
        if c == b'-' as i32 && v(Filetest) {
            advance!();
            if strchr(b"rwxoRWXOezsfdlpSbctugkTBMAC", c) {
                advance!();
                if !isidcont(c) {
                    token!(Filetest);
                }
            }
            return false;
        }
        if v(BraceAutoquoted)
            && !isidfirst(c)
            && c > b' ' as i32
            && !matches!(c, 125 | 123 | 94 | 35)
            && !(b'0' as i32..=b'9' as i32).contains(&c)
        {
            advance!();
            lexer.mark_end();
            while is_whitespace(c) {
                advance!();
            }
            if c == b'}' as i32 {
                token!(BraceAutoquoted);
            }
            return false;
        }
        if isidfirst(c) && (v(FatCommaAutoquoted) || v(BraceAutoquoted)) {
            loop {
                advance!();
                if c == 0 || !isidcont(c) {
                    break;
                }
            }
            lexer.mark_end();
            return kw_autoquote(lexer, valid, c);
        } else {
            lexer.mark_end();
            advance!();
            let c2 = c;
            if lexer.eof() {
                return false;
            }
            // Not validity-guarded: intentionally kills inappropriate GLR branches.
            if c1 == b'<' as i32 && c2 == b'<' as i32 {
                return heredoc_token_handling(lexer);
            }
            if v(BraceEndZw) && c1 == b'}' as i32 {
                token!(BraceEndZw);
            }
        }
        false
    }
}

pub(crate) fn create() -> Box<dyn ExternalScanner> {
    Box::new(Scanner::default())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Eq)]
    enum Event {
        Advance(usize, bool),
        Mark(usize),
    }

    struct TestLexer {
        input: Vec<i32>,
        pos: usize,
        end: Option<usize>,
        result: u16,
        events: Vec<Event>,
    }
    impl TestLexer {
        fn new(text: &str) -> Self {
            Self {
                input: text.chars().map(|c| c as i32).collect(),
                pos: 0,
                end: None,
                result: u16::MAX,
                events: Vec::new(),
            }
        }
        fn token_end(&self) -> usize {
            self.end.unwrap_or(self.pos)
        }
    }
    impl Lexer for TestLexer {
        fn lookahead(&self) -> i32 {
            self.input.get(self.pos).copied().unwrap_or(0)
        }
        fn result_symbol(&self) -> u16 {
            self.result
        }
        fn set_result_symbol(&mut self, symbol: u16) {
            self.result = symbol;
        }
        fn advance(&mut self, skip: bool) {
            self.events.push(Event::Advance(self.pos, skip));
            if self.pos < self.input.len() {
                self.pos += 1;
            }
        }
        fn mark_end(&mut self) {
            self.end = Some(self.pos);
            self.events.push(Event::Mark(self.pos));
        }
        fn get_column(&mut self) -> u32 {
            self.input[..self.pos]
                .iter()
                .rev()
                .take_while(|&&c| c != b'\n' as i32)
                .map(|&c| char::from_u32(c as u32).unwrap().len_utf8() as u32)
                .sum()
        }
        fn is_at_included_range_start(&self) -> bool {
            false
        }
        fn eof(&self) -> bool {
            self.pos == self.input.len()
        }
    }
    fn scan(scanner: &mut Scanner, lexer: &mut TestLexer, valid: &[Token]) -> bool {
        let mut symbols = [false; Error as usize + 1];
        for &token in valid {
            symbols[token as usize] = true;
        }
        scanner.scan(lexer, &symbols)
    }
    fn string(text: &str) -> TspString {
        let mut result = TspString::default();
        for c in text.chars() {
            result.push(c as i32);
        }
        result
    }

    #[test]
    fn snapshot_bytes_and_padding() {
        let mut scanner = Scanner::default();
        scanner.quotes.push(Quote {
            open: 0x12345,
            close: 0x54321,
            count: 7,
            body_leads_with_delim: true,
        });
        scanner.add_heredoc(string("λEND"), true, false);
        scanner.recovery_emitted = true;
        let mut buffer = [0xff; 1024];
        let size = scanner.serialize(&mut buffer);
        let mut expected = vec![1];
        expected.extend_from_slice(&0x12345i32.to_ne_bytes());
        expected.extend_from_slice(&0x54321i32.to_ne_bytes());
        expected.extend_from_slice(&7i32.to_ne_bytes());
        expected.extend_from_slice(&[1, 0, 0, 0]);
        expected.extend_from_slice(&[HeredocState::Start as u8, 1, 1, 0]);
        expected.extend_from_slice(&4i32.to_ne_bytes());
        for c in ['λ' as i32, 'E' as i32, 'N' as i32, 'D' as i32, 0, 0, 0, 0] {
            expected.extend_from_slice(&c.to_ne_bytes());
        }
        expected.push(1);
        assert_eq!(&buffer[..size], expected);
        assert_eq!(size, 58);
        let mut restored = Scanner::default();
        restored.deserialize(&buffer[..size]);
        let mut round_trip = [0; 1024];
        assert_eq!(restored.serialize(&mut round_trip), size);
        assert_eq!(&round_trip[..size], expected);
        restored.deserialize(&[]);
        assert!(restored.quotes.is_empty());
        assert_eq!(restored.heredoc_count, 0);
        assert_eq!(restored.heredoc_state, HeredocState::None);
        assert!(!restored.recovery_emitted);
        assert!(restored.heredocs[0].delim.same(&string("λEND")));
        assert_eq!(restored.serialize(&mut round_trip), 4);
        assert_eq!(&round_trip[..4], &[0, 0, 0, 0]);
    }

    #[test]
    fn fifo_overflow_and_snapshot_quote_cap() {
        let mut scanner = Scanner::default();
        for i in 0..10 {
            scanner.add_heredoc(string(&format!("END{i}")), i % 2 == 0, i % 3 == 0);
        }
        assert_eq!(scanner.heredoc_count, 8);
        assert!(scanner.heredocs[6].delim.same(&string("END6")));
        assert!(scanner.heredocs[7].delim.same(&string("END9")));
        for c in 0..60 {
            scanner.push_quote(0x1000 + c);
        }
        let mut buffer = [0; 1024];
        assert_eq!(scanner.serialize(&mut buffer), 1012);
        assert_eq!(buffer[0], 44);
        assert_eq!(scanner.quotes.len(), 60); // Serialize is not destructive.
        let mut restored = Scanner::default();
        restored.deserialize(&buffer[..1012]);
        assert_eq!(restored.quotes.len(), 44);
        assert_eq!(restored.quotes.last().unwrap().close, 0x1000 + 43);
        for name in [
            "END0", "END1", "END2", "END3", "END4", "END5", "END6", "END9",
        ] {
            assert!(restored.heredocs[0].delim.same(&string(name)));
            restored.finish_heredoc();
            assert_eq!(
                restored.heredoc_state,
                if restored.heredoc_count == 0 {
                    HeredocState::None
                } else {
                    HeredocState::Start
                }
            );
        }
        assert_eq!(restored.heredocs[0].delim.length, 0);
    }

    #[test]
    fn delimiter_comparison_keeps_length_but_only_eight_codepoints() {
        assert!(string("abcdefghX").same(&string("abcdefghY")));
        assert!(!string("abcdefghX").same(&string("abcdefghXY")));
        assert!(!string("abcdefghX").same(&string("abcdefgYX")));
    }

    #[test]
    fn escape_lookahead_does_not_move_the_mark() {
        let mut scanner = Scanner::default();
        let mut lexer = TestLexer::new("\\x{41}");
        assert!(scan(&mut scanner, &mut lexer, &[EscapeSequence]));
        assert_eq!(lexer.result, EscapeSequence as u16);
        assert_eq!(lexer.token_end(), 2);
        assert_eq!(lexer.pos, 6);
        assert_eq!(
            lexer.events,
            [
                Event::Advance(0, false),
                Event::Advance(1, false),
                Event::Mark(2),
                Event::Advance(2, false),
                Event::Advance(3, false),
                Event::Advance(4, false),
                Event::Advance(5, false)
            ]
        );
    }

    #[test]
    fn pattern_leading_bracket_and_quote_replacement() {
        let mut scanner = Scanner::default();
        let mut lexer = TestLexer::new("[ [abc]]");
        assert!(scan(&mut scanner, &mut lexer, &[QuotelikeBegin]));
        assert_eq!(lexer.token_end(), 1);
        assert_eq!(lexer.pos, 2);
        assert!(scanner.quotes[0].body_leads_with_delim);
        let mut lexer = TestLexer::new(" [abc]]");
        assert!(scan(
            &mut scanner,
            &mut lexer,
            &[RegexpOpenBracket, QqStringContent]
        ));
        assert_eq!(lexer.result, QqStringContent as u16);
        assert_eq!(lexer.token_end(), 6);
        assert!(!scanner.quotes[0].body_leads_with_delim);
        assert_eq!(scanner.quotes[0].count, 0);
        let mut lexer = TestLexer::new("]");
        assert!(scan(&mut scanner, &mut lexer, &[QuotelikeMiddleClose]));
        assert_eq!(scanner.quotes.len(), 1);
        let mut lexer = TestLexer::new("{replacement}");
        assert!(scan(
            &mut scanner,
            &mut lexer,
            &[QuotelikeMiddleSkip, QuotelikeBegin]
        ));
        assert_eq!(scanner.quotes.len(), 1);
        assert_eq!(scanner.quotes[0].close, b'}' as i32);
    }

    #[test]
    fn recovery_keyword_and_plain_autoquote_use_different_marks() {
        let mut scanner = Scanner::default();
        let mut lexer = TestLexer::new(" \nmethod foo {");
        assert!(scan(&mut scanner, &mut lexer, &[RecoverBracketClose]));
        assert_eq!(lexer.result, RecoverBracketClose as u16);
        assert_eq!(lexer.token_end(), 2);
        assert_eq!(lexer.pos, 9);
        assert!(scanner.recovery_emitted);
        assert_eq!(lexer.events.last(), Some(&Event::Advance(8, true)));
        let mut lexer = TestLexer::new("\nname => 1");
        assert!(scan(
            &mut scanner,
            &mut lexer,
            &[RecoverBracketClose, FatCommaAutoquoted]
        ));
        assert_eq!(lexer.result, FatCommaAutoquoted as u16);
        assert_eq!(lexer.token_end(), 5);
        assert_eq!(lexer.pos, 7);
        assert!(!scanner.recovery_emitted);
    }

    #[test]
    fn contextual_keyword_probe_does_not_autoquote_the_next_word() {
        let mut scanner = Scanner::default();
        let mut lexer = TestLexer::new("async send=>1");
        assert!(!scan(
            &mut scanner,
            &mut lexer,
            &[KwAsync, FatCommaAutoquoted]
        ));
        assert_eq!(lexer.token_end(), 5);
        assert_eq!(lexer.pos, 10);
        // Buffer truncation and C-char narrowing are deliberate source quirks.
        let mut lexer = TestLexer::new("async extendedName");
        assert!(scan(&mut scanner, &mut lexer, &[KwAsync]));
        assert_eq!(lexer.result, KwAsync as u16);
        let mut lexer = TestLexer::new("classĀ Foo");
        assert!(scan(&mut scanner, &mut lexer, &[KwClass]));
        assert_eq!(lexer.result, KwClass as u16);
    }

    #[test]
    fn glob_lookahead_includes_the_scalar_prefix() {
        let mut scanner = Scanner::default();
        let mut lexer = TestLexer::new("<$sner > + 1");
        assert!(scan(
            &mut scanner,
            &mut lexer,
            &[OpenReadlineBracket, OpenFileglobBracket]
        ));
        assert_eq!(lexer.result, OpenFileglobBracket as u16);
        assert_eq!(lexer.token_end(), 1);
        assert_eq!(lexer.pos, lexer.input.len());
        assert_eq!(scanner.quotes[0].open, b'<' as i32);
        let mut lexer = TestLexer::new("< $a/$b > $c");
        assert!(!scan(
            &mut scanner,
            &mut lexer,
            &[OpenReadlineBracket, OpenFileglobBracket]
        ));
    }

    #[test]
    fn heredoc_end_rearms_next_body_and_empty_delimiter_is_valid() {
        let mut scanner = Scanner::default();
        let mut lexer = TestLexer::new("''");
        assert!(scan(&mut scanner, &mut lexer, &[HeredocDelim]));
        assert_eq!(scanner.heredocs[0].delim.length, 0);
        scanner.add_heredoc(string("END"), true, false);
        let mut lexer = TestLexer::new("\n");
        assert!(scan(&mut scanner, &mut lexer, &[HeredocStart]));
        assert_eq!(scanner.heredoc_state, HeredocState::Unknown);
        let mut lexer = TestLexer::new("\n");
        assert!(scan(&mut scanner, &mut lexer, &[HeredocMiddle]));
        assert_eq!(lexer.result, HeredocMiddle as u16);
        assert_eq!(lexer.token_end(), 0);
        let mut lexer = TestLexer::new("\n");
        assert!(scan(&mut scanner, &mut lexer, &[HeredocMiddle]));
        assert_eq!(lexer.result, HeredocEnd as u16);
        assert_eq!(lexer.token_end(), 0);
        assert_eq!(scanner.heredoc_count, 1);
        assert_eq!(scanner.heredoc_state, HeredocState::Start);
        assert!(scanner.heredocs[0].delim.same(&string("END")));
    }

    #[test]
    fn filetest_at_eof_matches_the_custom_strchr_nul() {
        let mut scanner = Scanner::default();
        let mut lexer = TestLexer::new("-");
        assert!(scan(&mut scanner, &mut lexer, &[Filetest]));
        assert_eq!(lexer.result, Filetest as u16);
        assert_eq!(
            lexer.events,
            [Event::Advance(0, false), Event::Advance(1, false)]
        );
    }
}
