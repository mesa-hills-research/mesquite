//! The Elixir external scanner, translated from `src/scanner.c`.

use tree_sitter_language::{ExternalScanner, Lexer};

// In the same order as the C TokenType enum and grammar.externals.
#[derive(Clone, Copy)]
#[repr(u16)]
enum Token {
    QuotedContentISingle,
    QuotedContentIDouble,
    QuotedContentIHeredocSingle,
    QuotedContentIHeredocDouble,
    QuotedContentIParenthesis,
    QuotedContentICurly,
    QuotedContentISquare,
    QuotedContentIAngle,
    QuotedContentIBar,
    QuotedContentISlash,
    QuotedContentSingle,
    QuotedContentDouble,
    QuotedContentHeredocSingle,
    QuotedContentHeredocDouble,
    QuotedContentParenthesis,
    QuotedContentCurly,
    QuotedContentSquare,
    QuotedContentAngle,
    QuotedContentBar,
    QuotedContentSlash,
    NewlineBeforeDo,
    NewlineBeforeBinaryOperator,
    NewlineBeforeComment,
    BeforeUnaryOperator,
    NotIn,
    QuotedAtomStart,
}

#[derive(Clone, Copy)]
struct QuotedContentInfo {
    token: Token,
    supports_interpolation: bool,
    end_delimiter: i32,
    delimiter_length: u8,
}

impl QuotedContentInfo {
    const fn new(token: Token, supports_interpolation: bool, delimiter: u8, length: u8) -> Self {
        Self {
            token,
            supports_interpolation,
            end_delimiter: delimiter as i32,
            delimiter_length: length,
        }
    }
}

const QUOTED_CONTENT_INFOS: [QuotedContentInfo; 20] = [
    QuotedContentInfo::new(Token::QuotedContentISingle, true, b'\'', 1),
    QuotedContentInfo::new(Token::QuotedContentIDouble, true, b'"', 1),
    QuotedContentInfo::new(Token::QuotedContentIHeredocSingle, true, b'\'', 3),
    QuotedContentInfo::new(Token::QuotedContentIHeredocDouble, true, b'"', 3),
    QuotedContentInfo::new(Token::QuotedContentIParenthesis, true, b')', 1),
    QuotedContentInfo::new(Token::QuotedContentICurly, true, b'}', 1),
    QuotedContentInfo::new(Token::QuotedContentISquare, true, b']', 1),
    QuotedContentInfo::new(Token::QuotedContentIAngle, true, b'>', 1),
    QuotedContentInfo::new(Token::QuotedContentIBar, true, b'|', 1),
    QuotedContentInfo::new(Token::QuotedContentISlash, true, b'/', 1),
    QuotedContentInfo::new(Token::QuotedContentSingle, false, b'\'', 1),
    QuotedContentInfo::new(Token::QuotedContentDouble, false, b'"', 1),
    QuotedContentInfo::new(Token::QuotedContentHeredocSingle, false, b'\'', 3),
    QuotedContentInfo::new(Token::QuotedContentHeredocDouble, false, b'"', 3),
    QuotedContentInfo::new(Token::QuotedContentParenthesis, false, b')', 1),
    QuotedContentInfo::new(Token::QuotedContentCurly, false, b'}', 1),
    QuotedContentInfo::new(Token::QuotedContentSquare, false, b']', 1),
    QuotedContentInfo::new(Token::QuotedContentAngle, false, b'>', 1),
    QuotedContentInfo::new(Token::QuotedContentBar, false, b'|', 1),
    QuotedContentInfo::new(Token::QuotedContentSlash, false, b'/', 1),
];

fn is_whitespace(c: i32) -> bool {
    matches!(c, 0x20 | 0x09 | 0x0a | 0x0d)
}

fn is_inline_whitespace(c: i32) -> bool {
    matches!(c, 0x20 | 0x09)
}

fn is_newline(c: i32) -> bool {
    // CRLF deliberately counts as two newlines.
    matches!(c, 0x0a | 0x0d)
}

fn is_digit(c: i32) -> bool {
    (i32::from(b'0')..=i32::from(b'9')).contains(&c)
}

