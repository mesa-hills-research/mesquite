//! Swift's external scanner, translated from `src/scanner.c`.

use ts_port_tables::{ExternalScanner, Lexer};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
enum Token {
    BlockComment,
    RawStrPart,
    RawStrContinuingIndicator,
    RawStrEndPart,
    ImplicitSemi,
    ExplicitSemi,
    ArrowOperator,
    DotOperator,
    ConjunctionOperator,
    DisjunctionOperator,
    NilCoalescingOperator,
    DoubleOptional,
    EqualSign,
    EqEq,
    PlusThenWs,
    MinusThenWs,
    Bang,
    ThrowsKeyword,
    RethrowsKeyword,
    DefaultKeyword,
    WhereKeyword,
    ElseKeyword,
    CatchKeyword,
    AsKeyword,
    AsQuest,
    AsBang,
    AsyncKeyword,
    CustomOperator,
    HashSymbol,
    DirectiveIf,
    DirectiveElseif,
    DirectiveElse,
    DirectiveEndif,
    FakeTryBang,
}
use Token::*;

const OPERATORS: [&[u8]; 20] = [
    b"->",
    b".",
    b"&&",
    b"||",
    b"??",
    b"=",
    b"==",
    b"+",
    b"-",
    b"!",
    b"throws",
    b"rethrows",
    b"default",
    b"where",
    b"else",
    b"catch",
    b"as",
    b"as?",
    b"as!",
    b"async",
];

#[derive(Clone, Copy, PartialEq, Eq)]
enum IllegalTerminatorGroup {
    Alphanumeric,
    OperatorSymbols,
    OperatorOrDot,
    NonWhitespace,
}
use IllegalTerminatorGroup::*;

const OP_ILLEGAL_TERMINATORS: [IllegalTerminatorGroup; 20] = [
    OperatorSymbols, // ->
    OperatorOrDot,   // .
    OperatorSymbols, // &&
    OperatorSymbols, // ||
    OperatorSymbols, // ??
    OperatorSymbols, // =
    OperatorSymbols, // ==
    NonWhitespace,   // +
    NonWhitespace,   // -
    OperatorSymbols, // !
    Alphanumeric,    // throws
    Alphanumeric,    // rethrows
    Alphanumeric,    // default
    Alphanumeric,    // where
    Alphanumeric,    // else
    Alphanumeric,    // catch
    Alphanumeric,    // as
    OperatorSymbols, // as?
    OperatorSymbols, // as!
    Alphanumeric,    // async
];

const OP_SYMBOLS: [Token; 20] = [
    ArrowOperator,
    DotOperator,
    ConjunctionOperator,
    DisjunctionOperator,
    NilCoalescingOperator,
    EqualSign,
    EqEq,
    PlusThenWs,
    MinusThenWs,
    Bang,
    ThrowsKeyword,
    RethrowsKeyword,
    DefaultKeyword,
    WhereKeyword,
    ElseKeyword,
    CatchKeyword,
    AsKeyword,
    AsQuest,
    AsBang,
    AsyncKeyword,
];

const RESERVED_OPS: [&[u8]; 31] = [
    b"/", b"=", b"-", b"+", b"!", b"*", b"%", b"<", b">", b"&", b"|", b"^", b"?", b"~", b".",
    b"..", b"->", b"/*", b"*/", b"+=", b"-=", b"*=", b"/=", b"%=", b">>", b"<<", b"++", b"--",
    b"===", b"...", b"..<",
];

fn is_cross_semi_token(token: Token) -> bool {
    matches!(
        token,
        ArrowOperator
            | DotOperator
            | EqualSign
            | EqEq
            | PlusThenWs
            | MinusThenWs
            | ThrowsKeyword
            | RethrowsKeyword
            | DefaultKeyword
            | WhereKeyword
            | ElseKeyword
            | CatchKeyword
            | AsKeyword
            | AsQuest
            | AsBang
            | AsyncKeyword
            | CustomOperator
    )
}

// The reference uses the C locale's wide-character predicates, not Unicode classes.
fn is_space(character: i32) -> bool {
    matches!(character, 0x09..=0x0d | 0x20)
}

fn is_alnum(character: i32) -> bool {
    matches!(character, 0x30..=0x39 | 0x41..=0x5a | 0x61..=0x7a)
}

