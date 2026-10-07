# Runtime translation contract — tree-sitter 0.25.10

Read this before translating a unit. The C source is the algorithmic specification;
this document fixes representation, ownership, signatures and division of work.
The initial skeleton compiles but intentionally does not parse. Replace only your
unit's tagged stubs, add local helpers/tests as needed, and preserve other units'
interfaces. Do not replace an unimplemented dependency with an approximation.

## Modules and scope

`language`, `lexer`, `subtree`, `stack`, `parser`, `node`, `tree_cursor`, `tree`, and
`get_changed_ranges` mirror the corresponding C files. Header-only modules are
`point`, `length`, `error_costs`, `reduce_action`, `reusable_node`, `array`, `clock`,
`alloc`, `atomic`, `host`, `ts_assert`, and `unicode`. `types` contains internal C-width
value types and the input adapters. `api` is the official Rust binding surface,
implemented as inherent methods on runtime types, and re-exported by `lib.rs`.
There is no FFI, build script, C compiler, or dependency on the reference runtime.
Only the host-owned `ts_port_tables` crate is needed.

The official binding subset in the task is exposed, including `LanguageRef`, errors,
progress options, named/non-named iteration, offsets, editing and incremental
comparison. Queries, wasm, public lookahead iterators, custom decoder APIs and
public UTF-16 parsing are out of scope. Internal lookahead iteration **is required**
by error recovery and belongs to `language`. The optional public logger is omitted:
the official logger accepts non-Send captures, incompatible with safe storage in a
Send parser. An internal Send logger is provided for runtime diagnostics. No unsafe
Send implementation or callback lifetime extension is allowed.

Plan artifacts: entries named `if` are preprocessor macro bodies, not functions;
they have no stubs. The two wasm-store functions are excluded deliberately.
Repeated Windows/POSIX `_ts_dup` and DOT-output definitions map to one portable
function apiece, using `File::try_clone`/`io::Write` instead of native descriptors.
All other planned runtime functions have stubs; additional necessary language.h
and unicode.h helpers are assigned to language/lexer respectively.

## Coordinates and arithmetic

* Public `Point`, `Range`, `InputEdit` match the binding: byte/row/column values are
  `usize`. Internal `point::Point`, `types::Range`, `types::InputEdit`, and `Length`
  use `u32`, matching C memory size and arithmetic. Conversions intentionally use
  `as u32`, as the official binding does. Do not make internal coordinates usize.
* `Length` combines byte count with a relative point extent. `point_add` is **not**
  vector addition: a nonzero right row resets the resulting column to the right
  column. `point_sub` has C's special saturating-column behavior. All small length
  and point functions are already implemented and tested.
* Explicit `wrapping_add/sub` where C unsigned arithmetic wraps (especially reverse
  iteration and sentinels). Do not replace C operations with global saturation.
  Signed comparisons/casts such as `(int8_t)child_index == -1` must be preserved.
* Undefined length is `{0, {0, 1}}`; MAX uses `u32::MAX`, not `usize::MAX`.
  Ranges are half-open. Points count **bytes**, not Unicode scalar columns.
* Error costs and parser limits are exact constants, not tuning knobs. In
  particular inline size tests are `< 255`, **not** `<= 255` or `< 256`.

## Subtrees: values, sharing, mutation, release

`Subtree` is `Null | Inline(InlineLeaf) | Heap(Arc<SubtreeHeapData>)`. Null replaces
`NULL_SUBTREE` and is not an allocated leaf. On 64-bit targets the handle is 16
bytes versus C's 8; `InlineLeaf` itself is 8 bytes. This is the explicit safe-Rust
tradeoff: no pointer tagging, unions, provenance tricks, or allocation for the
common small leaves. Heap children are a contiguous Vec rather than the allocation
prefix immediately preceding the C header. This adds a child-buffer allocation;
measure it before proposing a different layout. Do not heap-box every leaf.

Inline flag bits are named constants. `padding_rows_and_lookahead` has rows in the
low nibble and lookahead byte count in the high nibble; padding columns occupy a
full byte. Symbol fits in u8. Keep **C's** inline eligibility conditions, including
external tokens and symbol bounds. Heap flags have explicit bool fields. The C
payload union becomes `SubtreePayload::{Leaf, Branch, External, Error}`. Branch
summaries live in `BranchData`; child count comes from children.len(), never an
independently maintained count. `FirstLeaf` stores symbol/state, not a reference.

`MutableSubtree` is an alias for an **owned** Subtree, not a second pointer type.
`heap_mut()` uses `Arc::make_mut`; mutating a shared heap without COW is forbidden.
All `ts_subtree_*` header accessors are implemented. Use these instead of duplicating
inline/heap distinctions throughout the parser. `branch()` provides summary data.
`ptr_eq` means C union-word identity: inline equality by packed value, heap equality
by `Arc::ptr_eq`, null equal only to null. It is not recursive structural equality.

Ownership rules for signatures:

* `&Subtree` borrows without retaining; `Subtree` transfers one owning reference.
* A C retain becomes `clone()` or `ts_subtree_retain(&tree) -> Subtree`; the caller
  must keep that returned handle. A C release consumes its owned handle.
* `ts_subtree_clone` means a new header and shallowly retained children, **not**
  the cheap derived Subtree clone. The derived clone is the C retain equivalent.
* `ts_subtree_new_node` and `new_error_node` consume their child Vec. Use
  `mem::take(&mut scratch)` when translating a C array-transfer operation.
* `ts_subtree_make_mut` returns an owned unique/COW-ready handle. Subsequent
  modifications still use `heap_mut` or `Arc::make_mut`. The C unsafe mutable
  reinterpretation helper is an ownership move here and contains no unsafe code.
* Pools store uniquely owned `Arc<SubtreeHeapData>` handles, permitting reuse of
  the heap allocation, not just the header value. Pool allocate/free return/take
  Arc handles. Only pool uniquely owned, drained data, with C's 32-entry limit.
  Use `Arc::get_mut` to reset a pooled allocation. Never keep a shared heap in
  the free pool. Do not clone a pooled handle merely to borrow it.
