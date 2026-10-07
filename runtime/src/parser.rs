use crate::{
    api::{LogType, ParseOptions, ParseState},
    clock::*,
    error_costs::{ERROR_COST_PER_SKIPPED_TREE, ERROR_STATE},
    language::*,
    length::*,
    lexer::*,
    reduce_action::*,
    reusable_node::*,
    stack::*,
    subtree::*,
    tree::Tree,
    types::*,
};
use std::sync::{Arc, atomic::AtomicUsize};
use ts_port_tables::{ExternalScanner, LexMode, SERIALIZATION_BUFFER_SIZE};
#[cfg(test)]
#[path = "parser_tests_1.rs"]
mod parser1_tests;
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

// These mirror parser.c's logging macros. Format only when an output is enabled,
// and finish formatting before mutably borrowing the parser for the callback.
macro_rules! parser_log {
    ($parser:expr, $($arg:tt)*) => {
        if $parser.logger.is_some() || $parser.dot_graph.is_some() {
            $parser.lexer.debug_buffer = parser_log_buffer(format!($($arg)*));
            ts_parser__log($parser);
        }
    };
}

macro_rules! parser_log_lookahead {
    ($parser:expr, $symbol:expr, $size:expr) => {
        if $parser.logger.is_some() || $parser.dot_graph.is_some() {
            let name = parser_escape_symbol($symbol);
            parser_log!($parser, "lexed_lookahead sym:{}, size:{}", name, $size);
        }
    };
}

macro_rules! parser_log_stack {
    ($parser:expr) => {
        if let Some(output) = $parser.dot_graph.as_mut() {
            let _ = ts_stack_print_dot_graph(
                &$parser.stack,
                $parser.language.as_ref().expect("parser language"),
                output.as_mut(),
            );
            let _ = output.write_all(b"\n\n");
        }
    };
}

macro_rules! parser_log_tree {
    ($parser:expr, $tree:expr) => {
        if let Some(output) = $parser.dot_graph.as_mut() {
            let _ = ts_subtree_print_dot_graph(
                $tree,
                $parser.language.as_ref().expect("parser language"),
                output.as_mut(),
            );
            let _ = output.write_all(b"\n");
        }
    };
}

fn parser_log_buffer(mut message: String) -> String {
    let mut end = message
        .find('\0')
        .unwrap_or(message.len())
        .min(SERIALIZATION_BUFFER_SIZE - 1);
    // Unlike C's char buffer, the logger's &str must end on a UTF-8 boundary.
    while !message.is_char_boundary(end) {
        end -= 1;
    }
    message.truncate(end);
    message
}

fn parser_escape_symbol(symbol: &str) -> String {
    let mut result = String::new();
    for ch in symbol.chars().take_while(|ch| *ch != '\0') {
        match ch {
            '\t' => result.push_str("\\t"),
            '\n' => result.push_str("\\n"),
            '\u{b}' => result.push_str("\\v"),
            '\u{c}' => result.push_str("\\f"),
            '\r' => result.push_str("\\r"),
            '\\' => result.push_str("\\\\"),
            _ => result.push(ch),
        }
    }
    result
}

fn parser_lexer<'a>(parser: &'a mut Parser, context: &'a mut ParseContext<'_, '_>) -> Lexer<'a> {
    Lexer {
        state: &mut parser.lexer,
        input: &mut *context.input,
        logger: parser.logger.as_mut(),
    }
}

pub(crate) fn ts_string_input_read(input: &[u8], byte: u32, _point: Point) -> &[u8] {
    input.get(byte as usize..).unwrap_or(&[])
}

pub(crate) fn ts_parser__log(parser: &mut Parser) {
    let message = &parser.lexer.debug_buffer;
    if let Some(logger) = parser.logger.as_mut() {
        logger(LogType::Parse, message);
    }
    if let Some(output) = parser.dot_graph.as_mut() {
        let _ = output.write_all(b"graph {\nlabel=\"");
        for byte in message.bytes() {
            if byte == b'"' || byte == b'\\' {
                let _ = output.write_all(b"\\");
            }
            let _ = output.write_all(&[byte]);
        }
        let _ = output.write_all(b"\"\n}\n\n");
    }
}

