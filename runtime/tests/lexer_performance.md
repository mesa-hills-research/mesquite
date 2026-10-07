# Lexer hot-path optimization

## Changes

The remaining lexer hot path fetched the current input chunk through a trait
object for every byte, checked both range and chunk boundaries, and maintained
separate byte/scalar counters. Profiling and disassembly also showed register
saves for infrequent logging and boundary handling on the common path.

The lexer now keeps a fixed 128-byte copied window, limited to the current input
chunk and included range. An ASCII cache hit needs neither input dispatch nor
boundary handling. Non-ASCII bytes and unused slots use the original full-chunk
decoder and retry logic; cache refill never calls the read callback. One-byte
advances and token start initialization are inline; complex advances, cache
misses, and logged advances are out of line.

Scalar columns are represented by a wrapping offset from the byte position.
Ordinary byte advances leave that offset alone. Multibyte characters, BOM,
newlines, and range/goto relocation preserve C's scalar-column values explicitly.

There is no unsafe code, new allocation, retained input reference, change to
progress checkpoints, or change to callback/read ordering. The cache adds 132
bytes to lexer state. Windows of 32, 256, and 1024 bytes were also tried; 128 was
the final measured configuration.

## Measurement

Final comparison against main `9edb29b` (including the merged deterministic
stack-pop and subtree changes), using `run_oracle(inputs="benchmark")`:

- Main overall port/C: **1.013**.
- Three candidate runs: **0.986, 0.989, 0.988**.
- Median candidate overall: **0.988**, approximately **2.5% less parse time**.
- Reported overall noise: **1.9%**.
- Every language's median was faster than main.

Rounded per-language medians reported by those runs:

| Language | Main port/C | Candidate port/C |
| --- | ---: | ---: |
| C | 0.99 | 0.96 |
| C++ | 1.03 | 1.01 |
| Go | 0.99 | 0.98 |
| Java | 0.96 | 0.93 |
| JavaScript | 1.06 | 1.03 |
| Python | 1.03 | 0.98 |
| Rust | 1.03 | 1.01 |
| TSX | 1.00 | 0.97 |
| TypeScript | 1.03 | 1.01 |

## Validation

- Oracle gate: **4,712 / 4,712**, incremental seed 7 and queries enabled;
  no differences, panics, crashes, timeouts, or skips.
- `cargo test -p ts_port`: all 185 unit tests and both integration tests pass.
- `cargo clippy -p ts_port --all-targets -- -D warnings`: clean.
- New tests cover cache/window/chunk crossings, split Unicode retry, embedded
  NUL, newlines, skipping, range shrinkage, same-byte goto relocation, and
  wrapping scalar columns. Existing UTF-16, BOM, malformed input, EOF, empty
  range, scanner rollback, and incremental tests continue to pass.

The full kernel/fresh repository comparison is left to the host merge gate.