* Heap Drop already drains uniquely owned descendants with an explicit worklist.
  Preserve this: deeply nested trees must not recurse through Rust destructors.
  The subtree-2 release algorithm may use its pool/worklist as well. No recursive
  deep clones, recursive parent storage, or reference cycles.

`ExternalScannerState` stores up to 24 bytes inline, larger states in `Arc<[u8]>`.
State comparison is byte equality with length checks, not allocation identity.
Copying long immutable scanner snapshots shares bytes safely.

## Graph-structured stack

The stack is a parser-local arena. `StackNodeId(usize)` indexes
`StackArena.nodes: Vec<Option<StackNode>>`; links and heads store IDs, never Rust
references into the reallocating Vec. Nodes have explicit C-style u32 refcounts,
fixed eight-link slots (`[Option<StackLink>; 8]`) and link_count. This preserves
shared link mutation without Rc/RefCell, locks, unsafe pointers, or deep copying.

Reference counts are counts of **graph ownership**, not copies of the index.
Retain/release explicitly when C does. Index copying by itself does not retain.
A slot may be recycled only when all live heads/links/iterators that own it have
released it; `arena.free` holds vacant slot indices. Use an explicit release
worklist, not recursive traversal. Vacant index reuse is an arena bookkeeping
operation, not a change to MAX_LINK_COUNT/MAX_ITERATOR_COUNT semantics. The arena
may keep capacity up to its high-water mark; unlike C's individually allocated
nodes there is no need to free each unused index after the 50-node cache limit.

`Stack` does not borrow its owner's subtree pool. Every operation needing it
accepts `&mut SubtreePool` explicitly. At parser call sites split borrows of
`parser.stack` and `parser.tree_pool`. Version remains u32 and NONE is u32::MAX.
Pop APIs return owned Vec<StackSlice>; do not leave aliases to scratch slice storage.
`stack__iter` accepts a closure over typed callback state plus an immutable arena
view, replacing void* payloads. Pop-error captures `&mut bool`; summary captures
`SummarizeStackSession`; callbacks inspect predecessor nodes through the arena.
Order of links, versions, slices and summaries is semantically important.

## Parser, input, lexer, scanner and progress

The public `Parser` is the persistent runtime state. It owns stack, pool, lexer
state, scanner, reusable-node path, token cache, old/finished roots and scratch
buffers. It is automatically Send. `Language` is a Copy/Clone handle to
`&'static LanguageTables`; conversion from that reference is implemented. Grammar
strings/action runs can therefore be returned with 'static lifetimes, without
leaking freshly allocated strings. Actions are the host's header-free slices of
`ParseActionEntry`; use `.action()` on each action entry, do not reinterpret a
slice or allocate a new action Vec for every lookup. TableEntry count is len().

`LexerState` is persistent data, while `Lexer<'a>` is a short-lived adapter
implementing `ts_port_tables::Lexer`. It borrows state, input, and an optional
logger, and is constructed only while calling a generated lexer/scanner. State-only
functions (goto/reset/finish/mark_end/set_input/included-ranges/column bookkeeping)
accept LexerState; functions that actually read input accept the adapter.

`types::Input` has `read(byte, point)` and `chunk()`. The provider owns/borrows the
last returned chunk. `CallbackInput<F,T>` stores `Option<T>` and calls the client's
callback only on read. `SliceInput` borrows the complete source and changes a
start index. No copy of the entire remaining document is made on each chunk read.
The lexer tracks chunk_start/chunk_size and invalidates this cache at input changes.
Empty chunks mean EOF according to the C lexer. Never retain a chunk reference
across another read. Decode malformed UTF-8 exactly as C, including consumed bytes
and -1 lookahead; lexer also owns the three unicode decoding helper stubs.

`ParseContext<'input,'options>` holds the borrowed Input and ParseOptions. Functions
that lex/check progress/advance/balance explicitly receive this context. Neither
client closures nor input data references are stored in Parser. They may be non-Send
and are dropped before parse returns, even after cancellation. The next parse
re-attaches input and options to the outstanding persistent state. Do not manufacture
'static lifetimes, retain raw callback pointers, or eagerly read the whole input.

External scanners come from tables.external_scanner.create, as
`Box<dyn ExternalScanner + Send>` (Send is a trait super-bound). Drop destroys them;
serialize/deserialize use the parser's 1024-byte scanner buffer and subtree state.
Respect the exact timing of deserialization, failed scans, retries and keyword lexing.

**Progress parity is required.** `operation_count`, 100-operation checkpoint,
position/has_error updates, scanner/recovery work and balancing's scaled operation
increments must match parser.c in order. Do not call progress per token, per byte,
or once at parse completion. Cancellation returns None with the same resumable
state. Timeout uses monotonic Instant; callback count is independent of wall time.
Cancellation-flag internals use owned Arc<AtomicUsize>, relaxed loads; the old unsafe
public borrowed flag API is outside this subset. No extra locks/atomics in the stack.

`ts_parser__breakdown_lookahead` uses parser.reusable_node directly; its redundant
C parameter was removed to avoid borrowing the parser and its own field mutably
at once. Other borrow conflicts should be resolved with scoped field borrows or
`mem::take` followed by restoration, never unsafe aliases or wholesale state clones.

## Trees, nodes, cursors, changes and API

Tree owns a **Box<Subtree> root slot**, Language and included-range Vec. Boxing keeps
the root node's id stable when the Tree value moves. Tree clone creates a new root
slot but shares heap subtrees; mutation is COW, so borrowed views forbid edits on
that Tree while live. Tree and Language are automatically Send + Sync + Clone.
Tree::clone itself is an api stub delegating to tree's C-copy translation.

Node<'tree> is Copy: references to its Tree and exact subtree **slot**, absolute
Length, and alias symbol. Its id is the slot's address cast to usize, not the symbol,
byte range, Arc header pointer or a global counter. Pointer-to-integer conversion
of a live reference requires no unsafe. Node existence is represented by Option;
there is no invalid Node containing null references. Internal null helpers return
Option<Node>. Parent/sibling navigation searches from the tree per C, without
adding parent links to shared subtrees. Aliases affect public kind/namedness, not
grammar kind. Anonymous, hidden, missing, extra, and ERROR semantics are distinct.

