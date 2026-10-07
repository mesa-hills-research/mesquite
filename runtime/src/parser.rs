use crate::{
    api::{LogType, ParseOptions, ParseState},
    clock::*,
    language::*,
    length::Length,
    lexer::LexerState,
    reduce_action::*,
    reusable_node::ReusableNode,
    stack::*,
    subtree::*,
    tree::Tree,
    types::*,
};
use std::sync::{Arc, atomic::AtomicUsize};
use ts_port_tables::{ExternalScanner, LexMode, SERIALIZATION_BUFFER_SIZE};
pub(crate) const MAX_VERSION_COUNT: u32 = 6;
pub(crate) const MAX_VERSION_COUNT_OVERFLOW: u32 = 4;
pub(crate) const MAX_SUMMARY_DEPTH: u32 = 16;
pub(crate) const MAX_COST_DIFFERENCE: u32 = 18 * crate::error_costs::ERROR_COST_PER_SKIPPED_TREE;
pub(crate) const OP_COUNT_PER_PARSER_TIMEOUT_CHECK: u32 = 100;
pub(crate) type Logger = Box<dyn FnMut(LogType, &str) + Send>;
#[derive(Debug, Default)]
pub(crate) struct TokenCache {
    pub token: Subtree,
    pub last_external_token: Subtree,
    pub byte_index: u32,
}
pub struct Parser {
    pub(crate) lexer: LexerState,
    pub(crate) stack: Stack,
    pub(crate) tree_pool: SubtreePool,
    pub(crate) language: Option<Language>,
    pub(crate) reduce_actions: ReduceActionSet,
    pub(crate) finished_tree: Subtree,
    pub(crate) trailing_extras: Vec<Subtree>,
    pub(crate) trailing_extras2: Vec<Subtree>,
    pub(crate) scratch_trees: Vec<Subtree>,
    pub(crate) token_cache: TokenCache,
    pub(crate) reusable_node: ReusableNode,
    pub(crate) external_scanner: Option<Box<dyn ExternalScanner>>,
    pub(crate) scanner_buffer: [u8; SERIALIZATION_BUFFER_SIZE],
    pub(crate) logger: Option<Logger>,
    pub(crate) dot_graph: Option<Box<dyn std::io::Write + Send>>,
    pub(crate) end_clock: Clock,
    pub(crate) timeout_duration: DurationMicros,
    pub(crate) accept_count: u32,
    pub(crate) operation_count: u32,
    pub(crate) cancellation_flag: Option<Arc<AtomicUsize>>,
    pub(crate) old_tree: Subtree,
    pub(crate) included_range_differences: Vec<Range>,
    pub(crate) parse_state: ParseState,
    pub(crate) included_range_difference_index: u32,
    pub(crate) has_scanner_error: bool,
    pub(crate) canceled_balancing: bool,
    pub(crate) has_error: bool,
    // Child indices to the next subtree to balance after cancellation. Unlike
    // C's unretained tree-stack pointers, this does not affect Arc uniqueness.
    pub(crate) balance_path: Vec<usize>,
    }
