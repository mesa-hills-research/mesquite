//! Query compiler and execution data, mirroring query.c. See PORTING.md.
use crate::{
    clock::{Clock, DurationMicros},
    language::Language,
    node::Node,
    point::Point,
    query_api::{CaptureQuantifier, QueryCapture, QueryCursorOptions, QueryCursorState},
    tree_cursor::TreeCursor,
    types::{FieldId, StateId, Symbol},
};

#[path = "query_access.rs"]
mod access;
#[path = "query_analysis.rs"]
mod analysis;
#[path = "query_exec.rs"]
mod exec;
#[path = "query_iter.rs"]
mod iter;
#[path = "query_parse.rs"]
mod parse;
pub(crate) use access::*;
pub(crate) use analysis::*;
pub(crate) use exec::*;
pub(crate) use iter::*;
pub(crate) use parse::*;

pub(crate) const MAX_STEP_CAPTURE_COUNT: usize = 3;
pub(crate) const MAX_NEGATED_FIELD_COUNT: usize = 8;
pub(crate) const MAX_STATE_PREDECESSOR_COUNT: usize = 256;
pub(crate) const MAX_ANALYSIS_STATE_DEPTH: usize = 8;
pub(crate) const MAX_ANALYSIS_ITERATION_COUNT: usize = 256;
pub(crate) const PATTERN_DONE_MARKER: u16 = u16::MAX;
pub(crate) const NONE: u16 = u16::MAX;
pub(crate) const WILDCARD_SYMBOL: Symbol = 0;
pub(crate) const OP_COUNT_PER_QUERY_TIMEOUT_CHECK: u32 = 100;

