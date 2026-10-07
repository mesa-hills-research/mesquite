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
    todo!("parser-3: ts_parser__handle_error")
}

pub(crate) fn ts_parser__check_progress(
    parser: &mut Parser,
    context: &mut ParseContext<'_, '_>,
    lookahead: Option<&mut Subtree>,
    position: Option<u32>,
    operations: u32,
) -> bool {
    todo!("parser-3: ts_parser__check_progress")
}

pub(crate) fn ts_parser__advance(
    parser: &mut Parser,
    context: &mut ParseContext<'_, '_>,
    version: StackVersion,
    allow_node_reuse: bool,
) -> bool {
    todo!("parser-3: ts_parser__advance")
}

pub(crate) fn ts_parser__condense_stack(parser: &mut Parser) -> u32 {
    todo!("parser-3: ts_parser__condense_stack")
}

pub(crate) fn ts_parser__balance_subtree(
    parser: &mut Parser,
    context: &mut ParseContext<'_, '_>,
) -> bool {
    todo!("parser-3: ts_parser__balance_subtree")
}

pub(crate) fn ts_parser_has_outstanding_parse(parser: &Parser) -> bool {
    todo!("parser-3: ts_parser_has_outstanding_parse")
}

pub(crate) fn ts_parser_new() -> Parser {
    todo!("parser-3: ts_parser_new")
}

pub(crate) fn ts_parser_delete(parser: &mut Parser) {
    todo!("parser-3: ts_parser_delete")
}

pub(crate) fn ts_parser_language(parser: &Parser) -> Option<&Language> {
    todo!("parser-3: ts_parser_language")
}

pub(crate) fn ts_parser_set_language(parser: &mut Parser, language: Option<&Language>) -> bool {
    todo!("parser-3: ts_parser_set_language")
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