/// Explicitly passed down the call chain: borrowed callback state cannot outlive
/// one invocation, even when the parser retains an outstanding parse.
pub(crate) struct ParseContext<'a, 'options> {
    pub input: &'a mut dyn Input,
    pub options: ParseOptions<'options>,
}
#[derive(Clone, Copy, Debug)]
pub(crate) struct ErrorStatus {
    pub cost: u32,
    pub node_count: u32,
    pub dynamic_precedence: i32,
    pub is_in_error: bool,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ErrorComparison {
    TakeLeft,
    PreferLeft,
    None,
    PreferRight,
    TakeRight,
}

pub(crate) fn ts_string_input_read(input: &[u8], byte: u32, point: Point) -> &[u8] {
    todo!("parser-1: ts_string_input_read")
}

pub(crate) fn ts_parser__log(parser: &mut Parser) {
    todo!("parser-1: ts_parser__log")
}

pub(crate) fn ts_parser__breakdown_top_of_stack(
    parser: &mut Parser,
    version: StackVersion,
) -> bool {
    todo!("parser-1: ts_parser__breakdown_top_of_stack")
}

pub(crate) fn ts_parser__breakdown_lookahead(
    parser: &mut Parser,
    lookahead: &mut Subtree,
    state: StateId,
) {
    todo!("parser-1: ts_parser__breakdown_lookahead")
}

pub(crate) fn ts_parser__compare_versions(
    parser: &Parser,
    a: ErrorStatus,
    b: ErrorStatus,
) -> ErrorComparison {
    todo!("parser-1: ts_parser__compare_versions")
}

pub(crate) fn ts_parser__version_status(parser: &Parser, version: StackVersion) -> ErrorStatus {
    todo!("parser-1: ts_parser__version_status")
}

pub(crate) fn ts_parser__better_version_exists(
    parser: &Parser,
    version: StackVersion,
    is_in_error: bool,
    cost: u32,
) -> bool {
    todo!("parser-1: ts_parser__better_version_exists")
}

pub(crate) fn ts_parser__call_main_lex_fn(
    parser: &mut Parser,
    context: &mut ParseContext<'_, '_>,
    lex_mode: LexMode,
) -> bool {
    todo!("parser-1: ts_parser__call_main_lex_fn")
}

pub(crate) fn ts_parser__call_keyword_lex_fn(
    parser: &mut Parser,
    context: &mut ParseContext<'_, '_>,
) -> bool {
    todo!("parser-1: ts_parser__call_keyword_lex_fn")
}

pub(crate) fn ts_parser__external_scanner_create(parser: &mut Parser) {
    todo!("parser-1: ts_parser__external_scanner_create")
}

pub(crate) fn ts_parser__external_scanner_destroy(parser: &mut Parser) {
    todo!("parser-1: ts_parser__external_scanner_destroy")
}

pub(crate) fn ts_parser__external_scanner_serialize(parser: &mut Parser) -> u32 {
    todo!("parser-1: ts_parser__external_scanner_serialize")
}

pub(crate) fn ts_parser__external_scanner_deserialize(
    parser: &mut Parser,
    external_token: &Subtree,
) {
    todo!("parser-1: ts_parser__external_scanner_deserialize")
}

pub(crate) fn ts_parser__external_scanner_scan(
    parser: &mut Parser,
    context: &mut ParseContext<'_, '_>,
    external_lex_state: StateId,
) -> bool {
    todo!("parser-1: ts_parser__external_scanner_scan")
}

pub(crate) fn ts_parser__can_reuse_first_leaf(
    parser: &Parser,
    state: StateId,
    tree: &Subtree,
    table_entry: &mut TableEntry,
) -> bool {
    todo!("parser-1: ts_parser__can_reuse_first_leaf")
}

pub(crate) fn ts_parser__lex(
    parser: &mut Parser,
    context: &mut ParseContext<'_, '_>,
    version: StackVersion,
    parse_state: StateId,
) -> Subtree {
    todo!("parser-1: ts_parser__lex")
}

pub(crate) fn ts_parser__get_cached_token(
    parser: &mut Parser,
    state: StateId,
    position: usize,
    last_external_token: &Subtree,
    table_entry: &mut TableEntry,
) -> Subtree {
    todo!("parser-2: ts_parser__get_cached_token")
}

pub(crate) fn ts_parser__set_cached_token(
    parser: &mut Parser,
    byte_index: u32,
    last_external_token: Subtree,
    token: Subtree,
) {
    todo!("parser-2: ts_parser__set_cached_token")
}

pub(crate) fn ts_parser__has_included_range_difference(
    parser: &Parser,
    start_position: u32,
    end_position: u32,
) -> bool {
    todo!("parser-2: ts_parser__has_included_range_difference")
}

pub(crate) fn ts_parser__reuse_node(
    parser: &mut Parser,
    version: StackVersion,
    state: &mut StateId,
    position: u32,
    last_external_token: &Subtree,
    table_entry: &mut TableEntry,
) -> Subtree {
    todo!("parser-2: ts_parser__reuse_node")
}

pub(crate) fn ts_parser__select_tree(parser: &mut Parser, left: &Subtree, right: &Subtree) -> bool {
    todo!("parser-2: ts_parser__select_tree")
}

pub(crate) fn ts_parser__select_children(
    parser: &mut Parser,
    left: &Subtree,
    children: &[Subtree],
) -> bool {
    todo!("parser-2: ts_parser__select_children")
}

pub(crate) fn ts_parser__shift(
    parser: &mut Parser,
    version: StackVersion,
    state: StateId,
    lookahead: Subtree,
    extra: bool,
) {
    todo!("parser-2: ts_parser__shift")
}

pub(crate) fn ts_parser__reduce(
    parser: &mut Parser,
    version: StackVersion,
    symbol: Symbol,
    count: u32,
    dynamic_precedence: i32,
    production_id: u16,
    is_fragile: bool,
    end_of_non_terminal_extra: bool,
) -> StackVersion {
    todo!("parser-2: ts_parser__reduce")
}

pub(crate) fn ts_parser__accept(parser: &mut Parser, version: StackVersion, lookahead: Subtree) {
    todo!("parser-2: ts_parser__accept")
}

pub(crate) fn ts_parser__do_all_potential_reductions(
    parser: &mut Parser,
    starting_version: StackVersion,
    lookahead_symbol: Symbol,
) -> bool {
    todo!("parser-2: ts_parser__do_all_potential_reductions")
}

pub(crate) fn ts_parser__recover_to_state(
    parser: &mut Parser,
    version: StackVersion,
    depth: u32,
    goal_state: StateId,
) -> bool {
    todo!("parser-2: ts_parser__recover_to_state")
}

pub(crate) fn ts_parser__recover(parser: &mut Parser, version: StackVersion, lookahead: Subtree) {
    todo!("parser-2: ts_parser__recover")
}

pub(crate) fn ts_parser__handle_error(
    parser: &mut Parser,
    version: StackVersion,
    lookahead: Subtree,
) {
    let language = parser.language.expect("parser language");
    let previous_version_count = ts_stack_version_count(&parser.stack);

    // Reductions that were blocked by an invalid token may now be possible.
    ts_parser__do_all_potential_reductions(parser, version, 0);
    let version_count = ts_stack_version_count(&parser.stack);
    let position = ts_stack_position(&parser.stack, version);
    let mut did_insert_missing_token = false;
    let mut v = version;
    while v < version_count {
        if !did_insert_missing_token {
            let state = ts_stack_state(&parser.stack, v);
            for missing_symbol in 1..language.tables.token_count as u16 {
                let next_state = ts_language_next_state(&language, state, missing_symbol);
                if next_state == 0 || next_state == state {
                    continue;
                }
                if ts_language_has_reduce_action(
                    &language,
                    next_state,
                    ts_subtree_leaf_symbol(&lookahead),
                ) {
                    // Reset snaps to the next included range; give the missing
                    // token padding that places it inside that range.
                    crate::lexer::ts_lexer_reset(&mut parser.lexer, position);
                    crate::lexer::ts_lexer_mark_end(&mut parser.lexer);
                    let padding =
                        crate::length::length_sub(parser.lexer.token_end_position, position);
                    let lookahead_bytes = ts_subtree_total_bytes(&lookahead)
                        .wrapping_add(ts_subtree_lookahead_bytes(&lookahead));
                    let missing_version = ts_stack_copy_version(&mut parser.stack, v);
                    let missing_tree = ts_subtree_new_missing_leaf(
                        &mut parser.tree_pool,
                        missing_symbol,
                        padding,
                        lookahead_bytes,
                        &language,
                    );
                    ts_stack_push(
                        &mut parser.stack,
                        &mut parser.tree_pool,
                        missing_version,
                        missing_tree,
                        false,
                        next_state,
                    );
                    if ts_parser__do_all_potential_reductions(
                        parser,
                        missing_version,
                        ts_subtree_leaf_symbol(&lookahead),
                    ) {
                        parser3_log(
                            parser,
                            format_args!(
                                "recover_with_missing symbol:{}, state:{}",
                                ts_language_symbol_name(&language, missing_symbol).unwrap_or(""),
                                ts_stack_state(&parser.stack, missing_version),
                            ),
                        );
                        did_insert_missing_token = true;
                        break;
                    }
                }
            }
        }
        ts_stack_push(
            &mut parser.stack,
            &mut parser.tree_pool,
            v,
            Subtree::Null,
            false,
            crate::error_costs::ERROR_STATE,
        );
        v = if v == version {
            previous_version_count
        } else {
            v + 1
        };
    }
    for _ in previous_version_count..version_count {
        let did_merge = ts_stack_merge(
            &mut parser.stack,
            &mut parser.tree_pool,
            version,
            previous_version_count,
        );
        assert!(did_merge);
    }
    ts_stack_record_summary(
        &mut parser.stack,
        &mut parser.tree_pool,
        version,
        MAX_SUMMARY_DEPTH,
    );

    // Recover immediately so this lookahead's scanned bytes are accounted for.
    let mut lookahead = lookahead;
    if ts_subtree_child_count(&lookahead) > 0 {
        ts_parser__breakdown_lookahead(parser, &mut lookahead, crate::error_costs::ERROR_STATE);
    }
    ts_parser__recover(parser, version, lookahead);
    parser3_log_stack(parser);
}

pub(crate) fn ts_parser__check_progress(
    parser: &mut Parser,
    context: &mut ParseContext<'_, '_>,
    lookahead: Option<&mut Subtree>,
    position: Option<u32>,
    operations: u32,
) -> bool {
    parser.operation_count = parser.operation_count.wrapping_add(operations);
    if parser.operation_count >= OP_COUNT_PER_PARSER_TIMEOUT_CHECK {
        // Not a modulus: even large balancing increments reset to zero.
        parser.operation_count = 0;
    }
    if let Some(position) = position {
        parser.parse_state.current_byte_offset = position;
        parser.parse_state.has_error = parser.has_error;
    }
    if parser.operation_count == 0
        && (parser
            .cancellation_flag
            .as_ref()
            .is_some_and(|flag| flag.load(std::sync::atomic::Ordering::Relaxed) != 0)
            || (!clock_is_null(parser.end_clock) && clock_is_gt(clock_now(), parser.end_clock))
            || context
                .options
                .progress_callback
                .as_mut()
                .is_some_and(|callback| callback(&parser.parse_state)))
    {
        if let Some(lookahead) = lookahead
            && !lookahead.is_null()
        {
            ts_subtree_release(&mut parser.tree_pool, std::mem::take(lookahead));
        }
        return false;
    }
    true
}

pub(crate) fn ts_parser__advance(
    parser: &mut Parser,
    context: &mut ParseContext<'_, '_>,
    version: StackVersion,
    allow_node_reuse: bool,
) -> bool {
    use ts_port_tables::{BUILTIN_SYM_END, ParseAction};
    let language = parser.language.expect("parser language");
    let mut state = ts_stack_state(&parser.stack, version);
    let position = ts_stack_position(&parser.stack, version).bytes;
    let last_external_token = ts_stack_last_external_token(&parser.stack, version).clone();
    let mut did_reuse = true;
    let mut lookahead = Subtree::Null;
    let mut table_entry = TableEntry::default();

    if allow_node_reuse {
        lookahead = ts_parser__reuse_node(
            parser,
            version,
            &mut state,
            position,
            &last_external_token,
            &mut table_entry,
        );
    }
    if lookahead.is_null() {
        did_reuse = false;
        lookahead = ts_parser__get_cached_token(
            parser,
            state,
            position as usize,
            &last_external_token,
            &mut table_entry,
        );
    }
    let mut needs_lex = lookahead.is_null();
    loop {
        if needs_lex {
            needs_lex = false;
            lookahead = ts_parser__lex(parser, context, version, state);
            if parser.has_scanner_error {
                return false;
            }
            let symbol = if !lookahead.is_null() {
                ts_parser__set_cached_token(
                    parser,
                    position,
                    last_external_token.clone(),
                    lookahead.clone(),
                );
                ts_subtree_symbol(&lookahead)
            } else {
                // Null lookahead terminates a non-terminal extra; its fixed
                // reduction is stored in the EOF entry. Lex again afterwards.
                BUILTIN_SYM_END
            };
            table_entry = ts_language_table_entry(&language, state, symbol);
        }
        if !ts_parser__check_progress(parser, context, Some(&mut lookahead), Some(position), 1) {
            return false;
        }

        let mut did_reduce = false;
        let mut last_reduction_version = STACK_VERSION_NONE;
        for entry in table_entry.actions {
            match *entry.action() {
                ParseAction::Shift {
                    state: shift_state,
                    extra,
                    repetition,
                } => {
                    if repetition {
                        continue;
                    }
                    let mut next_state = if extra {
                        parser3_log(parser, format_args!("shift_extra"));
                        state
                    } else {
                        parser3_log(parser, format_args!("shift state:{}", shift_state));
                        shift_state
                    };
                    if ts_subtree_child_count(&lookahead) > 0 {
                        ts_parser__breakdown_lookahead(parser, &mut lookahead, state);
                        next_state =
                            ts_language_next_state(&language, state, ts_subtree_symbol(&lookahead));
                    }
                    ts_parser__shift(parser, version, next_state, lookahead, extra);
                    if did_reuse {
                        crate::reusable_node::reusable_node_advance(&mut parser.reusable_node);
                    }
                    return true;
                }
                ParseAction::Reduce {
                    symbol,
                    child_count,
                    dynamic_precedence,
                    production_id,
                } => {
                    parser3_log(
                        parser,
                        format_args!(
                            "reduce sym:{}, child_count:{}",
                            ts_language_symbol_name(&language, symbol).unwrap_or(""),
                            child_count,
                        ),
                    );
                    let reduction_version = ts_parser__reduce(
                        parser,
                        version,
                        symbol,
                        u32::from(child_count),
                        i32::from(dynamic_precedence),
                        production_id,
                        table_entry.actions.len() > 1,
                        lookahead.is_null(),
                    );
                    did_reduce = true;
                    if reduction_version != STACK_VERSION_NONE {
                        last_reduction_version = reduction_version;
                    }
                }
                ParseAction::Accept => {
                    parser3_log(parser, format_args!("accept"));
                    ts_parser__accept(parser, version, lookahead);
                    return true;
                }
                ParseAction::Recover => {
                    if ts_subtree_child_count(&lookahead) > 0 {
                        ts_parser__breakdown_lookahead(
                            parser,
                            &mut lookahead,
                            crate::error_costs::ERROR_STATE,
                        );
                    }
                    ts_parser__recover(parser, version, lookahead);
                    if did_reuse {
                        crate::reusable_node::reusable_node_advance(&mut parser.reusable_node);
                    }
                    return true;
                }
            }
        }

        if last_reduction_version != STACK_VERSION_NONE {
            ts_stack_renumber_version(
                &mut parser.stack,
                &mut parser.tree_pool,
                last_reduction_version,
                version,
            );
            parser3_log_stack(parser);
            state = ts_stack_state(&parser.stack, version);
            if lookahead.is_null() {
                needs_lex = true;
            } else {
                table_entry =
                    ts_language_table_entry(&language, state, ts_subtree_leaf_symbol(&lookahead));
            }
            continue;
        }
        if did_reduce {
            if !lookahead.is_null() {
                ts_subtree_release(&mut parser.tree_pool, lookahead);
            }
            ts_stack_halt(&mut parser.stack, version);
            return true;
        }

        let word_symbol = language.tables.keyword_capture_token;
        if ts_subtree_is_keyword(&lookahead)
            && ts_subtree_symbol(&lookahead) != word_symbol
            && !ts_language_is_reserved_word(&language, state, ts_subtree_symbol(&lookahead))
        {
            table_entry = ts_language_table_entry(&language, state, word_symbol);
            if !table_entry.actions.is_empty() {
                parser3_log(
                    parser,
                    format_args!(
                        "switch from_keyword:{}, to_word_token:{}",
                        ts_language_symbol_name(&language, ts_subtree_symbol(&lookahead))
                            .unwrap_or(""),
                        ts_language_symbol_name(&language, word_symbol).unwrap_or(""),
                    ),
                );
                lookahead = ts_subtree_make_mut(&mut parser.tree_pool, lookahead);
                ts_subtree_set_symbol(&mut lookahead, word_symbol, &language);
                continue;
            }
        }

        // A reused predecessor may have been invalid in this context. Break it
        // down, discard the lookahead, and lex again against the new stack top.
        if ts_parser__breakdown_top_of_stack(parser, version) {
            state = ts_stack_state(&parser.stack, version);
            ts_subtree_release(&mut parser.tree_pool, std::mem::take(&mut lookahead));
            needs_lex = true;
            continue;
        }
        parser3_log(
            parser,
            format_args!(
                "detect_error lookahead:{}",
                ts_language_symbol_name(&language, ts_subtree_symbol(&lookahead)).unwrap_or(""),
            ),
        );
        ts_stack_pause(&mut parser.stack, &mut parser.tree_pool, version, lookahead);
        return true;
    }
}

pub(crate) fn ts_parser__condense_stack(parser: &mut Parser) -> u32 {
    let mut made_changes = false;
    let mut min_error_cost = u32::MAX;
    let mut i = 0;
    while i < ts_stack_version_count(&parser.stack) {
        if ts_stack_is_halted(&parser.stack, i) {
            ts_stack_remove_version(&mut parser.stack, &mut parser.tree_pool, i);
            continue;
        }
        let status_i = ts_parser__version_status(parser, i);
        if !status_i.is_in_error && status_i.cost < min_error_cost {
            min_error_cost = status_i.cost;
        }
        let mut j = 0;
        let mut removed_i = false;
        while j < i {
            let status_j = ts_parser__version_status(parser, j);
            match ts_parser__compare_versions(parser, status_j, status_i) {
                ErrorComparison::TakeLeft => {
                    made_changes = true;
                    ts_stack_remove_version(&mut parser.stack, &mut parser.tree_pool, i);
                    removed_i = true;
                    break;
                }
                ErrorComparison::PreferLeft | ErrorComparison::None => {
                    if ts_stack_merge(&mut parser.stack, &mut parser.tree_pool, j, i) {
                        made_changes = true;
                        removed_i = true;
                        break;
                    }
                }
                ErrorComparison::PreferRight => {
                    made_changes = true;
                    if ts_stack_merge(&mut parser.stack, &mut parser.tree_pool, j, i) {
                        removed_i = true;
                        break;
                    }
                    ts_stack_swap_versions(&mut parser.stack, i, j);
                }
                ErrorComparison::TakeRight => {
                    made_changes = true;
                    ts_stack_remove_version(&mut parser.stack, &mut parser.tree_pool, j);
                    i -= 1;
                    // The next item is now at j; C decrements j before the
                    // for-loop increment. Keep status_i as in the C loop.
                    continue;
                }
            }
            j += 1;
        }
        if !removed_i {
            i += 1;
        }
    }
    while ts_stack_version_count(&parser.stack) > MAX_VERSION_COUNT {
        ts_stack_remove_version(&mut parser.stack, &mut parser.tree_pool, MAX_VERSION_COUNT);
        made_changes = true;
    }

    let mut has_unpaused_version = false;
    let mut i = 0;
    // Error recovery may add versions. Only visit the versions present at the
    // beginning of this loop, decreasing the bound when one is removed.
    let mut n = ts_stack_version_count(&parser.stack);
    while i < n {
        if ts_stack_is_paused(&parser.stack, i) {
            if !has_unpaused_version && parser.accept_count < MAX_VERSION_COUNT {
                parser3_log(parser, format_args!("resume version:{}", i));
                min_error_cost = ts_stack_error_cost(&parser.stack, i);
                let lookahead = ts_stack_resume(&mut parser.stack, i);
                ts_parser__handle_error(parser, i, lookahead);
                has_unpaused_version = true;
            } else {
                ts_stack_remove_version(&mut parser.stack, &mut parser.tree_pool, i);
                made_changes = true;
                n -= 1;
                continue;
            }
        } else {
            has_unpaused_version = true;
        }
        i += 1;
    }
    if made_changes {
        parser3_log(parser, format_args!("condense"));
        parser3_log_stack(parser);
    }
    min_error_cost
}

pub(crate) fn ts_parser__balance_subtree(
    parser: &mut Parser,
    context: &mut ParseContext<'_, '_>,
) -> bool {
    if !parser.canceled_balancing {
        parser.tree_pool.tree_stack.clear();
        parser.balance_path.clear();
        if !BalanceCursor::is_unique_branch(&parser.finished_tree) {
            return true;
        }
    }
    let mut cursor = BalanceCursor {
        tree: std::mem::take(&mut parser.finished_tree),
        parents: Vec::new(),
    };
    for &index in &parser.balance_path {
        cursor.descend(index);
    }
    parser.balance_path.clear();
    loop {
        if !ts_parser__check_progress(parser, context, None, None, 1) {
            cursor.save(parser);
            return false;
        }
        if ts_subtree_repeat_depth(&cursor.tree) > 0 {
            let children = ts_subtree_children(&cursor.tree);
            let repeat_delta = i64::from(ts_subtree_repeat_depth(&children[0]))
                - i64::from(ts_subtree_repeat_depth(children.last().unwrap()));
            if repeat_delta > 0 {
                let mut i = repeat_delta as u32 / 2;
                while i > 0 {
                    ts_subtree_compress(
                        &mut cursor.tree,
                        i,
                        &parser.language.expect("parser language"),
                        &mut parser.tree_pool.tree_stack,
                    );
                    // C narrows this to uint8_t, including wrapping to zero.
                    let operations = (i >> 4).max(1) as u8;
                    if !ts_parser__check_progress(
                        parser,
                        context,
                        None,
                        None,
                        u32::from(operations),
                    ) {
                        cursor.save(parser);
                        return false;
                    }
                    i /= 2;
                }
            }
        }
        if !cursor.advance() {
            parser.finished_tree = cursor.tree;
            return true;
        }
    }
}

pub(crate) fn ts_parser_has_outstanding_parse(parser: &mut Parser) -> bool {
    if parser.canceled_balancing || parser.external_scanner.is_some() {
        return true;
    }
    // The node-count accessor updates its error baseline after a stack pop.
    let stack = &mut parser.stack;
    ts_stack_state(stack, 0) != 1 || ts_stack_node_count_since_error(stack, 0) != 0
}

pub(crate) fn ts_parser_new() -> Parser {
    let mut parser = Parser {
        lexer: crate::lexer::ts_lexer_init(),
        reduce_actions: Vec::with_capacity(4),
        tree_pool: ts_subtree_pool_new(32),
        stack: ts_stack_new(),
        finished_tree: Subtree::Null,
        reusable_node: crate::reusable_node::reusable_node_new(),
        dot_graph: None,
        logger: None,
        cancellation_flag: None,
        timeout_duration: 0,
        language: None,
        has_scanner_error: false,
        has_error: false,
        canceled_balancing: false,
        external_scanner: None,
        scanner_buffer: [0; SERIALIZATION_BUFFER_SIZE],
        end_clock: clock_null(),
        operation_count: 0,
        accept_count: 0,
        old_tree: Subtree::Null,
        included_range_differences: Vec::new(),
        included_range_difference_index: 0,
        token_cache: TokenCache::default(),
        trailing_extras: Vec::new(),
        trailing_extras2: Vec::new(),
        scratch_trees: Vec::new(),
        parse_state: ParseState::default(),
        balance_path: Vec::new(),
    };
    ts_parser__set_cached_token(&mut parser, 0, Subtree::Null, Subtree::Null);
    parser
}

pub(crate) fn ts_parser_delete(parser: &mut Parser) {
    ts_parser_set_language(parser, None);
    ts_stack_delete(&mut parser.stack, &mut parser.tree_pool);
    parser.reduce_actions = Vec::new();
    parser.included_range_differences = Vec::new();
    if !parser.old_tree.is_null() {
        ts_subtree_release(&mut parser.tree_pool, std::mem::take(&mut parser.old_tree));
    }
    crate::lexer::ts_lexer_delete(&mut parser.lexer);
    ts_parser__set_cached_token(parser, 0, Subtree::Null, Subtree::Null);
    ts_subtree_pool_delete(&mut parser.tree_pool);
    crate::reusable_node::reusable_node_delete(&mut parser.reusable_node);
    parser.trailing_extras = Vec::new();
    parser.trailing_extras2 = Vec::new();
    parser.scratch_trees = Vec::new();
    parser.balance_path = Vec::new();
    // The enclosing Parser and its owned I/O/closure handles are dropped by Rust.
}

pub(crate) fn ts_parser_language(parser: &Parser) -> Option<&Language> {
    parser.language.as_ref()
}

pub(crate) fn ts_parser_set_language(parser: &mut Parser, language: Option<&Language>) -> bool {
    ts_parser_reset(parser);
    if let Some(previous) = parser.language.take() {
        ts_language_delete(previous);
    }
    if let Some(language) = language {
        let version = language.tables.abi_version;
        if version > crate::api::LANGUAGE_VERSION as u32
            || version < crate::api::MIN_COMPATIBLE_LANGUAGE_VERSION as u32
        {
            return false;
        }
        // All supported languages are native Rust tables; wasm is out of scope.
        parser.language = Some(ts_language_copy(language));
    }
    true
}

pub(crate) fn ts_parser_logger(parser: &Parser) -> Option<&Logger> {
    parser.logger.as_ref()
}

pub(crate) fn ts_parser_set_logger(parser: &mut Parser, logger: Option<Logger>) {
    parser.logger = logger;
}

pub(crate) fn ts_parser_print_dot_graphs(
    parser: &mut Parser,
    output: Option<Box<dyn std::io::Write + Send>>,
) {
    // Closing C's previous FILE flushes any buffered graph output.
    if let Some(mut previous) = parser.dot_graph.take() {
        let _ = previous.flush();
    }
    parser.dot_graph = output;
}

pub(crate) fn ts_parser_cancellation_flag(parser: &Parser) -> Option<&Arc<AtomicUsize>> {
    parser.cancellation_flag.as_ref()
}

pub(crate) fn ts_parser_set_cancellation_flag(parser: &mut Parser, flag: Option<Arc<AtomicUsize>>) {
    parser.cancellation_flag = flag;
}

pub(crate) fn ts_parser_timeout_micros(parser: &Parser) -> u64 {
    duration_to_micros(parser.timeout_duration)
}

pub(crate) fn ts_parser_set_timeout_micros(parser: &mut Parser, timeout_micros: u64) {
    parser.timeout_duration = duration_from_micros(timeout_micros);
}

pub(crate) fn ts_parser_set_included_ranges(parser: &mut Parser, ranges: &[Range]) -> bool {
    crate::lexer::ts_lexer_set_included_ranges(&mut parser.lexer, ranges)
}

pub(crate) fn ts_parser_included_ranges(parser: &Parser) -> &[Range] {
    crate::lexer::ts_lexer_included_ranges(&parser.lexer)
}

pub(crate) fn ts_parser_reset(parser: &mut Parser) {
    ts_parser__external_scanner_destroy(parser);

    if !parser.old_tree.is_null() {
        ts_subtree_release(&mut parser.tree_pool, std::mem::take(&mut parser.old_tree));
    }

    crate::reusable_node::reusable_node_clear(&mut parser.reusable_node);
    crate::lexer::ts_lexer_reset(&mut parser.lexer, crate::length::length_zero());
    ts_stack_clear(&mut parser.stack, &mut parser.tree_pool);
    ts_parser__set_cached_token(parser, 0, Subtree::Null, Subtree::Null);
    if !parser.finished_tree.is_null() {
        ts_subtree_release(
            &mut parser.tree_pool,
            std::mem::take(&mut parser.finished_tree),
        );
    }
    parser.accept_count = 0;
    parser.has_scanner_error = false;
    parser.has_error = false;
    parser.canceled_balancing = false;
    // Parse options live only in the borrowed ParseContext, not in Parser.
    parser.parse_state = ParseState::default();
}

pub(crate) fn ts_parser_parse(
    parser: &mut Parser,
    old_tree: Option<&Tree>,
    context: &mut ParseContext<'_, '_>,
) -> Option<Tree> {
    let language = parser.language?;

    // Input is a trait object with a required read method, so C's null-read
    // callback check is unnecessary. The caller selects the input encoding.
    let encoding = parser.lexer.encoding;
    crate::lexer::ts_lexer_set_input(&mut parser.lexer, encoding);
    parser.included_range_differences.clear();
    parser.included_range_difference_index = 0;

    parser.operation_count = 0;
    parser.end_clock = if parser.timeout_duration != 0 {
        clock_after(clock_now(), parser.timeout_duration)
    } else {
        clock_null()
    };

    let resume_balancing = if ts_parser_has_outstanding_parse(parser) {
        if parser.logger.is_some() || parser.dot_graph.is_some() {
            parser.lexer.debug_buffer = "resume_parsing".into();
            ts_parser__log(parser);
        }
        parser.canceled_balancing
    } else {
        ts_parser__external_scanner_create(parser);
        if parser.has_scanner_error {
            ts_parser_reset(parser);
            return None;
        }

        if let Some(old_tree) = old_tree {
            parser.old_tree = old_tree.root.as_ref().clone();
            crate::get_changed_ranges::ts_range_array_get_changed_ranges(
                &old_tree.included_ranges,
                &parser.lexer.included_ranges,
                &mut parser.included_range_differences,
            );
            crate::reusable_node::reusable_node_reset(
                &mut parser.reusable_node,
                old_tree.root.as_ref().clone(),
            );
            if parser.logger.is_some() || parser.dot_graph.is_some() {
                parser.lexer.debug_buffer = "parse_after_edit".into();
                ts_parser__log(parser);
            }
            if let Some(output) = parser.dot_graph.as_mut() {
                let _ = ts_subtree_print_dot_graph(&parser.old_tree, &language, output);
                let _ = output.write_all(b"\n");
            }
            for i in 0..parser.included_range_differences.len() {
                if parser.logger.is_some() || parser.dot_graph.is_some() {
                    let range = &parser.included_range_differences[i];
                    parser.lexer.debug_buffer = format!(
                        "different_included_range {} - {}",
                        range.start_byte, range.end_byte,
                    );
                    ts_parser__log(parser);
                }
            }
        } else {
            crate::reusable_node::reusable_node_clear(&mut parser.reusable_node);
            if parser.logger.is_some() || parser.dot_graph.is_some() {
                parser.lexer.debug_buffer = "new_parse".into();
                ts_parser__log(parser);
            }
        }
        false
    };

    if !resume_balancing {
        let mut position = 0;
        let mut last_position = 0;
        loop {
            let mut version = 0;
            // Re-read the version count at every for-loop condition, including
            // the final failed condition, exactly as in C. Advance may split
            // or remove versions, but allow_node_reuse is fixed per version.
            let version_count = loop {
                let version_count = ts_stack_version_count(&parser.stack);
                if version >= version_count {
                    break version_count;
                }
                let allow_node_reuse = version_count == 1;
                while ts_stack_is_active(&parser.stack, version) {
                    if parser.logger.is_some() || parser.dot_graph.is_some() {
                        let point = ts_stack_position(&parser.stack, version).extent;
                        parser.lexer.debug_buffer = format!(
                            "process version:{}, version_count:{}, state:{}, row:{}, col:{}",
                            version,
                            ts_stack_version_count(&parser.stack),
                            ts_stack_state(&parser.stack, version),
                            point.row,
                            point.column,
                        );
                        ts_parser__log(parser);
                    }

                    if !ts_parser__advance(parser, context, version, allow_node_reuse) {
                        if parser.has_scanner_error {
                            ts_parser_reset(parser);
                        }
                        // Timeout/progress cancellation preserves the stack and
                        // scanner so the next call can resume this parse.
                        return None;
                    }

                    if let Some(output) = parser.dot_graph.as_mut() {
                        let _ = ts_stack_print_dot_graph(&mut parser.stack, &language, output);
                        let _ = output.write_all(b"\n\n");
                    }

                    position = ts_stack_position(&parser.stack, version).bytes;
                    if position > last_position || (version > 0 && position == last_position) {
                        last_position = position;
                        break;
                    }
                }
                version += 1;
            };

            let min_error_cost = ts_parser__condense_stack(parser);
            if !parser.finished_tree.is_null()
                && ts_subtree_error_cost(&parser.finished_tree) < min_error_cost
            {
                // Drop stack references before rebalancing the accepted tree.
                ts_stack_clear(&mut parser.stack, &mut parser.tree_pool);
                break;
            }

            while let Some(range) = parser
                .included_range_differences
                .get(parser.included_range_difference_index as usize)
            {
                if range.end_byte <= position {
                    parser.included_range_difference_index += 1;
                } else {
                    break;
                }
            }
            if version_count == 0 {
                break;
            }
        }
    }

    assert!(!parser.finished_tree.is_null());
    if !ts_parser__balance_subtree(parser, context) {
        parser.canceled_balancing = true;
        return None;
    }
    parser.canceled_balancing = false;
    if parser.logger.is_some() || parser.dot_graph.is_some() {
        parser.lexer.debug_buffer = "done".into();
        ts_parser__log(parser);
    }
    if let Some(output) = parser.dot_graph.as_mut() {
        let _ = ts_subtree_print_dot_graph(&parser.finished_tree, &language, output);
        let _ = output.write_all(b"\n");
    }

    let result = crate::tree::ts_tree_new(
        std::mem::take(&mut parser.finished_tree),
        &language,
        &parser.lexer.included_ranges,
    );
    ts_parser_reset(parser);
    Some(result)
}

pub(crate) fn ts_parser_parse_with_options(
    parser: &mut Parser,
    old_tree: Option<&Tree>,
    input: &mut dyn Input,
    options: ParseOptions<'_>,
) -> Option<Tree> {
    // Callback input in the supported public API is UTF-8. Options (including
    // their borrowed payload) are dropped at return, even after cancellation.
    parser.lexer.encoding = InputEncoding::Utf8;
    let mut context = ParseContext { input, options };
    ts_parser_parse(parser, old_tree, &mut context)
}

pub(crate) fn ts_parser_parse_string(
    parser: &mut Parser,
    old_tree: Option<&Tree>,
    string: &[u8],
) -> Option<Tree> {
    ts_parser_parse_string_encoding(parser, old_tree, string, InputEncoding::Utf8)
}

pub(crate) fn ts_parser_parse_string_encoding(
    parser: &mut Parser,
    old_tree: Option<&Tree>,
    string: &[u8],
    encoding: InputEncoding,
) -> Option<Tree> {
    let mut input = SliceInput {
        bytes: string,
        chunk_start: 0,
    };
    parser.lexer.encoding = encoding;
    let mut context = ParseContext {
        input: &mut input,
        options: ParseOptions::default(),
    };
    ts_parser_parse(parser, old_tree, &mut context)
}

// Local equivalents of parser.c's LOG / LOG_STACK macros. Formatting is kept
// out of the hot path when diagnostics are disabled.
fn parser3_log(parser: &mut Parser, message: std::fmt::Arguments<'_>) {
    if parser.logger.is_some() || parser.dot_graph.is_some() {
        use std::fmt::Write;
        parser.lexer.debug_buffer.clear();
        let _ = parser.lexer.debug_buffer.write_fmt(message);
        if parser.lexer.debug_buffer.len() >= SERIALIZATION_BUFFER_SIZE {
            let mut end = SERIALIZATION_BUFFER_SIZE - 1;
            while !parser.lexer.debug_buffer.is_char_boundary(end) {
                end -= 1;
            }
            parser.lexer.debug_buffer.truncate(end);
        }
        ts_parser__log(parser);
    }
}

fn parser3_log_stack(parser: &mut Parser) {
    if let Some(output) = parser.dot_graph.as_mut() {
        let stack = &mut parser.stack;
        let _ = ts_stack_print_dot_graph(stack, &parser.language.expect("parser language"), output);
        let _ = output.write_all(b"\n\n");
    }
}

/// An owning zipper for C's non-owning balancing worklist. A detached child is
/// replaced by Null in its parent until we ascend, so no Arc reference counts
/// change and compression still sees exactly the uniquely owned C subtrees.
/// Children are visited last-to-first, matching C's push-all-children worklist.
/// This takes O(depth) space, and does not repeatedly walk from the root.
struct BalanceCursor {
    tree: Subtree,
    parents: Vec<(Subtree, usize)>,
}

impl BalanceCursor {
    fn is_unique_branch(tree: &Subtree) -> bool {
        matches!(tree, Subtree::Heap(data) if !data.children.is_empty() && Arc::strong_count(data) == 1)
    }

