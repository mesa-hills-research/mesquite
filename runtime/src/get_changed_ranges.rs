use crate::{
    error_costs::ERROR_STATE,
    language::{Language, ts_language_alias_at, ts_language_symbol_name},
    length::*,
    subtree::*,
    tree_cursor::{TreeCursor, TreeCursorEntry},
    types::*,
};

pub(crate) struct RangeIterator<'tree> {
    pub cursor: TreeCursor<'tree>,
    pub language: Language,
    pub visible_depth: u32,
    pub in_padding: bool,
    pub prev_external_token: Subtree,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum IteratorComparison {
    Matches,
    MayDiffer,
    Differs,
}

pub(crate) fn ts_range_array_add(ranges: &mut Vec<Range>, start: Length, end: Length) {
    if let Some(last_range) = ranges.last_mut()
        && start.bytes <= last_range.end_byte
    {
        // The caller supplies ranges in traversal order. As in C, replace the
        // end (rather than taking a maximum), even for an empty new interval.
        last_range.end_byte = end.bytes;
        last_range.end_point = end.extent;
        return;
    }

    if start.bytes < end.bytes {
        ranges.push(Range {
            start_point: start.extent,
            end_point: end.extent,
            start_byte: start.bytes,
            end_byte: end.bytes,
        });
    }
}

pub(crate) fn ts_range_array_intersects(
    ranges: &[Range],
    start_index: u32,
    start_byte: u32,
    end_byte: u32,
) -> bool {
    for range in ranges.iter().skip(start_index as usize) {
        if range.end_byte > start_byte {
            if range.start_byte >= end_byte {
                break;
            }
            return true;
        }
    }
    false
}

pub(crate) fn ts_range_array_get_changed_ranges(
    old_ranges: &[Range],
    new_ranges: &[Range],
    differences: &mut Vec<Range>,
) {
    let mut new_index = 0;
    let mut old_index = 0;
    let mut current_position = length_zero();
    let mut in_old_range = false;
    let mut in_new_range = false;

    while old_index < old_ranges.len() || new_index < new_ranges.len() {
        let next_old_position = range_boundary(old_ranges.get(old_index), in_old_range);
        let next_new_position = range_boundary(new_ranges.get(new_index), in_new_range);

        match next_old_position.bytes.cmp(&next_new_position.bytes) {
            std::cmp::Ordering::Less => {
                if in_old_range != in_new_range {
                    ts_range_array_add(differences, current_position, next_old_position);
                }
                if in_old_range {
                    old_index += 1;
                }
                current_position = next_old_position;
                in_old_range = !in_old_range;
            }
            std::cmp::Ordering::Greater => {
                if in_old_range != in_new_range {
                    ts_range_array_add(differences, current_position, next_new_position);
                }
                if in_new_range {
                    new_index += 1;
                }
                current_position = next_new_position;
                in_new_range = !in_new_range;
            }
            std::cmp::Ordering::Equal => {
                if in_old_range != in_new_range {
                    ts_range_array_add(differences, current_position, next_new_position);
                }
                if in_old_range {
                    old_index += 1;
                }
                if in_new_range {
                    new_index += 1;
                }
                in_old_range = !in_old_range;
                in_new_range = !in_new_range;
                current_position = next_new_position;
            }
        }
    }
}

fn range_boundary(range: Option<&Range>, inside: bool) -> Length {
    match range {
        Some(range) if inside => Length {
            bytes: range.end_byte,
            extent: range.end_point,
        },
        Some(range) => Length {
            bytes: range.start_byte,
            extent: range.start_point,
        },
        None => LENGTH_MAX,
    }
}

pub(crate) fn iterator_new<'tree>(
    mut cursor: TreeCursor<'tree>,
    tree: &'tree Subtree,
    language: &Language,
) -> RangeIterator<'tree> {
    cursor.stack.clear();
    cursor.stack.push(TreeCursorEntry {
        subtree: tree,
        position: length_zero(),
        child_index: 0,
        structural_child_index: 0,
        descendant_index: 0,
    });
    RangeIterator {
        cursor,
        language: *language,
        visible_depth: 1,
        in_padding: false,
        prev_external_token: Subtree::Null,
    }
}

