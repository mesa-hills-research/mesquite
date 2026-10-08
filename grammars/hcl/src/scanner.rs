//! The HCL external scanner, translated from `hcl/src/scanner.c`.

use tree_sitter_language::{ExternalScanner, Lexer, SERIALIZATION_BUFFER_SIZE};

// External token indices, in C's TokenType order.
const QUOTED_TEMPLATE_START: usize = 0;
const QUOTED_TEMPLATE_END: usize = 1;
const TEMPLATE_LITERAL_CHUNK: usize = 2;
const TEMPLATE_INTERPOLATION_START: usize = 3;
const TEMPLATE_INTERPOLATION_END: usize = 4;
const TEMPLATE_DIRECTIVE_START: usize = 5;
const TEMPLATE_DIRECTIVE_END: usize = 6;
const HEREDOC_IDENTIFIER: usize = 7;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
enum ContextType {
    TemplateInterpolation = 0,
    TemplateDirective = 1,
    QuotedTemplate = 2,
    HeredocTemplate = 3,
}

#[derive(Debug, PartialEq, Eq)]
struct Context {
    kind: ContextType,
    // C stores char bytes, not Unicode code points, in this string.
    heredoc_identifier: Vec<u8>,
}

/// The scanner's state (C's `payload`).
#[derive(Default)]
pub(crate) struct Scanner {
    context_stack: Vec<Context>,
}

// Wide-character predicates as in C's default locale.
fn is_space(c: i32) -> bool {
    matches!(c, 0x09..=0x0d | 0x20)
}

fn is_alphanumeric(c: i32) -> bool {
    u8::try_from(c).is_ok_and(|c| c.is_ascii_alphanumeric())
}

fn consume_hex_digit(lexer: &mut dyn Lexer) -> bool {
    lexer.advance(false);
    u8::try_from(lexer.lookahead()).is_ok_and(|c| c.is_ascii_hexdigit())
}

fn accept_inplace(lexer: &mut dyn Lexer, token: usize) -> bool {
    lexer.set_result_symbol(token as u16);
    true
}

fn accept_and_advance(lexer: &mut dyn Lexer, token: usize) -> bool {
    lexer.advance(false);
    accept_inplace(lexer, token)
}

impl Scanner {
    fn in_context(&self, kind: ContextType) -> bool {
        self.context_stack
            .last()
            .is_some_and(|ctx| ctx.kind == kind)
    }

    fn push_context(&mut self, kind: ContextType) {
        self.context_stack.push(Context {
            kind,
            heredoc_identifier: Vec::new(),
        });
    }
}

