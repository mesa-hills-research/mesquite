//! Compile-only clients of the official query API. The skeleton does not execute
//! queries yet; the one runnable test checks required automatic thread traits.
#![allow(dead_code)]

use ts_port::{
    CaptureQuantifier, Language, Node, Point, Query, QueryCapture, QueryCaptures, QueryCursor,
    QueryCursorOptions, QueryCursorState, QueryError, QueryErrorKind, QueryMatch, QueryMatches,
    QueryPredicate, QueryPredicateArg, QueryProperty, StreamingIterator, StreamingIteratorMut,
    TextProvider,
};

#[test]
fn query_and_cursor_are_send_sync_without_unsafe_impls() {
    fn require<T: Send + Sync>() {}
    require::<Query>();
    require::<QueryCursor>();
}

fn metadata(language: &Language) -> Result<(), QueryError> {
    let mut query = Query::new(language, "(identifier) @name")?;
    let _: Vec<&str> = query.capture_names();
    let _: &[CaptureQuantifier] = query.capture_quantifiers(0);
    let _: usize = query.pattern_count();
    let _: usize = query.start_byte_for_pattern(0);
    let _: usize = query.end_byte_for_pattern(0);
    let _: &[QueryPredicate] = query.general_predicates(0);
    let _: &[QueryProperty] = query.property_settings(0);
    let _: &[(QueryProperty, bool)] = query.property_predicates(0);
    let _: Option<u32> = query.capture_index_for_name("name");
    let _: bool = query.is_pattern_rooted(0);
    let _: bool = query.is_pattern_non_local(0);
    let _: bool = query.is_pattern_guaranteed_at_step(0);
    query.disable_capture("name");
    query.disable_pattern(0);
    let _ = QueryProperty::new("local", Some("true"), Some(0));
    let _ = QueryPredicateArg::Capture(0);
    let _ = QueryPredicateArg::String("value".into());
    let error = QueryError {
        row: 0,
        column: 0,
        offset: 0,
        message: String::new(),
        kind: QueryErrorKind::Syntax,
    };
    let _: &dyn std::error::Error = &error;
    Ok(())
}

fn matches_and_captures(query: &Query, root: Node<'_>, source: &[u8]) {
    let mut cursor = QueryCursor::new();
    cursor.set_match_limit(128);
    let _: u32 = cursor.match_limit();
    cursor
        .set_byte_range(0..source.len())
        .set_point_range(Point::new(0, 0)..Point::new(10, 0))
        .set_max_start_depth(Some(8));
    {
        let mut matches = cursor.matches(query, root, source);
        matches.set_byte_range(0..source.len());
        matches.set_point_range(Point::new(0, 0)..Point::new(10, 0));
        while let Some(m) = matches.next() {
            let _: usize = m.pattern_index;
            let _: &[QueryCapture<'_>] = &m.captures;
            let _: u32 = m.id();
            let _: Vec<_> = m.nodes_for_capture_index(0).collect();
            let _ = format!("{m:?}");
            m.remove();
        }
        let _: Option<&mut QueryMatch<'_, '_>> = matches.get_mut();
    }
    {
        let mut captures = cursor.captures(query, root, source);
        captures.set_byte_range(0..source.len());
        captures.set_point_range(Point::new(0, 0)..Point::new(10, 0));
        while let Some((m, position)) = captures.next() {
            let c = m.captures[*position];
            let _: u32 = c.index;
            let _: Node<'_> = c.node;
        }
        let _: Option<&mut (QueryMatch<'_, '_>, usize)> = captures.get_mut();
    }
    let _: bool = cursor.did_exceed_match_limit();
}

fn with_progress(query: &Query, root: Node<'_>, source: &[u8]) {
    let mut cursor = QueryCursor::default();
    let mut count = 0;
    let mut callback = |state: &QueryCursorState| {
        let _: usize = state.current_byte_offset();
        count += 1;
        count == 3
    };
    {
        let options = QueryCursorOptions::new().progress_callback(&mut callback);
        let mut matches = cursor.matches_with_options(query, root, source, options);
        let _ = matches.next();
    }
    {
        let options = QueryCursorOptions {
            progress_callback: Some(&mut callback),
        };
        let mut captures = cursor.captures_with_options(query, root, source, options);
        let _ = captures.next();
    }
    let _: usize = count;
}

fn chunk_provider(query: &Query, root: Node<'_>, source: &[u8]) {
    let mut cursor = QueryCursor::new();
    let provider = |node: Node<'_>| {
        let range = node.byte_range();
        let midpoint = range.start + range.len() / 2;
        [&source[range.start..midpoint], &source[midpoint..range.end]].into_iter()
    };
    let mut matches = cursor.matches(query, root, provider);
    let _ = matches.next();
}

fn named_iterator_types<'query, 'tree: 'query, T: TextProvider<I>, I: AsRef<[u8]>>(
    matches: QueryMatches<'query, 'tree, T, I>,
    captures: QueryCaptures<'query, 'tree, T, I>,
) {
    fn check<S: StreamingIterator + StreamingIteratorMut>(_: S) {}
    check(matches);
    check(captures);
}

// Owned captures can outlive their iterator while remaining bounded by the tree.
fn save_captures<'query, 'tree: 'query>(
    cursor: &'query mut QueryCursor,
    query: &'query Query,
    root: Node<'tree>,
    source: &[u8],
) -> Vec<QueryCapture<'tree>> {
    let mut matches = cursor.matches(query, root, source);
    matches.next().unwrap().captures.clone()
}

fn move_match<'query, 'tree: 'query>(
    matches: &mut QueryMatches<'query, 'tree, &'tree [u8], &'tree [u8]>,
    replacement: QueryMatch<'query, 'tree>,
) -> QueryMatch<'query, 'tree> {
    std::mem::replace(matches.get_mut().unwrap(), replacement)
}

// The oracle reads the limit flag without an explicit iterator drop or nested
// scope. Like the binding, iterators must release borrows at their last use.
fn cursor_reborrow_after_last_use(query: &Query, root: Node<'_>, source: &[u8]) {
    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(query, root, source);
    while let Some(found) = matches.next() {
        for capture in &found.captures {
            let _ = capture.node.byte_range();
        }
    }
    let exceeded = cursor.did_exceed_match_limit();
    cursor.set_match_limit(32);
    let mut captures = cursor.captures(query, root, source);
    while let Some((found, index)) = captures.next() {
        let _ = found.captures[*index].node.byte_range();
    }
    let _: bool = exceeded || cursor.did_exceed_match_limit();
}

// Last use, not exhaustion, ends the borrow. Early abandonment and borrowed
// callbacks must also work without adding a scope or explicit drop.
fn options_reborrow_after_last_use(query: &Query, root: Node<'_>, source: &[u8]) {
    let mut cursor = QueryCursor::new();
    let mut calls = 0;
    let mut callback = |_: &QueryCursorState| {
        calls += 1;
        false
    };
    let mut matches = cursor.matches_with_options(
        query,
        root,
        source,
        QueryCursorOptions::new().progress_callback(&mut callback),
    );
    let _ = matches.next();
    let _: bool = cursor.did_exceed_match_limit();
    let mut captures = cursor.captures_with_options(
        query,
        root,
        source,
        QueryCursorOptions::new().progress_callback(&mut callback),
    );
    let _ = captures.next();
    let _: bool = cursor.did_exceed_match_limit();
    let _: usize = calls;
}