fn should_treat_as_wspace(character: i32) -> bool {
    is_space(character) || character == ';' as i32
}

fn is_legal_custom_operator(char_idx: usize, first_char: i32, cur_char: i32) -> bool {
    let is_first_char = char_idx == 0;
    match cur_char {
        // = - + ! % < > & | ^ ? ~
        0x3d | 0x2d | 0x2b | 0x21 | 0x25 | 0x3c | 0x3e | 0x26 | 0x7c | 0x5e | 0x3f | 0x7e => true,
        0x2e => is_first_char || first_char == '.' as i32,
        // /* and // cannot start custom operators.
        0x2a | 0x2f => char_idx != 1 || first_char != '/' as i32,
        0x00a1..=0x00a7
        | 0x00a9
        | 0x00ab
        | 0x00ac
        | 0x00ae
        | 0x00b0..=0x00b1
        | 0x00b6
        | 0x00bb
        | 0x00bf
        | 0x00d7
        | 0x00f7
        | 0x2016..=0x2017
        | 0x2020..=0x2027
        | 0x2030..=0x203e
        | 0x2041..=0x2053
        | 0x2055..=0x205e
        | 0x2190..=0x23ff
        | 0x2500..=0x2775
        | 0x2794..=0x2bff
        | 0x2e00..=0x2e7f
        | 0x3001..=0x3003
        | 0x3008..=0x3020
        | 0x3030 => true,
        0x0300..=0x036f
        | 0x1dc0..=0x1dff
        | 0x20d0..=0x20ff
        | 0xfe00..=0xfe0f
        | 0xfe20..=0xfe2f
        | 0xe0100..=0xe01ef => !is_first_char,
        _ => false,
    }
}

/// The C switch groups candidates by their first character in this same order.
fn collect_fixed_operator_candidates(
    character: i32,
    valid_symbols: &[bool],
    candidates: &mut [usize; 4],
) -> usize {
    let indices: &[usize] = match character {
        0x2d => &[0, 8],           // ->, -
        0x2e => &[1],              // .
        0x26 => &[2],              // &&
        0x7c => &[3],              // ||
        0x3f => &[4],              // ??
        0x3d => &[5, 6],           // =, ==
        0x2b => &[7],              // +
        0x21 => &[9],              // !
        0x74 => &[10],             // throws
        0x72 => &[11],             // rethrows
        0x64 => &[12],             // default
        0x77 => &[13],             // where
        0x65 => &[14],             // else
        0x63 => &[15],             // catch
        0x61 => &[16, 17, 18, 19], // as, as?, as!, async
        _ => &[],
    };
    let mut count = 0;
    for &index in indices {
        if valid_symbols[OP_SYMBOLS[index] as usize]
            || (index == 4 && valid_symbols[DoubleOptional as usize])
        {
            candidates[count] = index;
            count += 1;
        }
    }
    count
}

/// Whether the switch (including its intentional fallthroughs) accepts a terminator.
fn legal_terminator(character: i32, illegal: IllegalTerminatorGroup) -> bool {
    let operator_symbol = matches!(
        character,
        0x2f | 0x3d
            | 0x2d
            | 0x2b
            | 0x21
            | 0x2a
            | 0x25
            | 0x3c
            | 0x3e
            | 0x26
            | 0x7c
            | 0x5e
            | 0x3f
            | 0x7e
    );
    if operator_symbol && illegal == OperatorSymbols {
        return false;
    }
    if (operator_symbol || character == '.' as i32) && illegal == OperatorOrDot {
        return false;
    }
    if is_alnum(character) && illegal == Alphanumeric {
        return false;
    }
    if !is_space(character) && illegal == NonWhitespace {
        return false;
    }
    true
}