impl ExternalScanner for Scanner {
    fn scan(&mut self, lexer: &mut dyn Lexer, valid_symbols: &[bool]) -> bool {
        let mut has_leading_whitespace_with_newline = false;
        while is_space(lexer.lookahead()) {
            if lexer.lookahead() == '\n' as i32 {
                has_leading_whitespace_with_newline = true;
            }
            lexer.advance(true);
        }
        if lexer.lookahead() == 0 {
            return false;
        }

        // Manage quoted context.
        if valid_symbols[QUOTED_TEMPLATE_START]
            && !self.in_context(ContextType::QuotedTemplate)
            && lexer.lookahead() == '"' as i32
        {
            self.push_context(ContextType::QuotedTemplate);
            return accept_and_advance(lexer, QUOTED_TEMPLATE_START);
        }
        if valid_symbols[QUOTED_TEMPLATE_END]
            && self.in_context(ContextType::QuotedTemplate)
            && lexer.lookahead() == '"' as i32
        {
            self.context_stack.pop();
            return accept_and_advance(lexer, QUOTED_TEMPLATE_END);
        }

        // Manage template interpolations.
        if valid_symbols[TEMPLATE_INTERPOLATION_START]
            && valid_symbols[TEMPLATE_LITERAL_CHUNK]
            && !self.in_context(ContextType::TemplateInterpolation)
            && lexer.lookahead() == '$' as i32
        {
            lexer.advance(false);
            if lexer.lookahead() == '{' as i32 {
                self.push_context(ContextType::TemplateInterpolation);
                return accept_and_advance(lexer, TEMPLATE_INTERPOLATION_START);
            }
            if lexer.lookahead() == '$' as i32 {
                lexer.advance(false);
                if lexer.lookahead() == '{' as i32 {
                    return accept_and_advance(lexer, TEMPLATE_LITERAL_CHUNK);
                }
            }
            return accept_inplace(lexer, TEMPLATE_LITERAL_CHUNK);
        }
        if valid_symbols[TEMPLATE_INTERPOLATION_END]
            && self.in_context(ContextType::TemplateInterpolation)
            && lexer.lookahead() == '}' as i32
        {
            self.context_stack.pop();
            return accept_and_advance(lexer, TEMPLATE_INTERPOLATION_END);
        }

        // Manage template directives.
        if valid_symbols[TEMPLATE_DIRECTIVE_START]
            && valid_symbols[TEMPLATE_LITERAL_CHUNK]
            && !self.in_context(ContextType::TemplateDirective)
            && lexer.lookahead() == '%' as i32
        {
            lexer.advance(false);
            if lexer.lookahead() == '{' as i32 {
                self.push_context(ContextType::TemplateDirective);
                return accept_and_advance(lexer, TEMPLATE_DIRECTIVE_START);
            }
            if lexer.lookahead() == '%' as i32 {
                lexer.advance(false);
                if lexer.lookahead() == '{' as i32 {
                    return accept_and_advance(lexer, TEMPLATE_LITERAL_CHUNK);
                }
            }
            return accept_inplace(lexer, TEMPLATE_LITERAL_CHUNK);
        }
        if valid_symbols[TEMPLATE_DIRECTIVE_END]
            && self.in_context(ContextType::TemplateDirective)
            && lexer.lookahead() == '}' as i32
        {
            self.context_stack.pop();
            return accept_and_advance(lexer, TEMPLATE_DIRECTIVE_END);
        }

        // Manage heredoc context. Like C, accept even an empty identifier.
        if valid_symbols[HEREDOC_IDENTIFIER] && !self.in_context(ContextType::HeredocTemplate) {
            let mut identifier = Vec::new();
            while is_alphanumeric(lexer.lookahead())
                || lexer.lookahead() == '_' as i32
                || lexer.lookahead() == '-' as i32
            {
                identifier.push(lexer.lookahead() as u8);
                lexer.advance(false);
            }
            self.context_stack.push(Context {
                kind: ContextType::HeredocTemplate,
                heredoc_identifier: identifier,
            });
            return accept_inplace(lexer, HEREDOC_IDENTIFIER);
        }
        if valid_symbols[HEREDOC_IDENTIFIER]
            && self.in_context(ContextType::HeredocTemplate)
            && has_leading_whitespace_with_newline
        {
            let expected_identifier = &self.context_stack.last().unwrap().heredoc_identifier;
            for &c in expected_identifier {
                // Promote as C does with a signed `char` (x86-64).
                if lexer.lookahead() == i32::from(c as i8) {
                    lexer.advance(false);
                } else {
                    // This path emits a literal even if that token is not valid.
                    return accept_inplace(lexer, TEMPLATE_LITERAL_CHUNK);
                }
            }
            lexer.mark_end();
            while is_space(lexer.lookahead()) && lexer.lookahead() != '\n' as i32 {
                lexer.advance(false);
            }
            if lexer.lookahead() == '\n' as i32 {
                self.context_stack.pop();
                return accept_inplace(lexer, HEREDOC_IDENTIFIER);
            }
            // EOF is deliberately not accepted as a heredoc terminator.
            lexer.advance(false);
            lexer.mark_end();
            return accept_inplace(lexer, TEMPLATE_LITERAL_CHUNK);
        }

        // Quoted template escape sequences. consume_hex_digit advances before
        // inspecting each digit, including the first invalid digit on failure.
        if valid_symbols[TEMPLATE_LITERAL_CHUNK]
            && self.in_context(ContextType::QuotedTemplate)
            && lexer.lookahead() == '\\' as i32
        {
            lexer.advance(false);
            match char::from_u32(lexer.lookahead() as u32) {
                Some('"' | 'n' | 'r' | 't' | '\\') => {
                    return accept_and_advance(lexer, TEMPLATE_LITERAL_CHUNK);
                }
                Some('u') => {
                    for _ in 0..4 {
                        if !consume_hex_digit(lexer) {
                            return false;
                        }
                    }
                    return accept_and_advance(lexer, TEMPLATE_LITERAL_CHUNK);
                }
                Some('U') => {
                    for _ in 0..8 {
                        if !consume_hex_digit(lexer) {
                            return false;
                        }
                    }
                    return accept_and_advance(lexer, TEMPLATE_LITERAL_CHUNK);
                }
                _ => return false,
            }
        }

        // All other quoted template or string literal characters.
        if valid_symbols[TEMPLATE_LITERAL_CHUNK]
            && (self.in_context(ContextType::QuotedTemplate)
                || self.in_context(ContextType::HeredocTemplate))
        {
            return accept_and_advance(lexer, TEMPLATE_LITERAL_CHUNK);
        }
        false
    }

