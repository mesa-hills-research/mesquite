//! Query compiler and execution data, mirroring query.c.
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

// The query parser, analysis and execution are in the sibling `query_*` files.
pub(crate) fn stream_advance(stream: &mut Stream<'_>) -> bool {
    stream.input += usize::from(stream.next_size);
    if stream.input < stream.source.len() {
        let (size, next) = crate::unicode::ts_decode_utf8(&stream.source[stream.input..]);
        stream.next = next;
        if size > 0 {
            stream.next_size = size as u8;
            return true;
        }
    } else {
        stream.next_size = 0;
        stream.next = 0;
    }
    false
}

pub(crate) fn stream_reset(stream: &mut Stream<'_>, input: usize) {
    stream.input = input;
    stream.next_size = 0;
    stream_advance(stream);
}

pub(crate) fn stream_new(source: &[u8]) -> Stream<'_> {
    let mut stream = Stream {
        source,
        input: 0,
        next: 0,
        next_size: 0,
    };
    stream_advance(&mut stream);
    stream
}

pub(crate) fn stream_skip_whitespace(stream: &mut Stream<'_>) {
    loop {
        // iswspace in C's default locale, including vertical tab.
        if matches!(stream.next, 0x09..=0x0d | 0x20) {
            stream_advance(stream);
        } else if stream.next == i32::from(b';') {
            stream_advance(stream);
            while stream.next != 0 && stream.next != i32::from(b'\n') {
                if !stream_advance(stream) {
                    break;
                }
            }
        } else {
            break;
        }
    }
}

pub(crate) fn stream_is_ident_start(stream: &Stream<'_>) -> bool {
    // iswalnum in C's default locale, plus '_' and '-'.
    matches!(stream.next, 0x30..=0x39 | 0x41..=0x5a | 0x61..=0x7a | 0x5f | 0x2d)
}

pub(crate) fn stream_scan_identifier(stream: &mut Stream<'_>) {
    loop {
        stream_advance(stream);
        if !(stream_is_ident_start(stream) || matches!(stream.next, 0x2e | 0x3f | 0x21)) {
            break;
        }
    }
}

pub(crate) fn stream_offset(stream: &Stream<'_>) -> u32 {
    stream.input as u32
}

pub(crate) fn capture_list_pool_new<'tree>() -> CaptureListPool<'tree> {
    CaptureListPool {
        list: Vec::new(),
        empty_list: Vec::new(),
        max_capture_list_count: u32::MAX,
        free_capture_list_count: 0,
    }
}

pub(crate) fn capture_list_pool_reset(pool: &mut CaptureListPool<'_>) {
    // C truncates the loop bound to u16, but not the free-list count.
    let count = pool.list.len() as u16;
    for slot in &mut pool.list[..usize::from(count)] {
        slot.in_use = false;
    }
    pool.free_capture_list_count = pool.list.len() as u32;
}

pub(crate) fn capture_list_pool_delete(pool: CaptureListPool<'_>) {
    drop(pool);
}

pub(crate) fn capture_list_pool_get<'a, 'tree>(
    pool: &'a CaptureListPool<'tree>,
    id: u16,
) -> &'a CaptureList<'tree> {
    pool.list
        .get(usize::from(id))
        .map_or(&pool.empty_list, |slot| &slot.captures)
}

pub(crate) fn capture_list_pool_get_mut<'a, 'tree>(
    pool: &'a mut CaptureListPool<'tree>,
    id: u16,
) -> &'a mut CaptureList<'tree> {
    &mut pool.list[usize::from(id)].captures
}

pub(crate) fn capture_list_pool_is_empty(pool: &CaptureListPool<'_>) -> bool {
    pool.free_capture_list_count == 0 && pool.list.len() as u32 >= pool.max_capture_list_count
}

pub(crate) fn capture_list_pool_acquire(pool: &mut CaptureListPool<'_>) -> u16 {
    if pool.free_capture_list_count > 0 {
        for i in 0..pool.list.len() as u16 {
            let slot = &mut pool.list[usize::from(i)];
            if !slot.in_use {
                slot.captures.clear();
                slot.in_use = true;
                pool.free_capture_list_count -= 1;
                return i;
            }
        }
    }

    let i = pool.list.len() as u32;
    if i >= pool.max_capture_list_count {
        return NONE;
    }
    pool.list.push(CaptureListSlot {
        captures: Vec::new(),
        in_use: true,
    });
    // Even a new slot's id is truncated by C's uint16_t return type.
    i as u16
}