pub(crate) fn ts_parser__breakdown_top_of_stack(
    parser: &mut Parser,
    version: StackVersion,
) -> bool {
    let language = parser.language.expect("parser language");
    let mut did_break_down = false;
    loop {
        let pop = ts_stack_pop_pending(&mut parser.stack, &mut parser.tree_pool, version);
        if pop.is_empty() {
            break;
        }
        did_break_down = true;
        let mut pending = false;
        for slice in pop {
            let mut state = ts_stack_state(&parser.stack, slice.version);
            let mut subtrees = slice.subtrees.into_iter();
            let parent = subtrees.next().expect("pending subtree");
            let symbol = ts_subtree_symbol(&parent);
            for child in ts_subtree_children(&parent) {
                pending = ts_subtree_child_count(child) > 0;
                if ts_subtree_is_error(child) {
                    state = ERROR_STATE;
                } else if !ts_subtree_extra(child) {
                    state = ts_language_next_state(&language, state, ts_subtree_symbol(child));
                }
                ts_stack_push(
                    &mut parser.stack,
                    &mut parser.tree_pool,
                    slice.version,
                    child.clone(),
                    pending,
                    state,
                );
            }
            for tree in subtrees {
                ts_stack_push(
                    &mut parser.stack,
                    &mut parser.tree_pool,
                    slice.version,
                    tree,
                    false,
                    state,
                );
            }
            ts_subtree_release(&mut parser.tree_pool, parent);
            parser_log!(
                parser,
                "breakdown_top_of_stack tree:{}",
                ts_language_symbol_name(&language, symbol).unwrap_or("")
            );
            parser_log_stack!(parser);
        }
        if !pending {
            break;
        }
    }
    did_break_down
}

pub(crate) fn ts_parser__breakdown_lookahead(
    parser: &mut Parser,
    lookahead: &mut Subtree,
    state: StateId,
) {
    let mut did_descend = false;
    let mut tree = reusable_node_tree(&parser.reusable_node);
    while ts_subtree_child_count(&tree) > 0 && ts_subtree_parse_state(&tree) != state {
        parser_log!(
            parser,
            "state_mismatch sym:{}",
            ts_language_symbol_name(
                parser.language.as_ref().expect("parser language"),
                ts_subtree_symbol(&tree)
            )
            .unwrap_or("")
        );
        reusable_node_descend(&mut parser.reusable_node);
        tree = reusable_node_tree(&parser.reusable_node);
        did_descend = true;
    }
    if did_descend {
        ts_subtree_release(&mut parser.tree_pool, std::mem::replace(lookahead, tree));
    }
}

pub(crate) fn ts_parser__compare_versions(
    _parser: &Parser,
    a: ErrorStatus,
    b: ErrorStatus,
) -> ErrorComparison {
    compare_error_statuses(a, b)
}

fn compare_error_statuses(a: ErrorStatus, b: ErrorStatus) -> ErrorComparison {
    use ErrorComparison::*;
    if !a.is_in_error && b.is_in_error {
        return if a.cost < b.cost {
            TakeLeft
        } else {
            PreferLeft
        };
    }
    if a.is_in_error && !b.is_in_error {
        return if b.cost < a.cost {
            TakeRight
        } else {
            PreferRight
        };
    }
    if a.cost < b.cost {
        return if (b.cost - a.cost).wrapping_mul(a.node_count.wrapping_add(1)) > MAX_COST_DIFFERENCE
        {
            TakeLeft
        } else {
            PreferLeft
        };
    }
    if b.cost < a.cost {
        return if (a.cost - b.cost).wrapping_mul(b.node_count.wrapping_add(1)) > MAX_COST_DIFFERENCE
        {
            TakeRight
        } else {
            PreferRight
        };
    }
    if a.dynamic_precedence > b.dynamic_precedence {
        PreferLeft
    } else if b.dynamic_precedence > a.dynamic_precedence {
        PreferRight
    } else {
        None
    }
}