TreeCursor borrows the Tree and maintains a Vec of borrowed TreeCursorEntry slots.
Entries carry both child and structural-child indices and descendant index; keep
all three. Hidden-node flattening and aliases must be translated, not approximated.
For changed-range traversal only, cursor.tree may be None; that iterator owns its
Language separately and operates directly on subtree entries, never constructing
public Nodes. `current_status` returns a CursorStatus record in place of six out
parameters. Child iteration returns Option<(entry, visible)> instead of bool plus
out-parameters; node iteration returns Option<Node>. Negative C cursor indices
become Option<u32> internally and Option<usize> at the binding boundary.

`get_changed_ranges` owns RangeIterator cursor values. Its public runtime function
returns a Vec<internal Range>; API maps these to public Range and an ExactSizeIterator.
Always use C's included-range difference merging and visible-state comparisons.

API methods retain the binding's exact generic and lifetime signatures, including
opaque iterator returns and mutable cursor borrows. Rust 2024 precise `use<...>`
capture bounds prevent accidentally retaining the temporary Node/self reference
that the original edition-2021 binding did not capture. Children/FieldChildren are
concrete backing types, not eager Vec collections. Their Iterator methods belong
to api. A temporary local `todo` macro in api.rs expands to literal const-compatible
panics; typed arms provide backing iterator types for otherwise uninferable opaque
stub return types. This keeps each method body exactly one unit-tagged invocation
while preserving the official const getters and opaque signatures. Delete the
macro and `pending` helper after replacing all API stubs. Standard library todo
with a formatted message cannot compile in a const fn on the current toolchain.

Errors use Result/Option at the same API points as the binding. Invalid included
ranges report IncludedRangesError(index); language errors report the ABI version;
invalid UTF-8 text access uses str::Utf8Error. Malformed *source* is parsed into
error nodes, never converted to a Rust panic. Corrupt internal invariants and
malformed grammar tables may assert/panic. Allocation failure follows the Rust
allocator OOM policy, not a silently failed parse. DOT output uses io::Result.

## C idioms, safety and performance

* Array(T) -> Vec<T>; pointer/count -> slice; capacity is not length. Use reverse
  iterators or checked/wrapping indices as appropriate, with original visitation
  order. `array_clear` retains capacity; `array_delete` drops it. Array ownership
  assignments usually require mem::take, not clone.
* Tagged unions -> enums; pointer arithmetic -> indexed slices/arena IDs. No
  reference into a Vec may survive push/realloc or mutation through another owner.
* C out parameters -> return values/tuples/records where signatures specify them.
  An empty static slice denotes absent aliases/field maps; field ID zero is none.
* C access macros -> implemented functions; generated grammar macros already live
  in the host tables/lex modules, which must not be modified.
* Use ts_assert! for C assertions: it evaluates its operand even in release mode.
  debug_assert! alone would omit side effects. Rust indexing retains safety checks.
* alloc.h -> Box/Vec/Arc and the global Rust allocator. No C allocator hooks.
  atomic.h refcounts -> Arc; clock.h -> Option<Instant>/microsecond duration;
  host.h -> target cfg constants. These support pieces are complete.
* No unsafe unless truly unavoidable, and every use needs a precise `// SAFETY:`
  justification. The skeleton has zero unsafe blocks and no manual Send/Sync impls.
  Do not use the c2rust translation's ownership/pointer style.
* Target parse throughput within 1.5x C. Retain scratch capacity, avoid per-token
  String creation/table copying, preserve pooling and inline leaves, borrow rather
  than clone where ownership does not change. Keep traversal/release iterative.
  Do not optimize by changing error costs, pruning limits, scanner calls, progress
  checkpoints or tie-breaking. Any representation optimization needs measurement
  and oracle validation, not just a local microbenchmark.

## Validation and handoff

Run `cargo check --workspace --all-targets`, `cargo test -p ts_port`, and
`cargo clippy -p ts_port --all-targets`. Format **runtime only** (`rustfmt --edition
2024 runtime/src/*.rs`), not workspace-wide: grammar/table files are host-owned.
The initial skeleton allows unused items and C double-underscore names. Its
clippy::ptr_arg allowance is temporary: stub bodies cannot demonstrate that Vec
and String out-buffers grow. Remove unnecessary allowances once units are filled.

The skeleton's C oracle dumper compiles; all 824 C inputs currently stop at
`api: Parser::new`, as expected. Six support/layout/thread-trait tests passed before
handoff. No parse behavior has been claimed or validated yet. Translator units
must progressively eliminate their tagged stubs and run the differential oracle.

## Complete stub inventory

Names below are preserved verbatim for internal C helpers, including `__` (which
avoids collisions with their public C wrappers). Paths are relative to `runtime/`.
For api rows the function column names the official binding item rather than a
C function, because many such conveniences have no single C counterpart. Line
numbers identify the initial skeleton; use the function name after edits.

### api (116 stubs)

