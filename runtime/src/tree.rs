use crate::{
    language::Language,
    node::Node,
    point::Point,
    subtree::Subtree,
    types::{InputEdit, Range},
};
#[derive(Debug)]
pub struct Tree {
    pub(crate) root: Box<Subtree>,
    pub(crate) language: Language,
    pub(crate) included_ranges: Vec<Range>,
}

pub(crate) fn ts_tree_new(root: Subtree, language: &Language, included_ranges: &[Range]) -> Tree {
    todo!("tree: ts_tree_new")
}

pub(crate) fn ts_tree_copy(tree: &Tree) -> Tree {
    todo!("tree: ts_tree_copy")
}

pub(crate) fn ts_tree_delete(tree: &mut Tree) {
    todo!("tree: ts_tree_delete")
}

pub(crate) fn ts_tree_root_node(tree: &Tree) -> Node<'_> {
    todo!("tree: ts_tree_root_node")
}

pub(crate) fn ts_tree_root_node_with_offset(
    tree: &Tree,
    offset_bytes: u32,
    offset_extent: Point,
) -> Node<'_> {
    todo!("tree: ts_tree_root_node_with_offset")
}

pub(crate) fn ts_tree_language(tree: &Tree) -> &Language {
    todo!("tree: ts_tree_language")
}

pub(crate) fn ts_tree_edit(tree: &mut Tree, edit: &InputEdit) {
    todo!("tree: ts_tree_edit")
}

pub(crate) fn ts_tree_included_ranges(tree: &Tree) -> Vec<Range> {
    todo!("tree: ts_tree_included_ranges")
}

pub(crate) fn ts_tree_get_changed_ranges(old_tree: &Tree, new_tree: &Tree) -> Vec<Range> {
    todo!("tree: ts_tree_get_changed_ranges")
}

pub(crate) fn _ts_dup(file: &std::fs::File) -> std::io::Result<std::fs::File> {
    todo!("tree: _ts_dup")
}

pub(crate) fn ts_tree_print_dot_graph(
    tree: &Tree,
    output: &mut dyn std::io::Write,
) -> std::io::Result<()> {
    todo!("tree: ts_tree_print_dot_graph")
}
