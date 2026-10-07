use crate::{language::Language, length::*, subtree::*, types::*};
pub(crate) type StackVersion = u32;
pub(crate) const STACK_VERSION_NONE: StackVersion = u32::MAX;
pub(crate) const MAX_LINK_COUNT: usize = 8;
pub(crate) const MAX_NODE_POOL_SIZE: usize = 50;
pub(crate) const MAX_ITERATOR_COUNT: usize = 64;
/// Stable slot index, never a pointer into a reallocating Vec. Live edges/heads
/// retain a slot; it is recycled only after its explicit reference count is zero.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct StackNodeId(pub usize);
#[derive(Clone, Debug)]
pub(crate) struct StackLink {
    pub node: StackNodeId,
    pub subtree: Subtree,
    pub is_pending: bool,
}
#[derive(Debug)]
pub(crate) struct StackNode {
    pub state: StateId,
    pub position: Length,
    pub links: [Option<StackLink>; MAX_LINK_COUNT],
    pub link_count: u16,
    pub ref_count: u32,
    pub error_cost: u32,
    pub node_count: u32,
    pub dynamic_precedence: i32,
}
#[derive(Debug, Default)]
pub(crate) struct StackArena {
    pub nodes: Vec<Option<StackNode>>,
    pub free: Vec<StackNodeId>,
}
#[derive(Clone, Debug)]
pub(crate) struct StackIterator {
    pub node: StackNodeId,
    pub subtrees: Vec<Subtree>,
    pub subtree_count: u32,
    pub is_pending: bool,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum StackStatus {
    Active,
    Paused,
    Halted,
}
#[derive(Debug)]
pub(crate) struct StackHead {
    pub node: StackNodeId,
    pub summary: Option<StackSummary>,
    pub node_count_at_last_error: u32,
    pub last_external_token: Subtree,
    pub lookahead_when_paused: Subtree,
    pub status: StackStatus,
}
#[derive(Debug)]
pub(crate) struct Stack {
    pub heads: Vec<StackHead>,
    pub slices: Vec<StackSlice>,
    pub iterators: Vec<StackIterator>,
    pub arena: StackArena,
    pub base_node: StackNodeId,
}
#[derive(Debug)]
pub(crate) struct StackSlice {
    pub subtrees: Vec<Subtree>,
    pub version: StackVersion,
}
#[derive(Clone, Copy, Debug)]
pub(crate) struct StackSummaryEntry {
    pub position: Length,
    pub depth: u32,
    pub state: StateId,
}
pub(crate) type StackSummary = Vec<StackSummaryEntry>;
pub(crate) type StackAction = u8;
pub(crate) const STACK_ACTION_NONE: StackAction = 0;
pub(crate) const STACK_ACTION_STOP: StackAction = 1;
pub(crate) const STACK_ACTION_POP: StackAction = 2;
pub(crate) struct SummarizeStackSession<'a> {
    pub summary: &'a mut StackSummary,
    pub max_depth: u32,
}

impl StackArena {
    fn node(&self, id: StackNodeId) -> &StackNode {
        self.nodes[id.0].as_ref().expect("live stack node")
    }

    fn node_mut(&mut self, id: StackNodeId) -> &mut StackNode {
        self.nodes[id.0].as_mut().expect("live stack node")
    }
}

// The public helper borrows its array, but iteration can transfer it without
// copying. Keep the slice/version ordering algorithm in this owning helper.
fn stack_add_slice_owned(
    stack: &mut Stack,
    original_version: StackVersion,
    node: StackNodeId,
    subtrees: Vec<Subtree>,
) {
    for i in (0..stack.slices.len()).rev() {
        let version = stack.slices[i].version;
        if stack.heads[version as usize].node == node {
            stack.slices.insert(i + 1, StackSlice { subtrees, version });
            return;
        }
    }
    let version = ts_stack__add_version(stack, original_version, node);
    stack.slices.push(StackSlice { subtrees, version });
}

pub(crate) fn stack_node_retain(arena: &mut StackArena, node: StackNodeId) {
    let node = arena.node_mut(node);
    assert!(node.ref_count > 0);
    node.ref_count = node.ref_count.wrapping_add(1);
    assert_ne!(node.ref_count, 0);
}

pub(crate) fn stack_node_release(
    arena: &mut StackArena,
    node: StackNodeId,
    subtree_pool: &mut SubtreePool,
) {
    // This worklist preserves C's release order: links N-1 through 1,
    // link 0's subtree, this node, then link 0's predecessor (the tail call).
    // In particular, a long unbranched stack never grows the worklist.
    enum Release {
        Node(StackNodeId),
        Subtree(Subtree),
        Recycle(StackNodeId),
    }
    let mut pending = vec![Release::Node(node)];
    while let Some(action) = pending.pop() {
        match action {
            Release::Subtree(subtree) => ts_subtree_release(subtree_pool, subtree),
            Release::Recycle(node) => arena.free.push(node),
            Release::Node(node) => {
                let data = arena.node_mut(node);
                assert_ne!(data.ref_count, 0);
                data.ref_count -= 1;
                if data.ref_count > 0 {
                    continue;
                }
                let mut data = arena.nodes[node.0].take().expect("live stack node");
                if let Some(first) = &data.links[0] {
                    pending.push(Release::Node(first.node));
                }
                pending.push(Release::Recycle(node));
                for (i, link) in data.links[..data.link_count as usize]
                    .iter_mut()
                    .enumerate()
                {
                    let link = link.take().expect("initialized stack link");
                    if i > 0 {
                        pending.push(Release::Node(link.node));
                    }
                    if !link.subtree.is_null() {
                        pending.push(Release::Subtree(link.subtree));
                    }
                }
            }
        }
    }
    // Vacant arena slots are bookkeeping, not individually allocated C nodes.
    // Keep all of them reusable, as required by the arena ownership contract.
}