pub(crate) fn ts_parser__version_status(parser: &Parser, version: StackVersion) -> ErrorStatus {
    let mut cost = ts_stack_error_cost(&parser.stack, version);
    let is_paused = ts_stack_is_paused(&parser.stack, version);
    if is_paused {
        cost = cost.wrapping_add(ERROR_COST_PER_SKIPPED_TREE);
    }
    ErrorStatus {
        cost,
        node_count: ts_stack_node_count_since_error(&parser.stack, version),
        dynamic_precedence: ts_stack_dynamic_precedence(&parser.stack, version),
        is_in_error: is_paused || ts_stack_state(&parser.stack, version) == ERROR_STATE,
    }
}

pub(crate) fn ts_parser__better_version_exists(
    parser: &Parser,
    version: StackVersion,
    is_in_error: bool,
    cost: u32,
) -> bool {
    if !parser.finished_tree.is_null() && ts_subtree_error_cost(&parser.finished_tree) <= cost {
        return true;
    }
    let position = ts_stack_position(&parser.stack, version);
    let status = ErrorStatus {
        cost,
        is_in_error,
        dynamic_precedence: ts_stack_dynamic_precedence(&parser.stack, version),
        node_count: ts_stack_node_count_since_error(&parser.stack, version),
    };
    for i in 0..ts_stack_version_count(&parser.stack) {
        if i == version
            || !ts_stack_is_active(&parser.stack, i)
            || ts_stack_position(&parser.stack, i).bytes < position.bytes
        {
            continue;
        }
        let status_i = ts_parser__version_status(parser, i);
        match ts_parser__compare_versions(parser, status, status_i) {
            ErrorComparison::TakeRight => return true,
            ErrorComparison::PreferRight if ts_stack_can_merge(&parser.stack, i, version) => {
                return true;
            }
            _ => {}
        }
    }
    false
}

pub(crate) fn ts_parser__call_main_lex_fn(
    parser: &mut Parser,
    context: &mut ParseContext<'_, '_>,
    lex_mode: LexMode,
) -> bool {
    let lex_fn = parser.language.expect("parser language").tables.lex_fn;
    lex_fn(&mut parser_lexer(parser, context), lex_mode.lex_state)
}

pub(crate) fn ts_parser__call_keyword_lex_fn(
    parser: &mut Parser,
    context: &mut ParseContext<'_, '_>,
) -> bool {
    let lex_fn = parser
        .language
        .expect("parser language")
        .tables
        .keyword_lex_fn
        .expect("keyword capture token requires a keyword lexer");
    lex_fn(&mut parser_lexer(parser, context), 0)
}

pub(crate) fn ts_parser__external_scanner_create(parser: &mut Parser) {
    if let Some(language) = parser.language
        && let Some(scanner) = language.tables.external_scanner.as_ref()
    {
        parser.external_scanner = Some((scanner.create)());
    }
}

pub(crate) fn ts_parser__external_scanner_destroy(parser: &mut Parser) {
    parser.external_scanner = None;
}

pub(crate) fn ts_parser__external_scanner_serialize(parser: &mut Parser) -> u32 {
    let length = parser
        .external_scanner
        .as_mut()
        .expect("external scanner")
        .serialize(&mut parser.scanner_buffer);
    assert!(length <= SERIALIZATION_BUFFER_SIZE);
    length as u32
}

pub(crate) fn ts_parser__external_scanner_deserialize(
    parser: &mut Parser,
    external_token: &Subtree,
) {
    let data = ts_subtree_external_scanner_state(external_token)
        .map_or(&[][..], ts_external_scanner_state_data);
    parser
        .external_scanner
        .as_mut()
        .expect("external scanner")
        .deserialize(data);
}

pub(crate) fn ts_parser__external_scanner_scan(
    parser: &mut Parser,
    context: &mut ParseContext<'_, '_>,
    external_lex_state: StateId,
) -> bool {
    let valid_tokens = ts_language_enabled_external_tokens(
        parser.language.as_ref().expect("parser language"),
        u32::from(external_lex_state),
    )
    .expect("external lexer state");
    let mut lexer = Lexer {
        state: &mut parser.lexer,
        input: &mut *context.input,
        logger: parser.logger.as_mut(),
    };
    parser
        .external_scanner
        .as_mut()
        .expect("external scanner")
        .scan(&mut lexer, valid_tokens)
}