    fn children_mut(tree: &mut Subtree) -> &mut Vec<Subtree> {
        let Subtree::Heap(data) = tree else {
            unreachable!("only heap branches are balanced");
        };
        &mut Arc::get_mut(data)
            .expect("balancing only visits uniquely owned branches")
            .children
    }

    fn descend(&mut self, index: usize) {
        let child = std::mem::take(&mut Self::children_mut(&mut self.tree)[index]);
        let parent = std::mem::replace(&mut self.tree, child);
        self.parents.push((parent, index));
    }

    fn ascend(&mut self) -> Option<usize> {
        let (mut parent, index) = self.parents.pop()?;
        Self::children_mut(&mut parent)[index] = std::mem::take(&mut self.tree);
        self.tree = parent;
        Some(index)
    }

    fn advance(&mut self) -> bool {
        let mut end = ts_subtree_children(&self.tree).len();
        loop {
            if let Some(index) = ts_subtree_children(&self.tree)[..end]
                .iter()
                .rposition(Self::is_unique_branch)
            {
                self.descend(index);
                return true;
            }
            // This node is done; continue at its preceding sibling, without
            // reprocessing its parent or adding any progress checks on ascent.
            let Some(index) = self.ascend() else {
                return false;
            };
            end = index;
        }
    }

