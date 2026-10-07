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
    pub(crate) parse_table_cache: ParseTableCache,
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

// Evaluate diagnostic arguments lazily, including symbol and stack lookups.
// Building fmt::Arguments at a function call evaluates them even with no logger.
macro_rules! parser3_log {
    ($parser:expr, $message:expr $(,)?) => {
        if $parser.logger.is_some() || $parser.dot_graph.is_some() {
            parser3_log_message($parser, $message);
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
                &mut $parser.stack,
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
                    state = parser.parse_table_cache.next_state(&language, state, ts_subtree_symbol(child));
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

pub(crate) fn ts_parser__version_status(parser: &mut Parser, version: StackVersion) -> ErrorStatus {
    let mut cost = ts_stack_error_cost(&parser.stack, version);
    let is_paused = ts_stack_is_paused(&parser.stack, version);
    if is_paused {
        cost = cost.wrapping_add(ERROR_COST_PER_SKIPPED_TREE);
    }
    ErrorStatus {
        cost,
        node_count: ts_stack_node_count_since_error(&mut parser.stack, version),
        dynamic_precedence: ts_stack_dynamic_precedence(&parser.stack, version),
        is_in_error: is_paused || ts_stack_state(&parser.stack, version) == ERROR_STATE,
    }
}

pub(crate) fn ts_parser__better_version_exists(
    parser: &mut Parser,
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
        node_count: ts_stack_node_count_since_error(&mut parser.stack, version),
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
    scanner: &mut dyn ExternalScanner,
    external_token: &Subtree,
) {
    let data = ts_subtree_external_scanner_state(external_token)
        .map_or(&[][..], ts_external_scanner_state_data);
    scanner.deserialize(data);
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
    if !table_entry.actions().is_empty()
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
    current_lex_mode.external_lex_state == 0 && table_entry.is_reusable()
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
            // Lexing does not mutate the stack. Borrow the scanner snapshot
            // from its head instead of retaining/releasing an Arc per token.
            ts_parser__external_scanner_deserialize(
                parser
                    .external_scanner
                    .as_deref_mut()
                    .expect("external scanner"),
                ts_stack_last_external_token(&parser.stack, version),
            );
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
                    !ts_subtree_external_scanner_state(ts_stack_last_external_token(
                        &parser.stack,
                        version,
                    ))
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
                    let next_parse_state = parser.parse_table_cache.next_state(&language, parse_state, symbol);
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
                && (parser.parse_table_cache.has_actions(&language, parse_state, parser.lexer.result_symbol)
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

// This unit's diagnostics use the same buffer and logger as the parser's other
// phases. Keep formatting lazy: normal parsing does not allocate log messages.
macro_rules! parser2_log {
    ($parser:expr, $($args:tt)*) => {
        if $parser.logger.is_some() || $parser.dot_graph.is_some() {
            $parser.lexer.debug_buffer = format!($($args)*);
            ts_parser__log($parser);
        }
    };
}

fn parser2_tree_name(parser: &Parser, tree: &Subtree) -> &'static str {
    ts_language_symbol_name(parser.language.as_ref().unwrap(), ts_subtree_symbol(tree))
        .unwrap_or("")
}

fn parser2_log_stack(parser: &mut Parser) {
    if let Some(output) = parser.dot_graph.as_mut() {
        let _ = ts_stack_print_dot_graph(
            &mut parser.stack,
            parser.language.as_ref().unwrap(),
            output.as_mut(),
        );
        let _ = output.write_all(b"\n\n");
    }
}

pub(crate) fn ts_parser__get_cached_token(
    parser: &Parser,
    state: StateId,
    position: usize,
    last_external_token: &Subtree,
    table_entry: &mut TableEntry,
) -> Subtree {
    let cache = &parser.token_cache;
    if !cache.token.is_null()
        && cache.byte_index as usize == position
        && ts_subtree_external_scanner_state_eq(&cache.last_external_token, last_external_token)
    {
        *table_entry = parser.parse_table_cache.table_entry(
            parser.language.as_ref().unwrap(),
            state,
            ts_subtree_symbol(&cache.token),
        );
        if ts_parser__can_reuse_first_leaf(parser, state, &cache.token, table_entry) {
            return cache.token.clone();
        }
    }
    Subtree::Null
}

pub(crate) fn ts_parser__set_cached_token(
    parser: &mut Parser,
    byte_index: u32,
    last_external_token: Subtree,
    token: Subtree,
) {
    // Arguments transfer owning references; callers retain the lookahead and
    // scanner token when they still need them, instead of retaining again here.
    let old_token = std::mem::replace(&mut parser.token_cache.token, token);
    let old_external = std::mem::replace(
        &mut parser.token_cache.last_external_token,
        last_external_token,
    );
    if !old_token.is_null() {
        ts_subtree_release(&mut parser.tree_pool, old_token);
    }
    if !old_external.is_null() {
        ts_subtree_release(&mut parser.tree_pool, old_external);
    }
    parser.token_cache.byte_index = byte_index;
}

// A scanner snapshot normally survives many internal tokens. Replacing a cache
// entry with that very same handle would clone and release it unnecessarily.
#[inline]
fn cache_token_for_version(
    parser: &mut Parser,
    version: StackVersion,
    byte_index: u32,
    token: Subtree,
) {
    let last_external = ts_stack_last_external_token(&parser.stack, version);
    if parser.token_cache.last_external_token.ptr_eq(last_external) {
        let old_token = std::mem::replace(&mut parser.token_cache.token, token);
        if !old_token.is_null() {
            ts_subtree_release(&mut parser.tree_pool, old_token);
        }
        parser.token_cache.byte_index = byte_index;
    } else {
        let last_external = last_external.clone();
        ts_parser__set_cached_token(parser, byte_index, last_external, token);
    }
}

pub(crate) fn ts_parser__has_included_range_difference(
    parser: &Parser,
    start_position: u32,
    end_position: u32,
) -> bool {
    crate::get_changed_ranges::ts_range_array_intersects(
        &parser.included_range_differences,
        parser.included_range_difference_index,
        start_position,
        end_position,
    )
}

pub(crate) fn ts_parser__reuse_node(
    parser: &mut Parser,
    version: StackVersion,
    state: &mut StateId,
    position: u32,
    table_entry: &mut TableEntry,
) -> Subtree {
    use crate::reusable_node::*;

    loop {
        let result = reusable_node_tree(&parser.reusable_node);
        if result.is_null() {
            break;
        }
        let byte_offset = reusable_node_byte_offset(&parser.reusable_node);
        let mut end_byte_offset = byte_offset.wrapping_add(ts_subtree_total_bytes(&result));

        // EOF must also account for included-range changes later in the file.
        if ts_subtree_is_eof(&result) {
            end_byte_offset = u32::MAX;
        }
        if byte_offset > position {
            parser2_log!(
                parser,
                "before_reusable_node symbol:{}",
                parser2_tree_name(parser, &result)
            );
            break;
        }
        if byte_offset < position {
            parser2_log!(
                parser,
                "past_reusable_node symbol:{}",
                parser2_tree_name(parser, &result)
            );
            if end_byte_offset <= position || !reusable_node_descend(&mut parser.reusable_node) {
                reusable_node_advance(&mut parser.reusable_node);
            }
            continue;
        }
        if !ts_subtree_external_scanner_state_eq(
            &parser.reusable_node.last_external_token,
            ts_stack_last_external_token(&parser.stack, version),
        ) {
            parser2_log!(
                parser,
                "reusable_node_has_different_external_scanner_state symbol:{}",
                parser2_tree_name(parser, &result)
            );
            reusable_node_advance(&mut parser.reusable_node);
            continue;
        }

        let reason = if ts_subtree_has_changes(&result) {
            Some("has_changes")
        } else if ts_subtree_is_error(&result) {
            Some("is_error")
        } else if ts_subtree_missing(&result) {
            Some("is_missing")
        } else if ts_subtree_is_fragile(&result) {
            Some("is_fragile")
        } else if ts_parser__has_included_range_difference(parser, byte_offset, end_byte_offset) {
            Some("contains_different_included_range")
        } else {
            None
        };
        if let Some(reason) = reason {
            parser2_log!(
                parser,
                "cant_reuse_node_{} tree:{}",
                reason,
                parser2_tree_name(parser, &result)
            );
            if !reusable_node_descend(&mut parser.reusable_node) {
                reusable_node_advance(&mut parser.reusable_node);
                ts_parser__breakdown_top_of_stack(parser, version);
                *state = ts_stack_state(&parser.stack, version);
            }
            continue;
        }

        let leaf_symbol = ts_subtree_leaf_symbol(&result);
        *table_entry =
            parser.parse_table_cache.table_entry(parser.language.as_ref().unwrap(), *state, leaf_symbol);
        if !ts_parser__can_reuse_first_leaf(parser, *state, &result, table_entry) {
            parser2_log!(
                parser,
                "cant_reuse_node symbol:{}, first_leaf_symbol:{}",
                parser2_tree_name(parser, &result),
                ts_language_symbol_name(parser.language.as_ref().unwrap(), leaf_symbol)
                    .unwrap_or("")
            );
            reusable_node_advance_past_leaf(&mut parser.reusable_node);
            break;
        }
        parser2_log!(
            parser,
            "reuse_node symbol:{}",
            parser2_tree_name(parser, &result)
        );
        // reusable_node_tree already returns a retained handle.
        return result;
    }
    Subtree::Null
}

pub(crate) fn ts_parser__select_tree(parser: &mut Parser, left: &Subtree, right: &Subtree) -> bool {
    if left.is_null() {
        return true;
    }
    if right.is_null() {
        return false;
    }
    if ts_subtree_error_cost(right) < ts_subtree_error_cost(left) {
        parser2_log!(
            parser,
            "select_smaller_error symbol:{}, over_symbol:{}",
            parser2_tree_name(parser, right),
            parser2_tree_name(parser, left)
        );
        return true;
    }
    if ts_subtree_error_cost(left) < ts_subtree_error_cost(right) {
        parser2_log!(
            parser,
            "select_smaller_error symbol:{}, over_symbol:{}",
            parser2_tree_name(parser, left),
            parser2_tree_name(parser, right)
        );
        return false;
    }
    if ts_subtree_dynamic_precedence(right) > ts_subtree_dynamic_precedence(left) {
        parser2_log!(
            parser,
            "select_higher_precedence symbol:{}, prec:{}, over_symbol:{}, other_prec:{}",
            parser2_tree_name(parser, right),
            ts_subtree_dynamic_precedence(right),
            parser2_tree_name(parser, left),
            ts_subtree_dynamic_precedence(left)
        );
        return true;
    }
    if ts_subtree_dynamic_precedence(left) > ts_subtree_dynamic_precedence(right) {
        parser2_log!(
            parser,
            "select_higher_precedence symbol:{}, prec:{}, over_symbol:{}, other_prec:{}",
            parser2_tree_name(parser, left),
            ts_subtree_dynamic_precedence(left),
            parser2_tree_name(parser, right),
            ts_subtree_dynamic_precedence(right)
        );
        return false;
    }
    if ts_subtree_error_cost(left) > 0 {
        return true;
    }
    match ts_subtree_compare(left, right, &mut parser.tree_pool) {
        -1 => {
            parser2_log!(
                parser,
                "select_earlier symbol:{}, over_symbol:{}",
                parser2_tree_name(parser, left),
                parser2_tree_name(parser, right)
            );
            false
        }
        1 => {
            parser2_log!(
                parser,
                "select_earlier symbol:{}, over_symbol:{}",
                parser2_tree_name(parser, right),
                parser2_tree_name(parser, left)
            );
            true
        }
        _ => {
            parser2_log!(
                parser,
                "select_existing symbol:{}, over_symbol:{}",
                parser2_tree_name(parser, left),
                parser2_tree_name(parser, right)
            );
            false
        }
    }
}

pub(crate) fn ts_parser__select_children(
    parser: &mut Parser,
    left: &Subtree,
    children: &[Subtree],
) -> bool {
    parser.scratch_trees.clear();
    parser.scratch_trees.extend_from_slice(children);
    let mut scratch_tree = ts_subtree_new_node(
        ts_subtree_symbol(left),
        std::mem::take(&mut parser.scratch_trees),
        0,
        parser.language.as_ref().unwrap(),
    );
    let result = ts_parser__select_tree(parser, left, &scratch_tree);
    // C builds a non-owning header at the end of its scratch array. Rust owns
    // these temporary child handles, then recovers the Vec allocation for reuse.
    parser.scratch_trees = std::mem::take(&mut scratch_tree.heap_mut().unwrap().children);
    parser.scratch_trees.clear();
    result
}

pub(crate) fn ts_parser__shift(
    parser: &mut Parser,
    version: StackVersion,
    state: StateId,
    lookahead: Subtree,
    extra: bool,
) {
    let is_leaf = ts_subtree_child_count(&lookahead) == 0;
    let mut subtree_to_push = lookahead;
    if extra != ts_subtree_extra(&subtree_to_push) && is_leaf {
        subtree_to_push = ts_subtree_make_mut(&mut parser.tree_pool, subtree_to_push);
        ts_subtree_set_extra(&mut subtree_to_push, extra);
    }
    let last_external_token = ts_subtree_has_external_tokens(&subtree_to_push)
        .then(|| ts_subtree_last_external_token(&subtree_to_push));
    ts_stack_push(
        &mut parser.stack,
        &mut parser.tree_pool,
        version,
        subtree_to_push,
        !is_leaf,
        state,
    );
    if let Some(token) = last_external_token {
        ts_stack_set_last_external_token(&mut parser.stack, &mut parser.tree_pool, version, token);
    }
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
    replace_version: bool,
) -> StackVersion {
    // A committed reduction on the only active version needs neither a
    // temporary version nor a slice worklist. Keep this path separate from the
    // general reduction's grouping, selection, and version-merging machinery.
    if replace_version
        && let Some(mut children) = ts_stack_pop_count_in_place(&mut parser.stack, count)
    {
        let language = parser.language.unwrap();
        ts_subtree_array_remove_trailing_extras(&mut children, &mut parser.trailing_extras);
        let state = ts_stack_state(&parser.stack, version);
        let next_state = parser.parse_table_cache.next_state(&language, state, symbol);
        let parent = ts_subtree_new_node_with(
            symbol,
            children,
            production_id as u32,
            &language,
            |data| {
                if end_of_non_terminal_extra && next_state == state {
                    data.extra = true;
                }
                if is_fragile {
                    data.fragile_left = true;
                    data.fragile_right = true;
                    data.parse_state = TS_TREE_STATE_NONE;
                } else {
                    data.parse_state = state;
                }
                let SubtreePayload::Branch(branch) = &mut data.payload else {
                    unreachable!("a reduced node has branch data");
                };
                branch.dynamic_precedence += dynamic_precedence;
            },
        );
        ts_stack_push(
            &mut parser.stack,
            &mut parser.tree_pool,
            version,
            parent,
            false,
            next_state,
        );
        if !parser.trailing_extras.is_empty() {
            for extra in parser.trailing_extras.drain(..) {
                ts_stack_push(
                    &mut parser.stack,
                    &mut parser.tree_pool,
                    version,
                    extra,
                    false,
                    next_state,
                );
            }
        }
        return version;
    }
    ts_parser__reduce_general(
        parser,
        version,
        symbol,
        count,
        dynamic_precedence,
        production_id,
        is_fragile,
        end_of_non_terminal_extra,
    )
}

fn ts_parser__reduce_general(
    parser: &mut Parser,
    version: StackVersion,
    symbol: Symbol,
    count: u32,
    dynamic_precedence: i32,
    production_id: u16,
    is_fragile: bool,
    end_of_non_terminal_extra: bool,
) -> StackVersion {
    let language = parser.language.unwrap();
    let initial_version_count = ts_stack_version_count(&parser.stack);
    let mut slices = ts_stack_pop_count(&mut parser.stack, &mut parser.tree_pool, version, count);
    let pop_size = slices.len();
    let mut pop = slices.iter_mut().peekable();
    let mut removed_version_count = 0;
    let halted_version_count = ts_stack_halted_version_count(&parser.stack);

    // The stack may contain several paths back to the same state. Group slices
    // by their original version, while tracking versions removed along the way.
    while let Some(slice) = pop.next() {
        let slice_version = slice.version - removed_version_count;
        if slice_version > MAX_VERSION_COUNT + MAX_VERSION_COUNT_OVERFLOW + halted_version_count {
            ts_stack_remove_version(&mut parser.stack, &mut parser.tree_pool, slice_version);
            let mut children = std::mem::take(&mut slice.subtrees);
            ts_subtree_array_delete(&mut parser.tree_pool, &mut children);
            removed_version_count += 1;
            while let Some(next_slice) = pop.peek() {
                parser2_log!(parser, "aborting reduce with too many versions");
                if next_slice.version != slice.version {
                    break;
                }
                let next_slice = pop.next().unwrap();
                ts_subtree_array_delete(&mut parser.tree_pool, &mut next_slice.subtrees);
            }
            continue;
        }

        let mut children = std::mem::take(&mut slice.subtrees);
        ts_subtree_array_remove_trailing_extras(&mut children, &mut parser.trailing_extras);
        let state = ts_stack_state(&parser.stack, slice_version);
        let next_state = parser
            .parse_table_cache
            .next_state(&language, state, symbol);
        let initialize = |data: &mut SubtreeHeapData| {
            if end_of_non_terminal_extra && next_state == state {
                data.extra = true;
            }
            if is_fragile || pop_size > 1 || initial_version_count > 1 {
                data.fragile_left = true;
                data.fragile_right = true;
                data.parse_state = TS_TREE_STATE_NONE;
            } else {
                data.parse_state = state;
            }
            let SubtreePayload::Branch(branch) = &mut data.payload else {
                unreachable!("a reduced node has branch data");
            };
            branch.dynamic_precedence += dynamic_precedence;
        };
        // Alternative children are compared using their summarized precedence,
        // before adding this reduction's precedence, just as in C.
        let parent = if pop.peek().is_some_and(|next| next.version == slice.version) {
            let mut parent = ts_subtree_new_node(symbol, children, production_id as u32, &language);
            while pop.peek().is_some_and(|next| next.version == slice.version) {
                let mut children = std::mem::take(&mut pop.next().unwrap().subtrees);
                ts_subtree_array_remove_trailing_extras(
                    &mut children,
                    &mut parser.trailing_extras2,
                );
                if ts_parser__select_children(parser, &parent, &children) {
                    ts_subtree_array_clear(&mut parser.tree_pool, &mut parser.trailing_extras);
                    ts_subtree_release(&mut parser.tree_pool, parent);
                    std::mem::swap(&mut parser.trailing_extras, &mut parser.trailing_extras2);
                    parent = ts_subtree_new_node(symbol, children, production_id as u32, &language);
                } else {
                    // C deletes next_slice.subtrees with its original length,
                    // including the extras. Restore that order before releasing.
                    children.append(&mut parser.trailing_extras2);
                    ts_subtree_array_delete(&mut parser.tree_pool, &mut children);
                }
            }

            initialize(parent.heap_mut().unwrap());
            parent
        } else {
            // A newly reduced node is not shared yet. Finish its header before
            // wrapping it in Arc, avoiding copy-on-write synchronization.
            ts_subtree_new_node_with(
                symbol,
                children,
                production_id as u32,
                &language,
                initialize,
            )
        };

        ts_stack_push(
            &mut parser.stack,
            &mut parser.tree_pool,
            slice_version,
            parent,
            false,
            next_state,
        );
        if !parser.trailing_extras.is_empty() {
            for extra in parser.trailing_extras.drain(..) {
                ts_stack_push(
                    &mut parser.stack,
                    &mut parser.tree_pool,
                    slice_version,
                    extra,
                    false,
                    next_state,
                );
            }
        }
        for j in 0..slice_version {
            if j != version
                && ts_stack_merge(&mut parser.stack, &mut parser.tree_pool, j, slice_version)
            {
                removed_version_count += 1;
                break;
            }
        }
    }
    // Every slice has transferred or released its children. Return only the
    // empty scratch allocation, never retained tree handles.
    // C keeps this array on the stack; freeing it after every reduce needlessly
    // allocates a new buffer for the next pop.
    slices.clear();
    parser.stack.slices = slices;
    if ts_stack_version_count(&parser.stack) > initial_version_count {
        initial_version_count
    } else {
        STACK_VERSION_NONE
    }
}

pub(crate) fn ts_parser__accept(parser: &mut Parser, version: StackVersion, lookahead: Subtree) {
    assert!(ts_subtree_is_eof(&lookahead));
    ts_stack_push(
        &mut parser.stack,
        &mut parser.tree_pool,
        version,
        lookahead,
        false,
        1,
    );
    let pop = ts_stack_pop_all(&mut parser.stack, &mut parser.tree_pool, version);
    let first_version = pop[0].version;
    for slice in pop {
        let mut trees = slice.subtrees;
        let root_index = trees
            .iter()
            .rposition(|tree| !ts_subtree_extra(tree))
            .expect("accepted stack contains a non-extra root");
        let tree = trees.remove(root_index);
        assert!(tree.heap().is_some());
        trees.splice(
            root_index..root_index,
            ts_subtree_children(&tree).iter().cloned(),
        );
        let root = ts_subtree_new_node(
            ts_subtree_symbol(&tree),
            trees,
            ts_subtree_production_id(&tree) as u32,
            parser.language.as_ref().unwrap(),
        );
        ts_subtree_release(&mut parser.tree_pool, tree);
        parser.accept_count = parser.accept_count.wrapping_add(1);

        let finished_tree = std::mem::take(&mut parser.finished_tree);
        if finished_tree.is_null() {
            parser.finished_tree = root;
        } else if ts_parser__select_tree(parser, &finished_tree, &root) {
            ts_subtree_release(&mut parser.tree_pool, finished_tree);
            parser.finished_tree = root;
        } else {
            ts_subtree_release(&mut parser.tree_pool, root);
            parser.finished_tree = finished_tree;
        }
    }
    ts_stack_remove_version(&mut parser.stack, &mut parser.tree_pool, first_version);
    ts_stack_halt(&mut parser.stack, version);
}

pub(crate) fn ts_parser__do_all_potential_reductions(
    parser: &mut Parser,
    starting_version: StackVersion,
    lookahead_symbol: Symbol,
) -> bool {
    use ts_port_tables::ParseAction;

    let language = parser.language.unwrap();
    let initial_version_count = ts_stack_version_count(&parser.stack);
    let mut can_shift_lookahead_symbol = false;
    let mut version = starting_version;
    let mut i: u32 = 0;
    loop {
        let iteration = i;
        i = i.wrapping_add(1);
        let version_count = ts_stack_version_count(&parser.stack);
        if version >= version_count {
            break;
        }
        let mut merged = false;
        for j in initial_version_count..version {
            if ts_stack_merge(&mut parser.stack, &mut parser.tree_pool, j, version) {
                merged = true;
                break;
            }
        }
        if merged {
            continue;
        }

        let state = ts_stack_state(&parser.stack, version);
        let mut has_shift_action = false;
        parser.reduce_actions.clear();
        let (first_symbol, end_symbol) = if lookahead_symbol != 0 {
            (lookahead_symbol, lookahead_symbol.wrapping_add(1))
        } else {
            (1, language.tables.token_count as Symbol)
        };
        for symbol in first_symbol..end_symbol {
            let entry = parser.parse_table_cache.table_entry(&language, state, symbol);
            for action in entry.actions() {
                match *action.action() {
                    ParseAction::Shift {
                        extra: false,
                        repetition: false,
                        ..
                    }
                    | ParseAction::Recover => has_shift_action = true,
                    ParseAction::Reduce {
                        symbol,
                        child_count,
                        dynamic_precedence,
                        production_id,
                    } if child_count > 0 => {
                        ts_reduce_action_set_add(
                            &mut parser.reduce_actions,
                            ReduceAction {
                                symbol,
                                count: child_count as u32,
                                dynamic_precedence: dynamic_precedence as i32,
                                production_id,
                            },
                        );
                    }
                    _ => {}
                }
            }
        }
        let mut reduction_version = STACK_VERSION_NONE;
        for j in 0..parser.reduce_actions.len() {
            let action = parser.reduce_actions[j];
            reduction_version = ts_parser__reduce(
                parser,
                version,
                action.symbol,
                action.count,
                action.dynamic_precedence,
                action.production_id,
                true,
                false,
                false,
            );
        }
        if has_shift_action {
            can_shift_lookahead_symbol = true;
        } else if reduction_version != STACK_VERSION_NONE && iteration < MAX_VERSION_COUNT {
            ts_stack_renumber_version(
                &mut parser.stack,
                &mut parser.tree_pool,
                reduction_version,
                version,
            );
            continue;
        } else if lookahead_symbol != 0 {
            ts_stack_remove_version(&mut parser.stack, &mut parser.tree_pool, version);
        }
        if version == starting_version {
            version = version_count;
        } else {
            version += 1;
        }
    }
    can_shift_lookahead_symbol
}

pub(crate) fn ts_parser__recover_to_state(
    parser: &mut Parser,
    version: StackVersion,
    depth: u32,
    goal_state: StateId,
) -> bool {
    let pop = ts_stack_pop_count(&mut parser.stack, &mut parser.tree_pool, version, depth);
    let mut previous_version = STACK_VERSION_NONE;
    for mut slice in pop {
        if slice.version == previous_version {
            ts_subtree_array_delete(&mut parser.tree_pool, &mut slice.subtrees);
            continue;
        }
        if ts_stack_state(&parser.stack, slice.version) != goal_state {
            ts_stack_halt(&mut parser.stack, slice.version);
            ts_subtree_array_delete(&mut parser.tree_pool, &mut slice.subtrees);
            continue;
        }
        let mut error_trees =
            ts_stack_pop_error(&mut parser.stack, &mut parser.tree_pool, slice.version);
        if !error_trees.is_empty() {
            assert_eq!(error_trees.len(), 1);
            slice
                .subtrees
                .splice(0..0, ts_subtree_children(&error_trees[0]).iter().cloned());
            ts_subtree_array_delete(&mut parser.tree_pool, &mut error_trees);
        }
        ts_subtree_array_remove_trailing_extras(&mut slice.subtrees, &mut parser.trailing_extras);
        if !slice.subtrees.is_empty() {
            let error =
                ts_subtree_new_error_node(slice.subtrees, true, parser.language.as_ref().unwrap());
            ts_stack_push(
                &mut parser.stack,
                &mut parser.tree_pool,
                slice.version,
                error,
                false,
                goal_state,
            );
        }
        for tree in parser.trailing_extras.drain(..) {
            ts_stack_push(
                &mut parser.stack,
                &mut parser.tree_pool,
                slice.version,
                tree,
                false,
                goal_state,
            );
        }
        previous_version = slice.version;
    }
    previous_version != STACK_VERSION_NONE
}

pub(crate) fn ts_parser__recover(
    parser: &mut Parser,
    version: StackVersion,
    mut lookahead: Subtree,
) {
    use crate::error_costs::*;
    use ts_port_tables::ParseAction;

    let language = parser.language.unwrap();
    let mut did_recover = false;
    let previous_version_count = ts_stack_version_count(&parser.stack);
    let position = ts_stack_position(&parser.stack, version);
    let node_count_since_error = ts_stack_node_count_since_error(&mut parser.stack, version);
    let current_error_cost = ts_stack_error_cost(&parser.stack, version);

    // First try returning to a previous state that accepts this token. The
    // original head (and its summary) survives all these pop attempts; borrow
    // one entry at a time so newly-created versions may mutate the stack.
    if !ts_subtree_is_error(&lookahead) {
        let mut i = 0;
        while let Some(entry) = ts_stack_get_summary(&parser.stack, version)
            .and_then(|summary| summary.get(i))
            .copied()
        {
            i += 1;
            if entry.state == ERROR_STATE || entry.position.bytes == position.bytes {
                continue;
            }
            let mut depth = entry.depth;
            if node_count_since_error > 0 {
                depth += 1;
            }
            let would_merge = (0..previous_version_count).any(|j| {
                ts_stack_state(&parser.stack, j) == entry.state
                    && ts_stack_position(&parser.stack, j).bytes == position.bytes
            });
            if would_merge {
                continue;
            }
            let new_cost = current_error_cost
                .wrapping_add(entry.depth.wrapping_mul(ERROR_COST_PER_SKIPPED_TREE))
                .wrapping_add(
                    position
                        .bytes
                        .wrapping_sub(entry.position.bytes)
                        .wrapping_mul(ERROR_COST_PER_SKIPPED_CHAR),
                )
                .wrapping_add(
                    position
                        .extent
                        .row
                        .wrapping_sub(entry.position.extent.row)
                        .wrapping_mul(ERROR_COST_PER_SKIPPED_LINE),
                );
            if ts_parser__better_version_exists(parser, version, false, new_cost) {
                break;
            }
            if parser.parse_table_cache.has_actions(&language, entry.state, ts_subtree_symbol(&lookahead))
                && ts_parser__recover_to_state(parser, version, depth, entry.state)
            {
                did_recover = true;
                parser2_log!(
                    parser,
                    "recover_to_previous state:{}, depth:{}",
                    entry.state,
                    depth
                );
                parser2_log_stack(parser);
                break;
            }
        }
    }

    // Remove failed recovery attempts, without skipping shifted version indices.
    let mut i = previous_version_count;
    while i < ts_stack_version_count(&parser.stack) {
        if !ts_stack_is_active(&parser.stack, i) {
            parser2_log!(parser, "removed paused version:{}", i);
            ts_stack_remove_version(&mut parser.stack, &mut parser.tree_pool, i);
            parser2_log_stack(parser);
        } else {
            i += 1;
        }
    }
    if ts_subtree_is_eof(&lookahead) {
        parser2_log!(parser, "recover_eof");
        let parent = ts_subtree_new_error_node(Vec::new(), false, &language);
        ts_stack_push(
            &mut parser.stack,
            &mut parser.tree_pool,
            version,
            parent,
            false,
            1,
        );
        ts_parser__accept(parser, version, lookahead);
        return;
    }

    // Also try skipping the token, unless a successful previous-state recovery
    // makes this alternative too costly or would change external scanner state.
    if did_recover
        && (ts_stack_version_count(&parser.stack) > MAX_VERSION_COUNT
            || ts_subtree_has_external_scanner_state_change(&lookahead))
    {
        ts_stack_halt(&mut parser.stack, version);
        ts_subtree_release(&mut parser.tree_pool, lookahead);
        return;
    }
    let new_cost = current_error_cost
        .wrapping_add(ERROR_COST_PER_SKIPPED_TREE)
        .wrapping_add(ts_subtree_total_bytes(&lookahead).wrapping_mul(ERROR_COST_PER_SKIPPED_CHAR))
        .wrapping_add(
            ts_subtree_total_size(&lookahead)
                .extent
                .row
                .wrapping_mul(ERROR_COST_PER_SKIPPED_LINE),
        );
    if ts_parser__better_version_exists(parser, version, false, new_cost) {
        ts_stack_halt(&mut parser.stack, version);
        ts_subtree_release(&mut parser.tree_pool, lookahead);
        return;
    }

    // Extra tokens do not contribute to the error cost.
    let actions = parser.parse_table_cache.actions(&language, 1, ts_subtree_symbol(&lookahead));
    if actions
        .last()
        .is_some_and(|action| matches!(action.action(), ParseAction::Shift { extra: true, .. }))
    {
        lookahead = ts_subtree_make_mut(&mut parser.tree_pool, lookahead);
        ts_subtree_set_extra(&mut lookahead, true);
    }
    parser2_log!(
        parser,
        "skip_token symbol:{}",
        parser2_tree_name(parser, &lookahead)
    );
    let last_external_token = ts_subtree_has_external_tokens(&lookahead)
        .then(|| ts_subtree_last_external_token(&lookahead));
    let mut error_repeat =
        ts_subtree_new_node(BUILTIN_SYM_ERROR_REPEAT, vec![lookahead], 0, &language);
    if node_count_since_error > 0 {
        let pop = ts_stack_pop_count(&mut parser.stack, &mut parser.tree_pool, version, 1);
        let pop_size = pop.len();
        let mut pop = pop.into_iter();
        let mut first = pop
            .next()
            .expect("skipped tokens have an error at the top of the stack");
        // C arbitrarily keeps only the first error if these versions merged.
        if pop_size > 1 {
            for mut slice in pop {
                ts_subtree_array_delete(&mut parser.tree_pool, &mut slice.subtrees);
            }
            while ts_stack_version_count(&parser.stack) > first.version + 1 {
                ts_stack_remove_version(
                    &mut parser.stack,
                    &mut parser.tree_pool,
                    first.version + 1,
                );
            }
        }
        ts_stack_renumber_version(
            &mut parser.stack,
            &mut parser.tree_pool,
            first.version,
            version,
        );
        first.subtrees.push(error_repeat);
        error_repeat = ts_subtree_new_node(BUILTIN_SYM_ERROR_REPEAT, first.subtrees, 0, &language);
    }
    ts_stack_push(
        &mut parser.stack,
        &mut parser.tree_pool,
        version,
        error_repeat,
        false,
        ERROR_STATE,
    );
    if let Some(token) = last_external_token {
        ts_stack_set_last_external_token(&mut parser.stack, &mut parser.tree_pool, version, token);
    }
    let mut has_error = true;
    for i in 0..ts_stack_version_count(&parser.stack) {
        if !ts_parser__version_status(parser, i).is_in_error {
            has_error = false;
            break;
        }
    }
    parser.has_error = has_error;
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
                let next_state = parser.parse_table_cache.next_state(&language, state, missing_symbol);
                if next_state == 0 || next_state == state {
                    continue;
                }
                if parser.parse_table_cache.has_reduce_action(
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
                        parser3_log!(
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
    // Reuse, lexing and reductions preserve this head's scanner state; only a
    // shift/recovery (which returns from advance) installs a new external token.
    // Borrow from the head when needed, retaining only for the owning cache.
    let mut did_reuse = true;
    let mut lookahead = Subtree::Null;
    let mut table_entry = TableEntry::default();

    if allow_node_reuse {
        lookahead = ts_parser__reuse_node(
            parser,
            version,
            &mut state,
            position,
            &mut table_entry,
        );
    }
    if lookahead.is_null() {
        did_reuse = false;
        lookahead = ts_parser__get_cached_token(
            parser,
            state,
            position as usize,
            ts_stack_last_external_token(&parser.stack, version),
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
                cache_token_for_version(parser, version, position, lookahead.clone());
                ts_subtree_symbol(&lookahead)
            } else {
                // Null lookahead terminates a non-terminal extra; its fixed
                // reduction is stored in the EOF entry. Lex again afterwards.
                BUILTIN_SYM_END
            };
            table_entry = parser.parse_table_cache.table_entry(&language, state, symbol);
        }
        if !ts_parser__check_progress(parser, context, Some(&mut lookahead), Some(position), 1) {
            return false;
        }

        let mut did_reduce = false;
        let mut last_reduction_version = STACK_VERSION_NONE;
        for entry in table_entry.actions() {
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
                        parser3_log!(parser, format_args!("shift_extra"));
                        state
                    } else {
                        parser3_log!(parser, format_args!("shift state:{}", shift_state));
                        shift_state
                    };
                    if ts_subtree_child_count(&lookahead) > 0 {
                        ts_parser__breakdown_lookahead(parser, &mut lookahead, state);
                        next_state =
                            parser.parse_table_cache.next_state(&language, state, ts_subtree_symbol(&lookahead));
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
                    parser3_log!(
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
                        table_entry.actions().len() > 1,
                        lookahead.is_null(),
                        table_entry.actions().len() == 1,
                    );
                    did_reduce = true;
                    if reduction_version != STACK_VERSION_NONE {
                        last_reduction_version = reduction_version;
                    }
                }
                ParseAction::Accept => {
                    parser3_log!(parser, format_args!("accept"));
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
                    parser.parse_table_cache.table_entry(&language, state, ts_subtree_leaf_symbol(&lookahead));
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
            table_entry = parser.parse_table_cache.table_entry(&language, state, word_symbol);
            if !table_entry.actions().is_empty() {
                parser3_log!(
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
        parser3_log!(
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

#[inline]
pub(crate) fn ts_parser__condense_stack(parser: &mut Parser) -> u32 {
    // An active sole version cannot merge, be pruned, or need resuming. Keep
    // the version-status read: it also lowers the saved error node baseline.
    if ts_stack_version_count(&parser.stack) == 1 && ts_stack_is_active(&parser.stack, 0) {
        let status = ts_parser__version_status(parser, 0);
        return if status.is_in_error { u32::MAX } else { status.cost };
    }
    ts_parser__condense_stack_general(parser)
}

fn ts_parser__condense_stack_general(parser: &mut Parser) -> u32 {
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
                parser3_log!(parser, format_args!("resume version:{}", i));
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
        parser3_log!(parser, format_args!("condense"));
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
    // Borrow disjoint child slots instead of detaching/re-attaching every
    // visited node. This preserves Arc uniqueness without reference counting.
    let mut finished_tree = std::mem::take(&mut parser.finished_tree);
    let mut cursor = BalanceCursor::new(
        &mut finished_tree,
        std::mem::take(&mut parser.balance_path),
    );
    let mut completed = true;
    while let Some(tree) = cursor.next() {
        if !ts_parser__check_progress(parser, context, None, None, 1) {
            completed = false;
            break;
        }
        if ts_subtree_repeat_depth(tree) > 0 {
            let children = ts_subtree_children(tree);
            let repeat_delta = i64::from(ts_subtree_repeat_depth(&children[0]))
                - i64::from(ts_subtree_repeat_depth(children.last().unwrap()));
            if repeat_delta > 0 {
                let mut i = repeat_delta as u32 / 2;
                while i > 0 {
                    ts_subtree_compress(
                        tree,
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
                        completed = false;
                        break;
                    }
                    i /= 2;
                }
            }
        }
        if !completed {
            break;
        }
        cursor.push_children(tree);
    }
    parser.balance_path = cursor.path;
    if completed {
        parser.balance_path.clear();
    }
    parser.finished_tree = finished_tree;
    completed
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
        parse_table_cache: ParseTableCache::default(),
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
    parser.parse_table_cache.clear();
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
fn parser3_log_message(parser: &mut Parser, message: std::fmt::Arguments<'_>) {
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

/// Disjoint mutable child slots in C's LIFO work order. Borrowing the slots
/// neither increments reference counts nor moves tree handles. A path to the
/// current node is enough to reconstruct pending left siblings after cancellation.
struct BalanceCursor<'tree> {
    pending: Vec<(&'tree mut Subtree, usize, usize)>,
    path: Vec<usize>,
}

impl<'tree> BalanceCursor<'tree> {
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

    fn new(mut tree: &'tree mut Subtree, path: Vec<usize>) -> Self {
        let mut pending = Vec::new();
        // Ancestors and their right siblings have already been processed. Their
        // left siblings remain pending, below the canceled node in the worklist.
        for (depth, &index) in path.iter().enumerate() {
            let (left, right) = Self::children_mut(tree).split_at_mut(index);
            for (index, child) in left.iter_mut().enumerate() {
                if Self::is_unique_branch(child) {
                    pending.push((child, depth + 1, index));
                }
            }
            tree = &mut right[0];
        }
        pending.push((tree, path.len(), path.last().copied().unwrap_or(0)));
        Self { pending, path }
    }

    fn next(&mut self) -> Option<&'tree mut Subtree> {
        let (tree, depth, index) = self.pending.pop()?;
        self.path.truncate(depth.saturating_sub(1));
        if depth > 0 {
            self.path.push(index);
        }
        Some(tree)
    }

    fn push_children(&mut self, tree: &'tree mut Subtree) {
        let depth = self.path.len() + 1;
        for (index, child) in Self::children_mut(tree).iter_mut().enumerate() {
            if Self::is_unique_branch(child) {
                self.pending.push((child, depth, index));
            }
        }
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
            parse_table_cache: ParseTableCache::default(),
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

    fn reduction_language() -> Language {
        // Reductions only need the tables, not a generated lexer or scanner.
        static TABLES: std::sync::LazyLock<ts_port_tables::LanguageTables> =
            std::sync::LazyLock::new(|| {
                ts_port_tables::LanguageTables::decode(
                    include_bytes!("../../grammars/c/src/tables.bin"),
                    |_, _| panic!("reduction test does not lex"),
                    Some(|_, _| panic!("reduction test does not lex keywords")),
                    None,
                )
            });
        Language::from(&*TABLES)
    }

    #[test]
    fn reductions_reuse_slice_storage_without_retaining_children() {
        for replace_version in [false, true] {
            let mut parser = ts_parser_new();
            let language = reduction_language();
            parser.language = Some(language);
            let symbol = language.tables.token_count as Symbol;
            let allocation = parser.stack.slices.as_ptr();
            let capacity = parser.stack.slices.capacity();
            for i in 0..128 {
                let version = ts_parser__reduce(
                    &mut parser,
                    0,
                    symbol,
                    u32::from(i > 0),
                    0,
                    0,
                    false,
                    false,
                    replace_version,
                );
                assert_eq!(version, if replace_version { 0 } else { 1 });
                assert!(parser.stack.slices.is_empty());
                assert_eq!(parser.stack.slices.as_ptr(), allocation);
                assert_eq!(parser.stack.slices.capacity(), capacity);
                ts_stack_renumber_version(&mut parser.stack, &mut parser.tree_pool, version, 0);
                let head = parser.stack.heads[0].node;
                let node = parser.stack.arena.nodes[head.0].as_ref().unwrap();
                let tree = &node.links[0].as_ref().unwrap().subtree;
                let Subtree::Heap(data) = tree else {
                    panic!("reduced branch")
                };
                assert_eq!(std::sync::Arc::strong_count(data), 1);
                assert_eq!(data.children.len(), usize::from(i > 0));
            }
            ts_parser_reset(&mut parser);
            assert!(parser.stack.slices.is_empty());
        }
    }

    #[test]
    fn reduction_initializes_header_after_child_summaries() {
        let language = reduction_language();
        let symbol = language.tables.token_count as Symbol;
        for fragile in [false, true] {
            let mut parser = ts_parser_new();
            parser.language = Some(language);
            let child = ts_subtree_new_node_with(
                symbol,
                vec![Subtree::Inline(InlineLeaf {
                    symbol: 1,
                    size_bytes: 3,
                    parse_state: 9,
                    ..InlineLeaf::default()
                })],
                0,
                &language,
                |data| {
                    let SubtreePayload::Branch(branch) = &mut data.payload else {
                        unreachable!()
                    };
                    branch.dynamic_precedence = 7;
                },
            );
            ts_stack_push(&mut parser.stack, &mut parser.tree_pool, 0, child, false, 1);
            for symbol in [2, 3] {
                ts_stack_push(
                    &mut parser.stack,
                    &mut parser.tree_pool,
                    0,
                    Subtree::Inline(InlineLeaf {
                        symbol,
                        flags: EXTRA,
                        ..InlineLeaf::default()
                    }),
                    false,
                    1,
                );
            }
            let version = ts_parser__reduce(&mut parser, 0, symbol, 1, -3, 0, fragile, false, true);
            ts_stack_renumber_version(&mut parser.stack, &mut parser.tree_pool, version, 0);
            let mut node = parser.stack.heads[0].node;
            for symbol in [3, 2] {
                let link = parser.stack.arena.nodes[node.0].as_ref().unwrap().links[0]
                    .as_ref()
                    .unwrap();
                assert_eq!(ts_subtree_symbol(&link.subtree), symbol);
                assert!(ts_subtree_extra(&link.subtree));
                node = link.node;
            }
            let tree = &parser.stack.arena.nodes[node.0].as_ref().unwrap().links[0]
                .as_ref()
                .unwrap()
                .subtree;
            let data = tree.heap().unwrap();
            assert_eq!(data.children.len(), 1);
            assert_eq!(data.size.bytes, 3);
            assert_eq!(ts_subtree_dynamic_precedence(tree), 4);
            assert_eq!(data.fragile_left, fragile);
            assert_eq!(data.fragile_right, fragile);
            assert_eq!(
                data.parse_state,
                if fragile { TS_TREE_STATE_NONE } else { 1 }
            );
            assert!(parser.stack.slices.is_empty());
            ts_parser_reset(&mut parser);
        }
    }

    #[test]
    fn committed_reductions_match_general_reductions_including_empty_extras() {
        let language = reduction_language();
        let symbol = language.tables.token_count as Symbol;
        for state in [ERROR_STATE, 1] {
            for count in [0, 1, 3] {
                for fragile in [false, true] {
                    for end_of_extra in [false, true] {
                        let mut snapshots = Vec::new();
                        for replace_version in [false, true] {
                            let mut parser = ts_parser_new();
                            parser.language = Some(language);
                            // Leave a prefix below the reduction, including a
                            // valid link when reducing in the error state.
                            ts_stack_push(
                                &mut parser.stack,
                                &mut parser.tree_pool,
                                0,
                                Subtree::Null,
                                false,
                                state,
                            );
                            for i in 0..count {
                                for flags in [VISIBLE | NAMED, EXTRA, EXTRA] {
                                    ts_stack_push(
                                        &mut parser.stack,
                                        &mut parser.tree_pool,
                                        0,
                                        Subtree::Inline(InlineLeaf {
                                            symbol: (i + 1) as u8,
                                            size_bytes: 1,
                                            flags,
                                            ..InlineLeaf::default()
                                        }),
                                        false,
                                        state,
                                    );
                                }
                            }
                            let version = ts_parser__reduce(
                                &mut parser,
                                0,
                                symbol,
                                count,
                                -3,
                                0,
                                fragile,
                                end_of_extra,
                                replace_version,
                            );
                            ts_stack_renumber_version(
                                &mut parser.stack,
                                &mut parser.tree_pool,
                                version,
                                0,
                            );
                            let final_state = ts_stack_state(&parser.stack, 0);
                            let slices = ts_stack_pop_all(&mut parser.stack, &mut parser.tree_pool, 0);
                            // Debug includes every subtree header/branch field
                            // recursively, not Arc addresses or arena indices.
                            snapshots.push((final_state, format!("{:?}", slices[0].subtrees)));
                            ts_parser_reset(&mut parser);
                        }
                        assert_eq!(snapshots[0], snapshots[1]);
                    }
                }
            }
        }
    }

    #[test]
    fn condense_single_active_version_preserves_error_baseline_updates() {
        for state in [ERROR_STATE, 1, 2] {
            for null in [false, true] {
                for baseline in [0, 17] {
                    let mut snapshots = Vec::new();
                    for fast in [false, true] {
                        let mut parser = ts_parser_new();
                        let tree = if null {
                            Subtree::Null
                        } else {
                            Subtree::Inline(InlineLeaf {
                                flags: VISIBLE,
                                size_bytes: 1,
                                ..InlineLeaf::default()
                            })
                        };
                        ts_stack_push(
                            &mut parser.stack,
                            &mut parser.tree_pool,
                            0,
                            tree,
                            false,
                            state,
                        );
                        parser.stack.heads[0].node_count_at_last_error = baseline;
                        let cost = if fast {
                            ts_parser__condense_stack(&mut parser)
                        } else {
                            ts_parser__condense_stack_general(&mut parser)
                        };
                        snapshots.push((cost, parser.stack.heads[0].node_count_at_last_error));
                        assert_eq!(ts_stack_version_count(&parser.stack), 1);
                        assert!(ts_stack_is_active(&parser.stack, 0));
                        ts_parser_reset(&mut parser);
                    }
                    assert_eq!(snapshots[0], snapshots[1]);
                }
            }
        }
    }

    #[test]
    fn ambiguous_reductions_select_before_adding_action_precedence() {
        let language = reduction_language();
        let symbol = language.tables.token_count as Symbol;
        for precedences in [[1, 2], [2, 1]] {
            for action_precedence in [-10, 10] {
                let mut parser = ts_parser_new();
                parser.language = Some(language);
                ts_stack_copy_version(&mut parser.stack, 0);
                for (version, precedence) in precedences.into_iter().enumerate() {
                    let child = ts_subtree_new_node_with(
                        (precedence + 1) as Symbol,
                        vec![Subtree::Inline(InlineLeaf {
                            symbol: 1,
                            size_bytes: 1,
                            ..InlineLeaf::default()
                        })],
                        0,
                        &language,
                        |data| {
                            let SubtreePayload::Branch(branch) = &mut data.payload else {
                                unreachable!()
                            };
                            branch.dynamic_precedence = precedence;
                        },
                    );
                    ts_stack_push(
                        &mut parser.stack,
                        &mut parser.tree_pool,
                        version as StackVersion,
                        child,
                        false,
                        1,
                    );
                }
                assert!(ts_stack_merge(
                    &mut parser.stack,
                    &mut parser.tree_pool,
                    0,
                    1
                ));
                let version = ts_parser__reduce(
                    &mut parser,
                    0,
                    symbol,
                    1,
                    action_precedence,
                    0,
                    false,
                    false,
                    true,
                );
                ts_stack_renumber_version(&mut parser.stack, &mut parser.tree_pool, version, 0);
                let node = parser.stack.heads[0].node;
                let tree = &parser.stack.arena.nodes[node.0].as_ref().unwrap().links[0]
                    .as_ref()
                    .unwrap()
                    .subtree;
                assert_eq!(ts_subtree_symbol(&ts_subtree_children(tree)[0]), 3);
                assert_eq!(ts_subtree_dynamic_precedence(tree), 2 + action_precedence);
                assert!(ts_subtree_fragile_left(tree));
                assert!(ts_subtree_fragile_right(tree));
                assert_eq!(ts_subtree_parse_state(tree), TS_TREE_STATE_NONE);
                assert!(parser.stack.slices.is_empty());
                ts_parser_reset(&mut parser);
            }
        }
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
        let mut tree = branch(
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
        let mut cursor = BalanceCursor::new(&mut tree, Vec::new());
        let mut symbols = Vec::new();
        while let Some(tree) = cursor.next() {
            assert!(BalanceCursor::is_unique_branch(tree));
            symbols.push(ts_subtree_symbol(tree));
            cursor.push_children(tree);
        }
        assert_eq!(symbols, [1, 3, 4, 2]);
        assert!(cursor.pending.is_empty());
        let Subtree::Heap(root) = &tree else {
            unreachable!()
        };
        assert_eq!(Arc::as_ptr(root), root_address);
        assert!(root.children[1].ptr_eq(&shared));
        assert_eq!(ts_subtree_symbol(&root.children[0]), 2);
        assert_eq!(ts_subtree_symbol(&root.children[2]), 3);
    }

    #[test]
    fn balancing_resume_rebuilds_pending_left_siblings_at_every_boundary() {
        let shared = branch(7, vec![Subtree::Inline(InlineLeaf::default())]);
        let leaf = || Subtree::Inline(InlineLeaf::default());
        let mut tree = branch(
            1,
            vec![
                branch(2, vec![branch(5, vec![leaf()]), branch(6, vec![leaf()])]),
                shared.clone(),
                branch(3, vec![branch(4, vec![leaf()]), leaf()]),
            ],
        );
        let expected = [1, 3, 4, 2, 6, 5];
        for boundary in 0..expected.len() {
            let mut cursor = BalanceCursor::new(&mut tree, Vec::new());
            for &symbol in &expected[..boundary] {
                let node = cursor.next().unwrap();
                assert_eq!(ts_subtree_symbol(node), symbol);
                cursor.push_children(node);
            }
            let canceled_node = cursor.next().unwrap();
            assert_eq!(ts_subtree_symbol(canceled_node), expected[boundary]);
            let path = cursor.path;
            drop(cursor.pending);

            // No root or child slot is left detached while parsing is canceled.
            assert_eq!(ts_subtree_symbol(&tree), 1);
            assert!(ts_subtree_children(&tree)[1].ptr_eq(&shared));
            let mut resumed = BalanceCursor::new(&mut tree, path);
            let mut symbols = Vec::new();
            while let Some(node) = resumed.next() {
                symbols.push(ts_subtree_symbol(node));
                assert!(BalanceCursor::is_unique_branch(node));
                resumed.push_children(node);
            }
            assert_eq!(symbols, expected[boundary..]);
        }
    }

    #[test]
    fn balancing_cursor_resumes_before_every_child_including_earlier_siblings() {
        let expected = [1, 5, 7, 6, 4, 2, 3];
        for stop in 0..expected.len() {
            let leaf = || Subtree::Inline(InlineLeaf::default());
            let mut tree = branch(
                1,
                vec![
                    leaf(),
                    branch(2, vec![branch(3, vec![leaf()])]),
                    branch(4, vec![leaf()]),
                    leaf(),
                    branch(5, vec![branch(6, vec![leaf()]), branch(7, vec![leaf()])]),
                ],
            );
            let mut cursor = BalanceCursor::new(&mut tree, Vec::new());
            let mut visited = Vec::new();
            for _ in 0..stop {
                let node = cursor.next().unwrap();
                visited.push(ts_subtree_symbol(node));
                cursor.push_children(node);
            }
            assert_eq!(ts_subtree_symbol(cursor.next().unwrap()), expected[stop]);
            let path = cursor.path;
            drop(cursor.pending);

            // No restoration step or extra Arc owner is needed on cancellation.
            assert_eq!(ts_subtree_symbol(&tree), 1);
            let mut resumed = BalanceCursor::new(&mut tree, path);
            while let Some(node) = resumed.next() {
                visited.push(ts_subtree_symbol(node));
                resumed.push_children(node);
            }
            assert_eq!(visited, expected, "resumed at traversal index {stop}");
        }
    }

    #[test]
    fn disabled_diagnostics_do_not_evaluate_arguments() {
        let mut parser = parser();
        let mut evaluations = 0;
        parser3_log!(&mut parser, format_args!("{}", {
            evaluations += 1;
            "disabled"
        }));
        assert_eq!(evaluations, 0);
        assert!(parser.lexer.debug_buffer.is_empty());

        parser.logger = Some(Box::new(|kind, message| {
            assert_eq!(kind, LogType::Parse);
            assert_eq!(message, "enabled");
        }));
        parser3_log!(&mut parser, format_args!("{}", {
            evaluations += 1;
            "enabled"
        }));
        assert_eq!(evaluations, 1);
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