pub(crate) fn stack__subtree_node_count(subtree: &Subtree) -> u32 {
    let mut count = ts_subtree_visible_descendant_count(subtree);
    if ts_subtree_visible(subtree) {
        count = count.wrapping_add(1);
    }
    // Invisible intermediate errors still count as progress after recovery.
    if ts_subtree_symbol(subtree) == BUILTIN_SYM_ERROR_REPEAT {
        count = count.wrapping_add(1);
    }
    count
}

pub(crate) fn stack_node_new(
    arena: &mut StackArena,
    previous_node: Option<StackNodeId>,
    subtree: Subtree,
    is_pending: bool,
    state: StateId,
) -> StackNodeId {
    let mut node = StackNode {
        state,
        position: length_zero(),
        links: std::array::from_fn(|_| None),
        link_count: 0,
        ref_count: 1,
        error_cost: 0,
        node_count: 0,
        dynamic_precedence: 0,
    };
    if let Some(previous_node) = previous_node {
        let previous = arena.node(previous_node);
        node.position = previous.position;
        node.error_cost = previous.error_cost;
        node.dynamic_precedence = previous.dynamic_precedence;
        node.node_count = previous.node_count;
        if !subtree.is_null() {
            node.error_cost = node
                .error_cost
                .wrapping_add(ts_subtree_error_cost(&subtree));
            node.position = length_add(node.position, ts_subtree_total_size(&subtree));
            node.node_count = node
                .node_count
                .wrapping_add(stack__subtree_node_count(&subtree));
            node.dynamic_precedence = node
                .dynamic_precedence
                .wrapping_add(ts_subtree_dynamic_precedence(&subtree));
        }
        // A push transfers the old head's ownership to this edge. Do not retain
        // the predecessor or subtree a second time.
        node.link_count = 1;
        node.links[0] = Some(StackLink {
            node: previous_node,
            subtree,
            is_pending,
        });
    }
    if let Some(id) = arena.free.pop() {
        assert!(arena.nodes[id.0].is_none());
        arena.nodes[id.0] = Some(node);
        id
    } else {
        let id = StackNodeId(arena.nodes.len());
        arena.nodes.push(Some(node));
        id
    }
}

pub(crate) fn stack__subtree_is_equivalent(left: &Subtree, right: &Subtree) -> bool {
    if left.ptr_eq(right) {
        return true;
    }
    if left.is_null() || right.is_null() {
        return false;
    }
    if ts_subtree_symbol(left) != ts_subtree_symbol(right) {
        return false;
    }
    if ts_subtree_error_cost(left) > 0 && ts_subtree_error_cost(right) > 0 {
        return true;
    }
    ts_subtree_padding(left).bytes == ts_subtree_padding(right).bytes
        && ts_subtree_size(left).bytes == ts_subtree_size(right).bytes
        && ts_subtree_child_count(left) == ts_subtree_child_count(right)
        && ts_subtree_extra(left) == ts_subtree_extra(right)
        && ts_subtree_external_scanner_state_eq(left, right)
}

pub(crate) fn stack_node_add_link(
    arena: &mut StackArena,
    node: StackNodeId,
    link: StackLink,
    subtree_pool: &mut SubtreePool,
) {
    if link.node == node {
        return;
    }
    for i in 0..arena.node(node).link_count as usize {
        let existing = arena.node(node).links[i]
            .as_ref()
            .expect("initialized stack link");
        if !stack__subtree_is_equivalent(&existing.subtree, &link.subtree) {
            continue;
        }
        let existing_node = existing.node;
        if existing_node == link.node {
            // Remove ambiguity early only for equivalent links directly joining
            // the same pair of nodes. Keep the pending flag of the old link.
            if ts_subtree_dynamic_precedence(&link.subtree)
                > ts_subtree_dynamic_precedence(&existing.subtree)
            {
                let precedence = arena
                    .node(link.node)
                    .dynamic_precedence
                    .wrapping_add(ts_subtree_dynamic_precedence(&link.subtree));
                let data = arena.node_mut(node);
                let old = std::mem::replace(
                    &mut data.links[i]
                        .as_mut()
                        .expect("initialized stack link")
                        .subtree,
                    link.subtree,
                );
                data.dynamic_precedence = precedence;
                ts_subtree_release(subtree_pool, old);
            }
            return;
        }
        let previous = arena.node(existing_node);
        let other = arena.node(link.node);
        if previous.state == other.state
            && previous.position.bytes == other.position.bytes
            && previous.error_cost == other.error_cost
        {
            // Read each edge after the preceding recursive merge, just as C
            // does; merging can mutate nodes shared by both paths.
            let mut j = 0;
            while j < arena.node(link.node).link_count as usize {
                let predecessor = arena.node(link.node).links[j]
                    .as_ref()
                    .expect("initialized stack link")
                    .clone();
                stack_node_add_link(arena, existing_node, predecessor, subtree_pool);
                j += 1;
            }
            let mut precedence = arena.node(link.node).dynamic_precedence;
            if !link.subtree.is_null() {
                precedence = precedence.wrapping_add(ts_subtree_dynamic_precedence(&link.subtree));
            }
            let data = arena.node_mut(node);
            data.dynamic_precedence = data.dynamic_precedence.max(precedence);
            return;
        }
    }
    if arena.node(node).link_count as usize == MAX_LINK_COUNT {
        return;
    }
    stack_node_retain(arena, link.node);
    let previous = arena.node(link.node);
    let mut node_count = previous.node_count;
    let mut precedence = previous.dynamic_precedence;
    if !link.subtree.is_null() {
        node_count = node_count.wrapping_add(stack__subtree_node_count(&link.subtree));
        precedence = precedence.wrapping_add(ts_subtree_dynamic_precedence(&link.subtree));
    }
    let data = arena.node_mut(node);
    data.links[data.link_count as usize] = Some(link);
    data.link_count += 1;
    data.node_count = data.node_count.max(node_count);
    data.dynamic_precedence = data.dynamic_precedence.max(precedence);
}

