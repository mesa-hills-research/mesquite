use crate::{language, point, subtree, tree};
use crate::{language::*, length::*, subtree::*, tree::Tree, types::*};
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
    Node {
        tree,
        subtree,
        position,
        alias,
    }
}

pub(crate) fn ts_node__null<'tree>() -> Option<Node<'tree>> {
    None
}

#[inline]
pub(crate) fn ts_node_start_byte(node: Node<'_>) -> u32 {
    node.position.bytes
}

#[inline]
pub(crate) fn ts_node_start_point(node: Node<'_>) -> Point {
    node.position.extent
}

pub(crate) fn ts_node__alias(node: &Node<'_>) -> Symbol {
    node.alias
}

pub(crate) fn ts_node__subtree<'tree>(node: Node<'tree>) -> &'tree Subtree {
    node.subtree
}

pub(crate) fn ts_node_iterate_children<'tree>(node: &Node<'tree>) -> NodeChildIterator<'tree> {
    let subtree = ts_node__subtree(*node);
    if ts_subtree_child_count(subtree) == 0 {
        return NodeChildIterator {
            parent: &Subtree::Null,
            tree: node.tree,
            position: length_zero(),
            child_index: 0,
            structural_child_index: 0,
            alias_sequence: &[],
        };
    }
    NodeChildIterator {
        parent: subtree,
        tree: node.tree,
        position: node.position,
        child_index: 0,
        structural_child_index: 0,
        alias_sequence: ts_language_alias_sequence(
            &node.tree.language,
            ts_subtree_production_id(subtree) as u32,
        ),
    }
}

pub(crate) fn ts_node_child_iterator_done(iterator: &NodeChildIterator<'_>) -> bool {
    iterator.child_index == ts_subtree_child_count(iterator.parent)
}

pub(crate) fn ts_node_child_iterator_next<'tree>(
    iterator: &mut NodeChildIterator<'tree>,
) -> Option<Node<'tree>> {
    if iterator.parent.is_null() || ts_node_child_iterator_done(iterator) {
        return None;
    }
    let child = &ts_subtree_children(iterator.parent)[iterator.child_index as usize];
    let mut alias = 0;
    if !ts_subtree_extra(child) {
        if !iterator.alias_sequence.is_empty() {
            alias = iterator.alias_sequence[iterator.structural_child_index as usize];
        }
        iterator.structural_child_index = iterator.structural_child_index.wrapping_add(1);
    }
    // The parent's start already includes the first child's padding.
    if iterator.child_index > 0 {
        iterator.position = length_add(iterator.position, ts_subtree_padding(child));
    }
    let result = ts_node_new(iterator.tree, child, iterator.position, alias);
    iterator.position = length_add(iterator.position, ts_subtree_size(child));
    iterator.child_index = iterator.child_index.wrapping_add(1);
    Some(result)
}

pub(crate) fn ts_node__is_relevant(node: Node<'_>, include_anonymous: bool) -> bool {
    let tree = ts_node__subtree(node);
    let alias = ts_node__alias(&node);
    if include_anonymous {
        ts_subtree_visible(tree) || alias != 0
    } else if alias != 0 {
        ts_language_symbol_metadata(&node.tree.language, alias).named
    } else {
        ts_subtree_visible(tree) && ts_subtree_named(tree)
    }
}

pub(crate) fn ts_node__relevant_child_count(node: Node<'_>, include_anonymous: bool) -> u32 {
    let tree = ts_node__subtree(node);
    if ts_subtree_child_count(tree) == 0 {
        0
    } else if include_anonymous {
        ts_subtree_visible_child_count(tree)
    } else {
        ts_subtree_named_child_count(tree)
    }
}