fn eat_operators(
    lexer: &mut dyn Lexer,
    valid_symbols: &[bool],
    mark_end: bool,
    token_is_immediate: bool,
    prior_char: i32,
) -> Option<Token> {
    let first_char = if prior_char != 0 {
        prior_char
    } else {
        lexer.lookahead()
    };
    let mut possible_custom_operator = valid_symbols[CustomOperator as usize]
        && is_legal_custom_operator(0, first_char, first_char);
    let mut possible_operators = [0; 4];
    let mut possible_operator_count =
        collect_fixed_operator_candidates(first_char, valid_symbols, &mut possible_operators);
    if possible_operator_count == 0 && !possible_custom_operator {
        return None;
    }

    let mut reserved_operators = [0u8; RESERVED_OPS.len()];
    if possible_custom_operator {
        for (encountered, text) in reserved_operators.iter_mut().zip(RESERVED_OPS) {
            *encountered = u8::from(text[0] as i32 == first_char);
        }
    }

    let mut last_examined_char = first_char;
    let mut str_idx = usize::from(prior_char != 0);
    let mut full_match = None;
    loop {
        let mut candidate_idx = 0;
        while candidate_idx < possible_operator_count {
            let op_idx = possible_operators[candidate_idx];
            let text = OPERATORS[op_idx];
            if str_idx == text.len() {
                if legal_terminator(lexer.lookahead(), OP_ILLEGAL_TERMINATORS[op_idx]) {
                    full_match = Some(op_idx);
                    if mark_end {
                        lexer.mark_end();
                    }
                }
                possible_operator_count -= 1;
                possible_operators[candidate_idx] = possible_operators[possible_operator_count];
                continue;
            }
            if text[str_idx] as i32 != lexer.lookahead() {
                possible_operator_count -= 1;
                possible_operators[candidate_idx] = possible_operators[possible_operator_count];
                continue;
            }
            candidate_idx += 1;
        }

        if possible_custom_operator {
            for (encountered, text) in reserved_operators.iter_mut().zip(RESERVED_OPS) {
                if *encountered == 0 {
                    continue;
                }
                if str_idx == text.len() || text[str_idx] as i32 != lexer.lookahead() {
                    *encountered = 0;
                    continue;
                }
                if str_idx + 1 == text.len() {
                    *encountered = 2;
                }
            }
        }

        possible_custom_operator = possible_custom_operator
            && is_legal_custom_operator(str_idx, first_char, lexer.lookahead());
        if possible_operator_count == 0 {
            if !possible_custom_operator {
                break;
            } else if mark_end && full_match.is_none() {
                lexer.mark_end();
            }
        }

        last_examined_char = lexer.lookahead();
        lexer.advance(false);
        str_idx += 1;
        if possible_operator_count == 0
            && !is_legal_custom_operator(str_idx, first_char, lexer.lookahead())
        {
            break;
        }
    }

    if let Some(full_match) = full_match {
        // The only nonzero entry in OP_SYMBOL_SUPPRESSOR is BANG -> FAKE_TRY_BANG.
        if OP_SYMBOLS[full_match] == Bang && valid_symbols[FakeTryBang as usize] {
            return None;
        }
        let mut matched_symbol = OP_SYMBOLS[full_match];
        if matched_symbol == NilCoalescingOperator && valid_symbols[DoubleOptional as usize] {
            if !token_is_immediate && !valid_symbols[NilCoalescingOperator as usize] {
                return None;
            }
            matched_symbol = if token_is_immediate {
                DoubleOptional
            } else {
                NilCoalescingOperator
            };
        }
        return Some(matched_symbol);
    }

    if possible_custom_operator && !reserved_operators.contains(&2) {
        if (last_examined_char != '<' as i32 || is_space(lexer.lookahead())) && mark_end {
            lexer.mark_end();
        }
        return Some(CustomOperator);
    }
    None
}

/// Unlike the C out-parameter, a found token is carried only by a found directive.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ParseDirective {
    ContinueNothing,
    ContinueToken(Token),
    ContinueSlashConsumed,
    StopNothing,
    StopToken(Token),
    StopEndOfFile,
}
use ParseDirective::*;

fn eat_comment(lexer: &mut dyn Lexer, mark_end: bool) -> ParseDirective {
    if lexer.lookahead() != '/' as i32 {
        return ContinueNothing;
    }
    lexer.advance(false);
    if lexer.lookahead() != '*' as i32 {
        return ContinueSlashConsumed;
    }
    lexer.advance(false);

    let mut after_star = false;
    let mut nesting_depth = 1u32;
    loop {
        match lexer.lookahead() {
            0 => return StopEndOfFile,
            0x2a => {
                lexer.advance(false);
                after_star = true;
            }
            0x2f => {
                if after_star {
                    lexer.advance(false);
                    after_star = false;
                    nesting_depth = nesting_depth.wrapping_sub(1);
                    if nesting_depth == 0 {
                        if mark_end {
                            lexer.mark_end();
                        }
                        return StopToken(BlockComment);
                    }
                } else {
                    lexer.advance(false);
                    after_star = false;
                    if lexer.lookahead() == '*' as i32 {
                        nesting_depth = nesting_depth.wrapping_add(1);
                        lexer.advance(false);
                    }
                }
            }
            _ => {
                lexer.advance(false);
                after_star = false;
            }
        }
    }
}