| C function / binding item | Rust stub location |
|---|---|
| `Language::name` | `src/api.rs:117` |
| `Language::version` | `src/api.rs:120` |
| `Language::abi_version` | `src/api.rs:123` |
| `Language::metadata` | `src/api.rs:126` |
| `Language::node_kind_count` | `src/api.rs:129` |
| `Language::parse_state_count` | `src/api.rs:132` |
| `Language::supertypes` | `src/api.rs:135` |
| `Language::subtypes_for_supertype` | `src/api.rs:138` |
| `Language::node_kind_for_id` | `src/api.rs:141` |
| `Language::id_for_node_kind` | `src/api.rs:144` |
| `Language::node_kind_is_named` | `src/api.rs:147` |
| `Language::node_kind_is_visible` | `src/api.rs:150` |
| `Language::node_kind_is_supertype` | `src/api.rs:153` |
| `Language::field_count` | `src/api.rs:156` |
| `Language::field_name_for_id` | `src/api.rs:159` |
| `Language::field_id_for_name` | `src/api.rs:162` |
| `Language::next_state` | `src/api.rs:165` |
| `Parser::new` | `src/api.rs:171` |
| `Parser::set_language` | `src/api.rs:174` |
| `Parser::language` | `src/api.rs:177` |
| `Parser::parse` | `src/api.rs:180` |
| `Parser::parse_with` | `src/api.rs:187` |
| `Parser::parse_with_options` | `src/api.rs:195` |
| `Parser::reset` | `src/api.rs:198` |
| `Parser::timeout_micros` | `src/api.rs:201` |
| `Parser::set_timeout_micros` | `src/api.rs:204` |
| `Parser::set_included_ranges` | `src/api.rs:207` |
| `Parser::included_ranges` | `src/api.rs:210` |
| `Tree::root_node` | `src/api.rs:216` |
| `Tree::root_node_with_offset` | `src/api.rs:219` |
| `Tree::language` | `src/api.rs:222` |
| `Tree::edit` | `src/api.rs:225` |
| `Tree::walk` | `src/api.rs:228` |
| `Tree::changed_ranges` | `src/api.rs:231` |
| `Tree::included_ranges` | `src/api.rs:234` |
| `Node::id` | `src/api.rs:240` |
| `Node::kind_id` | `src/api.rs:243` |
| `Node::grammar_id` | `src/api.rs:246` |
| `Node::kind` | `src/api.rs:249` |
| `Node::grammar_name` | `src/api.rs:252` |
| `Node::language` | `src/api.rs:255` |
| `Node::is_named` | `src/api.rs:258` |
| `Node::is_extra` | `src/api.rs:261` |
| `Node::has_changes` | `src/api.rs:264` |
| `Node::has_error` | `src/api.rs:267` |
| `Node::is_error` | `src/api.rs:270` |
| `Node::parse_state` | `src/api.rs:273` |
| `Node::next_parse_state` | `src/api.rs:276` |
| `Node::is_missing` | `src/api.rs:279` |
| `Node::start_byte` | `src/api.rs:282` |
| `Node::end_byte` | `src/api.rs:285` |
| `Node::byte_range` | `src/api.rs:288` |
| `Node::range` | `src/api.rs:291` |
| `Node::start_position` | `src/api.rs:294` |
| `Node::end_position` | `src/api.rs:297` |
| `Node::child` | `src/api.rs:300` |
| `Node::child_count` | `src/api.rs:303` |
| `Node::named_child` | `src/api.rs:306` |
| `Node::named_child_count` | `src/api.rs:309` |
| `Node::child_by_field_name` | `src/api.rs:312` |
| `Node::child_by_field_id` | `src/api.rs:315` |
| `Node::field_name_for_child` | `src/api.rs:318` |
| `Node::field_name_for_named_child` | `src/api.rs:321` |
| `Node::children` | `src/api.rs:327` |
| `Node::named_children` | `src/api.rs:333` |
| `Node::children_by_field_name` | `src/api.rs:340` |
| `Node::children_by_field_id` | `src/api.rs:347` |
| `Node::parent` | `src/api.rs:350` |
| `Node::child_with_descendant` | `src/api.rs:353` |
| `Node::next_sibling` | `src/api.rs:356` |
| `Node::prev_sibling` | `src/api.rs:359` |
| `Node::next_named_sibling` | `src/api.rs:362` |
| `Node::prev_named_sibling` | `src/api.rs:365` |
| `Node::first_child_for_byte` | `src/api.rs:368` |
| `Node::first_named_child_for_byte` | `src/api.rs:371` |
| `Node::descendant_count` | `src/api.rs:374` |
| `Node::descendant_for_byte_range` | `src/api.rs:377` |
| `Node::named_descendant_for_byte_range` | `src/api.rs:380` |
| `Node::descendant_for_point_range` | `src/api.rs:383` |
| `Node::named_descendant_for_point_range` | `src/api.rs:386` |
| `Node::to_sexp` | `src/api.rs:389` |
| `Node::utf8_text` | `src/api.rs:392` |
| `Node::utf16_text` | `src/api.rs:395` |
| `Node::walk` | `src/api.rs:398` |
| `Node::edit` | `src/api.rs:401` |
| `TreeCursor::node` | `src/api.rs:407` |
| `TreeCursor::field_id` | `src/api.rs:410` |
| `TreeCursor::field_name` | `src/api.rs:413` |
| `TreeCursor::depth` | `src/api.rs:416` |
| `TreeCursor::descendant_index` | `src/api.rs:419` |
| `TreeCursor::goto_first_child` | `src/api.rs:422` |
| `TreeCursor::goto_last_child` | `src/api.rs:425` |
| `TreeCursor::goto_parent` | `src/api.rs:428` |
| `TreeCursor::goto_next_sibling` | `src/api.rs:431` |
| `TreeCursor::goto_descendant` | `src/api.rs:434` |
| `TreeCursor::goto_previous_sibling` | `src/api.rs:437` |
| `TreeCursor::goto_first_child_for_byte` | `src/api.rs:440` |
| `TreeCursor::goto_first_child_for_point` | `src/api.rs:443` |
| `TreeCursor::reset` | `src/api.rs:446` |
| `TreeCursor::reset_to` | `src/api.rs:449` |
| `ParseState::current_byte_offset` | `src/api.rs:455` |
| `ParseState::has_error` | `src/api.rs:458` |
| `ParseOptions::new` | `src/api.rs:464` |
| `ParseOptions::progress_callback` | `src/api.rs:467` |
| `Parser::default` | `src/api.rs:473` |
| `Tree::clone` | `src/api.rs:479` |
| `TreeCursor::clone` | `src/api.rs:485` |
| `Node::fmt` | `src/api.rs:491` |
| `Node::eq` | `src/api.rs:497` |
| `Node::hash` | `src/api.rs:503` |
| `Node::fmt_display` | `src/api.rs:509` |
| `LanguageError::fmt` | `src/api.rs:515` |
| `IncludedRangesError::fmt` | `src/api.rs:521` |
| `Children::next` | `src/api.rs:532` |
| `Children::size_hint` | `src/api.rs:535` |
| `FieldChildren::next` | `src/api.rs:542` |

### get_changed_ranges (15 stubs)