/// Byte offsets replace pointers into the UTF-8 source. `next` can be DECODE_ERROR.
pub(crate) struct Stream<'source> {
    pub source: &'source [u8],
    pub input: usize,
    pub next: i32,
    pub next_size: u8,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct QueryStep {
    pub symbol: Symbol,
    pub supertype_symbol: Symbol,
    pub field: FieldId,
    pub capture_ids: [u16; MAX_STEP_CAPTURE_COUNT],
    pub depth: u16,
    pub alternative_index: u16,
    pub negated_field_list_id: u16,
    pub is_named: bool,
    pub is_immediate: bool,
    pub is_last_child: bool,
    pub is_pass_through: bool,
    pub is_dead_end: bool,
    pub alternative_is_immediate: bool,
    pub contains_captures: bool,
    pub root_pattern_guaranteed: bool,
    pub parent_pattern_guaranteed: bool,
    pub is_missing: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct Slice {
    pub offset: u32,
    pub length: u32,
}

/// Retain the flat, NUL-separated byte layout and insertion order of C's table.
#[derive(Debug, Default)]
pub(crate) struct SymbolTable {
    pub characters: Vec<u8>,
    pub slices: Vec<Slice>,
}
pub(crate) type CaptureQuantifiers = Vec<CaptureQuantifier>;

#[derive(Clone, Copy, Debug)]
pub(crate) struct PatternEntry {
    pub step_index: u16,
    pub pattern_index: u16,
    pub is_rooted: bool,
}
#[derive(Clone, Copy, Debug)]
pub(crate) struct QueryPattern {
    pub steps: Slice,
    pub predicate_steps: Slice,
    pub start_byte: u32,
    pub end_byte: u32,
    pub is_non_local: bool,
}
#[derive(Clone, Copy, Debug)]
pub(crate) struct StepOffset {
    pub byte_offset: u32,
    pub step_index: u16,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum PredicateStepKind {
    Done,
    Capture,
    String,
}
#[derive(Clone, Copy, Debug)]
pub(crate) struct PredicateStep {
    pub kind: PredicateStepKind,
    pub value_id: u32,
}

/// C parser status, including the internal -1 PARENT_DONE sentinel. This is not
/// the public binding error: Predicate errors are diagnosed by query-api.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum QueryErrorCode {
    ParentDone,
    None,
    Syntax,
    NodeType,
    Field,
    Capture,
    Structure,
    Language,
}
pub(crate) const PARENT_DONE: QueryErrorCode = QueryErrorCode::ParentDone;
#[derive(Clone, Copy, Debug)]
pub(crate) struct QueryCompileError {
    pub offset: u32,
    pub kind: QueryErrorCode,
}

#[derive(Debug)]
pub(crate) struct CompiledQuery {
    pub captures: SymbolTable,
    pub predicate_values: SymbolTable,
    pub capture_quantifiers: Vec<CaptureQuantifiers>,
    pub steps: Vec<QueryStep>,
    pub pattern_map: Vec<PatternEntry>,
    pub predicate_steps: Vec<PredicateStep>,
    pub patterns: Vec<QueryPattern>,
    pub step_offsets: Vec<StepOffset>,
    pub negated_fields: Vec<FieldId>,
    pub string_buffer: Vec<u8>,
    pub repeat_symbols_with_rootless_patterns: Vec<Symbol>,
    pub language: Language,
    pub wildcard_root_pattern_count: u16,
}

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct AnalysisStateEntry {
    pub parse_state: StateId,
    pub parent_symbol: Symbol,
    pub child_index: u16,
    /// Assignments must mask to C's 15-bit field.
    pub field_id: FieldId,
    pub done: bool,
}
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct AnalysisState {
    pub stack: [AnalysisStateEntry; MAX_ANALYSIS_STATE_DEPTH],
    pub depth: u16,
    pub step_index: u16,
    pub root_symbol: Symbol,
}
/// States have value identity, not pointer identity. Moving them between these
/// ordered Vecs replaces C's allocation pool without changing insertion order.
pub(crate) type AnalysisStateSet = Vec<AnalysisState>;
#[derive(Debug, Default)]
pub(crate) struct QueryAnalysis {
    pub states: AnalysisStateSet,
    pub next_states: AnalysisStateSet,
    pub deeper_states: AnalysisStateSet,
    pub state_pool: AnalysisStateSet,
    pub final_step_indices: Vec<u16>,
    pub finished_parent_symbols: Vec<Symbol>,
    pub did_abort: bool,
}
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct AnalysisSubgraphNode {
    pub state: StateId,
    pub production_id: u16,
    /// Assignments must mask to C's 7-bit field.
    pub child_index: u8,
    pub done: bool,
}
#[derive(Debug, Default)]
pub(crate) struct AnalysisSubgraph {
    pub symbol: Symbol,
    pub start_states: Vec<StateId>,
    pub nodes: Vec<AnalysisSubgraphNode>,
}
pub(crate) type AnalysisSubgraphArray = Vec<AnalysisSubgraph>;
#[derive(Debug)]
pub(crate) struct StatePredecessorMap {
    /// Flat rows of 257 entries: count, then at most 256 predecessors.
    pub contents: Vec<StateId>,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct QueryState {
    pub id: u32,
    pub capture_list_id: u32,
    pub start_depth: u16,
    pub step_index: u16,
    pub pattern_index: u16,
    /// Assignments/increments must mask to C's 12-bit field.
    pub consumed_capture_count: u16,
    pub seeking_immediate_match: bool,
    pub has_in_progress_alternatives: bool,
    pub dead: bool,
    pub needs_parent: bool,
}
pub(crate) type CaptureList<'tree> = Vec<QueryCapture<'tree>>;
#[derive(Debug)]
pub(crate) struct CaptureListSlot<'tree> {
    pub captures: CaptureList<'tree>,
    /// Replaces C's UINT32_MAX length. Release retains the buffer and captures.
    pub in_use: bool,
}
#[derive(Debug)]
pub(crate) struct CaptureListPool<'tree> {
    pub list: Vec<CaptureListSlot<'tree>>,
    pub empty_list: CaptureList<'tree>,
    pub max_capture_list_count: u32,
    pub free_capture_list_count: u32,
}