pub(crate) fn stack_head_delete(
    head: StackHead,
    arena: &mut StackArena,
    subtree_pool: &mut SubtreePool,
) {
    if !head.last_external_token.is_null() {
        ts_subtree_release(subtree_pool, head.last_external_token);
    }
    if !head.lookahead_when_paused.is_null() {
        ts_subtree_release(subtree_pool, head.lookahead_when_paused);
    }
    drop(head.summary);
    stack_node_release(arena, head.node, subtree_pool);
}

pub(crate) fn ts_stack__add_version(
    stack: &mut Stack,
    original_version: StackVersion,
    node: StackNodeId,
) -> StackVersion {
    let original = &stack.heads[original_version as usize];
    let head = StackHead {
        node,
        summary: None,
        node_count_at_last_error: original.node_count_at_last_error,
        last_external_token: original.last_external_token.clone(),
        lookahead_when_paused: Subtree::Null,
        status: StackStatus::Active,
    };
    stack.heads.push(head);
    stack_node_retain(&mut stack.arena, node);
    (stack.heads.len() - 1) as StackVersion
}

pub(crate) fn ts_stack__add_slice(
    stack: &mut Stack,
    original_version: StackVersion,
    node: StackNodeId,
    subtrees: &[Subtree],
) {
    stack_add_slice_owned(stack, original_version, node, subtrees.to_vec());
}

pub(crate) fn stack__iter(
    stack: &mut Stack,
    pool: &mut SubtreePool,
    version: StackVersion,
    callback: &mut dyn FnMut(&StackArena, &StackIterator) -> StackAction,
    goal_subtree_count: i32,
) -> Vec<StackSlice> {
    stack.slices.clear();
    stack.iterators.clear();
    let include_subtrees = goal_subtree_count >= 0;
    let mut subtrees = Vec::new();
    if include_subtrees {
        // C reserves room for children plus a heap header in the same buffer.
        // The Rust header is a separate allocation; only children go here.
        subtrees.reserve(goal_subtree_count as usize);
    }
    stack.iterators.push(StackIterator {
        node: stack.heads[version as usize].node,
        subtrees,
        subtree_count: 0,
        is_pending: true,
    });
    while !stack.iterators.is_empty() {
        // Branches appended during this pass are not visited until the next
        // pass. Erasing a stopped iterator shifts the remaining original ones.
        let mut size = stack.iterators.len();
        let mut i = 0;
        while i < size {
            let node = stack.iterators[i].node;
            let action = callback(&stack.arena, &stack.iterators[i]);
            let link_count = stack.arena.node(node).link_count as usize;
            let should_pop = action & STACK_ACTION_POP != 0;
            let should_stop = action & STACK_ACTION_STOP != 0 || link_count == 0;
            if should_pop {
                let mut subtrees = if should_stop {
                    std::mem::take(&mut stack.iterators[i].subtrees)
                } else {
                    stack.iterators[i].subtrees.clone()
                };
                subtrees.reverse();
                stack_add_slice_owned(stack, version, node, subtrees);
            }
            if should_stop {
                let mut iterator = stack.iterators.remove(i);
                if !should_pop {
                    ts_subtree_array_delete(pool, &mut iterator.subtrees);
                }
                size -= 1;
                continue;
            }
            for j in 1..=link_count {
                let (link_index, iterator_index) = if j == link_count {
                    (0, i)
                } else {
                    if stack.iterators.len() >= MAX_ITERATOR_COUNT {
                        continue;
                    }
                    stack.iterators.push(stack.iterators[i].clone());
                    (j, stack.iterators.len() - 1)
                };
                let link = stack.arena.node(node).links[link_index]
                    .as_ref()
                    .expect("initialized stack link");
                let next = &mut stack.iterators[iterator_index];
                next.node = link.node;
                if !link.subtree.is_null() {
                    if include_subtrees {
                        next.subtrees.push(link.subtree.clone());
                    }
                    if !ts_subtree_extra(&link.subtree) {
                        next.subtree_count = next.subtree_count.wrapping_add(1);
                        if !link.is_pending {
                            next.is_pending = false;
                        }
                    }
                } else {
                    next.subtree_count = next.subtree_count.wrapping_add(1);
                    next.is_pending = false;
                }
            }
            i += 1;
        }
    }
    // The public pop APIs transfer ownership, rather than aliasing the scratch
    // array as C does. Slices must not keep their subtrees alive in the stack.
    std::mem::take(&mut stack.slices)
}