pub(crate) fn capture_list_pool_release(pool: &mut CaptureListPool<'_>, id: u16) {
    if let Some(slot) = pool.list.get_mut(usize::from(id)) {
        slot.in_use = false;
        // Like C, a repeated release increments the count again.
        pool.free_capture_list_count = pool.free_capture_list_count.wrapping_add(1);
    }
}

pub(crate) fn quantifier_mul(
    left: CaptureQuantifier,
    right: CaptureQuantifier,
) -> CaptureQuantifier {
    use CaptureQuantifier::*;
    match left {
        Zero => Zero,
        ZeroOrOne => match right {
            Zero => Zero,
            ZeroOrOne | One => ZeroOrOne,
            ZeroOrMore | OneOrMore => ZeroOrMore,
        },
        ZeroOrMore => match right {
            Zero => Zero,
            ZeroOrOne | ZeroOrMore | One | OneOrMore => ZeroOrMore,
        },
        One => right,
        OneOrMore => match right {
            Zero => Zero,
            ZeroOrOne | ZeroOrMore => ZeroOrMore,
            One | OneOrMore => OneOrMore,
        },
    }
}

pub(crate) fn quantifier_join(
    left: CaptureQuantifier,
    right: CaptureQuantifier,
) -> CaptureQuantifier {
    use CaptureQuantifier::*;
    match left {
        Zero => match right {
            Zero => Zero,
            ZeroOrOne | One => ZeroOrOne,
            ZeroOrMore | OneOrMore => ZeroOrMore,
        },
        ZeroOrOne => match right {
            Zero | ZeroOrOne | One => ZeroOrOne,
            ZeroOrMore | OneOrMore => ZeroOrMore,
        },
        ZeroOrMore => ZeroOrMore,
        One => match right {
            Zero | ZeroOrOne => ZeroOrOne,
            ZeroOrMore => ZeroOrMore,
            One => One,
            OneOrMore => OneOrMore,
        },
        OneOrMore => match right {
            Zero | ZeroOrOne | ZeroOrMore => ZeroOrMore,
            One | OneOrMore => OneOrMore,
        },
    }
}

pub(crate) fn quantifier_add(
    left: CaptureQuantifier,
    right: CaptureQuantifier,
) -> CaptureQuantifier {
    use CaptureQuantifier::*;
    match left {
        Zero => right,
        ZeroOrOne => match right {
            Zero => ZeroOrOne,
            ZeroOrOne | ZeroOrMore => ZeroOrMore,
            One | OneOrMore => OneOrMore,
        },
        ZeroOrMore => match right {
            Zero | ZeroOrOne | ZeroOrMore => ZeroOrMore,
            One | OneOrMore => OneOrMore,
        },
        One => match right {
            Zero => One,
            ZeroOrOne | ZeroOrMore | One | OneOrMore => OneOrMore,
        },
        OneOrMore => OneOrMore,
    }
}

pub(crate) fn capture_quantifiers_new() -> CaptureQuantifiers {
    Vec::new()
}

pub(crate) fn capture_quantifiers_delete(quantifiers: CaptureQuantifiers) {
    drop(quantifiers);
}

pub(crate) fn capture_quantifiers_clear(quantifiers: &mut CaptureQuantifiers) {
    quantifiers.clear();
}

pub(crate) fn capture_quantifiers_replace(
    target: &mut CaptureQuantifiers,
    source: &CaptureQuantifiers,
) {
    target.clear();
    target.extend_from_slice(source);
}

pub(crate) fn capture_quantifier_for_id(
    quantifiers: &CaptureQuantifiers,
    id: u16,
) -> CaptureQuantifier {
    quantifiers
        .get(usize::from(id))
        .copied()
        .unwrap_or(CaptureQuantifier::Zero)
}

pub(crate) fn capture_quantifiers_add_for_id(
    quantifiers: &mut CaptureQuantifiers,
    id: u16,
    quantifier: CaptureQuantifier,
) {
    let id = usize::from(id);
    if quantifiers.len() <= id {
        quantifiers.resize(id + 1, CaptureQuantifier::Zero);
    }
    quantifiers[id] = quantifier_add(quantifiers[id], quantifier);
}