pub(crate) fn iterator_done(iterator: &RangeIterator<'_>) -> bool {
    iterator.cursor.stack.is_empty()
}

pub(crate) fn iterator_start_position(iterator: &RangeIterator<'_>) -> Length {
    let entry = iterator
        .cursor
        .stack
        .last()
        .expect("nonempty range iterator");
    if iterator.in_padding {
        entry.position
    } else {
        length_add(entry.position, ts_subtree_padding(entry.subtree))
    }
}

pub(crate) fn iterator_end_position(iterator: &RangeIterator<'_>) -> Length {
    let entry = iterator
        .cursor
        .stack
        .last()
        .expect("nonempty range iterator");
    let result = length_add(entry.position, ts_subtree_padding(entry.subtree));
    if iterator.in_padding {
        result
    } else {
        length_add(result, ts_subtree_size(entry.subtree))
    }
}

pub(crate) fn iterator_tree_is_visible(iterator: &RangeIterator<'_>) -> bool {
    let stack = &iterator.cursor.stack;
    let entry = stack.last().expect("nonempty range iterator");
    if ts_subtree_visible(entry.subtree) {
        return true;
    }
    if stack.len() > 1 {
        let parent = stack[stack.len() - 2].subtree;
        return ts_language_alias_at(
            &iterator.language,
            ts_subtree_production_id(parent) as u32,
            entry.structural_child_index,
        ) != 0;
    }
    false
}

pub(crate) fn iterator_get_visible_state<'tree>(
    iterator: &RangeIterator<'tree>,
) -> Option<(&'tree Subtree, Symbol, u32)> {
    let stack = &iterator.cursor.stack;
    let mut count = stack.len();
    if iterator.in_padding {
        // Padding belongs to the visible ancestor, not the current node.
        count = count.saturating_sub(1);
    }
    let mut alias_symbol = 0;
    for i in (0..count).rev() {
        let entry = stack[i];
        if i > 0 {
            let parent = stack[i - 1].subtree;
            alias_symbol = ts_language_alias_at(
                &iterator.language,
                ts_subtree_production_id(parent) as u32,
                entry.structural_child_index,
            );
        }
        if ts_subtree_visible(entry.subtree) || alias_symbol != 0 {
            return Some((entry.subtree, alias_symbol, entry.position.bytes));
        }
    }
    None
}

pub(crate) fn iterator_ascend(iterator: &mut RangeIterator<'_>) {
    if iterator_done(iterator) {
        return;
    }
    if iterator_tree_is_visible(iterator) && !iterator.in_padding {
        iterator.visible_depth -= 1;
    }
    if iterator.cursor.stack.last().unwrap().child_index > 0 {
        iterator.in_padding = false;
    }
    iterator.cursor.stack.pop();
}

pub(crate) fn iterator_descend(iterator: &mut RangeIterator<'_>, goal_position: u32) -> bool {
    if iterator.in_padding {
        return false;
    }

    loop {
        let mut did_descend = false;
        let entry = *iterator
            .cursor
            .stack
            .last()
            .expect("nonempty range iterator");
        let mut position = entry.position;
        let mut structural_child_index = 0;
        for (i, child) in ts_subtree_children(entry.subtree).iter().enumerate() {
            let child_left = length_add(position, ts_subtree_padding(child));
            let child_right = length_add(child_left, ts_subtree_size(child));

            if child_right.bytes > goal_position {
                iterator.cursor.stack.push(TreeCursorEntry {
                    subtree: child,
                    position,
                    child_index: i as u32,
                    structural_child_index,
                    descendant_index: 0,
                });

                if iterator_tree_is_visible(iterator) {
                    if child_left.bytes > goal_position {
                        iterator.in_padding = true;
                    } else {
                        iterator.visible_depth += 1;
                    }
                    return true;
                }

                did_descend = true;
                break;
            }

            position = child_right;
            if !ts_subtree_extra(child) {
                structural_child_index += 1;
            }
            let last_external_token = ts_subtree_last_external_token(child);
            if !last_external_token.is_null() {
                iterator.prev_external_token = last_external_token;
            }
        }
        if !did_descend {
            return false;
        }
    }
}

