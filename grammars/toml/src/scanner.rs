//! The TOML external scanner, translated from `src/scanner.c`.

use ts_port_tables::{ExternalScanner, Lexer, Symbol};

const LINE_ENDING_OR_EOF: Symbol = 0;
const MULTILINE_BASIC_STRING_CONTENT: Symbol = 1;
const MULTILINE_BASIC_STRING_END: Symbol = 2;
const MULTILINE_LITERAL_STRING_CONTENT: Symbol = 3;
const MULTILINE_LITERAL_STRING_END: Symbol = 4;

/// The C scanner has no payload or serialized state.
pub(crate) struct Scanner;

fn scan_multiline_string_end(
    lexer: &mut dyn Lexer,
    delimiter: i32,
    content_symbol: Symbol,
    end_symbol: Symbol,
) -> bool {
    // The caller has checked the end token and first delimiter.
    lexer.advance(false);
    lexer.mark_end();

    if lexer.lookahead() != delimiter {
        lexer.set_result_symbol(content_symbol);
        return true;
    }

    lexer.advance(false);

    if lexer.lookahead() != delimiter {
        lexer.mark_end();
        lexer.set_result_symbol(content_symbol);
        return true;
    }

    lexer.advance(false);

    if lexer.lookahead() != delimiter {
        lexer.mark_end();
        lexer.set_result_symbol(end_symbol);
        return true;
    }

    // Four or more quotes: keep the mark after the first quote so the
    // remaining quotes can be scanned again as content or a closing delimiter.
    lexer.set_result_symbol(content_symbol);
    true
}

impl ExternalScanner for Scanner {
    fn scan(&mut self, lexer: &mut dyn Lexer, valid_symbols: &[bool]) -> bool {
        let Some(valid_symbols) = valid_symbols.first_chunk::<5>() else {
            return false;
        };
        let mut lookahead = lexer.lookahead();
        match lookahead {
            0x22 if valid_symbols[MULTILINE_BASIC_STRING_END as usize] => {
                return scan_multiline_string_end(
                    lexer,
                    lookahead,
                    MULTILINE_BASIC_STRING_CONTENT,
                    MULTILINE_BASIC_STRING_END,
                );
            }
            0x27 if valid_symbols[MULTILINE_LITERAL_STRING_END as usize] => {
                return scan_multiline_string_end(
                    lexer,
                    lookahead,
                    MULTILINE_LITERAL_STRING_CONTENT,
                    MULTILINE_LITERAL_STRING_END,
                );
            }
            _ => {}
        }

        if !valid_symbols[LINE_ENDING_OR_EOF as usize]
            || !matches!(lookahead, 0 | 0x09 | 0x0a | 0x0d | 0x20)
        {
            // The parser ignores result_symbol when a scanner returns false.
            // An immediate rejection need not write that unused output.
            return false;
        }
        lexer.set_result_symbol(LINE_ENDING_OR_EOF);
        while matches!(lookahead, 0x20 | 0x09) {
            lexer.advance(true);
            lookahead = lexer.lookahead();
        }
        if lookahead == 0x0d {
            lexer.advance(true);
            return lexer.lookahead() == i32::from(b'\n');
        }
        // C treats an embedded NUL as EOF here.
        matches!(lookahead, 0 | 0x0a)
    }

    fn serialize(&mut self, _buffer: &mut [u8]) -> usize {
        0
    }

    fn deserialize(&mut self, _buffer: &[u8]) {}
}

