use crate::{
    error_costs::{ERROR_COST_PER_RECOVERY, ERROR_STATE},
    language::{Language, ts_language_write_symbol_as_dot_string},
    length::{Length, length_add, length_zero},
    subtree::*,
    types::*,
};
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
    pub subtree: Subtree,
    node_and_pending: usize,
}

impl StackLink {

    #[inline]
    fn new(node: StackNodeId, subtree: Subtree, is_pending: bool) -> Self {
        // A Vec allocation cannot exceed isize::MAX bytes, and StackNode is
        // nonzero-sized. Every arena slot index therefore fits in usize::BITS-1
        // bits, leaving room to encode pending in the low bit after a shift.
        // This introduces no additional limit on a valid arena.
        assert!(node.0 <= isize::MAX as usize);
        Self {
            subtree,
            node_and_pending: (node.0 << 1) | usize::from(is_pending),
        }
    }

    #[inline]
    pub(crate) fn node(&self) -> StackNodeId {
        StackNodeId(self.node_and_pending >> 1)
    }

    #[inline]
    fn is_pending(&self) -> bool {
        self.node_and_pending & 1 != 0
    }
}
/// Almost all stack nodes have one predecessor. Keep that link in the arena
/// slot, and allocate the remaining fixed-capacity slots only when paths merge.
/// Link order and MAX_LINK_COUNT are unchanged from the C stack.
#[derive(Debug, Default)]
pub(crate) struct StackLinks {
    first: Option<StackLink>,
    rest: Option<Box<[Option<StackLink>; MAX_LINK_COUNT - 1]>>,
}

impl StackLinks {
    fn iter(&self) -> impl Iterator<Item = &Option<StackLink>> {
        std::iter::once(&self.first).chain(self.rest.iter().flat_map(|links| links.iter()))
    }

    fn iter_mut(&mut self) -> impl Iterator<Item = &mut Option<StackLink>> {
        std::iter::once(&mut self.first)
            .chain(self.rest.iter_mut().flat_map(|links| links.iter_mut()))
    }
}

impl std::ops::Index<usize> for StackLinks {
    type Output = Option<StackLink>;

    fn index(&self, index: usize) -> &Self::Output {
        if index == 0 {
            &self.first
        } else {
            &self.rest.as_ref().expect("allocated stack links")[index - 1]
        }
    }
}

impl std::ops::IndexMut<usize> for StackLinks {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index == 0 {
            &mut self.first
        } else {
            let rest = self
                .rest
                .get_or_insert_with(|| Box::new(std::array::from_fn(|_| None)));
            &mut rest[index - 1]
        }
    }
}