    fn save(mut self, parser: &mut Parser) {
        parser
            .balance_path
            .extend(self.parents.iter().map(|(_, index)| *index));
        while self.ascend().is_some() {}
        parser.finished_tree = self.tree;
    }
}

#[cfg(test)]
mod parser3_tests {
    use super::*;

    // A progress/balancing-only fixture, so these tests do not depend on other
    // translation units' constructors or require a generated grammar.
    fn parser() -> Parser {
        Parser {
            lexer: LexerState::default(),
            stack: Stack {
                heads: Vec::new(),
                slices: Vec::new(),
                iterators: Vec::new(),
                arena: StackArena::default(),
                base_node: StackNodeId(0),
            },
            tree_pool: SubtreePool::default(),
            language: None,
            reduce_actions: Vec::new(),
            finished_tree: Subtree::Null,
            trailing_extras: Vec::new(),
            trailing_extras2: Vec::new(),
            scratch_trees: Vec::new(),
            token_cache: TokenCache::default(),
            reusable_node: ReusableNode::default(),
            external_scanner: None,
            scanner_buffer: [0; SERIALIZATION_BUFFER_SIZE],
            logger: None,
            dot_graph: None,
            end_clock: clock_null(),
            timeout_duration: 0,
            accept_count: 0,
            operation_count: 0,
            cancellation_flag: None,
            old_tree: Subtree::Null,
            included_range_differences: Vec::new(),
            parse_state: ParseState::default(),
            included_range_difference_index: 0,
            has_scanner_error: false,
            canceled_balancing: false,
            has_error: false,
            balance_path: Vec::new(),
        }
    }