| C function / binding item | Rust stub location |
|---|---|
| `ts_range_array_add` | `src/get_changed_ranges.rs:19` |
| `ts_range_array_intersects` | `src/get_changed_ranges.rs:28` |
| `ts_range_array_get_changed_ranges` | `src/get_changed_ranges.rs:36` |
| `iterator_new` | `src/get_changed_ranges.rs:44` |
| `iterator_done` | `src/get_changed_ranges.rs:48` |
| `iterator_start_position` | `src/get_changed_ranges.rs:52` |
| `iterator_end_position` | `src/get_changed_ranges.rs:56` |
| `iterator_tree_is_visible` | `src/get_changed_ranges.rs:60` |
| `iterator_get_visible_state` | `src/get_changed_ranges.rs:66` |
| `iterator_ascend` | `src/get_changed_ranges.rs:70` |
| `iterator_descend` | `src/get_changed_ranges.rs:74` |
| `iterator_advance` | `src/get_changed_ranges.rs:78` |
| `iterator_compare` | `src/get_changed_ranges.rs:85` |
| `iterator_print_state` | `src/get_changed_ranges.rs:89` |
| `ts_subtree_get_changed_ranges` | `src/get_changed_ranges.rs:100` |

### language (43 stubs)

| C function / binding item | Rust stub location |
|---|---|
| `ts_language_copy` | `src/language.rs:52` |
| `ts_language_delete` | `src/language.rs:56` |
| `ts_language_symbol_count` | `src/language.rs:60` |
| `ts_language_state_count` | `src/language.rs:64` |
| `ts_language_supertypes` | `src/language.rs:68` |
| `ts_language_subtypes` | `src/language.rs:72` |
| `ts_language_version` | `src/language.rs:76` |
| `ts_language_abi_version` | `src/language.rs:80` |
| `ts_language_metadata` | `src/language.rs:84` |
| `ts_language_name` | `src/language.rs:88` |
| `ts_language_field_count` | `src/language.rs:92` |
| `ts_language_table_entry` | `src/language.rs:100` |
| `ts_language_lex_mode_for_state` | `src/language.rs:104` |
| `ts_language_is_reserved_word` | `src/language.rs:112` |
| `ts_language_symbol_metadata` | `src/language.rs:116` |
| `ts_language_public_symbol` | `src/language.rs:120` |
| `ts_language_next_state` | `src/language.rs:128` |
| `ts_language_symbol_name` | `src/language.rs:132` |
| `ts_language_symbol_for_name` | `src/language.rs:136` |
| `ts_language_symbol_type` | `src/language.rs:140` |
| `ts_language_field_name_for_id` | `src/language.rs:147` |
| `ts_language_field_id_for_name` | `src/language.rs:151` |
| `ts_lookahead_iterator_new` | `src/language.rs:158` |
| `ts_lookahead_iterator_delete` | `src/language.rs:162` |
| `ts_lookahead_iterator_reset_state` | `src/language.rs:169` |
| `ts_lookahead_iterator_language` | `src/language.rs:173` |
| `ts_lookahead_iterator_reset` | `src/language.rs:181` |
| `ts_lookahead_iterator_next` | `src/language.rs:185` |
| `ts_lookahead_iterator_current_symbol` | `src/language.rs:189` |
| `ts_lookahead_iterator_current_symbol_name` | `src/language.rs:195` |
| `ts_language_actions` | `src/language.rs:203` |
| `ts_language_has_reduce_action` | `src/language.rs:211` |
| `ts_language_lookup` | `src/language.rs:215` |
| `ts_language_has_actions` | `src/language.rs:219` |
| `ts_language_lookaheads` | `src/language.rs:223` |
| `ts_lookahead_iterator__next` | `src/language.rs:227` |
| `ts_language_alias_sequence` | `src/language.rs:234` |
| `ts_language_alias_at` | `src/language.rs:242` |
| `ts_language_field_map` | `src/language.rs:249` |
| `ts_language_aliases_for_symbol` | `src/language.rs:256` |
| `ts_language_state_is_primary` | `src/language.rs:260` |
| `ts_language_enabled_external_tokens` | `src/language.rs:267` |
| `ts_language_write_symbol_as_dot_string` | `src/language.rs:274` |

### lexer (26 stubs)

| C function / binding item | Rust stub location |
|---|---|
| `ts_lexer__set_column_data` | `src/lexer.rs:62` |
| `ts_lexer__increment_column_data` | `src/lexer.rs:66` |
| `ts_lexer__invalidate_column_data` | `src/lexer.rs:70` |
| `ts_lexer__eof` | `src/lexer.rs:74` |
| `ts_lexer__clear_chunk` | `src/lexer.rs:78` |
| `ts_lexer__get_chunk` | `src/lexer.rs:82` |
| `ts_lexer__get_lookahead` | `src/lexer.rs:86` |
| `ts_lexer_goto` | `src/lexer.rs:90` |
| `ts_lexer__do_advance` | `src/lexer.rs:94` |
| `ts_lexer__advance` | `src/lexer.rs:98` |
| `ts_lexer__mark_end` | `src/lexer.rs:102` |
| `ts_lexer__get_column` | `src/lexer.rs:106` |
| `ts_lexer__is_at_included_range_start` | `src/lexer.rs:110` |
| `ts_lexer__log` | `src/lexer.rs:114` |
| `ts_lexer_init` | `src/lexer.rs:118` |
| `ts_lexer_delete` | `src/lexer.rs:122` |
| `ts_lexer_set_input` | `src/lexer.rs:126` |
| `ts_lexer_reset` | `src/lexer.rs:130` |
| `ts_lexer_start` | `src/lexer.rs:134` |
| `ts_lexer_finish` | `src/lexer.rs:138` |
| `ts_lexer_mark_end` | `src/lexer.rs:142` |
| `ts_lexer_set_included_ranges` | `src/lexer.rs:146` |
| `ts_lexer_included_ranges` | `src/lexer.rs:150` |
| `ts_decode_utf8` | `src/unicode.rs:6` |
| `ts_decode_utf16_le` | `src/unicode.rs:10` |
| `ts_decode_utf16_be` | `src/unicode.rs:14` |

### node-1 (18 stubs)

