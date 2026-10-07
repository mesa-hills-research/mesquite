use crate::subtree::Subtree;
#[derive(Debug)]
pub(crate) struct StackEntry {
    pub tree: Subtree,
    pub child_index: u32,
    pub byte_offset: u32,
}
#[derive(Debug, Default)]
pub(crate) struct ReusableNode {
    pub stack: Vec<StackEntry>,
    pub last_external_token: Subtree,
}

pub(crate) fn reusable_node_new() -> ReusableNode {
    todo!("parser-4: reusable_node_new")
}

pub(crate) fn reusable_node_clear(node: &mut ReusableNode) {
    todo!("parser-4: reusable_node_clear")
}

pub(crate) fn reusable_node_tree(node: &ReusableNode) -> Subtree {
    todo!("parser-4: reusable_node_tree")
}

pub(crate) fn reusable_node_byte_offset(node: &ReusableNode) -> u32 {
    todo!("parser-4: reusable_node_byte_offset")
}

pub(crate) fn reusable_node_delete(node: &mut ReusableNode) {
    todo!("parser-4: reusable_node_delete")
}

pub(crate) fn reusable_node_advance(node: &mut ReusableNode) {
    todo!("parser-4: reusable_node_advance")
}

pub(crate) fn reusable_node_descend(node: &mut ReusableNode) -> bool {
    todo!("parser-4: reusable_node_descend")
}

pub(crate) fn reusable_node_advance_past_leaf(node: &mut ReusableNode) {
    todo!("parser-4: reusable_node_advance_past_leaf")
}

pub(crate) fn reusable_node_reset(node: &mut ReusableNode, tree: Subtree) {
    todo!("parser-4: reusable_node_reset")
}