    fn branch(symbol: Symbol, children: Vec<Subtree>) -> Subtree {
        Subtree::Heap(Arc::new(SubtreeHeapData {
            symbol,
            children,
            payload: SubtreePayload::Branch(BranchData::default()),
            ..SubtreeHeapData::default()
        }))
    }

    #[test]
    fn progress_checkpoints_and_state_updates() {
        let mut parser = parser();
        let mut input = SliceInput {
            bytes: b"",
            chunk_start: 0,
        };
        let mut observed = Vec::new();
        {
            let mut callback = |state: &ParseState| {
                observed.push((state.current_byte_offset, state.has_error));
                false
            };
            let mut context = ParseContext {
                input: &mut input,
                options: ParseOptions {
                    progress_callback: Some(&mut callback),
                },
            };
            for position in 0..99 {
                assert!(ts_parser__check_progress(
                    &mut parser,
                    &mut context,
                    None,
                    Some(position),
                    1
                ));
            }
            assert_eq!(parser.operation_count, 99);
            assert_eq!(parser.parse_state.current_byte_offset, 98);
            parser.has_error = true;
            assert!(ts_parser__check_progress(
                &mut parser,
                &mut context,
                None,
                Some(99),
                1
            ));
            assert_eq!(parser.operation_count, 0);
            parser.has_error = false;
            // Balancing keeps both parse-state fields from the last position
            // update. A large increment resets rather than retaining a remainder.
            assert!(ts_parser__check_progress(
                &mut parser,
                &mut context,
                None,
                None,
                150
            ));
            assert_eq!(parser.operation_count, 0);
            assert!(ts_parser__check_progress(
                &mut parser,
                &mut context,
                None,
                None,
                0
            ));
        }
        assert_eq!(observed, [(99, true), (99, true), (99, true)]);
    }

