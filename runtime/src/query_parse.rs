//! query-3: see query.c and PORTING.md.
use super::*;

pub(crate) fn ts_query__add_negated_fields(
    query: &mut CompiledQuery,
    step_index: u16,
    field_ids: &mut [FieldId],
) {
    todo!("query-3: ts_query__add_negated_fields")
}

pub(crate) fn ts_query__parse_string_literal(
    query: &mut CompiledQuery,
    stream: &mut Stream<'_>,
) -> QueryErrorCode {
    todo!("query-3: ts_query__parse_string_literal")
}

pub(crate) fn ts_query__parse_predicate(
    query: &mut CompiledQuery,
    stream: &mut Stream<'_>,
) -> QueryErrorCode {
    todo!("query-3: ts_query__parse_predicate")
}

pub(crate) fn ts_query__parse_pattern(
    query: &mut CompiledQuery,
    stream: &mut Stream<'_>,
    depth: u32,
    is_immediate: bool,
    capture_quantifiers: &mut CaptureQuantifiers,
) -> QueryErrorCode {
    todo!("query-3: ts_query__parse_pattern")
}