    fn serialize(&mut self, buffer: &mut [u8]) -> usize {
        // C's CHAR_MAX is 127 where `char` is signed (x86-64), despite writing u32s.
        if self.context_stack.len() > i8::MAX as usize || buffer.len() < 4 {
            return 0;
        }
        buffer[..4].copy_from_slice(&(self.context_stack.len() as u32).to_ne_bytes());
        let mut size = 4;
        for context in &self.context_stack {
            let length = context.heredoc_identifier.len();
            // The C boundary test is >=, not >. The slice-length check only
            // protects callers supplying less than the standard 1024 bytes.
            if size + 8 + length >= SERIALIZATION_BUFFER_SIZE
                || size + 8 + length > buffer.len()
                || length > i8::MAX as usize
            {
                return 0;
            }
            buffer[size..size + 4].copy_from_slice(&(context.kind as u32).to_ne_bytes());
            size += 4;
            buffer[size..size + 4].copy_from_slice(&(length as u32).to_ne_bytes());
            size += 4;
            buffer[size..size + length].copy_from_slice(&context.heredoc_identifier);
            size += length;
        }
        size
    }

    fn deserialize(&mut self, buffer: &[u8]) {
        self.context_stack.clear();
        if buffer.is_empty() {
            return;
        }

        fn read_u32(buffer: &[u8], size: &mut usize) -> u32 {
            let value = u32::from_ne_bytes(buffer[*size..*size + 4].try_into().unwrap());
            *size += 4;
            value
        }

        let mut size = 0;
        let count = read_u32(buffer, &mut size);
        for _ in 0..count {
            let kind = match read_u32(buffer, &mut size) {
                0 => ContextType::TemplateInterpolation,
                1 => ContextType::TemplateDirective,
                2 => ContextType::QuotedTemplate,
                3 => ContextType::HeredocTemplate,
                _ => panic!("invalid HCL scanner context type"),
            };
            let length = read_u32(buffer, &mut size) as usize;
            let heredoc_identifier = buffer[size..size + length].to_vec();
            size += length;
            self.context_stack.push(Context {
                kind,
                heredoc_identifier,
            });
        }
        assert_eq!(size, buffer.len());
    }
}