pub(crate) fn capture_quantifiers_add_all(
    target: &mut CaptureQuantifiers,
    source: &CaptureQuantifiers,
) {
    if target.len() < source.len() {
        target.resize(source.len(), CaptureQuantifier::Zero);
    }
    for id in 0..source.len() as u16 {
        let id = usize::from(id);
        target[id] = quantifier_add(target[id], source[id]);
    }
}

pub(crate) fn capture_quantifiers_mul(
    quantifiers: &mut CaptureQuantifiers,
    quantifier: CaptureQuantifier,
) {
    let count = quantifiers.len() as u16;
    for own_quantifier in &mut quantifiers[..usize::from(count)] {
        *own_quantifier = quantifier_mul(*own_quantifier, quantifier);
    }
}

pub(crate) fn capture_quantifiers_join_all(
    target: &mut CaptureQuantifiers,
    source: &CaptureQuantifiers,
) {
    if target.len() < source.len() {
        target.resize(source.len(), CaptureQuantifier::Zero);
    }
    for (own_quantifier, &quantifier) in target.iter_mut().zip(source) {
        *own_quantifier = quantifier_join(*own_quantifier, quantifier);
    }
    for own_quantifier in &mut target[source.len()..] {
        *own_quantifier = quantifier_join(*own_quantifier, CaptureQuantifier::Zero);
    }
}

pub(crate) fn symbol_table_new() -> SymbolTable {
    SymbolTable::default()
}

pub(crate) fn symbol_table_delete(table: SymbolTable) {
    drop(table);
}

pub(crate) fn symbol_table_id_for_name(table: &SymbolTable, name: &[u8]) -> i32 {
    for (id, slice) in table.slices.iter().enumerate() {
        if slice.length != name.len() as u32 {
            continue;
        }
        let start = slice.offset as usize;
        let stored = &table.characters[start..start + slice.length as usize];
        // C uses strncmp, not memcmp: equal-length names with identical
        // prefixes through an embedded NUL compare equal, even if tails differ.
        let mut equal = true;
        for (&left, &right) in stored.iter().zip(name) {
            if left != right {
                equal = false;
                break;
            }
            if left == 0 {
                break;
            }
        }
        if equal {
            return id as i32;
        }
    }
    -1
}

pub(crate) fn symbol_table_name_for_id(table: &SymbolTable, id: u16) -> &[u8] {
    let slice = table.slices[usize::from(id)];
    let start = slice.offset as usize;
    &table.characters[start..start + slice.length as usize]
}

pub(crate) fn symbol_table_insert_name(table: &mut SymbolTable, name: &[u8]) -> u16 {
    let id = symbol_table_id_for_name(table, name);
    if id >= 0 {
        return id as u16;
    }
    let slice = Slice {
        offset: table.characters.len() as u32,
        length: name.len() as u32,
    };
    table.characters.extend_from_slice(name);
    table.characters.push(0);
    table.slices.push(slice);
    (table.slices.len() - 1) as u16
}

pub(crate) fn query_step__new(symbol: Symbol, depth: u16, is_immediate: bool) -> QueryStep {
    QueryStep {
        symbol,
        supertype_symbol: 0,
        field: 0,
        capture_ids: [NONE; MAX_STEP_CAPTURE_COUNT],
        depth,
        alternative_index: NONE,
        negated_field_list_id: 0,
        is_named: false,
        is_immediate,
        is_last_child: false,
        is_pass_through: false,
        is_dead_end: false,
        alternative_is_immediate: false,
        contains_captures: false,
        root_pattern_guaranteed: false,
        parent_pattern_guaranteed: false,
        is_missing: false,
    }
}

pub(crate) fn query_step__add_capture(step: &mut QueryStep, capture_id: u16) {
    if let Some(slot) = step.capture_ids.iter_mut().find(|id| **id == NONE) {
        *slot = capture_id;
    }
}

pub(crate) fn query_step__remove_capture(step: &mut QueryStep, capture_id: u16) {
    for mut i in 0..MAX_STEP_CAPTURE_COUNT {
        if step.capture_ids[i] == capture_id {
            step.capture_ids[i] = NONE;
            while i + 1 < MAX_STEP_CAPTURE_COUNT {
                if step.capture_ids[i + 1] == NONE {
                    break;
                }
                step.capture_ids[i] = step.capture_ids[i + 1];
                step.capture_ids[i + 1] = NONE;
                i += 1;
            }
            break;
        }
    }
}

