use crate::{
    language::*,
    length::*,
    node::{Node, ts_node_new},
    point::POINT_ZERO,
    subtree::*,
    tree::Tree,
    types::*,
};
#[derive(Clone, Copy, Debug)]
pub(crate) struct TreeCursorEntry<'tree> {
    pub subtree: &'tree Subtree,
    pub position: Length,
    pub child_index: u32,
    pub structural_child_index: u32,
    pub descendant_index: u32,
    // Immutable edge metadata, resolved when the entry is visited. Field ids
    // propagate only through invisible, non-extra wrappers, just as C's
    // current_field_id walks back through the stack.
    pub alias: Symbol,
    pub field_id: FieldId,
    pub visible: bool,
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
    pub children: &'tree [Subtree],
    pub position: Length,
    pub child_index: u32,
    pub structural_child_index: u32,
    pub descendant_index: u32,
    pub alias_sequence: &'static [Symbol],
    pub field_map: &'static [tree_sitter_language::FieldMapEntry],
    pub inherited_field_id: FieldId,
}
#[derive(Debug, Default)]
pub(crate) struct CursorStatus {
    pub field_id: FieldId,
    pub has_later_siblings: bool,
    pub has_later_named_siblings: bool,
    pub can_have_later_siblings_with_this_field: bool,
    pub supertypes: Vec<Symbol>,
}
#[inline]
pub(crate) fn ts_tree_cursor_is_entry_visible(cursor: &TreeCursor<'_>, index: u32) -> bool {
    index == 0 || cursor.stack[index as usize].visible
}

// These C-static-inline helpers pass large iterator/entry records. Keep them
// inside traversal loops so unused state updates and record copies disappear;
// ordinary inline hints leave both helpers out of line in release profiles.
#[inline(always)]
pub(crate) fn ts_tree_cursor_iterate_children<'tree>(
    cursor: &TreeCursor<'tree>,
) -> CursorChildIterator<'tree> {
    iterate_children_at(cursor, cursor.stack.len() - 1)
}

// Iterating a prefix of the stack lets sibling traversal preserve its original
// entries on failure, just as C restores the array's size after popping entries.
#[inline(always)]
fn iterate_children_at<'tree>(
    cursor: &TreeCursor<'tree>,
    index: usize,
) -> CursorChildIterator<'tree> {
    let entry = &cursor.stack[index];
    let children = ts_subtree_children(entry.subtree);
    if children.is_empty() {
        return CursorChildIterator {
            children: &[],
            position: length_zero(),
            child_index: 0,
            structural_child_index: 0,
            descendant_index: 0,
            alias_sequence: &[],
            field_map: &[],
            inherited_field_id: 0,
        };
    }
    let language = &cursor.tree.expect("cursor has a tree").language;
    let production_id = ts_subtree_production_id(entry.subtree) as u32;
    let alias_sequence = ts_language_alias_sequence(language, production_id);
    let field_map = ts_language_field_map(language, production_id);
    let descendant_index =
        entry
            .descendant_index
            .wrapping_add(u32::from(ts_tree_cursor_is_entry_visible(
                cursor,
                index as u32,
            )));
    CursorChildIterator {
        children,
        position: entry.position,
        child_index: 0,
        structural_child_index: 0,
        descendant_index,
        alias_sequence,
        field_map,
        inherited_field_id: if entry.visible { 0 } else { entry.field_id },
    }
}

impl CursorChildIterator<'_> {
    #[inline]
    fn alias(&self, extra: bool) -> Symbol {
        if extra || self.alias_sequence.is_empty() {
            0
        } else {
            self.alias_sequence[self.structural_child_index as usize]
        }
    }

    #[inline]
    fn field_id(&self, extra: bool) -> FieldId {
        if extra {
            0
        } else {
            self.field_map
                .iter()
                .find(|map| {
                    !map.inherited && u32::from(map.child_index) == self.structural_child_index
                })
                .map_or(self.inherited_field_id, |map| map.field_id)
        }
    }

    #[inline]
    fn edge_metadata(&self, child: &Subtree) -> (Symbol, FieldId, bool) {
        let extra = ts_subtree_extra(child);
        let alias = self.alias(extra);
        (
            alias,
            self.field_id(extra),
            alias != 0 || ts_subtree_visible(child),
        )
    }
}

#[inline(always)]
pub(crate) fn ts_tree_cursor_child_iterator_next<'tree>(
    iterator: &mut CursorChildIterator<'tree>,
) -> Option<(TreeCursorEntry<'tree>, bool)> {
    let children = iterator.children;
    let child = children.get(iterator.child_index as usize)?;
    let (alias, field_id, visible) = iterator.edge_metadata(child);
    let entry = TreeCursorEntry {
        alias,
        field_id,
        visible,
        subtree: child,
        position: iterator.position,
        child_index: iterator.child_index,
        structural_child_index: iterator.structural_child_index,
        descendant_index: iterator.descendant_index,
    };
    if !ts_subtree_extra(child) {
        iterator.structural_child_index = iterator.structural_child_index.wrapping_add(1);
    }
    iterator.descendant_index = iterator
        .descendant_index
        .wrapping_add(ts_subtree_visible_descendant_count(child))
        .wrapping_add(u32::from(visible));
    iterator.position = length_add(iterator.position, ts_subtree_size(child));
    iterator.child_index = iterator.child_index.wrapping_add(1);
    if let Some(next_child) = children.get(iterator.child_index as usize) {
        iterator.position = length_add(iterator.position, ts_subtree_padding(next_child));
    }
    Some((entry, visible))
}

