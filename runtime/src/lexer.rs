use crate::{length::*, types::*};
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct ColumnData {
    pub value: u32,
    pub valid: bool,
}
#[derive(Debug, Default)]
pub(crate) struct LexerState {
    pub lookahead: i32,
    pub result_symbol: Symbol,
    pub current_position: Length,
    pub token_start_position: Length,
    pub token_end_position: Length,
    pub included_ranges: Vec<Range>,
    pub encoding: InputEncoding,
    pub current_included_range_index: u32,
    pub chunk_start: u32,
    pub chunk_size: u32,
    pub lookahead_size: u32,
    pub did_get_column: bool,
    pub column_data: ColumnData,
    pub debug_buffer: String,
}
/// The generated lexers see this transient adapter. No client callback or chunk
/// reference is ever stored in the persistent parser.
pub(crate) struct Lexer<'a> {
    pub state: &'a mut LexerState,
    pub input: &'a mut dyn Input,
    pub logger: Option<&'a mut crate::parser::Logger>,
}
impl ts_port_tables::Lexer for Lexer<'_> {
    fn lookahead(&self) -> i32 {
        self.state.lookahead
    }
    fn result_symbol(&self) -> Symbol {
        self.state.result_symbol
    }
    fn set_result_symbol(&mut self, symbol: Symbol) {
        self.state.result_symbol = symbol;
    }
    fn advance(&mut self, skip: bool) {
        ts_lexer__advance(self, skip);
    }
    fn mark_end(&mut self) {
        ts_lexer_mark_end(self.state);
    }
    fn get_column(&mut self) -> u32 {
        ts_lexer__get_column(self)
    }
    fn is_at_included_range_start(&self) -> bool {
        ts_lexer__is_at_included_range_start(self.state)
    }
    fn eof(&self) -> bool {
        ts_lexer__eof(self.state)
    }
    fn log(&mut self, args: std::fmt::Arguments<'_>) {
        ts_lexer__log(self, args);
    }
}

pub(crate) fn ts_lexer__set_column_data(lexer: &mut LexerState, value: u32) {
    todo!("lexer: ts_lexer__set_column_data")
}

pub(crate) fn ts_lexer__increment_column_data(lexer: &mut LexerState) {
    todo!("lexer: ts_lexer__increment_column_data")
}

pub(crate) fn ts_lexer__invalidate_column_data(lexer: &mut LexerState) {
    todo!("lexer: ts_lexer__invalidate_column_data")
}

pub(crate) fn ts_lexer__eof(lexer: &LexerState) -> bool {
    todo!("lexer: ts_lexer__eof")
}

pub(crate) fn ts_lexer__clear_chunk(lexer: &mut LexerState) {
    todo!("lexer: ts_lexer__clear_chunk")
}

pub(crate) fn ts_lexer__get_chunk(lexer: &mut Lexer<'_>) {
    todo!("lexer: ts_lexer__get_chunk")
}

pub(crate) fn ts_lexer__get_lookahead(lexer: &mut Lexer<'_>) {
    todo!("lexer: ts_lexer__get_lookahead")
}

pub(crate) fn ts_lexer_goto(lexer: &mut LexerState, position: Length) {
    todo!("lexer: ts_lexer_goto")
}

pub(crate) fn ts_lexer__do_advance(lexer: &mut Lexer<'_>, skip: bool) {
    todo!("lexer: ts_lexer__do_advance")
}

pub(crate) fn ts_lexer__advance(lexer: &mut Lexer<'_>, skip: bool) {
    todo!("lexer: ts_lexer__advance")
}

pub(crate) fn ts_lexer__mark_end(lexer: &mut LexerState) {
    todo!("lexer: ts_lexer__mark_end")
}

pub(crate) fn ts_lexer__get_column(lexer: &mut Lexer<'_>) -> u32 {
    todo!("lexer: ts_lexer__get_column")
}

pub(crate) fn ts_lexer__is_at_included_range_start(lexer: &LexerState) -> bool {
    todo!("lexer: ts_lexer__is_at_included_range_start")
}

pub(crate) fn ts_lexer__log(lexer: &mut Lexer<'_>, args: std::fmt::Arguments<'_>) {
    todo!("lexer: ts_lexer__log")
}

pub(crate) fn ts_lexer_init() -> LexerState {
    todo!("lexer: ts_lexer_init")
}

pub(crate) fn ts_lexer_delete(lexer: &mut LexerState) {
    todo!("lexer: ts_lexer_delete")
}

pub(crate) fn ts_lexer_set_input(lexer: &mut LexerState, encoding: InputEncoding) {
    todo!("lexer: ts_lexer_set_input")
}

pub(crate) fn ts_lexer_reset(lexer: &mut LexerState, position: Length) {
    todo!("lexer: ts_lexer_reset")
}

pub(crate) fn ts_lexer_start(lexer: &mut Lexer<'_>) {
    todo!("lexer: ts_lexer_start")
}

pub(crate) fn ts_lexer_finish(lexer: &mut LexerState, lookahead_end_byte: &mut u32) {
    todo!("lexer: ts_lexer_finish")
}

pub(crate) fn ts_lexer_mark_end(lexer: &mut LexerState) {
    todo!("lexer: ts_lexer_mark_end")
}

pub(crate) fn ts_lexer_set_included_ranges(lexer: &mut LexerState, ranges: &[Range]) -> bool {
    todo!("lexer: ts_lexer_set_included_ranges")
}

pub(crate) fn ts_lexer_included_ranges(lexer: &LexerState) -> &[Range] {
    todo!("lexer: ts_lexer_included_ranges")
}