pub(crate) fn state_predecessor_map_new(language: &Language) -> StatePredecessorMap {
    StatePredecessorMap {
        contents: vec![0; language.tables.state_count as usize * (MAX_STATE_PREDECESSOR_COUNT + 1)],
    }
}

pub(crate) fn state_predecessor_map_delete(map: StatePredecessorMap) {
    drop(map);
}

pub(crate) fn state_predecessor_map_add(
    map: &mut StatePredecessorMap,
    state: StateId,
    predecessor: StateId,
) {
    let index = usize::from(state) * (MAX_STATE_PREDECESSOR_COUNT + 1);
    let count = usize::from(map.contents[index]);
    // Only consecutive duplicates are suppressed, not all repeated states.
    if count == 0
        || (count < MAX_STATE_PREDECESSOR_COUNT && map.contents[index + count] != predecessor)
    {
        map.contents[index] += 1;
        map.contents[index + count + 1] = predecessor;
    }
}

pub(crate) fn state_predecessor_map_get(map: &StatePredecessorMap, state: StateId) -> &[StateId] {
    let index = usize::from(state) * (MAX_STATE_PREDECESSOR_COUNT + 1);
    let count = usize::from(map.contents[index]);
    &map.contents[index + 1..index + 1 + count]
}

pub(crate) fn analysis_state__recursion_depth(state: &AnalysisState) -> u32 {
    let mut result = 0;
    for i in 0..usize::from(state.depth) {
        let symbol = state.stack[i].parent_symbol;
        if state.stack[..i]
            .iter()
            .any(|entry| entry.parent_symbol == symbol)
        {
            result += 1;
        }
    }
    result
}

pub(crate) fn analysis_state__compare(left: &AnalysisState, right: &AnalysisState) -> i32 {
    // Preserve C's asymmetric depth handling: a shallower left state always
    // sorts later, but a deeper left state compares the common prefix first.
    if left.depth < right.depth {
        return 1;
    }
    for i in 0..usize::from(left.depth) {
        if i >= usize::from(right.depth) {
            return -1;
        }
        let s1 = left.stack[i];
        let s2 = right.stack[i];
        let comparison = (
            s1.child_index,
            s1.parent_symbol,
            s1.parse_state,
            s1.field_id,
        )
            .cmp(&(
                s2.child_index,
                s2.parent_symbol,
                s2.parse_state,
                s2.field_id,
            ));
        if comparison != std::cmp::Ordering::Equal {
            return comparison as i32;
        }
    }
    // Entry.done and root_symbol deliberately do not participate in ordering.
    left.step_index.cmp(&right.step_index) as i32
}

pub(crate) fn analysis_state__top(state: &mut AnalysisState) -> &mut AnalysisStateEntry {
    let index = if state.depth == 0 { 0 } else { state.depth - 1 };
    &mut state.stack[usize::from(index)]
}

pub(crate) fn analysis_state__has_supertype(state: &AnalysisState, symbol: Symbol) -> bool {
    state.stack[..usize::from(state.depth)]
        .iter()
        .any(|entry| entry.parent_symbol == symbol)
}

pub(crate) fn analysis_state_pool__clone_or_reuse(
    pool: &mut AnalysisStateSet,
    borrowed_item: &AnalysisState,
) -> AnalysisState {
    // States are inline Copy values instead of separately allocated objects.
    // Still consume one pool entry, preserving the C pool's order and size.
    pool.pop();
    *borrowed_item
}

pub(crate) fn analysis_state_set__insert_sorted(
    set: &mut AnalysisStateSet,
    pool: &mut AnalysisStateSet,
    borrowed_item: &AnalysisState,
) {
    // Use array_search_sorted_with's exact comparisons. The C comparator's
    // depth handling is not a total order, so a generic binary search can
    // choose a different insertion position.
    let mut index = 0;
    let mut size = set.len();
    if size > 0 {
        while size > 1 {
            let half_size = size / 2;
            let mid_index = index + half_size;
            if analysis_state__compare(&set[mid_index], borrowed_item) <= 0 {
                index = mid_index;
            }
            size -= half_size;
        }
        let comparison = analysis_state__compare(&set[index], borrowed_item);
        if comparison == 0 {
            return;
        }
        if comparison < 0 {
            index += 1;
        }
    }
    let new_item = analysis_state_pool__clone_or_reuse(pool, borrowed_item);
    set.insert(index, new_item);
}

