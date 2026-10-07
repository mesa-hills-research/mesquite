use crate::{language::Language, length::Length, subtree::Subtree, tree::Tree, types::*};
/// References point at stable subtree *slots*, not Arc allocations. Inline leaf
/// identity is the address of its slot, as in C. Borrowing prevents tree edits.
#[derive(Clone, Copy)]
pub struct Node<'tree> {
    pub(crate) tree: &'tree Tree,
    pub(crate) subtree: &'tree Subtree,
    pub(crate) position: Length,
    pub(crate) alias: Symbol,
}
pub(crate) struct NodeChildIterator<'tree> {
    pub parent: &'tree Subtree,
    pub tree: &'tree Tree,
    pub position: Length,
    pub child_index: u32,
    pub structural_child_index: u32,
    pub alias_sequence: &'static [Symbol],
}

pub(crate) fn ts_node_new<'tree>(
    tree: &'tree Tree,
    subtree: &'tree Subtree,
    position: Length,
    alias: Symbol,
) -> Node<'tree> {
    todo!("node-1: ts_node_new")
}

pub(crate) fn ts_node__null<'tree>() -> Option<Node<'tree>> {
    todo!("node-1: ts_node__null")
}

pub(crate) fn ts_node_start_byte(node: Node<'_>) -> u32 {
    todo!("node-1: ts_node_start_byte")
}

pub(crate) fn ts_node_start_point(node: Node<'_>) -> Point {
    todo!("node-1: ts_node_start_point")
}

pub(crate) fn ts_node__alias(node: &Node<'_>) -> Symbol {
    todo!("node-1: ts_node__alias")
}

pub(crate) fn ts_node__subtree<'tree>(node: Node<'tree>) -> &'tree Subtree {
    todo!("node-1: ts_node__subtree")
}

pub(crate) fn ts_node_iterate_children<'tree>(node: &Node<'tree>) -> NodeChildIterator<'tree> {
    todo!("node-1: ts_node_iterate_children")
}

pub(crate) fn ts_node_child_iterator_done(iterator: &NodeChildIterator<'_>) -> bool {
    todo!("node-1: ts_node_child_iterator_done")
}

pub(crate) fn ts_node_child_iterator_next<'tree>(
    iterator: &mut NodeChildIterator<'tree>,
) -> Option<Node<'tree>> {
    todo!("node-1: ts_node_child_iterator_next")
}

pub(crate) fn ts_node__is_relevant(node: Node<'_>, include_anonymous: bool) -> bool {
    todo!("node-1: ts_node__is_relevant")
}

pub(crate) fn ts_node__relevant_child_count(node: Node<'_>, include_anonymous: bool) -> u32 {
    todo!("node-1: ts_node__relevant_child_count")
}

pub(crate) fn ts_node__child(
    node: Node<'_>,
    child_index: u32,
    include_anonymous: bool,
) -> Option<Node<'_>> {
    todo!("node-1: ts_node__child")
}

pub(crate) fn ts_subtree_has_trailing_empty_descendant(tree: &Subtree, other: &Subtree) -> bool {
    todo!("node-1: ts_subtree_has_trailing_empty_descendant")
}

pub(crate) fn ts_node__prev_sibling(node: Node<'_>, include_anonymous: bool) -> Option<Node<'_>> {
    todo!("node-1: ts_node__prev_sibling")
}

pub(crate) fn ts_node__next_sibling(node: Node<'_>, include_anonymous: bool) -> Option<Node<'_>> {
    todo!("node-1: ts_node__next_sibling")
}

pub(crate) fn ts_node__first_child_for_byte(
    node: Node<'_>,
    goal: u32,
    include_anonymous: bool,
) -> Option<Node<'_>> {
    todo!("node-1: ts_node__first_child_for_byte")
}

pub(crate) fn ts_node__descendant_for_byte_range(
    node: Node<'_>,
    range_start: u32,
    range_end: u32,
    include_anonymous: bool,
) -> Option<Node<'_>> {
    todo!("node-1: ts_node__descendant_for_byte_range")
}

pub(crate) fn ts_node__descendant_for_point_range(
    node: Node<'_>,
    range_start: Point,
    range_end: Point,
    include_anonymous: bool,
) -> Option<Node<'_>> {
    todo!("node-1: ts_node__descendant_for_point_range")
}

pub(crate) fn ts_node_end_byte(node: Node<'_>) -> u32 {
    todo!("node-2: ts_node_end_byte")
}

pub(crate) fn ts_node_end_point(node: Node<'_>) -> Point {
    todo!("node-2: ts_node_end_point")
}

pub(crate) fn ts_node_symbol(node: Node<'_>) -> Symbol {
    todo!("node-2: ts_node_symbol")
}

