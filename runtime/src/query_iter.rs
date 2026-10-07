//! query-6: see query.c and PORTING.md.
use super::*;

pub(crate) fn ts_query_cursor_next_match<'query, 'tree: 'query>(
    cursor: &mut QueryExecution<'query, 'tree>,
) -> Option<QueryMatchData<'tree>> {
    todo!("query-6: ts_query_cursor_next_match")
}

pub(crate) fn ts_query_cursor_remove_match(cursor: &mut QueryExecution<'_, '_>, match_id: u32) {
    todo!("query-6: ts_query_cursor_remove_match")
}

pub(crate) fn ts_query_cursor_next_capture<'query, 'tree: 'query>(
    cursor: &mut QueryExecution<'query, 'tree>,
) -> Option<(QueryMatchData<'tree>, u32)> {
    todo!("query-6: ts_query_cursor_next_capture")
}

pub(crate) fn ts_query_cursor_set_max_start_depth(cursor: &mut CursorConfig, max_start_depth: u32) {
    todo!("query-6: ts_query_cursor_set_max_start_depth")
}