fn eat_whitespace(lexer: &mut dyn Lexer, valid_symbols: &[bool]) -> ParseDirective {
    let mut ws_directive = ContinueNothing;
    let semi_is_valid =
        valid_symbols[ImplicitSemi as usize] && valid_symbols[ExplicitSemi as usize];
    let mut lookahead;
    loop {
        lookahead = lexer.lookahead();
        if !should_treat_as_wspace(lookahead) {
            break;
        }
        if lookahead == ';' as i32 {
            if semi_is_valid {
                ws_directive = StopToken(ExplicitSemi);
                lexer.advance(false);
            }
            break;
        }
        lexer.advance(true);
        lexer.mark_end();
        if ws_directive == ContinueNothing && matches!(lookahead, 0x0a | 0x0d) {
            ws_directive = ContinueToken(ImplicitSemi);
        }
    }

    if ws_directive == ContinueToken(ImplicitSemi) && lookahead == '/' as i32 {
        let mut has_seen_single_comment = false;
        while lexer.lookahead() == '/' as i32 {
            // Explore past comments without moving the newline's mark_end.
            let comment = eat_comment(lexer, false);
            if let StopToken(token) = comment {
                if !has_seen_single_comment {
                    lexer.mark_end();
                    return StopToken(token);
                }
            } else if comment == StopEndOfFile {
                return StopEndOfFile;
            } else if comment == ContinueSlashConsumed {
                return ContinueSlashConsumed;
            } else if lexer.lookahead() == '/' as i32 {
                // Preserve C's ordering: a consumed slash has already returned above.
                has_seen_single_comment = true;
                while lexer.lookahead() != '\n' as i32 && lexer.lookahead() != 0 {
                    lexer.advance(true);
                }
            } else if is_space(lexer.lookahead()) {
                return StopNothing;
            }
            while is_space(lexer.lookahead()) {
                lexer.advance(true);
            }
        }
        if eat_operators(lexer, valid_symbols, false, false, 0).is_some() {
            return StopNothing;
        } else {
            ws_directive = StopToken(ImplicitSemi);
        }
    }

    if ws_directive == ContinueToken(ImplicitSemi)
        && matches!(
            lookahead,
            0x3f | 0x3a | 0x7b | 0x26 | 0x7c | 0x5e | 0x3c | 0x3e
        )
    {
        return ContinueNothing;
    }

    if semi_is_valid && ws_directive != ContinueNothing {
        // Prefer directives that can be the first member of a type body.
        let directive_is_valid = valid_symbols[DirectiveIf as usize]
            || valid_symbols[DirectiveElseif as usize]
            || valid_symbols[DirectiveElse as usize]
            || valid_symbols[DirectiveEndif as usize];
        if lookahead == '#' as i32 && directive_is_valid {
            return ContinueNothing;
        }
        return ws_directive;
    }
    ContinueNothing
}

const DIRECTIVES: [&[u8]; 4] = [b"if", b"elseif", b"else", b"endif"];
const DIRECTIVE_SYMBOLS: [Token; 4] = [DirectiveIf, DirectiveElseif, DirectiveElse, DirectiveEndif];

fn find_possible_compiler_directive(lexer: &mut dyn Lexer) -> Token {
    let mut possible_directives = [true; 4];
    let mut str_idx = 0;
    let mut full_match = None;
    loop {
        for (dir_idx, possible) in possible_directives.iter_mut().enumerate() {
            if !*possible {
                continue;
            }
            let text = DIRECTIVES[dir_idx];
            // At EOF C advances once past a matching string's NUL and then reads
            // out of bounds. Safely eliminate that candidate on the next iteration.
            if str_idx > text.len() {
                *possible = false;
                continue;
            }
            let expected_char = text.get(str_idx).copied().unwrap_or(0) as i32;
            if expected_char == 0 {
                full_match = Some(dir_idx);
                lexer.mark_end();
            }
            if expected_char != lexer.lookahead() {
                *possible = false;
            }
        }
        if !possible_directives.contains(&true) {
            break;
        }
        lexer.advance(false);
        str_idx += 1;
    }
    full_match.map_or(HashSymbol, |index| DIRECTIVE_SYMBOLS[index])
}