pub(crate) fn length_backtrack(a: Length, b: Length) -> Length {
    if length_is_undefined(a) || b.extent.row != 0 {
        LENGTH_UNDEFINED
    } else {
        Length {
            bytes: a.bytes.wrapping_sub(b.bytes),
            extent: Point {
                row: a.extent.row,
                column: a.extent.column.wrapping_sub(b.extent.column),
            },
        }
    }
}

#[inline]
pub(crate) fn ts_tree_cursor_child_iterator_previous<'tree>(
    iterator: &mut CursorChildIterator<'tree>,
) -> Option<(TreeCursorEntry<'tree>, bool)> {
    // Preserve the C int8_t sentinel test, including its truncation to eight bits.
    if iterator.children.is_empty() || iterator.child_index as i8 == -1 {
        return None;
    }
    let children = iterator.children;
    let child = &children[iterator.child_index as usize];
    let (alias, field_id, visible) = iterator.edge_metadata(child);
    let entry = TreeCursorEntry {
        alias,
        field_id,
        visible,
        subtree: child,
        position: iterator.position,
        child_index: iterator.child_index,
        structural_child_index: iterator.structural_child_index,
        // C's reverse iterator zero-initializes this member, rather than tracking it.
        descendant_index: 0,
    };
    iterator.position = length_backtrack(iterator.position, ts_subtree_padding(child));
    iterator.child_index = iterator.child_index.wrapping_sub(1);
    if !ts_subtree_extra(child)
        && !iterator.alias_sequence.is_empty()
        && iterator.structural_child_index > 0
    {
        iterator.structural_child_index -= 1;
    }
    if let Some(previous_child) = children.get(iterator.child_index as usize) {
        iterator.position = length_backtrack(iterator.position, ts_subtree_size(previous_child));
    }
    Some((entry, visible))
}

pub(crate) fn ts_tree_cursor_new(node: Node<'_>) -> TreeCursor<'_> {
    let mut cursor = TreeCursor {
        tree: None,
        stack: Vec::new(),
        root_alias_symbol: 0,
    };
    ts_tree_cursor_init(&mut cursor, node);
    cursor
}

pub(crate) fn ts_tree_cursor_reset<'tree>(cursor: &mut TreeCursor<'tree>, node: Node<'tree>) {
    ts_tree_cursor_init(cursor, node);
}

pub(crate) fn ts_tree_cursor_init<'tree>(cursor: &mut TreeCursor<'tree>, node: Node<'tree>) {
    cursor.tree = Some(node.tree);
    cursor.root_alias_symbol = node.alias;
    cursor.stack.clear();
    cursor.stack.push(TreeCursorEntry {
        alias: if ts_subtree_extra(node.subtree) {
            0
        } else {
            node.alias
        },
        field_id: 0,
        visible: true,
        subtree: node.subtree,
        position: node.position,
        child_index: 0,
        structural_child_index: 0,
        descendant_index: 0,
    });
}

pub(crate) fn ts_tree_cursor_delete(cursor: &mut TreeCursor<'_>) {
    cursor.stack = Vec::new();
}

// Find an edge to enter, rather than advancing an iterator past an edge that
// traversal immediately returns. In particular, do not resolve fields or read
// the following child's padding until we know they are needed.
#[inline(always)]
fn find_forward_child<'tree>(
    mut iterator: CursorChildIterator<'tree>,
) -> Option<(TreeCursorEntry<'tree>, TreeCursorStep)> {
    loop {
        let child = iterator.children.get(iterator.child_index as usize)?;
        let extra = ts_subtree_extra(child);
        let alias = iterator.alias(extra);
        let visible = alias != 0 || ts_subtree_visible(child);
        if visible || ts_subtree_visible_child_count(child) > 0 {
            let field_id = iterator.field_id(extra);
            return Some((
                TreeCursorEntry {
                    subtree: child,
                    position: iterator.position,
                    child_index: iterator.child_index,
                    structural_child_index: iterator.structural_child_index,
                    descendant_index: iterator.descendant_index,
                    alias,
                    field_id,
                    visible,
                },
                if visible {
                    TreeCursorStep::Visible
                } else {
                    TreeCursorStep::Hidden
                },
            ));
        }
        iterator.structural_child_index = iterator
            .structural_child_index
            .wrapping_add(u32::from(!extra));
        iterator.descendant_index = iterator
            .descendant_index
            .wrapping_add(ts_subtree_visible_descendant_count(child));
        iterator.position = length_add(iterator.position, ts_subtree_size(child));
        iterator.child_index = iterator.child_index.wrapping_add(1);
        if let Some(next) = iterator.children.get(iterator.child_index as usize) {
            iterator.position = length_add(iterator.position, ts_subtree_padding(next));
        }
    }
}

#[inline(always)]
pub(crate) fn ts_tree_cursor_goto_first_child_internal(
    cursor: &mut TreeCursor<'_>,
) -> TreeCursorStep {
    let iterator = ts_tree_cursor_iterate_children(cursor);
    if let Some((entry, step)) = find_forward_child(iterator) {
        cursor.stack.push(entry);
        step
    } else {
        TreeCursorStep::None
    }
}

#[inline]
pub(crate) fn ts_tree_cursor_goto_first_child(cursor: &mut TreeCursor<'_>) -> bool {
    if ts_subtree_children(cursor.stack.last().expect("nonempty cursor").subtree).is_empty() {
        return false;
    }
    goto_first_child_nonempty(cursor)
}

#[inline(never)]
fn goto_first_child_nonempty(cursor: &mut TreeCursor<'_>) -> bool {
    loop {
        match ts_tree_cursor_goto_first_child_internal(cursor) {
            TreeCursorStep::Hidden => continue,
            TreeCursorStep::Visible => return true,
            TreeCursorStep::None => return false,
        }
    }
}