fn consume(lexer: &mut dyn Lexer, c: u8) -> bool {
    if lexer.lookahead() == i32::from(c) {
        lexer.advance(false);
        true
    } else {
        false
    }
}

fn consume_word(lexer: &mut dyn Lexer, word: &[u8]) -> bool {
    word.iter().all(|&c| consume(lexer, c))
}

fn check_keyword_end(lexer: &mut dyn Lexer) -> bool {
    consume(lexer, b':') && is_whitespace(lexer.lookahead())
}

fn check_operator_end(lexer: &mut dyn Lexer) -> bool {
    if lexer.lookahead() == i32::from(b':') {
        return !check_keyword_end(lexer);
    }
    while is_inline_whitespace(lexer.lookahead()) {
        lexer.advance(false);
    }
    // An operator identifier with an arity is not a binary operator here.
    if consume(lexer, b'/') {
        while is_whitespace(lexer.lookahead()) {
            lexer.advance(false);
        }
        if is_digit(lexer.lookahead()) {
            return false;
        }
    }
    true
}

fn is_token_end(c: i32) -> bool {
    const TERMINATORS: &[u8] = b"@.+-^-*/<>|~=&\\%{}[]()\"',;#";
    // This deliberately neither recognizes EOF nor attempts Unicode classes.
    TERMINATORS
        .iter()
        .any(|&terminator| c == i32::from(terminator))
        || is_whitespace(c)
}

fn find_quoted_token_info(valid_symbols: &[bool]) -> Option<&'static QuotedContentInfo> {
    // This specific pair is C's error-recovery sentinel, not a general check
    // that exactly one of the twenty quoted-content tokens is enabled.
    if valid_symbols[Token::QuotedContentISingle as usize]
        && valid_symbols[Token::QuotedContentIDouble as usize]
    {
        return None;
    }
    QUOTED_CONTENT_INFOS
        .iter()
        .find(|info| valid_symbols[info.token as usize])
}

fn scan_quoted_content(lexer: &mut dyn Lexer, info: &QuotedContentInfo) -> bool {
    lexer.set_result_symbol(info.token as u16);
    let is_heredoc = info.delimiter_length == 3;
    let mut has_content = false;

    loop {
        let mut newline = false;
        if is_newline(lexer.lookahead()) {
            lexer.advance(false);
            has_content = true;
            newline = true;
            while is_whitespace(lexer.lookahead()) {
                lexer.advance(false);
            }
        }

        lexer.mark_end();
        if lexer.lookahead() == info.end_delimiter {
            let mut length = 1;
            while length < info.delimiter_length {
                lexer.advance(false);
                if lexer.lookahead() == info.end_delimiter {
                    length += 1;
                } else {
                    break;
                }
            }
            if length == info.delimiter_length && (!is_heredoc || newline) {
                return has_content;
            }
        } else if consume(lexer, b'#') {
            if info.supports_interpolation && lexer.lookahead() == i32::from(b'{') {
                return has_content;
            }
        } else if consume(lexer, b'\\') {
            if is_heredoc && lexer.lookahead() == i32::from(b'\n') {
                // Leave LF for the next iteration, even in interpolated content,
                // so that a following heredoc delimiter can be recognized.
            } else if info.supports_interpolation || lexer.lookahead() == info.end_delimiter {
                return has_content;
            }
        } else if lexer.lookahead() == 0 {
            // Unterminated quotes still emit all content scanned so far.
            return has_content;
        } else {
            lexer.advance(false);
        }
        has_content = true;
    }
}

fn scan_not_in(lexer: &mut dyn Lexer) -> bool {
    if !consume_word(lexer, b"not") {
        return false;
    }
    // C intentionally permits zero spaces: `notin` follows the same path.
    while is_inline_whitespace(lexer.lookahead()) {
        lexer.advance(false);
    }
    consume_word(lexer, b"in") && is_token_end(lexer.lookahead())
}

