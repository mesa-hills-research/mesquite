use crate::{
    language::Language, length::Length, subtree::Subtree, tree_cursor::TreeCursor, types::*,
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
    todo!("get_changed_ranges: ts_range_array_add")
}

pub(crate) fn ts_range_array_intersects(
    ranges: &[Range],
    start_index: u32,
    start_byte: u32,
    end_byte: u32,
) -> bool {
    todo!("get_changed_ranges: ts_range_array_intersects")
}

pub(crate) fn ts_range_array_get_changed_ranges(
    old_ranges: &[Range],
    new_ranges: &[Range],
    differences: &mut Vec<Range>,
) {
    todo!("get_changed_ranges: ts_range_array_get_changed_ranges")
}

pub(crate) fn iterator_new<'tree>(
    cursor: TreeCursor<'tree>,
    tree: &'tree Subtree,
    language: &Language,
) -> RangeIterator<'tree> {
    todo!("get_changed_ranges: iterator_new")
}

pub(crate) fn iterator_done(iterator: &RangeIterator<'_>) -> bool {
    todo!("get_changed_ranges: iterator_done")
}

pub(crate) fn iterator_start_position(iterator: &RangeIterator<'_>) -> Length {
    todo!("get_changed_ranges: iterator_start_position")
}

pub(crate) fn iterator_end_position(iterator: &RangeIterator<'_>) -> Length {
    todo!("get_changed_ranges: iterator_end_position")
}

pub(crate) fn iterator_tree_is_visible(iterator: &RangeIterator<'_>) -> bool {
    todo!("get_changed_ranges: iterator_tree_is_visible")
}

pub(crate) fn iterator_get_visible_state<'tree>(
    iterator: &RangeIterator<'tree>,
) -> Option<(&'tree Subtree, Symbol, u32)> {
    todo!("get_changed_ranges: iterator_get_visible_state")
}

pub(crate) fn iterator_ascend(iterator: &mut RangeIterator<'_>) {
    todo!("get_changed_ranges: iterator_ascend")
}

pub(crate) fn iterator_descend(iterator: &mut RangeIterator<'_>, goal_position: u32) -> bool {
    todo!("get_changed_ranges: iterator_descend")
}

pub(crate) fn iterator_advance(iterator: &mut RangeIterator<'_>) {
    todo!("get_changed_ranges: iterator_advance")
}

pub(crate) fn iterator_compare(
    old_iterator: &RangeIterator<'_>,
    new_iterator: &RangeIterator<'_>,
) -> IteratorComparison {
    todo!("get_changed_ranges: iterator_compare")
}

pub(crate) fn iterator_print_state(iterator: &RangeIterator<'_>) {
    todo!("get_changed_ranges: iterator_print_state")
}

pub(crate) fn ts_subtree_get_changed_ranges<'old, 'new>(
    old_tree: &'old Subtree,
    new_tree: &'new Subtree,
    cursor1: &mut TreeCursor<'old>,
    cursor2: &mut TreeCursor<'new>,
    language: &Language,
    included_range_differences: &[Range],
) -> Vec<Range> {
    todo!("get_changed_ranges: ts_subtree_get_changed_ranges")
}