pub(crate) fn ts_stack_new() -> Stack {
    let mut arena = StackArena {
        nodes: Vec::new(),
        free: Vec::with_capacity(MAX_NODE_POOL_SIZE),
    };
    let base_node = stack_node_new(&mut arena, None, Subtree::Null, false, 1);
    // ts_stack_clear on a newly empty stack only retains the base and adds an
    // active head. No subtree pool is needed for this initial clear.
    stack_node_retain(&mut arena, base_node);
    let mut heads = Vec::with_capacity(4);
    heads.push(StackHead {
        node: base_node,
        summary: None,
        node_count_at_last_error: 0,
        last_external_token: Subtree::Null,
        lookahead_when_paused: Subtree::Null,
        status: StackStatus::Active,
    });
    Stack {
        heads,
        slices: Vec::with_capacity(4),
        iterators: Vec::with_capacity(4),
        arena,
        base_node,
    }
}

pub(crate) fn ts_stack_delete(stack: &mut Stack, pool: &mut SubtreePool) {
    stack.slices = Vec::new();
    stack.iterators = Vec::new();
    stack_node_release(&mut stack.arena, stack.base_node, pool);
    for head in std::mem::take(&mut stack.heads) {
        stack_head_delete(head, &mut stack.arena, pool);
    }
    stack.arena = StackArena::default();
}

pub(crate) fn ts_stack_version_count(stack: &Stack) -> u32 {
    stack.heads.len() as u32
}

pub(crate) fn ts_stack_halted_version_count(stack: &Stack) -> u32 {
    stack
        .heads
        .iter()
        .filter(|head| head.status == StackStatus::Halted)
        .count() as u32
}

pub(crate) fn ts_stack_state(stack: &Stack, version: StackVersion) -> StateId {
    stack.arena.node(stack.heads[version as usize].node).state
}

pub(crate) fn ts_stack_position(stack: &Stack, version: StackVersion) -> Length {
    stack
        .arena
        .node(stack.heads[version as usize].node)
        .position
}

pub(crate) fn ts_stack_last_external_token(stack: &Stack, version: StackVersion) -> &Subtree {
    &stack.heads[version as usize].last_external_token
}

pub(crate) fn ts_stack_set_last_external_token(
    stack: &mut Stack,
    pool: &mut SubtreePool,
    version: StackVersion,
    token: Subtree,
) {
    // The owned argument is the retained handle; callers keeping their token
    // pass a clone. Retaining it again here would leak an extra reference.
    let old = std::mem::replace(
        &mut stack.heads[version as usize].last_external_token,
        token,
    );
    if !old.is_null() {
        ts_subtree_release(pool, old);
    }
}

pub(crate) fn ts_stack_error_cost(stack: &Stack, version: StackVersion) -> u32 {
    todo!("stack-2: ts_stack_error_cost")
}

pub(crate) fn ts_stack_node_count_since_error(stack: &Stack, version: StackVersion) -> u32 {
    todo!("stack-2: ts_stack_node_count_since_error")
}

pub(crate) fn ts_stack_push(
    stack: &mut Stack,
    pool: &mut SubtreePool,
    version: StackVersion,
    subtree: Subtree,
    pending: bool,
    state: StateId,
) {
    todo!("stack-2: ts_stack_push")
}

pub(crate) fn pop_count_callback(count: u32, iterator: &StackIterator) -> StackAction {
    todo!("stack-2: pop_count_callback")
}

pub(crate) fn ts_stack_pop_count(
    stack: &mut Stack,
    pool: &mut SubtreePool,
    version: StackVersion,
    count: u32,
) -> Vec<StackSlice> {
    todo!("stack-2: ts_stack_pop_count")
}

pub(crate) fn pop_pending_callback(iterator: &StackIterator) -> StackAction {
    todo!("stack-2: pop_pending_callback")
}

pub(crate) fn ts_stack_pop_pending(
    stack: &mut Stack,
    pool: &mut SubtreePool,
    version: StackVersion,
) -> Vec<StackSlice> {
    todo!("stack-2: ts_stack_pop_pending")
}

pub(crate) fn pop_error_callback(found_error: &mut bool, iterator: &StackIterator) -> StackAction {
    todo!("stack-2: pop_error_callback")
}

pub(crate) fn ts_stack_pop_error(
    stack: &mut Stack,
    pool: &mut SubtreePool,
    version: StackVersion,
) -> Vec<Subtree> {
    todo!("stack-2: ts_stack_pop_error")
}

pub(crate) fn pop_all_callback(arena: &StackArena, iterator: &StackIterator) -> StackAction {
    todo!("stack-2: pop_all_callback")
}

pub(crate) fn ts_stack_pop_all(
    stack: &mut Stack,
    pool: &mut SubtreePool,
    version: StackVersion,
) -> Vec<StackSlice> {
    todo!("stack-2: ts_stack_pop_all")
}

pub(crate) fn summarize_stack_callback(
    session: &mut SummarizeStackSession<'_>,
    arena: &StackArena,
    iterator: &StackIterator,
) -> StackAction {
    todo!("stack-2: summarize_stack_callback")
}

pub(crate) fn ts_stack_record_summary(
    stack: &mut Stack,
    pool: &mut SubtreePool,
    version: StackVersion,
    max_depth: u32,
) {
    todo!("stack-2: ts_stack_record_summary")
}