fn scan_binary_operator(lexer: &mut dyn Lexer) -> bool {
    match lexer.lookahead() {
        // &&, &&&, ++, +++
        c if c == i32::from(b'&') || c == i32::from(b'+') => {
            lexer.advance(false);
            if !consume(lexer, c as u8) {
                return false;
            }
            consume(lexer, c as u8);
        }
        // =, ==, ===, =~, =>
        c if c == i32::from(b'=') => {
            lexer.advance(false);
            if consume(lexer, b'=') {
                consume(lexer, b'=');
            } else if lexer.lookahead() == i32::from(b'~') || lexer.lookahead() == i32::from(b'>') {
                lexer.advance(false);
            }
        }
        // :: (but not the ::: atom)
        c if c == i32::from(b':') => {
            lexer.advance(false);
            if !consume(lexer, b':') || lexer.lookahead() == i32::from(b':') {
                return false;
            }
        }
        // --, ---, ->
        c if c == i32::from(b'-') => {
            lexer.advance(false);
            if consume(lexer, b'-') {
                consume(lexer, b'-');
            } else if !consume(lexer, b'>') {
                return false;
            }
        }
        // <, <=, <-, <>, <~, <~>, <|>, <<<, <<~
        c if c == i32::from(b'<') => {
            lexer.advance(false);
            if lexer.lookahead() == i32::from(b'=')
                || lexer.lookahead() == i32::from(b'-')
                || lexer.lookahead() == i32::from(b'>')
            {
                lexer.advance(false);
            } else if consume(lexer, b'~') {
                consume(lexer, b'>');
            } else if consume(lexer, b'|') {
                if !consume(lexer, b'>') {
                    return false;
                }
            } else if consume(lexer, b'<') && !consume(lexer, b'<') && !consume(lexer, b'~') {
                return false;
            }
        }
        // >, >=, >>>
        c if c == i32::from(b'>') => {
            lexer.advance(false);
            if !consume(lexer, b'=') && consume(lexer, b'>') && !consume(lexer, b'>') {
                return false;
            }
        }
        // ^^^
        c if c == i32::from(b'^') => {
            lexer.advance(false);
            if !consume(lexer, b'^') || !consume(lexer, b'^') {
                return false;
            }
        }
        // !=, !==
        c if c == i32::from(b'!') => {
            lexer.advance(false);
            if !consume(lexer, b'=') {
                return false;
            }
            consume(lexer, b'=');
        }
        // ~>, ~>>
        c if c == i32::from(b'~') => {
            lexer.advance(false);
            if !consume(lexer, b'>') {
                return false;
            }
            consume(lexer, b'>');
        }
        // |, ||, |||, |>
        c if c == i32::from(b'|') => {
            lexer.advance(false);
            if consume(lexer, b'|') {
                consume(lexer, b'|');
            } else {
                consume(lexer, b'>');
            }
        }
        // *, **, /, //
        c if c == i32::from(b'*') || c == i32::from(b'/') => {
            lexer.advance(false);
            consume(lexer, c as u8);
        }
        // ., .. (but not the ... identifier)
        c if c == i32::from(b'.') => {
            lexer.advance(false);
            if consume(lexer, b'.') && lexer.lookahead() == i32::from(b'.') {
                return false;
            }
        }
        // Two backslashes.
        c if c == i32::from(b'\\') => {
            lexer.advance(false);
            if !consume(lexer, b'\\') {
                return false;
            }
        }
        c if c == i32::from(b'w') => {
            if !consume_word(lexer, b"when") || !is_token_end(lexer.lookahead()) {
                return false;
            }
        }
        c if c == i32::from(b'a') => {
            if !consume_word(lexer, b"and") || !is_token_end(lexer.lookahead()) {
                return false;
            }
        }
        c if c == i32::from(b'o') => {
            if !consume_word(lexer, b"or") || !is_token_end(lexer.lookahead()) {
                return false;
            }
        }
        c if c == i32::from(b'i') => {
            if !consume_word(lexer, b"in") || !is_token_end(lexer.lookahead()) {
                return false;
            }
        }
        c if c == i32::from(b'n') => {
            if !scan_not_in(lexer) {
                return false;
            }
        }
        _ => return false,
    }
    check_operator_end(lexer)
}