pub(crate) fn ts_node__child(
    node: Node<'_>,
    child_index: u32,
    include_anonymous: bool,
) -> Option<Node<'_>> {
    let mut result = node;
    let mut child_index = child_index;
    'descend: loop {
        let mut index: u32 = 0;
        let mut iterator = ts_node_iterate_children(&result);
        while let Some(child) = ts_node_child_iterator_next(&mut iterator) {
            if ts_node__is_relevant(child, include_anonymous) {
                if index == child_index {
                    return Some(child);
                }
                index = index.wrapping_add(1);
            } else {
                let grandchild_index = child_index.wrapping_sub(index);
                let grandchild_count = ts_node__relevant_child_count(child, include_anonymous);
                if grandchild_index < grandchild_count {
                    result = child;
                    child_index = grandchild_index;
                    continue 'descend;
                }
                index = index.wrapping_add(grandchild_count);
            }
        }
        return None;
    }
}

pub(crate) fn ts_subtree_has_trailing_empty_descendant(tree: &Subtree, other: &Subtree) -> bool {
    for child in ts_subtree_children(tree).iter().rev() {
        if ts_subtree_total_bytes(child) > 0 {
            break;
        }
        if child.ptr_eq(other) || ts_subtree_has_trailing_empty_descendant(child, other) {
            return true;
        }
    }
    false
}

pub(crate) fn ts_node__prev_sibling(node: Node<'_>, include_anonymous: bool) -> Option<Node<'_>> {
    let self_subtree = ts_node__subtree(node);
    let self_is_empty = ts_subtree_total_bytes(self_subtree) == 0;
    let target_end_byte = ts_node_end_byte(node);
    let mut current = ts_node_parent(node);
    let mut earlier_node = None;

    while let Some(parent) = current {
        let mut earlier_child = None;
        let mut child_containing_target = None;
        let mut iterator = ts_node_iterate_children(&parent);
        while let Some(child) = ts_node_child_iterator_next(&mut iterator) {
            // Node identity is the subtree slot, not the shared heap's identity.
            if std::ptr::eq(child.subtree, node.subtree) {
                break;
            }
            if iterator.position.bytes > target_end_byte
                || (iterator.position.bytes == target_end_byte
                    && (!self_is_empty
                        || ts_subtree_has_trailing_empty_descendant(
                            ts_node__subtree(child),
                            self_subtree,
                        )))
            {
                child_containing_target = Some(child);
                break;
            }
            if ts_node__is_relevant(child, include_anonymous) {
                earlier_child = Some((child, true));
            } else if ts_node__relevant_child_count(child, include_anonymous) > 0 {
                earlier_child = Some((child, false));
            }
        }

        if let Some(child) = child_containing_target {
            if earlier_child.is_some() {
                earlier_node = earlier_child;
            }
            current = Some(child);
        } else if let Some((child, relevant)) = earlier_child {
            if relevant {
                return Some(child);
            }
            current = Some(child);
        } else if let Some((earlier, relevant)) = earlier_node.take() {
            if relevant {
                return Some(earlier);
            }
            current = Some(earlier);
        } else {
            current = None;
        }
    }
    None
}

pub(crate) fn ts_node__next_sibling(node: Node<'_>, include_anonymous: bool) -> Option<Node<'_>> {
    let target_end_byte = ts_node_end_byte(node);
    let mut current = ts_node_parent(node);
    let mut later_node = None;

    while let Some(parent) = current {
        let mut later_child = None;
        let mut child_containing_target = None;
        let mut iterator = ts_node_iterate_children(&parent);
        while let Some(child) = ts_node_child_iterator_next(&mut iterator) {
            if iterator.position.bytes <= target_end_byte {
                continue;
            }
            let start_byte = ts_node_start_byte(node);
            let child_start_byte = ts_node_start_byte(child);
            let is_empty = start_byte == target_end_byte;
            let contains_target = if is_empty {
                child_start_byte < start_byte
            } else {
                child_start_byte <= start_byte
            };
            if contains_target {
                // Unlike the slot check in prev_sibling, C compares subtree words here.
                if !ts_node__subtree(child).ptr_eq(ts_node__subtree(node)) {
                    child_containing_target = Some(child);
                }
            } else if ts_node__is_relevant(child, include_anonymous) {
                later_child = Some((child, true));
                break;
            } else if ts_node__relevant_child_count(child, include_anonymous) > 0 {
                later_child = Some((child, false));
                break;
            }
        }

        if let Some(child) = child_containing_target {
            if later_child.is_some() {
                later_node = later_child;
            }
            current = Some(child);
        } else if let Some((child, relevant)) = later_child {
            if relevant {
                return Some(child);
            }
            current = Some(child);
        } else if let Some((later, relevant)) = later_node {
            if relevant {
                return Some(later);
            }
            current = Some(later);
        } else {
            current = None;
        }
    }
    None
}

pub(crate) fn ts_node__first_child_for_byte(
    node: Node<'_>,
    goal: u32,
    include_anonymous: bool,
) -> Option<Node<'_>> {
    let mut node = node;
    let mut last_iterator = None;
    'descend: loop {
        let mut iterator = ts_node_iterate_children(&node);
        loop {
            while let Some(child) = ts_node_child_iterator_next(&mut iterator) {
                if ts_node_end_byte(child) > goal {
                    if ts_node__is_relevant(child, include_anonymous) {
                        return Some(child);
                    } else if ts_node_child_count(child) > 0 {
                        // This deliberately compares against the child's count, as in C,
                        // and retains only one iterator rather than an ancestor stack.
                        if iterator.child_index < ts_subtree_child_count(ts_node__subtree(child)) {
                            last_iterator = Some(iterator);
                        }
                        node = child;
                        continue 'descend;
                    }
                }
            }
            iterator = last_iterator.take()?;
        }
    }
}

