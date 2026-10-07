//! Recursive query-pattern parser, translated from query.c.
use super::*;
use crate::language::{
    ts_language_field_id_for_name, ts_language_subtypes, ts_language_symbol_for_name,
    ts_language_symbol_metadata,
};

pub(crate) fn ts_query__add_negated_fields(
    query: &mut CompiledQuery,
    step_index: u16,
    field_ids: &mut [FieldId],
) {
    // Lists are separated by zeroes. Reuse only an exact, ordered match.
    let mut failed_match = false;
    let mut match_count = 0;
    let mut start_index = 0;
    for (i, &existing_field_id) in query.negated_fields.iter().enumerate() {
        if existing_field_id == 0 {
            if match_count == field_ids.len() {
                query.steps[step_index as usize].negated_field_list_id = start_index as u16;
                return;
            }
            start_index = i + 1;
            match_count = 0;
            failed_match = false;
        } else if match_count < field_ids.len()
            && existing_field_id == field_ids[match_count]
            && !failed_match
        {
            match_count += 1;
        } else {
            match_count = 0;
            failed_match = true;
        }
    }

    query.steps[step_index as usize].negated_field_list_id = query.negated_fields.len() as u16;
    query.negated_fields.extend_from_slice(field_ids);
    query.negated_fields.push(0);
}

pub(crate) fn ts_query__parse_string_literal(
    query: &mut CompiledQuery,
    stream: &mut Stream<'_>,
) -> QueryErrorCode {
    let string_start = stream.input;
    if stream.next != '"' as i32 {
        return QueryErrorCode::Syntax;
    }
    stream_advance(stream);
    let mut prev_position = stream.input;

    let mut is_escaped = false;
    query.string_buffer.clear();
    loop {
        if is_escaped {
            is_escaped = false;
            match stream.next {
                c if c == 'n' as i32 => query.string_buffer.push(b'\n'),
                c if c == 'r' as i32 => query.string_buffer.push(b'\r'),
                c if c == 't' as i32 => query.string_buffer.push(b'\t'),
                c if c == '0' as i32 => query.string_buffer.push(0),
                _ => query.string_buffer.extend_from_slice(
                    &stream.source[stream.input..stream.input + stream.next_size as usize],
                ),
            }
            prev_position = stream.input + stream.next_size as usize;
        } else if stream.next == '\\' as i32 {
            query
                .string_buffer
                .extend_from_slice(&stream.source[prev_position..stream.input]);
            prev_position = stream.input + 1;
            is_escaped = true;
        } else if stream.next == '"' as i32 {
            query
                .string_buffer
                .extend_from_slice(&stream.source[prev_position..stream.input]);
            stream_advance(stream);
            return QueryErrorCode::None;
        } else if stream.next == '\n' as i32 {
            stream_reset(stream, string_start);
            return QueryErrorCode::Syntax;
        }
        if !stream_advance(stream) {
            stream_reset(stream, string_start);
            return QueryErrorCode::Syntax;
        }
    }
}

/// Predicates are arbitrary S-expressions; their semantics belong to the binding.
pub(crate) fn ts_query__parse_predicate(
    query: &mut CompiledQuery,
    stream: &mut Stream<'_>,
) -> QueryErrorCode {
    if !stream_is_ident_start(stream) {
        return QueryErrorCode::Syntax;
    }
    let predicate_name = stream.input;
    stream_scan_identifier(stream);
    let id = symbol_table_insert_name(
        &mut query.predicate_values,
        &stream.source[predicate_name..stream.input],
    );
    query.predicate_steps.push(PredicateStep {
        kind: PredicateStepKind::String,
        value_id: u32::from(id),
    });
    stream_skip_whitespace(stream);

    loop {
        if stream.next == ')' as i32 {
            stream_advance(stream);
            stream_skip_whitespace(stream);
            query.predicate_steps.push(PredicateStep {
                kind: PredicateStepKind::Done,
                value_id: 0,
            });
            break;
        } else if stream.next == '@' as i32 {
            stream_advance(stream);
            if !stream_is_ident_start(stream) {
                return QueryErrorCode::Syntax;
            }
            let capture_name = stream.input;
            stream_scan_identifier(stream);
            let capture_id = symbol_table_id_for_name(
                &query.captures,
                &stream.source[capture_name..stream.input],
            );
            if capture_id == -1 {
                stream_reset(stream, capture_name);
                return QueryErrorCode::Capture;
            }
            query.predicate_steps.push(PredicateStep {
                kind: PredicateStepKind::Capture,
                value_id: capture_id as u32,
            });
        } else if stream.next == '"' as i32 {
            let error = ts_query__parse_string_literal(query, stream);
            if error != QueryErrorCode::None {
                return error;
            }
            let id = symbol_table_insert_name(&mut query.predicate_values, &query.string_buffer);
            query.predicate_steps.push(PredicateStep {
                kind: PredicateStepKind::String,
                value_id: u32::from(id),
            });
        } else if stream_is_ident_start(stream) {
            let symbol_start = stream.input;
            stream_scan_identifier(stream);
            let id = symbol_table_insert_name(
                &mut query.predicate_values,
                &stream.source[symbol_start..stream.input],
            );
            query.predicate_steps.push(PredicateStep {
                kind: PredicateStepKind::String,
                value_id: u32::from(id),
            });
        } else {
            return QueryErrorCode::Syntax;
        }
        stream_skip_whitespace(stream);
    }
    QueryErrorCode::None
}