pub(crate) fn ts_stack_get_summary(stack: &Stack, version: StackVersion) -> Option<&StackSummary> {
    todo!("stack-2: ts_stack_get_summary")
}

pub(crate) fn ts_stack_dynamic_precedence(stack: &Stack, version: StackVersion) -> i32 {
    todo!("stack-2: ts_stack_dynamic_precedence")
}

pub(crate) fn ts_stack_has_advanced_since_error(stack: &Stack, version: StackVersion) -> bool {
    todo!("stack-2: ts_stack_has_advanced_since_error")
}

pub(crate) fn ts_stack_remove_version(
    stack: &mut Stack,
    pool: &mut SubtreePool,
    version: StackVersion,
) {
    todo!("stack-2: ts_stack_remove_version")
}

pub(crate) fn ts_stack_renumber_version(
    stack: &mut Stack,
    pool: &mut SubtreePool,
    v1: StackVersion,
    v2: StackVersion,
) {
    todo!("stack-2: ts_stack_renumber_version")
}

pub(crate) fn ts_stack_swap_versions(stack: &mut Stack, v1: StackVersion, v2: StackVersion) {
    todo!("stack-2: ts_stack_swap_versions")
}

pub(crate) fn ts_stack_copy_version(stack: &mut Stack, version: StackVersion) -> StackVersion {
    todo!("stack-2: ts_stack_copy_version")
}

pub(crate) fn ts_stack_merge(
    stack: &mut Stack,
    pool: &mut SubtreePool,
    version1: StackVersion,
    version2: StackVersion,
) -> bool {
    todo!("stack-2: ts_stack_merge")
}

pub(crate) fn ts_stack_can_merge(
    stack: &Stack,
    version1: StackVersion,
    version2: StackVersion,
) -> bool {
    todo!("stack-2: ts_stack_can_merge")
}

pub(crate) fn ts_stack_halt(stack: &mut Stack, version: StackVersion) {
    todo!("stack-2: ts_stack_halt")
}

pub(crate) fn ts_stack_pause(
    stack: &mut Stack,
    pool: &mut SubtreePool,
    version: StackVersion,
    lookahead: Subtree,
) {
    todo!("stack-2: ts_stack_pause")
}

pub(crate) fn ts_stack_is_active(stack: &Stack, version: StackVersion) -> bool {
    todo!("stack-2: ts_stack_is_active")
}

pub(crate) fn ts_stack_is_halted(stack: &Stack, version: StackVersion) -> bool {
    todo!("stack-2: ts_stack_is_halted")
}

pub(crate) fn ts_stack_is_paused(stack: &Stack, version: StackVersion) -> bool {
    todo!("stack-2: ts_stack_is_paused")
}

pub(crate) fn ts_stack_resume(stack: &mut Stack, version: StackVersion) -> Subtree {
    todo!("stack-2: ts_stack_resume")
}

pub(crate) fn ts_stack_clear(stack: &mut Stack, pool: &mut SubtreePool) {
    todo!("stack-2: ts_stack_clear")
}

pub(crate) fn ts_stack_print_dot_graph(
    stack: &Stack,
    language: &Language,
    output: &mut dyn std::io::Write,
) -> std::io::Result<()> {
    todo!("stack-2: ts_stack_print_dot_graph")
}

#[cfg(test)]
mod stack_1_tests {
    use super::*;
    use std::sync::Arc;

    fn leaf(symbol: u8, flags: u8) -> Subtree {
        Subtree::Inline(InlineLeaf {
            symbol,
            flags,
            size_bytes: 1,
            ..InlineLeaf::default()
        })
    }

    fn symbols(trees: &[Subtree]) -> Vec<Symbol> {
        trees.iter().map(ts_subtree_symbol).collect()
    }

    fn null_link(node: StackNodeId) -> StackLink {
        StackLink {
            node,
            subtree: Subtree::Null,
            is_pending: false,
        }
    }

    #[test]
    fn new_stack_and_added_version_metadata() {
        let mut stack = ts_stack_new();
        let mut pool = SubtreePool::default();
        assert_eq!(ts_stack_version_count(&stack), 1);
        assert_eq!(ts_stack_halted_version_count(&stack), 0);
        assert_eq!(ts_stack_state(&stack, 0), 1);
        assert_eq!(ts_stack_position(&stack, 0), length_zero());
        assert!(ts_stack_last_external_token(&stack, 0).is_null());
        assert_eq!(stack.arena.node(stack.base_node).ref_count, 2);

        let token = Subtree::Heap(Arc::new(SubtreeHeapData::default()));
        ts_stack_set_last_external_token(&mut stack, &mut pool, 0, token.clone());
        stack.heads[0].node_count_at_last_error = 17;
        stack.heads[0].summary = Some(Vec::new());
        stack.heads[0].lookahead_when_paused = leaf(9, 0);
        stack.heads[0].status = StackStatus::Paused;
        let base = stack.base_node;
        let version = ts_stack__add_version(&mut stack, 0, base);
        assert_eq!(version, 1);
        assert_eq!(ts_stack_version_count(&stack), 2);
        assert_eq!(stack.arena.node(base).ref_count, 3);
        let head = &stack.heads[version as usize];
        assert_eq!(head.status, StackStatus::Active);
        assert_eq!(head.node_count_at_last_error, 17);
        assert!(head.summary.is_none());
        assert!(head.lookahead_when_paused.is_null());
        assert!(head.last_external_token.ptr_eq(&token));
        assert_eq!(
            Arc::strong_count(match &token {
                Subtree::Heap(d) => d,
                _ => unreachable!(),
            }),
            3
        );
        stack.heads[0].status = StackStatus::Halted;
        assert_eq!(ts_stack_halted_version_count(&stack), 1);
    }

