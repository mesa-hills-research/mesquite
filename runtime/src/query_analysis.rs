//! Query pattern-map insertion and parse-table analysis (query.c).
use super::*;
use crate::language::{
    ts_language_alias_at, ts_language_aliases_for_symbol, ts_language_field_map,
    ts_language_lookaheads, ts_language_state_is_primary, ts_language_symbol_metadata,
    ts_lookahead_iterator__next,
};
use std::cmp::Ordering;
use ts_port_tables::{BUILTIN_SYM_ERROR, ParseAction};

/// The array.h search, including its choice of the last equal entry. In
/// particular, step_offsets may contain more than one entry for a step.
fn analysis_search<T>(items: &[T], compare: impl Fn(&T) -> Ordering) -> (usize, bool) {
    let mut index = 0;
    let mut size = items.len();
    if size == 0 {
        return (index, false);
    }
    while size > 1 {
        let half_size = size / 2;
        let mid_index = index + half_size;
        if compare(&items[mid_index]) != Ordering::Greater {
            index = mid_index;
        }
        size -= half_size;
    }
    match compare(&items[index]) {
        Ordering::Equal => (index, true),
        Ordering::Less => (index + 1, false),
        Ordering::Greater => (index, false),
    }
}

fn analysis_insert_symbol(items: &mut Vec<u16>, symbol: u16) {
    let (index, exists) = analysis_search(items, |item| item.cmp(&symbol));
    if !exists {
        items.insert(index, symbol);
    }
}

pub(crate) fn ts_query__pattern_map_insert(
    query: &mut CompiledQuery,
    symbol: Symbol,
    new_entry: PatternEntry,
) {
    let (_, index) = ts_query__pattern_map_search(query, symbol);
    let mut index = index as usize;

    // Sort by pattern index within each symbol so earlier patterns' states are
    // initiated first. Equal pattern indices are inserted before existing ones.
    while let Some(entry) = query.pattern_map.get(index) {
        if query.steps[entry.step_index as usize].symbol == symbol
            && entry.pattern_index < new_entry.pattern_index
        {
            index += 1;
        } else {
            break;
        }
    }
    query.pattern_map.insert(index, new_entry);
}

