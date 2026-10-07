//! The TOML external scanner, translated from `src/scanner.c`.

use ts_port_tables::{ExternalScanner, Lexer, Symbol};

const LINE_ENDING_OR_EOF: Symbol = 0;
const MULTILINE_BASIC_STRING_CONTENT: Symbol = 1;
const MULTILINE_BASIC_STRING_END: Symbol = 2;
const MULTILINE_LITERAL_STRING_CONTENT: Symbol = 3;
const MULTILINE_LITERAL_STRING_END: Symbol = 4;

/// The C scanner has no payload or serialized state.
pub(crate) struct Scanner;

// The cold non-LF dispatch shares the quote loop for both delimiters.
// Inline it there to avoid a second call frame; only the delimiter needs to
// survive the lexer callbacks, with token IDs computed at the final position.
#[inline(always)]
fn scan_multiline_string_end(lexer: &mut dyn Lexer, delimiter: i32) -> bool {
    // The caller has checked the first quote and its end-token validity.
    lexer.advance(false);
    lexer.mark_end();

    let mut is_end = false;
    if lexer.lookahead() == delimiter {
        lexer.advance(false);
        if lexer.lookahead() == delimiter {
            lexer.advance(false);
            if lexer.lookahead() != delimiter {
                lexer.mark_end();
                is_end = true;
            }
        } else {
            lexer.mark_end();
        }
    }

    // Four or more quotes retain the mark after the first quote, as in C.
    let content_symbol = if delimiter == 0x22 {
        MULTILINE_BASIC_STRING_CONTENT
    } else {
        MULTILINE_LITERAL_STRING_CONTENT
    };
    lexer.set_result_symbol(content_symbol + Symbol::from(is_end));
    true
}

// LF takes the direct path in scan. Dispatch the remaining starts only
// when needed. Marking this helper cold lets the enabled LF path fall
// through the entry checks instead of branching around failure returns.
#[cold]
#[inline(never)]
fn scan_non_newline(lexer: &mut dyn Lexer, valid_symbols: &[bool; 5], mut lookahead: i32) -> bool {
    // Only these non-LF starts can lead to a line-ending token. The range
    // check keeps decoder errors and non-ASCII low-byte aliases out of the
    // shift, and the mask avoids an indirect dispatch for this small set.
    const LINE_STARTS: u64 = (1 << 0) | (1 << 9) | (1 << 13) | (1 << 32);
    if lookahead as u32 <= 32 && (LINE_STARTS >> lookahead) & 1 != 0 {
        if !valid_symbols[LINE_ENDING_OR_EOF as usize] {
            return false;
        }
    } else {
        let end_symbol = match lookahead {
            0x22 => MULTILINE_BASIC_STRING_END,
            0x27 => MULTILINE_LITERAL_STRING_END,
            _ => return false,
        };
        return valid_symbols[end_symbol as usize] && scan_multiline_string_end(lexer, lookahead);
    }

    while matches!(lookahead, 0x20 | 0x09) {
        lexer.advance(true);
        lookahead = lexer.lookahead();
    }
    let accepted = if lookahead == 0x0d {
        lexer.advance(true);
        lexer.lookahead() == i32::from(b'\n')
    } else {
        // C treats an embedded NUL as EOF here.
        matches!(lookahead, 0 | 0x0a)
    };
    if accepted {
        // The parser ignores result_symbol on failure. Defer the write
        // so whitespace-prefixed failed scans do not make a virtual call
        // solely to produce an unused symbol.
        lexer.set_result_symbol(LINE_ENDING_OR_EOF);
    }
    accepted
}