pub(crate) fn iterator_advance(iterator: &mut RangeIterator<'_>) {
    if iterator.in_padding {
        iterator.in_padding = false;
        if iterator_tree_is_visible(iterator) {
            iterator.visible_depth += 1;
        } else {
            iterator_descend(iterator, 0);
        }
        return;
    }

    loop {
        if iterator_tree_is_visible(iterator) {
            iterator.visible_depth -= 1;
        }
        let entry = iterator
            .cursor
            .stack
            .pop()
            .expect("nonempty range iterator");
        if iterator_done(iterator) {
            return;
        }

        let parent = iterator.cursor.stack.last().unwrap().subtree;
        let child_index = entry.child_index + 1;
        let last_external_token = ts_subtree_last_external_token(entry.subtree);
        if !last_external_token.is_null() {
            iterator.prev_external_token = last_external_token;
        }
        if ts_subtree_child_count(parent) > child_index {
            let position = length_add(entry.position, ts_subtree_total_size(entry.subtree));
            let mut structural_child_index = entry.structural_child_index;
            if !ts_subtree_extra(entry.subtree) {
                structural_child_index += 1;
            }
            let next_child = &ts_subtree_children(parent)[child_index as usize];
            iterator.cursor.stack.push(TreeCursorEntry {
                subtree: next_child,
                position,
                child_index,
                structural_child_index,
                descendant_index: 0,
            });

            if iterator_tree_is_visible(iterator) {
                if ts_subtree_padding(next_child).bytes > 0 {
                    iterator.in_padding = true;
                } else {
                    iterator.visible_depth += 1;
                }
            } else {
                iterator_descend(iterator, 0);
            }
            break;
        }
    }
}

pub(crate) fn iterator_compare(
    old_iterator: &RangeIterator<'_>,
    new_iterator: &RangeIterator<'_>,
) -> IteratorComparison {
    let (old_tree, old_alias_symbol, old_start, new_tree, new_alias_symbol, new_start) = match (
        iterator_get_visible_state(old_iterator),
        iterator_get_visible_state(new_iterator),
    ) {
        (None, None) => return IteratorComparison::Matches,
        (None, Some(_)) | (Some(_), None) => return IteratorComparison::Differs,
        (Some((old_tree, old_alias, old_start)), Some((new_tree, new_alias, new_start))) => (
            old_tree, old_alias, old_start, new_tree, new_alias, new_start,
        ),
    };
    let old_symbol = ts_subtree_symbol(old_tree);
    let new_symbol = ts_subtree_symbol(new_tree);
    if old_alias_symbol != new_alias_symbol || old_symbol != new_symbol {
        return IteratorComparison::Differs;
    }

    let old_size = ts_subtree_size(old_tree).bytes;
    let new_size = ts_subtree_size(new_tree).bytes;
    let old_state = ts_subtree_parse_state(old_tree);
    let new_state = ts_subtree_parse_state(new_tree);
    let old_has_external_tokens = ts_subtree_has_external_tokens(old_tree);
    let new_has_external_tokens = ts_subtree_has_external_tokens(new_tree);
    let old_error_cost = ts_subtree_error_cost(old_tree);
    let new_error_cost = ts_subtree_error_cost(new_tree);

    if old_start != new_start
        || old_symbol == ts_port_tables::BUILTIN_SYM_ERROR
        || old_size != new_size
        || old_state == TS_TREE_STATE_NONE
        || new_state == TS_TREE_STATE_NONE
        || ((old_state == ERROR_STATE) != (new_state == ERROR_STATE))
        || old_error_cost != new_error_cost
        || old_has_external_tokens != new_has_external_tokens
        || ts_subtree_has_changes(old_tree)
        || (old_has_external_tokens
            && !ts_subtree_external_scanner_state_eq(
                &old_iterator.prev_external_token,
                &new_iterator.prev_external_token,
            ))
    {
        IteratorComparison::MayDiffer
    } else {
        IteratorComparison::Matches
    }
}