pub(crate) fn analysis_state_set__push(
    set: &mut AnalysisStateSet,
    pool: &mut AnalysisStateSet,
    borrowed_item: &AnalysisState,
) {
    let new_item = analysis_state_pool__clone_or_reuse(pool, borrowed_item);
    set.push(new_item);
}

pub(crate) fn analysis_state_set__clear(set: &mut AnalysisStateSet, pool: &mut AnalysisStateSet) {
    pool.append(set);
}

pub(crate) fn analysis_state_set__delete(set: AnalysisStateSet) {
    drop(set);
}

pub(crate) fn query_analysis__new() -> QueryAnalysis {
    QueryAnalysis::default()
}

pub(crate) fn query_analysis__delete(analysis: QueryAnalysis) {
    drop(analysis);
}

pub(crate) fn analysis_subgraph_node__compare(
    left: &AnalysisSubgraphNode,
    right: &AnalysisSubgraphNode,
) -> i32 {
    (left.state, left.child_index, left.done, left.production_id).cmp(&(
        right.state,
        right.child_index,
        right.done,
        right.production_id,
    )) as i32
}

pub(crate) fn ts_query__pattern_map_search(query: &CompiledQuery, needle: Symbol) -> (bool, u32) {
    let mut base_index = u32::from(query.wildcard_root_pattern_count);
    let mut size = query.pattern_map.len() as u32 - base_index;
    if size == 0 {
        return (false, base_index);
    }
    while size > 1 {
        let half_size = size / 2;
        let mid_index = base_index + half_size;
        let mid_symbol =
            query.steps[usize::from(query.pattern_map[mid_index as usize].step_index)].symbol;
        if needle > mid_symbol {
            base_index = mid_index;
        }
        size -= half_size;
    }

    let mut symbol =
        query.steps[usize::from(query.pattern_map[base_index as usize].step_index)].symbol;
    if needle > symbol {
        base_index += 1;
        if base_index < query.pattern_map.len() as u32 {
            symbol =
                query.steps[usize::from(query.pattern_map[base_index as usize].step_index)].symbol;
        }
    }
    (needle == symbol, base_index)
}

#[cfg(test)]
mod tests {
    use super::*;
    use CaptureQuantifier::{One, OneOrMore, Zero, ZeroOrMore, ZeroOrOne};

    #[test]
    fn stream_decoding_offsets_comments_and_c_locale() {
        let mut stream = stream_new(b" \x0b;comment\nname-1.?! \xc3\xa9\xe2\x82!");
        stream_skip_whitespace(&mut stream);
        assert_eq!(stream_offset(&stream), 11);
        assert!(stream_is_ident_start(&stream));
        stream_scan_identifier(&mut stream);
        assert_eq!(stream_offset(&stream), 20);
        stream_skip_whitespace(&mut stream);
        assert_eq!(stream.next, 0xe9);
        assert!(!stream_is_ident_start(&stream));
        assert!(stream_advance(&mut stream));
        assert_eq!((stream.next, stream.next_size), (-1, 2));
        assert!(stream_advance(&mut stream));
        assert_eq!(stream.next, i32::from(b'!'));
        assert!(!stream_advance(&mut stream));
        assert_eq!((stream.next, stream.next_size), (0, 0));
        stream_reset(&mut stream, 11);
        assert_eq!(stream.next, i32::from(b'n'));
        let mut non_ascii_space = stream_new("\u{a0}x".as_bytes());
        stream_skip_whitespace(&mut non_ascii_space);
        assert_eq!(stream_offset(&non_ascii_space), 0);
    }

