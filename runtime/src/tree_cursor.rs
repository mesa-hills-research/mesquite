use crate::{
    language::Language, length::Length, node::Node, subtree::Subtree, tree::Tree, types::*,
};
#[derive(Clone, Copy, Debug)]
pub(crate) struct TreeCursorEntry<'tree> {
    pub subtree: &'tree Subtree,
    pub position: Length,
    pub child_index: u32,
    pub structural_child_index: u32,
    pub descendant_index: u32,
}
#[derive(Debug)]
pub struct TreeCursor<'cursor> {
    // None is used only by the internal changed-range walker, which needs no Node.
    pub(crate) tree: Option<&'cursor Tree>,
    pub(crate) stack: Vec<TreeCursorEntry<'cursor>>,
    pub(crate) root_alias_symbol: Symbol,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum TreeCursorStep {
    None,
    Hidden,
    Visible,
}
pub(crate) struct CursorChildIterator<'tree> {
    pub parent: &'tree Subtree,
    pub position: Length,
    pub child_index: u32,
    pub structural_child_index: u32,
    pub descendant_index: u32,
    pub alias_sequence: &'static [Symbol],
}
#[derive(Debug, Default)]
pub(crate) struct CursorStatus {
    pub field_id: FieldId,
    pub has_later_siblings: bool,
    pub has_later_named_siblings: bool,
    pub can_have_later_siblings_with_this_field: bool,
    pub supertypes: Vec<Symbol>,
}
pub(crate) type AdvanceChild =
    for<'a> fn(&mut CursorChildIterator<'a>) -> Option<(TreeCursorEntry<'a>, bool)>;

pub(crate) fn ts_tree_cursor_is_entry_visible(cursor: &TreeCursor<'_>, index: u32) -> bool {
    todo!("tree_cursor: ts_tree_cursor_is_entry_visible")
}

pub(crate) fn ts_tree_cursor_iterate_children<'tree>(
    cursor: &TreeCursor<'tree>,
) -> CursorChildIterator<'tree> {
    todo!("tree_cursor: ts_tree_cursor_iterate_children")
}

pub(crate) fn ts_tree_cursor_child_iterator_next<'tree>(
    iterator: &mut CursorChildIterator<'tree>,
) -> Option<(TreeCursorEntry<'tree>, bool)> {
    todo!("tree_cursor: ts_tree_cursor_child_iterator_next")
}

pub(crate) fn length_backtrack(a: Length, b: Length) -> Length {
    todo!("tree_cursor: length_backtrack")
}

pub(crate) fn ts_tree_cursor_child_iterator_previous<'tree>(
    iterator: &mut CursorChildIterator<'tree>,
) -> Option<(TreeCursorEntry<'tree>, bool)> {
    todo!("tree_cursor: ts_tree_cursor_child_iterator_previous")
}

pub(crate) fn ts_tree_cursor_new(node: Node<'_>) -> TreeCursor<'_> {
    todo!("tree_cursor: ts_tree_cursor_new")
}

pub(crate) fn ts_tree_cursor_reset<'tree>(cursor: &mut TreeCursor<'tree>, node: Node<'tree>) {
    todo!("tree_cursor: ts_tree_cursor_reset")
}

pub(crate) fn ts_tree_cursor_init<'tree>(cursor: &mut TreeCursor<'tree>, node: Node<'tree>) {
    todo!("tree_cursor: ts_tree_cursor_init")
}

pub(crate) fn ts_tree_cursor_delete(cursor: &mut TreeCursor<'_>) {
    todo!("tree_cursor: ts_tree_cursor_delete")
}

pub(crate) fn ts_tree_cursor_goto_first_child_internal(
    cursor: &mut TreeCursor<'_>,
) -> TreeCursorStep {
    todo!("tree_cursor: ts_tree_cursor_goto_first_child_internal")
}

pub(crate) fn ts_tree_cursor_goto_first_child(cursor: &mut TreeCursor<'_>) -> bool {
    todo!("tree_cursor: ts_tree_cursor_goto_first_child")
}

pub(crate) fn ts_tree_cursor_goto_last_child_internal(
    cursor: &mut TreeCursor<'_>,
) -> TreeCursorStep {
    todo!("tree_cursor: ts_tree_cursor_goto_last_child_internal")
}

pub(crate) fn ts_tree_cursor_goto_last_child(cursor: &mut TreeCursor<'_>) -> bool {
    todo!("tree_cursor: ts_tree_cursor_goto_last_child")
}

