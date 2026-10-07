# Perl scanner port handoff

The operator stopped this task during initial source inspection. No scanner code
has been translated: `src/scanner.rs` is still the original host stub. No build,
unit tests, or differential oracle were run for this task.

## Sources

Translate the read-only sources under
`/work/ts-port-run4b/sources/grammars/perl/src/`: `scanner.c` (1612 lines),
`tsp_unicode.h` (1226), `bsearch.h` (90), `tsp_keywords.h` (33),
`tsp_intuit_more.h` (210), `tsp_intuit_keywords.h` (1165), and
`tsp_intuit_readline.h` (207).

Keep `create()` and the `ExternalScanner` implementation interface. Follow
`runtime/PORTING.md`, and consult `/work/ts-port-run4b/lessons.md`. Do not edit
host-owned tables or generated grammar files.

## Inspected details

- `TSPString` retains the first eight i32 codepoints but records the full length;
  equality checks full lengths and the retained prefix.
- Quote opener/closer lookup starts at the innermost quote. The one-shot
  `body_leads_with_delim` flag affects regex bracket heuristics.
- Heredocs use an eight-entry FIFO. Overflow overwrites the last slot. Finishing
  the front re-arms HEREDOC_START if entries remain and clears the vacated slot.
- Custom `tsp_strchr` compares the codepoint directly with promoted characters,
  including the terminating NUL, without libc-style byte narrowing.
- Unicode ranges are half-open, unlike tables' inclusive `CharacterRange`.
- Preserve consumed input and marks across forward jumps to `kw_autoquote`,
  `fat_comma_check`, and `heredoc_token_handling`.
- The final `<<` probe intentionally has no valid-symbol guard.
- Snapshot routines and the intuit headers still require detailed inspection.

## Remaining work

Implement the whole scanner and helpers, add tests, run
`cargo check --workspace --all-targets`, run the Perl differential oracle when
available, and commit. This is an interruption note, not a completed translation.