    #[test]
    fn shared_nodes_release_in_c_order_and_reuse_slots() {
        let mut arena = StackArena::default();
        let mut pool = SubtreePool::default();
        let base = stack_node_new(&mut arena, None, Subtree::Null, false, 1);
        let left = stack_node_new(&mut arena, Some(base), Subtree::Null, false, 2);
        stack_node_retain(&mut arena, base);
        let right = stack_node_new(&mut arena, Some(base), Subtree::Null, false, 3);
        let top = stack_node_new(&mut arena, Some(left), Subtree::Null, false, 4);
        stack_node_add_link(&mut arena, top, null_link(right), &mut pool);
        assert_eq!(arena.node(right).ref_count, 2);
        stack_node_release(&mut arena, right, &mut pool);
        stack_node_retain(&mut arena, top);
        stack_node_release(&mut arena, top, &mut pool);
        assert!(arena.free.is_empty());
        stack_node_release(&mut arena, top, &mut pool);
        assert!(arena.nodes.iter().all(Option::is_none));
        assert_eq!(arena.free, [right, top, left, base]);
        let reused = stack_node_new(&mut arena, None, Subtree::Null, false, 7);
        assert_eq!(reused, base);
        assert_eq!(arena.nodes.len(), 4);
        assert_eq!(arena.node(reused).state, 7);
        assert_eq!(arena.node(reused).ref_count, 1);
        assert_eq!(arena.node(reused).link_count, 0);
        assert!(arena.node(reused).links.iter().all(Option::is_none));
    }

    #[test]
    fn deep_release_uses_no_recursion() {
        std::thread::Builder::new()
            .stack_size(128 * 1024)
            .spawn(|| {
                let mut arena = StackArena::default();
                let mut pool = SubtreePool::default();
                let mut top = stack_node_new(&mut arena, None, Subtree::Null, false, 1);
                for _ in 0..50_000 {
                    top = stack_node_new(&mut arena, Some(top), Subtree::Null, false, 2);
                }
                stack_node_release(&mut arena, top, &mut pool);
                assert_eq!(arena.free.len(), 50_001);
                assert!(arena.nodes.iter().all(Option::is_none));
            })
            .unwrap()
            .join()
            .unwrap();
    }

    #[test]
    fn node_metrics_count_invisible_errors_and_add_extents() {
        let mut arena = StackArena::default();
        let base = stack_node_new(&mut arena, None, Subtree::Null, false, 1);
        let first_tree = Subtree::Inline(InlineLeaf {
            symbol: 3,
            flags: VISIBLE,
            size_bytes: 2,
            padding_bytes: 3,
            padding_columns: 1,
            padding_rows_and_lookahead: 1,
            ..InlineLeaf::default()
        });
        assert_eq!(stack__subtree_node_count(&first_tree), 1);
        assert_eq!(stack__subtree_node_count(&leaf(1, NAMED)), 0);
        let first = stack_node_new(&mut arena, Some(base), first_tree, true, 2);
        assert_eq!(
            arena.node(first).position,
            Length {
                bytes: 5,
                extent: Point { row: 1, column: 3 }
            }
        );
        let error = Subtree::Heap(Arc::new(SubtreeHeapData {
            symbol: BUILTIN_SYM_ERROR_REPEAT,
            padding: Length {
                bytes: 1,
                extent: Point { row: 0, column: 1 },
            },
            size: Length {
                bytes: 4,
                extent: Point { row: 2, column: 2 },
            },
            error_cost: 23,
            children: vec![leaf(2, VISIBLE)],
            payload: SubtreePayload::Branch(BranchData {
                visible_descendant_count: 3,
                dynamic_precedence: -7,
                ..BranchData::default()
            }),
            ..SubtreeHeapData::default()
        }));
        assert_eq!(stack__subtree_node_count(&error), 4);
        let second = stack_node_new(&mut arena, Some(first), error, false, 3);
        let data = arena.node(second);
        assert_eq!(
            data.position,
            Length {
                bytes: 10,
                extent: Point { row: 3, column: 2 }
            }
        );
        assert_eq!(data.error_cost, 23);
        assert_eq!(data.node_count, 5);
        assert_eq!(data.dynamic_precedence, -7);
        let recovery = stack_node_new(&mut arena, Some(second), Subtree::Null, false, 0);
        assert_eq!(arena.node(recovery).position, arena.node(second).position);
        assert_eq!(arena.node(recovery).node_count, 5);
        assert_eq!(arena.node(recovery).error_cost, 23);
        assert_eq!(arena.node(recovery).dynamic_precedence, -7);
    }