pub(crate) fn ts_node__descendant_for_byte_range(
    node: Node<'_>,
    range_start: u32,
    range_end: u32,
    include_anonymous: bool,
) -> Option<Node<'_>> {
    if range_start > range_end {
        return None;
    }
    let mut node = node;
    let mut last_visible_node = node;
    'descend: loop {
        let mut iterator = ts_node_iterate_children(&node);
        while let Some(child) = ts_node_child_iterator_next(&mut iterator) {
            let node_end = iterator.position.bytes;
            // The child must touch the range's end and exceed its start, unless
            // the child is empty, in which case touching the start is enough.
            if node_end < range_end {
                continue;
            }
            let child_start = ts_node_start_byte(child);
            let is_empty = child_start == node_end;
            if if is_empty {
                node_end < range_start
            } else {
                node_end <= range_start
            } {
                continue;
            }
            if range_start < child_start {
                break;
            }
            node = child;
            if ts_node__is_relevant(node, include_anonymous) {
                last_visible_node = node;
            }
            continue 'descend;
        }
        return Some(last_visible_node);
    }
}

pub(crate) fn ts_node__descendant_for_point_range(
    node: Node<'_>,
    range_start: Point,
    range_end: Point,
    include_anonymous: bool,
) -> Option<Node<'_>> {
    if range_start > range_end {
        return None;
    }
    let mut node = node;
    let mut last_visible_node = node;
    'descend: loop {
        let mut iterator = ts_node_iterate_children(&node);
        while let Some(child) = ts_node_child_iterator_next(&mut iterator) {
            let node_end = iterator.position.extent;
            if node_end < range_end {
                continue;
            }
            let child_start = ts_node_start_point(child);
            let is_empty = child_start == node_end;
            if if is_empty {
                node_end < range_start
            } else {
                node_end <= range_start
            } {
                continue;
            }
            if range_start < child_start {
                break;
            }
            node = child;
            if ts_node__is_relevant(node, include_anonymous) {
                last_visible_node = node;
            }
            continue 'descend;
        }
        return Some(last_visible_node);
    }
}

#[cfg(test)]
#[path = "node1_tests.rs"]
mod node1_tests;

#[inline]
pub(crate) fn ts_node_end_byte(node: Node<'_>) -> u32 {
    ts_node_start_byte(node).wrapping_add(subtree::ts_subtree_size(ts_node__subtree(node)).bytes)
}

#[inline]
pub(crate) fn ts_node_end_point(node: Node<'_>) -> Point {
    point::point_add(
        ts_node_start_point(node),
        subtree::ts_subtree_size(ts_node__subtree(node)).extent,
    )
}