/// Creates a scanner (C's `tree_sitter_toml_external_scanner_create`).
pub(crate) fn create() -> Box<dyn ExternalScanner> {
    Box::new(Scanner)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    #[derive(Debug, PartialEq, Eq)]
    enum Event {
        Advance { position: usize, skip: bool },
        MarkEnd(usize),
        Symbol(Symbol),
    }

    struct TestLexer {
        input: Vec<i32>,
        position: usize,
        symbol: Symbol,
        events: Vec<Event>,
        lookahead_calls: Cell<usize>,
    }

    impl TestLexer {
        fn new(input: &str) -> Self {
            Self {
                input: input.chars().map(|c| c as i32).collect(),
                position: 0,
                symbol: Symbol::MAX,
                events: Vec::new(),
                lookahead_calls: Cell::new(0),
            }
        }
    }

    impl Lexer for TestLexer {
        fn lookahead(&self) -> i32 {
            self.lookahead_calls.set(self.lookahead_calls.get() + 1);
            self.input.get(self.position).copied().unwrap_or(0)
        }

        fn result_symbol(&self) -> Symbol {
            self.symbol
        }

        fn set_result_symbol(&mut self, symbol: Symbol) {
            self.symbol = symbol;
            self.events.push(Event::Symbol(symbol));
        }

        fn advance(&mut self, skip: bool) {
            assert!(self.position < self.input.len());
            self.events.push(Event::Advance {
                position: self.position,
                skip,
            });
            self.position += 1;
        }

        fn mark_end(&mut self) {
            self.events.push(Event::MarkEnd(self.position));
        }

        fn get_column(&mut self) -> u32 {
            panic!("TOML scanner must not query columns")
        }

        fn is_at_included_range_start(&self) -> bool {
            panic!("TOML scanner must not query included ranges")
        }

        fn eof(&self) -> bool {
            panic!("TOML scanner must use lookahead, not eof()")
        }
    }

    #[test]
    fn quote_runs_preserve_advance_mark_and_symbol_order() {
        for (delimiter, content, end) in [
            (
                '"',
                MULTILINE_BASIC_STRING_CONTENT,
                MULTILINE_BASIC_STRING_END,
            ),
            (
                '\'',
                MULTILINE_LITERAL_STRING_CONTENT,
                MULTILINE_LITERAL_STRING_END,
            ),
        ] {
            for count in 1..=6 {
                for suffix in ["", "x", "\n"] {
                    let input = format!("{}{suffix}", delimiter.to_string().repeat(count));
                    let mut lexer = TestLexer::new(&input);
                    // Content validity is intentionally not required by C.
                    let mut valid_symbols = [false; 5];
                    valid_symbols[end as usize] = true;
                    assert!(Scanner.scan(&mut lexer, &valid_symbols));

                    let mut expected = vec![
                        Event::Advance {
                            position: 0,
                            skip: false,
                        },
                        Event::MarkEnd(1),
                    ];
                    for position in 1..count.min(3) {
                        expected.push(Event::Advance {
                            position,
                            skip: false,
                        });
                    }
                    if count == 2 || count == 3 {
                        expected.push(Event::MarkEnd(count));
                    }
                    expected.push(Event::Symbol(if count == 3 { end } else { content }));
                    assert_eq!(lexer.events, expected, "input {input:?}");
                }
            }
        }
    }

    #[test]
    fn only_the_matching_end_symbol_enables_quote_scanning() {
        for input in ["\"\"\"", "'''", "x"] {
            let mut lexer = TestLexer::new(input);
            assert!(!Scanner.scan(&mut lexer, &[false, true, false, true, false]));
            assert!(lexer.events.is_empty());
        }
        for (input, valid_symbols) in [
            ("\"\"\"", [false, false, false, false, true]),
            ("'''", [false, false, true, false, false]),
            ("x", [true; 5]),
        ] {
            let mut lexer = TestLexer::new(input);
            assert!(!Scanner.scan(&mut lexer, &valid_symbols));
            assert_eq!(lexer.position, 0);
        }
    }

    #[test]
    fn line_endings_skip_only_spaces_tabs_and_an_optional_carriage_return() {
        for (input, accepted, advances) in [
            ("", true, 0),
            ("\n", true, 0),
            ("\0x", true, 0),
            (" \t", true, 2),
            (" \t\n", true, 2),
            (" \t\r\n", true, 3),
            ("\r\n", true, 1),
            ("\r", false, 1),
            ("\r\0", false, 1),
            (" \t\rx", false, 3),
            (" x", false, 1),
            ("\u{b}\n", false, 0),
            ("\u{c}\n", false, 0),
            ("\u{a0}\n", false, 0),
        ] {
            let mut lexer = TestLexer::new(input);
            assert_eq!(
                Scanner.scan(&mut lexer, &[true, false, false, false, false]),
                accepted,
                "input {input:?}"
            );
            // Immediate failures leave the unused result symbol untouched.
            let mut expected = if accepted || advances != 0 {
                vec![Event::Symbol(LINE_ENDING_OR_EOF)]
            } else {
                vec![]
            };
            expected.extend((0..advances).map(|position| Event::Advance {
                position,
                skip: true,
            }));
            assert_eq!(lexer.events, expected, "input {input:?}");
            assert_eq!(lexer.position, advances);

            let mut disabled = TestLexer::new(input);
            assert!(!Scanner.scan(&mut disabled, &[false; 5]));
            assert!(disabled.events.is_empty());
        }
    }

    #[test]
    fn multiline_tokens_take_priority_over_line_endings() {
        for (input, end) in [
            ("\"\"\"", MULTILINE_BASIC_STRING_END),
            ("'''", MULTILINE_LITERAL_STRING_END),
        ] {
            let mut lexer = TestLexer::new(input);
            assert!(Scanner.scan(&mut lexer, &[true; 5]));
            assert_eq!(lexer.result_symbol(), end);
            assert!(!lexer.events.contains(&Event::Symbol(LINE_ENDING_OR_EOF)));
        }
    }

    #[test]
    fn initial_character_dispatch_preserves_c_tokens_for_every_validity_mask() {
        // Include decoder errors, non-ASCII spaces, and values whose low byte
        // aliases a delimiter. Classification must not truncate code points.
        for first in (0..=127).chain([-1, 0x85, 0xa0, 0x109, 0x122, 0x127, 0x2003, i32::MAX]) {
            for mask in 0..32 {
                let valid: [bool; 5] = std::array::from_fn(|i| mask & (1 << i) != 0);
                let mut lexer = TestLexer::new(" x");
                lexer.input[0] = first;
                let accepted = Scanner.scan(&mut lexer, &valid);

                let quote_content = match first {
                    0x22 if valid[MULTILINE_BASIC_STRING_END as usize] => {
                        Some(MULTILINE_BASIC_STRING_CONTENT)
                    }
                    0x27 if valid[MULTILINE_LITERAL_STRING_END as usize] => {
                        Some(MULTILINE_LITERAL_STRING_CONTENT)
                    }
                    _ => None,
                };
                let (expected_accepted, expected_events) = if let Some(content) = quote_content {
                    (
                        true,
                        vec![
                            Event::Advance {
                                position: 0,
                                skip: false,
                            },
                            Event::MarkEnd(1),
                            Event::Symbol(content),
                        ],
                    )
                } else if valid[LINE_ENDING_OR_EOF as usize] {
                    // The sole omitted C event is a dead result-symbol write
                    // on an immediate failure, which the runtime never reads.
                    let mut events = if matches!(first, 0 | 0x09 | 0x0a | 0x0d | 0x20) {
                        vec![Event::Symbol(LINE_ENDING_OR_EOF)]
                    } else {
                        vec![]
                    };
                    if matches!(first, 0x09 | 0x0d | 0x20) {
                        events.push(Event::Advance {
                            position: 0,
                            skip: true,
                        });
                    }
                    (matches!(first, 0 | 0x0a), events)
                } else {
                    (false, vec![])
                };
                assert_eq!(accepted, expected_accepted, "first {first}, mask {mask}");
                assert_eq!(lexer.events, expected_events, "first {first}, mask {mask}");
            }
        }
    }

    #[test]
    fn one_lookahead_read_per_visited_position() {
        for input in [
            "",
            "\0x",
            "\n",
            " \t\n",
            " \t\r\n",
            "\r",
            "\r\0",
            " \t\r\rx",
            " \t#comment",
            "x",
            "é",
            "\u{a0}\n",
            "\"",
            "\"\"",
            "\"\"\"",
            "\"\"\"\"",
            "'",
            "''",
            "'''",
            "''''",
            "\"'",
            "'\"",
            " \t\"\"\"",
        ] {
            for mask in 0..32 {
                let valid: [bool; 5] = std::array::from_fn(|i| mask & (1 << i) != 0);
                let mut lexer = TestLexer::new(input);
                Scanner.scan(&mut lexer, &valid);
                assert_eq!(
                    lexer.lookahead_calls.get(),
                    lexer.position + 1,
                    "input {input:?}, mask {mask}"
                );
            }
        }
    }

    #[test]
    fn serialization_has_no_state_and_leaves_buffer_untouched() {
        let mut scanner = create();
        let mut buffer = [0xa5; 16];
        scanner.deserialize(&[1, 2, 3]);
        assert_eq!(scanner.serialize(&mut buffer), 0);
        assert_eq!(buffer, [0xa5; 16]);
        scanner.deserialize(&[]);
        assert_eq!(scanner.serialize(&mut []), 0);
    }
}