    #[test]
    fn quantifier_tables_cover_all_pairs() {
        let values = [Zero, ZeroOrOne, ZeroOrMore, One, OneOrMore];
        let multiplication = [
            [Zero, Zero, Zero, Zero, Zero],
            [Zero, ZeroOrOne, ZeroOrMore, ZeroOrOne, ZeroOrMore],
            [Zero, ZeroOrMore, ZeroOrMore, ZeroOrMore, ZeroOrMore],
            [Zero, ZeroOrOne, ZeroOrMore, One, OneOrMore],
            [Zero, ZeroOrMore, ZeroOrMore, OneOrMore, OneOrMore],
        ];
        let join = [
            [Zero, ZeroOrOne, ZeroOrMore, ZeroOrOne, ZeroOrMore],
            [ZeroOrOne, ZeroOrOne, ZeroOrMore, ZeroOrOne, ZeroOrMore],
            [ZeroOrMore, ZeroOrMore, ZeroOrMore, ZeroOrMore, ZeroOrMore],
            [ZeroOrOne, ZeroOrOne, ZeroOrMore, One, OneOrMore],
            [ZeroOrMore, ZeroOrMore, ZeroOrMore, OneOrMore, OneOrMore],
        ];
        let addition = [
            [Zero, ZeroOrOne, ZeroOrMore, One, OneOrMore],
            [ZeroOrOne, ZeroOrMore, ZeroOrMore, OneOrMore, OneOrMore],
            [ZeroOrMore, ZeroOrMore, ZeroOrMore, OneOrMore, OneOrMore],
            [One, OneOrMore, OneOrMore, OneOrMore, OneOrMore],
            [OneOrMore, OneOrMore, OneOrMore, OneOrMore, OneOrMore],
        ];
        for (i, &left) in values.iter().enumerate() {
            for (j, &right) in values.iter().enumerate() {
                assert_eq!(quantifier_mul(left, right), multiplication[i][j]);
                assert_eq!(quantifier_join(left, right), join[i][j]);
                assert_eq!(quantifier_add(left, right), addition[i][j]);
            }
        }
    }

    #[test]
    fn capture_quantifiers_handle_absent_entries_and_truncated_loop_bounds() {
        let mut target = capture_quantifiers_new();
        capture_quantifiers_add_for_id(&mut target, 2, One);
        assert_eq!(target, [Zero, Zero, One]);
        assert_eq!(capture_quantifier_for_id(&target, 3), Zero);
        capture_quantifiers_add_all(&mut target, &vec![One, ZeroOrOne]);
        assert_eq!(target, [One, ZeroOrOne, One]);
        capture_quantifiers_join_all(&mut target, &vec![One]);
        assert_eq!(target, [One, ZeroOrOne, ZeroOrOne]);
        capture_quantifiers_join_all(&mut target, &vec![One, One, One, One]);
        assert_eq!(target, [One, ZeroOrOne, ZeroOrOne, ZeroOrOne]);
        capture_quantifiers_replace(&mut target, &vec![One; 65536]);
        capture_quantifiers_mul(&mut target, Zero);
        assert!(target.iter().all(|q| *q == One));
        capture_quantifiers_add_all(&mut target, &vec![One; 65536]);
        assert!(target.iter().all(|q| *q == One));
        capture_quantifiers_clear(&mut target);
        assert!(target.is_empty());
    }

    #[test]
    fn symbol_table_preserves_bytes_order_and_strncmp_semantics() {
        let mut table = symbol_table_new();
        assert_eq!(symbol_table_insert_name(&mut table, b"abc"), 0);
        assert_eq!(symbol_table_insert_name(&mut table, b"a\0x"), 1);
        assert_eq!(symbol_table_insert_name(&mut table, b"a\0y"), 1);
        assert_eq!(symbol_table_insert_name(&mut table, b"a\0yz"), 2);
        assert_eq!(symbol_table_insert_name(&mut table, b""), 3);
        assert_eq!(symbol_table_name_for_id(&table, 1), b"a\0x");
        assert_eq!(symbol_table_name_for_id(&table, 3), b"");
        assert_eq!(table.characters, b"abc\0a\0x\0a\0yz\0\0");
        assert_eq!(symbol_table_id_for_name(&table, b"not present"), -1);
    }

    #[test]
    fn capture_pool_reuses_lowest_free_slot_even_after_lowering_limit() {
        let mut pool = capture_list_pool_new();
        pool.max_capture_list_count = 2;
        assert_eq!(capture_list_pool_acquire(&mut pool), 0);
        assert_eq!(capture_list_pool_acquire(&mut pool), 1);
        assert!(capture_list_pool_is_empty(&pool));
        assert_eq!(capture_list_pool_acquire(&mut pool), NONE);
        capture_list_pool_release(&mut pool, 0);
        pool.max_capture_list_count = 0;
        assert!(!capture_list_pool_is_empty(&pool));
        assert_eq!(capture_list_pool_acquire(&mut pool), 0);
        capture_list_pool_reset(&mut pool);
        assert_eq!(pool.free_capture_list_count, 2);
        assert_eq!(capture_list_pool_acquire(&mut pool), 0);
        assert_eq!(capture_list_pool_acquire(&mut pool), 1);
        assert!(capture_list_pool_get(&pool, NONE).is_empty());
    }