pub(crate) fn ts_node_type(node: Node<'_>) -> &'static str {
    todo!("node-2: ts_node_type")
}

pub(crate) fn ts_node_language(node: Node<'_>) -> &Language {
    todo!("node-2: ts_node_language")
}

pub(crate) fn ts_node_grammar_symbol(node: Node<'_>) -> Symbol {
    todo!("node-2: ts_node_grammar_symbol")
}

pub(crate) fn ts_node_grammar_type(node: Node<'_>) -> &'static str {
    todo!("node-2: ts_node_grammar_type")
}

pub(crate) fn ts_node_string(node: Node<'_>) -> String {
    todo!("node-2: ts_node_string")
}

pub(crate) fn ts_node_eq(node: Node<'_>, other: Node<'_>) -> bool {
    todo!("node-2: ts_node_eq")
}

pub(crate) fn ts_node_is_null(node: Option<Node<'_>>) -> bool {
    todo!("node-2: ts_node_is_null")
}

pub(crate) fn ts_node_is_extra(node: Node<'_>) -> bool {
    todo!("node-2: ts_node_is_extra")
}

pub(crate) fn ts_node_is_named(node: Node<'_>) -> bool {
    todo!("node-2: ts_node_is_named")
}

pub(crate) fn ts_node_is_missing(node: Node<'_>) -> bool {
    todo!("node-2: ts_node_is_missing")
}

pub(crate) fn ts_node_has_changes(node: Node<'_>) -> bool {
    todo!("node-2: ts_node_has_changes")
}

pub(crate) fn ts_node_has_error(node: Node<'_>) -> bool {
    todo!("node-2: ts_node_has_error")
}

pub(crate) fn ts_node_is_error(node: Node<'_>) -> bool {
    todo!("node-2: ts_node_is_error")
}

pub(crate) fn ts_node_descendant_count(node: Node<'_>) -> u32 {
    todo!("node-2: ts_node_descendant_count")
}

pub(crate) fn ts_node_parse_state(node: Node<'_>) -> StateId {
    todo!("node-2: ts_node_parse_state")
}

pub(crate) fn ts_node_next_parse_state(node: Node<'_>) -> StateId {
    todo!("node-2: ts_node_next_parse_state")
}

pub(crate) fn ts_node_parent(node: Node<'_>) -> Option<Node<'_>> {
    todo!("node-2: ts_node_parent")
}

pub(crate) fn ts_node_child_with_descendant<'tree>(
    node: Node<'tree>,
    descendant: Node<'tree>,
) -> Option<Node<'tree>> {
    todo!("node-2: ts_node_child_with_descendant")
}

pub(crate) fn ts_node_child(node: Node<'_>, child_index: u32) -> Option<Node<'_>> {
    todo!("node-2: ts_node_child")
}

pub(crate) fn ts_node_named_child(node: Node<'_>, child_index: u32) -> Option<Node<'_>> {
    todo!("node-2: ts_node_named_child")
}

pub(crate) fn ts_node_child_by_field_id(node: Node<'_>, field_id: FieldId) -> Option<Node<'_>> {
    todo!("node-2: ts_node_child_by_field_id")
}

pub(crate) fn ts_node__field_name_from_language(
    node: Node<'_>,
    structural_child_index: u32,
) -> Option<&'static str> {
    todo!("node-2: ts_node__field_name_from_language")
}

pub(crate) fn ts_node_field_name_for_child(
    node: Node<'_>,
    child_index: u32,
) -> Option<&'static str> {
    todo!("node-2: ts_node_field_name_for_child")
}

pub(crate) fn ts_node_field_name_for_named_child(
    node: Node<'_>,
    named_child_index: u32,
) -> Option<&'static str> {
    todo!("node-2: ts_node_field_name_for_named_child")
}

pub(crate) fn ts_node_child_by_field_name<'tree>(
    node: Node<'tree>,
    name: &[u8],
) -> Option<Node<'tree>> {
    todo!("node-2: ts_node_child_by_field_name")
}

pub(crate) fn ts_node_child_count(node: Node<'_>) -> u32 {
    todo!("node-2: ts_node_child_count")
}

pub(crate) fn ts_node_named_child_count(node: Node<'_>) -> u32 {
    todo!("node-2: ts_node_named_child_count")
}

pub(crate) fn ts_node_next_sibling(node: Node<'_>) -> Option<Node<'_>> {
    todo!("node-2: ts_node_next_sibling")
}

pub(crate) fn ts_node_next_named_sibling(node: Node<'_>) -> Option<Node<'_>> {
    todo!("node-2: ts_node_next_named_sibling")
}

pub(crate) fn ts_node_prev_sibling(node: Node<'_>) -> Option<Node<'_>> {
    todo!("node-2: ts_node_prev_sibling")
}

pub(crate) fn ts_node_prev_named_sibling(node: Node<'_>) -> Option<Node<'_>> {
    todo!("node-2: ts_node_prev_named_sibling")
}

pub(crate) fn ts_node_first_child_for_byte(node: Node<'_>, byte: u32) -> Option<Node<'_>> {
    todo!("node-2: ts_node_first_child_for_byte")
}

pub(crate) fn ts_node_first_named_child_for_byte(node: Node<'_>, byte: u32) -> Option<Node<'_>> {
    todo!("node-2: ts_node_first_named_child_for_byte")
}

pub(crate) fn ts_node_descendant_for_byte_range(
    node: Node<'_>,
    start: u32,
    end: u32,
) -> Option<Node<'_>> {
    todo!("node-2: ts_node_descendant_for_byte_range")
}

pub(crate) fn ts_node_named_descendant_for_byte_range(
    node: Node<'_>,
    start: u32,
    end: u32,
) -> Option<Node<'_>> {
    todo!("node-2: ts_node_named_descendant_for_byte_range")
}

pub(crate) fn ts_node_descendant_for_point_range(
    node: Node<'_>,
    start: Point,
    end: Point,
) -> Option<Node<'_>> {
    todo!("node-2: ts_node_descendant_for_point_range")
}

pub(crate) fn ts_node_named_descendant_for_point_range(
    node: Node<'_>,
    start: Point,
    end: Point,
) -> Option<Node<'_>> {
    todo!("node-2: ts_node_named_descendant_for_point_range")
}

pub(crate) fn ts_node_edit(node: &mut Node<'_>, edit: &InputEdit) {
    todo!("node-2: ts_node_edit")
}