    #[test]
    fn equivalence_identity_symbols_and_error_shortcuts() {
        let a = leaf(1, 0);
        let b = leaf(2, 0);
        assert!(stack__subtree_is_equivalent(&Subtree::Null, &Subtree::Null));
        assert!(stack__subtree_is_equivalent(&a, &a.clone()));
        assert!(!stack__subtree_is_equivalent(&a, &Subtree::Null));
        assert!(!stack__subtree_is_equivalent(&a, &b));
        let error = Subtree::Heap(Arc::new(SubtreeHeapData {
            symbol: 9,
            error_cost: 1,
            children: Vec::new(),
            payload: SubtreePayload::Leaf,
            ..SubtreeHeapData::default()
        }));
        let mut other = error.clone();
        other.heap_mut().unwrap().error_cost = 10;
        other.heap_mut().unwrap().extra = true;
        other.heap_mut().unwrap().size.bytes = 100;
        assert!(!error.ptr_eq(&other));
        assert!(stack__subtree_is_equivalent(&error, &other));
        other.heap_mut().unwrap().symbol = 10;
        assert!(!stack__subtree_is_equivalent(&error, &other));
        other.heap_mut().unwrap().symbol = 9;
        other.heap_mut().unwrap().error_cost = 0;
        assert!(!stack__subtree_is_equivalent(&error, &other));
    }

    #[test]
    fn link_limit_self_links_and_duplicate_links() {
        let mut arena = StackArena::default();
        let mut pool = SubtreePool::default();
        let top = stack_node_new(&mut arena, None, Subtree::Null, false, 1);
        stack_node_add_link(&mut arena, top, null_link(top), &mut pool);
        assert_eq!(arena.node(top).link_count, 0);
        let mut predecessors = Vec::new();
        for i in 0..=MAX_LINK_COUNT {
            let previous =
                stack_node_new(&mut arena, None, Subtree::Null, false, (i + 2) as StateId);
            arena.node_mut(previous).node_count = (i + 1) as u32;
            arena.node_mut(previous).dynamic_precedence = i as i32;
            stack_node_add_link(&mut arena, top, null_link(previous), &mut pool);
            predecessors.push(previous);
        }
        assert_eq!(arena.node(top).link_count as usize, MAX_LINK_COUNT);
        assert_eq!(arena.node(top).node_count as usize, MAX_LINK_COUNT);
        assert_eq!(
            arena.node(top).dynamic_precedence as usize,
            MAX_LINK_COUNT - 1
        );
        for (i, &previous) in predecessors.iter().enumerate() {
            assert_eq!(
                arena.node(previous).ref_count,
                if i < MAX_LINK_COUNT { 2 } else { 1 }
            );
        }
        let mut duplicate = null_link(predecessors[0]);
        duplicate.is_pending = true;
        stack_node_add_link(&mut arena, top, duplicate, &mut pool);
        assert!(!arena.node(top).links[0].as_ref().unwrap().is_pending);
        assert_eq!(arena.node(predecessors[0]).ref_count, 2);
        stack_node_release(&mut arena, top, &mut pool);
        for previous in predecessors {
            assert_eq!(arena.node(previous).ref_count, 1);
            stack_node_release(&mut arena, previous, &mut pool);
        }
        assert!(arena.nodes.iter().all(Option::is_none));
    }

    #[test]
    fn recursively_merged_predecessors_keep_outer_node_count() {
        let mut arena = StackArena::default();
        let mut pool = SubtreePool::default();
        let p = stack_node_new(&mut arena, None, Subtree::Null, false, 1);
        let q = stack_node_new(&mut arena, None, Subtree::Null, false, 2);
        arena.node_mut(p).node_count = 2;
        arena.node_mut(q).node_count = 9;
        arena.node_mut(q).dynamic_precedence = 7;
        let left = stack_node_new(&mut arena, Some(p), Subtree::Null, false, 3);
        let right = stack_node_new(&mut arena, Some(q), Subtree::Null, false, 3);
        let top = stack_node_new(&mut arena, Some(left), Subtree::Null, false, 4);
        stack_node_add_link(&mut arena, top, null_link(right), &mut pool);
        assert_eq!(arena.node(top).link_count, 1);
        assert_eq!(arena.node(left).link_count, 2);
        assert_eq!(arena.node(right).ref_count, 1);
        assert_eq!(arena.node(q).ref_count, 2);
        assert_eq!(arena.node(left).node_count, 9);
        assert_eq!(arena.node(top).node_count, 2);
        assert_eq!(arena.node(top).dynamic_precedence, 7);
        stack_node_release(&mut arena, right, &mut pool);
        stack_node_release(&mut arena, top, &mut pool);
        assert!(arena.nodes.iter().all(Option::is_none));
    }

    #[test]
    fn slices_group_by_node_and_share_versions() {
        let mut stack = ts_stack_new();
        let mut pool = SubtreePool::default();
        let base = stack.base_node;
        let other = stack_node_new(&mut stack.arena, None, Subtree::Null, false, 2);
        ts_stack__add_slice(&mut stack, 0, base, &[leaf(1, 0)]);
        ts_stack__add_slice(&mut stack, 0, other, &[leaf(2, 0)]);
        ts_stack__add_slice(&mut stack, 0, base, &[leaf(3, 0)]);
        assert_eq!(ts_stack_version_count(&stack), 3);
        assert_eq!(
            stack.slices.iter().map(|s| s.version).collect::<Vec<_>>(),
            [1, 1, 2]
        );
        assert_eq!(
            stack
                .slices
                .iter()
                .map(|s| symbols(&s.subtrees))
                .collect::<Vec<_>>(),
            [vec![1], vec![3], vec![2]]
        );
        stack_node_release(&mut stack.arena, other, &mut pool);
        ts_stack_delete(&mut stack, &mut pool);
        assert!(stack.heads.is_empty());
        assert!(stack.arena.nodes.is_empty());
    }