fn scan_newline(lexer: &mut dyn Lexer, valid_symbols: &[bool]) -> bool {
    lexer.advance(false);
    while is_whitespace(lexer.lookahead()) {
        lexer.advance(false);
    }
    // Include all whitespace following the newline in the token, but not any
    // operator/keyword consumed by the subsequent lookahead.
    lexer.mark_end();

    if lexer.lookahead() == i32::from(b'#') {
        // Deliberately not gated by NewlineBeforeComment's validity.
        lexer.set_result_symbol(Token::NewlineBeforeComment as u16);
        return true;
    }
    if lexer.lookahead() == i32::from(b'd') && valid_symbols[Token::NewlineBeforeDo as usize] {
        lexer.set_result_symbol(Token::NewlineBeforeDo as u16);
        lexer.advance(false);
        return consume(lexer, b'o') && is_token_end(lexer.lookahead());
    }
    if valid_symbols[Token::NewlineBeforeBinaryOperator as usize] {
        lexer.set_result_symbol(Token::NewlineBeforeBinaryOperator as u16);
        return scan_binary_operator(lexer);
    }
    false
}

/// C uses a null payload: the scanner has no persistent state.
pub(crate) struct Scanner;

impl ExternalScanner for Scanner {
    fn scan(&mut self, lexer: &mut dyn Lexer, valid_symbols: &[bool]) -> bool {
        if let Some(info) = find_quoted_token_info(valid_symbols) {
            return scan_quoted_content(lexer, info);
        }

        let mut skipped_whitespace = false;
        while is_inline_whitespace(lexer.lookahead()) {
            skipped_whitespace = true;
            lexer.advance(true);
        }

        if is_newline(lexer.lookahead())
            && (valid_symbols[Token::NewlineBeforeDo as usize]
                || valid_symbols[Token::NewlineBeforeBinaryOperator as usize]
                || valid_symbols[Token::NewlineBeforeComment as usize])
        {
            return scan_newline(lexer, valid_symbols);
        }

        match lexer.lookahead() {
            c if c == i32::from(b'+')
                && skipped_whitespace
                && valid_symbols[Token::BeforeUnaryOperator as usize] =>
            {
                lexer.mark_end();
                lexer.advance(false);
                if lexer.lookahead() == i32::from(b'+')
                    || lexer.lookahead() == i32::from(b':')
                    || lexer.lookahead() == i32::from(b'/')
                    || is_whitespace(lexer.lookahead())
                {
                    return false;
                }
                lexer.set_result_symbol(Token::BeforeUnaryOperator as u16);
                return true;
            }
            c if c == i32::from(b'-')
                && skipped_whitespace
                && valid_symbols[Token::BeforeUnaryOperator as usize] =>
            {
                lexer.mark_end();
                // Unlike unary +, C sets this symbol even if the scan fails.
                lexer.set_result_symbol(Token::BeforeUnaryOperator as u16);
                lexer.advance(false);
                if lexer.lookahead() == i32::from(b'-')
                    || lexer.lookahead() == i32::from(b'>')
                    || lexer.lookahead() == i32::from(b':')
                    || lexer.lookahead() == i32::from(b'/')
                    || is_whitespace(lexer.lookahead())
                {
                    return false;
                }
                return true;
            }
            c if c == i32::from(b'n') && valid_symbols[Token::NotIn as usize] => {
                lexer.set_result_symbol(Token::NotIn as u16);
                return scan_not_in(lexer);
            }
            c if c == i32::from(b':') && valid_symbols[Token::QuotedAtomStart as usize] => {
                lexer.advance(false);
                lexer.mark_end();
                lexer.set_result_symbol(Token::QuotedAtomStart as u16);
                return lexer.lookahead() == i32::from(b'"')
                    || lexer.lookahead() == i32::from(b'\'');
            }
            _ => {}
        }
        false
    }

    fn serialize(&mut self, _buffer: &mut [u8]) -> usize {
        0
    }

    fn deserialize(&mut self, _buffer: &[u8]) {}
}