pub(crate) fn iterator_print_state(iterator: &RangeIterator<'_>) {
    let entry = iterator
        .cursor
        .stack
        .last()
        .expect("nonempty range iterator");
    let start = iterator_start_position(iterator).extent;
    let end = iterator_end_position(iterator).extent;
    let name = ts_language_symbol_name(&iterator.language, ts_subtree_symbol(entry.subtree))
        .unwrap_or("(null)");
    print!(
        "({:<25} {}\t depth:{} [{}, {}] - [{}, {}])",
        name,
        if iterator.in_padding { "(p)" } else { "   " },
        iterator.visible_depth,
        start.row,
        start.column,
        end.row,
        end.column,
    );
}

pub(crate) fn ts_subtree_get_changed_ranges<'old, 'new>(
    old_tree: &'old Subtree,
    new_tree: &'new Subtree,
    cursor1: &mut TreeCursor<'old>,
    cursor2: &mut TreeCursor<'new>,
    language: &Language,
    included_range_differences: &[Range],
) -> Vec<Range> {
    let mut results = Vec::new();
    let mut old_iter = iterator_new(take_cursor(cursor1), old_tree, language);
    let mut new_iter = iterator_new(take_cursor(cursor2), new_tree, language);
    let mut included_range_difference_index = 0;

    let mut position = iterator_start_position(&old_iter);
    let mut next_position = iterator_start_position(&new_iter);
    if position.bytes < next_position.bytes {
        ts_range_array_add(&mut results, position, next_position);
        position = next_position;
    } else if position.bytes > next_position.bytes {
        ts_range_array_add(&mut results, next_position, position);
        next_position = position;
    }

    loop {
        let mut comparison = iterator_compare(&old_iter, &new_iter);

        // Identical-looking subtrees can still contain text that has moved
        // into or out of the included ranges.
        if comparison == IteratorComparison::Matches
            && ts_range_array_intersects(
                included_range_differences,
                included_range_difference_index,
                position.bytes,
                iterator_end_position(&old_iter).bytes,
            )
        {
            comparison = IteratorComparison::MayDiffer;
        }

        let mut is_changed = false;
        match comparison {
            IteratorComparison::Matches => {
                next_position = iterator_end_position(&old_iter);
            }
            IteratorComparison::MayDiffer => {
                if iterator_descend(&mut old_iter, position.bytes) {
                    if !iterator_descend(&mut new_iter, position.bytes) {
                        is_changed = true;
                        next_position = iterator_end_position(&old_iter);
                    }
                    // If both descend, retain next_position and compare their
                    // children on the next iteration, just as in C.
                } else if iterator_descend(&mut new_iter, position.bytes) {
                    is_changed = true;
                    next_position = iterator_end_position(&new_iter);
                } else {
                    next_position = length_min(
                        iterator_end_position(&old_iter),
                        iterator_end_position(&new_iter),
                    );
                }
            }
            IteratorComparison::Differs => {
                is_changed = true;
                next_position = length_min(
                    iterator_end_position(&old_iter),
                    iterator_end_position(&new_iter),
                );
            }
        }

        // Catch up to the same byte position and visible depth.
        while !iterator_done(&old_iter)
            && iterator_end_position(&old_iter).bytes <= next_position.bytes
        {
            iterator_advance(&mut old_iter);
        }
        while !iterator_done(&new_iter)
            && iterator_end_position(&new_iter).bytes <= next_position.bytes
        {
            iterator_advance(&mut new_iter);
        }
        while old_iter.visible_depth > new_iter.visible_depth {
            iterator_ascend(&mut old_iter);
        }
        while new_iter.visible_depth > old_iter.visible_depth {
            iterator_ascend(&mut new_iter);
        }

        if is_changed {
            ts_range_array_add(&mut results, position, next_position);
        }
        position = next_position;

        // Avoid rescanning included-range differences already passed.
        while let Some(range) =
            included_range_differences.get(included_range_difference_index as usize)
        {
            if range.end_byte <= position.bytes {
                included_range_difference_index += 1;
            } else {
                break;
            }
        }
        if iterator_done(&old_iter) || iterator_done(&new_iter) {
            break;
        }
    }

    let old_size = ts_subtree_total_size(old_tree);
    let new_size = ts_subtree_total_size(new_tree);
    if old_size.bytes < new_size.bytes {
        ts_range_array_add(&mut results, old_size, new_size);
    } else if new_size.bytes < old_size.bytes {
        ts_range_array_add(&mut results, new_size, old_size);
    }

    *cursor1 = old_iter.cursor;
    *cursor2 = new_iter.cursor;
    results
}

