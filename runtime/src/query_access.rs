//! Query construction/accessors and cursor setup/state preparation (query.c).
use super::*;
use crate::{
    clock::{clock_after, clock_now, clock_null, duration_from_micros, duration_to_micros},
    node::{ts_node_end_byte, ts_node_end_point, ts_node_start_byte},
    point::{POINT_MAX, POINT_ZERO},
    tree_cursor::ts_tree_cursor_new,
};

pub(crate) fn ts_query_new(
    language: &Language,
    source: &[u8],
) -> Result<CompiledQuery, QueryCompileError> {
    if language.tables.abi_version > crate::api::LANGUAGE_VERSION as u32
        || language.tables.abi_version < crate::api::MIN_COMPATIBLE_LANGUAGE_VERSION as u32
    {
        return Err(QueryCompileError {
            offset: 0,
            kind: QueryErrorCode::Language,
        });
    }

    let mut query = CompiledQuery {
        captures: symbol_table_new(),
        predicate_values: symbol_table_new(),
        capture_quantifiers: Vec::new(),
        steps: Vec::new(),
        pattern_map: Vec::new(),
        predicate_steps: Vec::new(),
        patterns: Vec::new(),
        step_offsets: Vec::new(),
        negated_fields: vec![0],
        string_buffer: Vec::new(),
        repeat_symbols_with_rootless_patterns: Vec::new(),
        language: *language,
        wildcard_root_pattern_count: 0,
    };

    // The binding passes the source length to C as a u32.
    let mut stream = stream_new(&source[..source.len() as u32 as usize]);
    stream_skip_whitespace(&mut stream);
    while stream.input < stream.source.len() {
        let pattern_index = query.patterns.len() as u32;
        let mut start_step_index = query.steps.len() as u32;
        let start_predicate_step_index = query.predicate_steps.len() as u32;
        query.patterns.push(QueryPattern {
            steps: Slice {
                offset: start_step_index,
                length: 0,
            },
            predicate_steps: Slice {
                offset: start_predicate_step_index,
                length: 0,
            },
            start_byte: stream_offset(&stream),
            end_byte: 0,
            is_non_local: false,
        });
        let mut capture_quantifiers = capture_quantifiers_new();
        let error =
            ts_query__parse_pattern(&mut query, &mut stream, 0, false, &mut capture_quantifiers);
        query
            .steps
            .push(query_step__new(0, PATTERN_DONE_MARKER, false));

        let pattern = query.patterns.last_mut().unwrap();
        pattern.steps.length = (query.steps.len() as u32).wrapping_sub(start_step_index);
        pattern.predicate_steps.length =
            (query.predicate_steps.len() as u32).wrapping_sub(start_predicate_step_index);
        pattern.end_byte = stream_offset(&stream);

        if error != QueryErrorCode::None {
            return Err(QueryCompileError {
                offset: stream_offset(&stream),
                kind: if error == PARENT_DONE {
                    QueryErrorCode::Syntax
                } else {
                    error
                },
            });
        }
        query.capture_quantifiers.push(capture_quantifiers);

        let mut wildcard_root_alternative_index = NONE;
        loop {
            let mut step = query.steps[start_step_index as usize];

            // Start at a concrete child of a wildcard root when possible. The
            // execution checks/captures the skipped parent via needs_parent.
            if step.symbol == WILDCARD_SYMBOL && step.depth == 0 && step.field == 0 {
                let second_step = query.steps[start_step_index as usize + 1];
                if second_step.symbol != WILDCARD_SYMBOL
                    && second_step.depth == 1
                    && !second_step.is_immediate
                {
                    wildcard_root_alternative_index = step.alternative_index;
                    start_step_index = start_step_index.wrapping_add(1);
                    step = second_step;
                }
            }

            let start_depth = step.depth;
            let mut is_rooted = start_depth == 0;
            for child_step in &query.steps[start_step_index as usize + 1..] {
                if child_step.is_dead_end {
                    break;
                }
                if child_step.depth == start_depth {
                    is_rooted = false;
                    break;
                }
            }
            ts_query__pattern_map_insert(
                &mut query,
                step.symbol,
                PatternEntry {
                    step_index: start_step_index as u16,
                    pattern_index: pattern_index as u16,
                    is_rooted,
                },
            );
            if step.symbol == WILDCARD_SYMBOL {
                query.wildcard_root_pattern_count =
                    query.wildcard_root_pattern_count.wrapping_add(1);
            }

            if step.alternative_index != NONE {
                start_step_index = u32::from(step.alternative_index);
            } else if wildcard_root_alternative_index != NONE {
                start_step_index = u32::from(wildcard_root_alternative_index);
                wildcard_root_alternative_index = NONE;
            } else {
                break;
            }
        }
    }

    ts_query__analyze_patterns(&mut query).map_err(|offset| QueryCompileError {
        offset,
        kind: QueryErrorCode::Structure,
    })?;
    query.string_buffer = Vec::new();
    Ok(query)
}