#[inline]
pub(crate) fn ts_node_symbol(node: Node<'_>) -> Symbol {
    let alias = ts_node__alias(&node);
    let symbol = if alias != 0 {
        alias
    } else {
        subtree::ts_subtree_symbol(ts_node__subtree(node))
    };
    language::ts_language_public_symbol(&node.tree.language, symbol)
}

pub(crate) fn ts_node_type(node: Node<'_>) -> &'static str {
    let alias = ts_node__alias(&node);
    let symbol = if alias != 0 {
        alias
    } else {
        subtree::ts_subtree_symbol(ts_node__subtree(node))
    };
    language::ts_language_symbol_name(&node.tree.language, symbol)
        .expect("node symbol must have a name")
}

pub(crate) fn ts_node_language(node: Node<'_>) -> &Language {
    &node.tree.language
}

#[inline]
pub(crate) fn ts_node_grammar_symbol(node: Node<'_>) -> Symbol {
    subtree::ts_subtree_symbol(ts_node__subtree(node))
}

pub(crate) fn ts_node_grammar_type(node: Node<'_>) -> &'static str {
    let symbol = subtree::ts_subtree_symbol(ts_node__subtree(node));
    language::ts_language_symbol_name(&node.tree.language, symbol)
        .expect("node grammar symbol must have a name")
}

pub(crate) fn ts_node_string(node: Node<'_>) -> String {
    let alias = ts_node__alias(&node);
    subtree::ts_subtree_string(
        ts_node__subtree(node),
        alias,
        language::ts_language_symbol_metadata(&node.tree.language, alias).visible,
        &node.tree.language,
        false,
    )
}

pub(crate) fn ts_node_eq(node: Node<'_>, other: Node<'_>) -> bool {
    std::ptr::eq(node.tree, other.tree) && std::ptr::eq(node.subtree, other.subtree)
}

pub(crate) fn ts_node_is_null(node: Option<Node<'_>>) -> bool {
    node.is_none()
}

#[inline]
pub(crate) fn ts_node_is_extra(node: Node<'_>) -> bool {
    subtree::ts_subtree_extra(ts_node__subtree(node))
}

#[inline]
pub(crate) fn ts_node_is_named(node: Node<'_>) -> bool {
    let alias = ts_node__alias(&node);
    if alias != 0 {
        language::ts_language_symbol_metadata(&node.tree.language, alias).named
    } else {
        subtree::ts_subtree_named(ts_node__subtree(node))
    }
}

#[inline]
pub(crate) fn ts_node_is_missing(node: Node<'_>) -> bool {
    subtree::ts_subtree_missing(ts_node__subtree(node))
}

#[inline]
pub(crate) fn ts_node_has_changes(node: Node<'_>) -> bool {
    subtree::ts_subtree_has_changes(ts_node__subtree(node))
}

#[inline]
pub(crate) fn ts_node_has_error(node: Node<'_>) -> bool {
    subtree::ts_subtree_error_cost(ts_node__subtree(node)) > 0
}

#[inline]
pub(crate) fn ts_node_is_error(node: Node<'_>) -> bool {
    ts_node_symbol(node) == ts_port_tables::BUILTIN_SYM_ERROR
}

pub(crate) fn ts_node_descendant_count(node: Node<'_>) -> u32 {
    subtree::ts_subtree_visible_descendant_count(ts_node__subtree(node)).wrapping_add(1)
}

#[inline]
pub(crate) fn ts_node_parse_state(node: Node<'_>) -> StateId {
    subtree::ts_subtree_parse_state(ts_node__subtree(node))
}

pub(crate) fn ts_node_next_parse_state(node: Node<'_>) -> StateId {
    let state = ts_node_parse_state(node);
    if state == subtree::TS_TREE_STATE_NONE {
        return subtree::TS_TREE_STATE_NONE;
    }
    language::ts_language_next_state(&node.tree.language, state, ts_node_grammar_symbol(node))
}

pub(crate) fn ts_node_parent(node: Node<'_>) -> Option<Node<'_>> {
    let mut parent = tree::ts_tree_root_node(node.tree);
    if std::ptr::eq(parent.subtree, node.subtree) {
        return None;
    }

    while let Some(next) = ts_node_child_with_descendant(parent, node) {
        if std::ptr::eq(next.subtree, node.subtree) {
            break;
        }
        parent = next;
    }
    Some(parent)
}

pub(crate) fn ts_node_child_with_descendant<'tree>(
    node: Node<'tree>,
    descendant: Node<'tree>,
) -> Option<Node<'tree>> {
    let start_byte = ts_node_start_byte(descendant);
    let end_byte = ts_node_end_byte(descendant);
    let is_empty = start_byte == end_byte;
    let mut node = node;

    loop {
        let mut iterator = ts_node_iterate_children(&node);
        loop {
            node = ts_node_child_iterator_next(&mut iterator)?;
            if ts_node_start_byte(node) > start_byte {
                return None;
            }
            if std::ptr::eq(node.subtree, descendant.subtree) {
                return Some(node);
            }

            // An empty descendant can sit at the boundary of two children.
            // Check its actual identity within each candidate, not just its range.
            if is_empty
                && iterator.position.bytes >= end_byte
                && ts_node_child_count(node) > 0
                && let Some(child) = ts_node_child_with_descendant(node, descendant)
            {
                return Some(if ts_node__is_relevant(node, true) {
                    node
                } else {
                    child
                });
            }

            let before_end = if is_empty {
                iterator.position.bytes <= end_byte
            } else {
                iterator.position.bytes < end_byte
            };
            if !before_end && ts_node_child_count(node) != 0 {
                break;
            }
        }
        if ts_node__is_relevant(node, true) {
            return Some(node);
        }
    }
}

pub(crate) fn ts_node_child(node: Node<'_>, child_index: u32) -> Option<Node<'_>> {
    ts_node__child(node, child_index, true)
}

pub(crate) fn ts_node_named_child(node: Node<'_>, child_index: u32) -> Option<Node<'_>> {
    ts_node__child(node, child_index, false)
}

pub(crate) fn ts_node_child_by_field_id(node: Node<'_>, field_id: FieldId) -> Option<Node<'_>> {
    let mut node = node;
    'recur: loop {
        if field_id == 0 || ts_node_child_count(node) == 0 {
            return None;
        }
        let mut field_map = language::ts_language_field_map(
            &node.tree.language,
            u32::from(subtree::ts_subtree_production_id(ts_node__subtree(node))),
        );

        // Entries are sorted by field id. Keep exactly the run for this field.
        while field_map.first()?.field_id < field_id {
            field_map = &field_map[1..];
        }
        while field_map.last()?.field_id > field_id {
            field_map = &field_map[..field_map.len() - 1];
        }

        let mut iterator = ts_node_iterate_children(&node);
        while let Some(child) = ts_node_child_iterator_next(&mut iterator) {
            if subtree::ts_subtree_extra(ts_node__subtree(child)) {
                continue;
            }
            let index = iterator.structural_child_index.wrapping_sub(1);
            let entry = field_map.first()?;
            if index < u32::from(entry.child_index) {
                continue;
            }

            if entry.inherited {
                // The last possible inherited field is a tail call in C too.
                if field_map.len() == 1 {
                    node = child;
                    continue 'recur;
                }
                if let Some(result) = ts_node_child_by_field_id(child, field_id) {
                    return Some(result);
                }
            } else if ts_node__is_relevant(child, true) {
                return Some(child);
            } else if ts_node_child_count(child) > 0 {
                // A field on a hidden child refers to its first visible child.
                return ts_node_child(child, 0);
            }

            field_map = &field_map[1..];
            if field_map.is_empty() {
                return None;
            }
        }
        return None;
    }
}

pub(crate) fn ts_node__field_name_from_language(
    node: Node<'_>,
    structural_child_index: u32,
) -> Option<&'static str> {
    let field_map = language::ts_language_field_map(
        &node.tree.language,
        u32::from(subtree::ts_subtree_production_id(ts_node__subtree(node))),
    );
    for entry in field_map {
        if !entry.inherited && u32::from(entry.child_index) == structural_child_index {
            return node.tree.language.tables.field_names[usize::from(entry.field_id)].as_deref();
        }
    }
    None
}

pub(crate) fn ts_node_field_name_for_child(
    node: Node<'_>,
    child_index: u32,
) -> Option<&'static str> {
    field_name_for_child(node, child_index, true)
}

pub(crate) fn ts_node_field_name_for_named_child(
    node: Node<'_>,
    named_child_index: u32,
) -> Option<&'static str> {
    field_name_for_child(node, named_child_index, false)
}

