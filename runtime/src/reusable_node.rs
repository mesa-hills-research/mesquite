use crate::subtree::{
    Subtree, ts_subtree_children, ts_subtree_has_external_tokens, ts_subtree_last_external_token,
    ts_subtree_total_bytes,
};

/// C's ReusableNode as the parser keeps it between the calls of one parse: the
/// child index and byte offset of each node on the cursor's path, from the old
/// root down. A parse that resumes after a timeout or cancellation rebuilds a
/// [`ReuseCursor`] from it over the same old tree.
#[derive(Debug, Default)]
pub(crate) struct ReusableNode {
    path: Vec<PathEntry>,
    last_external_token: Subtree,
}

#[derive(Clone, Copy, Debug)]
struct PathEntry {
    child_index: u32,
    byte_offset: u32,
}

/// C's ReusableNode during one call of a parse. It borrows the old tree's
/// nodes, which the parser keeps alive, so moving into, past or out of a node
/// costs no reference counting. Only a node the parser reuses is retained.
#[derive(Debug)]
pub(crate) struct ReuseCursor<'t> {
    pub stack: Vec<StackEntry<'t>>,
    pub last_external_token: std::borrow::Cow<'t, Subtree>,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct StackEntry<'t> {
    pub tree: &'t Subtree,
    pub child_index: u32,
    pub byte_offset: u32,
}

pub(crate) fn reusable_node_new() -> ReusableNode {
    ReusableNode::default()
}

pub(crate) fn reusable_node_clear(node: &mut ReusableNode) {
    node.path.clear();
    node.last_external_token = Subtree::Null;
}

pub(crate) fn reusable_node_delete(node: &mut ReusableNode) {
    node.path = Vec::new();
}

pub(crate) fn reusable_node_reset(node: &mut ReusableNode, tree: &Subtree) {
    reusable_node_clear(node);
    // Accepted roots contain the EOF child and possibly additional extras, so
    // their non-standard structure must never be reused as a single subtree.
    // The cursor starts at the root's first child, if it has one.
    if !ts_subtree_children(tree).is_empty() {
        let start = PathEntry {
            child_index: 0,
            byte_offset: 0,
        };
        node.path.extend([start, start]);
    }
}

impl<'t> ReuseCursor<'t> {
    /// Takes up the saved position, whose path starts at `root`.
    pub(crate) fn resume(root: &'t Subtree, saved: &mut ReusableNode) -> Self {
        let mut stack = Vec::with_capacity(saved.path.len() + 16);
        let mut tree = root;
        for (depth, entry) in saved.path.iter().enumerate() {
            if depth > 0 {
                tree = &ts_subtree_children(tree)[entry.child_index as usize];
            }
            stack.push(StackEntry {
                tree,
                child_index: entry.child_index,
                byte_offset: entry.byte_offset,
            });
        }
        ReuseCursor {
            stack,
            last_external_token: std::borrow::Cow::Owned(std::mem::take(
                &mut saved.last_external_token,
            )),
        }
    }

    /// Keeps the position for a later call that resumes this parse.
    pub(crate) fn save(self, saved: &mut ReusableNode) {
        saved.path.clear();
        saved.path.extend(self.stack.iter().map(|entry| PathEntry {
            child_index: entry.child_index,
            byte_offset: entry.byte_offset,
        }));
        saved.last_external_token = self.last_external_token.into_owned();
    }
}

pub(crate) fn reusable_node_tree<'t>(node: &ReuseCursor<'t>) -> Option<&'t Subtree> {
    node.stack.last().map(|entry| entry.tree)
}

pub(crate) fn reusable_node_byte_offset(node: &ReuseCursor<'_>) -> u32 {
    node.stack
        .last()
        .map_or(u32::MAX, |entry| entry.byte_offset)
}

pub(crate) fn reusable_node_advance(node: &mut ReuseCursor<'_>) {
    let last_entry = *node.stack.last().expect("reusable node stack is nonempty");
    let byte_offset = last_entry
        .byte_offset
        .wrapping_add(ts_subtree_total_bytes(last_entry.tree));
    if ts_subtree_has_external_tokens(last_entry.tree) {
        node.last_external_token =
            std::borrow::Cow::Owned(ts_subtree_last_external_token(last_entry.tree));
    }

    loop {
        let popped_entry = node.stack.pop().expect("reusable node stack is nonempty");
        let next_index = popped_entry.child_index.wrapping_add(1);
        let Some(parent) = node.stack.last() else {
            return;
        };
        if let Some(tree) = ts_subtree_children(parent.tree).get(next_index as usize) {
            node.stack.push(StackEntry {
                tree,
                child_index: next_index,
                byte_offset,
            });
            return;
        }
    }
}

pub(crate) fn reusable_node_descend(node: &mut ReuseCursor<'_>) -> bool {
    let last_entry = *node.stack.last().expect("reusable node stack is nonempty");
    if let Some(tree) = ts_subtree_children(last_entry.tree).first() {
        node.stack.push(StackEntry {
            tree,
            child_index: 0,
            byte_offset: last_entry.byte_offset,
        });
        true
    } else {
        false
    }
}