/// Lifetime-independent half of TSQueryCursor. It survives executions; in
/// particular, lowering the limit must not discard already allocated pool slots.
#[derive(Debug)]
pub(crate) struct CursorConfig {
    pub max_capture_list_count: u32,
    pub max_start_depth: u32,
    pub start_byte: u32,
    pub end_byte: u32,
    pub start_point: Point,
    pub end_point: Point,
    pub timeout_duration: DurationMicros,
    pub did_exceed_match_limit: bool,
    /// Full pool slot count, updated by prepare_to_capture on each acquisition.
    /// It persists even if execution stops early or the match limit is lowered.
    pub allocated_capture_list_count: u32,
    /// States contain ids, not borrowed nodes. Mutate these Vecs in place through
    /// QueryExecution.config and clear them on exec; do not move them out.
    pub states: Vec<QueryState>,
    pub finished_states: Vec<QueryState>,
}

/// Borrowed half of TSQueryCursor. No query/tree/callback reference is stored in
/// the persistent public cursor. Settings, states and limit status live in `config`.
///
/// Do not implement Drop here (or on another wrapper borrowing the cursor).
/// Drop checking would keep the exclusive cursor borrow live until scope exit,
/// rejecting binding-compatible access to the cursor after the iterator's last use.
/// Persist bookkeeping as execution runs; implicit field drop needs no write-back.
pub(crate) struct QueryExecution<'query, 'tree: 'query> {
    pub config: &'query mut CursorConfig,
    pub query: &'query CompiledQuery,
    pub cursor: TreeCursor<'tree>,
    pub capture_list_pool: CaptureListPool<'tree>,
    pub depth: u32,
    pub next_state_id: u32,
    pub end_clock: Clock,
    pub query_options: QueryCursorOptions<'query>,
    pub query_state: QueryCursorState,
    pub operation_count: u32,
    pub on_visible_node: bool,
    pub ascending: bool,
    pub halted: bool,
}

/// Owned, transient result. Clone captures before C's released pool slot is reused;
/// truncate the exposed length to u16 just as TSQueryMatch.capture_count does.
pub(crate) struct QueryMatchData<'tree> {
    pub id: u32,
    pub pattern_index: u16,
    pub captures: CaptureList<'tree>,
}
#[derive(Clone, Copy, Debug)]
pub(crate) struct InProgressCapture {
    pub state_index: u32,
    pub byte_offset: u32,
    pub pattern_index: u32,
    pub is_definite: bool,
}

// query-1 functions follow. Other units are isolated in sibling source files.
pub(crate) fn stream_advance(stream: &mut Stream<'_>) -> bool {
    todo!("query-1: stream_advance")
}

pub(crate) fn stream_reset(stream: &mut Stream<'_>, input: usize) {
    todo!("query-1: stream_reset")
}

pub(crate) fn stream_new(source: &[u8]) -> Stream<'_> {
    todo!("query-1: stream_new")
}

pub(crate) fn stream_skip_whitespace(stream: &mut Stream<'_>) {
    todo!("query-1: stream_skip_whitespace")
}

pub(crate) fn stream_is_ident_start(stream: &Stream<'_>) -> bool {
    todo!("query-1: stream_is_ident_start")
}

pub(crate) fn stream_scan_identifier(stream: &mut Stream<'_>) {
    todo!("query-1: stream_scan_identifier")
}

pub(crate) fn stream_offset(stream: &Stream<'_>) -> u32 {
    todo!("query-1: stream_offset")
}

pub(crate) fn capture_list_pool_new<'tree>() -> CaptureListPool<'tree> {
    todo!("query-1: capture_list_pool_new")
}

pub(crate) fn capture_list_pool_reset(pool: &mut CaptureListPool<'_>) {
    todo!("query-1: capture_list_pool_reset")
}

