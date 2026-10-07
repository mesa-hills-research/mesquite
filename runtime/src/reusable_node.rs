use crate::subtree::{
    Subtree, ts_subtree_children, ts_subtree_has_external_tokens, ts_subtree_last_external_token,
    ts_subtree_total_bytes,
};
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
    ReusableNode {
        stack: Vec::new(),
        last_external_token: Subtree::Null,
    }
}

pub(crate) fn reusable_node_clear(node: &mut ReusableNode) {
    node.stack.clear();
    node.last_external_token = Subtree::Null;
}

pub(crate) fn reusable_node_tree(node: &ReusableNode) -> Subtree {
    node.stack
        .last()
        .map_or(Subtree::Null, |entry| entry.tree.clone())
}

pub(crate) fn reusable_node_byte_offset(node: &ReusableNode) -> u32 {
    node.stack
        .last()
        .map_or(u32::MAX, |entry| entry.byte_offset)
}

pub(crate) fn reusable_node_delete(node: &mut ReusableNode) {
    node.stack = Vec::new();
}

pub(crate) fn reusable_node_advance(node: &mut ReusableNode) {
    let last_entry = node.stack.last().expect("reusable node stack is nonempty");
    let byte_offset = last_entry
        .byte_offset
        .wrapping_add(ts_subtree_total_bytes(&last_entry.tree));
    if ts_subtree_has_external_tokens(&last_entry.tree) {
        node.last_external_token = ts_subtree_last_external_token(&last_entry.tree);
    }

    loop {
        let popped_entry = node.stack.pop().expect("reusable node stack is nonempty");
        let next_index = popped_entry.child_index.wrapping_add(1);
        let Some(parent) = node.stack.last() else {
            return;
        };
        if let Some(tree) = ts_subtree_children(&parent.tree).get(next_index as usize) {
            node.stack.push(StackEntry {
                tree: tree.clone(),
                child_index: next_index,
                byte_offset,
            });
            return;
        }
    }
}

pub(crate) fn reusable_node_descend(node: &mut ReusableNode) -> bool {
    let last_entry = node.stack.last().expect("reusable node stack is nonempty");
    if let Some(tree) = ts_subtree_children(&last_entry.tree).first() {
        node.stack.push(StackEntry {
            tree: tree.clone(),
            child_index: 0,
            byte_offset: last_entry.byte_offset,
        });
        true
    } else {
        false
    }
}

pub(crate) fn reusable_node_advance_past_leaf(node: &mut ReusableNode) {
    while reusable_node_descend(node) {}
    reusable_node_advance(node);
}

