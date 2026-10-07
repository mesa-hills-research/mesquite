use crate::{language::Language, length::Length, subtree::*, types::*};
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