    #[test]
    fn cancellation_checks_short_circuit_in_c_order() {
        let mut parser = parser();
        parser.operation_count = 99;
        let flag = Arc::new(AtomicUsize::new(1));
        parser.cancellation_flag = Some(flag.clone());
        let mut input = SliceInput {
            bytes: b"",
            chunk_start: 0,
        };
        let mut calls = 0;
        {
            let mut callback = |_: &ParseState| {
                calls += 1;
                true
            };
            let mut context = ParseContext {
                input: &mut input,
                options: ParseOptions {
                    progress_callback: Some(&mut callback),
                },
            };
            let mut lookahead = Subtree::Null;
            assert!(!ts_parser__check_progress(
                &mut parser,
                &mut context,
                Some(&mut lookahead),
                Some(12),
                1
            ));
            assert_eq!(parser.parse_state.current_byte_offset, 12);
            flag.store(0, std::sync::atomic::Ordering::Relaxed);
            parser.end_clock = Some(std::time::Instant::now() - std::time::Duration::from_secs(1));
            assert!(!ts_parser__check_progress(
                &mut parser,
                &mut context,
                None,
                None,
                100
            ));
            parser.end_clock = clock_null();
            assert!(!ts_parser__check_progress(
                &mut parser,
                &mut context,
                None,
                None,
                100
            ));
        }
        assert_eq!(calls, 1);
    }