pub(crate) fn ts_query__perform_analysis(
    query: &mut CompiledQuery,
    subgraphs: &[AnalysisSubgraph],
    analysis: &mut QueryAnalysis,
) {
    let language = query.language;
    let mut recursion_depth_limit = 0;
    let mut prev_final_step_count = 0;
    analysis.final_step_indices.clear();
    analysis.finished_parent_symbols.clear();

    for iteration in 0.. {
        if iteration == MAX_ANALYSIS_ITERATION_COUNT {
            analysis.did_abort = true;
            break;
        }

        // Only increase the recursion limit if new termination points have been
        // found since the previous increase.
        if analysis.states.is_empty() {
            if !analysis.deeper_states.is_empty()
                && analysis.final_step_indices.len() > prev_final_step_count
            {
                prev_final_step_count = analysis.final_step_indices.len();
                recursion_depth_limit += 1;
                std::mem::swap(&mut analysis.states, &mut analysis.deeper_states);
                continue;
            }
            break;
        }

        analysis_state_set__clear(&mut analysis.next_states, &mut analysis.state_pool);
        for j in 0..analysis.states.len() {
            let mut state = analysis.states[j];

            // Advance the least-progressed states first. A local state copy lets
            // us grow the other sets without borrowing into any of their Vecs.
            if let Some(last_state) = analysis.next_states.last() {
                let comparison = analysis_state__compare(&state, last_state);
                if comparison == 0 {
                    analysis_state_set__insert_sorted(
                        &mut analysis.next_states,
                        &mut analysis.state_pool,
                        &state,
                    );
                    continue;
                } else if comparison > 0 {
                    for remaining_state in &analysis.states[j..] {
                        analysis_state_set__push(
                            &mut analysis.next_states,
                            &mut analysis.state_pool,
                            remaining_state,
                        );
                    }
                    break;
                }
            }

            let top = *analysis_state__top(&mut state);
            let parse_state = top.parse_state;
            let parent_symbol = top.parent_symbol;
            let parent_field_id = top.field_id;
            let child_index = top.child_index;
            let step = query.steps[state.step_index as usize];

            let (subgraph_index, exists) =
                analysis_search(subgraphs, |subgraph| subgraph.symbol.cmp(&parent_symbol));
            if !exists {
                continue;
            }
            let subgraph = &subgraphs[subgraph_index];

            // Follow every parse-table path that stays in this parent's subgraph.
            let mut lookahead = ts_language_lookaheads(&language, parse_state);
            while ts_lookahead_iterator__next(&mut lookahead) {
                let symbol = lookahead.symbol;
                let mut successor = AnalysisSubgraphNode {
                    state: parse_state,
                    child_index: (child_index as u8) & 0x7f,
                    ..AnalysisSubgraphNode::default()
                };
                if let Some(action) = lookahead.actions.last() {
                    if let ParseAction::Shift { state, extra, .. } = *action.action() {
                        if !extra {
                            successor.state = state;
                            successor.child_index = (successor.child_index + 1) & 0x7f;
                        }
                    } else {
                        continue;
                    }
                } else if lookahead.next_state != 0 {
                    successor.state = lookahead.next_state;
                    successor.child_index = (successor.child_index + 1) & 0x7f;
                } else {
                    continue;
                }

                let (node_index, _) = analysis_search(&subgraph.nodes, |node| {
                    analysis_subgraph_node__compare(node, &successor).cmp(&0)
                });
                for node in &subgraph.nodes[node_index..] {
                    if node.state != successor.state || node.child_index != successor.child_index {
                        break;
                    }

                    // The eventual reduction determines this child's alias and
                    // field. An inherited field from a hidden ancestor wins.
                    let alias = ts_language_alias_at(
                        &language,
                        u32::from(node.production_id),
                        u32::from(child_index),
                    );
                    let visible_symbol = if alias != 0 {
                        alias
                    } else if language.tables.symbol_metadata[symbol as usize].visible {
                        language.tables.public_symbol_map[symbol as usize]
                    } else {
                        0
                    };
                    let mut field_id = parent_field_id;
                    if field_id == 0 {
                        for field in ts_language_field_map(&language, u32::from(node.production_id))
                        {
                            if !field.inherited && u16::from(field.child_index) == child_index {
                                field_id = field.field_id;
                                break;
                            }
                        }
                    }

                    let mut next_state = state;
                    let next_top = analysis_state__top(&mut next_state);
                    next_top.child_index = u16::from(successor.child_index);
                    next_top.parse_state = successor.state;
                    if node.done {
                        next_top.done = true;
                    }

                    let mut does_match = false;
                    // ERROR can occur anywhere; this case deliberately bypasses
                    // the field, namedness, and supertype checks below.
                    if step.symbol == BUILTIN_SYM_ERROR {
                        does_match = true;
                    } else if visible_symbol != 0 {
                        does_match = true;
                        if step.symbol == WILDCARD_SYMBOL {
                            if step.is_named
                                && !language.tables.symbol_metadata[visible_symbol as usize].named
                            {
                                does_match = false;
                            }
                        } else if step.symbol != visible_symbol {
                            does_match = false;
                        }
                        if step.field != 0 && step.field != field_id {
                            does_match = false;
                        }
                        if step.supertype_symbol != 0
                            && !analysis_state__has_supertype(&state, step.supertype_symbol)
                        {
                            does_match = false;
                        }
                    } else if u32::from(symbol) >= language.tables.token_count {
                        // Hidden children are flattened. Reuse a completed frame
                        // instead of pushing another frame in that case.
                        if !analysis_state__top(&mut next_state).done {
                            if usize::from(next_state.depth) + 1 >= MAX_ANALYSIS_STATE_DEPTH {
                                analysis.did_abort = true;
                                continue;
                            }
                            next_state.depth += 1;
                        }
                        *analysis_state__top(&mut next_state) = AnalysisStateEntry {
                            parse_state,
                            parent_symbol: symbol,
                            child_index: 0,
                            field_id: field_id & 0x7fff,
                            done: false,
                        };
                        if analysis_state__recursion_depth(&next_state) > recursion_depth_limit {
                            analysis_state_set__insert_sorted(
                                &mut analysis.deeper_states,
                                &mut analysis.state_pool,
                                &next_state,
                            );
                            continue;
                        }
                    }

                    while next_state.depth > 0 && analysis_state__top(&mut next_state).done {
                        next_state.depth -= 1;
                    }

                    // On a match, skip descendant query steps: this invocation
                    // analyzes only siblings at the current query depth.
                    let mut next_step_index = usize::from(state.step_index);
                    let mut next_step = step;
                    if does_match {
                        loop {
                            next_state.step_index = next_state.step_index.wrapping_add(1);
                            next_step_index = usize::from(next_state.step_index);
                            next_step = query.steps[next_step_index];
                            if next_step.depth == PATTERN_DONE_MARKER
                                || next_step.depth <= step.depth
                            {
                                break;
                            }
                        }
                    } else if successor.state == parse_state {
                        continue;
                    }

                    loop {
                        // Repetition pass-through alternatives are unnecessary
                        // when deciding possibility and definiteness.
                        if next_step.is_pass_through {
                            next_step_index += 1;
                            next_state.step_index = next_state.step_index.wrapping_add(1);
                            next_step = query.steps[next_step_index];
                            continue;
                        }

                        if !next_step.is_dead_end {
                            let did_finish_pattern =
                                query.steps[next_state.step_index as usize].depth != step.depth;
                            if did_finish_pattern {
                                analysis_insert_symbol(
                                    &mut analysis.finished_parent_symbols,
                                    state.root_symbol,
                                );
                            } else if next_state.depth == 0 {
                                analysis_insert_symbol(
                                    &mut analysis.final_step_indices,
                                    next_state.step_index,
                                );
                            } else {
                                analysis_state_set__insert_sorted(
                                    &mut analysis.next_states,
                                    &mut analysis.state_pool,
                                    &next_state,
                                );
                            }
                        }

                        // Only forward alternatives matter; backwards edges are
                        // repetitions, which need not be processed here.
                        if does_match
                            && next_step.alternative_index != NONE
                            && next_step.alternative_index > next_state.step_index
                        {
                            next_state.step_index = next_step.alternative_index;
                            next_step_index = usize::from(next_state.step_index);
                            next_step = query.steps[next_step_index];
                        } else {
                            break;
                        }
                    }
                }
            }
        }
        std::mem::swap(&mut analysis.states, &mut analysis.next_states);
    }
}