pub(crate) fn ts_query_delete(query: CompiledQuery) {
    drop(query);
}

pub(crate) fn ts_query_pattern_count(query: &CompiledQuery) -> u32 {
    query.patterns.len() as u32
}

pub(crate) fn ts_query_capture_count(query: &CompiledQuery) -> u32 {
    query.captures.slices.len() as u32
}

pub(crate) fn ts_query_string_count(query: &CompiledQuery) -> u32 {
    query.predicate_values.slices.len() as u32
}

pub(crate) fn ts_query_capture_name_for_id(query: &CompiledQuery, index: u32) -> &[u8] {
    symbol_table_name_for_id(&query.captures, index as u16)
}

pub(crate) fn ts_query_capture_quantifier_for_id(
    query: &CompiledQuery,
    pattern_index: u32,
    capture_index: u32,
) -> CaptureQuantifier {
    capture_quantifier_for_id(
        &query.capture_quantifiers[pattern_index as usize],
        capture_index as u16,
    )
}

pub(crate) fn ts_query_string_value_for_id(query: &CompiledQuery, index: u32) -> &[u8] {
    symbol_table_name_for_id(&query.predicate_values, index as u16)
}

pub(crate) fn ts_query_predicates_for_pattern(
    query: &CompiledQuery,
    pattern_index: u32,
) -> &[PredicateStep] {
    let slice = query.patterns[pattern_index as usize].predicate_steps;
    if slice.length == 0 {
        return &[];
    }
    &query.predicate_steps[slice.offset as usize..slice.offset as usize + slice.length as usize]
}

pub(crate) fn ts_query_start_byte_for_pattern(query: &CompiledQuery, pattern_index: u32) -> u32 {
    query.patterns[pattern_index as usize].start_byte
}

pub(crate) fn ts_query_end_byte_for_pattern(query: &CompiledQuery, pattern_index: u32) -> u32 {
    query.patterns[pattern_index as usize].end_byte
}

pub(crate) fn ts_query_is_pattern_rooted(query: &CompiledQuery, pattern_index: u32) -> bool {
    query
        .pattern_map
        .iter()
        .all(|entry| u32::from(entry.pattern_index) != pattern_index || entry.is_rooted)
}

pub(crate) fn ts_query_is_pattern_non_local(query: &CompiledQuery, pattern_index: u32) -> bool {
    query
        .patterns
        .get(pattern_index as usize)
        .is_some_and(|pattern| pattern.is_non_local)
}

pub(crate) fn ts_query_is_pattern_guaranteed_at_step(
    query: &CompiledQuery,
    byte_offset: u32,
) -> bool {
    let mut step_index = u32::MAX;
    for offset in &query.step_offsets {
        if offset.byte_offset > byte_offset {
            break;
        }
        step_index = u32::from(offset.step_index);
    }
    query
        .steps
        .get(step_index as usize)
        .is_some_and(|step| step.root_pattern_guaranteed)
}

pub(crate) fn ts_query__step_is_fallible(query: &CompiledQuery, step_index: u16) -> bool {
    assert!(usize::from(step_index) + 1 < query.steps.len());
    let step = &query.steps[usize::from(step_index)];
    let next_step = &query.steps[usize::from(step_index) + 1];
    next_step.depth != PATTERN_DONE_MARKER
        && next_step.depth > step.depth
        && (!next_step.parent_pattern_guaranteed || step.symbol == WILDCARD_SYMBOL)
}

pub(crate) fn ts_query_disable_capture(query: &mut CompiledQuery, name: &[u8]) {
    let id = symbol_table_id_for_name(&query.captures, name);
    if id != -1 {
        for step in &mut query.steps {
            query_step__remove_capture(step, id as u16);
        }
    }
}