pub(crate) fn ts_parser__can_reuse_first_leaf(
    parser: &Parser,
    state: StateId,
    tree: &Subtree,
    table_entry: &mut TableEntry,
) -> bool {
    let language = parser.language.as_ref().expect("parser language");
    let leaf_symbol = ts_subtree_leaf_symbol(tree);
    let leaf_state = ts_subtree_leaf_parse_state(tree);
    let current_lex_mode = ts_language_lex_mode_for_state(language, state);
    let leaf_lex_mode = ts_language_lex_mode_for_state(language, leaf_state);

    // A nonterminal extra ends by reducing on a null lookahead, not by lexing.
    if current_lex_mode.lex_state == StateId::MAX {
        return false;
    }
    if !table_entry.actions.is_empty()
        && leaf_lex_mode == current_lex_mode
        && (leaf_symbol != language.tables.keyword_capture_token
            || (!ts_subtree_is_keyword(tree) && ts_subtree_parse_state(tree) == state))
    {
        return true;
    }
    // Empty tokens cannot be reused with different lookaheads (except EOF).
    if ts_subtree_size(tree).bytes == 0 && leaf_symbol != ts_port_tables::BUILTIN_SYM_END {
        return false;
    }
    current_lex_mode.external_lex_state == 0 && table_entry.is_reusable
}

