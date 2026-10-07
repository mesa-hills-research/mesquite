//! Haskell external scanner: layout, newline lookahead, operators and extras.
//!
//! The order of the scanner's phases and lexer calls follows scanner.c. Transient
//! lookahead is kept in the scanner for allocation reuse, but is not serialized.

use ts_port_tables::{ExternalScanner, Lexer};
#[path = "scanner_unicode.rs"]
mod unicode;
use ContextSort::*;
use unicode::*;

// Like C's SEQ: each enum's first (default) variant means "continue".
macro_rules! step {
    ($expr:expr) => {{
        let result = $expr;
        if result != Default::default() {
            return result;
        }
    }};
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
enum Symbol {
    #[default]
    Fail,
    Semicolon,
    Start,
    StartDo,
    StartCase,
    StartIf,
    StartLet,
    StartQuote,
    StartExplicit,
    End,
    EndExplicit,
    StartBrace,
    EndBrace,
    StartTexp,
    EndTexp,
    Where,
    In,
    Arrow,
    Bar,
    Deriving,
    Comment,
    Haddock,
    Cpp,
    Pragma,
    QqStart,
    QqBody,
    Splice,
    QualDot,
    TightDot,
    PrefixDot,
    Dotdot,
    TightAt,
    PrefixAt,
    TightBang,
    PrefixBang,
    TightTilde,
    PrefixTilde,
    PrefixPercent,
    QualifiedOp,
    LeftSectionOp,
    NoSectionOp,
    Minus,
    Context,
    Infix,
    DataInfix,
    TypeInstance,
    Varsym,
    Consym,
    Update,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
enum ContextSort {
    #[default]
    DeclLayout,
    DoLayout,
    CaseLayout,
    LetLayout,
    QuoteLayout,
    MultiWayIfLayout,
    Braces,
    TExp,
    ModuleHeader,
    NoContext,
}
impl ContextSort {
    fn from_u32(value: u32) -> Self {
        match value {
            0 => DeclLayout,
            1 => DoLayout,
            2 => CaseLayout,
            3 => LetLayout,
            4 => QuoteLayout,
            5 => MultiWayIfLayout,
            6 => Braces,
            7 => TExp,
            8 => ModuleHeader,
            9 => NoContext,
            _ => Self::default(),
        }
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
enum Lexed {
    #[default]
    Nothing,
    Eof,
    Where,
    In,
    Then,
    Else,
    Deriving,
    Module,
    Upper,
    Tick,
    Symop,
    SymopSpecial,
    DotDot,
    DotId,
    DotSymop,
    DotOpen,
    Dollar,
    Bang,
    Tilde,
    At,
    Percent,
    Hash,
    Bar,
    Arrow,
    CArrow,
    TexpCloser,
    QuoteClose,
    Pragma,
    BlockComment,
    LineComment,
    BraceClose,
    BraceOpen,
    BracketOpen,
    UnboxedClose,
    Semi,
    CppElse,
    Cpp,
}
impl Lexed {
    fn from_u32(value: u32) -> Self {
        match value {
            0 => Lexed::Nothing,
            1 => Lexed::Eof,
            2 => Lexed::Where,
            3 => Lexed::In,
            4 => Lexed::Then,
            5 => Lexed::Else,
            6 => Lexed::Deriving,
            7 => Lexed::Module,
            8 => Lexed::Upper,
            9 => Lexed::Tick,
            10 => Lexed::Symop,
            11 => Lexed::SymopSpecial,
            12 => Lexed::DotDot,
            13 => Lexed::DotId,
            14 => Lexed::DotSymop,
            15 => Lexed::DotOpen,
            16 => Lexed::Dollar,
            17 => Lexed::Bang,
            18 => Lexed::Tilde,
            19 => Lexed::At,
            20 => Lexed::Percent,
            21 => Lexed::Hash,
            22 => Lexed::Bar,
            23 => Lexed::Arrow,
            24 => Lexed::CArrow,
            25 => Lexed::TexpCloser,
            26 => Lexed::QuoteClose,
            27 => Lexed::Pragma,
            28 => Lexed::BlockComment,
            29 => Lexed::LineComment,
            30 => Lexed::BraceClose,
            31 => Lexed::BraceOpen,
            32 => Lexed::BracketOpen,
            33 => Lexed::UnboxedClose,
            34 => Lexed::Semi,
            35 => Lexed::CppElse,
            36 => Lexed::Cpp,
            _ => Self::default(),
        }
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
enum NewlineState {
    #[default]
    Inactive,
    Init,
    Process,
    Resume,
}
impl NewlineState {
    fn from_u32(value: u32) -> Self {
        match value {
            0 => NewlineState::Inactive,
            1 => NewlineState::Init,
            2 => NewlineState::Process,
            3 => NewlineState::Resume,
            _ => Self::default(),
        }
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
enum Space {
    #[default]
    None,
    Indented,
    Bol,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
enum QualifiedName {
    #[default]
    None,
    Target,
    Conid,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
enum CppDirective {
    #[default]
    Nothing,
    Start,
    Else,
    End,
    Other,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
enum CtrResult {
    #[default]
    Undecided,
    Impossible,
    ArrowFound,
    InfixFound,
    EqualsFound,
    BarFound,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct Context {
    sort: ContextSort,
    indent: u32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct Newline {
    state: NewlineState,
    end: Lexed,
    indent: u32,
    eof: bool,
    no_semi: bool,
    skip_semi: bool,
    unsafe_lookahead: bool,
}

#[derive(Default)]
struct Lookahead {
    contents: Vec<i32>,
    offset: u32,
}

pub(crate) struct Scanner {
    contexts: Vec<Context>,
    newline: Newline,
    lookahead: Lookahead,
}

struct Env<'a> {
    lexer: &'a mut dyn Lexer,
    symbols: &'a [bool],
    symop: u32,
    state: &'a mut Scanner,
}

#[derive(Clone, Copy)]
struct StartLayout {
    sym: Symbol,
    sort: ContextSort,
}

#[derive(Default)]
struct CtrState {
    reset: u32,
    brackets: u32,
    context: bool,
    infix: bool,
    data_infix: bool,
    type_instance: bool,
}

fn is_newline(c: i32) -> bool {
    matches!(c, 10 | 13 | 12)
}
fn varid_start_char(c: i32) -> bool {
    c == '_' as i32 || is_varid_start_char(c)
}
fn is_id_char(c: i32) -> bool {
    c == '_' as i32 || c == '\'' as i32 || is_identifier_char(c)
}
fn is_inner_id_char(c: i32) -> bool {
    is_id_char(c) || c == '#' as i32
}
fn quoter_char(c: i32) -> bool {
    is_id_char(c) || c == '.' as i32
}
fn reserved_symbolic(c: i32) -> bool {
    matches!(
        c,
        40 | 41 | 44 | 59 | 91 | 93 | 96 | 123 | 125 | 34 | 39 | 95
    )
}
fn symop_char(c: i32) -> bool {
    is_symop_char(c) && !reserved_symbolic(c)
}
fn is_space_or_tab(c: i32) -> bool {
    c == ' ' as i32 || c == '\t' as i32
}
fn token_end(c: i32) -> bool {
    !is_inner_id_char(c)
}
fn context_end_sym(sort: ContextSort) -> Symbol {
    match sort {
        TExp => Symbol::EndTexp,
        Braces => Symbol::EndBrace,
        _ if sort < Braces => Symbol::End,
        _ => Symbol::Fail,
    }
}
fn layout_sort(s: Symbol) -> ContextSort {
    match s {
        Symbol::StartDo => DoLayout,
        Symbol::StartCase => CaseLayout,
        Symbol::StartIf => MultiWayIfLayout,
        Symbol::StartLet => LetLayout,
        Symbol::StartQuote => QuoteLayout,
        _ => DeclLayout,
    }
}
fn valid_symop_two_chars(first: i32, second: i32) -> bool {
    match first {
        // '='
        61 => second != '>' as i32,
        // '<'
        60 => second != '-' as i32,
        // ':'
        58 => second != ':' as i32,
        _ => true,
    }
}
fn lex_splice(c: i32) -> Lexed {
    if varid_start_char(c) || c == '(' as i32 {
        Lexed::Dollar
    } else {
        Lexed::Symop
    }
}

impl Env<'_> {
    fn reset_newline(&mut self) {
        self.state.newline = Newline::default();
    }
    fn newline_active(&self) -> bool {
        matches!(
            self.state.newline.state,
            NewlineState::Init | NewlineState::Process
        )
    }
    fn newline_init(&self) -> bool {
        self.state.newline.state == NewlineState::Init
    }
    fn is_eof(&self) -> bool {
        self.lexer.eof()
    }
    fn not_eof(&self) -> bool {
        !self.is_eof()
    }
    fn column(&mut self) -> u32 {
        if self.is_eof() {
            0
        } else {
            self.lexer.get_column()
        }
    }
    fn advance(&mut self) {
        if self.not_eof() {
            self.state.lookahead.contents.push(self.lexer.lookahead());
            self.lexer.advance(false);
        }
    }
    fn valid(&self, s: Symbol) -> bool {
        self.symbols[s as usize]
    }
    fn finish_if_valid(&self, s: Symbol) -> Symbol {
        if self.valid(s) { s } else { Symbol::Fail }
    }
    fn finish_marked(&mut self, s: Symbol) -> Symbol {
        self.lexer.mark_end();
        s
    }
    fn lookahead_size(&self) -> u32 {
        self.state.lookahead.contents.len() as u32
    }
    fn advance_over_abs(&mut self, abs: u32) {
        // Use a bounded range even at EOF, where advance does not append.
        for _ in self.lookahead_size()..=abs {
            self.advance();
        }
    }
    fn advance_over(&mut self, rel: u32) {
        self.advance_over_abs(self.state.lookahead.offset.wrapping_add(rel));
    }
    fn skip_over(&mut self, rel: u32) {
        if self.state.lookahead.offset > self.lookahead_size() {
            self.advance_over_abs(self.state.lookahead.offset - 1);
        }
        let abs = self.state.lookahead.offset.wrapping_add(rel);
        for _ in self.lookahead_size()..=abs {
            self.lexer.advance(true);
        }
    }
    fn advance_before(&mut self, rel: u32) {
        let abs = self.state.lookahead.offset.wrapping_add(rel);
        if abs > 0 {
            self.advance_over_abs(abs - 1);
        }
    }
    // C's "unsafe_peek": it never advances; missing lookahead is simply zero.
    fn cached_peek(&self, rel: u32) -> i32 {
        self.state
            .lookahead
            .contents
            .get(self.state.lookahead.offset.wrapping_add(rel) as usize)
            .copied()
            .unwrap_or(0)
    }
    fn peek(&mut self, rel: u32) -> i32 {
        if self.state.lookahead.offset.wrapping_add(rel) < self.lookahead_size() {
            self.cached_peek(rel)
        } else {
            self.advance_before(rel);
            self.lexer.lookahead()
        }
    }
    fn peek0(&mut self) -> i32 {
        self.peek(0)
    }
    fn peek1(&mut self) -> i32 {
        self.peek(1)
    }
    fn peek2(&mut self) -> i32 {
        self.peek(2)
    }
    fn char_at(&mut self, n: u32, c: char) -> bool {
        self.peek(n) == c as i32
    }
    fn char0(&mut self, c: char) -> bool {
        self.char_at(0, c)
    }
    fn char1(&mut self, c: char) -> bool {
        self.char_at(1, c)
    }
    fn char2(&mut self, c: char) -> bool {
        self.char_at(2, c)
    }
    fn reset_lookahead_abs(&mut self, abs: u32) {
        self.state.lookahead.offset = abs;
        self.symop = 0;
    }
    fn reset_lookahead_to(&mut self, rel: u32) {
        self.reset_lookahead_abs(self.state.lookahead.offset.wrapping_add(rel));
    }
    fn reset_lookahead(&mut self) {
        self.reset_lookahead_abs(self.lookahead_size());
    }
    fn no_lookahead(&self) -> bool {
        self.state.lookahead.contents.is_empty()
    }
    fn start_column(&mut self) -> u32 {
        self.column().wrapping_sub(self.lookahead_size())
    }
    fn advance_while(&mut self, mut i: u32, pred: fn(i32) -> bool) -> u32 {
        while pred(self.peek(i)) {
            i = i.wrapping_add(1);
        }
        i
    }
    fn advance_until_char(&mut self, mut i: u32, c: char) -> u32 {
        while self.not_eof() && !self.char_at(i, c) {
            i = i.wrapping_add(1);
        }
        i
    }
    fn has_contexts(&self) -> bool {
        !self.state.contexts.is_empty()
    }
    fn push_context(&mut self, sort: ContextSort, indent: u32) {
        self.state.contexts.push(Context { sort, indent });
    }
    fn pop(&mut self) {
        self.state.contexts.pop();
    }
    fn current_context(&self) -> ContextSort {
        self.state.contexts.last().map_or(NoContext, |c| c.sort)
    }
    fn is_layout_context(&self) -> bool {
        self.current_context() < Braces
    }
    fn is_semicolon_context(&self) -> bool {
        self.current_context() < MultiWayIfLayout
    }
    fn current_indent(&self) -> u32 {
        self.state
            .contexts
            .iter()
            .rev()
            .find(|c| c.sort < Braces)
            .map_or(0, |c| c.indent)
    }
    fn indent_less(&self, indent: u32) -> bool {
        self.is_layout_context() && indent < self.current_indent()
    }
    fn indent_lesseq(&self, indent: u32) -> bool {
        self.is_layout_context() && indent <= self.current_indent()
    }
    fn top_layout(&self) -> bool {
        self.state.contexts.len() == 1
    }
    fn in_module_header(&self) -> bool {
        self.current_context() == ModuleHeader
    }
    fn symop_lookahead(&mut self) -> u32 {
        if self.symop == 0 {
            self.symop = self.advance_while(0, symop_char);
        }
        self.symop
    }
    fn is_symop(&mut self) -> bool {
        self.symop_lookahead() > 0
    }
    fn seq_from(&mut self, s: &str, start: u32) -> bool {
        for (i, c) in s.bytes().enumerate() {
            if i32::from(c) != self.peek(start.wrapping_add(i as u32)) {
                return false;
            }
        }
        self.peek(start.wrapping_add(s.len() as u32));
        true
    }
    fn seq(&mut self, s: &str) -> bool {
        self.seq_from(s, 0)
    }
    fn take_line(&mut self) {
        while self.not_eof() && !is_newline(self.lexer.lookahead()) {
            self.advance();
        }
    }
    fn take_line_escaped_newline(&mut self) {
        loop {
            while self.not_eof()
                && !is_newline(self.lexer.lookahead())
                && self.lexer.lookahead() != '\\' as i32
            {
                self.advance();
            }
            if self.lexer.lookahead() == '\\' as i32 {
                self.advance();
                if is_space_or_tab(self.lexer.lookahead()) {
                    while is_space_or_tab(self.lexer.lookahead()) {
                        self.advance();
                    }
                    if is_newline(self.lexer.lookahead()) {
                        self.advance();
                    }
                } else {
                    self.advance();
                }
            } else {
                return;
            }
        }
    }
    // The grammar's Unicode space bitmap intentionally excludes ASCII tabs.
    // Tabs are handled separately by newline lookahead, at a fixed width of 8.
    fn skip_space(&mut self) -> bool {
        if !is_space_char(self.lexer.lookahead()) {
            return false;
        }
        self.lexer.advance(true);
        while is_space_char(self.lexer.lookahead()) {
            self.lexer.advance(true);
        }
        true
    }
    fn skip_newlines(&mut self) -> bool {
        if !is_newline(self.lexer.lookahead()) {
            return false;
        }
        self.lexer.advance(true);
        while is_newline(self.lexer.lookahead()) {
            self.lexer.advance(true);
        }
        true
    }
    fn skip_whitespace(&mut self) -> Space {
        let mut space = Space::None;
        loop {
            if self.skip_space() {
                space = Space::Indented;
            } else if self.skip_newlines() {
                space = Space::Bol;
            } else {
                return space;
            }
        }
    }
    fn take_space_from(&mut self, start: u32) -> u32 {
        self.advance_while(start, is_space_char)
    }
    fn token_from(&mut self, s: &str, start: u32) -> bool {
        self.seq_from(s, start) && token_end(self.peek(start.wrapping_add(s.len() as u32)))
    }
    fn token(&mut self, s: &str) -> bool {
        self.seq(s) && token_end(self.peek(s.len() as u32))
    }
    fn any_token_from(&mut self, tokens: &[&str], start: u32) -> bool {
        tokens.iter().any(|t| self.token_from(t, start))
    }
    fn match_symop(&mut self, target: &str) -> bool {
        self.symop_lookahead() == target.len() as u32 && self.seq(target)
    }
    fn uninitialized(&self) -> bool {
        !self.has_contexts()
    }
    fn conid(&mut self) -> u32 {
        if !is_conid_start_char(self.peek0()) {
            0
        } else {
            self.advance_while(1, is_inner_id_char)
        }
    }
    fn qualified_name(&mut self) -> QualifiedName {
        // The sole caller supplies is_symop as C's name predicate.
        let mut qualified = false;
        loop {
            let end = self.conid();
            if end == 0 {
                break;
            }
            if !self.char_at(end, '.') {
                if qualified {
                    return QualifiedName::Conid;
                } else {
                    break;
                }
            }
            qualified = true;
            self.reset_lookahead_to(end.wrapping_add(1));
            if self.is_symop() {
                return QualifiedName::Target;
            }
        }
        QualifiedName::None
    }
    fn odd_backslashes_before(&mut self, mut index: i32) -> bool {
        let mut odd = false;
        while index >= 0 && self.peek(index as u32) == '\\' as i32 {
            odd = !odd;
            index -= 1;
        }
        odd
    }
    fn take_string_literal(&mut self) -> u32 {
        let mut end = 1;
        loop {
            end = self.advance_until_char(end, '"').wrapping_add(1);
            if self.is_eof() || !self.odd_backslashes_before((end as i32).wrapping_sub(2)) {
                return end;
            }
        }
    }
    fn take_char_literal(&mut self) -> u32 {
        if self.char1('\\') {
            self.advance_until_char(2, '\'').wrapping_add(2)
        } else if self.char_at(2, '\'') {
            3
        } else {
            1
        }
    }
    fn cpp_directive_other(&mut self, start: u32) -> bool {
        self.any_token_from(
            &[
                "define", "undef", "include", "pragma", "error", "warning", "line",
            ],
            start,
        ) || is_newline(self.peek(start))
            || (self.char1('!') && self.uninitialized())
    }
    fn cpp_directive(&mut self) -> CppDirective {
        if !self.char0('#') {
            return CppDirective::Nothing;
        }
        let start = self.take_space_from(1);
        if self.any_token_from(&["if", "ifdef", "ifndef"], start) {
            CppDirective::Start
        } else if self.any_token_from(&["else", "elif", "elifdef", "elifndef"], start) {
            CppDirective::Else
        } else if self.token_from("endif", start) {
            CppDirective::End
        } else if self.cpp_directive_other(start) {
            CppDirective::Other
        } else {
            CppDirective::Nothing
        }
    }
    fn start_brace(&mut self) -> Symbol {
        if self.valid(Symbol::StartBrace) {
            self.push_context(Braces, 0);
            Symbol::StartBrace
        } else {
            Symbol::Fail
        }
    }
    fn end_brace(&mut self) -> Symbol {
        if self.valid(Symbol::EndBrace) && self.current_context() == Braces {
            self.pop();
            Symbol::EndBrace
        } else {
            Symbol::Fail
        }
    }
    fn valid_layout_start_sym(&self) -> Symbol {
        [
            Symbol::Start,
            Symbol::StartDo,
            Symbol::StartCase,
            Symbol::StartIf,
            Symbol::StartLet,
            Symbol::StartQuote,
            Symbol::StartExplicit,
        ]
        .into_iter()
        .find(|&s| self.valid(s))
        .unwrap_or(Symbol::Fail)
    }
    fn valid_layout_start(&self, next: Lexed) -> StartLayout {
        let mut start = StartLayout {
            sym: self.valid_layout_start_sym(),
            sort: NoContext,
        };
        if self.uninitialized() || start.sym == Symbol::Fail {
            return start;
        }
        let mut sort = layout_sort(start.sym);
        match next {
            Lexed::Bar => {}
            Lexed::BraceOpen => {
                if self.newline_active() {
                    return start;
                }
                sort = Braces;
                start.sym = Symbol::StartExplicit;
            }
            _ => {
                if sort == MultiWayIfLayout {
                    return start;
                }
            }
        }
        start.sort = sort;
        start
    }
    fn indent_can_start_layout(&self, sort: ContextSort, indent: u32) -> bool {
        if self.current_context() == Braces {
            return true;
        }
        let cur = self.current_indent();
        indent > cur || (indent == cur && sort == DoLayout)
    }
    fn start_layout(&mut self, start: StartLayout, indent: u32) -> Symbol {
        if self.in_module_header() {
            self.pop();
        } else if start.sort == Braces {
            self.lexer.mark_end();
        } else if !self.indent_can_start_layout(start.sort, indent) {
            return Symbol::Fail;
        }
        self.push_context(start.sort, indent);
        start.sym
    }
    fn start_layout_interior(&mut self, next: Lexed) -> Symbol {
        let start = self.valid_layout_start(next);
        if start.sort == NoContext {
            return Symbol::Fail;
        }
        let col = self.start_column();
        self.start_layout(start, col)
    }
    fn start_layout_newline(&mut self) -> Symbol {
        let start = self.valid_layout_start(self.state.newline.end);
        if start.sort == NoContext {
            return Symbol::Fail;
        }
        let result = self.start_layout(start, self.state.newline.indent);
        if result != Symbol::Fail {
            self.state.newline.no_semi = true;
        }
        result
    }
    fn texp_context(&mut self) -> Symbol {
        if self.valid(Symbol::StartTexp) {
            self.push_context(TExp, 0);
            Symbol::StartTexp
        } else if self.valid(Symbol::EndTexp) && self.current_context() == TExp {
            self.pop();
            Symbol::EndTexp
        } else {
            Symbol::Fail
        }
    }
    fn end_layout_unchecked(&mut self) -> Symbol {
        self.pop();
        Symbol::End
    }
    fn end_layout(&mut self) -> Symbol {
        if self.valid(Symbol::End) {
            self.end_layout_unchecked()
        } else {
            Symbol::Fail
        }
    }
    fn end_layout_brace(&mut self) -> Symbol {
        if self.valid(Symbol::EndExplicit) && self.current_context() == Braces {
            self.advance_over(0);
            self.lexer.mark_end();
            self.pop();
            Symbol::EndExplicit
        } else {
            Symbol::Fail
        }
    }
    fn end_layout_indent(&mut self) -> Symbol {
        if self.valid(Symbol::End) && self.indent_less(self.state.newline.indent) {
            if self.top_layout() {
                self.state.contexts.last_mut().unwrap().indent = self.state.newline.indent;
                return Symbol::Update;
            } else {
                self.state.newline.skip_semi = false;
                return self.end_layout_unchecked();
            }
        }
        Symbol::Fail
    }
    fn end_layout_infix(&mut self) -> Symbol {
        if !self.valid(Symbol::Varsym) && !self.valid(Symbol::Consym) {
            self.end_layout()
        } else {
            Symbol::Fail
        }
    }
    fn end_layout_where(&mut self) -> Symbol {
        if self.valid(Symbol::End) && !self.valid(Symbol::Where) && self.is_layout_context() {
            self.end_layout()
        } else {
            Symbol::Fail
        }
    }
    fn end_layout_in(&mut self) -> Symbol {
        if self.valid(Symbol::End)
            && (!self.valid(Symbol::In) || self.current_context() == LetLayout)
        {
            self.end_layout()
        } else {
            Symbol::Fail
        }
    }
    fn end_layout_deriving(&mut self) -> Symbol {
        if self.valid(Symbol::End)
            && !self.valid(Symbol::Deriving)
            && !self.top_layout()
            && self.current_context() == DeclLayout
        {
            self.end_layout()
        } else {
            Symbol::Fail
        }
    }
    fn layouts_in_texp(&self) -> bool {
        if self.is_layout_context() && self.state.contexts.len() > 1 {
            for cur in self.state.contexts[..self.state.contexts.len() - 1]
                .iter()
                .rev()
            {
                if cur.sort == TExp || cur.sort == Braces {
                    return true;
                } else if cur.sort > Braces {
                    break;
                }
            }
        }
        false
    }
    fn token_end_layout_texp(&mut self) -> Symbol {
        if self.valid(Symbol::End) && self.layouts_in_texp() {
            self.end_layout()
        } else {
            Symbol::Fail
        }
    }
    fn force_end_context(&mut self) -> Symbol {
        while let Some(ctx) = self.state.contexts.last() {
            let s = context_end_sym(ctx.sort);
            self.pop();
            if s != Symbol::Fail && self.valid(s) {
                return s;
            }
        }
        Symbol::Fail
    }
    fn opening_token(&mut self, i: u32) -> bool {
        let c = self.peek(i);
        match c {
            0x27e6 | 0x2987 | 40 | 91 | 34 => true,
            // '{'
            123 => self.peek(i.wrapping_add(1)) != '-' as i32,
            _ => is_id_char(c),
        }
    }
    fn lex_prefix(&mut self, token: Lexed) -> Lexed {
        if self.opening_token(1) {
            token
        } else {
            Lexed::Symop
        }
    }
    fn lex_symop(&mut self) -> Lexed {
        let len = self.symop_lookahead();
        if len == 0 {
            return Lexed::Nothing;
        }
        let c1 = self.cached_peek(0);
        if len == 1 {
            match c1 {
                // '?'
                63 => {
                    return if varid_start_char(self.peek1()) {
                        Lexed::Nothing
                    } else {
                        Lexed::Symop
                    };
                }
                // '#'
                35 => {
                    return if self.char1(')') {
                        Lexed::UnboxedClose
                    } else {
                        Lexed::Hash
                    };
                }
                // '|'
                124 => {
                    return if self.char1(']') {
                        Lexed::QuoteClose
                    } else {
                        Lexed::Bar
                    };
                }
                // '!'
                33 => return self.lex_prefix(Lexed::Bang),
                // '~'
                126 => return self.lex_prefix(Lexed::Tilde),
                // '@'
                64 => return self.lex_prefix(Lexed::At),
                // '%'
                37 => return self.lex_prefix(Lexed::Percent),
                // '$'
                36 => return lex_splice(self.peek1()),
                // '.'
                46 => {
                    return if is_id_char(self.peek1()) {
                        Lexed::DotId
                    } else if self.opening_token(1) {
                        Lexed::DotOpen
                    } else {
                        Lexed::Symop
                    };
                }
                0x2192 | 0x22b8 => return Lexed::Arrow,
                0x21d2 => return Lexed::CArrow,
                // '='
                61 | 0x27e7 | 0x2988 => return Lexed::TexpCloser,
                // '*', '-'
                42 | 45 => return Lexed::SymopSpecial,
                92 | 0x2190 | 0x2200 | 0x2237 | 0x2605 | 0x27e6 | 0x2919 | 0x291a | 0x291b
                | 0x291c | 0x2987 => return Lexed::Nothing,
                _ => {}
            }
        } else if len == 2 {
            if self.seq("->") {
                return Lexed::Arrow;
            }
            if self.seq("=>") {
                return Lexed::CArrow;
            }
            let c2 = self.cached_peek(1);
            match c1 {
                // '$'
                36 => {
                    if c2 == '$' as i32 {
                        return lex_splice(self.peek2());
                    }
                }
                // '|'
                124 => {
                    if c2 == '|' as i32 && self.char2(']') {
                        return Lexed::QuoteClose;
                    }
                }
                // '.'
                46 => {
                    return if c2 == '.' as i32 {
                        Lexed::DotDot
                    } else {
                        Lexed::DotSymop
                    };
                }
                // '#'
                35 => {
                    if c2 == '#' as i32 || c2 == '|' as i32 {
                        return Lexed::SymopSpecial;
                    }
                }
                _ => {
                    if !valid_symop_two_chars(c1, c2) {
                        return Lexed::Nothing;
                    }
                }
            }
        } else {
            match c1 {
                // '-'
                45 => {
                    if self.seq("->.") {
                        return Lexed::Arrow;
                    }
                }
                // '.'
                46 => return Lexed::DotSymop,
                _ => {}
            }
        }
        Lexed::Symop
    }
    fn left_section_op(&mut self, start: u32) -> Symbol {
        if self.valid(Symbol::LeftSectionOp) {
            self.advance_before(start);
            let space = self.skip_whitespace();
            if self.char_at(start, ')') {
                return Symbol::LeftSectionOp;
            }
            if space != Space::None {
                return self.finish_if_valid(Symbol::NoSectionOp);
            }
        }
        Symbol::Fail
    }
    fn left_section_ticked(&mut self) -> Symbol {
        if self.valid(Symbol::LeftSectionOp) {
            let end_tick = self.advance_until_char(1, '`');
            if self.char_at(end_tick, '`') {
                return self.left_section_op(end_tick.wrapping_add(1));
            }
        }
        Symbol::Fail
    }
    fn finish_symop(&mut self, s: Symbol) -> Symbol {
        if self.valid(s) || self.valid(Symbol::LeftSectionOp) {
            let after_symop = self.symop_lookahead();
            step!(self.left_section_op(after_symop));
            self.lexer.mark_end();
            return s;
        }
        Symbol::Fail
    }
    fn tight_op(&self, whitespace: bool, s: Symbol) -> Symbol {
        if !whitespace {
            self.finish_if_valid(s)
        } else {
            Symbol::Fail
        }
    }
    fn prefix_or_varsym(&mut self, whitespace: bool, s: Symbol) -> Symbol {
        if whitespace {
            step!(self.finish_if_valid(s));
        }
        self.finish_symop(Symbol::Varsym)
    }
    fn tight_or_varsym(&mut self, whitespace: bool, s: Symbol) -> Symbol {
        step!(self.tight_op(whitespace, s));
        self.finish_symop(Symbol::Varsym)
    }
    fn infix_or_varsym(&mut self, whitespace: bool, prefix: Symbol, tight: Symbol) -> Symbol {
        step!(self.finish_if_valid(if whitespace { prefix } else { tight }));
        self.finish_symop(Symbol::Varsym)
    }
    fn qualified_op(&mut self) -> Symbol {
        if self.qualified_name() == QualifiedName::Target {
            let after = self.symop_lookahead();
            step!(self.left_section_op(after));
            return Symbol::QualifiedOp;
        }
        Symbol::Fail
    }
    fn is_qq_start(&mut self) -> bool {
        let end = self.advance_while(1, quoter_char);
        self.char_at(end, '|')
    }
    fn try_end_token(&mut self, target: &str, matched: Lexed) -> Lexed {
        if self.token(target) {
            matched
        } else {
            Lexed::Nothing
        }
    }
    fn only_minus(&mut self) -> bool {
        let mut i: u32 = 2;
        while self.peek(i) == '-' as i32 {
            i = i.wrapping_add(1);
        }
        !symop_char(self.peek(i))
    }
    fn line_comment_herald(&mut self) -> bool {
        self.seq("--") && self.only_minus()
    }
    fn lex_cpp(&mut self) -> Lexed {
        match self.cpp_directive() {
            CppDirective::Else => Lexed::CppElse,
            CppDirective::Nothing => Lexed::Nothing,
            _ => Lexed::Cpp,
        }
    }
    fn lex_extras(&mut self, bol: bool) -> Lexed {
        match self.peek0() {
            // '{'
            123 => {
                if self.char1('-') {
                    return if self.char2('#') {
                        Lexed::Pragma
                    } else {
                        Lexed::BlockComment
                    };
                }
            }
            // '#'
            35 => {
                if bol {
                    return self.lex_cpp();
                }
            }
            // '-'
            45 if self.line_comment_herald() => return Lexed::LineComment,
            _ => {}
        }
        Lexed::Nothing
    }
    // As in C, EOF is not synthesized here (despite the Lexed::Eof variant).
    // scan's final process_result phase handles it after lookahead is exhausted.
    fn lex(&mut self, bol: bool) -> Lexed {
        step!(self.lex_extras(bol));
        if symop_char(self.peek0()) {
            step!(self.lex_symop());
        } else {
            match self.peek0() {
                // 'w'
                119 => return self.try_end_token("where", Lexed::Where),
                // 'i'
                105 => return self.try_end_token("in", Lexed::In),
                // 't'
                116 => return self.try_end_token("then", Lexed::Then),
                // 'e'
                101 => return self.try_end_token("else", Lexed::Else),
                // 'd'
                100 => return self.try_end_token("deriving", Lexed::Deriving),
                // 'm'
                109 => {
                    if (self.uninitialized() || self.in_module_header()) && self.token("module") {
                        return Lexed::Module;
                    }
                }
                // '{'
                123 => return Lexed::BraceOpen,
                // '}'
                125 => return Lexed::BraceClose,
                // ';'
                59 => return Lexed::Semi,
                // '`'
                96 => return Lexed::Tick,
                // '['
                91 => {
                    if self.valid(Symbol::QqStart) && self.is_qq_start() {
                        return Lexed::BracketOpen;
                    }
                }
                // ']', ')', ','
                93 | 41 | 44 => return Lexed::TexpCloser,
                _ => {
                    if is_conid_start_char(self.peek0()) {
                        return Lexed::Upper;
                    }
                }
            }
        }
        Lexed::Nothing
    }
    fn cpp_else(&mut self, emit: bool) -> Symbol {
        let mut nesting: u32 = 1;
        loop {
            self.take_line_escaped_newline();
            if emit {
                self.lexer.mark_end();
            }
            self.advance();
            self.reset_lookahead();
            match self.cpp_directive() {
                CppDirective::Start => nesting = nesting.wrapping_add(1),
                CppDirective::End => nesting = nesting.wrapping_sub(1),
                _ => {}
            }
            if !self.not_eof() || nesting == 0 {
                break;
            }
        }
        if emit { Symbol::Cpp } else { Symbol::Fail }
    }
    fn cpp_line(&mut self) -> Symbol {
        self.take_line_escaped_newline();
        self.finish_marked(Symbol::Cpp)
    }
    fn comment_type(&mut self) -> Symbol {
        let mut i: u32 = 2;
        while self.peek(i) == '-' as i32 {
            i = i.wrapping_add(1);
        }
        while self.not_eof() {
            let c = self.peek(i);
            i = i.wrapping_add(1);
            if c == '|' as i32 || c == '^' as i32 {
                return Symbol::Haddock;
            } else if !is_space_char(c) {
                break;
            }
        }
        Symbol::Comment
    }
    fn inline_comment(&mut self) -> Symbol {
        let sym = self.comment_type();
        loop {
            self.take_line();
            self.lexer.mark_end();
            self.advance();
            self.reset_lookahead();
            if !self.line_comment_herald() {
                break;
            }
        }
        sym
    }
    fn consume_block_comment(&mut self, mut col: u32) -> u32 {
        let mut level: u32 = 0;
        loop {
            if self.is_eof() {
                return col;
            }
            col = col.wrapping_add(1);
            match self.lexer.lookahead() {
                // '{'
                123 => {
                    self.advance();
                    if self.lexer.lookahead() == '-' as i32 {
                        self.advance();
                        col = col.wrapping_add(1);
                        level = level.wrapping_add(1);
                    }
                }
                // '-'
                45 => {
                    self.advance();
                    if self.lexer.lookahead() == '}' as i32 {
                        self.advance();
                        col = col.wrapping_add(1);
                        if level == 0 {
                            return col;
                        }
                        level -= 1;
                    }
                }
                // '\n', '\r', '\x0c'
                10 | 13 | 12 => {
                    self.advance();
                    col = 0;
                }
                // '\t'
                9 => {
                    self.advance();
                    col = col.wrapping_add(7);
                }
                _ => self.advance(),
            }
        }
    }
    fn block_comment(&mut self) -> Symbol {
        let sym = self.comment_type();
        self.consume_block_comment(self.lookahead_size());
        self.finish_marked(sym)
    }
    fn consume_pragma(&mut self) -> bool {
        if self.seq("{-#") {
            while !self.seq("#-}") && self.not_eof() {
                self.reset_lookahead();
                self.advance_over(0);
            }
            true
        } else {
            false
        }
    }
    fn pragma(&mut self) -> Symbol {
        if self.consume_pragma() {
            self.lexer.mark_end();
            if self.state.newline.state != NewlineState::Inactive {
                self.state.newline.state = NewlineState::Resume;
            }
            Symbol::Pragma
        } else {
            Symbol::Fail
        }
    }
    fn qq_body(&mut self) -> Symbol {
        loop {
            if self.is_eof() {
                return Symbol::QqBody;
            } else if self.lexer.lookahead() == 0x27e7 {
                return self.finish_marked(Symbol::QqBody);
            } else if self.lexer.lookahead() == '|' as i32 {
                self.lexer.mark_end();
                self.advance();
                if self.lexer.lookahead() == ']' as i32 {
                    return Symbol::QqBody;
                }
            } else {
                self.advance();
            }
        }
    }
    fn explicit_semicolon(&mut self) -> Symbol {
        if self.valid(Symbol::Semicolon) && !self.state.newline.skip_semi {
            self.state.newline.skip_semi = true;
            Symbol::Update
        } else {
            Symbol::Fail
        }
    }
    fn resolve_semicolon(&mut self, next: Lexed) -> Symbol {
        if self.state.newline.skip_semi {
            match next {
                Lexed::LineComment | Lexed::BlockComment | Lexed::Pragma | Lexed::Semi => {}
                _ => {
                    self.state.newline.skip_semi = false;
                    return Symbol::Update;
                }
            }
        }
        Symbol::Fail
    }
    fn semicolon(&mut self) -> Symbol {
        if self.is_semicolon_context()
            && !(self.state.newline.no_semi || self.state.newline.skip_semi)
            && self.indent_lesseq(self.state.newline.indent)
        {
            self.state.newline.no_semi = true;
            Symbol::Semicolon
        } else {
            Symbol::Fail
        }
    }
    fn process_token_safe(&mut self, next: Lexed) -> Symbol {
        match next {
            Lexed::Where => self.end_layout_where(),
            Lexed::In => self.end_layout_in(),
            Lexed::Then | Lexed::Else => self.end_layout(),
            Lexed::Deriving => self.end_layout_deriving(),
            Lexed::Bar => {
                if !self.valid(Symbol::Bar) {
                    self.end_layout()
                } else {
                    Symbol::Fail
                }
            }
            Lexed::Pragma => self.pragma(),
            Lexed::BlockComment => self.block_comment(),
            Lexed::LineComment => self.inline_comment(),
            Lexed::CppElse => self.cpp_else(true),
            Lexed::Cpp => self.cpp_line(),
            Lexed::Symop | Lexed::Tick | Lexed::Hash => self.end_layout_infix(),
            Lexed::UnboxedClose => {
                step!(self.token_end_layout_texp());
                self.end_layout_infix()
            }
            Lexed::Arrow => {
                if !self.valid(Symbol::Arrow) {
                    self.token_end_layout_texp()
                } else {
                    Symbol::Fail
                }
            }
            Lexed::TexpCloser => self.token_end_layout_texp(),
            Lexed::QuoteClose => self.end_layout(),
            _ => Symbol::Fail,
        }
    }
    fn process_token_symop(&mut self, whitespace: bool, next: Lexed) -> Symbol {
        match next {
            Lexed::DotDot => {
                step!(self.finish_if_valid(Symbol::Dotdot));
                self.tight_op(whitespace, Symbol::QualDot)
            }
            Lexed::DotId => {
                step!(self.finish_if_valid(if whitespace {
                    Symbol::PrefixDot
                } else {
                    Symbol::TightDot
                }));
                self.tight_op(whitespace, Symbol::QualDot)
            }
            Lexed::DotSymop => self.tight_or_varsym(whitespace, Symbol::QualDot),
            Lexed::DotOpen => self.prefix_or_varsym(whitespace, Symbol::PrefixDot),
            Lexed::Bang => self.infix_or_varsym(whitespace, Symbol::PrefixBang, Symbol::TightBang),
            Lexed::Tilde => {
                self.infix_or_varsym(whitespace, Symbol::PrefixTilde, Symbol::TightTilde)
            }
            Lexed::At => self.infix_or_varsym(whitespace, Symbol::PrefixAt, Symbol::TightAt),
            Lexed::Percent => self.prefix_or_varsym(whitespace, Symbol::PrefixPercent),
            Lexed::Symop => {
                if self.char0(':') {
                    self.finish_symop(Symbol::Consym)
                } else {
                    self.finish_symop(Symbol::Varsym)
                }
            }
            Lexed::SymopSpecial => {
                let after = self.symop_lookahead();
                step!(self.left_section_op(after));
                if self.valid(Symbol::Minus) && self.match_symop("-") {
                    Symbol::Minus
                } else {
                    Symbol::Fail
                }
            }
            Lexed::UnboxedClose | Lexed::Hash => {
                let after = self.symop_lookahead();
                self.left_section_op(after)
            }
            Lexed::Tick => self.left_section_ticked(),
            Lexed::Upper => {
                if self.valid(Symbol::QualifiedOp) || self.valid(Symbol::LeftSectionOp) {
                    step!(self.qualified_op());
                }
                Symbol::Fail
            }
            _ => Symbol::Fail,
        }
    }
    fn process_token_splice(&self, next: Lexed) -> Symbol {
        if next == Lexed::Dollar {
            self.finish_if_valid(Symbol::Splice)
        } else {
            Symbol::Fail
        }
    }
    fn process_token_interior(&mut self, next: Lexed) -> Symbol {
        match next {
            Lexed::BraceClose => {
                step!(self.end_layout_brace());
                return self.token_end_layout_texp();
            }
            Lexed::Module => return Symbol::Fail,
            Lexed::Semi => return self.explicit_semicolon(),
            Lexed::BracketOpen => return Symbol::QqStart,
            _ => {}
        }
        step!(self.process_token_safe(next));
        self.start_layout_interior(next)
    }
    fn process_token_init(&mut self, indent: u32, next: Lexed) -> Symbol {
        match next {
            Lexed::Module => {
                self.push_context(ModuleHeader, 0);
                Symbol::Update
            }
            Lexed::BraceOpen => {
                self.advance_over(0);
                self.lexer.mark_end();
                self.push_context(Braces, indent);
                Symbol::StartExplicit
            }
            _ => {
                self.push_context(DeclLayout, indent);
                Symbol::Start
            }
        }
    }
    fn newline_extras(&mut self, space: Space) -> Symbol {
        let bol = space == Space::Bol || (space == Space::None && self.newline_init());
        let next = self.lex_extras(bol);
        self.process_token_safe(next)
    }
    fn newline_process(&mut self) -> Symbol {
        let indent = self.state.newline.indent;
        let end = self.state.newline.end;
        step!(self.end_layout_indent());
        step!(self.process_token_safe(end));
        let space = self.skip_whitespace();
        self.lexer.mark_end();
        if self.state.newline.unsafe_lookahead {
            step!(self.newline_extras(space));
        }
        if !self.state.newline.eof {
            step!(self.start_layout_newline());
        }
        step!(self.semicolon());
        self.reset_newline();
        if self.uninitialized() {
            step!(self.process_token_init(indent, end));
        } else {
            step!(self.process_token_symop(true, end));
            step!(self.process_token_splice(end));
        }
        Symbol::Update
    }
    fn newline_post(&mut self) -> Symbol {
        let result = self.newline_process();
        if self.newline_init() {
            self.state.newline.state = NewlineState::Process;
        }
        result
    }
    fn newline_lookahead(&mut self, newline: &mut Newline) {
        loop {
            match self.peek0() {
                // '\n', '\r', '\x0c'
                10 | 13 | 12 => {
                    self.skip_over(0);
                    newline.indent = 0;
                }
                // '\t'
                9 => {
                    self.skip_over(0);
                    newline.indent = newline.indent.wrapping_add(8);
                }
                _ => {
                    if is_space_char(self.peek0()) {
                        self.skip_over(0);
                        newline.indent = newline.indent.wrapping_add(1);
                    } else {
                        newline.end = self.lex(newline.indent == 0);
                        newline.unsafe_lookahead |= !self.no_lookahead();
                        match newline.end {
                            Lexed::Eof => {
                                newline.indent = 0;
                                newline.eof = true;
                                return;
                            }
                            Lexed::Then | Lexed::Else | Lexed::Semi => {
                                newline.no_semi = true;
                                return;
                            }
                            Lexed::BlockComment => {
                                newline.indent =
                                    self.consume_block_comment(newline.indent.wrapping_add(2));
                            }
                            Lexed::LineComment => {
                                newline.indent = 0;
                                self.take_line();
                            }
                            Lexed::CppElse => {
                                self.cpp_else(false);
                                self.take_line_escaped_newline();
                            }
                            Lexed::Cpp => self.take_line_escaped_newline(),
                            _ => return,
                        }
                    }
                }
            }
            self.reset_lookahead();
        }
    }
    fn newline_start(&mut self) -> Symbol {
        self.state.newline.state = NewlineState::Init;
        // Lookahead helpers do not inspect the persistent newline. Use a local
        // value so both it and the rest of Env can be mutated without aliasing.
        let mut newline = self.state.newline;
        self.newline_lookahead(&mut newline);
        self.state.newline = newline;
        if self.state.newline.unsafe_lookahead {
            Symbol::Update
        } else {
            self.newline_post()
        }
    }
    fn newline_resume(&mut self) -> Symbol {
        let indent = self.state.newline.indent;
        self.skip_space();
        self.reset_newline();
        self.state.newline.indent = indent;
        self.newline_start()
    }
    fn ctr_stop_on_token(&mut self, target: &str) -> CtrResult {
        if self.token(target) {
            CtrResult::Impossible
        } else {
            CtrResult::Undecided
        }
    }
    fn ctr_top(&mut self, next: Lexed) -> CtrResult {
        match next {
            Lexed::CArrow => return CtrResult::ArrowFound,
            Lexed::Symop | Lexed::SymopSpecial | Lexed::Tilde | Lexed::Tick => {
                return CtrResult::InfixFound;
            }
            Lexed::Bar => return CtrResult::BarFound,
            Lexed::Arrow | Lexed::Where | Lexed::DotDot | Lexed::Semi => {}
            Lexed::TexpCloser => {
                if self.peek0() == '=' as i32 {
                    return CtrResult::EqualsFound;
                }
            }
            _ => match self.peek0() {
                // '='
                61 => return CtrResult::EqualsFound,
                0x2200 => {}
                // ':'
                58 => {
                    if !self.char1(':') {
                        return CtrResult::Undecided;
                    }
                }
                // 'f'
                102 => {
                    step!(self.ctr_stop_on_token("forall"));
                    return self.ctr_stop_on_token("family");
                }
                // 'i'
                105 => return self.ctr_stop_on_token("instance"),
                _ => return CtrResult::Undecided,
            },
        }
        CtrResult::Impossible
    }
    fn ctr_lookahead_step(&mut self, state: &mut CtrState, next: Lexed) -> CtrResult {
        state.reset = 1;
        match next {
            Lexed::BraceClose => return state.bracket_close(),
            Lexed::UnboxedClose => {
                step!(state.bracket_close());
                state.reset = 2;
                return CtrResult::Undecided;
            }
            Lexed::BraceOpen => return state.bracket_open(),
            Lexed::SymopSpecial | Lexed::Symop => state.reset = self.symop_lookahead(),
            Lexed::Upper => {
                state.reset = self.conid();
                return CtrResult::Undecided;
            }
            Lexed::DotId => return CtrResult::Undecided,
            Lexed::Pragma => {
                if self.consume_pragma() {
                    state.reset = 3;
                }
                return CtrResult::Undecided;
            }
            Lexed::TexpCloser | Lexed::Nothing => match self.peek0() {
                // ')', ']'
                41 | 93 => return state.bracket_close(),
                // '(', '['
                40 | 91 => return state.bracket_open(),
                // '"'
                34 => {
                    state.reset = self.take_string_literal();
                    return CtrResult::Undecided;
                }
                // '\''
                39 => {
                    state.reset = self.take_char_literal();
                    return CtrResult::Undecided;
                }
                _ => {
                    if varid_start_char(self.peek0()) {
                        state.reset = self.advance_while(1, is_id_char);
                    }
                }
            },
            _ => {}
        }
        if state.brackets != 0 {
            CtrResult::Undecided
        } else {
            self.ctr_top(next)
        }
    }
    fn constraint_lookahead(&mut self) -> Symbol {
        let mut state = CtrState::default();
        let mut done = false;
        while !done && self.not_eof() {
            let mut newline = Newline {
                indent: 99999,
                ..Newline::default()
            };
            self.newline_lookahead(&mut newline);
            if newline.indent <= self.current_indent() && self.current_context() != Braces {
                break;
            }
            match self.ctr_lookahead_step(&mut state, newline.end) {
                CtrResult::ArrowFound => {
                    state.context = true;
                    done = true;
                }
                CtrResult::InfixFound => {
                    if self.char0(':') || self.char0('`') {
                        state.data_infix = true;
                    }
                    state.infix = true;
                    done = !self.valid(Symbol::Context);
                }
                CtrResult::EqualsFound => {
                    done = !self.valid(Symbol::TypeInstance);
                    state.type_instance = true;
                }
                CtrResult::BarFound => {
                    done = true;
                    state.type_instance = false;
                }
                CtrResult::Impossible => done = true,
                CtrResult::Undecided => {}
            }
            self.reset_lookahead_to(state.reset);
            state.reset = 0;
        }
        if state.context {
            step!(self.finish_if_valid(Symbol::Context));
        }
        if state.infix {
            step!(self.finish_if_valid(Symbol::Infix));
        }
        if state.data_infix {
            step!(self.finish_if_valid(Symbol::DataInfix));
        }
        if state.type_instance {
            step!(self.finish_if_valid(Symbol::TypeInstance));
        }
        Symbol::Fail
    }
    fn process_token_constraint(&mut self) -> Symbol {
        if self.valid(Symbol::Context)
            || self.valid(Symbol::Infix)
            || self.valid(Symbol::DataInfix)
            || self.valid(Symbol::TypeInstance)
        {
            self.constraint_lookahead()
        } else {
            Symbol::Fail
        }
    }
    fn interior(&mut self, whitespace: bool) -> Symbol {
        let next = self.lex(false);
        step!(self.resolve_semicolon(next));
        step!(self.process_token_interior(next));
        step!(self.process_token_symop(whitespace, next));
        step!(self.process_token_constraint());
        step!(self.process_token_splice(next));
        Symbol::Fail
    }
    fn pre_ws_commands(&mut self) -> Symbol {
        step!(self.texp_context());
        step!(self.start_brace());
        step!(self.end_brace());
        if self.valid(Symbol::QqBody) {
            return self.qq_body();
        }
        if self.newline_active() {
            step!(self.newline_post());
        } else if self.state.newline.state == NewlineState::Resume {
            step!(self.newline_resume());
        }
        Symbol::Fail
    }
    fn scan_main(&mut self) -> Symbol {
        self.lexer.mark_end();
        step!(self.pre_ws_commands());
        let whitespace = self.skip_space();
        if is_newline(self.lexer.lookahead()) {
            self.newline_start()
        } else if self.not_eof() {
            self.interior(whitespace)
        } else {
            Symbol::Fail
        }
    }
    fn process_result(&mut self, mut result: Symbol) -> bool {
        if result == Symbol::Fail && self.is_eof() && self.no_lookahead() {
            self.lexer.mark_end();
            result = if self.valid(Symbol::End) {
                self.end_layout_unchecked()
            } else if self.valid(Symbol::Semicolon) {
                Symbol::Semicolon
            } else {
                self.force_end_context()
            };
        }
        if result != Symbol::Fail {
            self.lexer.set_result_symbol(result as u16);
            true
        } else {
            false
        }
    }
    fn scan(&mut self) -> bool {
        if self.valid(Symbol::Fail) {
            return false;
        }
        let result = self.scan_main();
        self.process_result(result)
    }
}

impl CtrState {
    fn bracket_open(&mut self) -> CtrResult {
        self.brackets = self.brackets.wrapping_add(1);
        self.reset = 1;
        CtrResult::Undecided
    }
    fn bracket_close(&mut self) -> CtrResult {
        if self.brackets == 0 {
            return CtrResult::Impossible;
        }
        self.brackets -= 1;
        self.reset = 1;
        CtrResult::Undecided
    }
}

impl ExternalScanner for Scanner {
    fn scan(&mut self, lexer: &mut dyn Lexer, valid_symbols: &[bool]) -> bool {
        Env {
            lexer,
            symbols: valid_symbols,
            symop: 0,
            state: self,
        }
        .scan()
    }

    fn serialize(&mut self, buffer: &mut [u8]) -> usize {
        // C memcpy's Persist (u32 count, 16-byte Newline), then Context entries
        // (u32 enum, u32 indent). All words are native endian; bools are bytes.
        let to_copy = 20 + self.contexts.len() * 8;
        if buffer.len() < 20 {
            return 0;
        }
        buffer[..4].copy_from_slice(&(self.contexts.len() as u32).to_ne_bytes());
        buffer[4..8].copy_from_slice(&(self.newline.state as u32).to_ne_bytes());
        buffer[8..12].copy_from_slice(&(self.newline.end as u32).to_ne_bytes());
        buffer[12..16].copy_from_slice(&self.newline.indent.to_ne_bytes());
        buffer[16] = u8::from(self.newline.eof);
        buffer[17] = u8::from(self.newline.no_semi);
        buffer[18] = u8::from(self.newline.skip_semi);
        buffer[19] = u8::from(self.newline.unsafe_lookahead);
        // C writes the header even when the context stack is too large.
        if to_copy > buffer.len().min(1024) {
            return 0;
        }
        for (ctx, bytes) in self
            .contexts
            .iter()
            .zip(buffer[20..to_copy].as_chunks_mut::<8>().0.iter_mut())
        {
            bytes[..4].copy_from_slice(&(ctx.sort as u32).to_ne_bytes());
            bytes[4..].copy_from_slice(&ctx.indent.to_ne_bytes());
        }
        to_copy
    }

    fn deserialize(&mut self, buffer: &[u8]) {
        self.contexts.clear();
        self.newline = Newline {
            state: NewlineState::Resume,
            ..Newline::default()
        };
        // Valid snapshots have the full header and count * 8 trailing bytes.
        // Unlike C's unchecked memcpy, malformed/truncated input is bounded.
        if buffer.len() >= 20 {
            let word = |at| u32::from_ne_bytes(buffer[at..at + 4].try_into().unwrap());
            let count = word(0) as usize;
            self.newline = Newline {
                state: NewlineState::from_u32(word(4)),
                end: Lexed::from_u32(word(8)),
                indent: word(12),
                eof: buffer[16] != 0,
                no_semi: buffer[17] != 0,
                skip_semi: buffer[18] != 0,
                unsafe_lookahead: buffer[19] != 0,
            };
            for bytes in buffer[20..].as_chunks::<8>().0.iter().take(count) {
                self.contexts.push(Context {
                    sort: ContextSort::from_u32(u32::from_ne_bytes(bytes[..4].try_into().unwrap())),
                    indent: u32::from_ne_bytes(bytes[4..].try_into().unwrap()),
                });
            }
        }
        self.lookahead.contents.clear();
        self.lookahead.offset = 0;
        self.lookahead.contents.reserve(8);
    }
}

pub(crate) fn create() -> Box<dyn ExternalScanner> {
    Box::new(Scanner {
        contexts: Vec::with_capacity(8),
        newline: Newline::default(),
        lookahead: Lookahead {
            contents: Vec::with_capacity(8),
            offset: 0,
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Eq)]
    enum Event {
        Advance(usize, bool),
        Mark(usize),
        Column(usize),
        Result(u16),
    }

    struct TestLexer {
        text: Vec<i32>,
        position: usize,
        end: usize,
        result: u16,
        events: Vec<Event>,
    }

    impl TestLexer {
        fn new(text: &str) -> Self {
            Self {
                text: text.chars().map(|c| c as i32).collect(),
                position: 0,
                end: 0,
                result: 0,
                events: Vec::new(),
            }
        }
    }

    impl Lexer for TestLexer {
        fn lookahead(&self) -> i32 {
            self.text.get(self.position).copied().unwrap_or(0)
        }
        fn result_symbol(&self) -> u16 {
            self.result
        }
        fn set_result_symbol(&mut self, symbol: u16) {
            self.events.push(Event::Result(symbol));
            self.result = symbol;
        }
        fn advance(&mut self, skip: bool) {
            self.events.push(Event::Advance(self.position, skip));
            if !self.eof() {
                self.position += 1;
            }
        }
        fn mark_end(&mut self) {
            self.events.push(Event::Mark(self.position));
            self.end = self.position;
        }
        fn get_column(&mut self) -> u32 {
            self.events.push(Event::Column(self.position));
            self.text[..self.position]
                .iter()
                .rev()
                .take_while(|&&c| c != '\n' as i32)
                .count() as u32
        }
        fn is_at_included_range_start(&self) -> bool {
            false
        }
        fn eof(&self) -> bool {
            self.position == self.text.len()
        }
    }

    fn scanner() -> Scanner {
        Scanner {
            contexts: Vec::with_capacity(8),
            newline: Newline::default(),
            lookahead: Lookahead::default(),
        }
    }

    fn symbols(enabled: &[Symbol]) -> [bool; 49] {
        let mut result = [false; 49];
        for &s in enabled {
            result[s as usize] = true;
        }
        result
    }

    #[test]
    fn snapshot_bytes_and_empty_reset() {
        let mut state = scanner();
        // calloc creates Inactive; only empty deserialization selects Resume.
        assert_eq!(state.newline.state, NewlineState::Inactive);
        state.contexts = vec![
            Context {
                sort: DeclLayout,
                indent: 0,
            },
            Context {
                sort: DoLayout,
                indent: 0x1234,
            },
            Context {
                sort: TExp,
                indent: 0,
            },
        ];
        state.newline = Newline {
            state: NewlineState::Process,
            end: Lexed::Where,
            indent: 0x87654321,
            eof: false,
            no_semi: true,
            skip_semi: false,
            unsafe_lookahead: true,
        };
        let mut expected = Vec::new();
        for word in [3u32, 2, 2, 0x87654321] {
            expected.extend(word.to_ne_bytes());
        }
        expected.extend([0, 1, 0, 1]);
        for word in [0u32, 0, 1, 0x1234, 7, 0] {
            expected.extend(word.to_ne_bytes());
        }
        let mut buffer = [0xab; 1024];
        let size = state.serialize(&mut buffer);
        assert_eq!(&buffer[..size], expected);
        assert_eq!(buffer[size], 0xab);
        let mut restored = scanner();
        restored.lookahead.contents.extend([1, 2, 3]);
        restored.lookahead.offset = 2;
        restored.deserialize(&buffer[..size]);
        assert_eq!(restored.contexts, state.contexts);
        assert_eq!(restored.newline, state.newline);
        assert!(restored.lookahead.contents.is_empty());
        assert_eq!(restored.lookahead.offset, 0);
        assert_eq!(restored.serialize(&mut buffer), size);
        assert_eq!(&buffer[..size], expected);
        restored.deserialize(&[]);
        assert!(restored.contexts.is_empty());
        assert_eq!(
            restored.newline,
            Newline {
                state: NewlineState::Resume,
                ..Newline::default()
            }
        );
    }

    #[test]
    fn snapshot_capacity_is_a_hard_limit() {
        let mut state = scanner();
        let mut buffer = [0xab; 1024];
        state.contexts.resize(125, Context::default());
        assert_eq!(state.serialize(&mut buffer), 1020);
        assert_eq!(buffer[1020], 0xab);
        state.contexts.push(Context::default());
        assert_eq!(state.serialize(&mut buffer), 0);
        assert_eq!(&buffer[..4], &126u32.to_ne_bytes());
    }

    #[test]
    fn lookahead_reuses_cached_characters_and_does_not_append_eof() {
        let mut state = scanner();
        let mut lexer = TestLexer::new("A.b");
        let valid = symbols(&[]);
        let mut env = Env {
            lexer: &mut lexer,
            symbols: &valid,
            symop: 0,
            state: &mut state,
        };
        assert_eq!(env.peek(2), 'b' as i32);
        assert_eq!(env.peek0(), 'A' as i32);
        assert_eq!(env.peek1(), '.' as i32);
        env.skip_over(1); // Already cached: do not skip the current 'b'.
        env.reset_lookahead_to(2);
        assert_eq!(env.peek0(), 'b' as i32);
        assert_eq!(env.peek(4), 0);
        assert_eq!(env.cached_peek(4), 0);
        assert_eq!(env.lookahead_size(), 3);
        assert_eq!(
            lexer.events,
            vec![
                Event::Advance(0, false),
                Event::Advance(1, false),
                Event::Advance(2, false)
            ]
        );
    }

    #[test]
    fn unicode_classes_are_the_grammars_tables_not_rust_categories() {
        assert!(is_space_char(' ' as i32));
        assert!(is_space_char('\u{3000}' as i32));
        assert!(!is_space_char('\t' as i32));
        assert!(!is_space_char('\n' as i32));
        assert!(is_varid_start_char('λ' as i32));
        assert!(is_conid_start_char('Λ' as i32));
        assert!(is_identifier_char('\u{e0100}' as i32));
        assert!(!is_identifier_char('\u{e01f0}' as i32));
        // Keep the gaps between bitmap ranges, even when the Unicode category
        // would otherwise look suitable (e.g. CJK unified ideographs).
        assert!(is_identifier_char(19968));
        assert!(!is_identifier_char(19969));
        assert!(symop_char('⇒' as i32));
        assert!(!symop_char('(' as i32));
    }

    #[test]
    fn nested_comments_haddock_and_consecutive_lines() {
        let valid = symbols(&[]);
        for (text, expected, end) in [
            ("{-| doc {- nested -} end -}next", Symbol::Haddock, 27),
            ("-- one\n-- two\nnext", Symbol::Comment, 13),
            ("-- ^ doc\n  -- indented", Symbol::Haddock, 8),
        ] {
            let mut state = scanner();
            let mut lexer = TestLexer::new(text);
            assert!(state.scan(&mut lexer, &valid));
            assert_eq!(lexer.result, expected as u16, "{text}");
            assert_eq!(lexer.end, end, "{text}");
        }
    }

    #[test]
    fn quasiquote_delimiter_and_eof_marks() {
        let valid = symbols(&[Symbol::QqBody]);
        for (text, end) in [("body|]tail", 4), ("body⟧tail", 4), ("body", 0), ("x|y", 1)] {
            let mut state = scanner();
            let mut lexer = TestLexer::new(text);
            assert!(state.scan(&mut lexer, &valid));
            assert_eq!(lexer.result, Symbol::QqBody as u16);
            assert_eq!(lexer.end, end, "{text}");
        }
    }

    #[test]
    fn newline_lookahead_precedes_layout_ends_and_semicolons() {
        let valid = symbols(&[Symbol::End, Symbol::Semicolon]);
        let mut state = scanner();
        state.contexts = vec![
            Context {
                sort: DeclLayout,
                indent: 0,
            },
            Context {
                sort: DoLayout,
                indent: 2,
            },
        ];
        let mut lexer = TestLexer::new("\nwhere x");
        assert!(state.scan(&mut lexer, &valid));
        assert_eq!(lexer.result, Symbol::Update as u16);
        assert_eq!(lexer.end, 0);
        assert_eq!(state.newline.state, NewlineState::Init);
        assert_eq!(state.newline.end, Lexed::Where);
        assert!(state.newline.unsafe_lookahead);
        // The runtime restores the snapshot and starts again at mark_end, not
        // at the lexer's lookahead position. End the inner layout first.
        let mut buffer = [0; 1024];
        let size = state.serialize(&mut buffer);
        state.deserialize(&buffer[..size]);
        lexer = TestLexer::new("\nwhere x");
        assert!(state.scan(&mut lexer, &valid));
        assert_eq!(lexer.result, Symbol::End as u16);
        assert_eq!(
            lexer.events,
            vec![Event::Mark(0), Event::Result(Symbol::End as u16)]
        );
        assert_eq!(state.contexts.len(), 1);
        assert_eq!(state.newline.state, NewlineState::Process);
    }

    #[test]
    fn error_recovery_does_not_touch_the_lexer() {
        let mut state = scanner();
        let mut lexer = TestLexer::new("{-# anything #-}");
        assert!(!state.scan(&mut lexer, &symbols(&[Symbol::Fail])));
        assert!(lexer.events.is_empty());
    }
}