/// The scanner's entire persistent payload is one C-width hash count.
#[derive(Default)]
pub(crate) struct Scanner {
    ongoing_raw_str_hash_count: u32,
}

impl Scanner {
    fn eat_raw_str_part(&mut self, lexer: &mut dyn Lexer, valid_symbols: &[bool]) -> Option<Token> {
        let mut hash_count = self.ongoing_raw_str_hash_count;
        if !valid_symbols[RawStrPart as usize] {
            return None;
        } else if hash_count == 0 {
            while lexer.lookahead() == '#' as i32 {
                hash_count = hash_count.wrapping_add(1);
                lexer.advance(false);
            }
            if hash_count == 0 {
                return None;
            }
            if lexer.lookahead() == '"' as i32 {
                lexer.advance(false);
            } else if hash_count == 1 {
                lexer.mark_end();
                return Some(find_possible_compiler_directive(lexer));
            } else {
                return None;
            }
        } else if !valid_symbols[RawStrContinuingIndicator as usize] {
            return None;
        }

        while lexer.lookahead() != 0 {
            let mut last_char = 0u8;
            lexer.mark_end();
            while lexer.lookahead() != '#' as i32 && lexer.lookahead() != 0 {
                // C stores this in uint8_t, including for non-ASCII lookahead.
                last_char = lexer.lookahead() as u8;
                lexer.advance(false);
                if last_char != b'\\' || lexer.lookahead() == '\\' as i32 {
                    lexer.mark_end();
                }
            }
            let mut current_hash_count = 0u32;
            while lexer.lookahead() == '#' as i32 && current_hash_count < hash_count {
                current_hash_count = current_hash_count.wrapping_add(1);
                lexer.advance(false);
            }
            if current_hash_count == hash_count {
                if last_char == b'\\' && lexer.lookahead() == '(' as i32 {
                    self.ongoing_raw_str_hash_count = hash_count;
                    return Some(RawStrPart);
                } else if last_char == b'"' {
                    lexer.mark_end();
                    self.ongoing_raw_str_hash_count = 0;
                    return Some(RawStrEndPart);
                }
            }
        }
        None
    }
}

impl ExternalScanner for Scanner {
    fn scan(&mut self, lexer: &mut dyn Lexer, valid_symbols: &[bool]) -> bool {
        let token_is_immediate = !should_treat_as_wspace(lexer.lookahead());
        let ws_directive = eat_whitespace(lexer, valid_symbols);
        if let StopToken(token) = ws_directive {
            lexer.set_result_symbol(token as u16);
            return true;
        }
        if matches!(ws_directive, StopNothing | StopEndOfFile) {
            return false;
        }
        let has_ws_result = matches!(ws_directive, ContinueToken(_));

        let comment = if ws_directive == ContinueSlashConsumed {
            ws_directive
        } else {
            eat_comment(lexer, true)
        };
        if let StopToken(token) = comment {
            lexer.mark_end();
            lexer.set_result_symbol(token as u16);
            return true;
        }
        if comment == StopEndOfFile {
            return false;
        }

        let prior_char = if comment == ContinueSlashConsumed {
            '/' as i32
        } else {
            0
        };
        if let Some(token) = eat_operators(
            lexer,
            valid_symbols,
            !has_ws_result,
            token_is_immediate,
            prior_char,
        ) && (!has_ws_result || is_cross_semi_token(token))
        {
            lexer.set_result_symbol(token as u16);
            if has_ws_result {
                lexer.mark_end();
            }
            return true;
        }
        if let ContinueToken(token) = ws_directive {
            lexer.set_result_symbol(token as u16);
            return true;
        }

        // Keep this last: even a failed attempt consumes hashes.
        if let Some(token) = self.eat_raw_str_part(lexer, valid_symbols) {
            lexer.set_result_symbol(token as u16);
            return true;
        }
        false
    }

    fn serialize(&mut self, buffer: &mut [u8]) -> usize {
        buffer[..4].copy_from_slice(&self.ongoing_raw_str_hash_count.to_be_bytes());
        4
    }