| C function / binding item | Rust stub location |
|---|---|
| `ts_node_new` | `src/node.rs:26` |
| `ts_node__null` | `src/node.rs:30` |
| `ts_node_start_byte` | `src/node.rs:34` |
| `ts_node_start_point` | `src/node.rs:38` |
| `ts_node__alias` | `src/node.rs:42` |
| `ts_node__subtree` | `src/node.rs:46` |
| `ts_node_iterate_children` | `src/node.rs:50` |
| `ts_node_child_iterator_done` | `src/node.rs:54` |
| `ts_node_child_iterator_next` | `src/node.rs:60` |
| `ts_node__is_relevant` | `src/node.rs:64` |
| `ts_node__relevant_child_count` | `src/node.rs:68` |
| `ts_node__child` | `src/node.rs:76` |
| `ts_subtree_has_trailing_empty_descendant` | `src/node.rs:80` |
| `ts_node__prev_sibling` | `src/node.rs:84` |
| `ts_node__next_sibling` | `src/node.rs:88` |
| `ts_node__first_child_for_byte` | `src/node.rs:96` |
| `ts_node__descendant_for_byte_range` | `src/node.rs:105` |
| `ts_node__descendant_for_point_range` | `src/node.rs:114` |

### node-2 (41 stubs)

| C function / binding item | Rust stub location |
|---|---|
| `ts_node_end_byte` | `src/node.rs:118` |
| `ts_node_end_point` | `src/node.rs:122` |
| `ts_node_symbol` | `src/node.rs:126` |
| `ts_node_type` | `src/node.rs:130` |
| `ts_node_language` | `src/node.rs:134` |
| `ts_node_grammar_symbol` | `src/node.rs:138` |
| `ts_node_grammar_type` | `src/node.rs:142` |
| `ts_node_string` | `src/node.rs:146` |
| `ts_node_eq` | `src/node.rs:150` |
| `ts_node_is_null` | `src/node.rs:154` |
| `ts_node_is_extra` | `src/node.rs:158` |
| `ts_node_is_named` | `src/node.rs:162` |
| `ts_node_is_missing` | `src/node.rs:166` |
| `ts_node_has_changes` | `src/node.rs:170` |
| `ts_node_has_error` | `src/node.rs:174` |
| `ts_node_is_error` | `src/node.rs:178` |
| `ts_node_descendant_count` | `src/node.rs:182` |
| `ts_node_parse_state` | `src/node.rs:186` |
| `ts_node_next_parse_state` | `src/node.rs:190` |
| `ts_node_parent` | `src/node.rs:194` |
| `ts_node_child_with_descendant` | `src/node.rs:201` |
| `ts_node_child` | `src/node.rs:205` |
| `ts_node_named_child` | `src/node.rs:209` |
| `ts_node_child_by_field_id` | `src/node.rs:213` |
| `ts_node__field_name_from_language` | `src/node.rs:220` |
| `ts_node_field_name_for_child` | `src/node.rs:227` |
| `ts_node_field_name_for_named_child` | `src/node.rs:234` |
| `ts_node_child_by_field_name` | `src/node.rs:241` |
| `ts_node_child_count` | `src/node.rs:245` |
| `ts_node_named_child_count` | `src/node.rs:249` |
| `ts_node_next_sibling` | `src/node.rs:253` |
| `ts_node_next_named_sibling` | `src/node.rs:257` |
| `ts_node_prev_sibling` | `src/node.rs:261` |
| `ts_node_prev_named_sibling` | `src/node.rs:265` |
| `ts_node_first_child_for_byte` | `src/node.rs:269` |
| `ts_node_first_named_child_for_byte` | `src/node.rs:273` |
| `ts_node_descendant_for_byte_range` | `src/node.rs:281` |
| `ts_node_named_descendant_for_byte_range` | `src/node.rs:289` |
| `ts_node_descendant_for_point_range` | `src/node.rs:297` |
| `ts_node_named_descendant_for_point_range` | `src/node.rs:305` |
| `ts_node_edit` | `src/node.rs:309` |

### parser-1 (16 stubs)

| C function / binding item | Rust stub location |
|---|---|
| `ts_string_input_read` | `src/parser.rs:80` |
| `ts_parser__log` | `src/parser.rs:84` |
| `ts_parser__breakdown_top_of_stack` | `src/parser.rs:91` |
| `ts_parser__breakdown_lookahead` | `src/parser.rs:99` |
| `ts_parser__compare_versions` | `src/parser.rs:107` |
| `ts_parser__version_status` | `src/parser.rs:111` |
| `ts_parser__better_version_exists` | `src/parser.rs:120` |
| `ts_parser__call_main_lex_fn` | `src/parser.rs:128` |
| `ts_parser__call_keyword_lex_fn` | `src/parser.rs:135` |
| `ts_parser__external_scanner_create` | `src/parser.rs:139` |
| `ts_parser__external_scanner_destroy` | `src/parser.rs:143` |
| `ts_parser__external_scanner_serialize` | `src/parser.rs:147` |
| `ts_parser__external_scanner_deserialize` | `src/parser.rs:154` |
| `ts_parser__external_scanner_scan` | `src/parser.rs:162` |
| `ts_parser__can_reuse_first_leaf` | `src/parser.rs:171` |
| `ts_parser__lex` | `src/parser.rs:180` |

### parser-2 (12 stubs)

| C function / binding item | Rust stub location |
|---|---|
| `ts_parser__get_cached_token` | `src/parser.rs:190` |
| `ts_parser__set_cached_token` | `src/parser.rs:199` |
| `ts_parser__has_included_range_difference` | `src/parser.rs:207` |
| `ts_parser__reuse_node` | `src/parser.rs:218` |
| `ts_parser__select_tree` | `src/parser.rs:222` |
| `ts_parser__select_children` | `src/parser.rs:230` |
| `ts_parser__shift` | `src/parser.rs:240` |
| `ts_parser__reduce` | `src/parser.rs:253` |
| `ts_parser__accept` | `src/parser.rs:257` |
| `ts_parser__do_all_potential_reductions` | `src/parser.rs:265` |
| `ts_parser__recover_to_state` | `src/parser.rs:274` |
| `ts_parser__recover` | `src/parser.rs:278` |

### parser-3 (10 stubs)