    #[test]
    fn iteration_is_breadth_first_and_returns_owned_reversed_paths() {
        let mut stack = ts_stack_new();
        let mut pool = SubtreePool::default();
        let base = stack.base_node;
        let a = stack_node_new(&mut stack.arena, Some(base), leaf(1, VISIBLE), true, 2);
        stack_node_retain(&mut stack.arena, base);
        let b = stack_node_new(&mut stack.arena, Some(base), leaf(2, VISIBLE), true, 3);
        let top = stack_node_new(&mut stack.arena, Some(a), leaf(3, VISIBLE), true, 4);
        stack_node_add_link(
            &mut stack.arena,
            top,
            StackLink {
                node: b,
                subtree: leaf(4, VISIBLE),
                is_pending: false,
            },
            &mut pool,
        );
        stack_node_release(&mut stack.arena, b, &mut pool);
        stack.heads[0].node = top;
        let mut visited = Vec::new();
        let slices = stack__iter(
            &mut stack,
            &mut pool,
            0,
            &mut |arena, it| {
                visited.push((arena.node(it.node).state, it.subtree_count, it.is_pending));
                if it.subtree_count == 2 {
                    STACK_ACTION_POP | STACK_ACTION_STOP
                } else {
                    STACK_ACTION_NONE
                }
            },
            2,
        );
        assert_eq!(
            visited,
            [
                (4, 0, true),
                (2, 1, true),
                (3, 1, false),
                (1, 2, true),
                (1, 2, false)
            ]
        );
        assert_eq!(slices.len(), 2);
        assert_eq!(slices[0].version, slices[1].version);
        assert_eq!(symbols(&slices[0].subtrees), [1, 3]);
        assert_eq!(symbols(&slices[1].subtrees), [2, 4]);
        assert!(stack.slices.is_empty());
        assert!(stack.iterators.is_empty());
        let zero_pop = stack__iter(
            &mut stack,
            &mut pool,
            0,
            &mut |_, _| STACK_ACTION_POP | STACK_ACTION_STOP,
            0,
        );
        assert_eq!(zero_pop.len(), 1);
        assert!(zero_pop[0].subtrees.is_empty());
        assert_eq!(symbols(&slices[0].subtrees), [1, 3]);
    }

    #[test]
    fn extras_and_null_links_have_distinct_pending_and_count_behavior() {
        let mut stack = ts_stack_new();
        let mut pool = SubtreePool::default();
        let base = stack.base_node;
        let recovery = stack_node_new(&mut stack.arena, Some(base), Subtree::Null, true, 0);
        let regular = stack_node_new(&mut stack.arena, Some(recovery), leaf(1, VISIBLE), true, 2);
        let extra = stack_node_new(&mut stack.arena, Some(regular), leaf(2, EXTRA), false, 2);
        stack.heads[0].node = extra;
        for goal in [0, -1] {
            let mut visited = Vec::new();
            let slices = stack__iter(
                &mut stack,
                &mut pool,
                0,
                &mut |_, it| {
                    visited.push((it.subtree_count, it.is_pending));
                    STACK_ACTION_POP
                },
                goal,
            );
            assert_eq!(visited, [(0, true), (0, true), (1, true), (2, false)]);
            if goal < 0 {
                assert!(slices.iter().all(|s| s.subtrees.is_empty()));
            } else {
                assert_eq!(
                    slices
                        .iter()
                        .map(|s| symbols(&s.subtrees))
                        .collect::<Vec<_>>(),
                    [vec![], vec![2], vec![1, 2], vec![1, 2]]
                );
            }
        }
    }

    #[test]
    fn iterator_limit_keeps_first_links_after_sixty_four_paths() {
        fn branch(arena: &mut StackArena, pool: &mut SubtreePool, depth: usize) -> StackNodeId {
            let state = (arena.nodes.len() + 2) as StateId;
            let node = stack_node_new(arena, None, Subtree::Null, false, state);
            if depth > 0 {
                for _ in 0..MAX_LINK_COUNT {
                    let child = branch(arena, pool, depth - 1);
                    stack_node_add_link(arena, node, null_link(child), pool);
                    stack_node_release(arena, child, pool);
                }
            }
            node
        }
        let mut stack = ts_stack_new();
        let mut pool = SubtreePool::default();
        let top = branch(&mut stack.arena, &mut pool, 3);
        stack_node_release(&mut stack.arena, stack.base_node, &mut pool);
        stack.heads[0].node = top;
        let mut by_depth = [0; 4];
        let mut expected_leaves = Vec::new();
        let mut visited_leaves = Vec::new();
        let slices = stack__iter(
            &mut stack,
            &mut pool,
            0,
            &mut |arena, it| {
                by_depth[it.subtree_count as usize] += 1;
                if it.subtree_count == 2 {
                    expected_leaves.push(arena.node(it.node).links[0].as_ref().unwrap().node);
                }
                if it.subtree_count == 3 {
                    visited_leaves.push(it.node);
                    STACK_ACTION_POP | STACK_ACTION_STOP
                } else {
                    STACK_ACTION_NONE
                }
            },
            3,
        );
        assert_eq!(by_depth, [1, 8, 64, 64]);
        assert_eq!(visited_leaves, expected_leaves);
        assert_eq!(slices.len(), MAX_ITERATOR_COUNT);
        assert!(slices.iter().all(|s| s.subtrees.is_empty()));
        ts_stack_delete(&mut stack, &mut pool);
        assert!(stack.arena.nodes.is_empty());
    }
}
