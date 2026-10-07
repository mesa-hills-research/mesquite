//! Query matching state machine, translated from query.c.
use super::*;
use crate::{
    clock::{clock_is_gt, clock_is_null, clock_now},
    node::{
        ts_node_child_by_field_id, ts_node_end_byte, ts_node_end_point, ts_node_is_missing,
        ts_node_is_named, ts_node_start_byte, ts_node_start_point, ts_node_symbol,
    },
    subtree::{ts_subtree_is_repetition, ts_subtree_symbol},
    tree_cursor::{
        TreeCursorStep, ts_tree_cursor_current_node, ts_tree_cursor_current_status,
        ts_tree_cursor_current_subtree, ts_tree_cursor_goto_first_child_internal,
        ts_tree_cursor_goto_next_sibling_internal, ts_tree_cursor_goto_parent,
        ts_tree_cursor_parent_node,
    },
};
use ts_port_tables::BUILTIN_SYM_ERROR;

pub(crate) fn ts_query_cursor__capture<'query, 'tree: 'query>(
    cursor: &mut QueryExecution<'query, 'tree>,
    state_index: usize,
    step: &QueryStep,
    node: Node<'tree>,
) {
    let mut state = cursor.config.states[state_index];
    if state.dead {
        return;
    }
    if let Some(captures) = ts_query_cursor__prepare_to_capture(cursor, &mut state, u32::MAX) {
        for &capture_id in &step.capture_ids {
            if capture_id == NONE {
                break;
            }
            captures.push(QueryCapture {
                node,
                index: u32::from(capture_id),
            });
        }
    } else {
        state.dead = true;
    }
    // Preparation may have marked a different state dead while stealing its list.
    // Only write back the state that was detached for the capture operation.
    cursor.config.states[state_index] = state;
}

pub(crate) fn ts_query_cursor__copy_state(
    cursor: &mut QueryExecution<'_, '_>,
    state_index: usize,
) -> Option<usize> {
    let state = cursor.config.states[state_index];
    let mut copy = state;
    copy.capture_list_id = u32::from(NONE);

    if state.capture_list_id != u32::from(NONE) {
        // Never steal the original state's captures to make its copy.
        ts_query_cursor__prepare_to_capture(cursor, &mut copy, state_index as u32)?;
        let old_id = state.capture_list_id as u16;
        let new_id = copy.capture_list_id as u16;
        let count = capture_list_pool_get(&cursor.capture_list_pool, old_id).len();
        // Copy each capture value with disjoint, short-lived borrows. No extra
        // capture buffer is allocated, and both pool ids retain C's u16 width.
        for i in 0..count {
            let capture = capture_list_pool_get(&cursor.capture_list_pool, old_id)[i];
            capture_list_pool_get_mut(&mut cursor.capture_list_pool, new_id).push(capture);
        }
    }

    cursor.config.states.insert(state_index + 1, copy);
    Some(state_index + 1)
}

pub(crate) fn ts_query_cursor__should_descend(
    cursor: &QueryExecution<'_, '_>,
    node_intersects_range: bool,
) -> bool {
    if node_intersects_range && cursor.depth < cursor.config.max_start_depth {
        return true;
    }

    // A match already in progress may need children even outside the range or
    // beyond the maximum depth at which new matches can begin.
    for state in &cursor.config.states {
        let next_step = &cursor.query.steps[state.step_index as usize];
        if next_step.depth != PATTERN_DONE_MARKER
            && u32::from(state.start_depth) + u32::from(next_step.depth) > cursor.depth
        {
            return true;
        }
    }

    if cursor.depth >= cursor.config.max_start_depth {
        return false;
    }

    if !cursor.on_visible_node {
        // Rootless patterns may span hidden nodes. Repetitions can be large,
        // however, so descend outside the range only for analyzed rootless roots.
        let subtree = ts_tree_cursor_current_subtree(&cursor.cursor);
        if ts_subtree_is_repetition(subtree) {
            return cursor
                .query
                .repeat_symbols_with_rootless_patterns
                .binary_search(&ts_subtree_symbol(subtree))
                .is_ok();
        }
        return true;
    }
    false
}