/// Creates a scanner (C's `tree_sitter_hcl_external_scanner_create`).
pub(crate) fn create() -> Box<dyn ExternalScanner> {
    Box::<Scanner>::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Eq)]
    enum Call {
        Advance(bool),
        MarkEnd(usize),
        Result(u16),
    }

    struct TestLexer {
        input: Vec<i32>,
        position: usize,
        symbol: u16,
        calls: Vec<Call>,
    }

    impl TestLexer {
        fn new(input: &str) -> Self {
            Self {
                input: input.chars().map(|c| c as i32).collect(),
                position: 0,
                symbol: u16::MAX,
                calls: Vec::new(),
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
            self.calls.push(Call::Result(symbol));
        }

        fn advance(&mut self, skip: bool) {
            self.calls.push(Call::Advance(skip));
            if self.position < self.input.len() {
                self.position += 1;
            }
        }

        fn mark_end(&mut self) {
            self.calls.push(Call::MarkEnd(self.position));
        }

        fn get_column(&mut self) -> u32 {
            panic!("HCL scanner does not request columns")
        }

        fn is_at_included_range_start(&self) -> bool {
            panic!("HCL scanner does not request included range starts")
        }

        fn eof(&self) -> bool {
            panic!("HCL scanner tests NUL lookahead, not eof()")
        }
    }

    fn scan(scanner: &mut Scanner, input: &str, valid: &[usize]) -> (bool, TestLexer) {
        let mut valid_symbols = [false; 8];
        for &symbol in valid {
            valid_symbols[symbol] = true;
        }
        let mut lexer = TestLexer::new(input);
        let accepted = scanner.scan(&mut lexer, &valid_symbols);
        (accepted, lexer)
    }

    fn accept(scanner: &mut Scanner, input: &str, valid: &[usize], expected: usize) -> TestLexer {
        let (accepted, lexer) = scan(scanner, input, valid);
        assert!(accepted);
        assert_eq!(lexer.symbol, expected as u16);
        lexer
    }

    fn quoted_scanner() -> Scanner {
        let mut scanner = Scanner::default();
        scanner.push_context(ContextType::QuotedTemplate);
        scanner
    }

    fn heredoc_scanner(identifier: &str) -> Scanner {
        let mut scanner = Scanner::default();
        accept(
            &mut scanner,
            identifier,
            &[HEREDOC_IDENTIFIER],
            HEREDOC_IDENTIFIER,
        );
        scanner
    }

    #[test]
    fn nested_templates_use_only_the_top_context() {
        let mut scanner = Scanner::default();
        for (input, symbol) in [
            ("\"", QUOTED_TEMPLATE_START),
            ("${", TEMPLATE_INTERPOLATION_START),
            ("\"", QUOTED_TEMPLATE_START),
            ("%{", TEMPLATE_DIRECTIVE_START),
        ] {
            let lexer = accept(
                &mut scanner,
                input,
                &[symbol, TEMPLATE_LITERAL_CHUNK],
                symbol,
            );
            assert_eq!(lexer.position, input.len());
            let mut expected_calls = (0..input.len())
                .map(|_| Call::Advance(false))
                .collect::<Vec<_>>();
            expected_calls.push(Call::Result(symbol as u16));
            assert_eq!(lexer.calls, expected_calls);
        }
        assert_eq!(scanner.context_stack.len(), 4);
        for (input, symbol) in [
            ("}", TEMPLATE_DIRECTIVE_END),
            ("\"", QUOTED_TEMPLATE_END),
            ("}", TEMPLATE_INTERPOLATION_END),
            ("\"", QUOTED_TEMPLATE_END),
        ] {
            accept(&mut scanner, input, &[symbol], symbol);
        }
        assert!(scanner.context_stack.is_empty());
    }

    #[test]
    fn interpolation_and_directive_prefixes_and_escapes() {
        for (prefix, start, context) in [
            (
                '$',
                TEMPLATE_INTERPOLATION_START,
                ContextType::TemplateInterpolation,
            ),
            (
                '%',
                TEMPLATE_DIRECTIVE_START,
                ContextType::TemplateDirective,
            ),
        ] {
            let mut scanner = quoted_scanner();
            let (accepted, lexer) = scan(&mut scanner, &format!("{prefix}{{"), &[start]);
            assert!(!accepted); // Both the start and literal flags are required.
            assert!(lexer.calls.is_empty());
            for (text, length) in [
                (format!("{prefix}x"), 1),
                (format!("{prefix}{prefix}x"), 2),
                (format!("{prefix}{prefix}{{"), 3),
            ] {
                let lexer = accept(
                    &mut scanner,
                    &text,
                    &[start, TEMPLATE_LITERAL_CHUNK],
                    TEMPLATE_LITERAL_CHUNK,
                );
                assert_eq!(lexer.position, length);
                assert_eq!(scanner.context_stack.len(), 1);
            }
            accept(
                &mut scanner,
                &format!("{prefix}{{"),
                &[start, TEMPLATE_LITERAL_CHUNK],
                start,
            );
            assert!(scanner.in_context(context));
            let (accepted, lexer) = scan(
                &mut scanner,
                &format!("{prefix}{{"),
                &[start, TEMPLATE_LITERAL_CHUNK],
            );
            assert!(!accepted); // No recursive interpolation/directive of the same kind.
            assert!(lexer.calls.is_empty());
        }
    }

    #[test]
    fn whitespace_is_skipped_even_in_literal_chunks() {
        let mut scanner = quoted_scanner();
        let lexer = accept(
            &mut scanner,
            " \t\r\n\u{b}\u{c}x",
            &[TEMPLATE_LITERAL_CHUNK],
            TEMPLATE_LITERAL_CHUNK,
        );
        assert_eq!(lexer.position, 7);
        let mut calls = (0..6).map(|_| Call::Advance(true)).collect::<Vec<_>>();
        calls.push(Call::Advance(false));
        calls.push(Call::Result(TEMPLATE_LITERAL_CHUNK as u16));
        assert_eq!(lexer.calls, calls);

        // C-locale iswspace does not skip non-ASCII whitespace.
        let lexer = accept(
            &mut scanner,
            "\u{a0}x",
            &[TEMPLATE_LITERAL_CHUNK],
            TEMPLATE_LITERAL_CHUNK,
        );
        assert_eq!(lexer.position, 1);
        assert_eq!(lexer.calls[0], Call::Advance(false));
        let (accepted, lexer) = scan(&mut scanner, " \0x", &[TEMPLATE_LITERAL_CHUNK]);
        assert!(!accepted);
        assert_eq!(lexer.calls, [Call::Advance(true)]);
    }

    #[test]
    fn quoted_escapes_advance_before_testing_hex_digits() {
        let mut scanner = quoted_scanner();
        for input in [
            r#"\""#,
            r"\n",
            r"\r",
            r"\t",
            r"\\",
            r"\u012a",
            r"\U0010Ff00",
        ] {
            let lexer = accept(
                &mut scanner,
                input,
                &[TEMPLATE_LITERAL_CHUNK],
                TEMPLATE_LITERAL_CHUNK,
            );
            assert_eq!(lexer.position, input.len());
            assert_eq!(lexer.calls.len(), input.len() + 1);
        }
        for (input, position) in [(r"\u12X4", 4), (r"\U123", 5), (r"\q", 1), (r"\", 1)] {
            let (accepted, lexer) = scan(&mut scanner, input, &[TEMPLATE_LITERAL_CHUNK]);
            assert!(!accepted);
            assert_eq!(lexer.position, position);
            assert!(lexer.calls.iter().all(|call| *call == Call::Advance(false)));
        }
    }

    #[test]
    fn heredoc_start_accepts_ascii_identifier_or_empty_identifier() {
        let mut scanner = heredoc_scanner(" _a-Z09\n");
        assert_eq!(scanner.context_stack[0].heredoc_identifier, b"_a-Z09");
        scanner.deserialize(&[]);
        let lexer = accept(&mut scanner, "é", &[HEREDOC_IDENTIFIER], HEREDOC_IDENTIFIER);
        assert_eq!(lexer.position, 0);
        assert!(scanner.context_stack[0].heredoc_identifier.is_empty());
        assert_eq!(lexer.calls, [Call::Result(HEREDOC_IDENTIFIER as u16)]);
    }

    #[test]
    fn heredoc_end_marks_before_trailing_whitespace() {
        let mut scanner = heredoc_scanner("EOF");
        let lexer = accept(
            &mut scanner,
            "\r\n \tEOF \r\nrest",
            &[HEREDOC_IDENTIFIER],
            HEREDOC_IDENTIFIER,
        );
        assert_eq!(lexer.position, 9);
        assert_eq!(
            lexer.calls,
            [
                Call::Advance(true),
                Call::Advance(true),
                Call::Advance(true),
                Call::Advance(true),
                Call::Advance(false),
                Call::Advance(false),
                Call::Advance(false),
                Call::MarkEnd(7),
                Call::Advance(false),
                Call::Advance(false),
                Call::Result(HEREDOC_IDENTIFIER as u16),
            ]
        );
        assert!(scanner.context_stack.is_empty());
    }

    #[test]
    fn heredoc_mismatch_and_eof_emit_literals_even_when_invalid() {
        let mut scanner = heredoc_scanner("EOF");
        for (input, position, marks) in [
            ("\nEOX", 3, vec![]),
            ("\nX", 1, vec![]),
            ("\nEOFX", 5, vec![4, 5]),
            ("\nEOF", 4, vec![4, 4]),
        ] {
            let lexer = accept(
                &mut scanner,
                input,
                &[HEREDOC_IDENTIFIER],
                TEMPLATE_LITERAL_CHUNK,
            );
            assert_eq!(lexer.position, position);
            assert_eq!(
                lexer
                    .calls
                    .iter()
                    .filter_map(|call| match call {
                        Call::MarkEnd(position) => Some(*position),
                        _ => None,
                    })
                    .collect::<Vec<_>>(),
                marks
            );
            assert!(scanner.in_context(ContextType::HeredocTemplate));
        }
        // A CR alone is not enough to attempt a closing identifier.
        let lexer = accept(
            &mut scanner,
            "\rEOF\n",
            &[HEREDOC_IDENTIFIER, TEMPLATE_LITERAL_CHUNK],
            TEMPLATE_LITERAL_CHUNK,
        );
        assert_eq!(lexer.position, 2);
        assert!(scanner.in_context(ContextType::HeredocTemplate));
    }

    #[test]
    fn snapshots_use_native_u32_fields_and_reset_all_contexts() {
        let mut scanner = heredoc_scanner("EOF");
        scanner.push_context(ContextType::TemplateInterpolation);
        scanner.push_context(ContextType::QuotedTemplate);
        let mut expected = 3_u32.to_ne_bytes().to_vec();
        expected.extend_from_slice(&3_u32.to_ne_bytes());
        expected.extend_from_slice(&3_u32.to_ne_bytes());
        expected.extend_from_slice(b"EOF");
        for kind in [0_u32, 2] {
            expected.extend_from_slice(&kind.to_ne_bytes());
            expected.extend_from_slice(&0_u32.to_ne_bytes());
        }
        let mut buffer = [0; SERIALIZATION_BUFFER_SIZE];
        let length = scanner.serialize(&mut buffer);
        assert_eq!(&buffer[..length], expected);
        let mut restored = quoted_scanner();
        restored.deserialize(&buffer[..length]);
        assert_eq!(restored.context_stack, scanner.context_stack);
        restored.deserialize(&[]);
        assert!(restored.context_stack.is_empty());
        assert_eq!(restored.serialize(&mut buffer), 4);
        assert_eq!(buffer[..4], [0; 4]);
        scanner.deserialize(&buffer[..4]);
        assert!(scanner.context_stack.is_empty());
    }

    #[test]
    fn serialization_preserves_char_max_and_strict_buffer_limit() {
        let mut scanner = Scanner::default();
        let mut buffer = [0; SERIALIZATION_BUFFER_SIZE];
        for _ in 0..127 {
            scanner.push_context(ContextType::QuotedTemplate);
        }
        assert_eq!(scanner.serialize(&mut buffer), 1020);
        scanner.push_context(ContextType::QuotedTemplate);
        assert_eq!(scanner.serialize(&mut buffer), 0);
        scanner.context_stack.pop();
        scanner.context_stack.last_mut().unwrap().heredoc_identifier = vec![b'x'; 3];
        assert_eq!(scanner.serialize(&mut buffer), 1023);
        scanner
            .context_stack
            .last_mut()
            .unwrap()
            .heredoc_identifier
            .push(b'x');
        assert_eq!(scanner.serialize(&mut buffer), 0); // Exactly 1024 is rejected.

        scanner.context_stack.clear();
        scanner.context_stack.push(Context {
            kind: ContextType::HeredocTemplate,
            heredoc_identifier: vec![b'x'; 127],
        });
        assert_eq!(scanner.serialize(&mut buffer), 139);
        scanner.context_stack[0].heredoc_identifier.push(b'x');
        assert_eq!(scanner.serialize(&mut buffer), 0);
    }
}