impl ExternalScanner for Scanner {
    fn scan(&mut self, lexer: &mut dyn Lexer, valid_symbols: &[bool]) -> bool {
        let Some(valid_symbols) = valid_symbols.first_chunk::<5>() else {
            return false;
        };
        let lookahead = lexer.lookahead();
        if lookahead == 0x0a {
            let accepted = valid_symbols[LINE_ENDING_OR_EOF as usize];
            if accepted {
                lexer.set_result_symbol(LINE_ENDING_OR_EOF);
            }
            accepted
        } else {
            scan_non_newline(lexer, valid_symbols, lookahead)
        }
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
    fn quote_scans_preserve_boundaries_for_every_validity_mask() {
        for (delimiter, content, end, other_quote) in [
            (
                '"',
                MULTILINE_BASIC_STRING_CONTENT,
                MULTILINE_BASIC_STRING_END,
                "''''",
            ),
            (
                '\'',
                MULTILINE_LITERAL_STRING_CONTENT,
                MULTILINE_LITERAL_STRING_END,
                "\"\"\"\"",
            ),
        ] {
            for count in 1..=8 {
                for suffix in ["", "x", "\0x", "\n", "\r\n", " ", other_quote] {
                    let input = format!("{}{suffix}", delimiter.to_string().repeat(count));
                    for mask in 0..32 {
                        let valid: [bool; 5] = std::array::from_fn(|i| mask & (1 << i) != 0);
                        let enabled = valid[end as usize];
                        let mut lexer = TestLexer::new(&input);
                        assert_eq!(Scanner.scan(&mut lexer, &valid), enabled);

                        let mut expected = vec![];
                        if enabled {
                            expected.push(Event::Advance {
                                position: 0,
                                skip: false,
                            });
                            expected.push(Event::MarkEnd(1));
                            for position in 1..count.min(3) {
                                expected.push(Event::Advance {
                                    position,
                                    skip: false,
                                });
                            }
                            if count == 2 || count == 3 {
                                expected.push(Event::MarkEnd(count));
                            }
                            // Neither content validity nor the other quote's
                            // end flag can change this delimiter's token.
                            expected.push(Event::Symbol(if count == 3 { end } else { content }));
                        }
                        assert_eq!(lexer.events, expected, "{input:?}, mask {mask}");
                        assert_eq!(lexer.position, if enabled { count.min(3) } else { 0 });
                        assert_eq!(lexer.lookahead_calls.get(), lexer.position + 1);
                    }
                }
            }
        }
    }

    #[test]
    fn quote_runs_compare_full_width_lookahead_after_every_advance() {
        for (delimiter, content, end) in [
            (
                0x22,
                MULTILINE_BASIC_STRING_CONTENT,
                MULTILINE_BASIC_STRING_END,
            ),
            (
                0x27,
                MULTILINE_LITERAL_STRING_CONTENT,
                MULTILINE_LITERAL_STRING_END,
            ),
        ] {
            // Include both quote bytes with upper bits set: narrowing the
            // shared delimiter comparison would change the mark or token.
            for terminator in [
                -1,
                i32::MIN,
                i32::MAX,
                0,
                0x0a,
                0x0d,
                0x122,
                0x127,
                0x10022,
                0x10027,
                0x10ff22,
                0x10ff27,
            ] {
                for count in 1..=3 {
                    for mask in 0..32 {
                        let valid: [bool; 5] = std::array::from_fn(|i| mask & (1 << i) != 0);
                        let enabled = valid[end as usize];
                        let mut lexer = TestLexer::new("");
                        lexer.input.resize(count, delimiter);
                        lexer.input.extend([terminator, delimiter, delimiter]);
                        assert_eq!(Scanner.scan(&mut lexer, &valid), enabled);

                        let mut expected = vec![];
                        if enabled {
                            expected.push(Event::Advance {
                                position: 0,
                                skip: false,
                            });
                            expected.push(Event::MarkEnd(1));
                            for position in 1..count {
                                expected.push(Event::Advance {
                                    position,
                                    skip: false,
                                });
                            }
                            if count > 1 {
                                expected.push(Event::MarkEnd(count));
                            }
                            expected.push(Event::Symbol(if count == 3 { end } else { content }));
                        }
                        assert_eq!(
                            lexer.events, expected,
                            "delimiter {delimiter}, terminator {terminator}, count {count}, mask {mask}"
                        );
                        assert_eq!(lexer.position, if enabled { count } else { 0 });
                        assert_eq!(lexer.lookahead_calls.get(), lexer.position + 1);
                    }
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
            let mut expected: Vec<_> = (0..advances)
                .map(|position| Event::Advance {
                    position,
                    skip: true,
                })
                .collect();
            if accepted {
                expected.push(Event::Symbol(LINE_ENDING_OR_EOF));
            } else {
                // Failed scans leave the unused result symbol untouched,
                // even after skipping whitespace or a carriage return.
                assert_eq!(lexer.symbol, Symbol::MAX);
            }
            assert_eq!(lexer.events, expected, "input {input:?}");
            assert_eq!(lexer.position, advances);

            let mut disabled = TestLexer::new(input);
            assert!(!Scanner.scan(&mut disabled, &[false; 5]));
            assert!(disabled.events.is_empty());
        }
    }

    #[test]
    fn whitespace_prefixed_scans_write_a_symbol_only_on_acceptance() {
        for length in 1..=6 {
            for whitespace_mask in 0..(1 << length) {
                let prefix: String = (0..length)
                    .map(|i| {
                        if whitespace_mask & (1 << i) == 0 {
                            ' '
                        } else {
                            '\t'
                        }
                    })
                    .collect();
                for (suffix, accepted, consumes_cr) in [
                    ("", true, false),
                    ("\n", true, false),
                    ("\0x", true, false),
                    ("\r\n", true, true),
                    ("\r", false, true),
                    ("\r\0", false, true),
                    ("\rx", false, true),
                    ("x", false, false),
                    ("#comment", false, false),
                    ("\"\"\"", false, false),
                    ("'''", false, false),
                    ("\u{a0}\n", false, false),
                ] {
                    let input = format!("{prefix}{suffix}");
                    for mask in 0..32 {
                        let valid: [bool; 5] = std::array::from_fn(|i| mask & (1 << i) != 0);
                        let enabled = valid[LINE_ENDING_OR_EOF as usize];
                        let mut lexer = TestLexer::new(&input);
                        let result = Scanner.scan(&mut lexer, &valid);
                        assert_eq!(result, enabled && accepted, "{input:?}, mask {mask}");

                        // C checks quotes only before whitespace is skipped;
                        // quote validity must not turn a failed line-ending
                        // attempt into a multiline string token.
                        let advances = if enabled {
                            length + usize::from(consumes_cr)
                        } else {
                            0
                        };
                        let mut expected: Vec<_> = (0..advances)
                            .map(|position| Event::Advance {
                                position,
                                skip: true,
                            })
                            .collect();
                        if result {
                            expected.push(Event::Symbol(LINE_ENDING_OR_EOF));
                        } else {
                            assert_eq!(lexer.symbol, Symbol::MAX);
                        }
                        assert_eq!(lexer.events, expected, "{input:?}, mask {mask}");
                        assert_eq!(lexer.lookahead_calls.get(), advances + 1);
                    }
                }
            }
        }
    }

    #[test]
    fn post_whitespace_dispatch_preserves_code_points_and_cr_rules() {
        for length in [1, 2, 7, 32, 64] {
            for terminator in (0..=127).chain([
                -1,
                0x100,
                0x109,
                0x10a,
                0x10d,
                0x120,
                0x122,
                0x127,
                i32::MIN,
                i32::MAX,
            ]) {
                // Spaces and tabs extend the prefix rather than end it. They
                // are already tested exhaustively in the preceding test;
                // this checks every other exit from that loop.
                if matches!(terminator, 0x09 | 0x20) {
                    continue;
                }
                for following in [0, 0x0a, 0x0d, 0x22, 0x27, 0x78] {
                    for mask in 0..32 {
                        let valid: [bool; 5] = std::array::from_fn(|i| mask & (1 << i) != 0);
                        let mut lexer = TestLexer::new("");
                        lexer.input.extend((0..length).map(|i| [0x20, 0x09][i % 2]));
                        lexer.input.extend([terminator, following, 0x78]);
                        let enabled = valid[LINE_ENDING_OR_EOF as usize];
                        let accepted = enabled
                            && (matches!(terminator, 0 | 0x0a)
                                || terminator == 0x0d && following == 0x0a);
                        let advances = if enabled {
                            length + usize::from(terminator == 0x0d)
                        } else {
                            0
                        };
                        assert_eq!(Scanner.scan(&mut lexer, &valid), accepted);
                        let mut expected: Vec<_> = (0..advances)
                            .map(|position| Event::Advance {
                                position,
                                skip: true,
                            })
                            .collect();
                        if accepted {
                            expected.push(Event::Symbol(LINE_ENDING_OR_EOF));
                        } else {
                            assert_eq!(lexer.symbol, Symbol::MAX);
                        }
                        assert_eq!(
                            lexer.events, expected,
                            "length {length}, terminator {terminator}, following {following}, mask {mask}"
                        );
                        assert_eq!(lexer.position, advances);
                        assert_eq!(lexer.lookahead_calls.get(), advances + 1);
                    }
                }
            }
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
                    // Only successful scans write a result symbol. C writes
                    // it earlier, but the runtime never reads it on failure.
                    let mut events = vec![];
                    if matches!(first, 0x09 | 0x0d | 0x20) {
                        events.push(Event::Advance {
                            position: 0,
                            skip: true,
                        });
                    }
                    let accepted = matches!(first, 0 | 0x0a);
                    if accepted {
                        events.push(Event::Symbol(LINE_ENDING_OR_EOF));
                    }
                    (accepted, events)
                } else {
                    (false, vec![])
                };
                assert_eq!(accepted, expected_accepted, "first {first}, mask {mask}");
                assert_eq!(lexer.events, expected_events, "first {first}, mask {mask}");
            }
        }
    }

    #[test]
    fn dispatch_does_not_truncate_line_break_or_delimiter_code_points() {
        for upper_bits in [0x100, 0x10000, 0x10ff00, i32::MIN] {
            for low_byte in [0, 0x09, 0x0a, 0x0d, 0x20, 0x22, 0x27] {
                let first = upper_bits | low_byte;
                for mask in 0..32 {
                    let valid: [bool; 5] = std::array::from_fn(|i| mask & (1 << i) != 0);
                    let mut lexer = TestLexer::new("x\n");
                    lexer.input[0] = first;
                    assert!(
                        !Scanner.scan(&mut lexer, &valid),
                        "first {first}, mask {mask}"
                    );
                    assert!(lexer.events.is_empty(), "first {first}, mask {mask}");
                    assert_eq!(lexer.lookahead_calls.get(), 1);
                }
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