pub(crate) fn ts_node_child_by_field_name<'tree>(
    node: Node<'tree>,
    name: &[u8],
) -> Option<Node<'tree>> {
    let field_id = language::ts_language_field_id_for_name(&node.tree.language, name);
    ts_node_child_by_field_id(node, field_id)
}

#[inline]
pub(crate) fn ts_node_child_count(node: Node<'_>) -> u32 {
    let tree = ts_node__subtree(node);
    if subtree::ts_subtree_child_count(tree) > 0 {
        subtree::ts_subtree_visible_child_count(tree)
    } else {
        0
    }
}

#[inline]
pub(crate) fn ts_node_named_child_count(node: Node<'_>) -> u32 {
    let tree = ts_node__subtree(node);
    if subtree::ts_subtree_child_count(tree) > 0 {
        subtree::ts_subtree_named_child_count(tree)
    } else {
        0
    }
}

pub(crate) fn ts_node_next_sibling(node: Node<'_>) -> Option<Node<'_>> {
    ts_node__next_sibling(node, true)
}

pub(crate) fn ts_node_next_named_sibling(node: Node<'_>) -> Option<Node<'_>> {
    ts_node__next_sibling(node, false)
}

pub(crate) fn ts_node_prev_sibling(node: Node<'_>) -> Option<Node<'_>> {
    ts_node__prev_sibling(node, true)
}

pub(crate) fn ts_node_prev_named_sibling(node: Node<'_>) -> Option<Node<'_>> {
    ts_node__prev_sibling(node, false)
}

pub(crate) fn ts_node_first_child_for_byte(node: Node<'_>, byte: u32) -> Option<Node<'_>> {
    ts_node__first_child_for_byte(node, byte, true)
}

pub(crate) fn ts_node_first_named_child_for_byte(node: Node<'_>, byte: u32) -> Option<Node<'_>> {
    ts_node__first_child_for_byte(node, byte, false)
}

pub(crate) fn ts_node_descendant_for_byte_range(
    node: Node<'_>,
    start: u32,
    end: u32,
) -> Option<Node<'_>> {
    ts_node__descendant_for_byte_range(node, start, end, true)
}

pub(crate) fn ts_node_named_descendant_for_byte_range(
    node: Node<'_>,
    start: u32,
    end: u32,
) -> Option<Node<'_>> {
    ts_node__descendant_for_byte_range(node, start, end, false)
}

pub(crate) fn ts_node_descendant_for_point_range(
    node: Node<'_>,
    start: Point,
    end: Point,
) -> Option<Node<'_>> {
    ts_node__descendant_for_point_range(node, start, end, true)
}

pub(crate) fn ts_node_named_descendant_for_point_range(
    node: Node<'_>,
    start: Point,
    end: Point,
) -> Option<Node<'_>> {
    ts_node__descendant_for_point_range(node, start, end, false)
}

pub(crate) fn ts_node_edit(node: &mut Node<'_>, edit: &InputEdit) {
    let mut start_byte = ts_node_start_byte(*node);
    let mut start_point = ts_node_start_point(*node);

    if start_byte >= edit.old_end_byte {
        start_byte = edit
            .new_end_byte
            .wrapping_add(start_byte.wrapping_sub(edit.old_end_byte));
        start_point = point::point_add(
            edit.new_end_point,
            point::point_sub(start_point, edit.old_end_point),
        );
    } else if start_byte > edit.start_byte {
        start_byte = edit.new_end_byte;
        start_point = edit.new_end_point;
    }

    node.position = Length {
        bytes: start_byte,
        extent: start_point,
    };
}

/// Shared traversal for the named and anonymous field-name accessors. A field
/// on a hidden ancestor is inherited unless a closer structural child overrides it.
fn field_name_for_child(
    mut node: Node<'_>,
    mut child_index: u32,
    include_anonymous: bool,
) -> Option<&'static str> {
    let mut inherited_field_name = None;
    'descend: loop {
        let mut index: u32 = 0;
        let mut iterator = ts_node_iterate_children(&node);
        while let Some(child) = ts_node_child_iterator_next(&mut iterator) {
            if ts_node__is_relevant(child, include_anonymous) {
                if index == child_index {
                    if ts_node_is_extra(child) {
                        return None;
                    }
                    return ts_node__field_name_from_language(
                        node,
                        iterator.structural_child_index.wrapping_sub(1),
                    )
                    .or(inherited_field_name);
                }
                index = index.wrapping_add(1);
            } else {
                let grandchild_index = child_index.wrapping_sub(index);
                let grandchild_count = ts_node__relevant_child_count(child, include_anonymous);
                if grandchild_index < grandchild_count {
                    let field_name = ts_node__field_name_from_language(
                        node,
                        iterator.structural_child_index.wrapping_sub(1),
                    );
                    if field_name.is_some() {
                        inherited_field_name = field_name;
                    }
                    node = child;
                    child_index = grandchild_index;
                    continue 'descend;
                }
                index = index.wrapping_add(grandchild_count);
            }
        }
        return None;
    }
}

