//! query-5: see query.c and PORTING.md.
use super::*;

pub(crate) fn ts_query_cursor__capture<'query, 'tree: 'query>(
    cursor: &mut QueryExecution<'query, 'tree>,
    state_index: usize,
    step: &QueryStep,
    node: Node<'tree>,
) {
    todo!("query-5: ts_query_cursor__capture")
}

pub(crate) fn ts_query_cursor__copy_state(
    cursor: &mut QueryExecution<'_, '_>,
    state_index: usize,
) -> Option<usize> {
    todo!("query-5: ts_query_cursor__copy_state")
}

pub(crate) fn ts_query_cursor__should_descend(
    cursor: &QueryExecution<'_, '_>,
    node_intersects_range: bool,
) -> bool {
    todo!("query-5: ts_query_cursor__should_descend")
}

pub(crate) fn ts_query_cursor__advance(
    cursor: &mut QueryExecution<'_, '_>,
    stop_on_definite_step: bool,
) -> bool {
    todo!("query-5: ts_query_cursor__advance")
}