    #[test]
    fn capture_pool_retains_c_u16_limits() {
        let mut pool = capture_list_pool_new();
        for _ in 0..65536 {
            pool.list.push(CaptureListSlot {
                captures: Vec::new(),
                in_use: true,
            });
        }
        capture_list_pool_reset(&mut pool);
        assert_eq!(pool.free_capture_list_count, 65536);
        assert!(pool.list.iter().all(|slot| slot.in_use));
        capture_list_pool_release(&mut pool, 0);
        assert_eq!(capture_list_pool_acquire(&mut pool), 0);
        // The bound truncated to zero, so the free slot was not reused.
        assert_eq!(pool.list.len(), 65537);
        assert!(!pool.list[0].in_use);
        assert_eq!(capture_list_pool_acquire(&mut pool), 0);
        assert!(pool.list[0].in_use);
    }

    #[test]
    fn capture_slots_limit_and_remove_only_first_occurrence() {
        let mut step = query_step__new(17, 2, true);
        assert_eq!(step.capture_ids, [NONE; 3]);
        for id in [1, 1, 2, 3] {
            query_step__add_capture(&mut step, id);
        }
        assert_eq!(step.capture_ids, [1, 1, 2]);
        query_step__remove_capture(&mut step, 1);
        assert_eq!(step.capture_ids, [1, 2, NONE]);
        query_step__remove_capture(&mut step, 7);
        assert_eq!(step.capture_ids, [1, 2, NONE]);
    }

    #[test]
    fn predecessors_deduplicate_only_adjacent_values_and_stop_at_limit() {
        let mut map = StatePredecessorMap {
            contents: vec![0; 2 * 257],
        };
        for predecessor in [4, 4, 5, 4] {
            state_predecessor_map_add(&mut map, 1, predecessor);
        }
        assert_eq!(state_predecessor_map_get(&map, 1), [4, 5, 4]);
        for predecessor in 0..300 {
            state_predecessor_map_add(&mut map, 1, predecessor);
        }
        assert_eq!(state_predecessor_map_get(&map, 1).len(), 256);
        assert!(state_predecessor_map_get(&map, 0).is_empty());
    }

    #[test]
    fn analysis_order_ignores_done_and_root_and_preserves_depth_asymmetry() {
        let mut shallow = AnalysisState {
            depth: 1,
            ..AnalysisState::default()
        };
        let mut deep = AnalysisState {
            depth: 2,
            ..AnalysisState::default()
        };
        deep.stack[0].child_index = 1;
        assert_eq!(analysis_state__compare(&shallow, &deep), 1);
        assert_eq!(analysis_state__compare(&deep, &shallow), 1);
        deep.stack[0].child_index = 0;
        assert_eq!(analysis_state__compare(&deep, &shallow), -1);
        let equal = shallow;
        shallow.root_symbol = 123;
        shallow.stack[0].done = true;
        assert_eq!(analysis_state__compare(&shallow, &equal), 0);
        let mut set = Vec::new();
        let mut pool = vec![AnalysisState::default(); 2];
        analysis_state_set__insert_sorted(&mut set, &mut pool, &shallow);
        analysis_state_set__insert_sorted(&mut set, &mut pool, &equal);
        assert_eq!((set.len(), pool.len()), (1, 1));
        analysis_state_set__insert_sorted(&mut set, &mut pool, &deep);
        assert_eq!(set[0].depth, 2);
        analysis_state_set__clear(&mut set, &mut pool);
        assert!(set.is_empty());
        assert_eq!(pool.len(), 2);
        assert_eq!(analysis_state__recursion_depth(&deep), 1);
        assert!(analysis_state__has_supertype(&deep, 0));
        assert!(!analysis_state__has_supertype(&deep, 1));
        let mut empty = AnalysisState::default();
        analysis_state__top(&mut empty).parse_state = 7;
        assert_eq!(empty.stack[0].parse_state, 7);
    }
}