// The C iterators copy cursor structs but share their arrays. Move the Vec into
// the owning Rust iterator instead, and restore it (including capacity) afterward.
fn take_cursor<'tree>(cursor: &mut TreeCursor<'tree>) -> TreeCursor<'tree> {
    TreeCursor {
        tree: cursor.tree,
        stack: std::mem::take(&mut cursor.stack),
        root_alias_symbol: cursor.root_alias_symbol,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::LazyLock;
    use ts_port_tables::LanguageTables;

    fn length(bytes: u32) -> Length {
        Length {
            bytes,
            extent: Point {
                row: bytes / 3,
                column: bytes % 3,
            },
        }
    }

    fn range(start: u32, end: u32) -> Range {
        Range {
            start_byte: start,
            end_byte: end,
            start_point: length(start).extent,
            end_point: length(end).extent,
        }
    }

    #[test]
    fn range_add_merges_touching_ranges_and_replaces_end() {
        let mut ranges = Vec::new();
        ts_range_array_add(&mut ranges, length(1), length(1));
        assert!(ranges.is_empty());
        ts_range_array_add(&mut ranges, length(1), length(4));
        ts_range_array_add(&mut ranges, length(4), length(7));
        ts_range_array_add(&mut ranges, length(9), length(11));
        assert_eq!(ranges, [range(1, 7), range(9, 11)]);

        // Do not turn this helper into a general interval union. C replaces
        // the last end unconditionally when the new start overlaps it.
        ts_range_array_add(&mut ranges, length(10), length(10));
        assert_eq!(ranges, [range(1, 7), range(9, 10)]);
        ts_range_array_add(&mut ranges, length(13), length(12));
        assert_eq!(ranges, [range(1, 7), range(9, 10)]);
    }

    #[test]
    fn range_intersections_preserve_half_open_boundaries_and_start_index() {
        let ranges = [range(2, 5), range(7, 10)];
        assert!(!ts_range_array_intersects(&ranges, 0, 0, 2));
        assert!(!ts_range_array_intersects(&ranges, 0, 5, 7));
        assert!(!ts_range_array_intersects(&ranges, 0, 10, 15));
        assert!(ts_range_array_intersects(&ranges, 0, 4, 7));
        assert!(ts_range_array_intersects(&ranges, 0, 5, 8));
        assert!(!ts_range_array_intersects(&ranges, 1, 2, 5));
        assert!(!ts_range_array_intersects(&ranges, 2, 0, 15));
        assert!(!ts_range_array_intersects(&ranges, u32::MAX, 0, 15));
        // C tests endpoints directly, including for an empty query interval.
        assert!(ts_range_array_intersects(&ranges, 0, 3, 3));
        assert!(!ts_range_array_intersects(&ranges, 0, 2, 2));
    }

    fn mask_ranges(mask: u32) -> Vec<Range> {
        let mut result = Vec::new();
        for i in 0..8 {
            if mask & (1 << i) != 0 {
                ts_range_array_add(&mut result, length(i), length(i + 1));
            }
        }
        result
    }

    #[test]
    fn included_range_differences_are_symmetric_difference() {
        for old in 0..256 {
            for new in 0..256 {
                let mut result = Vec::new();
                ts_range_array_get_changed_ranges(
                    &mask_ranges(old),
                    &mask_ranges(new),
                    &mut result,
                );
                assert_eq!(result, mask_ranges(old ^ new), "old={old} new={new}");
            }
        }
    }

    #[test]
    fn included_range_boundaries_can_be_empty_adjacent_and_unbounded() {
        let mut result = Vec::new();
        ts_range_array_get_changed_ranges(&[range(2, 2)], &[], &mut result);
        assert!(result.is_empty());
        ts_range_array_get_changed_ranges(&[range(1, 3), range(3, 5)], &[range(1, 5)], &mut result);
        assert!(result.is_empty());

        let entire_document = Range {
            start_byte: 0,
            start_point: Point::default(),
            end_byte: LENGTH_MAX.bytes,
            end_point: LENGTH_MAX.extent,
        };
        ts_range_array_get_changed_ranges(&[], &[entire_document], &mut result);
        assert_eq!(result, [entire_document]);
        result.clear();
        ts_range_array_get_changed_ranges(&[entire_document], &[], &mut result);
        assert_eq!(result, [entire_document]);
        result.clear();
        ts_range_array_get_changed_ranges(&[entire_document], &[entire_document], &mut result);
        assert!(result.is_empty());
    }

    #[test]
    fn included_range_differences_append_and_use_new_point_on_equal_bytes() {
        let mut new = range(1, 4);
        new.start_point = Point { row: 2, column: 0 };
        let mut result = vec![range(0, 1)];
        ts_range_array_get_changed_ranges(&[range(1, 3)], &[new], &mut result);
        assert_eq!(result, [range(0, 1), range(3, 4)]);

        result.clear();
        let mut new = range(4, 4);
        new.start_point = Point { row: 2, column: 0 };
        new.end_point = new.start_point;
        ts_range_array_get_changed_ranges(&[range(1, 4)], &[new], &mut result);
        let mut expected = range(1, 4);
        expected.end_point = new.end_point;
        assert_eq!(result, [expected]);
    }

    // No lexer or parse tables are needed to walk hand-built trees. Providing
    // just alias rows also makes structural-child index tests independent of
    // any generated grammar.
    fn language() -> Language {
        static TABLES: LazyLock<LanguageTables> = LazyLock::new(|| LanguageTables {
            abi_version: 15,
            name: None,
            metadata: None,
            symbol_count: 4,
            alias_count: 1,
            token_count: 2,
            external_token_count: 0,
            state_count: 3,
            large_state_count: 0,
            production_id_count: 3,
            field_count: 0,
            max_alias_sequence_length: 3,
            parse_table: vec![],
            small_parse_table: vec![],
            small_parse_table_map: vec![],
            parse_actions: vec![],
            symbol_names: ["end", "a", "b", "root", "alias"]
                .map(String::from)
                .to_vec(),
            field_names: vec![],
            field_map_slices: vec![],
            field_map_entries: vec![],
            symbol_metadata: vec![],
            public_symbol_map: vec![],
            alias_map: vec![],
            alias_sequences: vec![0, 0, 0, 0, 4, 0, 0, 0, 4],
            lex_modes: vec![],
            lex_fn: |_, _| unreachable!("changed-range tests do not lex"),
            keyword_lex_fn: None,
            keyword_capture_token: 0,
            external_scanner: None,
            primary_state_ids: vec![],
            reserved_words: vec![],
            max_reserved_word_set_size: 0,
            supertype_count: 0,
            supertype_symbols: vec![],
            supertype_map_slices: vec![],
            supertype_map_entries: vec![],
        });
        Language::from(&*TABLES)
    }

    fn cursor<'tree>() -> TreeCursor<'tree> {
        TreeCursor {
            tree: None,
            stack: Vec::with_capacity(8),
            root_alias_symbol: 0,
        }
    }

    fn leaf(symbol: u8, padding: u8, size: u8) -> Subtree {
        Subtree::Inline(InlineLeaf {
            symbol,
            parse_state: 1,
            flags: VISIBLE,
            padding_bytes: padding,
            padding_columns: padding,
            size_bytes: size,
            padding_rows_and_lookahead: 0,
        })
    }

    fn changed_ranges(old: &Subtree, new: &Subtree) -> Vec<Range> {
        ts_subtree_get_changed_ranges(old, new, &mut cursor(), &mut cursor(), &language(), &[])
    }

    fn flat_range(start: u32, end: u32) -> Range {
        Range {
            start_byte: start,
            end_byte: end,
            start_point: Point {
                row: 0,
                column: start,
            },
            end_point: Point {
                row: 0,
                column: end,
            },
        }
    }

    #[test]
    fn root_leaf_ranges_include_symbol_changes_padding_and_trailing_length() {
        assert!(changed_ranges(&leaf(1, 0, 4), &leaf(1, 0, 4)).is_empty());
        assert_eq!(
            changed_ranges(&leaf(1, 0, 4), &leaf(2, 0, 4)),
            [flat_range(0, 4)]
        );
        assert_eq!(
            changed_ranges(&leaf(1, 0, 3), &leaf(1, 0, 5)),
            [flat_range(3, 5)]
        );
        assert_eq!(
            changed_ranges(&leaf(1, 0, 5), &leaf(1, 0, 3)),
            [flat_range(3, 5)]
        );
        assert_eq!(
            changed_ranges(&leaf(1, 2, 5), &leaf(1, 4, 3)),
            [flat_range(2, 4)]
        );
        assert_eq!(
            changed_ranges(&leaf(1, 4, 3), &leaf(1, 2, 5)),
            [flat_range(2, 4)]
        );
        assert!(changed_ranges(&leaf(1, 0, 0), &leaf(2, 0, 0)).is_empty());
    }

    #[test]
    fn comparison_checks_old_changes_but_not_new_changes_or_nonerror_state_ids() {
        let old = leaf(1, 0, 4);
        let mut new = old.clone();
        if let Subtree::Inline(data) = &mut new {
            data.flags |= HAS_CHANGES;
            data.parse_state = 2;
        }
        let old_iter = iterator_new(cursor(), &old, &language());
        let new_iter = iterator_new(cursor(), &new, &language());
        assert_eq!(
            iterator_compare(&old_iter, &new_iter),
            IteratorComparison::Matches
        );
        assert_eq!(
            iterator_compare(&new_iter, &old_iter),
            IteratorComparison::MayDiffer
        );
        assert!(changed_ranges(&new, &old).is_empty());

        for state in [ERROR_STATE, TS_TREE_STATE_NONE] {
            let mut new = old.clone();
            if let Subtree::Inline(data) = &mut new {
                data.parse_state = state;
            }
            assert_eq!(
                iterator_compare(&old_iter, &iterator_new(cursor(), &new, &language())),
                IteratorComparison::MayDiffer
            );
        }
    }

    #[test]
    fn range_iterators_restore_scratch_cursor_allocations() {
        let old = leaf(1, 0, 4);
        let new = leaf(2, 0, 4);
        let mut cursor1 = cursor();
        let mut cursor2 = cursor();
        let storage1 = cursor1.stack.as_ptr();
        let storage2 = cursor2.stack.as_ptr();
        let result =
            ts_subtree_get_changed_ranges(&old, &new, &mut cursor1, &mut cursor2, &language(), &[]);
        assert_eq!(result, [flat_range(0, 4)]);
        assert!(cursor1.stack.is_empty());
        assert!(cursor2.stack.is_empty());
        assert_eq!(storage1, cursor1.stack.as_ptr());
        assert_eq!(storage2, cursor2.stack.as_ptr());
    }

    #[test]
    fn root_padding_has_no_visible_state_and_hidden_roots_match() {
        let old = leaf(1, 2, 4);
        let mut iterator = iterator_new(cursor(), &old, &language());
        iterator.in_padding = true;
        assert!(iterator_get_visible_state(&iterator).is_none());
        assert_eq!(iterator_start_position(&iterator), length_zero());
        assert_eq!(iterator_end_position(&iterator).bytes, 2);
        iterator.visible_depth = 0;
        iterator_advance(&mut iterator);
        assert_eq!(iterator.visible_depth, 1);
        assert_eq!(iterator_start_position(&iterator).bytes, 2);
        assert_eq!(iterator_end_position(&iterator).bytes, 6);
        iterator_ascend(&mut iterator);
        assert!(iterator_done(&iterator));
        assert_eq!(iterator.visible_depth, 0);
        iterator_ascend(&mut iterator);

        let hidden = Subtree::Inline(InlineLeaf::default());
        let hidden_iterator = iterator_new(cursor(), &hidden, &language());
        assert!(iterator_get_visible_state(&hidden_iterator).is_none());
        assert_eq!(
            iterator_compare(&hidden_iterator, &hidden_iterator),
            IteratorComparison::Matches
        );
        assert_eq!(
            iterator_compare(&hidden_iterator, &iterator_new(cursor(), &old, &language())),
            IteratorComparison::Differs
        );
    }
}