pub(crate) fn ts_query_disable_pattern(query: &mut CompiledQuery, pattern_index: u32) {
    // C removes only map entries, retaining steps and the wildcard-root count.
    query
        .pattern_map
        .retain(|entry| u32::from(entry.pattern_index) != pattern_index);
}

pub(crate) fn ts_query_cursor_new() -> CursorConfig {
    CursorConfig {
        max_capture_list_count: u32::MAX,
        max_start_depth: u32::MAX,
        start_byte: 0,
        end_byte: u32::MAX,
        start_point: POINT_ZERO,
        end_point: POINT_MAX,
        timeout_duration: 0,
        did_exceed_match_limit: false,
        allocated_capture_list_count: 0,
        states: Vec::with_capacity(8),
        finished_states: Vec::with_capacity(8),
    }
}

pub(crate) fn ts_query_cursor_delete(cursor: CursorConfig) {
    drop(cursor);
}

pub(crate) fn ts_query_cursor_did_exceed_match_limit(cursor: &CursorConfig) -> bool {
    cursor.did_exceed_match_limit
}

pub(crate) fn ts_query_cursor_match_limit(cursor: &CursorConfig) -> u32 {
    cursor.max_capture_list_count
}

pub(crate) fn ts_query_cursor_set_match_limit(cursor: &mut CursorConfig, limit: u32) {
    cursor.max_capture_list_count = limit;
}

pub(crate) fn ts_query_cursor_timeout_micros(cursor: &CursorConfig) -> u64 {
    duration_to_micros(cursor.timeout_duration)
}

pub(crate) fn ts_query_cursor_set_timeout_micros(cursor: &mut CursorConfig, timeout_micros: u64) {
    cursor.timeout_duration = duration_from_micros(timeout_micros);
}

pub(crate) fn ts_query_cursor_exec<'query, 'tree: 'query>(
    cursor: &'query mut CursorConfig,
    query: &'query CompiledQuery,
    node: Node<'tree>,
) -> QueryExecution<'query, 'tree> {
    cursor.states.clear();
    cursor.finished_states.clear();
    let tree_cursor = ts_tree_cursor_new(node);
    cursor.did_exceed_match_limit = false;

    // Keep pool slot ids/count across executions without retaining borrowed
    // nodes in the public cursor. Buffers themselves are execution-local.
    let mut capture_list_pool = capture_list_pool_new();
    capture_list_pool.max_capture_list_count = cursor.max_capture_list_count;
    capture_list_pool
        .list
        .resize_with(cursor.allocated_capture_list_count as usize, || {
            CaptureListSlot {
                captures: Vec::new(),
                in_use: false,
            }
        });
    capture_list_pool_reset(&mut capture_list_pool);

    let end_clock = if cursor.timeout_duration != 0 {
        clock_after(clock_now(), cursor.timeout_duration)
    } else {
        clock_null()
    };
    QueryExecution {
        config: cursor,
        query,
        cursor: tree_cursor,
        capture_list_pool,
        depth: 0,
        next_state_id: 0,
        end_clock,
        query_options: QueryCursorOptions::default(),
        query_state: QueryCursorState::default(),
        operation_count: 0,
        on_visible_node: true,
        ascending: false,
        halted: false,
    }
}

pub(crate) fn ts_query_cursor_exec_with_options<'query, 'tree: 'query>(
    cursor: &'query mut CursorConfig,
    query: &'query CompiledQuery,
    node: Node<'tree>,
    options: QueryCursorOptions<'query>,
) -> QueryExecution<'query, 'tree> {
    let mut execution = ts_query_cursor_exec(cursor, query, node);
    execution.query_options = options;
    execution
}

pub(crate) fn ts_query_cursor_set_byte_range(
    cursor: &mut CursorConfig,
    start_byte: u32,
    end_byte: u32,
) -> bool {
    let end_byte = if end_byte == 0 { u32::MAX } else { end_byte };
    if start_byte > end_byte {
        return false;
    }
    cursor.start_byte = start_byte;
    cursor.end_byte = end_byte;
    true
}

pub(crate) fn ts_query_cursor_set_point_range(
    cursor: &mut CursorConfig,
    start_point: Point,
    end_point: Point,
) -> bool {
    let end_point = if end_point == POINT_ZERO {
        POINT_MAX
    } else {
        end_point
    };
    if start_point > end_point {
        return false;
    }
    cursor.start_point = start_point;
    cursor.end_point = end_point;
    true
}