pub(crate) fn ts_parser__lex(
    parser: &mut Parser,
    context: &mut ParseContext<'_, '_>,
    version: StackVersion,
    parse_state: StateId,
) -> Subtree {
    let language = parser.language.expect("parser language");
    let mut lex_mode = ts_language_lex_mode_for_state(&language, parse_state);
    if lex_mode.lex_state == StateId::MAX {
        parser_log!(parser, "no_lookahead_after_non_terminal_extra");
        return Subtree::Null;
    }
    let start_position = ts_stack_position(&parser.stack, version);
    // Retain the handle across mutable parser calls, not a borrow of its stack.
    let external_token = ts_stack_last_external_token(&parser.stack, version).clone();
    let mut found_external_token = false;
    let mut error_mode = parse_state == ERROR_STATE;
    let mut skipped_error = false;
    let mut called_get_column = false;
    let mut first_error_character = 0;
    let mut error_start_position = length_zero();
    let mut error_end_position = length_zero();
    let mut lookahead_end_byte = 0;
    let mut external_scanner_state_len = 0;
    let mut external_scanner_state_changed = false;
    ts_lexer_reset(&mut parser.lexer, start_position);

    loop {
        let current_position = parser.lexer.current_position;
        let column_data = parser.lexer.column_data;
        if lex_mode.external_lex_state != 0 {
            parser_log!(
                parser,
                "lex_external state:{}, row:{}, column:{}",
                lex_mode.external_lex_state,
                current_position.extent.row,
                current_position.extent.column
            );
            ts_lexer_start(&mut parser_lexer(parser, context));
            ts_parser__external_scanner_deserialize(parser, &external_token);
            let mut found_token =
                ts_parser__external_scanner_scan(parser, context, lex_mode.external_lex_state);
            if parser.has_scanner_error {
                return Subtree::Null;
            }
            ts_lexer_finish(&mut parser.lexer, &mut lookahead_end_byte);
            if found_token {
                external_scanner_state_len = ts_parser__external_scanner_serialize(parser);
                let bytes = &parser.scanner_buffer[..external_scanner_state_len as usize];
                external_scanner_state_changed =
                    !ts_subtree_external_scanner_state(&external_token)
                        .map_or(bytes.is_empty(), |state| {
                            ts_external_scanner_state_eq(state, bytes)
                        });
                // Empty external tokens are allowed only if they change scanner state,
                // or can advance parsing outside error recovery without being extras.
                if parser.lexer.token_end_position.bytes <= current_position.bytes
                    && !external_scanner_state_changed
                {
                    let symbol = language
                        .tables
                        .external_scanner
                        .as_ref()
                        .expect("external scanner tables")
                        .symbol_map[parser.lexer.result_symbol as usize];
                    let next_parse_state = ts_language_next_state(&language, parse_state, symbol);
                    let token_is_extra = next_parse_state == parse_state;
                    if error_mode
                        || !ts_stack_has_advanced_since_error(&parser.stack, version)
                        || token_is_extra
                    {
                        parser_log!(
                            parser,
                            "ignore_empty_external_token symbol:{}",
                            ts_language_symbol_name(&language, symbol).unwrap_or("")
                        );
                        found_token = false;
                    }
                }
            }
            if found_token {
                found_external_token = true;
                called_get_column = parser.lexer.did_get_column;
                break;
            }
            ts_lexer_reset(&mut parser.lexer, current_position);
            parser.lexer.column_data = column_data;
        }
        parser_log!(
            parser,
            "lex_internal state:{}, row:{}, column:{}",
            lex_mode.lex_state,
            current_position.extent.row,
            current_position.extent.column
        );
        ts_lexer_start(&mut parser_lexer(parser, context));
        let found_token = ts_parser__call_main_lex_fn(parser, context, lex_mode);
        ts_lexer_finish(&mut parser.lexer, &mut lookahead_end_byte);
        if found_token {
            break;
        }
        if !error_mode {
            error_mode = true;
            lex_mode = ts_language_lex_mode_for_state(&language, ERROR_STATE);
            ts_lexer_reset(&mut parser.lexer, start_position);
            continue;
        }
        if !skipped_error {
            parser_log!(parser, "skip_unrecognized_character");
            skipped_error = true;
            error_start_position = parser.lexer.token_start_position;
            error_end_position = parser.lexer.token_start_position;
            first_error_character = parser.lexer.lookahead;
        }
        if parser.lexer.current_position.bytes == error_end_position.bytes {
            if ts_lexer__eof(&parser.lexer) {
                parser.lexer.result_symbol = ts_port_tables::BUILTIN_SYM_ERROR;
                break;
            }
            ts_lexer__advance(&mut parser_lexer(parser, context), false);
        }
        error_end_position = parser.lexer.current_position;
    }

    let result = if skipped_error {
        let padding = length_sub(error_start_position, start_position);
        let size = length_sub(error_end_position, error_start_position);
        let lookahead_bytes = lookahead_end_byte.wrapping_sub(error_end_position.bytes);
        ts_subtree_new_error(
            &mut parser.tree_pool,
            first_error_character,
            padding,
            size,
            lookahead_bytes,
            parse_state,
            &language,
        )
    } else {
        let mut is_keyword = false;
        let mut symbol = parser.lexer.result_symbol;
        let padding = length_sub(parser.lexer.token_start_position, start_position);
        let size = length_sub(
            parser.lexer.token_end_position,
            parser.lexer.token_start_position,
        );
        let lookahead_bytes =
            lookahead_end_byte.wrapping_sub(parser.lexer.token_end_position.bytes);
        if found_external_token {
            symbol = language
                .tables
                .external_scanner
                .as_ref()
                .expect("external scanner tables")
                .symbol_map[symbol as usize];
        } else if symbol == language.tables.keyword_capture_token && symbol != 0 {
            let end_byte = parser.lexer.token_end_position.bytes;
            let token_start = parser.lexer.token_start_position;
            ts_lexer_reset(&mut parser.lexer, token_start);
            ts_lexer_start(&mut parser_lexer(parser, context));
            is_keyword = ts_parser__call_keyword_lex_fn(parser, context);
            if is_keyword
                && parser.lexer.token_end_position.bytes == end_byte
                && (ts_language_has_actions(&language, parse_state, parser.lexer.result_symbol)
                    || ts_language_is_reserved_word(
                        &language,
                        parse_state,
                        parser.lexer.result_symbol,
                    ))
            {
                symbol = parser.lexer.result_symbol;
            }
        }
        let mut result = ts_subtree_new_leaf(
            &mut parser.tree_pool,
            symbol,
            padding,
            size,
            lookahead_bytes,
            parse_state,
            found_external_token,
            called_get_column,
            is_keyword,
            &language,
        );
        if found_external_token {
            let data = result
                .heap_mut()
                .expect("external tokens are heap subtrees");
            data.payload = SubtreePayload::External(ts_external_scanner_state_init(
                &parser.scanner_buffer[..external_scanner_state_len as usize],
            ));
            data.has_external_scanner_state_change = external_scanner_state_changed;
        }
        result
    };
    parser_log_lookahead!(
        parser,
        ts_language_symbol_name(&language, ts_subtree_symbol(&result)).unwrap_or(""),
        ts_subtree_total_size(&result).bytes
    );
    result
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