pub(crate) fn create() -> Box<dyn ExternalScanner> {
    Box::new(Scanner)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Eq)]
    enum Event {
        Advance(usize, bool),
        MarkEnd(usize),
        Symbol(u16),
    }

    struct TestLexer {
        input: Vec<i32>,
        position: usize,
        end: Option<usize>,
        symbol: u16,
        events: Vec<Event>,
    }

    impl TestLexer {
        fn new(input: &str) -> Self {
            Self {
                input: input.chars().map(|c| c as i32).collect(),
                position: 0,
                end: None,
                symbol: u16::MAX,
                events: Vec::new(),
            }
        }
    }

    impl Lexer for TestLexer {
        fn lookahead(&self) -> i32 {
            self.input.get(self.position).copied().unwrap_or(0)
        }

        fn result_symbol(&self) -> u16 {
            self.symbol
        }

        fn set_result_symbol(&mut self, symbol: u16) {
            self.symbol = symbol;
            self.events.push(Event::Symbol(symbol));
        }

        fn advance(&mut self, skip: bool) {
            assert!(
                self.position < self.input.len(),
                "unexpected advance at EOF"
            );
            self.events.push(Event::Advance(self.position, skip));
            self.position += 1;
        }

        fn mark_end(&mut self) {
            self.end = Some(self.position);
            self.events.push(Event::MarkEnd(self.position));
        }

        fn get_column(&mut self) -> u32 {
            panic!("the Elixir scanner does not request columns")
        }

        fn is_at_included_range_start(&self) -> bool {
            panic!("the Elixir scanner does not request included-range starts")
        }

        fn eof(&self) -> bool {
            panic!("the Elixir scanner tests lookahead == 0, not eof()")
        }
    }

    fn scan(input: &str, tokens: &[Token]) -> (bool, TestLexer) {
        let mut valid = [false; 26];
        for &token in tokens {
            valid[token as usize] = true;
        }
        let mut lexer = TestLexer::new(input);
        let accepted = Scanner.scan(&mut lexer, &valid);
        (accepted, lexer)
    }

    #[test]
    fn scanner_has_no_serialized_state() {
        let mut scanner = create();
        let mut buffer = [0x5a; 1024];
        scanner.deserialize(&[1, 2, 3]);
        assert_eq!(scanner.serialize(&mut buffer), 0);
        assert_eq!(buffer, [0x5a; 1024]);
        scanner.deserialize(&[]);
        assert_eq!(scanner.serialize(&mut []), 0);
    }

    #[test]
    fn all_quote_delimiters_have_the_original_boundaries() {
        for info in QUOTED_CONTENT_INFOS {
            let delimiter = char::from_u32(info.end_delimiter as u32).unwrap();
            let (input, end, position) = if info.delimiter_length == 3 {
                (format!("x\n  {delimiter}{delimiter}{delimiter}"), 4, 6)
            } else {
                (format!("x{delimiter}"), 1, 1)
            };
            let (accepted, lexer) = scan(&input, &[info.token]);
            assert!(accepted, "{input:?}");
            assert_eq!(lexer.symbol, info.token as u16);
            assert_eq!(lexer.end, Some(end), "{input:?}");
            assert_eq!(lexer.position, position, "{input:?}");
            assert!(
                lexer
                    .events
                    .iter()
                    .all(|e| !matches!(e, Event::Advance(_, true)))
            );
        }
        let (accepted, lexer) = scan("'", &[Token::QuotedContentISingle]);
        assert!(!accepted);
        assert_eq!(lexer.position, 0);
        assert_eq!(lexer.end, Some(0));
    }

    #[test]
    fn interpolation_and_escape_lookahead_do_not_extend_the_token() {
        for (input, token, accepted, end, position) in [
            ("a#{b}'", Token::QuotedContentISingle, true, 1, 2),
            ("#{b}'", Token::QuotedContentISingle, false, 0, 1),
            ("a#{b}'", Token::QuotedContentSingle, true, 5, 5),
            ("a\\x'", Token::QuotedContentISingle, true, 1, 2),
            ("a\\x'", Token::QuotedContentSingle, true, 3, 3),
            ("a\\'", Token::QuotedContentSingle, true, 1, 2),
            ("unterminated", Token::QuotedContentIDouble, true, 12, 12),
            ("", Token::QuotedContentIDouble, false, 0, 0),
            ("x\0ignored", Token::QuotedContentIDouble, true, 1, 1),
        ] {
            let (result, lexer) = scan(input, &[token]);
            assert_eq!(result, accepted, "{input:?}");
            assert_eq!(lexer.end, Some(end), "{input:?}");
            assert_eq!(lexer.position, position, "{input:?}");
        }
    }

    #[test]
    fn heredocs_require_a_newline_and_preserve_escaped_lf() {
        for (input, accepted, end, position) in [
            ("'''x", true, 4, 4),
            ("a\n ''x", true, 6, 6),
            ("\\\n'''", true, 2, 4),
            ("\\\r'''", false, 0, 1),
            ("\r\n\t '''", true, 4, 6),
        ] {
            let (result, lexer) = scan(input, &[Token::QuotedContentIHeredocSingle]);
            assert_eq!(result, accepted, "{input:?}");
            assert_eq!(lexer.end, Some(end), "{input:?}");
            assert_eq!(lexer.position, position, "{input:?}");
        }
        let (accepted, lexer) = scan("\n  '''", &[Token::QuotedContentIHeredocSingle]);
        assert!(accepted);
        assert_eq!(
            lexer.events,
            [
                Event::Symbol(Token::QuotedContentIHeredocSingle as u16),
                Event::Advance(0, false),
                Event::Advance(1, false),
                Event::Advance(2, false),
                Event::MarkEnd(3),
                Event::Advance(3, false),
                Event::Advance(4, false),
            ]
        );
    }

    #[test]
    fn quote_selection_uses_only_the_specific_recovery_sentinel() {
        let (accepted, lexer) = scan(
            "abc",
            &[Token::QuotedContentISingle, Token::QuotedContentIDouble],
        );
        assert!(!accepted);
        assert!(lexer.events.is_empty());

        let (accepted, lexer) = scan(
            "abc'",
            &[Token::QuotedContentISingle, Token::QuotedContentSingle],
        );
        assert!(accepted);
        assert_eq!(lexer.symbol, Token::QuotedContentISingle as u16);
    }

    #[test]
    fn all_newline_binary_operators_and_their_rejected_suffixes() {
        let operators = [
            "&&", "&&&", "=", "==", "===", "=~", "=>", "::", "++", "+++", "--", "---", "->", "<",
            "<=", "<-", "<>", "<~", "<~>", "<|>", "<<<", "<<~", ">", ">=", ">>>", "^^^", "!=",
            "!==", "~>", "~>>", "|", "||", "|||", "|>", "*", "**", "/", "//", ".", "..", "\\\\",
            "when", "and", "or", "in", "not in", "not\tin", "notin",
        ];
        for operator in operators {
            let input = format!("\n \t{operator} x");
            let (accepted, lexer) = scan(&input, &[Token::NewlineBeforeBinaryOperator]);
            assert!(accepted, "{input:?}");
            assert_eq!(lexer.end, Some(3), "{input:?}");
            assert_eq!(lexer.position, input.len() - 1, "{input:?}");
            assert_eq!(lexer.symbol, Token::NewlineBeforeBinaryOperator as u16);
            for suffix in [": x", " / \n2"] {
                let input = format!("\n{operator}{suffix}");
                assert!(
                    !scan(&input, &[Token::NewlineBeforeBinaryOperator]).0,
                    "{input:?}"
                );
            }
        }
        for input in [
            "\n&",
            "\n+",
            "\n-",
            "\n<|",
            "\n<<",
            "\n>>",
            "\n^^",
            "\n!",
            "\n~",
            "\n\\",
            "\n...",
            "\n::: ",
            "\nwhenx",
            "\nand",
            "\nin:",
            "\nnot\nin ",
        ] {
            assert!(
                !scan(input, &[Token::NewlineBeforeBinaryOperator]).0,
                "{input:?}"
            );
        }
        // Symbolic operators can end at EOF; word operators cannot.
        assert!(scan("\n==", &[Token::NewlineBeforeBinaryOperator]).0);
        assert!(!scan("\nwhen", &[Token::NewlineBeforeBinaryOperator]).0);
        // No keyword whitespace: the colon belongs to the following expression.
        assert!(scan("\n==:x", &[Token::NewlineBeforeBinaryOperator]).0);
        // The arity heuristic only recognizes ASCII digits.
        assert!(scan("\n== / ٢", &[Token::NewlineBeforeBinaryOperator]).0);
    }

    #[test]
    fn newline_comments_do_and_whitespace_callback_order() {
        let (accepted, lexer) = scan(" \r\n\t#x", &[Token::NewlineBeforeDo]);
        assert!(accepted);
        // A comment is emitted even with only NewlineBeforeDo enabled.
        assert_eq!(
            lexer.events,
            [
                Event::Advance(0, true),
                Event::Advance(1, false),
                Event::Advance(2, false),
                Event::Advance(3, false),
                Event::MarkEnd(4),
                Event::Symbol(Token::NewlineBeforeComment as u16),
            ]
        );
        for (input, accepted) in [
            ("\ndo ", true),
            ("\ndo#", true),
            ("\ndo", false),
            ("\ndo:", false),
            ("\ndog", false),
        ] {
            assert_eq!(
                scan(input, &[Token::NewlineBeforeDo]).0,
                accepted,
                "{input:?}"
            );
        }
        assert!(!scan("\ndo ", &[Token::NewlineBeforeBinaryOperator]).0);
        for input in ["\u{b}\n#", "\u{c}\n#", "\u{a0}\n#"] {
            assert!(!scan(input, &[Token::NewlineBeforeComment]).0);
        }
    }

    #[test]
    fn unary_operator_symbol_assignment_order_is_asymmetric() {
        let (accepted, lexer) = scan(" +x", &[Token::BeforeUnaryOperator]);
        assert!(accepted);
        assert_eq!(
            lexer.events,
            [
                Event::Advance(0, true),
                Event::MarkEnd(1),
                Event::Advance(1, false),
                Event::Symbol(Token::BeforeUnaryOperator as u16),
            ]
        );
        let (accepted, lexer) = scan(" -x", &[Token::BeforeUnaryOperator]);
        assert!(accepted);
        assert_eq!(
            lexer.events,
            [
                Event::Advance(0, true),
                Event::MarkEnd(1),
                Event::Symbol(Token::BeforeUnaryOperator as u16),
                Event::Advance(1, false),
            ]
        );
        for (input, symbol) in [
            (" ++", u16::MAX),
            (" + ", u16::MAX),
            (" ->", Token::BeforeUnaryOperator as u16),
            (" -/", Token::BeforeUnaryOperator as u16),
        ] {
            let (accepted, lexer) = scan(input, &[Token::BeforeUnaryOperator]);
            assert!(!accepted);
            assert_eq!(lexer.symbol, symbol);
        }
        assert!(!scan("+x", &[Token::BeforeUnaryOperator]).0);
        assert!(scan(" +", &[Token::BeforeUnaryOperator]).0);
        assert!(scan(" -", &[Token::BeforeUnaryOperator]).0);
    }

    #[test]
    fn not_in_and_quoted_atom_start_boundaries() {
        for input in ["not in ", "not\tin#", "notin("] {
            let (accepted, lexer) = scan(input, &[Token::NotIn]);
            assert!(accepted, "{input:?}");
            assert_eq!(lexer.position, input.len() - 1);
            assert_eq!(lexer.end, None);
            assert_eq!(lexer.symbol, Token::NotIn as u16);
        }
        for input in ["not in", "not inside ", "not\nin ", "not in:"] {
            assert!(!scan(input, &[Token::NotIn]).0, "{input:?}");
        }
        for (input, accepted) in [(":\"a\"", true), (":'a'", true), (":atom", false)] {
            let (result, lexer) = scan(input, &[Token::QuotedAtomStart]);
            assert_eq!(result, accepted);
            assert_eq!(
                lexer.events,
                [
                    Event::Advance(0, false),
                    Event::MarkEnd(1),
                    Event::Symbol(Token::QuotedAtomStart as u16),
                ]
            );
        }
    }
}