pub(crate) fn ts_query__analyze_patterns(query: &mut CompiledQuery) -> Result<(), u32> {
    let language = query.language;
    let mut non_rooted_pattern_start_steps = Vec::new();
    for (i, pattern) in query.pattern_map.iter().enumerate() {
        if !pattern.is_rooted && query.steps[pattern.step_index as usize].symbol != WILDCARD_SYMBOL
        {
            non_rooted_pattern_start_steps.push(i as u16);
        }
    }

    // Compute capture containment, record non-wildcard parents, and tentatively
    // mark their descendants as guaranteed. Analysis can revoke those marks.
    let mut parent_step_indices = Vec::new();
    for i in 0..query.steps.len() {
        if query.steps[i].depth == PATTERN_DONE_MARKER {
            query.steps[i].parent_pattern_guaranteed = true;
            query.steps[i].root_pattern_guaranteed = true;
            continue;
        }

        let depth = query.steps[i].depth;
        let is_wildcard = query.steps[i].symbol == WILDCARD_SYMBOL;
        let mut contains_captures = query.steps[i].capture_ids[0] != NONE;
        let mut has_children = false;
        for next_step in &mut query.steps[i + 1..] {
            if next_step.depth == PATTERN_DONE_MARKER || next_step.depth <= depth {
                break;
            }
            if next_step.capture_ids[0] != NONE {
                contains_captures = true;
            }
            if !is_wildcard {
                next_step.root_pattern_guaranteed = true;
                next_step.parent_pattern_guaranteed = true;
            }
            has_children = true;
        }
        query.steps[i].contains_captures = contains_captures;
        if has_children && !is_wildcard {
            parent_step_indices.push(i as u32);
        }
    }

    // Build a subgraph for each queried parent and every hidden nonterminal.
    let mut subgraphs: AnalysisSubgraphArray = Vec::new();
    for &parent_step_index in &parent_step_indices {
        let parent_symbol = query.steps[parent_step_index as usize].symbol;
        insert_subgraph(&mut subgraphs, parent_symbol);
    }
    for symbol in language.tables.token_count as u16..language.tables.symbol_count as u16 {
        if !ts_language_symbol_metadata(&language, symbol).visible {
            insert_subgraph(&mut subgraphs, symbol);
        }
    }

    // Collect possible starting states, reductions, and predecessor edges in
    // exactly the parse table's lookahead/action order.
    let mut predecessor_map = state_predecessor_map_new(&language);
    for state in 1..language.tables.state_count as u16 {
        let mut lookahead = ts_language_lookaheads(&language, state);
        while ts_lookahead_iterator__next(&mut lookahead) {
            if !lookahead.actions.is_empty() {
                for action in lookahead.actions {
                    match *action.action() {
                        ParseAction::Reduce {
                            symbol,
                            child_count,
                            production_id,
                            ..
                        } => {
                            for symbol in ts_language_aliases_for_symbol(&language, symbol) {
                                let (index, exists) = analysis_search(&subgraphs, |subgraph| {
                                    subgraph.symbol.cmp(symbol)
                                });
                                if exists {
                                    let subgraph = &mut subgraphs[index];
                                    if subgraph.nodes.last().is_none_or(|node| node.state != state)
                                    {
                                        subgraph.nodes.push(AnalysisSubgraphNode {
                                            state,
                                            production_id,
                                            child_index: child_count & 0x7f,
                                            done: true,
                                        });
                                    }
                                }
                            }
                        }
                        ParseAction::Shift {
                            state: next_state,
                            extra: false,
                            ..
                        } => state_predecessor_map_add(&mut predecessor_map, next_state, state),
                        _ => {}
                    }
                }
            } else if lookahead.next_state != 0 {
                if lookahead.next_state != state {
                    state_predecessor_map_add(&mut predecessor_map, lookahead.next_state, state);
                }
                if ts_language_state_is_primary(&language, state) {
                    for symbol in ts_language_aliases_for_symbol(&language, lookahead.symbol) {
                        let (index, exists) =
                            analysis_search(&subgraphs, |subgraph| subgraph.symbol.cmp(symbol));
                        if exists {
                            let subgraph = &mut subgraphs[index];
                            if subgraph.start_states.last() != Some(&state) {
                                subgraph.start_states.push(state);
                            }
                        }
                    }
                }
            }
        }
    }

    // Walk backwards from reductions. Drop empty subgraphs, including symbols
    // that are terminals even though a query gave them child steps.
    let mut next_nodes = Vec::new();
    let mut i = 0;
    while i < subgraphs.len() {
        let subgraph = &mut subgraphs[i];
        if subgraph.nodes.is_empty() {
            subgraphs.remove(i);
            continue;
        }
        next_nodes.clone_from(&subgraph.nodes);
        while let Some(node) = next_nodes.pop() {
            if node.child_index > 1 {
                for &predecessor in state_predecessor_map_get(&predecessor_map, node.state) {
                    let predecessor_node = AnalysisSubgraphNode {
                        state: predecessor,
                        child_index: node.child_index - 1,
                        production_id: node.production_id,
                        done: false,
                    };
                    let (index, exists) = analysis_search(&subgraph.nodes, |node| {
                        analysis_subgraph_node__compare(node, &predecessor_node).cmp(&0)
                    });
                    if !exists {
                        subgraph.nodes.insert(index, predecessor_node);
                        next_nodes.push(predecessor_node);
                    }
                }
            }
        }
        i += 1;
    }

    let mut result = Ok(());
    let mut analysis = query_analysis__new();
    for parent_step_index in parent_step_indices {
        // C narrows this u32 array entry to uint16_t before using it.
        let parent_step_index = parent_step_index as u16;
        let parent_depth = query.steps[parent_step_index as usize].depth;
        let parent_symbol = query.steps[parent_step_index as usize].symbol;
        if parent_symbol == BUILTIN_SYM_ERROR {
            continue;
        }

        let (subgraph_index, exists) =
            analysis_search(&subgraphs, |subgraph| subgraph.symbol.cmp(&parent_symbol));
        if !exists {
            let first_child_step_index = u32::from(parent_step_index) + 1;
            let (offset_index, child_exists) = analysis_search(&query.step_offsets, |offset| {
                u32::from(offset.step_index).cmp(&first_child_step_index)
            });
            assert!(child_exists);
            result = Err(query.step_offsets[offset_index].byte_offset);
            break;
        }

        let subgraph = &subgraphs[subgraph_index];
        analysis_state_set__clear(&mut analysis.states, &mut analysis.state_pool);
        analysis_state_set__clear(&mut analysis.deeper_states, &mut analysis.state_pool);
        for &parse_state in &subgraph.start_states {
            let state = initial_analysis_state(
                parent_step_index.wrapping_add(1),
                parse_state,
                parent_symbol,
            );
            analysis_state_set__push(&mut analysis.states, &mut analysis.state_pool, &state);
        }
        analysis.did_abort = false;
        ts_query__perform_analysis(query, &subgraphs, &mut analysis);

        // An incomplete analysis is not evidence of impossibility or certainty.
        if analysis.did_abort {
            for step in &mut query.steps[usize::from(parent_step_index) + 1..] {
                if step.depth <= parent_depth || step.depth == PATTERN_DONE_MARKER {
                    break;
                }
                if !step.is_dead_end {
                    step.parent_pattern_guaranteed = false;
                    step.root_pattern_guaranteed = false;
                }
            }
            continue;
        }

        if analysis.finished_parent_symbols.is_empty() {
            let impossible_step_index = *analysis
                .final_step_indices
                .last()
                .expect("analysis must have a termination step for an impossible pattern");
            let (offset_index, _) = analysis_search(&query.step_offsets, |offset| {
                offset.step_index.cmp(&impossible_step_index)
            });
            let offset_index = offset_index.min(query.step_offsets.len() - 1);
            result = Err(query.step_offsets[offset_index].byte_offset);
            break;
        }

        for &final_step_index in &analysis.final_step_indices {
            let step = &mut query.steps[final_step_index as usize];
            if step.depth != PATTERN_DONE_MARKER && step.depth > parent_depth && !step.is_dead_end {
                step.parent_pattern_guaranteed = false;
                step.root_pattern_guaranteed = false;
            }
        }
    }

    // Captures inspected by predicates cannot guarantee the whole root pattern,
    // but do not affect parent_pattern_guaranteed.
    let mut predicate_capture_ids = Vec::new();
    for pattern in &query.patterns {
        predicate_capture_ids.clear();
        let start = pattern.predicate_steps.offset;
        let end = start.wrapping_add(pattern.predicate_steps.length);
        for step in &query.predicate_steps[start as usize..end as usize] {
            if step.kind == PredicateStepKind::Capture {
                analysis_insert_symbol(&mut predicate_capture_ids, step.value_id as u16);
            }
        }
        let start = pattern.steps.offset;
        let end = start.wrapping_add(pattern.steps.length);
        for step in &mut query.steps[start as usize..end as usize] {
            for capture_id in step.capture_ids {
                if capture_id == NONE {
                    break;
                }
                let (_, exists) = analysis_search(&predicate_capture_ids, |id| id.cmp(&capture_id));
                if exists {
                    step.root_pattern_guaranteed = false;
                    break;
                }
            }
        }
    }

    // Propagate root fallibility backwards until fixed. A definite forward
    // alternative can still guarantee a step whose own path is fallible.
    let mut done = query.steps.is_empty();
    while !done {
        done = true;
        for i in (1..query.steps.len()).rev() {
            let mut step = &query.steps[i];
            if step.depth == PATTERN_DONE_MARKER {
                continue;
            }
            let mut parent_pattern_guaranteed = false;
            loop {
                if step.root_pattern_guaranteed {
                    parent_pattern_guaranteed = true;
                    break;
                }
                if step.alternative_index == NONE || usize::from(step.alternative_index) < i {
                    break;
                }
                step = &query.steps[step.alternative_index as usize];
            }
            if !parent_pattern_guaranteed {
                let prev_step = &mut query.steps[i - 1];
                if !prev_step.is_dead_end
                    && prev_step.depth != PATTERN_DONE_MARKER
                    && prev_step.root_pattern_guaranteed
                {
                    prev_step.root_pattern_guaranteed = false;
                    done = false;
                }
            }
        }
    }

    // Identify repetition symbols that can match rootless patterns. This work,
    // like predicate fallibility above, also runs after a structural error.
    analysis.did_abort = false;
    for pattern_entry_index in non_rooted_pattern_start_steps {
        let pattern_entry = query.pattern_map[pattern_entry_index as usize];
        analysis_state_set__clear(&mut analysis.states, &mut analysis.state_pool);
        analysis_state_set__clear(&mut analysis.deeper_states, &mut analysis.state_pool);
        for subgraph in &subgraphs {
            let metadata = ts_language_symbol_metadata(&language, subgraph.symbol);
            if metadata.visible || metadata.named {
                continue;
            }
            for &parse_state in &subgraph.start_states {
                let state =
                    initial_analysis_state(pattern_entry.step_index, parse_state, subgraph.symbol);
                analysis_state_set__push(&mut analysis.states, &mut analysis.state_pool, &state);
            }
        }
        ts_query__perform_analysis(query, &subgraphs, &mut analysis);
        if !analysis.finished_parent_symbols.is_empty() {
            query.patterns[pattern_entry.pattern_index as usize].is_non_local = true;
        }
        for &symbol in &analysis.finished_parent_symbols {
            analysis_insert_symbol(&mut query.repeat_symbols_with_rootless_patterns, symbol);
        }
    }

    // Vec ownership reclaims all subgraphs, worklists, and pooled state values.
    result
}

