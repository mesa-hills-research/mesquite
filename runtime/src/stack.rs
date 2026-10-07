use crate::{
    error_costs::{ERROR_COST_PER_RECOVERY, ERROR_STATE},
    language::{Language, ts_language_write_symbol_as_dot_string},
    length::Length,
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
#[derive(Debug)]
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
#[derive(Debug)]
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

pub(crate) fn stack_node_retain(arena: &mut StackArena, node: StackNodeId) {
    todo!("stack-1: stack_node_retain")
}

pub(crate) fn stack_node_release(
    arena: &mut StackArena,
    node: StackNodeId,
    subtree_pool: &mut SubtreePool,
) {
    todo!("stack-1: stack_node_release")
}

pub(crate) fn stack__subtree_node_count(subtree: &Subtree) -> u32 {
    todo!("stack-1: stack__subtree_node_count")
}

pub(crate) fn stack_node_new(
    arena: &mut StackArena,
    previous_node: Option<StackNodeId>,
    subtree: Subtree,
    is_pending: bool,
    state: StateId,
) -> StackNodeId {
    todo!("stack-1: stack_node_new")
}

pub(crate) fn stack__subtree_is_equivalent(left: &Subtree, right: &Subtree) -> bool {
    todo!("stack-1: stack__subtree_is_equivalent")
}

pub(crate) fn stack_node_add_link(
    arena: &mut StackArena,
    node: StackNodeId,
    link: StackLink,
    subtree_pool: &mut SubtreePool,
) {
    todo!("stack-1: stack_node_add_link")
}

pub(crate) fn stack_head_delete(
    head: StackHead,
    arena: &mut StackArena,
    subtree_pool: &mut SubtreePool,
) {
    todo!("stack-1: stack_head_delete")
}

pub(crate) fn ts_stack__add_version(
    stack: &mut Stack,
    original_version: StackVersion,
    node: StackNodeId,
) -> StackVersion {
    todo!("stack-1: ts_stack__add_version")
}

pub(crate) fn ts_stack__add_slice(
    stack: &mut Stack,
    original_version: StackVersion,
    node: StackNodeId,
    subtrees: &[Subtree],
) {
    todo!("stack-1: ts_stack__add_slice")
}

pub(crate) fn stack__iter(
    stack: &mut Stack,
    pool: &mut SubtreePool,
    version: StackVersion,
    callback: &mut dyn FnMut(&StackArena, &StackIterator) -> StackAction,
    goal_subtree_count: i32,
) -> Vec<StackSlice> {
    todo!("stack-1: stack__iter")
}

pub(crate) fn ts_stack_new() -> Stack {
    todo!("stack-1: ts_stack_new")
}

pub(crate) fn ts_stack_delete(stack: &mut Stack, pool: &mut SubtreePool) {
    todo!("stack-1: ts_stack_delete")
}

pub(crate) fn ts_stack_version_count(stack: &Stack) -> u32 {
    todo!("stack-1: ts_stack_version_count")
}

pub(crate) fn ts_stack_halted_version_count(stack: &Stack) -> u32 {
    todo!("stack-1: ts_stack_halted_version_count")
}

pub(crate) fn ts_stack_state(stack: &Stack, version: StackVersion) -> StateId {
    todo!("stack-1: ts_stack_state")
}

pub(crate) fn ts_stack_position(stack: &Stack, version: StackVersion) -> Length {
    todo!("stack-1: ts_stack_position")
}

pub(crate) fn ts_stack_last_external_token(stack: &Stack, version: StackVersion) -> &Subtree {
    todo!("stack-1: ts_stack_last_external_token")
}

pub(crate) fn ts_stack_set_last_external_token(
    stack: &mut Stack,
    pool: &mut SubtreePool,
    version: StackVersion,
    token: Subtree,
) {
    todo!("stack-1: ts_stack_set_last_external_token")
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
    let has_error_link = node.links[..node.link_count as usize].iter().any(|link| {
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
                    node = stack.arena.nodes[link.node.0].as_ref().unwrap();
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
        let link = StackLink {
            node: link.node,
            subtree: link.subtree.clone(),
            is_pending: link.is_pending,
        };
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
                    node_id.0, link.node.0
                )?;
                if link.is_pending {
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
                    stack.iterators[i - 1].node = link.node;
                } else {
                    stack.iterators.push(StackIterator {
                        node: link.node,
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
            links: std::array::from_fn(|_| None),
            link_count: 0,
            ref_count: 1,
            error_cost,
            node_count,
            dynamic_precedence: 0,
        }
    }

    fn link(node: &mut StackNode, predecessor: usize, subtree: Subtree) {
        node.links[node.link_count as usize] = Some(StackLink {
            node: StackNodeId(predecessor),
            subtree,
            is_pending: false,
        });
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