pub(crate) fn ts_tree_cursor_goto_last_child_internal(
    cursor: &mut TreeCursor<'_>,
) -> TreeCursorStep {
    let mut iterator = ts_tree_cursor_iterate_children(cursor);
    let mut last = None;
    while let Some((entry, visible)) = ts_tree_cursor_child_iterator_next(&mut iterator) {
        if visible {
            last = Some((entry, TreeCursorStep::Visible));
        } else if ts_subtree_visible_child_count(entry.subtree) > 0 {
            last = Some((entry, TreeCursorStep::Hidden));
        }
    }
    if let Some((entry, step)) = last {
        cursor.stack.push(entry);
        step
    } else {
        TreeCursorStep::None
    }
}

pub(crate) fn ts_tree_cursor_goto_last_child(cursor: &mut TreeCursor<'_>) -> bool {
    loop {
        match ts_tree_cursor_goto_last_child_internal(cursor) {
            TreeCursorStep::Hidden => continue,
            TreeCursorStep::Visible => return true,
            TreeCursorStep::None => return false,
        }
    }
}

pub(crate) fn ts_tree_cursor_goto_first_child_for_byte_and_point(
    cursor: &mut TreeCursor<'_>,
    goal_byte: u32,
    goal_point: Point,
) -> Option<u32> {
    let initial_size = cursor.stack.len();
    let mut visible_child_index: u32 = 0;
    loop {
        let mut did_descend = false;
        let mut iterator = ts_tree_cursor_iterate_children(cursor);
        while let Some((entry, visible)) = ts_tree_cursor_child_iterator_next(&mut iterator) {
            let end = length_add(entry.position, ts_subtree_size(entry.subtree));
            let at_goal = end.bytes > goal_byte && end.extent > goal_point;
            let visible_child_count = ts_subtree_visible_child_count(entry.subtree);
            if at_goal {
                if visible {
                    cursor.stack.push(entry);
                    return Some(visible_child_index);
                }
                if visible_child_count > 0 {
                    cursor.stack.push(entry);
                    did_descend = true;
                    break;
                }
            } else if visible {
                visible_child_index = visible_child_index.wrapping_add(1);
            } else {
                visible_child_index = visible_child_index.wrapping_add(visible_child_count);
            }
        }
        if !did_descend {
            cursor.stack.truncate(initial_size);
            return None;
        }
    }
}

pub(crate) fn ts_tree_cursor_goto_first_child_for_byte(
    cursor: &mut TreeCursor<'_>,
    goal_byte: u32,
) -> Option<u32> {
    ts_tree_cursor_goto_first_child_for_byte_and_point(cursor, goal_byte, POINT_ZERO)
}

pub(crate) fn ts_tree_cursor_goto_first_child_for_point(
    cursor: &mut TreeCursor<'_>,
    goal_point: Point,
) -> Option<u32> {
    ts_tree_cursor_goto_first_child_for_byte_and_point(cursor, 0, goal_point)
}

fn goto_previous_sibling_internal(cursor: &mut TreeCursor<'_>) -> TreeCursorStep {
    let initial_size = cursor.stack.len();
    let mut size = initial_size;
    while size > 1 {
        let entry = cursor.stack[size - 1];
        size -= 1;
        let mut iterator = iterate_children_at(cursor, size - 1);
        iterator.child_index = entry.child_index;
        iterator.structural_child_index = entry.structural_child_index;
        iterator.position = entry.position;
        iterator.descendant_index = entry.descendant_index;
        let visible = ts_tree_cursor_child_iterator_previous(&mut iterator)
            .is_some_and(|(_, visible)| visible);
        if visible && size + 1 < initial_size {
            break;
        }
        while let Some((entry, visible)) = ts_tree_cursor_child_iterator_previous(&mut iterator) {
            let step = if visible {
                TreeCursorStep::Visible
            } else if ts_subtree_visible_child_count(entry.subtree) > 0 {
                TreeCursorStep::Hidden
            } else {
                continue;
            };
            cursor.stack.truncate(size);
            cursor.stack.push(entry);
            return step;
        }
    }
    TreeCursorStep::None
}

pub(crate) fn ts_tree_cursor_goto_next_sibling_internal(
    cursor: &mut TreeCursor<'_>,
) -> TreeCursorStep {
    let initial_size = cursor.stack.len();
    for size in (1..initial_size).rev() {
        let entry = &cursor.stack[size];
        if entry.visible && size + 1 < initial_size {
            break;
        }
        let children = ts_subtree_children(cursor.stack[size - 1].subtree);
        let child_index = entry.child_index.wrapping_add(1);
        let Some(next) = children.get(child_index as usize) else {
            continue;
        };
        let mut iterator = iterate_children_at(cursor, size - 1);
        iterator.child_index = child_index;
        iterator.structural_child_index = entry
            .structural_child_index
            .wrapping_add(u32::from(!ts_subtree_extra(entry.subtree)));
        iterator.descendant_index = entry
            .descendant_index
            .wrapping_add(ts_subtree_visible_descendant_count(entry.subtree))
            .wrapping_add(u32::from(entry.visible));
        iterator.position = length_add(
            length_add(entry.position, ts_subtree_size(entry.subtree)),
            ts_subtree_padding(next),
        );
        if let Some((entry, step)) = find_forward_child(iterator) {
            // This slot already exists: no capacity check or grow path is
            // necessary when replacing the current edge by a sibling.
            cursor.stack[size] = entry;
            cursor.stack.truncate(size + 1);
            return step;
        }
    }
    TreeCursorStep::None
}