pub(crate) fn ts_query_cursor__first_in_progress_capture(
    cursor: &mut QueryExecution<'_, '_>,
) -> Option<InProgressCapture> {
    first_in_progress_capture(cursor, true)
}

// `include_definite == false` corresponds to C's NULL is_definite out-parameter:
// guaranteed states are not eviction candidates. Both modes consume captures
// that end at/before the range start, and therefore require a mutable execution.
fn first_in_progress_capture(
    cursor: &mut QueryExecution<'_, '_>,
    include_definite: bool,
) -> Option<InProgressCapture> {
    let mut result: Option<InProgressCapture> = None;
    for (i, state) in cursor.config.states.iter_mut().enumerate() {
        if state.dead {
            continue;
        }
        let captures =
            capture_list_pool_get(&cursor.capture_list_pool, state.capture_list_id as u16);
        let node = loop {
            let Some(capture) = captures.get(usize::from(state.consumed_capture_count)) else {
                break None;
            };
            let node = capture.node;
            if ts_node_end_byte(node) <= cursor.config.start_byte
                || ts_node_end_point(node) <= cursor.config.start_point
            {
                state.consumed_capture_count = state.consumed_capture_count.wrapping_add(1) & 0xfff;
            } else {
                break Some(node);
            }
        };
        let Some(node) = node else { continue };
        let node_start_byte = ts_node_start_byte(node);
        if result.is_none_or(|previous| {
            node_start_byte < previous.byte_offset
                || (node_start_byte == previous.byte_offset
                    && u32::from(state.pattern_index) < previous.pattern_index)
        }) {
            let step = &cursor.query.steps[usize::from(state.step_index)];
            if !include_definite && step.root_pattern_guaranteed {
                continue;
            }
            result = Some(InProgressCapture {
                state_index: i as u32,
                byte_offset: node_start_byte,
                pattern_index: u32::from(state.pattern_index),
                is_definite: step.root_pattern_guaranteed && !step.is_immediate,
            });
        }
    }
    result
}

pub(crate) fn ts_query_cursor__compare_nodes(left: Node<'_>, right: Node<'_>) -> i32 {
    // This is C's .id comparison, not node equality (which includes context).
    if !std::ptr::eq(left.subtree, right.subtree) {
        let left_start = ts_node_start_byte(left);
        let right_start = ts_node_start_byte(right);
        if left_start < right_start {
            return -1;
        }
        if left_start > right_start {
            return 1;
        }
        let left_end = ts_node_end_byte(left);
        let right_end = ts_node_end_byte(right);
        if left_end > right_end {
            return -1;
        }
        if left_end < right_end {
            return 1;
        }
    }
    0
}

pub(crate) fn ts_query_cursor__compare_captures(
    cursor: &QueryExecution<'_, '_>,
    left_state: &QueryState,
    right_state: &QueryState,
) -> (bool, bool) {
    let left = capture_list_pool_get(&cursor.capture_list_pool, left_state.capture_list_id as u16);
    let right = capture_list_pool_get(
        &cursor.capture_list_pool,
        right_state.capture_list_id as u16,
    );
    let mut left_contains_right = true;
    let mut right_contains_left = true;
    let (mut i, mut j) = (0, 0);
    loop {
        if i < left.len() {
            if j < right.len() {
                if std::ptr::eq(left[i].node.subtree, right[j].node.subtree)
                    && left[i].index == right[j].index
                {
                    i += 1;
                    j += 1;
                } else {
                    match ts_query_cursor__compare_nodes(left[i].node, right[j].node) {
                        -1 => {
                            right_contains_left = false;
                            i += 1;
                        }
                        1 => {
                            left_contains_right = false;
                            j += 1;
                        }
                        _ => {
                            right_contains_left = false;
                            left_contains_right = false;
                            i += 1;
                            j += 1;
                        }
                    }
                }
            } else {
                right_contains_left = false;
                break;
            }
        } else {
            if j < right.len() {
                left_contains_right = false;
            }
            break;
        }
    }
    (left_contains_right, right_contains_left)
}