    #[test]
    fn balancing_cursor_preserves_ownership_and_lifo_order() {
        let shared = branch(7, vec![Subtree::Inline(InlineLeaf::default())]);
        let tree = branch(
            1,
            vec![
                branch(2, vec![Subtree::Inline(InlineLeaf::default())]),
                shared.clone(),
                branch(
                    3,
                    vec![branch(4, vec![Subtree::Inline(InlineLeaf::default())])],
                ),
            ],
        );
        let root_address = Arc::as_ptr(match &tree {
            Subtree::Heap(data) => data,
            _ => unreachable!(),
        });
        let mut cursor = BalanceCursor {
            tree,
            parents: Vec::new(),
        };
        let mut symbols = Vec::new();
        loop {
            assert!(BalanceCursor::is_unique_branch(&cursor.tree));
            symbols.push(ts_subtree_symbol(&cursor.tree));
            if !cursor.advance() {
                break;
            }
        }
        assert_eq!(symbols, [1, 3, 4, 2]);
        assert!(cursor.parents.is_empty());
        let Subtree::Heap(root) = &cursor.tree else {
            unreachable!()
        };
        assert_eq!(Arc::as_ptr(root), root_address);
        assert!(root.children[1].ptr_eq(&shared));
        assert_eq!(ts_subtree_symbol(&root.children[0]), 2);
        assert_eq!(ts_subtree_symbol(&root.children[2]), 3);
    }