fn insert_subgraph(subgraphs: &mut AnalysisSubgraphArray, symbol: Symbol) {
    let (index, exists) = analysis_search(subgraphs, |subgraph| subgraph.symbol.cmp(&symbol));
    if !exists {
        subgraphs.insert(
            index,
            AnalysisSubgraph {
                symbol,
                ..AnalysisSubgraph::default()
            },
        );
    }
}

fn initial_analysis_state(step_index: u16, parse_state: StateId, symbol: Symbol) -> AnalysisState {
    let mut state = AnalysisState {
        step_index,
        depth: 1,
        root_symbol: symbol,
        ..AnalysisState::default()
    };
    state.stack[0] = AnalysisStateEntry {
        parse_state,
        parent_symbol: symbol,
        ..AnalysisStateEntry::default()
    };
    state
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn array_search_selects_last_equal_offset() {
        let offsets = [
            StepOffset {
                step_index: 1,
                byte_offset: 4,
            },
            StepOffset {
                step_index: 3,
                byte_offset: 8,
            },
            StepOffset {
                step_index: 3,
                byte_offset: 12,
            },
            StepOffset {
                step_index: 5,
                byte_offset: 20,
            },
        ];
        assert_eq!(
            analysis_search(&offsets, |entry| entry.step_index.cmp(&3)),
            (2, true)
        );
        assert_eq!(
            analysis_search(&offsets, |entry| entry.step_index.cmp(&2)),
            (1, false)
        );
        assert_eq!(
            analysis_search(&offsets, |entry| entry.step_index.cmp(&0)),
            (0, false)
        );
        assert_eq!(
            analysis_search(&offsets, |entry| entry.step_index.cmp(&6)),
            (4, false)
        );
        assert_eq!(
            analysis_search::<u16>(&[], |entry| entry.cmp(&0)),
            (0, false)
        );
    }

    #[test]
    fn recorded_symbols_and_steps_are_sorted_and_unique() {
        let mut symbols = Vec::new();
        for symbol in [4, 1, 4, 3, 8, 0, 3, 8] {
            analysis_insert_symbol(&mut symbols, symbol);
        }
        assert_eq!(symbols, [0, 1, 3, 4, 8]);
    }

    #[test]
    fn subgraph_insertion_preserves_existing_graph() {
        let mut subgraphs = Vec::new();
        insert_subgraph(&mut subgraphs, 7);
        subgraphs[0].start_states.push(11);
        insert_subgraph(&mut subgraphs, 3);
        insert_subgraph(&mut subgraphs, 7);
        assert_eq!(subgraphs.len(), 2);
        assert_eq!(subgraphs[0].symbol, 3);
        assert_eq!(subgraphs[1].symbol, 7);
        assert_eq!(subgraphs[1].start_states, [11]);
    }

    #[test]
    fn initial_state_has_one_unfinished_parent() {
        let state = initial_analysis_state(12, 34, 56);
        assert_eq!(state.step_index, 12);
        assert_eq!(state.depth, 1);
        assert_eq!(state.root_symbol, 56);
        assert_eq!(state.stack[0].parse_state, 34);
        assert_eq!(state.stack[0].parent_symbol, 56);
        assert_eq!(state.stack[0].child_index, 0);
        assert_eq!(state.stack[0].field_id, 0);
        assert!(!state.stack[0].done);
    }
}