    fn deserialize(&mut self, buffer: &[u8]) {
        if buffer.len() < 4 {
            // Unlike most scanners, C leaves the state unchanged on empty input.
            return;
        }
        // C casts signed `char` directly to uint32_t, sign-extending high bytes.
        self.ongoing_raw_str_hash_count = ((buffer[0] as i8 as u32) << 24)
            | ((buffer[1] as i8 as u32) << 16)
            | ((buffer[2] as i8 as u32) << 8)
            | (buffer[3] as i8 as u32);
    }
}

pub(crate) fn create() -> Box<dyn ExternalScanner> {
    Box::new(Scanner::default())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Eq)]
    enum Call {
        Advance(bool),
        MarkEnd,
        Result(u16),
    }

    struct TestLexer {
        input: Vec<i32>,
        position: usize,
        end: usize,
        result: u16,
        calls: Vec<(usize, Call)>,
    }

    impl TestLexer {
        fn new(input: &str) -> Self {
            Self {
                input: input.chars().map(|c| c as i32).collect(),
                position: 0,
                end: 0,
                result: u16::MAX,
                calls: Vec::new(),
            }
        }
    }

    impl Lexer for TestLexer {
        fn lookahead(&self) -> i32 {
            self.input.get(self.position).copied().unwrap_or(0)
        }
        fn result_symbol(&self) -> u16 {
            self.result
        }
        fn set_result_symbol(&mut self, symbol: u16) {
            self.calls.push((self.position, Call::Result(symbol)));
            self.result = symbol;
        }
        fn advance(&mut self, skip: bool) {
            self.calls.push((self.position, Call::Advance(skip)));
            if self.position < self.input.len() {
                self.position += 1;
            }
        }
        fn mark_end(&mut self) {
            self.calls.push((self.position, Call::MarkEnd));
            self.end = self.position;
        }
        fn get_column(&mut self) -> u32 {
            panic!("Swift scanner does not query columns")
        }
        fn is_at_included_range_start(&self) -> bool {
            panic!("Swift scanner does not query included ranges")
        }
        fn eof(&self) -> bool {
            panic!("Swift scanner checks lookahead, not eof()")
        }
    }

    fn valid(tokens: &[Token]) -> [bool; 34] {
        let mut result = [false; 34];
        for &token in tokens {
            result[token as usize] = true;
        }
        result
    }

    #[test]
    fn serialization_and_signed_char_deserialization() {
        let mut scanner = Scanner {
            ongoing_raw_str_hash_count: 0x1234_5678,
        };
        let mut buffer = [0; 8];
        assert_eq!(scanner.serialize(&mut buffer), 4);
        assert_eq!(buffer, [0x12, 0x34, 0x56, 0x78, 0, 0, 0, 0]);
        scanner.deserialize(&[]);
        scanner.deserialize(&[0, 0, 0]);
        assert_eq!(scanner.ongoing_raw_str_hash_count, 0x1234_5678);
        scanner.deserialize(&[0, 0, 0, 0x80]);
        assert_eq!(scanner.ongoing_raw_str_hash_count, 0xffff_ff80);
        scanner.deserialize(&[0, 0, 0x80, 1]);
        assert_eq!(scanner.ongoing_raw_str_hash_count, 0xffff_8001);
        scanner.deserialize(&[0, 0x80, 0, 1]);
        assert_eq!(scanner.ongoing_raw_str_hash_count, 0xff80_0001);
        scanner.deserialize(&[0, 0, 0, 0]);
        assert_eq!(scanner.ongoing_raw_str_hash_count, 0);
    }

    #[test]
    fn fixed_operators_and_terminators() {
        for (text, symbol) in OPERATORS.into_iter().zip(OP_SYMBOLS) {
            let input = format!("{} ", std::str::from_utf8(text).unwrap());
            let mut lexer = TestLexer::new(&input);
            assert!(Scanner::default().scan(&mut lexer, &valid(&[symbol])));
            assert_eq!(lexer.result, symbol as u16, "{input}");
            assert_eq!(lexer.end, text.len(), "{input}");
            assert_eq!(lexer.position, text.len(), "{input}");
        }
        for (input, symbols, expected) in [
            (
                "as? ",
                vec![AsKeyword, AsQuest, AsBang, AsyncKeyword],
                Some(AsQuest),
            ),
            (
                "async ",
                vec![AsKeyword, AsQuest, AsBang, AsyncKeyword],
                Some(AsyncKeyword),
            ),
            ("as_name", vec![AsKeyword], Some(AsKeyword)),
            ("asé", vec![AsKeyword], Some(AsKeyword)),
            ("as1", vec![AsKeyword], None),
            ("+\u{b}", vec![PlusThenWs], Some(PlusThenWs)),
            ("+\u{a0}", vec![PlusThenWs], None),
            ("+", vec![PlusThenWs], None),
            ("->.", vec![ArrowOperator], Some(ArrowOperator)),
            (".+", vec![DotOperator], None),
            ("! ", vec![Bang, FakeTryBang], None),
        ] {
            let mut lexer = TestLexer::new(input);
            assert_eq!(
                eat_operators(&mut lexer, &valid(&symbols), true, true, 0),
                expected,
                "{input}",
            );
        }
    }

    #[test]
    fn double_optional_requires_immediacy() {
        for (input, symbols, expected) in [
            ("??", vec![DoubleOptional], Some(DoubleOptional)),
            (" ??", vec![DoubleOptional], None),
            (
                " ??",
                vec![DoubleOptional, NilCoalescingOperator],
                Some(NilCoalescingOperator),
            ),
            (
                "??",
                vec![DoubleOptional, NilCoalescingOperator],
                Some(DoubleOptional),
            ),
        ] {
            let mut lexer = TestLexer::new(input);
            let found = Scanner::default().scan(&mut lexer, &valid(&symbols));
            assert_eq!(found, expected.is_some(), "{input}");
            if let Some(symbol) = expected {
                assert_eq!(lexer.result, symbol as u16);
            }
        }
    }

    #[test]
    fn custom_operators_preserve_reserved_matches_and_angle_boundary() {
        for text in RESERVED_OPS {
            let input = format!("{}x", std::str::from_utf8(text).unwrap());
            let mut lexer = TestLexer::new(&input);
            assert_eq!(
                eat_operators(&mut lexer, &valid(&[CustomOperator]), true, true, 0),
                None,
                "{input}",
            );
        }
        for (input, end) in [
            ("+<Type", 1),
            ("+< Type", 2),
            ("/+++x", 4),
            ("⊕\u{301}x", 2),
        ] {
            let mut lexer = TestLexer::new(input);
            assert!(Scanner::default().scan(&mut lexer, &valid(&[CustomOperator])));
            assert_eq!(lexer.result, CustomOperator as u16);
            assert_eq!(lexer.end, end, "{input}");
        }
        assert!(!is_legal_custom_operator(0, 0x301, 0x301));
        assert!(!is_legal_custom_operator(1, '/' as i32, '/' as i32));
        assert!(!is_legal_custom_operator(1, '+' as i32, '.' as i32));
    }

    #[test]
    fn nested_comments_and_exact_mark_order() {
        let mut lexer = TestLexer::new("/**/x");
        assert!(Scanner::default().scan(&mut lexer, &valid(&[])));
        assert_eq!(
            lexer.calls,
            [
                (0, Call::Advance(false)),
                (1, Call::Advance(false)),
                (2, Call::Advance(false)),
                (3, Call::Advance(false)),
                (4, Call::MarkEnd),
                (4, Call::MarkEnd),
                (4, Call::Result(BlockComment as u16)),
            ]
        );
        let text = "/* a /* b */ c */";
        let mut lexer = TestLexer::new(text);
        assert!(Scanner::default().scan(&mut lexer, &valid(&[BlockComment])));
        assert_eq!(lexer.end, text.len());
        let mut lexer = TestLexer::new("/* a /* b */");
        assert!(!Scanner::default().scan(&mut lexer, &valid(&[BlockComment])));
        assert_eq!(lexer.end, 0);

        let mut lexer = TestLexer::new("\n//comment");
        assert!(!Scanner::default().scan(&mut lexer, &valid(&[ImplicitSemi, ExplicitSemi])));
        assert_eq!(lexer.position, 2); // The first slash is consumed, the second is not.
    }

    #[test]
    fn semicolon_and_cross_newline_operators() {
        let mut lexer = TestLexer::new(" ;x");
        assert!(Scanner::default().scan(&mut lexer, &valid(&[ImplicitSemi, ExplicitSemi])));
        // The explicit semicolon deliberately doesn't mark its own end.
        assert_eq!(
            lexer.calls,
            [
                (0, Call::Advance(true)),
                (1, Call::MarkEnd),
                (1, Call::Advance(false)),
                (2, Call::Result(ExplicitSemi as u16)),
            ]
        );
        for (input, symbol, end) in [
            ("\n= x", EqualSign, 2),
            ("\n!x", ImplicitSemi, 1),
            ("\nx", ImplicitSemi, 1),
        ] {
            let mut lexer = TestLexer::new(input);
            assert!(Scanner::default().scan(
                &mut lexer,
                &valid(&[ImplicitSemi, ExplicitSemi, EqualSign, Bang]),
            ));
            assert_eq!(lexer.result, symbol as u16);
            assert_eq!(lexer.end, end);
        }
        for c in ['?', ':', '{', '&', '|', '^', '<', '>'] {
            let mut lexer = TestLexer::new(&format!("\n{c}"));
            assert!(!Scanner::default().scan(&mut lexer, &valid(&[ImplicitSemi, ExplicitSemi])));
        }
    }

    #[test]
    fn directive_prefixes_and_newline_preference() {
        for (input, expected, end) in [
            ("#if DEBUG", DirectiveIf, 3),
            ("#ifdef", DirectiveIf, 3),
            ("#elseifx", DirectiveElseif, 7),
            ("#elsewhere", DirectiveElse, 5),
            ("#endif", DirectiveEndif, 6),
            ("#other", HashSymbol, 1),
        ] {
            let mut lexer = TestLexer::new(input);
            assert!(Scanner::default().scan(&mut lexer, &valid(&[RawStrPart])));
            assert_eq!(lexer.result, expected as u16);
            assert_eq!(lexer.end, end);
        }
        let mut lexer = TestLexer::new("\n#if DEBUG");
        assert!(Scanner::default().scan(
            &mut lexer,
            &valid(&[RawStrPart, ImplicitSemi, ExplicitSemi, DirectiveIf]),
        ));
        assert_eq!(lexer.result, DirectiveIf as u16);
        let mut lexer = TestLexer::new("\n#if DEBUG");
        assert!(Scanner::default().scan(
            &mut lexer,
            &valid(&[RawStrPart, ImplicitSemi, ExplicitSemi]),
        ));
        assert_eq!(lexer.result, ImplicitSemi as u16);
        assert_eq!(lexer.end, 1);
    }

    #[test]
    fn raw_string_interpolation_and_continuation() {
        let mut scanner = Scanner::default();
        let mut lexer = TestLexer::new("#\"a\\#(x)");
        assert!(scanner.scan(&mut lexer, &valid(&[RawStrPart])));
        assert_eq!(lexer.result, RawStrPart as u16);
        assert_eq!(lexer.position, 5);
        assert_eq!(lexer.end, 3);
        assert_eq!(scanner.ongoing_raw_str_hash_count, 1);
        let mut lexer = TestLexer::new("b\"# tail");
        assert!(!scanner.scan(&mut lexer, &valid(&[RawStrPart])));
        assert_eq!(lexer.position, 0);
        assert!(scanner.scan(&mut lexer, &valid(&[RawStrPart, RawStrContinuingIndicator])));
        assert_eq!(lexer.result, RawStrEndPart as u16);
        assert_eq!(lexer.end, 3);
        assert_eq!(scanner.ongoing_raw_str_hash_count, 0);

        for (input, expected, end) in [
            ("##\"a\"##x", RawStrEndPart, 7),
            ("#\"a\u{122}#", RawStrEndPart, 5), // uint8_t last_char aliases a quote.
            ("#\"a\\\\#(x)", RawStrPart, 4),    // Adjacent backslashes mark the first one.
        ] {
            let mut lexer = TestLexer::new(input);
            assert!(Scanner::default().scan(&mut lexer, &valid(&[RawStrPart])));
            assert_eq!(lexer.result, expected as u16);
            assert_eq!(lexer.end, end);
        }
        let mut lexer = TestLexer::new("##\"unterminated");
        assert!(!scanner.scan(&mut lexer, &valid(&[RawStrPart])));
        assert_eq!(scanner.ongoing_raw_str_hash_count, 0);
    }
}