pub(crate) fn ts_tree_cursor_goto_next_sibling(cursor: &mut TreeCursor<'_>) -> bool {
    match ts_tree_cursor_goto_next_sibling_internal(cursor) {
        TreeCursorStep::Hidden => {
            ts_tree_cursor_goto_first_child(cursor);
            true
        }
        TreeCursorStep::Visible => true,
        TreeCursorStep::None => false,
    }
}

pub(crate) fn ts_tree_cursor_goto_previous_sibling_internal(
    cursor: &mut TreeCursor<'_>,
) -> TreeCursorStep {
    let step = goto_previous_sibling_internal(cursor);
    if step == TreeCursorStep::None {
        return step;
    }
    let entry = cursor.stack.last().expect("nonempty cursor");
    if !length_is_undefined(entry.position) {
        return step;
    }
    let child_index = entry.child_index as usize;
    let parent = &cursor.stack[cursor.stack.len() - 2];
    let mut position = parent.position;
    let children = ts_subtree_children(parent.subtree);
    if child_index > 0 {
        // The first child's position equals its parent's; skip its padding.
        position = length_add(position, ts_subtree_size(&children[0]));
        for child in &children[1..child_index] {
            position = length_add(position, ts_subtree_total_size(child));
        }
        position = length_add(position, ts_subtree_padding(&children[child_index]));
    }
    cursor.stack.last_mut().expect("nonempty cursor").position = position;
    step
}

pub(crate) fn ts_tree_cursor_goto_previous_sibling(cursor: &mut TreeCursor<'_>) -> bool {
    match ts_tree_cursor_goto_previous_sibling_internal(cursor) {
        TreeCursorStep::Hidden => {
            ts_tree_cursor_goto_last_child(cursor);
            true
        }
        TreeCursorStep::Visible => true,
        TreeCursorStep::None => false,
    }
}

#[inline]
pub(crate) fn ts_tree_cursor_goto_parent(cursor: &mut TreeCursor<'_>) -> bool {
    for i in (0..cursor.stack.len() - 1).rev() {
        if ts_tree_cursor_is_entry_visible(cursor, i as u32) {
            cursor.stack.truncate(i + 1);
            return true;
        }
    }
    false
}

pub(crate) fn ts_tree_cursor_goto_descendant(
    cursor: &mut TreeCursor<'_>,
    goal_descendant_index: u32,
) {
    // Ascend to the lowest ancestor containing the goal node.
    loop {
        let i = cursor.stack.len() - 1;
        let entry = &cursor.stack[i];
        let next_descendant_index = entry
            .descendant_index
            .wrapping_add(u32::from(ts_tree_cursor_is_entry_visible(cursor, i as u32)))
            .wrapping_add(ts_subtree_visible_descendant_count(entry.subtree));
        if entry.descendant_index <= goal_descendant_index
            && next_descendant_index > goal_descendant_index
        {
            break;
        } else if cursor.stack.len() <= 1 {
            return;
        } else {
            cursor.stack.pop();
        }
    }
    // Descend to the goal node.
    loop {
        let mut did_descend = false;
        let mut iterator = ts_tree_cursor_iterate_children(cursor);
        if iterator.descendant_index > goal_descendant_index {
            return;
        }
        while let Some((entry, visible)) = ts_tree_cursor_child_iterator_next(&mut iterator) {
            if iterator.descendant_index > goal_descendant_index {
                cursor.stack.push(entry);
                if visible && entry.descendant_index == goal_descendant_index {
                    return;
                } else {
                    did_descend = true;
                    break;
                }
            }
        }
        if !did_descend {
            return;
        }
    }
}

pub(crate) fn ts_tree_cursor_current_descendant_index(cursor: &TreeCursor<'_>) -> u32 {
    cursor
        .stack
        .last()
        .expect("nonempty cursor")
        .descendant_index
}

#[inline]
pub(crate) fn ts_tree_cursor_current_node<'tree>(cursor: &TreeCursor<'tree>) -> Node<'tree> {
    let entry = cursor.stack.last().expect("nonempty cursor");
    ts_node_new(
        cursor.tree.expect("cursor has a tree"),
        entry.subtree,
        entry.position,
        entry.alias,
    )
}

pub(crate) fn ts_tree_cursor_current_status(cursor: &TreeCursor<'_>) -> CursorStatus {
    let mut status = CursorStatus::default();
    // Fields can refer to children through invisible wrapper nodes.
    for i in (1..cursor.stack.len()).rev() {
        let entry = &cursor.stack[i];
        let parent = &cursor.stack[i - 1];
        let language = &cursor.tree.expect("cursor has a tree").language;
        let production_id = ts_subtree_production_id(parent.subtree) as u32;
        let aliases = ts_language_alias_sequence(language, production_id);
        let subtree_symbol = |subtree: &Subtree, structural_child_index: u32| {
            if !ts_subtree_extra(subtree) && !aliases.is_empty() {
                let alias = aliases[structural_child_index as usize];
                if alias != 0 {
                    return alias;
                }
            }
            ts_subtree_symbol(subtree)
        };
        let entry_symbol = subtree_symbol(entry.subtree, entry.structural_child_index);
        let metadata = ts_language_symbol_metadata(language, entry_symbol);
        if i != cursor.stack.len() - 1 && metadata.visible {
            break;
        }
        // The Rust out-parameter record owns its buffer, so no caller capacity
        // limits the number of supertypes recorded.
        if metadata.supertype {
            status.supertypes.push(entry_symbol);
        }
        if !status.has_later_siblings {
            let mut structural_child_index = entry.structural_child_index;
            if !ts_subtree_extra(entry.subtree) {
                structural_child_index = structural_child_index.wrapping_add(1);
            }
            for sibling in &ts_subtree_children(parent.subtree)[entry.child_index as usize + 1..] {
                let metadata = ts_language_symbol_metadata(
                    language,
                    subtree_symbol(sibling, structural_child_index),
                );
                if metadata.visible {
                    status.has_later_siblings = true;
                    if status.has_later_named_siblings {
                        break;
                    }
                    if metadata.named {
                        status.has_later_named_siblings = true;
                        break;
                    }
                } else if ts_subtree_visible_child_count(sibling) > 0 {
                    status.has_later_siblings = true;
                    if status.has_later_named_siblings {
                        break;
                    }
                    if ts_subtree_named_child_count(sibling) > 0 {
                        status.has_later_named_siblings = true;
                        break;
                    }
                }
                if !ts_subtree_extra(sibling) {
                    structural_child_index = structural_child_index.wrapping_add(1);
                }
            }
        }
        if !ts_subtree_extra(entry.subtree) {
            let field_map = ts_language_field_map(language, production_id);
            if status.field_id == 0 {
                for map in field_map {
                    if !map.inherited && u32::from(map.child_index) == entry.structural_child_index
                    {
                        status.field_id = map.field_id;
                        break;
                    }
                }
            }
            if status.field_id != 0 {
                for map in field_map {
                    if map.field_id == status.field_id
                        && u32::from(map.child_index) > entry.structural_child_index
                    {
                        status.can_have_later_siblings_with_this_field = true;
                        break;
                    }
                }
            }
        }
    }
    status
}