pub(crate) fn reusable_node_advance_past_leaf(node: &mut ReuseCursor<'_>) {
    while reusable_node_descend(node) {}
    reusable_node_advance(node);
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
        Subtree::inline(InlineLeaf {
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

    fn cursor<'t>(root: &'t Subtree, saved: &mut ReusableNode) -> ReuseCursor<'t> {
        reusable_node_reset(saved, root);
        ReuseCursor::resume(root, saved)
    }

    fn current<'t>(node: &ReuseCursor<'t>) -> &'t Subtree {
        reusable_node_tree(node).expect("a current node")
    }

    #[test]
    fn empty_and_leaf_roots_are_not_reused() {
        let mut saved = reusable_node_new();
        let root = leaf(1, 0, 2);
        let node = cursor(&root, &mut saved);
        assert!(reusable_node_tree(&node).is_none());
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
        let mut saved = reusable_node_new();
        let mut node = cursor(&root, &mut saved);
        assert!(current(&node).ptr_eq(&left));
        assert_eq!(reusable_node_byte_offset(&node), 0);
        assert!(reusable_node_descend(&mut node));
        assert!(current(&node).ptr_eq(&a));
        assert!(!reusable_node_descend(&mut node));
        reusable_node_advance(&mut node);
        assert!(current(&node).ptr_eq(&b));
        assert_eq!(reusable_node_byte_offset(&node), 5);
        reusable_node_advance(&mut node);
        assert_eq!(reusable_node_byte_offset(&node), 8);
        assert!(reusable_node_descend(&mut node));
        assert!(current(&node).ptr_eq(&c));
        reusable_node_advance(&mut node);
        assert!(current(&node).ptr_eq(&d));
        assert_eq!(reusable_node_byte_offset(&node), 12);
        reusable_node_advance(&mut node);
        assert!(reusable_node_tree(&node).is_none());
        assert_eq!(reusable_node_byte_offset(&node), u32::MAX);
    }

    #[test]
    fn advance_skips_a_branch_but_advance_past_leaf_only_skips_its_first_leaf() {
        let a = leaf(1, 0, 2);
        let b = leaf(2, 0, 3);
        let c = leaf(3, 0, 4);
        let root = branch(vec![branch(vec![branch(vec![a]), b.clone()]), c.clone()]);
        let mut saved = reusable_node_new();
        let mut node = cursor(&root, &mut saved);
        reusable_node_advance(&mut node);
        assert!(current(&node).ptr_eq(&c));
        assert_eq!(reusable_node_byte_offset(&node), 5);
        let mut node = cursor(&root, &mut saved);
        reusable_node_advance_past_leaf(&mut node);
        assert!(current(&node).ptr_eq(&b));
        assert_eq!(reusable_node_byte_offset(&node), 2);
    }

    #[test]
    fn advancing_wraps_byte_offsets_and_preserves_previous_external_token() {
        let root = branch(vec![leaf(1, 1, 2), leaf(2, 0, 1)]);
        let mut saved = reusable_node_new();
        let mut node = cursor(&root, &mut saved);
        node.stack.last_mut().unwrap().byte_offset = u32::MAX - 1;
        let previous_external = leaf(3, 0, 1);
        node.last_external_token = std::borrow::Cow::Owned(previous_external.clone());
        reusable_node_advance(&mut node);
        assert_eq!(reusable_node_byte_offset(&node), 1);
        assert!(node.last_external_token.ptr_eq(&previous_external));
        reusable_node_advance(&mut node);
        assert!(node.stack.is_empty());
        assert!(node.last_external_token.ptr_eq(&previous_external));
    }

    #[test]
    fn a_saved_position_resumes_at_the_same_node_and_external_token() {
        let a = leaf(1, 0, 2);
        let b = leaf(2, 0, 3);
        let c = leaf(3, 0, 4);
        let root = branch(vec![branch(vec![a, b.clone()]), c.clone()]);
        let mut saved = reusable_node_new();
        let mut node = cursor(&root, &mut saved);
        assert!(reusable_node_descend(&mut node));
        reusable_node_advance(&mut node);
        let external = leaf(4, 0, 1);
        node.last_external_token = std::borrow::Cow::Owned(external.clone());
        node.save(&mut saved);
        let mut node = ReuseCursor::resume(&root, &mut saved);
        assert!(current(&node).ptr_eq(&b));
        assert_eq!(reusable_node_byte_offset(&node), 2);
        assert!(node.last_external_token.ptr_eq(&external));
        reusable_node_advance(&mut node);
        assert!(current(&node).ptr_eq(&c));
        assert_eq!(reusable_node_byte_offset(&node), 5);
    }

    #[test]
    fn clear_retains_capacity_and_delete_releases_it() {
        let root = branch(vec![leaf(1, 0, 1)]);
        let mut saved = reusable_node_new();
        reusable_node_reset(&mut saved, &root);
        saved.last_external_token = leaf(2, 0, 1);
        let capacity = saved.path.capacity();
        reusable_node_clear(&mut saved);
        assert!(saved.path.is_empty());
        assert!(saved.last_external_token.is_null());
        assert_eq!(saved.path.capacity(), capacity);
        reusable_node_delete(&mut saved);
        assert_eq!(saved.path.capacity(), 0);
    }
}