pub(crate) fn ts_query_cursor__add_state(
    cursor: &mut QueryExecution<'_, '_>,
    pattern: &PatternEntry,
) {
    let step = &cursor.query.steps[usize::from(pattern.step_index)];
    let start_depth = cursor.depth.wrapping_sub(u32::from(step.depth));
    let states = &mut cursor.config.states;
    let mut index = states.len();
    // Search backwards to preserve ordering by start depth, then pattern. Only
    // the encountered same-depth/same-pattern/same-step state is deduplicated.
    while index > 0 {
        let previous = &states[index - 1];
        if u32::from(previous.start_depth) < start_depth {
            break;
        }
        if u32::from(previous.start_depth) == start_depth {
            if previous.pattern_index == pattern.pattern_index
                && previous.step_index == pattern.step_index
            {
                return;
            }
            if previous.pattern_index <= pattern.pattern_index {
                break;
            }
        }
        index -= 1;
    }
    states.insert(
        index,
        QueryState {
            id: u32::MAX,
            capture_list_id: u32::from(NONE),
            start_depth: start_depth as u16,
            step_index: pattern.step_index,
            pattern_index: pattern.pattern_index,
            consumed_capture_count: 0,
            seeking_immediate_match: true,
            has_in_progress_alternatives: false,
            dead: false,
            needs_parent: step.depth == 1,
        },
    );
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
    if state.capture_list_id == u32::from(NONE) {
        state.capture_list_id = u32::from(capture_list_pool_acquire(&mut cursor.capture_list_pool));
        cursor.config.allocated_capture_list_count = cursor.capture_list_pool.list.len() as u32;
        if state.capture_list_id == u32::from(NONE) {
            cursor.config.did_exceed_match_limit = true;
            if let Some(first) = first_in_progress_capture(cursor, false)
                && first.state_index != state_index_to_preserve
            {
                let other = &mut cursor.config.states[first.state_index as usize];
                state.capture_list_id = other.capture_list_id;
                other.capture_list_id = u32::from(NONE);
                other.dead = true;
                let list = capture_list_pool_get_mut(
                    &mut cursor.capture_list_pool,
                    state.capture_list_id as u16,
                );
                list.clear();
                return Some(list);
            }
            return None;
        }
    }
    Some(capture_list_pool_get_mut(
        &mut cursor.capture_list_pool,
        state.capture_list_id as u16,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cursor_defaults_and_settings() {
        let mut cursor = ts_query_cursor_new();
        assert_eq!(ts_query_cursor_match_limit(&cursor), u32::MAX);
        assert_eq!(cursor.max_start_depth, u32::MAX);
        assert_eq!((cursor.start_byte, cursor.end_byte), (0, u32::MAX));
        assert_eq!(
            (cursor.start_point, cursor.end_point),
            (POINT_ZERO, POINT_MAX)
        );
        assert_eq!(ts_query_cursor_timeout_micros(&cursor), 0);
        assert!(!ts_query_cursor_did_exceed_match_limit(&cursor));
        assert!(cursor.states.capacity() >= 8);
        assert!(cursor.finished_states.capacity() >= 8);

        cursor.allocated_capture_list_count = 7;
        ts_query_cursor_set_match_limit(&mut cursor, 2);
        assert_eq!(ts_query_cursor_match_limit(&cursor), 2);
        // Lowering the limit must not revoke already allocated pool slots.
        assert_eq!(cursor.allocated_capture_list_count, 7);
        ts_query_cursor_set_timeout_micros(&mut cursor, 123_456);
        assert_eq!(ts_query_cursor_timeout_micros(&cursor), 123_456);
        ts_query_cursor_delete(cursor);
    }

    #[test]
    fn byte_range_zero_end_is_unbounded_and_rejection_is_atomic() {
        let mut cursor = ts_query_cursor_new();
        assert!(ts_query_cursor_set_byte_range(&mut cursor, 4, 9));
        assert!(!ts_query_cursor_set_byte_range(&mut cursor, 10, 9));
        assert_eq!((cursor.start_byte, cursor.end_byte), (4, 9));
        assert!(ts_query_cursor_set_byte_range(&mut cursor, 9, 9));
        assert!(ts_query_cursor_set_byte_range(&mut cursor, u32::MAX, 0));
        assert_eq!((cursor.start_byte, cursor.end_byte), (u32::MAX, u32::MAX));
    }

    #[test]
    fn point_range_uses_lexicographic_order_and_zero_end_is_unbounded() {
        let mut cursor = ts_query_cursor_new();
        let start = Point { row: 2, column: 8 };
        let end = Point { row: 3, column: 0 };
        assert!(ts_query_cursor_set_point_range(&mut cursor, start, end));
        assert!(!ts_query_cursor_set_point_range(&mut cursor, end, start));
        assert_eq!((cursor.start_point, cursor.end_point), (start, end));
        assert!(ts_query_cursor_set_point_range(&mut cursor, start, start));
        assert!(ts_query_cursor_set_point_range(
            &mut cursor,
            POINT_MAX,
            POINT_ZERO
        ));
        assert_eq!(
            (cursor.start_point, cursor.end_point),
            (POINT_MAX, POINT_MAX)
        );
        assert_eq!((cursor.start_byte, cursor.end_byte), (0, u32::MAX));
    }

    fn language() -> Language {
        static TABLES: std::sync::LazyLock<ts_port_tables::LanguageTables> =
            std::sync::LazyLock::new(|| {
                ts_port_tables::LanguageTables::decode(
                    include_bytes!("../../grammars/c/src/tables.bin"),
                    |_, _| unreachable!("these tests do not lex"),
                    Some(|_, _| unreachable!("these tests do not lex keywords")),
                    None,
                )
            });
        Language::from(&*TABLES)
    }

    fn step(depth: u16) -> QueryStep {
        QueryStep {
            symbol: 1,
            supertype_symbol: 0,
            field: 0,
            capture_ids: [NONE; MAX_STEP_CAPTURE_COUNT],
            depth,
            alternative_index: NONE,
            negated_field_list_id: 0,
            is_named: false,
            is_immediate: false,
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

    fn query(steps: Vec<QueryStep>) -> CompiledQuery {
        CompiledQuery {
            captures: SymbolTable::default(),
            predicate_values: SymbolTable::default(),
            capture_quantifiers: Vec::new(),
            steps,
            pattern_map: Vec::new(),
            predicate_steps: Vec::new(),
            patterns: Vec::new(),
            step_offsets: Vec::new(),
            negated_fields: vec![0],
            string_buffer: Vec::new(),
            repeat_symbols_with_rootless_patterns: Vec::new(),
            language: language(),
            wildcard_root_pattern_count: 0,
        }
    }

    #[test]
    fn fallibility_distinguishes_wildcard_roots_and_done_steps() {
        let mut query = query(vec![step(0), step(1), step(PATTERN_DONE_MARKER)]);
        assert!(ts_query__step_is_fallible(&query, 0));
        query.steps[1].parent_pattern_guaranteed = true;
        assert!(!ts_query__step_is_fallible(&query, 0));
        query.steps[0].symbol = WILDCARD_SYMBOL;
        assert!(ts_query__step_is_fallible(&query, 0));
        assert!(!ts_query__step_is_fallible(&query, 1));
        query.steps[1].depth = 0;
        assert!(!ts_query__step_is_fallible(&query, 0));
    }

    #[test]
    fn guaranteed_step_uses_last_offset_at_or_before_requested_byte() {
        let mut query = query(vec![step(0), step(1), step(1)]);
        query.steps[1].root_pattern_guaranteed = true;
        query.step_offsets = vec![
            StepOffset {
                byte_offset: 3,
                step_index: 0,
            },
            StepOffset {
                byte_offset: 5,
                step_index: 1,
            },
            StepOffset {
                byte_offset: 8,
                step_index: 1,
            },
            StepOffset {
                byte_offset: 8,
                step_index: 2,
            },
        ];
        assert!(!ts_query_is_pattern_guaranteed_at_step(&query, 2));
        assert!(!ts_query_is_pattern_guaranteed_at_step(&query, 4));
        assert!(ts_query_is_pattern_guaranteed_at_step(&query, 5));
        assert!(ts_query_is_pattern_guaranteed_at_step(&query, 7));
        assert!(!ts_query_is_pattern_guaranteed_at_step(&query, 8));
        assert!(!ts_query_is_pattern_guaranteed_at_step(&query, u32::MAX));
    }

    #[test]
    fn disabling_pattern_removes_all_map_entries_only() {
        let mut query = query(vec![step(0), step(PATTERN_DONE_MARKER)]);
        query.wildcard_root_pattern_count = 2;
        query.pattern_map = vec![
            PatternEntry {
                pattern_index: 0,
                step_index: 0,
                is_rooted: true,
            },
            PatternEntry {
                pattern_index: 1,
                step_index: 0,
                is_rooted: true,
            },
            PatternEntry {
                pattern_index: 0,
                step_index: 0,
                is_rooted: false,
            },
        ];
        assert!(!ts_query_is_pattern_rooted(&query, 0));
        assert!(ts_query_is_pattern_rooted(&query, 1));
        assert!(ts_query_is_pattern_rooted(&query, u32::MAX));
        assert!(!ts_query_is_pattern_non_local(&query, u32::MAX));
        ts_query_disable_pattern(&mut query, 0);
        assert_eq!(query.pattern_map.len(), 1);
        assert_eq!(query.pattern_map[0].pattern_index, 1);
        assert!(ts_query_is_pattern_rooted(&query, 0));
        assert_eq!(query.steps.len(), 2);
        assert_eq!(query.wildcard_root_pattern_count, 2);
    }

    #[test]
    fn states_are_ordered_and_deduplicated_with_c_width_depths() {
        let query = query(vec![step(0), step(1)]);
        let tree = crate::tree::Tree {
            root: Box::new(crate::subtree::Subtree::Null),
            language: language(),
            included_ranges: Vec::new(),
        };
        let node = crate::tree::ts_tree_root_node(&tree);
        let mut config = ts_query_cursor_new();
        // Construct only the storage needed by add_state; compilation and pool
        // allocation are independently translated units.
        let mut execution = QueryExecution {
            config: &mut config,
            query: &query,
            cursor: ts_tree_cursor_new(node),
            capture_list_pool: CaptureListPool {
                list: Vec::new(),
                empty_list: Vec::new(),
                max_capture_list_count: u32::MAX,
                free_capture_list_count: 0,
            },
            depth: 3,
            next_state_id: 0,
            end_clock: None,
            query_options: QueryCursorOptions::default(),
            query_state: QueryCursorState::default(),
            operation_count: 0,
            on_visible_node: true,
            ascending: false,
            halted: false,
        };
        let pattern = |pattern_index, step_index| PatternEntry {
            pattern_index,
            step_index,
            is_rooted: true,
        };
        ts_query_cursor__add_state(&mut execution, &pattern(2, 0));
        ts_query_cursor__add_state(&mut execution, &pattern(0, 0));
        ts_query_cursor__add_state(&mut execution, &pattern(1, 0));
        ts_query_cursor__add_state(&mut execution, &pattern(1, 0));
        ts_query_cursor__add_state(&mut execution, &pattern(4, 1));
        let states = &execution.config.states;
        assert_eq!(
            states
                .iter()
                .map(|s| (s.start_depth, s.pattern_index))
                .collect::<Vec<_>>(),
            [(2, 4), (3, 0), (3, 1), (3, 2)],
        );
        assert!(states[0].needs_parent);
        assert!(!states[1].needs_parent);
        assert!(states.iter().all(|s| s.id == u32::MAX
            && s.capture_list_id == u32::from(NONE)
            && s.seeking_immediate_match
            && !s.dead));

        // C subtracts as u32, compares before truncating, then stores as u16.
        execution.depth = 0;
        ts_query_cursor__add_state(&mut execution, &pattern(5, 1));
        let last = execution.config.states.last().unwrap();
        assert_eq!(last.start_depth, u16::MAX);
        assert_eq!(last.pattern_index, 5);
    }

    #[test]
    fn node_order_uses_slot_identity_then_start_then_decreasing_end() {
        use crate::{
            length::Length,
            subtree::{InlineLeaf, Subtree},
            tree::Tree,
        };
        let tree = Tree {
            root: Box::new(Subtree::Inline(InlineLeaf {
                size_bytes: 10,
                ..InlineLeaf::default()
            })),
            language: language(),
            included_ranges: Vec::new(),
        };
        let shorter = Subtree::Inline(InlineLeaf {
            size_bytes: 5,
            ..InlineLeaf::default()
        });
        let same_length = (*tree.root).clone();
        let root = crate::tree::ts_tree_root_node(&tree);
        let child = Node {
            subtree: &shorter,
            ..root
        };
        let equal_range = Node {
            subtree: &same_length,
            ..root
        };
        assert_eq!(ts_query_cursor__compare_nodes(root, child), -1);
        assert_eq!(ts_query_cursor__compare_nodes(child, root), 1);
        assert_eq!(ts_query_cursor__compare_nodes(root, equal_range), 0);
        let offset = Node {
            position: Length {
                bytes: 1,
                extent: Point { row: 0, column: 1 },
            },
            ..root
        };
        // Identical slots compare equal even with differing context positions.
        assert_eq!(ts_query_cursor__compare_nodes(root, offset), 0);
        assert_eq!(ts_query_cursor__compare_nodes(equal_range, offset), -1);
    }
}
