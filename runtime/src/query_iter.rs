//! Match and capture iteration, mirroring query.c.
use super::*;
use crate::node::{ts_node_end_byte, ts_node_end_point, ts_node_start_byte, ts_node_start_point};

/// C lends out a pool buffer until the next cursor operation. The Rust execution
/// interface returns an owned snapshot, restricted to C's u16 capture_count.
fn match_for_state<'tree>(
    state: &mut QueryState,
    next_state_id: &mut u32,
    pool: &CaptureListPool<'tree>,
) -> QueryMatchData<'tree> {
    if state.id == u32::MAX {
        state.id = *next_state_id;
        *next_state_id = next_state_id.wrapping_add(1);
    }
    let captures = capture_list_pool_get(pool, state.capture_list_id as u16);
    QueryMatchData {
        id: state.id,
        pattern_index: state.pattern_index,
        captures: captures[..captures.len() as u16 as usize].to_vec(),
    }
}

pub(crate) fn ts_query_cursor_next_match<'query, 'tree: 'query>(
    cursor: &mut QueryExecution<'query, 'tree>,
) -> Option<QueryMatchData<'tree>> {
    if cursor.config.finished_states.is_empty() && !ts_query_cursor__advance(cursor, false) {
        return None;
    }

    let state = &mut cursor.config.finished_states[0];
    let result = match_for_state(state, &mut cursor.next_state_id, &cursor.capture_list_pool);
    capture_list_pool_release(&mut cursor.capture_list_pool, state.capture_list_id as u16);
    cursor.config.finished_states.remove(0);
    Some(result)
}

pub(crate) fn ts_query_cursor_remove_match(cursor: &mut QueryExecution<'_, '_>, match_id: u32) {
    if let Some(index) = cursor
        .config
        .finished_states
        .iter()
        .position(|state| state.id == match_id)
    {
        capture_list_pool_release(
            &mut cursor.capture_list_pool,
            cursor.config.finished_states[index].capture_list_id as u16,
        );
        cursor.config.finished_states.remove(index);
        return;
    }

    // Unfinished matches may have emitted definite captures already. Removing
    // their state also prevents any later captures from being emitted.
    if let Some(index) = cursor
        .config
        .states
        .iter()
        .position(|state| state.id == match_id)
    {
        capture_list_pool_release(
            &mut cursor.capture_list_pool,
            cursor.config.states[index].capture_list_id as u16,
        );
        cursor.config.states.remove(index);
    }
}

pub(crate) fn ts_query_cursor_next_capture<'query, 'tree: 'query>(
    cursor: &mut QueryExecution<'query, 'tree>,
) -> Option<(QueryMatchData<'tree>, u32)> {
    // Patterns overlap, so discovery order need not be capture order. Wait for
    // a finished capture before all unfinished ones, or a definite unfinished one.
    loop {
        let first_unfinished = ts_query_cursor__first_in_progress_capture(cursor);
        let (mut first_finished_capture_byte, mut first_finished_pattern_index) = first_unfinished
            .map_or((u32::MAX, u32::MAX), |capture| {
                (capture.byte_offset, capture.pattern_index)
            });
        let mut first_finished_state = None;
        let mut i = 0;
        while i < cursor.config.finished_states.len() {
            let state = &mut cursor.config.finished_states[i];
            let captures =
                capture_list_pool_get(&cursor.capture_list_pool, state.capture_list_id as u16);

            // Releasing exhausted matches is delayed until the next iteration,
            // just as in C, rather than happening when their last capture is read.
            if state.consumed_capture_count as usize >= captures.len() {
                capture_list_pool_release(
                    &mut cursor.capture_list_pool,
                    state.capture_list_id as u16,
                );
                cursor.config.finished_states.remove(i);
                continue;
            }

            let node = captures[state.consumed_capture_count as usize].node;
            let node_precedes_range = ts_node_end_byte(node) <= cursor.config.start_byte
                || ts_node_end_point(node) <= cursor.config.start_point;
            let node_follows_range = ts_node_start_byte(node) >= cursor.config.end_byte
                || ts_node_start_point(node) >= cursor.config.end_point;
            if node_precedes_range || node_follows_range {
                state.consumed_capture_count =
                    state.consumed_capture_count.wrapping_add(1) & 0x0fff;
                continue;
            }

            let node_start_byte = ts_node_start_byte(node);
            if node_start_byte < first_finished_capture_byte
                || (node_start_byte == first_finished_capture_byte
                    && (state.pattern_index as u32) < first_finished_pattern_index)
            {
                first_finished_state = Some(i);
                first_finished_capture_byte = node_start_byte;
                first_finished_pattern_index = state.pattern_index as u32;
            }
            i += 1;
        }

        let state = if let Some(index) = first_finished_state {
            Some(&mut cursor.config.finished_states[index])
        } else if let Some(capture) = first_unfinished.filter(|capture| capture.is_definite) {
            Some(&mut cursor.config.states[capture.state_index as usize])
        } else {
            None
        };
        if let Some(state) = state {
            let result =
                match_for_state(state, &mut cursor.next_state_id, &cursor.capture_list_pool);
            let capture_index = state.consumed_capture_count as u32;
            state.consumed_capture_count = state.consumed_capture_count.wrapping_add(1) & 0x0fff;
            return Some((result, capture_index));
        }

        if capture_list_pool_is_empty(&cursor.capture_list_pool)
            && let Some(capture) = first_unfinished
        {
            let index = capture.state_index as usize;
            capture_list_pool_release(
                &mut cursor.capture_list_pool,
                cursor.config.states[index].capture_list_id as u16,
            );
            cursor.config.states.remove(index);
        }

        if !ts_query_cursor__advance(cursor, true) && cursor.config.finished_states.is_empty() {
            return None;
        }
    }
}

pub(crate) fn ts_query_cursor_set_max_start_depth(cursor: &mut CursorConfig, max_start_depth: u32) {
    cursor.max_start_depth = max_start_depth;
}