pub(crate) fn capture_list_pool_delete(pool: CaptureListPool<'_>) {
    todo!("query-1: capture_list_pool_delete")
}

pub(crate) fn capture_list_pool_get<'a, 'tree>(
    pool: &'a CaptureListPool<'tree>,
    id: u16,
) -> &'a CaptureList<'tree> {
    todo!("query-1: capture_list_pool_get")
}

pub(crate) fn capture_list_pool_get_mut<'a, 'tree>(
    pool: &'a mut CaptureListPool<'tree>,
    id: u16,
) -> &'a mut CaptureList<'tree> {
    todo!("query-1: capture_list_pool_get_mut")
}

pub(crate) fn capture_list_pool_is_empty(pool: &CaptureListPool<'_>) -> bool {
    todo!("query-1: capture_list_pool_is_empty")
}

pub(crate) fn capture_list_pool_acquire(pool: &mut CaptureListPool<'_>) -> u16 {
    todo!("query-1: capture_list_pool_acquire")
}

pub(crate) fn capture_list_pool_release(pool: &mut CaptureListPool<'_>, id: u16) {
    todo!("query-1: capture_list_pool_release")
}

pub(crate) fn quantifier_mul(
    left: CaptureQuantifier,
    right: CaptureQuantifier,
) -> CaptureQuantifier {
    todo!("query-1: quantifier_mul")
}

pub(crate) fn quantifier_join(
    left: CaptureQuantifier,
    right: CaptureQuantifier,
) -> CaptureQuantifier {
    todo!("query-1: quantifier_join")
}

pub(crate) fn quantifier_add(
    left: CaptureQuantifier,
    right: CaptureQuantifier,
) -> CaptureQuantifier {
    todo!("query-1: quantifier_add")
}

pub(crate) fn capture_quantifiers_new() -> CaptureQuantifiers {
    todo!("query-1: capture_quantifiers_new")
}

pub(crate) fn capture_quantifiers_delete(quantifiers: CaptureQuantifiers) {
    todo!("query-1: capture_quantifiers_delete")
}

pub(crate) fn capture_quantifiers_clear(quantifiers: &mut CaptureQuantifiers) {
    todo!("query-1: capture_quantifiers_clear")
}

pub(crate) fn capture_quantifiers_replace(
    target: &mut CaptureQuantifiers,
    source: &CaptureQuantifiers,
) {
    todo!("query-1: capture_quantifiers_replace")
}

pub(crate) fn capture_quantifier_for_id(
    quantifiers: &CaptureQuantifiers,
    id: u16,
) -> CaptureQuantifier {
    todo!("query-1: capture_quantifier_for_id")
}

pub(crate) fn capture_quantifiers_add_for_id(
    quantifiers: &mut CaptureQuantifiers,
    id: u16,
    quantifier: CaptureQuantifier,
) {
    todo!("query-1: capture_quantifiers_add_for_id")
}

pub(crate) fn capture_quantifiers_add_all(
    target: &mut CaptureQuantifiers,
    source: &CaptureQuantifiers,
) {
    todo!("query-1: capture_quantifiers_add_all")
}

pub(crate) fn capture_quantifiers_mul(
    quantifiers: &mut CaptureQuantifiers,
    quantifier: CaptureQuantifier,
) {
    todo!("query-1: capture_quantifiers_mul")
}

pub(crate) fn capture_quantifiers_join_all(
    target: &mut CaptureQuantifiers,
    source: &CaptureQuantifiers,
) {
    todo!("query-1: capture_quantifiers_join_all")
}

pub(crate) fn symbol_table_new() -> SymbolTable {
    todo!("query-1: symbol_table_new")
}

pub(crate) fn symbol_table_delete(table: SymbolTable) {
    todo!("query-1: symbol_table_delete")
}

pub(crate) fn symbol_table_id_for_name(table: &SymbolTable, name: &[u8]) -> i32 {
    todo!("query-1: symbol_table_id_for_name")
}