    #[test]
    fn canceled_balancing_restores_tree_and_resumes_at_same_node() {
        let mut parser = parser();
        let mut tree = Subtree::Inline(InlineLeaf::default());
        for symbol in 1..=250 {
            tree = branch(symbol, vec![tree]);
        }
        parser.finished_tree = tree;
        let mut input = SliceInput {
            bytes: b"",
            chunk_start: 0,
        };
        let mut calls = 0;
        {
            let mut cancel_once = |_: &ParseState| {
                calls += 1;
                calls == 1
            };
            let mut context = ParseContext {
                input: &mut input,
                options: ParseOptions {
                    progress_callback: Some(&mut cancel_once),
                },
            };
            assert!(!ts_parser__balance_subtree(&mut parser, &mut context));
            assert_eq!(parser.balance_path.len(), 99);
            assert_eq!(ts_subtree_symbol(&parser.finished_tree), 250);
            // These updates are made by ts_parser_parse on cancellation/resume.
            parser.canceled_balancing = true;
            parser.operation_count = 0;
            assert!(ts_parser__balance_subtree(&mut parser, &mut context));
            assert!(parser.balance_path.is_empty());
            assert_eq!(parser.operation_count, 51);
        }
        assert_eq!(calls, 2);
        let mut tree = &parser.finished_tree;
        for symbol in (1..=250).rev() {
            assert_eq!(ts_subtree_symbol(tree), symbol);
            assert!(BalanceCursor::is_unique_branch(tree));
            tree = &ts_subtree_children(tree)[0];
        }
        assert!(matches!(tree, Subtree::Inline(_)));
    }

    #[test]
    fn shared_root_is_not_balanced_or_counted() {
        let root = branch(
            1,
            vec![branch(2, vec![Subtree::Inline(InlineLeaf::default())])],
        );
        let mut parser = parser();
        parser.finished_tree = root.clone();
        let mut input = SliceInput {
            bytes: b"",
            chunk_start: 0,
        };
        let mut context = ParseContext {
            input: &mut input,
            options: ParseOptions::default(),
        };
        assert!(ts_parser__balance_subtree(&mut parser, &mut context));
        assert_eq!(parser.operation_count, 0);
        assert!(parser.finished_tree.ptr_eq(&root));
    }
}