pub(crate) fn ts_tree_cursor_current_depth(cursor: &TreeCursor<'_>) -> u32 {
    let mut depth: u32 = 0;
    for i in 1..cursor.stack.len() {
        if ts_tree_cursor_is_entry_visible(cursor, i as u32) {
            depth = depth.wrapping_add(1);
        }
    }
    depth
}

pub(crate) fn ts_tree_cursor_parent_node<'tree>(cursor: &TreeCursor<'tree>) -> Option<Node<'tree>> {
    for i in (0..cursor.stack.len() - 1).rev() {
        let entry = &cursor.stack[i];
        let tree = cursor.tree.expect("cursor has a tree");
        let mut visible = true;
        let mut alias = 0;
        if i > 0 {
            let parent = &cursor.stack[i - 1];
            alias = ts_language_alias_at(
                &tree.language,
                ts_subtree_production_id(parent.subtree) as u32,
                entry.structural_child_index,
            );
            visible = alias != 0 || ts_subtree_visible(entry.subtree);
        }
        if visible {
            return Some(ts_node_new(tree, entry.subtree, entry.position, alias));
        }
    }
    None
}

#[inline]
pub(crate) fn ts_tree_cursor_current_field_id(cursor: &TreeCursor<'_>) -> FieldId {
    cursor.stack.last().expect("nonempty cursor").field_id
}

pub(crate) fn ts_tree_cursor_current_field_name(cursor: &TreeCursor<'_>) -> Option<&'static str> {
    let id = ts_tree_cursor_current_field_id(cursor);
    if id != 0 {
        ts_language_field_name_for_id(&cursor.tree.expect("cursor has a tree").language, id)
    } else {
        None
    }
}

pub(crate) fn ts_tree_cursor_copy<'tree>(cursor: &TreeCursor<'tree>) -> TreeCursor<'tree> {
    TreeCursor {
        tree: cursor.tree,
        stack: cursor.stack.clone(),
        root_alias_symbol: cursor.root_alias_symbol,
    }
}

pub(crate) fn ts_tree_cursor_reset_to<'tree>(
    destination: &mut TreeCursor<'tree>,
    source: &TreeCursor<'tree>,
) {
    destination.tree = source.tree;
    destination.root_alias_symbol = source.root_alias_symbol;
    destination.stack.clear();
    destination.stack.extend_from_slice(&source.stack);
}

