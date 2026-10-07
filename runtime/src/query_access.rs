//! query-4: see query.c and PORTING.md.
use super::*;

pub(crate) fn ts_query_new(
    language: &Language,
    source: &[u8],
) -> Result<CompiledQuery, QueryCompileError> {
    todo!("query-4: ts_query_new")
}

pub(crate) fn ts_query_delete(query: CompiledQuery) {
    todo!("query-4: ts_query_delete")
}

pub(crate) fn ts_query_pattern_count(query: &CompiledQuery) -> u32 {
    todo!("query-4: ts_query_pattern_count")
}

pub(crate) fn ts_query_capture_count(query: &CompiledQuery) -> u32 {
    todo!("query-4: ts_query_capture_count")
}

pub(crate) fn ts_query_string_count(query: &CompiledQuery) -> u32 {
    todo!("query-4: ts_query_string_count")
}

pub(crate) fn ts_query_capture_name_for_id(query: &CompiledQuery, index: u32) -> &[u8] {
    todo!("query-4: ts_query_capture_name_for_id")
}

pub(crate) fn ts_query_capture_quantifier_for_id(
    query: &CompiledQuery,
    pattern_index: u32,
    capture_index: u32,
) -> CaptureQuantifier {
    todo!("query-4: ts_query_capture_quantifier_for_id")
}

pub(crate) fn ts_query_string_value_for_id(query: &CompiledQuery, index: u32) -> &[u8] {
    todo!("query-4: ts_query_string_value_for_id")
}

pub(crate) fn ts_query_predicates_for_pattern(
    query: &CompiledQuery,
    pattern_index: u32,
) -> &[PredicateStep] {
    todo!("query-4: ts_query_predicates_for_pattern")
}

pub(crate) fn ts_query_start_byte_for_pattern(query: &CompiledQuery, pattern_index: u32) -> u32 {
    todo!("query-4: ts_query_start_byte_for_pattern")
}

pub(crate) fn ts_query_end_byte_for_pattern(query: &CompiledQuery, pattern_index: u32) -> u32 {
    todo!("query-4: ts_query_end_byte_for_pattern")
}

pub(crate) fn ts_query_is_pattern_rooted(query: &CompiledQuery, pattern_index: u32) -> bool {
    todo!("query-4: ts_query_is_pattern_rooted")
}

pub(crate) fn ts_query_is_pattern_non_local(query: &CompiledQuery, pattern_index: u32) -> bool {
    todo!("query-4: ts_query_is_pattern_non_local")
}

pub(crate) fn ts_query_is_pattern_guaranteed_at_step(
    query: &CompiledQuery,
    byte_offset: u32,
) -> bool {
    todo!("query-4: ts_query_is_pattern_guaranteed_at_step")
}

pub(crate) fn ts_query__step_is_fallible(query: &CompiledQuery, step_index: u16) -> bool {
    todo!("query-4: ts_query__step_is_fallible")
}

pub(crate) fn ts_query_disable_capture(query: &mut CompiledQuery, name: &[u8]) {
    todo!("query-4: ts_query_disable_capture")
}

pub(crate) fn ts_query_disable_pattern(query: &mut CompiledQuery, pattern_index: u32) {
    todo!("query-4: ts_query_disable_pattern")
}

pub(crate) fn ts_query_cursor_new() -> CursorConfig {
    todo!("query-4: ts_query_cursor_new")
}

pub(crate) fn ts_query_cursor_delete(cursor: CursorConfig) {
    todo!("query-4: ts_query_cursor_delete")
}

pub(crate) fn ts_query_cursor_did_exceed_match_limit(cursor: &CursorConfig) -> bool {
    todo!("query-4: ts_query_cursor_did_exceed_match_limit")
}

pub(crate) fn ts_query_cursor_match_limit(cursor: &CursorConfig) -> u32 {
    todo!("query-4: ts_query_cursor_match_limit")
}

pub(crate) fn ts_query_cursor_set_match_limit(cursor: &mut CursorConfig, limit: u32) {
    todo!("query-4: ts_query_cursor_set_match_limit")
}

pub(crate) fn ts_query_cursor_timeout_micros(cursor: &CursorConfig) -> u64 {
    todo!("query-4: ts_query_cursor_timeout_micros")
}

pub(crate) fn ts_query_cursor_set_timeout_micros(cursor: &mut CursorConfig, timeout_micros: u64) {
    todo!("query-4: ts_query_cursor_set_timeout_micros")
}

pub(crate) fn ts_query_cursor_exec<'query, 'tree: 'query>(
    cursor: &'query mut CursorConfig,
    query: &'query CompiledQuery,
    node: Node<'tree>,
) -> QueryExecution<'query, 'tree> {
    todo!("query-4: ts_query_cursor_exec")
}

pub(crate) fn ts_query_cursor_exec_with_options<'query, 'tree: 'query>(
    cursor: &'query mut CursorConfig,
    query: &'query CompiledQuery,
    node: Node<'tree>,
    options: QueryCursorOptions<'query>,
) -> QueryExecution<'query, 'tree> {
    todo!("query-4: ts_query_cursor_exec_with_options")
}

pub(crate) fn ts_query_cursor_set_byte_range(
    cursor: &mut CursorConfig,
    start_byte: u32,
    end_byte: u32,
) -> bool {
    todo!("query-4: ts_query_cursor_set_byte_range")
}

pub(crate) fn ts_query_cursor_set_point_range(
    cursor: &mut CursorConfig,
    start_point: Point,
    end_point: Point,
) -> bool {
    todo!("query-4: ts_query_cursor_set_point_range")
}

pub(crate) fn ts_query_cursor__first_in_progress_capture(
    cursor: &QueryExecution<'_, '_>,
) -> Option<InProgressCapture> {
    todo!("query-4: ts_query_cursor__first_in_progress_capture")
}

pub(crate) fn ts_query_cursor__compare_nodes(left: Node<'_>, right: Node<'_>) -> i32 {
    todo!("query-4: ts_query_cursor__compare_nodes")
}

pub(crate) fn ts_query_cursor__compare_captures(
    cursor: &QueryExecution<'_, '_>,
    left_state: &QueryState,
    right_state: &QueryState,
) -> (bool, bool) {
    todo!("query-4: ts_query_cursor__compare_captures")
}

pub(crate) fn ts_query_cursor__add_state(
    cursor: &mut QueryExecution<'_, '_>,
    pattern: &PatternEntry,
) {
    todo!("query-4: ts_query_cursor__add_state")
}

// This is the execution's only capture_list_pool_acquire call site. Record the
// pool's full slot count in config.allocated_capture_list_count immediately after
// acquisition, even when a returned u16 id is NONE or truncated. Do not defer
// persistence to a destructor: a new execution can start after the old iterator's
// last use but before its lexical scope ends.
pub(crate) fn ts_query_cursor__prepare_to_capture<'a, 'query, 'tree: 'query>(
    cursor: &'a mut QueryExecution<'query, 'tree>,
    state: &mut QueryState,
    state_index_to_preserve: u32,
) -> Option<&'a mut CaptureList<'tree>> {
    todo!("query-4: ts_query_cursor__prepare_to_capture")
}