pub(crate) fn symbol_table_name_for_id(table: &SymbolTable, id: u16) -> &[u8] {
    todo!("query-1: symbol_table_name_for_id")
}

pub(crate) fn symbol_table_insert_name(table: &mut SymbolTable, name: &[u8]) -> u16 {
    todo!("query-1: symbol_table_insert_name")
}

pub(crate) fn query_step__new(symbol: Symbol, depth: u16, is_immediate: bool) -> QueryStep {
    todo!("query-1: query_step__new")
}

pub(crate) fn query_step__add_capture(step: &mut QueryStep, capture_id: u16) {
    todo!("query-1: query_step__add_capture")
}

pub(crate) fn query_step__remove_capture(step: &mut QueryStep, capture_id: u16) {
    todo!("query-1: query_step__remove_capture")
}

pub(crate) fn state_predecessor_map_new(language: &Language) -> StatePredecessorMap {
    todo!("query-1: state_predecessor_map_new")
}

pub(crate) fn state_predecessor_map_delete(map: StatePredecessorMap) {
    todo!("query-1: state_predecessor_map_delete")
}

pub(crate) fn state_predecessor_map_add(
    map: &mut StatePredecessorMap,
    state: StateId,
    predecessor: StateId,
) {
    todo!("query-1: state_predecessor_map_add")
}

pub(crate) fn state_predecessor_map_get(map: &StatePredecessorMap, state: StateId) -> &[StateId] {
    todo!("query-1: state_predecessor_map_get")
}

pub(crate) fn analysis_state__recursion_depth(state: &AnalysisState) -> u32 {
    todo!("query-1: analysis_state__recursion_depth")
}

pub(crate) fn analysis_state__compare(left: &AnalysisState, right: &AnalysisState) -> i32 {
    todo!("query-1: analysis_state__compare")
}

pub(crate) fn analysis_state__top(state: &mut AnalysisState) -> &mut AnalysisStateEntry {
    todo!("query-1: analysis_state__top")
}

pub(crate) fn analysis_state__has_supertype(state: &AnalysisState, symbol: Symbol) -> bool {
    todo!("query-1: analysis_state__has_supertype")
}

pub(crate) fn analysis_state_pool__clone_or_reuse(
    pool: &mut AnalysisStateSet,
    borrowed_item: &AnalysisState,
) -> AnalysisState {
    todo!("query-1: analysis_state_pool__clone_or_reuse")
}

pub(crate) fn analysis_state_set__insert_sorted(
    set: &mut AnalysisStateSet,
    pool: &mut AnalysisStateSet,
    borrowed_item: &AnalysisState,
) {
    todo!("query-1: analysis_state_set__insert_sorted")
}

pub(crate) fn analysis_state_set__push(
    set: &mut AnalysisStateSet,
    pool: &mut AnalysisStateSet,
    borrowed_item: &AnalysisState,
) {
    todo!("query-1: analysis_state_set__push")
}

pub(crate) fn analysis_state_set__clear(set: &mut AnalysisStateSet, pool: &mut AnalysisStateSet) {
    todo!("query-1: analysis_state_set__clear")
}

pub(crate) fn analysis_state_set__delete(set: AnalysisStateSet) {
    todo!("query-1: analysis_state_set__delete")
}

pub(crate) fn query_analysis__new() -> QueryAnalysis {
    todo!("query-1: query_analysis__new")
}

pub(crate) fn query_analysis__delete(analysis: QueryAnalysis) {
    todo!("query-1: query_analysis__delete")
}

pub(crate) fn analysis_subgraph_node__compare(
    left: &AnalysisSubgraphNode,
    right: &AnalysisSubgraphNode,
) -> i32 {
    todo!("query-1: analysis_subgraph_node__compare")
}

pub(crate) fn ts_query__pattern_map_search(query: &CompiledQuery, needle: Symbol) -> (bool, u32) {
    todo!("query-1: ts_query__pattern_map_search")
}