pub(crate) fn reusable_node_reset(node: &mut ReusableNode, tree: Subtree) {
    reusable_node_clear(node);
    node.stack.push(StackEntry {
        tree,
        child_index: 0,
        byte_offset: 0,
    });

    // Accepted roots contain the EOF child and possibly additional extras, so
    // their non-standard structure must never be reused as a single subtree.
    if !reusable_node_descend(node) {
        reusable_node_clear(node);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        length::Length,
        subtree::{InlineLeaf, SubtreeHeapData},
    };
    use std::sync::Arc;

    fn leaf(symbol: u8, padding: u8, size: u8) -> Subtree {
        Subtree::Inline(InlineLeaf {
            symbol,
            padding_bytes: padding,
            padding_columns: padding,
            size_bytes: size,
            ..InlineLeaf::default()
        })
    }

    fn branch(children: Vec<Subtree>) -> Subtree {
        let bytes = children.iter().map(ts_subtree_total_bytes).sum();
        let mut data = SubtreeHeapData::default();
        data.size = Length {
            bytes,
            ..Length::default()
        };
        data.children = children;
        Subtree::Heap(Arc::new(data))
    }

    #[test]
    fn empty_and_leaf_roots_are_not_reused() {
        let mut node = reusable_node_new();
        assert!(reusable_node_tree(&node).is_null());
        assert_eq!(reusable_node_byte_offset(&node), u32::MAX);
        reusable_node_reset(&mut node, leaf(1, 0, 2));
        assert!(reusable_node_tree(&node).is_null());
        assert!(node.last_external_token.is_null());
        assert_eq!(reusable_node_byte_offset(&node), u32::MAX);
    }

    #[test]
    fn traversal_skips_root_and_counts_padding() {
        let a = leaf(1, 2, 3);
        let b = leaf(2, 1, 2);
        let c = leaf(3, 0, 4);
        let d = leaf(4, 1, 1);
        let left = branch(vec![a.clone(), b.clone()]);
        let root = branch(vec![left.clone(), branch(vec![c.clone()]), d.clone()]);
        let mut node = reusable_node_new();
        reusable_node_reset(&mut node, root);
        assert!(reusable_node_tree(&node).ptr_eq(&left));
        assert_eq!(reusable_node_byte_offset(&node), 0);
        assert!(reusable_node_descend(&mut node));
        assert!(reusable_node_tree(&node).ptr_eq(&a));
        assert!(!reusable_node_descend(&mut node));
        reusable_node_advance(&mut node);
        assert!(reusable_node_tree(&node).ptr_eq(&b));
        assert_eq!(reusable_node_byte_offset(&node), 5);
        reusable_node_advance(&mut node);
        assert_eq!(reusable_node_byte_offset(&node), 8);
        assert!(reusable_node_descend(&mut node));
        assert!(reusable_node_tree(&node).ptr_eq(&c));
        reusable_node_advance(&mut node);
        assert!(reusable_node_tree(&node).ptr_eq(&d));
        assert_eq!(reusable_node_byte_offset(&node), 12);
        reusable_node_advance(&mut node);
        assert!(reusable_node_tree(&node).is_null());
        assert_eq!(reusable_node_byte_offset(&node), u32::MAX);
    }

    #[test]
    fn advance_skips_a_branch_but_advance_past_leaf_only_skips_its_first_leaf() {
        let a = leaf(1, 0, 2);
        let b = leaf(2, 0, 3);
        let c = leaf(3, 0, 4);
        let root = branch(vec![branch(vec![branch(vec![a]), b.clone()]), c.clone()]);
        let mut node = reusable_node_new();
        reusable_node_reset(&mut node, root.clone());
        reusable_node_advance(&mut node);
        assert!(reusable_node_tree(&node).ptr_eq(&c));
        assert_eq!(reusable_node_byte_offset(&node), 5);
        reusable_node_reset(&mut node, root);
        reusable_node_advance_past_leaf(&mut node);
        assert!(reusable_node_tree(&node).ptr_eq(&b));
        assert_eq!(reusable_node_byte_offset(&node), 2);
    }

    #[test]
    fn advancing_wraps_byte_offsets_and_preserves_previous_external_token() {
        let mut node = reusable_node_new();
        reusable_node_reset(&mut node, branch(vec![leaf(1, 1, 2), leaf(2, 0, 1)]));
        node.stack.last_mut().unwrap().byte_offset = u32::MAX - 1;
        let previous_external = leaf(3, 0, 1);
        node.last_external_token = previous_external.clone();
        reusable_node_advance(&mut node);
        assert_eq!(reusable_node_byte_offset(&node), 1);
        assert!(node.last_external_token.ptr_eq(&previous_external));
        reusable_node_advance(&mut node);
        assert!(node.stack.is_empty());
        assert!(node.last_external_token.ptr_eq(&previous_external));
    }

    #[test]
    fn clear_retains_capacity_and_delete_releases_it() {
        let mut node = reusable_node_new();
        reusable_node_reset(&mut node, branch(vec![leaf(1, 0, 1)]));
        node.last_external_token = leaf(2, 0, 1);
        let capacity = node.stack.capacity();
        reusable_node_clear(&mut node);
        assert!(node.stack.is_empty());
        assert!(node.last_external_token.is_null());
        assert_eq!(node.stack.capacity(), capacity);
        reusable_node_delete(&mut node);
        assert_eq!(node.stack.capacity(), 0);
    }
}