| C function / binding item | Rust stub location |
|---|---|
| `ts_parser__handle_error` | `src/parser.rs:286` |
| `ts_parser__check_progress` | `src/parser.rs:296` |
| `ts_parser__advance` | `src/parser.rs:305` |
| `ts_parser__condense_stack` | `src/parser.rs:309` |
| `ts_parser__balance_subtree` | `src/parser.rs:316` |
| `ts_parser_has_outstanding_parse` | `src/parser.rs:320` |
| `ts_parser_new` | `src/parser.rs:324` |
| `ts_parser_delete` | `src/parser.rs:328` |
| `ts_parser_language` | `src/parser.rs:332` |
| `ts_parser_set_language` | `src/parser.rs:336` |

### parser-4 (23 stubs)

| C function / binding item | Rust stub location |
|---|---|
| `ts_parser_logger` | `src/parser.rs:340` |
| `ts_parser_set_logger` | `src/parser.rs:344` |
| `ts_parser_print_dot_graphs` | `src/parser.rs:351` |
| `ts_parser_cancellation_flag` | `src/parser.rs:355` |
| `ts_parser_set_cancellation_flag` | `src/parser.rs:359` |
| `ts_parser_timeout_micros` | `src/parser.rs:363` |
| `ts_parser_set_timeout_micros` | `src/parser.rs:367` |
| `ts_parser_set_included_ranges` | `src/parser.rs:371` |
| `ts_parser_included_ranges` | `src/parser.rs:375` |
| `ts_parser_reset` | `src/parser.rs:379` |
| `ts_parser_parse` | `src/parser.rs:387` |
| `ts_parser_parse_with_options` | `src/parser.rs:396` |
| `ts_parser_parse_string` | `src/parser.rs:404` |
| `ts_parser_parse_string_encoding` | `src/parser.rs:413` |
| `reusable_node_new` | `src/reusable_node.rs:15` |
| `reusable_node_clear` | `src/reusable_node.rs:19` |
| `reusable_node_tree` | `src/reusable_node.rs:23` |
| `reusable_node_byte_offset` | `src/reusable_node.rs:27` |
| `reusable_node_delete` | `src/reusable_node.rs:31` |
| `reusable_node_advance` | `src/reusable_node.rs:35` |
| `reusable_node_descend` | `src/reusable_node.rs:39` |
| `reusable_node_advance_past_leaf` | `src/reusable_node.rs:43` |
| `reusable_node_reset` | `src/reusable_node.rs:47` |

### stack-1 (18 stubs)

| C function / binding item | Rust stub location |
|---|---|
| `stack_node_retain` | `src/stack.rs:85` |
| `stack_node_release` | `src/stack.rs:93` |
| `stack__subtree_node_count` | `src/stack.rs:97` |
| `stack_node_new` | `src/stack.rs:107` |
| `stack__subtree_is_equivalent` | `src/stack.rs:111` |
| `stack_node_add_link` | `src/stack.rs:120` |
| `stack_head_delete` | `src/stack.rs:128` |
| `ts_stack__add_version` | `src/stack.rs:136` |
| `ts_stack__add_slice` | `src/stack.rs:145` |
| `stack__iter` | `src/stack.rs:155` |
| `ts_stack_new` | `src/stack.rs:159` |
| `ts_stack_delete` | `src/stack.rs:163` |
| `ts_stack_version_count` | `src/stack.rs:167` |
| `ts_stack_halted_version_count` | `src/stack.rs:171` |
| `ts_stack_state` | `src/stack.rs:175` |
| `ts_stack_position` | `src/stack.rs:179` |
| `ts_stack_last_external_token` | `src/stack.rs:183` |
| `ts_stack_set_last_external_token` | `src/stack.rs:192` |

### stack-2 (30 stubs)

| C function / binding item | Rust stub location |
|---|---|
| `ts_stack_error_cost` | `src/stack.rs:196` |
| `ts_stack_node_count_since_error` | `src/stack.rs:200` |
| `ts_stack_push` | `src/stack.rs:211` |
| `pop_count_callback` | `src/stack.rs:215` |
| `ts_stack_pop_count` | `src/stack.rs:224` |
| `pop_pending_callback` | `src/stack.rs:228` |
| `ts_stack_pop_pending` | `src/stack.rs:236` |
| `pop_error_callback` | `src/stack.rs:240` |
| `ts_stack_pop_error` | `src/stack.rs:248` |
| `pop_all_callback` | `src/stack.rs:252` |
| `ts_stack_pop_all` | `src/stack.rs:260` |
| `summarize_stack_callback` | `src/stack.rs:268` |
| `ts_stack_record_summary` | `src/stack.rs:277` |
| `ts_stack_get_summary` | `src/stack.rs:281` |
| `ts_stack_dynamic_precedence` | `src/stack.rs:285` |
| `ts_stack_has_advanced_since_error` | `src/stack.rs:289` |
| `ts_stack_remove_version` | `src/stack.rs:297` |
| `ts_stack_renumber_version` | `src/stack.rs:306` |
| `ts_stack_swap_versions` | `src/stack.rs:310` |
| `ts_stack_copy_version` | `src/stack.rs:314` |
| `ts_stack_merge` | `src/stack.rs:323` |
| `ts_stack_can_merge` | `src/stack.rs:331` |
| `ts_stack_halt` | `src/stack.rs:335` |
| `ts_stack_pause` | `src/stack.rs:344` |
| `ts_stack_is_active` | `src/stack.rs:348` |
| `ts_stack_is_halted` | `src/stack.rs:352` |
| `ts_stack_is_paused` | `src/stack.rs:356` |
| `ts_stack_resume` | `src/stack.rs:360` |
| `ts_stack_clear` | `src/stack.rs:364` |
| `ts_stack_print_dot_graph` | `src/stack.rs:372` |

### subtree-1 (25 stubs)