pub(crate) fn ts_tree_cursor_goto_first_child_for_byte_and_point(
    cursor: &mut TreeCursor<'_>,
    goal_byte: u32,
    goal_point: Point,
) -> Option<u32> {
    todo!("tree_cursor: ts_tree_cursor_goto_first_child_for_byte_and_point")
}

pub(crate) fn ts_tree_cursor_goto_first_child_for_byte(
    cursor: &mut TreeCursor<'_>,
    goal_byte: u32,
) -> Option<u32> {
    todo!("tree_cursor: ts_tree_cursor_goto_first_child_for_byte")
}

pub(crate) fn ts_tree_cursor_goto_first_child_for_point(
    cursor: &mut TreeCursor<'_>,
    goal_point: Point,
) -> Option<u32> {
    todo!("tree_cursor: ts_tree_cursor_goto_first_child_for_point")
}

pub(crate) fn ts_tree_cursor_goto_sibling_internal(
    cursor: &mut TreeCursor<'_>,
    advance: AdvanceChild,
) -> TreeCursorStep {
    todo!("tree_cursor: ts_tree_cursor_goto_sibling_internal")
}

pub(crate) fn ts_tree_cursor_goto_next_sibling_internal(
    cursor: &mut TreeCursor<'_>,
) -> TreeCursorStep {
    todo!("tree_cursor: ts_tree_cursor_goto_next_sibling_internal")
}

pub(crate) fn ts_tree_cursor_goto_next_sibling(cursor: &mut TreeCursor<'_>) -> bool {
    todo!("tree_cursor: ts_tree_cursor_goto_next_sibling")
}

pub(crate) fn ts_tree_cursor_goto_previous_sibling_internal(
    cursor: &mut TreeCursor<'_>,
) -> TreeCursorStep {
    todo!("tree_cursor: ts_tree_cursor_goto_previous_sibling_internal")
}

pub(crate) fn ts_tree_cursor_goto_previous_sibling(cursor: &mut TreeCursor<'_>) -> bool {
    todo!("tree_cursor: ts_tree_cursor_goto_previous_sibling")
}

pub(crate) fn ts_tree_cursor_goto_parent(cursor: &mut TreeCursor<'_>) -> bool {
    todo!("tree_cursor: ts_tree_cursor_goto_parent")
}

pub(crate) fn ts_tree_cursor_goto_descendant(
    cursor: &mut TreeCursor<'_>,
    goal_descendant_index: u32,
) {
    todo!("tree_cursor: ts_tree_cursor_goto_descendant")
}

pub(crate) fn ts_tree_cursor_current_descendant_index(cursor: &TreeCursor<'_>) -> u32 {
    todo!("tree_cursor: ts_tree_cursor_current_descendant_index")
}

pub(crate) fn ts_tree_cursor_current_node<'tree>(cursor: &TreeCursor<'tree>) -> Node<'tree> {
    todo!("tree_cursor: ts_tree_cursor_current_node")
}

pub(crate) fn ts_tree_cursor_current_status(cursor: &TreeCursor<'_>) -> CursorStatus {
    todo!("tree_cursor: ts_tree_cursor_current_status")
}

pub(crate) fn ts_tree_cursor_current_depth(cursor: &TreeCursor<'_>) -> u32 {
    todo!("tree_cursor: ts_tree_cursor_current_depth")
}

pub(crate) fn ts_tree_cursor_parent_node<'tree>(cursor: &TreeCursor<'tree>) -> Option<Node<'tree>> {
    todo!("tree_cursor: ts_tree_cursor_parent_node")
}

pub(crate) fn ts_tree_cursor_current_field_id(cursor: &TreeCursor<'_>) -> FieldId {
    todo!("tree_cursor: ts_tree_cursor_current_field_id")
}

pub(crate) fn ts_tree_cursor_current_field_name(cursor: &TreeCursor<'_>) -> Option<&'static str> {
    todo!("tree_cursor: ts_tree_cursor_current_field_name")
}

pub(crate) fn ts_tree_cursor_copy<'tree>(cursor: &TreeCursor<'tree>) -> TreeCursor<'tree> {
    todo!("tree_cursor: ts_tree_cursor_copy")
}

pub(crate) fn ts_tree_cursor_reset_to<'tree>(
    destination: &mut TreeCursor<'tree>,
    source: &TreeCursor<'tree>,
) {
    todo!("tree_cursor: ts_tree_cursor_reset_to")
}

pub(crate) fn ts_tree_cursor_current_subtree<'tree>(cursor: &TreeCursor<'tree>) -> &'tree Subtree {
    todo!("tree_cursor: ts_tree_cursor_current_subtree")
}