/// Each recursive call has its own quantifier buffer, combined into its caller's
/// buffer only after that subpattern succeeds.
pub(crate) fn ts_query__parse_pattern(
    query: &mut CompiledQuery,
    stream: &mut Stream<'_>,
    depth: u32,
    is_immediate: bool,
    capture_quantifiers: &mut CaptureQuantifiers,
) -> QueryErrorCode {
    if stream.next == 0 {
        return QueryErrorCode::Syntax;
    }
    if stream.next == ')' as i32 || stream.next == ']' as i32 {
        return PARENT_DONE;
    }

    let starting_step_index = query.steps.len() as u32;
    if query
        .step_offsets
        .last()
        .is_none_or(|offset| u32::from(offset.step_index) != starting_step_index)
    {
        query.step_offsets.push(StepOffset {
            step_index: starting_step_index as u16,
            byte_offset: stream_offset(stream),
        });
    }

    if stream.next == '[' as i32 {
        stream_advance(stream);
        stream_skip_whitespace(stream);

        // Put a placeholder between each pair of alternation branches.
        let mut branch_step_indices = Vec::new();
        let mut branch_capture_quantifiers = capture_quantifiers_new();
        loop {
            let start_index = query.steps.len() as u32;
            let mut error = ts_query__parse_pattern(
                query,
                stream,
                depth,
                is_immediate,
                &mut branch_capture_quantifiers,
            );
            if error == PARENT_DONE {
                if stream.next == ']' as i32 && !branch_step_indices.is_empty() {
                    stream_advance(stream);
                    break;
                }
                error = QueryErrorCode::Syntax;
            }
            if error != QueryErrorCode::None {
                return error;
            }

            if start_index == starting_step_index {
                capture_quantifiers_replace(capture_quantifiers, &branch_capture_quantifiers);
            } else {
                capture_quantifiers_join_all(capture_quantifiers, &branch_capture_quantifiers);
            }
            branch_step_indices.push(start_index);
            query.steps.push(query_step__new(0, depth as u16, false));
            capture_quantifiers_clear(&mut branch_capture_quantifiers);
        }
        query.steps.pop();

        // Link each branch to the next, and its placeholder to the overall end.
        for indices in branch_step_indices.windows(2) {
            let step_index = indices[0] as usize;
            let next_step_index = indices[1] as usize;
            query.steps[step_index].alternative_index = next_step_index as u16;
            let end_index = query.steps.len() as u16;
            let end_step = &mut query.steps[next_step_index - 1];
            end_step.alternative_index = end_index;
            end_step.is_dead_end = true;
        }
    } else if stream.next == '(' as i32 {
        stream_advance(stream);
        stream_skip_whitespace(stream);

        // An opening node, string, or alternation makes this a grouped sequence.
        if stream.next == '(' as i32 || stream.next == '"' as i32 || stream.next == '[' as i32 {
            let mut child_is_immediate = is_immediate;
            let mut child_capture_quantifiers = capture_quantifiers_new();
            loop {
                if stream.next == '.' as i32 {
                    child_is_immediate = true;
                    stream_advance(stream);
                    stream_skip_whitespace(stream);
                }
                let mut error = ts_query__parse_pattern(
                    query,
                    stream,
                    depth,
                    child_is_immediate,
                    &mut child_capture_quantifiers,
                );
                if error == PARENT_DONE {
                    if stream.next == ')' as i32 {
                        stream_advance(stream);
                        break;
                    }
                    error = QueryErrorCode::Syntax;
                }
                if error != QueryErrorCode::None {
                    return error;
                }
                capture_quantifiers_add_all(capture_quantifiers, &child_capture_quantifiers);
                capture_quantifiers_clear(&mut child_capture_quantifiers);
                child_is_immediate = false;
            }
        } else if stream.next == '.' as i32 || stream.next == '#' as i32 {
            stream_advance(stream);
            return ts_query__parse_predicate(query, stream);
        } else {
            // A named node, a named wildcard, or a missing-node pattern.
            let symbol;
            let mut is_missing = false;
            let node_name = stream.input;
            if stream_is_ident_start(stream) {
                stream_scan_identifier(stream);
                let name = &stream.source[node_name..stream.input];
                if name == b"_" {
                    symbol = WILDCARD_SYMBOL;
                } else if b"MISSING".starts_with(name) {
                    // C's strncmp uses the parsed name length, accepting prefixes.
                    is_missing = true;
                    stream_skip_whitespace(stream);
                    if stream_is_ident_start(stream) {
                        let missing_node_name = stream.input;
                        stream_scan_identifier(stream);
                        symbol = ts_language_symbol_for_name(
                            &query.language,
                            &stream.source[missing_node_name..stream.input],
                            true,
                        );
                        if symbol == 0 {
                            stream_reset(stream, missing_node_name);
                            return QueryErrorCode::NodeType;
                        }
                    } else if stream.next == '"' as i32 {
                        let string_start = stream.input;
                        let error = ts_query__parse_string_literal(query, stream);
                        if error != QueryErrorCode::None {
                            return error;
                        }
                        symbol = ts_language_symbol_for_name(
                            &query.language,
                            &query.string_buffer,
                            false,
                        );
                        if symbol == 0 {
                            stream_reset(stream, string_start + 1);
                            return QueryErrorCode::NodeType;
                        }
                    } else if stream.next == ')' as i32 {
                        symbol = WILDCARD_SYMBOL;
                    } else {
                        stream_reset(stream, stream.input);
                        return QueryErrorCode::Syntax;
                    }
                } else {
                    symbol = ts_language_symbol_for_name(&query.language, name, true);
                    if symbol == 0 {
                        stream_reset(stream, node_name);
                        return QueryErrorCode::NodeType;
                    }
                }
            } else {
                return QueryErrorCode::Syntax;
            }

            query
                .steps
                .push(query_step__new(symbol, depth as u16, is_immediate));
            let step = query.steps.last_mut().unwrap();
            if ts_language_symbol_metadata(&query.language, symbol).supertype {
                step.supertype_symbol = step.symbol;
                step.symbol = WILDCARD_SYMBOL;
            }
            if is_missing {
                step.is_missing = true;
            }
            if symbol == WILDCARD_SYMBOL {
                step.is_named = true;
            }
            stream_skip_whitespace(stream);

            if stream.next == '/' as i32 {
                if step.supertype_symbol == 0 {
                    stream_reset(stream, node_name - 1);
                    return QueryErrorCode::Structure;
                }
                stream_advance(stream);
                if !stream_is_ident_start(stream) {
                    return QueryErrorCode::Syntax;
                }
                let subtype_node_name = stream.input;
                stream_scan_identifier(stream);
                step.symbol = ts_language_symbol_for_name(
                    &query.language,
                    &stream.source[subtype_node_name..stream.input],
                    true,
                );
                if step.symbol == 0 {
                    stream_reset(stream, subtype_node_name);
                    return QueryErrorCode::NodeType;
                }
                // ABI 15 adds the explicit supertype-to-subtype map.
                if query.language.tables.abi_version >= 15
                    && !ts_language_subtypes(&query.language, step.supertype_symbol)
                        .contains(&step.symbol)
                {
                    stream_reset(stream, node_name - 1);
                    return QueryErrorCode::Structure;
                }
                stream_skip_whitespace(stream);
            }

            let mut child_is_immediate = false;
            let mut last_child_step_index = 0u16;
            let mut negated_field_count = 0;
            let mut negated_field_ids = [0; MAX_NEGATED_FIELD_COUNT];
            let mut child_capture_quantifiers = capture_quantifiers_new();
            loop {
                if stream.next == '!' as i32 {
                    stream_advance(stream);
                    stream_skip_whitespace(stream);
                    if !stream_is_ident_start(stream) {
                        return QueryErrorCode::Syntax;
                    }
                    let field_name = stream.input;
                    stream_scan_identifier(stream);
                    let field_name_end = stream.input;
                    stream_skip_whitespace(stream);
                    let field_id = ts_language_field_id_for_name(
                        &query.language,
                        &stream.source[field_name..field_name_end],
                    );
                    if field_id == 0 {
                        // Unlike stream_reset, C changes only the error position.
                        stream.input = field_name;
                        return QueryErrorCode::Field;
                    }
                    // Despite C's "sorted" comment, insertion order is retained.
                    if negated_field_count < MAX_NEGATED_FIELD_COUNT {
                        negated_field_ids[negated_field_count] = field_id;
                        negated_field_count += 1;
                    }
                    continue;
                }

                if stream.next == '.' as i32 {
                    child_is_immediate = true;
                    stream_advance(stream);
                    stream_skip_whitespace(stream);
                }
                let mut step_index = query.steps.len() as u16;
                let mut error = ts_query__parse_pattern(
                    query,
                    stream,
                    depth.wrapping_add(1),
                    child_is_immediate,
                    &mut child_capture_quantifiers,
                );
                // A predicate adds no steps. Point back at the preceding step.
                if usize::from(step_index) == query.steps.len() {
                    step_index = step_index.wrapping_sub(1);
                }
                if error == PARENT_DONE {
                    if stream.next == ')' as i32 {
                        if child_is_immediate {
                            if last_child_step_index == 0 {
                                return QueryErrorCode::Syntax;
                            }
                            // Mark the last child and every one of its alternatives.
                            let mut index = last_child_step_index;
                            loop {
                                let step = &mut query.steps[index as usize];
                                step.is_last_child = true;
                                let alternative_index = step.alternative_index;
                                if alternative_index != NONE
                                    && usize::from(alternative_index) < query.steps.len()
                                {
                                    index = alternative_index;
                                } else {
                                    break;
                                }
                            }
                        }
                        if negated_field_count > 0 {
                            ts_query__add_negated_fields(
                                query,
                                starting_step_index as u16,
                                &mut negated_field_ids[..negated_field_count],
                            );
                        }
                        stream_advance(stream);
                        break;
                    }
                    error = QueryErrorCode::Syntax;
                }
                if error != QueryErrorCode::None {
                    return error;
                }
                capture_quantifiers_add_all(capture_quantifiers, &child_capture_quantifiers);
                last_child_step_index = step_index;
                child_is_immediate = false;
                capture_quantifiers_clear(&mut child_capture_quantifiers);
            }
        }
    } else if stream.next == '_' as i32 {
        stream_advance(stream);
        stream_skip_whitespace(stream);
        query
            .steps
            .push(query_step__new(WILDCARD_SYMBOL, depth as u16, is_immediate));
    } else if stream.next == '"' as i32 {
        let string_start = stream.input;
        let error = ts_query__parse_string_literal(query, stream);
        if error != QueryErrorCode::None {
            return error;
        }
        let symbol = ts_language_symbol_for_name(&query.language, &query.string_buffer, false);
        if symbol == 0 {
            stream_reset(stream, string_start + 1);
            return QueryErrorCode::NodeType;
        }
        query
            .steps
            .push(query_step__new(symbol, depth as u16, is_immediate));
    } else if stream_is_ident_start(stream) {
        let field_name = stream.input;
        stream_scan_identifier(stream);
        let field_name_end = stream.input;
        stream_skip_whitespace(stream);
        if stream.next != ':' as i32 {
            stream_reset(stream, field_name);
            return QueryErrorCode::Syntax;
        }
        stream_advance(stream);
        stream_skip_whitespace(stream);

        let mut field_capture_quantifiers = capture_quantifiers_new();
        let error = ts_query__parse_pattern(
            query,
            stream,
            depth,
            is_immediate,
            &mut field_capture_quantifiers,
        );
        if error != QueryErrorCode::None {
            return if error == PARENT_DONE {
                QueryErrorCode::Syntax
            } else {
                error
            };
        }
        let field_id = ts_language_field_id_for_name(
            &query.language,
            &stream.source[field_name..field_name_end],
        );
        if field_id == 0 {
            stream.input = field_name;
            return QueryErrorCode::Field;
        }
        let mut step_index = starting_step_index as usize;
        loop {
            let step = &mut query.steps[step_index];
            step.field = field_id;
            let alternative_index = step.alternative_index;
            if alternative_index != NONE
                && usize::from(alternative_index) > step_index
                && usize::from(alternative_index) < query.steps.len()
            {
                step_index = alternative_index as usize;
            } else {
                break;
            }
        }
        capture_quantifiers_add_all(capture_quantifiers, &field_capture_quantifiers);
    } else {
        return QueryErrorCode::Syntax;
    }

    stream_skip_whitespace(stream);

    let mut quantifier = CaptureQuantifier::One;
    loop {
        if stream.next == '+' as i32 {
            quantifier = quantifier_join(CaptureQuantifier::OneOrMore, quantifier);
            stream_advance(stream);
            stream_skip_whitespace(stream);
            let mut repeat_step = query_step__new(WILDCARD_SYMBOL, depth as u16, false);
            repeat_step.alternative_index = starting_step_index as u16;
            repeat_step.is_pass_through = true;
            repeat_step.alternative_is_immediate = true;
            query.steps.push(repeat_step);
        } else if stream.next == '*' as i32 {
            quantifier = quantifier_join(CaptureQuantifier::ZeroOrMore, quantifier);
            stream_advance(stream);
            stream_skip_whitespace(stream);
            let mut repeat_step = query_step__new(WILDCARD_SYMBOL, depth as u16, false);
            repeat_step.alternative_index = starting_step_index as u16;
            repeat_step.is_pass_through = true;
            repeat_step.alternative_is_immediate = true;
            query.steps.push(repeat_step);

            // Stop before the repeat step just pushed, or at an absent alternative.
            let mut step_index = starting_step_index as usize;
            loop {
                let alternative_index = query.steps[step_index].alternative_index;
                if alternative_index != NONE
                    && usize::from(alternative_index) < query.steps.len() - 1
                {
                    step_index = alternative_index as usize;
                } else {
                    break;
                }
            }
            query.steps[step_index].alternative_index = query.steps.len() as u16;
        } else if stream.next == '?' as i32 {
            quantifier = quantifier_join(CaptureQuantifier::ZeroOrOne, quantifier);
            stream_advance(stream);
            stream_skip_whitespace(stream);
            let mut step_index = starting_step_index as usize;
            loop {
                let alternative_index = query.steps[step_index].alternative_index;
                if alternative_index != NONE && usize::from(alternative_index) < query.steps.len() {
                    step_index = alternative_index as usize;
                } else {
                    break;
                }
            }
            query.steps[step_index].alternative_index = query.steps.len() as u16;
        } else if stream.next == '@' as i32 {
            stream_advance(stream);
            if !stream_is_ident_start(stream) {
                return QueryErrorCode::Syntax;
            }
            let capture_name = stream.input;
            stream_scan_identifier(stream);
            let capture_name_end = stream.input;
            stream_skip_whitespace(stream);
            let capture_id = symbol_table_insert_name(
                &mut query.captures,
                &stream.source[capture_name..capture_name_end],
            );
            capture_quantifiers_add_for_id(capture_quantifiers, capture_id, CaptureQuantifier::One);

            let mut step_index = starting_step_index as usize;
            loop {
                let step = &mut query.steps[step_index];
                query_step__add_capture(step, capture_id);
                let alternative_index = step.alternative_index;
                if alternative_index != NONE
                    && usize::from(alternative_index) > step_index
                    && usize::from(alternative_index) < query.steps.len()
                {
                    step_index = alternative_index as usize;
                } else {
                    break;
                }
            }
        } else {
            break;
        }
    }
    capture_quantifiers_mul(capture_quantifiers, quantifier);
    QueryErrorCode::None
}

#[cfg(test)]
#[path = "query_parse/tests.rs"]
mod tests;