| C function / binding item | Rust stub location |
|---|---|
| `ts_external_scanner_state_init` | `src/subtree.rs:115` |
| `ts_external_scanner_state_copy` | `src/subtree.rs:119` |
| `ts_external_scanner_state_delete` | `src/subtree.rs:123` |
| `ts_external_scanner_state_data` | `src/subtree.rs:127` |
| `ts_external_scanner_state_eq` | `src/subtree.rs:131` |
| `ts_subtree_array_copy` | `src/subtree.rs:135` |
| `ts_subtree_array_clear` | `src/subtree.rs:139` |
| `ts_subtree_array_delete` | `src/subtree.rs:143` |
| `ts_subtree_array_remove_trailing_extras` | `src/subtree.rs:150` |
| `ts_subtree_array_reverse` | `src/subtree.rs:154` |
| `ts_subtree_pool_new` | `src/subtree.rs:158` |
| `ts_subtree_pool_delete` | `src/subtree.rs:162` |
| `ts_subtree_pool_allocate` | `src/subtree.rs:166` |
| `ts_subtree_pool_free` | `src/subtree.rs:170` |
| `ts_subtree_can_inline` | `src/subtree.rs:174` |
| `ts_subtree_new_leaf` | `src/subtree.rs:189` |
| `ts_subtree_set_symbol` | `src/subtree.rs:193` |
| `ts_subtree_new_error` | `src/subtree.rs:205` |
| `ts_subtree_clone` | `src/subtree.rs:209` |
| `ts_subtree_make_mut` | `src/subtree.rs:213` |
| `ts_subtree_compress` | `src/subtree.rs:222` |
| `ts_subtree_summarize_children` | `src/subtree.rs:226` |
| `ts_subtree_new_node` | `src/subtree.rs:235` |
| `ts_subtree_new_error_node` | `src/subtree.rs:243` |
| `ts_subtree_new_missing_leaf` | `src/subtree.rs:253` |

### subtree-2 (13 stubs)

| C function / binding item | Rust stub location |
|---|---|
| `ts_subtree_retain` | `src/subtree.rs:257` |
| `ts_subtree_release` | `src/subtree.rs:261` |
| `ts_subtree_compare` | `src/subtree.rs:265` |
| `ts_subtree_set_has_changes` | `src/subtree.rs:269` |
| `ts_subtree_edit` | `src/subtree.rs:273` |
| `ts_subtree_last_external_token` | `src/subtree.rs:277` |
| `ts_subtree__write_char_to_string` | `src/subtree.rs:281` |
| `ts_subtree__write_to_string` | `src/subtree.rs:293` |
| `ts_subtree_string` | `src/subtree.rs:303` |
| `ts_subtree__print_dot_graph` | `src/subtree.rs:313` |
| `ts_subtree_print_dot_graph` | `src/subtree.rs:321` |
| `ts_subtree_external_scanner_state` | `src/subtree.rs:325` |
| `ts_subtree_external_scanner_state_eq` | `src/subtree.rs:329` |

### tree (11 stubs)

| C function / binding item | Rust stub location |
|---|---|
| `ts_tree_new` | `src/tree.rs:16` |
| `ts_tree_copy` | `src/tree.rs:20` |
| `ts_tree_delete` | `src/tree.rs:24` |
| `ts_tree_root_node` | `src/tree.rs:28` |
| `ts_tree_root_node_with_offset` | `src/tree.rs:36` |
| `ts_tree_language` | `src/tree.rs:40` |
| `ts_tree_edit` | `src/tree.rs:44` |
| `ts_tree_included_ranges` | `src/tree.rs:48` |
| `ts_tree_get_changed_ranges` | `src/tree.rs:52` |
| `_ts_dup` | `src/tree.rs:56` |
| `ts_tree_print_dot_graph` | `src/tree.rs:63` |

### tree_cursor (33 stubs)

| C function / binding item | Rust stub location |
|---|---|
| `ts_tree_cursor_is_entry_visible` | `src/tree_cursor.rs:45` |
| `ts_tree_cursor_iterate_children` | `src/tree_cursor.rs:51` |
| `ts_tree_cursor_child_iterator_next` | `src/tree_cursor.rs:57` |
| `length_backtrack` | `src/tree_cursor.rs:61` |
| `ts_tree_cursor_child_iterator_previous` | `src/tree_cursor.rs:67` |
| `ts_tree_cursor_new` | `src/tree_cursor.rs:71` |
| `ts_tree_cursor_reset` | `src/tree_cursor.rs:75` |
| `ts_tree_cursor_init` | `src/tree_cursor.rs:79` |
| `ts_tree_cursor_delete` | `src/tree_cursor.rs:83` |
| `ts_tree_cursor_goto_first_child_internal` | `src/tree_cursor.rs:89` |
| `ts_tree_cursor_goto_first_child` | `src/tree_cursor.rs:93` |
| `ts_tree_cursor_goto_last_child_internal` | `src/tree_cursor.rs:99` |
| `ts_tree_cursor_goto_last_child` | `src/tree_cursor.rs:103` |
| `ts_tree_cursor_goto_first_child_for_byte_and_point` | `src/tree_cursor.rs:111` |
| `ts_tree_cursor_goto_first_child_for_byte` | `src/tree_cursor.rs:118` |
| `ts_tree_cursor_goto_first_child_for_point` | `src/tree_cursor.rs:125` |
| `ts_tree_cursor_goto_sibling_internal` | `src/tree_cursor.rs:132` |
| `ts_tree_cursor_goto_next_sibling_internal` | `src/tree_cursor.rs:138` |
| `ts_tree_cursor_goto_next_sibling` | `src/tree_cursor.rs:142` |
| `ts_tree_cursor_goto_previous_sibling_internal` | `src/tree_cursor.rs:148` |
| `ts_tree_cursor_goto_previous_sibling` | `src/tree_cursor.rs:152` |
| `ts_tree_cursor_goto_parent` | `src/tree_cursor.rs:156` |
| `ts_tree_cursor_goto_descendant` | `src/tree_cursor.rs:163` |
| `ts_tree_cursor_current_descendant_index` | `src/tree_cursor.rs:167` |
| `ts_tree_cursor_current_node` | `src/tree_cursor.rs:171` |
| `ts_tree_cursor_current_status` | `src/tree_cursor.rs:175` |
| `ts_tree_cursor_current_depth` | `src/tree_cursor.rs:179` |
| `ts_tree_cursor_parent_node` | `src/tree_cursor.rs:183` |
| `ts_tree_cursor_current_field_id` | `src/tree_cursor.rs:187` |
| `ts_tree_cursor_current_field_name` | `src/tree_cursor.rs:191` |
| `ts_tree_cursor_copy` | `src/tree_cursor.rs:195` |
| `ts_tree_cursor_reset_to` | `src/tree_cursor.rs:202` |
| `ts_tree_cursor_current_subtree` | `src/tree_cursor.rs:206` |