#[cfg(test)]
mod node_2_tests {
    use super::*;
    use std::sync::{Arc, LazyLock};
    use ts_port_tables::LanguageTables;

    fn language() -> Language {
        // Only the immutable tables are needed for these node identity tests.
        static TABLES: LazyLock<LanguageTables> = LazyLock::new(|| {
            LanguageTables::decode(
                include_bytes!("../../grammars/c/src/tables.bin"),
                |_, _| unreachable!("identity tests do not lex"),
                Some(|_, _| unreachable!("identity tests do not lex keywords")),
                None,
            )
        });
        Language::from(&*TABLES)
    }

    #[test]
    fn identity_uses_tree_and_slot_not_subtree_value_or_context() {
        let leaf = Subtree::Inline(subtree::InlineLeaf::default());
        let mut data = subtree::SubtreeHeapData::default();
        data.children = vec![leaf.clone(), leaf];
        let tree = Tree {
            root: Box::new(Subtree::Heap(Arc::new(data))),
            language: language(),
            included_ranges: Vec::new(),
        };
        let children = subtree::ts_subtree_children(&tree.root);
        let node = Node {
            tree: &tree,
            subtree: &children[0],
            position: Length::default(),
            alias: 0,
        };
        let sibling = Node {
            subtree: &children[1],
            ..node
        };
        assert!(children[0].ptr_eq(&children[1]));
        assert!(!ts_node_eq(node, sibling));

        let offset_alias = Node {
            position: Length {
                bytes: 42,
                extent: Point { row: 2, column: 7 },
            },
            alias: 1,
            ..node
        };
        assert!(ts_node_eq(node, offset_alias));

        let copied_tree = Tree {
            root: Box::new((*tree.root).clone()),
            language: tree.language,
            included_ranges: Vec::new(),
        };
        let copy_node = Node {
            tree: &copied_tree,
            subtree: &subtree::ts_subtree_children(&copied_tree.root)[0],
            ..node
        };
        assert!(std::ptr::eq(node.subtree, copy_node.subtree));
        assert!(!ts_node_eq(node, copy_node));
        assert!(std::ptr::eq(ts_node_language(node), &tree.language));
        assert!(!ts_node_is_null(Some(node)));
        assert!(ts_node_is_null(None));
    }
}