pub(crate) fn ts_tree_cursor_current_subtree<'tree>(cursor: &TreeCursor<'tree>) -> &'tree Subtree {
    cursor.stack.last().expect("nonempty cursor").subtree
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    fn length(bytes: u32, row: u32, column: u32) -> Length {
        Length {
            bytes,
            extent: Point { row, column },
        }
    }

    fn leaf(size: u8, padding: u8, flags: u8) -> Subtree {
        Subtree::inline(InlineLeaf {
            size_bytes: size,
            padding_bytes: padding,
            padding_columns: padding,
            flags,
            ..InlineLeaf::default()
        })
    }

    fn parent(children: Vec<Subtree>) -> Subtree {
        Subtree::Heap(Arc::new(SubtreeHeapData {
            children,
            payload: SubtreePayload::Leaf,
            ..SubtreeHeapData::default()
        }))
    }

    fn iterator<'tree>(
        parent: &'tree Subtree,
        aliases: &'static [Symbol],
    ) -> CursorChildIterator<'tree> {
        CursorChildIterator {
            children: ts_subtree_children(parent),
            position: length_zero(),
            child_index: 0,
            structural_child_index: 0,
            descendant_index: 0,
            alias_sequence: aliases,
            field_map: &[],
            inherited_field_id: 0,
        }
    }

    #[test]
    fn forward_iteration_counts_aliases_extras_and_hidden_descendants() {
        let hidden = Subtree::Heap(Arc::new(SubtreeHeapData {
            size: length(3, 0, 3),
            children: Vec::new(),
            payload: SubtreePayload::Branch(BranchData {
                visible_descendant_count: 2,
                ..BranchData::default()
            }),
            ..SubtreeHeapData::default()
        }));
        let parent = parent(vec![hidden, leaf(1, 2, EXTRA), leaf(2, 3, 0)]);
        let mut iterator = iterator(&parent, &[0, 8]);
        iterator.position = length(10, 1, 5);
        iterator.descendant_index = 1;

        let (first, visible) = ts_tree_cursor_child_iterator_next(&mut iterator).unwrap();
        assert!(!visible);
        assert_eq!(first.position, length(10, 1, 5));
        assert_eq!(first.structural_child_index, 0);
        assert_eq!(first.descendant_index, 1);
        assert!(std::ptr::eq(
            first.subtree,
            &ts_subtree_children(&parent)[0]
        ));

        let (extra, visible) = ts_tree_cursor_child_iterator_next(&mut iterator).unwrap();
        assert!(!visible, "extras do not receive aliases");
        assert_eq!(extra.position, length(15, 1, 10));
        assert_eq!(extra.child_index, 1);
        assert_eq!(extra.structural_child_index, 1);
        assert_eq!(extra.descendant_index, 3);

        let (aliased, visible) = ts_tree_cursor_child_iterator_next(&mut iterator).unwrap();
        assert!(visible);
        assert_eq!(aliased.position, length(19, 1, 14));
        assert_eq!(aliased.child_index, 2);
        assert_eq!(aliased.structural_child_index, 1);
        assert_eq!(aliased.descendant_index, 3);
        assert_eq!(iterator.descendant_index, 4);
        assert_eq!(iterator.position, length(21, 1, 16));
        assert!(ts_tree_cursor_child_iterator_next(&mut iterator).is_none());
    }

    #[test]
    fn forward_seek_matches_iteration_through_uninteresting_edges() {
        use tree_sitter_language::FieldMapEntry;
        // Include extras, line breaks, aliases, skipped leaves, and invisible
        // branches. The seek path must retain exactly the iterator's position,
        // structural index, descendant index, and inherited field semantics.
        for skipped_count in 0..8 {
            for selected in 0..6 {
                let mut children = Vec::new();
                for index in 0..skipped_count {
                    children.push(Subtree::Heap(Arc::new(SubtreeHeapData {
                        size: length(3, 1, 2),
                        padding: length(2, 0, 2),
                        extra: index % 2 == 0,
                        children: Vec::new(),
                        payload: SubtreePayload::Leaf,
                        ..SubtreeHeapData::default()
                    })));
                }
                children.push(match selected {
                    0 => leaf(1, 2, VISIBLE),
                    1 => leaf(1, 2, 0), // Made visible by an alias below.
                    2 => Subtree::Heap(Arc::new(SubtreeHeapData {
                        size: length(4, 2, 1),
                        padding: length(2, 1, 1),
                        children: Vec::new(),
                        payload: SubtreePayload::Branch(BranchData {
                            visible_child_count: 2,
                            visible_descendant_count: 3,
                            ..BranchData::default()
                        }),
                        ..SubtreeHeapData::default()
                    })),
                    3 => leaf(1, 2, EXTRA | VISIBLE),
                    4 => leaf(1, 2, EXTRA),
                    _ => leaf(1, 2, 0),
                });
                let parent = parent(children);
                let aliases: &'static [Symbol] = if selected == 1 {
                    match skipped_count / 2 {
                        0 => &[7],
                        1 => &[0, 7],
                        2 => &[0, 0, 7],
                        _ => &[0, 0, 0, 7],
                    }
                } else {
                    &[]
                };
                let make_iterator = || {
                    let mut iter = iterator(&parent, aliases);
                    iter.position = length(11, 2, 4);
                    iter.descendant_index = 17;
                    iter.inherited_field_id = 8;
                    iter.field_map = &[
                        FieldMapEntry {
                            field_id: 2,
                            child_index: 1,
                            inherited: true,
                        },
                        FieldMapEntry {
                            field_id: 3,
                            child_index: 1,
                            inherited: false,
                        },
                    ];
                    iter
                };
                let actual = find_forward_child(make_iterator());
                let mut reference = make_iterator();
                let expected = loop {
                    let Some((entry, visible)) = ts_tree_cursor_child_iterator_next(&mut reference)
                    else {
                        break None;
                    };
                    if visible {
                        break Some((entry, TreeCursorStep::Visible));
                    }
                    if ts_subtree_visible_child_count(entry.subtree) > 0 {
                        break Some((entry, TreeCursorStep::Hidden));
                    }
                };
                match (actual, expected) {
                    (None, None) => (),
                    (Some((actual, actual_step)), Some((expected, expected_step))) => {
                        assert_eq!(actual_step, expected_step);
                        assert!(std::ptr::eq(actual.subtree, expected.subtree));
                        assert_eq!(actual.position, expected.position);
                        assert_eq!(actual.child_index, expected.child_index);
                        assert_eq!(
                            actual.structural_child_index,
                            expected.structural_child_index
                        );
                        assert_eq!(actual.descendant_index, expected.descendant_index);
                        assert_eq!(actual.alias, expected.alias);
                        assert_eq!(actual.field_id, expected.field_id);
                        assert_eq!(actual.visible, expected.visible);
                    }
                    _ => panic!("seek and iterator disagree for {skipped_count}/{selected}"),
                }
            }
        }
    }

    #[test]
    fn reverse_iteration_preserves_c_index_and_alias_rules() {
        let parent = parent(vec![leaf(2, 0, VISIBLE), leaf(1, 1, EXTRA), leaf(3, 2, 0)]);
        let mut iterator = iterator(&parent, &[7, 8]);
        iterator.position = length(6, 0, 6);
        iterator.child_index = 2;
        iterator.structural_child_index = 1;
        iterator.descendant_index = 100;

        let (last, visible) = ts_tree_cursor_child_iterator_previous(&mut iterator).unwrap();
        assert!(visible);
        assert_eq!(last.position, length(6, 0, 6));
        assert_eq!(last.descendant_index, 0);
        assert_eq!(iterator.position, length(3, 0, 3));
        assert_eq!(iterator.structural_child_index, 0);
        let (extra, visible) = ts_tree_cursor_child_iterator_previous(&mut iterator).unwrap();
        assert!(!visible);
        assert_eq!(extra.position, length(3, 0, 3));
        assert_eq!(iterator.structural_child_index, 0);
        let (first, visible) = ts_tree_cursor_child_iterator_previous(&mut iterator).unwrap();
        assert!(visible);
        assert_eq!(first.position, length_zero());
        assert_eq!(iterator.child_index, u32::MAX);
        assert!(ts_tree_cursor_child_iterator_previous(&mut iterator).is_none());

        // C decrements structural indices only when an alias sequence exists.
        iterator.alias_sequence = &[];
        iterator.child_index = 2;
        iterator.structural_child_index = 1;
        ts_tree_cursor_child_iterator_previous(&mut iterator).unwrap();
        assert_eq!(iterator.structural_child_index, 1);
    }

    #[test]
    fn reverse_iteration_uses_an_eight_bit_sentinel() {
        let parent = parent(vec![leaf(1, 0, VISIBLE); 257]);
        let mut iterator = iterator(&parent, &[]);
        iterator.child_index = 256;
        iterator.position = length(256, 0, 256);
        let (entry, visible) = ts_tree_cursor_child_iterator_previous(&mut iterator).unwrap();
        assert!(visible);
        assert_eq!(entry.child_index, 256);
        // This seemingly premature stop is intentional: (int8_t)255 == -1.
        assert!(ts_tree_cursor_child_iterator_previous(&mut iterator).is_none());
    }

    #[test]
    fn backtracking_marks_unknown_columns_and_wraps_unsigned_arithmetic() {
        assert_eq!(
            length_backtrack(length(12, 3, 9), length(4, 0, 4)),
            length(8, 3, 5)
        );
        assert_eq!(
            length_backtrack(length(12, 3, 9), length(4, 1, 0)),
            LENGTH_UNDEFINED
        );
        assert_eq!(
            length_backtrack(LENGTH_UNDEFINED, length_zero()),
            LENGTH_UNDEFINED
        );
        assert_eq!(
            length_backtrack(length_zero(), length(1, 0, 1)),
            length(u32::MAX, 0, u32::MAX)
        );

        let parent = parent(vec![
            leaf(2, 0, VISIBLE),
            Subtree::Heap(Arc::new(SubtreeHeapData {
                padding: length(3, 1, 1),
                size: length(1, 0, 1),
                visible: true,
                children: Vec::new(),
                payload: SubtreePayload::Leaf,
                ..SubtreeHeapData::default()
            })),
        ]);
        let mut iterator = iterator(&parent, &[]);
        iterator.child_index = 1;
        iterator.position = length(5, 1, 1);
        ts_tree_cursor_child_iterator_previous(&mut iterator).unwrap();
        assert_eq!(iterator.position, LENGTH_UNDEFINED);
        let (entry, _) = ts_tree_cursor_child_iterator_previous(&mut iterator).unwrap();
        assert_eq!(entry.position, LENGTH_UNDEFINED);
    }

    #[test]
    fn cached_edge_metadata_preserves_field_precedence_and_extra_boundaries() {
        use tree_sitter_language::FieldMapEntry;
        let parent = parent(vec![leaf(1, 0, 0), leaf(1, 0, EXTRA), leaf(1, 0, VISIBLE)]);
        let mut iter = iterator(&parent, &[5, 0]);
        iter.inherited_field_id = 7;
        iter.field_map = &[
            FieldMapEntry {
                field_id: 2,
                child_index: 0,
                inherited: true,
            },
            FieldMapEntry {
                field_id: 3,
                child_index: 0,
                inherited: false,
            },
            FieldMapEntry {
                field_id: 4,
                child_index: 0,
                inherited: false,
            },
        ];
        let (first, visible) = ts_tree_cursor_child_iterator_next(&mut iter).unwrap();
        assert_eq!(first.alias, 5);
        assert_eq!(first.field_id, 3);
        assert!(visible && first.visible);
        let (extra, visible) = ts_tree_cursor_child_iterator_next(&mut iter).unwrap();
        assert_eq!(extra.alias, 0);
        assert_eq!(extra.field_id, 0);
        assert!(!visible && !extra.visible);
        let (last, visible) = ts_tree_cursor_child_iterator_next(&mut iter).unwrap();
        assert_eq!(last.alias, 0);
        assert_eq!(last.field_id, 7);
        assert!(visible && last.visible);
    }

    #[test]
    fn cached_fields_follow_hidden_wrappers_but_stop_at_visible_and_extra_nodes() {
        use std::sync::LazyLock;
        use tree_sitter_language::{FieldMapEntry, LanguageTables, MapSlice, SymbolMetadata};
        // Reuse a decoded table fixture without lexing; replace only the edge
        // metadata needed to make inheritance boundaries explicit in this tree.
        static TABLES: LazyLock<LanguageTables> = LazyLock::new(|| {
            let mut tables = LanguageTables::decode(
                include_bytes!("../../grammars/c/src/tables.bin"),
                |_, _| unreachable!("cursor fixture does not lex"),
                Some(|_, _| unreachable!("cursor fixture does not lex keywords")),
                None,
            );
            for (symbol, visible) in [(1, true), (2, false), (4, true)] {
                tables.symbol_metadata[symbol] = SymbolMetadata {
                    visible,
                    named: visible,
                    supertype: false,
                };
            }
            tables.max_alias_sequence_length = 4;
            tables.alias_sequences = vec![0; 12];
            tables.alias_sequences[10] = 4; // production 2, structural child 2
            tables.field_map_slices = vec![
                MapSlice {
                    index: 0,
                    length: 0,
                },
                MapSlice {
                    index: 0,
                    length: 2,
                },
                MapSlice {
                    index: 0,
                    length: 0,
                },
            ];
            tables.field_map_entries = vec![
                FieldMapEntry {
                    field_id: 5,
                    child_index: 0,
                    inherited: false,
                },
                FieldMapEntry {
                    field_id: 6,
                    child_index: 1,
                    inherited: false,
                },
            ];
            tables
        });
        let language = Language::from(&*TABLES);
        let branch = |symbol, children, production| {
            ts_subtree_new_node(symbol, children, production, &language)
        };
        let token = || leaf(1, 0, VISIBLE | NAMED);
        let mut extra = branch(2, vec![token()], 0);
        extra.heap_mut().unwrap().extra = true;
        let wrapper = branch(
            2,
            vec![
                branch(2, vec![token()], 0),
                branch(1, vec![token()], 0),
                extra,
                branch(2, vec![token()], 0),
            ],
            2,
        );
        let tree = Tree {
            root: Box::new(branch(1, vec![wrapper, token()], 1)),
            language,
            included_ranges: Vec::new(),
        };
        let root = ts_node_new(&tree, &tree.root, length_zero(), 0);
        let mut cursor = ts_tree_cursor_new(root);
        let expected = [0, 5, 5, 0, 0, 5, 0, 6];
        let mut fields = Vec::new();
        loop {
            fields.push(ts_tree_cursor_current_field_id(&cursor));
            if ts_tree_cursor_goto_first_child(&mut cursor) {
                continue;
            }
            while !ts_tree_cursor_goto_next_sibling(&mut cursor) {
                if !ts_tree_cursor_goto_parent(&mut cursor) {
                    assert_eq!(fields, expected);
                    break;
                }
            }
            if cursor.stack.len() == 1 {
                break;
            }
        }
        // Descendant jumps and independent copies must carry the same fields.
        for (index, field) in expected.into_iter().enumerate().rev() {
            ts_tree_cursor_goto_descendant(&mut cursor, index as u32);
            assert_eq!(ts_tree_cursor_current_field_id(&cursor), field);
            assert_eq!(
                ts_tree_cursor_current_field_id(&ts_tree_cursor_copy(&cursor)),
                field
            );
        }
        ts_tree_cursor_goto_descendant(&mut cursor, 5);
        assert_eq!(ts_tree_cursor_current_node(&cursor).alias, 4);
        let aliased_node = ts_tree_cursor_current_node(&cursor);
        ts_tree_cursor_reset(&mut cursor, aliased_node);
        assert_eq!(ts_tree_cursor_current_field_id(&cursor), 0);
        assert_eq!(ts_tree_cursor_current_node(&cursor).alias, 4);
        assert!(ts_tree_cursor_goto_first_child(&mut cursor));
        assert_eq!(ts_tree_cursor_current_field_id(&cursor), 0);
    }

    #[test]
    fn leaf_cursor_failures_preserve_state_and_copies_are_independent() {
        let leaf = leaf(1, 0, 0);
        let mut cursor = TreeCursor {
            tree: None,
            stack: vec![TreeCursorEntry {
                subtree: &leaf,
                alias: 7,
                field_id: 0,
                visible: true,
                position: length(5, 1, 2),
                child_index: 0,
                structural_child_index: 0,
                descendant_index: 0,
            }],
            root_alias_symbol: 7,
        };
        assert!(ts_tree_cursor_is_entry_visible(&cursor, 0));
        assert!(!ts_tree_cursor_goto_first_child(&mut cursor));
        assert!(!ts_tree_cursor_goto_last_child(&mut cursor));
        assert!(!ts_tree_cursor_goto_next_sibling(&mut cursor));
        assert!(!ts_tree_cursor_goto_previous_sibling(&mut cursor));
        assert!(!ts_tree_cursor_goto_parent(&mut cursor));
        assert_eq!(
            ts_tree_cursor_goto_first_child_for_byte(&mut cursor, 0),
            None
        );
        assert_eq!(
            ts_tree_cursor_goto_first_child_for_point(&mut cursor, POINT_ZERO),
            None
        );
        ts_tree_cursor_goto_descendant(&mut cursor, 0);
        ts_tree_cursor_goto_descendant(&mut cursor, 10);
        assert_eq!(ts_tree_cursor_current_depth(&cursor), 0);
        assert_eq!(ts_tree_cursor_current_descendant_index(&cursor), 0);
        assert_eq!(ts_tree_cursor_current_field_name(&cursor), None);
        assert!(ts_tree_cursor_parent_node(&cursor).is_none());
        assert!(std::ptr::eq(ts_tree_cursor_current_subtree(&cursor), &leaf));
        assert_eq!(cursor.stack[0].position, length(5, 1, 2));

        let mut copy = ts_tree_cursor_copy(&cursor);
        ts_tree_cursor_delete(&mut cursor);
        assert!(cursor.stack.is_empty());
        assert_eq!(cursor.stack.capacity(), 0);
        assert_eq!(copy.stack.len(), 1);
        ts_tree_cursor_reset_to(&mut cursor, &copy);
        ts_tree_cursor_delete(&mut copy);
        assert_eq!(cursor.stack.len(), 1);
        assert_eq!(cursor.root_alias_symbol, 7);
        assert!(std::ptr::eq(cursor.stack[0].subtree, &leaf));
    }
}