pub(crate) fn ts_query_cursor__advance(
    cursor: &mut QueryExecution<'_, '_>,
    stop_on_definite_step: bool,
) -> bool {
    let mut did_match = false;
    loop {
        if cursor.halted {
            while let Some(state) = cursor.config.states.pop() {
                capture_list_pool_release(
                    &mut cursor.capture_list_pool,
                    state.capture_list_id as u16,
                );
            }
        }

        cursor.operation_count = cursor.operation_count.wrapping_add(1);
        if cursor.operation_count == OP_COUNT_PER_QUERY_TIMEOUT_CHECK {
            cursor.operation_count = 0;
        }
        if cursor.query_options.progress_callback.is_some() {
            cursor.query_state.current_byte_offset =
                ts_node_start_byte(ts_tree_cursor_current_node(&cursor.cursor));
        }
        if did_match
            || cursor.halted
            || (cursor.operation_count == 0
                && ((!clock_is_null(cursor.end_clock)
                    && clock_is_gt(clock_now(), cursor.end_clock))
                    || cursor
                        .query_options
                        .progress_callback
                        .as_mut()
                        .is_some_and(|callback| callback(&cursor.query_state))))
        {
            return did_match;
        }

        if cursor.ascending {
            if cursor.on_visible_node {
                // Compact in place, preserving the order of both the remaining
                // states and the states whose longest match finishes here.
                let mut deleted_count = 0;
                let count = cursor.config.states.len();
                for i in 0..count {
                    let state = cursor.config.states[i];
                    let step = &cursor.query.steps[state.step_index as usize];
                    if step.depth == PATTERN_DONE_MARKER
                        && (u32::from(state.start_depth) > cursor.depth || cursor.depth == 0)
                    {
                        cursor.config.finished_states.push(state);
                        did_match = true;
                        deleted_count += 1;
                    } else if step.depth != PATTERN_DONE_MARKER
                        && u32::from(state.start_depth) + u32::from(step.depth) > cursor.depth
                    {
                        capture_list_pool_release(
                            &mut cursor.capture_list_pool,
                            state.capture_list_id as u16,
                        );
                        deleted_count += 1;
                    } else if deleted_count > 0 {
                        cursor.config.states[i - deleted_count] = state;
                    }
                }
                cursor.config.states.truncate(count - deleted_count);
            }

            match ts_tree_cursor_goto_next_sibling_internal(&mut cursor.cursor) {
                TreeCursorStep::Visible => {
                    if !cursor.on_visible_node {
                        cursor.depth = cursor.depth.wrapping_add(1);
                        cursor.on_visible_node = true;
                    }
                    cursor.ascending = false;
                }
                TreeCursorStep::Hidden => {
                    if cursor.on_visible_node {
                        cursor.depth = cursor.depth.wrapping_sub(1);
                        cursor.on_visible_node = false;
                    }
                    cursor.ascending = false;
                }
                TreeCursorStep::None => {
                    if ts_tree_cursor_goto_parent(&mut cursor.cursor) {
                        cursor.depth = cursor.depth.wrapping_sub(1);
                    } else {
                        cursor.halted = true;
                    }
                }
            }
        } else {
            let node = ts_tree_cursor_current_node(&cursor.cursor);
            let parent_node = ts_tree_cursor_parent_node(&cursor.cursor);
            let start_byte = ts_node_start_byte(node);
            let end_byte = ts_node_end_byte(node);
            let start_point = ts_node_start_point(node);
            let end_point = ts_node_end_point(node);
            let is_empty = start_byte == end_byte;

            let parent_precedes_range = parent_node.is_some_and(|parent| {
                ts_node_end_byte(parent) <= cursor.config.start_byte
                    || ts_node_end_point(parent) <= cursor.config.start_point
            });
            let parent_follows_range = parent_node.is_some_and(|parent| {
                ts_node_start_byte(parent) >= cursor.config.end_byte
                    || ts_node_start_point(parent) >= cursor.config.end_point
            });
            let node_precedes_range = parent_precedes_range
                || end_byte < cursor.config.start_byte
                || end_point < cursor.config.start_point
                || (!is_empty && end_byte == cursor.config.start_byte)
                || (!is_empty && end_point == cursor.config.start_point);
            let node_follows_range = parent_follows_range
                || start_byte >= cursor.config.end_byte
                || start_point >= cursor.config.end_point;
            let parent_intersects_range = !parent_precedes_range && !parent_follows_range;
            let node_intersects_range = !node_precedes_range && !node_follows_range;

            if cursor.on_visible_node {
                let symbol = ts_node_symbol(node);
                let is_named = ts_node_is_named(node);
                let is_missing = ts_node_is_missing(node);
                let status = ts_tree_cursor_current_status(&cursor.cursor);
                let field_id = status.field_id;
                // query.c supplies an eight-element buffer to current_status.
                let supertypes = &status.supertypes[..status.supertypes.len().min(8)];
                let node_is_error = symbol == BUILTIN_SYM_ERROR;
                let parent_is_error =
                    parent_node.is_some_and(|parent| ts_node_symbol(parent) == BUILTIN_SYM_ERROR);

                if !node_is_error {
                    for i in 0..cursor.query.wildcard_root_pattern_count as usize {
                        let pattern = cursor.query.pattern_map[i];
                        let step = &cursor.query.steps[pattern.step_index as usize];
                        let start_depth = cursor.depth.wrapping_sub(u32::from(step.depth));
                        if (if pattern.is_rooted {
                            node_intersects_range
                        } else {
                            parent_intersects_range && !parent_is_error
                        }) && (step.field == 0 || field_id == step.field)
                            && (step.supertype_symbol == 0 || !supertypes.is_empty())
                            && start_depth <= cursor.config.max_start_depth
                        {
                            ts_query_cursor__add_state(cursor, &pattern);
                        }
                    }
                }

                let (found, index) = ts_query__pattern_map_search(cursor.query, symbol);
                if found {
                    let mut i = index as usize;
                    let first_step =
                        &cursor.query.steps[cursor.query.pattern_map[i].step_index as usize];
                    // C computes this once, from the first entry for this symbol,
                    // rather than recomputing it for subsequent pattern entries.
                    let start_depth = cursor.depth.wrapping_sub(u32::from(first_step.depth));
                    loop {
                        let pattern = cursor.query.pattern_map[i];
                        let step = &cursor.query.steps[pattern.step_index as usize];
                        if (if pattern.is_rooted {
                            node_intersects_range
                        } else {
                            parent_intersects_range && !parent_is_error
                        }) && (step.field == 0 || field_id == step.field)
                            && start_depth <= cursor.config.max_start_depth
                        {
                            ts_query_cursor__add_state(cursor, &pattern);
                        }
                        i += 1;
                        if i == cursor.query.pattern_map.len()
                            || cursor.query.steps[cursor.query.pattern_map[i].step_index as usize]
                                .symbol
                                != symbol
                        {
                            break;
                        }
                    }
                }

                // Each original state is processed once. Copies made while
                // matching this node are skipped here, but their alternatives
                // are expanded by the inner branching loop below.
                let mut j = 0;
                while j < cursor.config.states.len() {
                    cursor.config.states[j].has_in_progress_alternatives = false;
                    let state = cursor.config.states[j];
                    let step = cursor.query.steps[state.step_index as usize];
                    let mut copy_count = 0;
                    if u32::from(state.start_depth) + u32::from(step.depth) != cursor.depth {
                        j += 1;
                        continue;
                    }

                    let mut node_does_match = if step.symbol == WILDCARD_SYMBOL {
                        if step.is_missing {
                            is_missing
                        } else {
                            !node_is_error && (is_named || !step.is_named)
                        }
                    } else {
                        symbol == step.symbol && (!step.is_missing || is_missing)
                    };
                    let mut later_sibling_can_match = status.has_later_siblings;
                    if (step.is_immediate && is_named) || state.seeking_immediate_match {
                        later_sibling_can_match = false;
                    }
                    if step.is_last_child && status.has_later_named_siblings {
                        node_does_match = false;
                    }
                    if step.supertype_symbol != 0 && !supertypes.contains(&step.supertype_symbol) {
                        node_does_match = false;
                    }
                    if step.field != 0 {
                        if step.field == field_id {
                            if !status.can_have_later_siblings_with_this_field {
                                later_sibling_can_match = false;
                            }
                        } else {
                            node_does_match = false;
                        }
                    }
                    if step.negated_field_list_id != 0 {
                        for &negated_field in
                            &cursor.query.negated_fields[step.negated_field_list_id as usize..]
                        {
                            if negated_field == 0 {
                                break;
                            }
                            if ts_node_child_by_field_id(node, negated_field).is_some() {
                                node_does_match = false;
                                break;
                            }
                        }
                    }

                    if !node_does_match {
                        if !later_sibling_can_match {
                            capture_list_pool_release(
                                &mut cursor.capture_list_pool,
                                state.capture_list_id as u16,
                            );
                            cursor.config.states.remove(j);
                        } else {
                            j += 1;
                        }
                        continue;
                    }

                    if later_sibling_can_match
                        && (step.contains_captures
                            || ts_query__step_is_fallible(cursor.query, state.step_index))
                        && ts_query_cursor__copy_state(cursor, j).is_some()
                    {
                        copy_count += 1;
                    }

                    // A pattern-map entry can skip a leading wildcard. Capture
                    // its parent, searching backward past branches and children.
                    if cursor.config.states[j].needs_parent {
                        if let Some(parent) = ts_tree_cursor_parent_node(&cursor.cursor) {
                            cursor.config.states[j].needs_parent = false;
                            let mut skipped_index = state.step_index as usize;
                            loop {
                                skipped_index -= 1;
                                let skipped_step = cursor.query.steps[skipped_index];
                                if !skipped_step.is_dead_end
                                    && !skipped_step.is_pass_through
                                    && skipped_step.depth == 0
                                {
                                    if skipped_step.capture_ids[0] != NONE {
                                        ts_query_cursor__capture(cursor, j, &skipped_step, parent);
                                    }
                                    break;
                                }
                            }
                        } else {
                            cursor.config.states[j].dead = true;
                        }
                    }
                    if step.capture_ids[0] != NONE {
                        ts_query_cursor__capture(cursor, j, &step, node);
                    }
                    if cursor.config.states[j].dead {
                        cursor.config.states.remove(j);
                        j += copy_count;
                        continue;
                    }

                    cursor.config.states[j].step_index =
                        cursor.config.states[j].step_index.wrapping_add(1);
                    let next_step = cursor.query.steps[cursor.config.states[j].step_index as usize];
                    // An unnamed wildcard with an anchor also treats anonymous
                    // siblings as significant for the immediate next match.
                    cursor.config.states[j].seeking_immediate_match =
                        step.symbol == WILDCARD_SYMBOL && !step.is_named && next_step.is_immediate;
                    if stop_on_definite_step && next_step.root_pattern_guaranteed {
                        did_match = true;
                    }

                    let mut end_index = j + 1;
                    let mut k = j;
                    while k < end_index {
                        let child_step =
                            cursor.query.steps[cursor.config.states[k].step_index as usize];
                        if child_step.alternative_index != NONE {
                            if child_step.is_dead_end {
                                cursor.config.states[k].step_index = child_step.alternative_index;
                                continue;
                            }
                            if child_step.is_pass_through {
                                cursor.config.states[k].step_index =
                                    cursor.config.states[k].step_index.wrapping_add(1);
                            }
                            if let Some(copy_index) = ts_query_cursor__copy_state(cursor, k) {
                                end_index += 1;
                                copy_count += 1;
                                let copy = &mut cursor.config.states[copy_index];
                                copy.step_index = child_step.alternative_index;
                                if child_step.alternative_is_immediate {
                                    copy.seeking_immediate_match = true;
                                }
                            }
                            // A pass-through step must now process its sequential
                            // successor before visiting the alternative copy.
                            if child_step.is_pass_through {
                                continue;
                            }
                        }
                        k += 1;
                    }
                    j += 1 + copy_count;
                }

                let mut j = 0;
                while j < cursor.config.states.len() {
                    if cursor.config.states[j].dead {
                        cursor.config.states.remove(j);
                        continue;
                    }

                    // Longest-match pruning only compares states belonging to
                    // the same pattern and root. The state array is ordered by
                    // start depth and then pattern index.
                    let mut did_remove = false;
                    let mut k = j + 1;
                    while k < cursor.config.states.len() {
                        let state = cursor.config.states[j];
                        let other_state = cursor.config.states[k];
                        if other_state.start_depth != state.start_depth
                            || other_state.pattern_index != state.pattern_index
                        {
                            break;
                        }
                        let (left_contains_right, right_contains_left) =
                            ts_query_cursor__compare_captures(cursor, &state, &other_state);
                        if left_contains_right {
                            if state.step_index == other_state.step_index {
                                capture_list_pool_release(
                                    &mut cursor.capture_list_pool,
                                    other_state.capture_list_id as u16,
                                );
                                cursor.config.states.remove(k);
                                continue;
                            }
                            cursor.config.states[k].has_in_progress_alternatives = true;
                        }
                        if right_contains_left {
                            if state.step_index == other_state.step_index {
                                capture_list_pool_release(
                                    &mut cursor.capture_list_pool,
                                    state.capture_list_id as u16,
                                );
                                cursor.config.states.remove(j);
                                did_remove = true;
                                break;
                            }
                            cursor.config.states[j].has_in_progress_alternatives = true;
                        }
                        k += 1;
                    }
                    if did_remove {
                        continue;
                    }

                    let state = cursor.config.states[j];
                    let next_step = &cursor.query.steps[state.step_index as usize];
                    if next_step.depth == PATTERN_DONE_MARKER && !state.has_in_progress_alternatives
                    {
                        cursor.config.finished_states.push(state);
                        cursor.config.states.remove(j);
                        did_match = true;
                    } else {
                        j += 1;
                    }
                }
            }

            if ts_query_cursor__should_descend(cursor, node_intersects_range) {
                match ts_tree_cursor_goto_first_child_internal(&mut cursor.cursor) {
                    TreeCursorStep::Visible => {
                        cursor.depth = cursor.depth.wrapping_add(1);
                        cursor.on_visible_node = true;
                        continue;
                    }
                    TreeCursorStep::Hidden => {
                        cursor.on_visible_node = false;
                        continue;
                    }
                    TreeCursorStep::None => {}
                }
            }
            cursor.ascending = true;
        }
    }
}