#[derive(Debug)]
pub(crate) struct StackNode {
    pub state: StateId,
    pub position: Length,
    pub links: StackLinks,
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

impl StackIterator {
    #[inline]
    fn advance(&mut self, link: &StackLink, include_subtrees: bool) {
        self.node = link.node();
        if !link.subtree.is_null() {
            if include_subtrees {
                self.subtrees.push(link.subtree.clone());
            }
            if !ts_subtree_extra(&link.subtree) {
                self.subtree_count = self.subtree_count.wrapping_add(1);
                if !link.is_pending() {
                    self.is_pending = false;
                }
            }
        } else {
            self.subtree_count = self.subtree_count.wrapping_add(1);
            self.is_pending = false;
        }
    }
}

// The public helper borrows its array, but iteration can transfer it without
// copying. Keep the slice/version ordering algorithm in this owning helper.
#[inline]
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
    // Preserve C's release order: links N-1 through 1, link 0's subtree,
    // this node, then link 0's predecessor (the tail call). Shared nodes and
    // unbranched paths need no worklist allocation at all.
    enum Release {
        Node(StackNodeId),
        Subtree(Subtree),
        Recycle(StackNodeId),
    }
    let mut pending = Vec::new();
    let mut action = Release::Node(node);
    loop {
        match action {
            Release::Subtree(subtree) => ts_subtree_release(subtree_pool, subtree),
            Release::Recycle(node) => arena.free.push(node),
            Release::Node(node) => {
                let data = arena.node_mut(node);
                assert_ne!(data.ref_count, 0);
                data.ref_count -= 1;
                if data.ref_count == 0 {
                    if data.link_count <= 1 {
                        let first = data.links[0].take();
                        arena.nodes[node.0] = None;
                        if let Some(first) = first {
                            let previous = first.node();
                            if !first.subtree.is_null() {
                                ts_subtree_release(subtree_pool, first.subtree);
                            }
                            arena.free.push(node);
                            action = Release::Node(previous);
                            continue;
                        }
                        arena.free.push(node);
                    } else {
                        let mut data = arena.nodes[node.0].take().expect("live stack node");
                        let first = data.links[0].as_ref().expect("initialized stack link");
                        pending.push(Release::Node(first.node()));
                        pending.push(Release::Recycle(node));
                        for (i, link) in data
                            .links
                            .iter_mut()
                            .take(data.link_count as usize)
                            .enumerate()
                        {
                            let link = link.take().expect("initialized stack link");
                            if i > 0 {
                                pending.push(Release::Node(link.node()));
                            }
                            if !link.subtree.is_null() {
                                pending.push(Release::Subtree(link.subtree));
                            }
                        }
                    }
                }
            }
        }
        let Some(next) = pending.pop() else {
            break;
        };
        action = next;
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

// Push sites always have a predecessor. Inlining specializes away the base
// node setup and avoids passing the owned subtree through another call layer.
#[inline]
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
        links: StackLinks::default(),
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
        node.links[0] = Some(StackLink::new(previous_node, subtree, is_pending));
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
    if link.node() == node {
        return;
    }
    for i in 0..arena.node(node).link_count as usize {
        let existing = arena.node(node).links[i]
            .as_ref()
            .expect("initialized stack link");
        if !stack__subtree_is_equivalent(&existing.subtree, &link.subtree) {
            continue;
        }
        let existing_node = existing.node();
        if existing_node == link.node() {
            // Remove ambiguity early only for equivalent links directly joining
            // the same pair of nodes. Keep the pending flag of the old link.
            if ts_subtree_dynamic_precedence(&link.subtree)
                > ts_subtree_dynamic_precedence(&existing.subtree)
            {
                let precedence = arena
                    .node(link.node())
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
        let other = arena.node(link.node());
        if previous.state == other.state
            && previous.position.bytes == other.position.bytes
            && previous.error_cost == other.error_cost
        {
            // Read each edge after the preceding recursive merge, just as C
            // does; merging can mutate nodes shared by both paths.
            let mut j = 0;
            while j < arena.node(link.node()).link_count as usize {
                let predecessor = arena.node(link.node()).links[j]
                    .as_ref()
                    .expect("initialized stack link")
                    .clone();
                stack_node_add_link(arena, existing_node, predecessor, subtree_pool);
                j += 1;
            }
            let mut precedence = arena.node(link.node()).dynamic_precedence;
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
    stack_node_retain(arena, link.node());
    let previous = arena.node(link.node());
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
    callback: &mut impl FnMut(&StackArena, &StackIterator) -> StackAction,
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
    // Most reductions walk a single path. Keep that iterator local until a
    // branch actually needs the breadth-first frontier; repeatedly indexing
    // the frontier for each predecessor otherwise dominates this short walk.
    let mut iterator = StackIterator {
        node: stack.heads[version as usize].node,
        subtrees,
        subtree_count: 0,
        is_pending: true,
    };
    loop {
        let node = stack.arena.node(iterator.node);
        if node.link_count > 1 {
            break;
        }
        let action = callback(&stack.arena, &iterator);
        let should_pop = action & STACK_ACTION_POP != 0;
        let should_stop = action & STACK_ACTION_STOP != 0 || node.link_count == 0;
        if should_pop {
            let mut subtrees = if should_stop {
                std::mem::take(&mut iterator.subtrees)
            } else {
                iterator.subtrees.clone()
            };
            subtrees.reverse();
            stack_add_slice_owned(stack, version, iterator.node, subtrees);
        }
        if should_stop {
            if !should_pop {
                ts_subtree_array_delete(pool, &mut iterator.subtrees);
            }
            return std::mem::take(&mut stack.slices);
        }
        let link = stack.arena.node(iterator.node).links[0]
            .as_ref()
            .expect("initialized stack link");
        iterator.advance(link, include_subtrees);
    }
    stack.iterators.push(iterator);
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
                next.advance(link, include_subtrees);
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
    let head = &stack.heads[version as usize];
    let node = stack.arena.nodes[head.node.0].as_ref().unwrap();
    let mut result = node.error_cost;
    if head.status == StackStatus::Paused
        || (node.state == ERROR_STATE && node.links[0].as_ref().unwrap().subtree.is_null())
    {
        result = result.wrapping_add(ERROR_COST_PER_RECOVERY);
    }
    result
}

pub(crate) fn ts_stack_node_count_since_error(stack: &mut Stack, version: StackVersion) -> u32 {
    // C's const Stack pointer still permits this mutation of its head array.
    let head = &mut stack.heads[version as usize];
    let node_count = stack.arena.nodes[head.node.0].as_ref().unwrap().node_count;
    if node_count < head.node_count_at_last_error {
        head.node_count_at_last_error = node_count;
    }
    node_count - head.node_count_at_last_error
}

#[inline]
pub(crate) fn ts_stack_push(
    stack: &mut Stack,
    _pool: &mut SubtreePool,
    version: StackVersion,
    subtree: Subtree,
    pending: bool,
    state: StateId,
) {
    let head = &mut stack.heads[version as usize];
    let is_error = subtree.is_null();
    // Transfer the head's node reference to the new node's predecessor link.
    let new_node = stack_node_new(&mut stack.arena, Some(head.node), subtree, pending, state);
    if is_error {
        head.node_count_at_last_error = stack.arena.nodes[new_node.0].as_ref().unwrap().node_count;
    }
    head.node = new_node;
}

pub(crate) fn pop_count_callback(count: u32, iterator: &StackIterator) -> StackAction {
    if iterator.subtree_count == count {
        STACK_ACTION_POP | STACK_ACTION_STOP
    } else {
        STACK_ACTION_NONE
    }
}

pub(crate) fn ts_stack_pop_count(
    stack: &mut Stack,
    pool: &mut SubtreePool,
    version: StackVersion,
    count: u32,
) -> Vec<StackSlice> {
    stack__iter(
        stack,
        pool,
        version,
        &mut |_, iterator| pop_count_callback(count, iterator),
        count as i32,
    )
}

/// Fuse a single-path pop with removal of its original version. The caller must
/// be committed to replacing that version (no alternative reduction/shift).
/// Refuse shared or branching prefixes without changing anything, so the full
/// graph traversal can handle them with its usual ordering and limits.
#[inline]
pub(crate) fn ts_stack_pop_count_in_place(
    stack: &mut Stack,
    count: u32,
) -> Option<Vec<Subtree>> {
    if stack.heads.len() != 1 || stack.heads[0].status != StackStatus::Active {
        return None;
    }
    let top = stack.heads[0].node;
    let mut node = top;
    let mut remaining = count;
    let mut subtree_count = 0;
    while remaining > 0 {
        let data = stack.arena.node(node);
        if data.ref_count != 1 || data.link_count != 1 {
            return None;
        }
        let link = data.links[0].as_ref().expect("initialized stack link");
        if link.subtree.is_null() {
            remaining -= 1;
        } else {
            subtree_count += 1;
            if !ts_subtree_extra(&link.subtree) {
                remaining -= 1;
            }
        }
        node = link.node();
    }
    let goal = node;
    let mut subtrees = Vec::with_capacity(subtree_count);
    node = top;
    while node != goal {
        let data = stack.arena.node_mut(node);
        let link = data.links[0].take().expect("initialized stack link");
        // Each removed node has exactly one owner. Transfer its link's node
        // ownership to the head, and its subtree ownership to the result,
        // instead of retaining and immediately releasing both after reduction.
        stack.arena.nodes[node.0] = None;
        stack.arena.free.push(node);
        let previous = link.node();
        if !link.subtree.is_null() {
            subtrees.push(link.subtree);
        }
        node = previous;
    }
    stack.heads[0].node = goal;
    subtrees.reverse();
    stack.slices.clear();
    stack.iterators.clear();
    Some(subtrees)
}

pub(crate) fn pop_pending_callback(iterator: &StackIterator) -> StackAction {
    if iterator.subtree_count >= 1 {
        if iterator.is_pending {
            STACK_ACTION_POP | STACK_ACTION_STOP
        } else {
            STACK_ACTION_STOP
        }
    } else {
        STACK_ACTION_NONE
    }
}

pub(crate) fn ts_stack_pop_pending(
    stack: &mut Stack,
    pool: &mut SubtreePool,
    version: StackVersion,
) -> Vec<StackSlice> {
    let mut pop = stack__iter(
        stack,
        pool,
        version,
        &mut |_, iterator| pop_pending_callback(iterator),
        0,
    );
    if let Some(first) = pop.first_mut() {
        ts_stack_renumber_version(stack, pool, first.version, version);
        first.version = version;
    }
    pop
}

pub(crate) fn pop_error_callback(found_error: &mut bool, iterator: &StackIterator) -> StackAction {
    if let Some(first) = iterator.subtrees.first() {
        if !*found_error && ts_subtree_is_error(first) {
            *found_error = true;
            STACK_ACTION_POP | STACK_ACTION_STOP
        } else {
            STACK_ACTION_STOP
        }
    } else {
        STACK_ACTION_NONE
    }
}

pub(crate) fn ts_stack_pop_error(
    stack: &mut Stack,
    pool: &mut SubtreePool,
    version: StackVersion,
) -> Vec<Subtree> {
    let node = stack.arena.nodes[stack.heads[version as usize].node.0]
        .as_ref()
        .unwrap();
    let has_error_link = node.links.iter().take(node.link_count as usize).any(|link| {
        let subtree = &link.as_ref().unwrap().subtree;
        !subtree.is_null() && ts_subtree_is_error(subtree)
    });
    if has_error_link {
        let mut found_error = false;
        let mut pop = stack__iter(
            stack,
            pool,
            version,
            &mut |_, iterator| pop_error_callback(&mut found_error, iterator),
            1,
        );
        if !pop.is_empty() {
            assert_eq!(pop.len(), 1);
            let slice = pop.pop().unwrap();
            ts_stack_renumber_version(stack, pool, slice.version, version);
            return slice.subtrees;
        }
    }
    Vec::new()
}

pub(crate) fn pop_all_callback(arena: &StackArena, iterator: &StackIterator) -> StackAction {
    if arena.nodes[iterator.node.0].as_ref().unwrap().link_count == 0 {
        STACK_ACTION_POP
    } else {
        STACK_ACTION_NONE
    }
}

pub(crate) fn ts_stack_pop_all(
    stack: &mut Stack,
    pool: &mut SubtreePool,
    version: StackVersion,
) -> Vec<StackSlice> {
    stack__iter(stack, pool, version, &mut pop_all_callback, 0)
}

pub(crate) fn summarize_stack_callback(
    session: &mut SummarizeStackSession<'_>,
    arena: &StackArena,
    iterator: &StackIterator,
) -> StackAction {
    let node = arena.nodes[iterator.node.0].as_ref().unwrap();
    let state = node.state;
    let depth = iterator.subtree_count;
    if depth > session.max_depth {
        return STACK_ACTION_STOP;
    }
    for entry in session.summary.iter().rev() {
        if entry.depth < depth {
            break;
        }
        if entry.depth == depth && entry.state == state {
            return STACK_ACTION_NONE;
        }
    }
    session.summary.push(StackSummaryEntry {
        position: node.position,
        depth,
        state,
    });
    STACK_ACTION_NONE
}

pub(crate) fn ts_stack_record_summary(
    stack: &mut Stack,
    pool: &mut SubtreePool,
    version: StackVersion,
    max_depth: u32,
) {
    let mut summary = Vec::new();
    let mut session = SummarizeStackSession {
        summary: &mut summary,
        max_depth,
    };
    stack__iter(
        stack,
        pool,
        version,
        &mut |arena, iterator| summarize_stack_callback(&mut session, arena, iterator),
        -1,
    );
    stack.heads[version as usize].summary = Some(summary);
}

pub(crate) fn ts_stack_get_summary(stack: &Stack, version: StackVersion) -> Option<&StackSummary> {
    stack.heads[version as usize].summary.as_ref()
}

pub(crate) fn ts_stack_dynamic_precedence(stack: &Stack, version: StackVersion) -> i32 {
    stack.arena.nodes[stack.heads[version as usize].node.0]
        .as_ref()
        .unwrap()
        .dynamic_precedence
}

pub(crate) fn ts_stack_has_advanced_since_error(stack: &Stack, version: StackVersion) -> bool {
    let head = &stack.heads[version as usize];
    let mut node = stack.arena.nodes[head.node.0].as_ref().unwrap();
    if node.error_cost == 0 {
        return true;
    }
    loop {
        if node.link_count > 0 {
            let link = node.links[0].as_ref().unwrap();
            if !link.subtree.is_null() {
                if ts_subtree_total_bytes(&link.subtree) > 0 {
                    return true;
                } else if node.node_count > head.node_count_at_last_error
                    && ts_subtree_error_cost(&link.subtree) == 0
                {
                    node = stack.arena.nodes[link.node().0].as_ref().unwrap();
                    continue;
                }
            }
        }
        break;
    }
    false
}

pub(crate) fn ts_stack_remove_version(
    stack: &mut Stack,
    pool: &mut SubtreePool,
    version: StackVersion,
) {
    let head = stack.heads.remove(version as usize);
    stack_head_delete(head, &mut stack.arena, pool);
}

pub(crate) fn ts_stack_renumber_version(
    stack: &mut Stack,
    pool: &mut SubtreePool,
    v1: StackVersion,
    v2: StackVersion,
) {
    if v1 == v2 {
        return;
    }
    assert!(v2 < v1);
    assert!((v1 as usize) < stack.heads.len());
    let mut source_head = stack.heads.remove(v1 as usize);
    let target_head = &mut stack.heads[v2 as usize];
    if target_head.summary.is_some() && source_head.summary.is_none() {
        source_head.summary = target_head.summary.take();
    }
    let old_head = std::mem::replace(target_head, source_head);
    stack_head_delete(old_head, &mut stack.arena, pool);
}

pub(crate) fn ts_stack_swap_versions(stack: &mut Stack, v1: StackVersion, v2: StackVersion) {
    stack.heads.swap(v1 as usize, v2 as usize);
}

pub(crate) fn ts_stack_copy_version(stack: &mut Stack, version: StackVersion) -> StackVersion {
    assert!((version as usize) < stack.heads.len());
    let head = &stack.heads[version as usize];
    let copy = StackHead {
        node: head.node,
        summary: None,
        node_count_at_last_error: head.node_count_at_last_error,
        last_external_token: head.last_external_token.clone(),
        lookahead_when_paused: head.lookahead_when_paused.clone(),
        status: head.status,
    };
    stack_node_retain(&mut stack.arena, copy.node);
    stack.heads.push(copy);
    (stack.heads.len() - 1) as StackVersion
}

pub(crate) fn ts_stack_merge(
    stack: &mut Stack,
    pool: &mut SubtreePool,
    version1: StackVersion,
    version2: StackVersion,
) -> bool {
    if !ts_stack_can_merge(stack, version1, version2) {
        return false;
    }
    let node1 = stack.heads[version1 as usize].node;
    let node2 = stack.heads[version2 as usize].node;
    let mut i = 0;
    while i < stack.arena.nodes[node2.0].as_ref().unwrap().link_count as usize {
        let link = stack.arena.nodes[node2.0].as_ref().unwrap().links[i]
            .as_ref()
            .unwrap();
        // StackLink owns its subtree handle, but copying its node ID does not
        // retain the node. stack_node_add_link retains graph edges as needed.
        let link = StackLink::new(link.node(), link.subtree.clone(), link.is_pending());
        stack_node_add_link(&mut stack.arena, node1, link, pool);
        i += 1;
    }
    let node = stack.arena.nodes[node1.0].as_ref().unwrap();
    if node.state == ERROR_STATE {
        stack.heads[version1 as usize].node_count_at_last_error = node.node_count;
    }
    ts_stack_remove_version(stack, pool, version2);
    true
}

pub(crate) fn ts_stack_can_merge(
    stack: &Stack,
    version1: StackVersion,
    version2: StackVersion,
) -> bool {
    let head1 = &stack.heads[version1 as usize];
    let head2 = &stack.heads[version2 as usize];
    let node1 = stack.arena.nodes[head1.node.0].as_ref().unwrap();
    let node2 = stack.arena.nodes[head2.node.0].as_ref().unwrap();
    head1.status == StackStatus::Active
        && head2.status == StackStatus::Active
        && node1.state == node2.state
        && node1.position.bytes == node2.position.bytes
        && node1.error_cost == node2.error_cost
        && ts_subtree_external_scanner_state_eq(
            &head1.last_external_token,
            &head2.last_external_token,
        )
}

pub(crate) fn ts_stack_halt(stack: &mut Stack, version: StackVersion) {
    stack.heads[version as usize].status = StackStatus::Halted;
}

pub(crate) fn ts_stack_pause(
    stack: &mut Stack,
    _pool: &mut SubtreePool,
    version: StackVersion,
    lookahead: Subtree,
) {
    let head = &mut stack.heads[version as usize];
    head.status = StackStatus::Paused;
    head.lookahead_when_paused = lookahead;
    head.node_count_at_last_error = stack.arena.nodes[head.node.0].as_ref().unwrap().node_count;
}

pub(crate) fn ts_stack_is_active(stack: &Stack, version: StackVersion) -> bool {
    stack.heads[version as usize].status == StackStatus::Active
}

pub(crate) fn ts_stack_is_halted(stack: &Stack, version: StackVersion) -> bool {
    stack.heads[version as usize].status == StackStatus::Halted
}

pub(crate) fn ts_stack_is_paused(stack: &Stack, version: StackVersion) -> bool {
    stack.heads[version as usize].status == StackStatus::Paused
}

pub(crate) fn ts_stack_resume(stack: &mut Stack, version: StackVersion) -> Subtree {
    let head = &mut stack.heads[version as usize];
    assert_eq!(head.status, StackStatus::Paused);
    head.status = StackStatus::Active;
    std::mem::take(&mut head.lookahead_when_paused)
}

pub(crate) fn ts_stack_clear(stack: &mut Stack, pool: &mut SubtreePool) {
    stack_node_retain(&mut stack.arena, stack.base_node);
    for head in stack.heads.drain(..) {
        stack_head_delete(head, &mut stack.arena, pool);
    }
    stack.heads.push(StackHead {
        node: stack.base_node,
        summary: None,
        node_count_at_last_error: 0,
        last_external_token: Subtree::Null,
        lookahead_when_paused: Subtree::Null,
        status: StackStatus::Active,
    });
}

pub(crate) fn ts_stack_print_dot_graph(
    stack: &mut Stack,
    language: &Language,
    output: &mut dyn std::io::Write,
) -> std::io::Result<()> {
    stack.iterators.clear();
    stack.iterators.reserve(32);
    writeln!(output, "digraph stack {{")?;
    writeln!(output, "rankdir=\"RL\";")?;
    writeln!(output, "edge [arrowhead=none]")?;
    let mut visited_nodes = Vec::new();

    // Arena slot IDs replace C pointer addresses in DOT node identifiers.
    for i in 0..stack.heads.len() {
        if stack.heads[i].status == StackStatus::Halted {
            continue;
        }
        let node_count = ts_stack_node_count_since_error(stack, i as StackVersion);
        let error_cost = ts_stack_error_cost(stack, i as StackVersion);
        let head = &stack.heads[i];
        writeln!(output, "node_head_{i} [shape=none, label=\"\"]")?;
        write!(output, "node_head_{i} -> node_0x{:x} [", head.node.0)?;
        if head.status == StackStatus::Paused {
            write!(output, "color=red ")?;
        }
        write!(
            output,
            "label={i}, fontcolor=blue, weight=10000, labeltooltip=\"node_count: {node_count}\nerror_cost: {error_cost}"
        )?;
        if let Some(summary) = &head.summary {
            write!(output, "\nsummary:")?;
            for entry in summary {
                write!(output, " {}", entry.state)?;
            }
        }
        if !head.last_external_token.is_null() {
            let state = ts_subtree_external_scanner_state(&head.last_external_token)
                .expect("last external token must have scanner state");
            write!(output, "\nexternal_scanner_state:")?;
            for &byte in ts_external_scanner_state_data(state) {
                // C promotes each signed char to int before formatting as %X.
                write!(output, " {:2X}", byte as i8 as i32 as u32)?;
            }
        }
        writeln!(output, "\"]")?;
        stack.iterators.push(StackIterator {
            node: head.node,
            subtrees: Vec::new(),
            subtree_count: 0,
            is_pending: false,
        });
    }

    let mut all_iterators_done = false;
    while !all_iterators_done {
        all_iterators_done = true;
        let mut i = 0;
        // Like C, process newly appended branch iterators in this same pass.
        while i < stack.iterators.len() {
            let node_id = stack.iterators[i].node;
            i += 1;
            if visited_nodes.contains(&node_id) {
                continue;
            }
            all_iterators_done = false;
            let node = stack.arena.nodes[node_id.0].as_ref().unwrap();
            write!(output, "node_0x{:x} [", node_id.0)?;
            if node.state == ERROR_STATE {
                write!(output, "label=\"?\"")?;
            } else if node.link_count == 1
                && !node.links[0].as_ref().unwrap().subtree.is_null()
                && ts_subtree_extra(&node.links[0].as_ref().unwrap().subtree)
            {
                write!(output, "shape=point margin=0 label=\"\"")?;
            } else {
                write!(output, "label=\"{}\"", node.state)?;
            }
            writeln!(
                output,
                " tooltip=\"position: {},{}\nnode_count:{}\nerror_cost: {}\ndynamic_precedence: {}\"];",
                node.position.extent.row.wrapping_add(1),
                node.position.extent.column,
                node.node_count,
                node.error_cost,
                node.dynamic_precedence,
            )?;
            for j in 0..node.link_count as usize {
                let link = node.links[j].as_ref().unwrap();
                write!(
                    output,
                    "node_0x{:x} -> node_0x{:x} [",
                    node_id.0, link.node().0
                )?;
                if link.is_pending() {
                    write!(output, "style=dashed ")?;
                }
                if !link.subtree.is_null() && ts_subtree_extra(&link.subtree) {
                    write!(output, "fontcolor=gray ")?;
                }
                if link.subtree.is_null() {
                    write!(output, "color=red")?;
                } else {
                    write!(output, "label=\"")?;
                    let quoted =
                        ts_subtree_visible(&link.subtree) && !ts_subtree_named(&link.subtree);
                    if quoted {
                        write!(output, "'")?;
                    }
                    ts_language_write_symbol_as_dot_string(
                        language,
                        output,
                        ts_subtree_symbol(&link.subtree),
                    )?;
                    if quoted {
                        write!(output, "'")?;
                    }
                    write!(
                        output,
                        "\"labeltooltip=\"error_cost: {}\ndynamic_precedence: {}\"",
                        ts_subtree_error_cost(&link.subtree),
                        ts_subtree_dynamic_precedence(&link.subtree),
                    )?;
                }
                writeln!(output, "];")?;
                if j == 0 {
                    stack.iterators[i - 1].node = link.node();
                } else {
                    stack.iterators.push(StackIterator {
                        node: link.node(),
                        subtrees: Vec::new(),
                        subtree_count: 0,
                        is_pending: false,
                    });
                }
            }
            visited_nodes.push(node_id);
        }
    }
    writeln!(output, "}}")
}

#[cfg(test)]
mod stack2_tests {
    use super::*;
    use std::sync::Arc;

    fn leaf(size: u8) -> Subtree {
        Subtree::Inline(InlineLeaf {
            size_bytes: size,
            flags: VISIBLE | NAMED,
            symbol: 2,
            ..InlineLeaf::default()
        })
    }

    fn node(state: StateId, node_count: u32, error_cost: u32) -> StackNode {
        StackNode {
            state,
            position: Length::default(),
            links: StackLinks::default(),
            link_count: 0,
            ref_count: 1,
            error_cost,
            node_count,
            dynamic_precedence: 0,
        }
    }

    fn link(node: &mut StackNode, predecessor: usize, subtree: Subtree) {
        node.links[node.link_count as usize] = Some(StackLink::new(StackNodeId(predecessor), subtree, false));
        node.link_count += 1;
    }

    fn stack() -> Stack {
        let base = node(1, 0, 0);
        let mut error = node(ERROR_STATE, 4, 100);
        link(&mut error, 0, Subtree::Null);
        let mut top = node(2, 5, 100);
        link(&mut top, 1, leaf(0));
        Stack {
            heads: vec![StackHead {
                node: StackNodeId(2),
                summary: None,
                node_count_at_last_error: 4,
                last_external_token: Subtree::Null,
                lookahead_when_paused: Subtree::Null,
                status: StackStatus::Active,
            }],
            slices: Vec::new(),
            iterators: Vec::new(),
            arena: StackArena {
                nodes: vec![Some(base), Some(error), Some(top)],
                free: Vec::new(),
            },
            base_node: StackNodeId(0),
        }
    }

    fn iterator(count: u32, pending: bool) -> StackIterator {
        StackIterator {
            node: StackNodeId(2),
            subtrees: Vec::new(),
            subtree_count: count,
            is_pending: pending,
        }
    }

    #[test]
    fn owned_pop_transfers_children_and_the_predecessor_reference() {
        let mut pool = SubtreePool::default();
        let mut stack = ts_stack_new();
        let base = stack.base_node;
        let heap = Arc::new(SubtreeHeapData::default());
        ts_stack_push(&mut stack, &mut pool, 0, Subtree::Heap(heap.clone()), false, 2);
        let old_head = stack.heads[0].node;
        assert_eq!(Arc::strong_count(&heap), 2);
        assert_eq!(stack.arena.node(base).ref_count, 2);
        let children = ts_stack_pop_count_in_place(&mut stack, 1).unwrap();
        assert_eq!(children.len(), 1);
        assert_eq!(Arc::strong_count(&heap), 2, "the child handle was moved");
        assert_eq!(stack.heads[0].node, base);
        assert_eq!(stack.arena.node(base).ref_count, 2);
        assert!(stack.arena.nodes[old_head.0].is_none());
        assert_eq!(stack.arena.free.last(), Some(&old_head));
        drop(children);
        assert_eq!(Arc::strong_count(&heap), 1);
        ts_stack_delete(&mut stack, &mut pool);
    }

    #[test]
    fn owned_pop_preserves_extra_and_null_counting_and_child_order() {
        let mut pool = SubtreePool::default();
        let mut stack = ts_stack_new();
        ts_stack_push(&mut stack, &mut pool, 0, leaf(1), false, 2);
        let mut extra = leaf(2);
        ts_subtree_set_extra(&mut extra, true);
        ts_stack_push(&mut stack, &mut pool, 0, extra, false, 2);
        let extra_head = stack.heads[0].node;
        ts_stack_push(&mut stack, &mut pool, 0, Subtree::Null, false, 0);
        ts_stack_push(&mut stack, &mut pool, 0, leaf(3), true, 3);
        let top = ts_stack_pop_count_in_place(&mut stack, 2).unwrap();
        assert_eq!(top.len(), 1, "null counts but is not a child");
        assert_eq!(ts_subtree_size(&top[0]).bytes, 3);
        assert_eq!(stack.heads[0].node, extra_head);
        let bottom = ts_stack_pop_count_in_place(&mut stack, 1).unwrap();
        assert_eq!(bottom.len(), 2, "extras are children but do not count");
        assert_eq!(ts_subtree_size(&bottom[0]).bytes, 1);
        assert_eq!(ts_subtree_size(&bottom[1]).bytes, 2);
        assert!(ts_subtree_extra(&bottom[1]));
        assert_eq!(stack.heads[0].node, stack.base_node);
        ts_stack_delete(&mut stack, &mut pool);
    }

    #[test]
    fn owned_pop_preflight_does_not_change_a_shared_or_short_path() {
        let mut pool = SubtreePool::default();
        let mut stack = ts_stack_new();
        ts_stack_push(&mut stack, &mut pool, 0, leaf(1), false, 2);
        let shared = stack.heads[0].node;
        let copy = ts_stack_copy_version(&mut stack, 0);
        ts_stack_push(&mut stack, &mut pool, 0, leaf(2), false, 3);
        let original = stack.heads[0].node;
        assert!(ts_stack_pop_count_in_place(&mut stack, 2).is_none());
        assert_eq!(stack.heads[0].node, original);
        assert_eq!(stack.arena.node(shared).ref_count, 2);
        assert!(stack.arena.free.is_empty());
        ts_stack_remove_version(&mut stack, &mut pool, copy);
        assert!(ts_stack_pop_count_in_place(&mut stack, 3).is_none());
        assert_eq!(stack.heads[0].node, original);
        assert!(stack.arena.free.is_empty());
        assert_eq!(ts_stack_pop_count_in_place(&mut stack, 2).unwrap().len(), 2);
        ts_stack_delete(&mut stack, &mut pool);
    }

    #[test]
    fn owned_pop_leaves_branching_paths_to_the_general_walker() {
        let mut pool = SubtreePool::default();
        let mut stack = ts_stack_new();
        ts_stack_push(&mut stack, &mut pool, 0, leaf(1), false, 2);
        let head = stack.heads[0].node;
        let base = stack.base_node;
        stack_node_retain(&mut stack.arena, base);
        let data = stack.arena.node_mut(head);
        data.links[1] = Some(StackLink::new(base, leaf(2), false));
        data.link_count = 2;
        assert!(ts_stack_pop_count_in_place(&mut stack, 1).is_none());
        assert_eq!(stack.heads[0].node, head);
        assert_eq!(stack.arena.node(head).link_count, 2);
        assert!(stack.arena.free.is_empty());
        // Empty reductions do not remove any prefix, even at a branch.
        assert!(ts_stack_pop_count_in_place(&mut stack, 0).unwrap().is_empty());
        assert_eq!(stack.heads[0].node, head);
        ts_stack_delete(&mut stack, &mut pool);
    }

    #[test]
    fn error_cost_applies_exactly_one_recovery_penalty() {
        let mut stack = stack();
        assert_eq!(ts_stack_error_cost(&stack, 0), 100);
        stack.heads[0].node = StackNodeId(1);
        assert_eq!(ts_stack_error_cost(&stack, 0), 600);
        stack.heads[0].status = StackStatus::Paused;
        assert_eq!(ts_stack_error_cost(&stack, 0), 600);
        stack.arena.nodes[1].as_mut().unwrap().error_cost = u32::MAX;
        assert_eq!(ts_stack_error_cost(&stack, 0), 499);
        stack.heads[0].status = StackStatus::Active;
        stack.arena.nodes[1].as_mut().unwrap().links[0]
            .as_mut()
            .unwrap()
            .subtree = leaf(0);
        assert_eq!(ts_stack_error_cost(&stack, 0), u32::MAX);
    }

    #[test]
    fn node_count_query_moves_the_error_baseline_backward() {
        let mut stack = stack();
        assert_eq!(ts_stack_node_count_since_error(&mut stack, 0), 1);
        stack.heads[0].node_count_at_last_error = 10;
        assert_eq!(ts_stack_node_count_since_error(&mut stack, 0), 0);
        assert_eq!(stack.heads[0].node_count_at_last_error, 5);
        stack.arena.nodes[2].as_mut().unwrap().node_count = 7;
        assert_eq!(ts_stack_node_count_since_error(&mut stack, 0), 2);
    }

    #[test]
    fn count_and_pending_callbacks_use_non_extra_count() {
        assert_eq!(
            pop_count_callback(0, &iterator(0, true)),
            STACK_ACTION_POP | STACK_ACTION_STOP
        );
        assert_eq!(pop_count_callback(1, &iterator(0, true)), STACK_ACTION_NONE);
        assert_eq!(pop_count_callback(1, &iterator(2, true)), STACK_ACTION_NONE);
        assert_eq!(pop_pending_callback(&iterator(0, false)), STACK_ACTION_NONE);
        assert_eq!(pop_pending_callback(&iterator(1, false)), STACK_ACTION_STOP);
        assert_eq!(
            pop_pending_callback(&iterator(2, true)),
            STACK_ACTION_POP | STACK_ACTION_STOP
        );
        let mut iter = iterator(0, true);
        iter.subtrees.push(leaf(1));
        assert_eq!(pop_pending_callback(&iter), STACK_ACTION_NONE);
    }

    #[test]
    fn error_callback_pops_only_the_first_error_path() {
        let error = Subtree::Heap(Arc::new(SubtreeHeapData {
            symbol: ts_port_tables::BUILTIN_SYM_ERROR,
            children: Vec::new(),
            payload: SubtreePayload::Leaf,
            ..SubtreeHeapData::default()
        }));
        let mut iter = iterator(0, true);
        let mut found_error = false;
        assert_eq!(
            pop_error_callback(&mut found_error, &iter),
            STACK_ACTION_NONE
        );
        iter.subtrees = vec![leaf(0), error.clone()];
        assert_eq!(
            pop_error_callback(&mut found_error, &iter),
            STACK_ACTION_STOP
        );
        assert!(!found_error);
        iter.subtrees = vec![error];
        assert_eq!(
            pop_error_callback(&mut found_error, &iter),
            STACK_ACTION_POP | STACK_ACTION_STOP
        );
        assert!(found_error);
        assert_eq!(
            pop_error_callback(&mut found_error, &iter),
            STACK_ACTION_STOP
        );
    }

    #[test]
    fn summary_deduplicates_state_and_depth_but_not_distinct_depths() {
        let mut stack = stack();
        let mut summary = Vec::new();
        let mut session = SummarizeStackSession {
            summary: &mut summary,
            max_depth: 2,
        };
        let mut iter = iterator(0, true);
        assert_eq!(
            summarize_stack_callback(&mut session, &stack.arena, &iter),
            STACK_ACTION_NONE
        );
        stack.arena.nodes[2].as_mut().unwrap().position.bytes = 42;
        summarize_stack_callback(&mut session, &stack.arena, &iter);
        assert_eq!(session.summary.len(), 1);
        assert_eq!(session.summary[0].position.bytes, 0);
        iter.subtree_count = 1;
        summarize_stack_callback(&mut session, &stack.arena, &iter);
        assert_eq!(session.summary.len(), 2);
        assert_eq!(session.summary[1].position.bytes, 42);
        iter.node = StackNodeId(1);
        summarize_stack_callback(&mut session, &stack.arena, &iter);
        assert_eq!(session.summary.len(), 3);
        iter.subtree_count = 3;
        assert_eq!(
            summarize_stack_callback(&mut session, &stack.arena, &iter),
            STACK_ACTION_STOP
        );
        assert_eq!(session.summary.len(), 3);
        assert_eq!(pop_all_callback(&stack.arena, &iter), STACK_ACTION_NONE);
        iter.node = StackNodeId(0);
        assert_eq!(pop_all_callback(&stack.arena, &iter), STACK_ACTION_POP);
    }

    #[test]
    fn progress_walks_only_zero_width_error_free_first_links() {
        let mut stack = stack();
        assert!(!ts_stack_has_advanced_since_error(&stack, 0));
        stack.arena.nodes[1].as_mut().unwrap().links[0]
            .as_mut()
            .unwrap()
            .subtree = leaf(1);
        assert!(ts_stack_has_advanced_since_error(&stack, 0));
        stack.heads[0].node_count_at_last_error = 5;
        assert!(!ts_stack_has_advanced_since_error(&stack, 0));
        stack.heads[0].node_count_at_last_error = 4;
        let top = stack.arena.nodes[2].as_mut().unwrap();
        top.links[0].as_mut().unwrap().subtree = Subtree::Inline(InlineLeaf {
            flags: MISSING,
            ..InlineLeaf::default()
        });
        link(top, 0, leaf(1));
        assert!(!ts_stack_has_advanced_since_error(&stack, 0));
        stack.arena.nodes[2].as_mut().unwrap().error_cost = 0;
        assert!(ts_stack_has_advanced_since_error(&stack, 0));
    }

    #[test]
    fn pause_resume_transfers_lookahead_and_updates_baseline() {
        let mut stack = stack();
        let mut pool = SubtreePool::default();
        let lookahead = Subtree::Heap(Arc::new(SubtreeHeapData::default()));
        assert!(ts_stack_is_active(&stack, 0));
        ts_stack_pause(&mut stack, &mut pool, 0, lookahead.clone());
        assert!(ts_stack_is_paused(&stack, 0));
        assert!(!ts_stack_is_active(&stack, 0));
        assert_eq!(stack.heads[0].node_count_at_last_error, 5);
        let result = ts_stack_resume(&mut stack, 0);
        assert!(result.ptr_eq(&lookahead));
        assert!(stack.heads[0].lookahead_when_paused.is_null());
        assert!(ts_stack_is_active(&stack, 0));
        ts_stack_halt(&mut stack, 0);
        assert!(ts_stack_is_halted(&stack, 0));
    }
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
        StackLink::new(node, Subtree::Null, false)
    }

    #[test]
    fn links_allocate_only_for_multiple_predecessors() {
        let mut links = StackLinks::default();
        assert!(links.first.is_none());
        assert!(links.rest.is_none());
        links[0] = Some(null_link(StackNodeId(0)));
        assert!(links.rest.is_none());

        for i in 1..MAX_LINK_COUNT {
            links[i] = Some(null_link(StackNodeId(i)));
        }
        assert!(links.rest.is_some());
        assert_eq!(links.iter().count(), MAX_LINK_COUNT);
        for (i, link) in links.iter().enumerate() {
            assert_eq!(link.as_ref().unwrap().node(), StackNodeId(i));
            assert_eq!(links[i].as_ref().unwrap().node(), StackNodeId(i));
        }
        for link in links.iter_mut() {
            link.take();
        }
        assert!(links.iter().all(Option::is_none));

        // The common arena slot must not silently grow back to eight links.
        assert!(std::mem::size_of::<StackNode>() < 128);
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
    fn branched_release_pools_subtrees_in_c_order() {
        let heap = |symbol| {
            let data = Arc::new(SubtreeHeapData {
                symbol,
                children: Vec::new(),
                payload: SubtreePayload::Leaf,
                ..SubtreeHeapData::default()
            });
            (Arc::as_ptr(&data), Subtree::Heap(data))
        };
        let mut arena = StackArena::default();
        let mut pool = ts_subtree_pool_new(32);
        let base = stack_node_new(&mut arena, None, Subtree::Null, false, 1);
        let (left_ptr, left_tree) = heap(1);
        let left = stack_node_new(&mut arena, Some(base), left_tree, false, 2);
        stack_node_retain(&mut arena, base);
        let (right_ptr, right_tree) = heap(2);
        let right = stack_node_new(&mut arena, Some(base), right_tree, false, 3);
        let (first_ptr, first_tree) = heap(3);
        let top = stack_node_new(&mut arena, Some(left), first_tree, false, 4);
        let (last_ptr, last_tree) = heap(4);
        stack_node_add_link(
            &mut arena,
            top,
            StackLink::new(right, last_tree, false),
            &mut pool,
        );
        stack_node_release(&mut arena, right, &mut pool);
        assert!(pool.free_trees.is_empty());
        stack_node_release(&mut arena, top, &mut pool);
        assert_eq!(arena.free, [right, top, left, base]);
        assert_eq!(
            pool.free_trees.iter().map(Arc::as_ptr).collect::<Vec<_>>(),
            [last_ptr, right_ptr, first_ptr, left_ptr],
        );
        assert!(pool.free_trees.iter().all(|tree| Arc::strong_count(tree) == 1));
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
        duplicate = StackLink::new(duplicate.node(), duplicate.subtree, true);
        stack_node_add_link(&mut arena, top, duplicate, &mut pool);
        assert!(!arena.node(top).links[0].as_ref().unwrap().is_pending());
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
            StackLink::new(b, leaf(4, VISIBLE), false),
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
    fn linear_prefix_preserves_iterator_state_when_entering_a_branch() {
        let mut stack = ts_stack_new();
        let mut pool = SubtreePool::default();
        let base = stack.base_node;
        let a = stack_node_new(&mut stack.arena, Some(base), leaf(1, VISIBLE), true, 2);
        stack_node_retain(&mut stack.arena, base);
        let b = stack_node_new(&mut stack.arena, Some(base), leaf(2, VISIBLE), true, 3);
        let fork = stack_node_new(&mut stack.arena, Some(a), leaf(3, VISIBLE), true, 4);
        stack_node_add_link(
            &mut stack.arena,
            fork,
            StackLink::new(b, leaf(4, VISIBLE), false),
            &mut pool,
        );
        stack_node_release(&mut stack.arena, b, &mut pool);
        let prefix = stack_node_new(&mut stack.arena, Some(fork), leaf(5, VISIBLE), true, 5);
        let extra = stack_node_new(&mut stack.arena, Some(prefix), leaf(6, EXTRA), false, 6);
        stack.heads[0].node = extra;

        let mut visited = Vec::new();
        let slices = stack__iter(
            &mut stack,
            &mut pool,
            0,
            &mut |arena, it| {
                visited.push((arena.node(it.node).state, it.subtree_count, it.is_pending));
                pop_count_callback(3, it)
            },
            3,
        );
        assert_eq!(
            visited,
            [
                (6, 0, true),
                (5, 0, true),
                (4, 1, true),
                (2, 2, true),
                (3, 2, false),
                (1, 3, true),
                (1, 3, false),
            ]
        );
        assert_eq!(slices.len(), 2);
        assert_eq!(slices[0].version, slices[1].version);
        assert_eq!(symbols(&slices[0].subtrees), [1, 3, 5, 6]);
        assert_eq!(symbols(&slices[1].subtrees), [2, 4, 5, 6]);
        assert!(stack.slices.is_empty());
        assert!(stack.iterators.is_empty());
        ts_stack_delete(&mut stack, &mut pool);
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
    fn in_place_pop_matches_pop_then_renumber() {
        let heap = Subtree::Heap(Arc::new(SubtreeHeapData {
            symbol: 3,
            visible: true,
            children: Vec::new(),
            payload: SubtreePayload::Leaf,
            size: Length {
                bytes: 3,
                extent: Point { row: 0, column: 3 },
            },
            ..SubtreeHeapData::default()
        }));
        for count in 0..=4 {
            let build = || {
                let mut stack = ts_stack_new();
                let mut pool = SubtreePool::default();
                for (i, tree) in [
                    leaf(7, EXTRA),
                    leaf(1, VISIBLE),
                    Subtree::Null,
                    leaf(2, EXTRA),
                    heap.clone(),
                    leaf(4, EXTRA),
                ]
                .into_iter()
                .enumerate()
                {
                    ts_stack_push(
                        &mut stack,
                        &mut pool,
                        0,
                        tree,
                        i % 2 == 0,
                        (i + 2) as StateId,
                    );
                }
                stack.heads[0].summary = Some(vec![StackSummaryEntry {
                    position: length_zero(),
                    depth: 0,
                    state: 5,
                }]);
                stack.heads[0].node_count_at_last_error = 17;
                ts_stack_set_last_external_token(&mut stack, &mut pool, 0, heap.clone());
                stack
            };
            let mut ordinary = build();
            let mut optimized = build();
            let mut pool = SubtreePool::default();
            let original_top = optimized.heads[0].node;
            let original_refs = Arc::strong_count(match &heap {
                Subtree::Heap(data) => data,
                _ => unreachable!(),
            });
            let fast = ts_stack_pop_count_in_place(&mut optimized, count);
            // Moving the handles preserves the subtree reference counts.
            assert_eq!(
                Arc::strong_count(match &heap {
                    Subtree::Heap(data) => data,
                    _ => unreachable!(),
                }),
                original_refs
            );
            let mut slow = ts_stack_pop_count(&mut ordinary, &mut pool, 0, count);
            if count == 4 {
                assert!(fast.is_none());
                assert!(slow.is_empty());
                assert_eq!(optimized.heads[0].node, original_top);
                assert!(optimized.arena.free.is_empty());
            } else {
                let fast = fast.unwrap();
                let slow = slow.pop().unwrap();
                ts_stack_renumber_version(&mut ordinary, &mut pool, slow.version, 0);
                assert_eq!(symbols(&fast), symbols(&slow.subtrees));
                for (fast, slow) in fast.iter().zip(&slow.subtrees) {
                    assert!(fast.ptr_eq(slow));
                }
                assert_eq!(ts_stack_state(&optimized, 0), ts_stack_state(&ordinary, 0));
                assert_eq!(
                    ts_stack_position(&optimized, 0),
                    ts_stack_position(&ordinary, 0)
                );
                assert_eq!(optimized.heads[0].node_count_at_last_error, 17);
                assert_eq!(optimized.heads[0].summary.as_ref().unwrap()[0].state, 5);
                assert!(ts_stack_last_external_token(&optimized, 0).ptr_eq(&heap));
                assert_eq!(optimized.arena.free, ordinary.arena.free);
                assert_eq!(
                    optimized.arena.node(optimized.heads[0].node).ref_count,
                    ordinary.arena.node(ordinary.heads[0].node).ref_count
                );
            }
            ts_stack_delete(&mut optimized, &mut pool);
            ts_stack_delete(&mut ordinary, &mut pool);
        }
    }

    #[test]
    fn in_place_pop_refuses_shared_or_branching_prefix_without_mutation() {
        let mut stack = ts_stack_new();
        let mut pool = SubtreePool::default();
        ts_stack_push(&mut stack, &mut pool, 0, leaf(1, VISIBLE), false, 2);
        let shared = stack.heads[0].node;
        let copy = ts_stack_copy_version(&mut stack, 0);
        assert!(ts_stack_pop_count_in_place(&mut stack, 1).is_none());
        ts_stack_remove_version(&mut stack, &mut pool, copy);
        ts_stack_push(&mut stack, &mut pool, 0, leaf(2, VISIBLE), false, 3);
        let top = stack.heads[0].node;
        stack_node_retain(&mut stack.arena, shared);
        assert!(ts_stack_pop_count_in_place(&mut stack, 2).is_none());
        assert_eq!(stack.heads[0].node, top);
        assert!(
            stack.arena.node(top).links[0]
                .as_ref()
                .unwrap()
                .subtree
                .ptr_eq(&leaf(2, VISIBLE))
        );
        assert!(stack.arena.free.is_empty());
        stack_node_release(&mut stack.arena, shared, &mut pool);
        let base = stack.base_node;
        stack_node_add_link(
            &mut stack.arena,
            shared,
            StackLink::new(base, leaf(3, VISIBLE), true),
            &mut pool,
        );
        assert_eq!(stack.arena.node(shared).link_count, 2);
        assert!(ts_stack_pop_count_in_place(&mut stack, 2).is_none());
        assert_eq!(stack.heads[0].node, top);
        assert!(stack.arena.free.is_empty());
        // A branch at the goal, not in the removed prefix, is fine.
        let pop = ts_stack_pop_count_in_place(&mut stack, 1).unwrap();
        assert_eq!(symbols(&pop), [2]);
        assert_eq!(stack.heads[0].node, shared);
        assert_eq!(stack.arena.node(shared).ref_count, 1);
        assert_eq!(stack.arena.node(shared).link_count, 2);
        ts_stack_delete(&mut stack, &mut pool);
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
                    expected_leaves.push(arena.node(it.node).links[0].as_ref().unwrap().node());
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
