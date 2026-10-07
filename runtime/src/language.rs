use crate::ts_assert::ts_assert;
use crate::types::*;
use ts_port_tables::{
    BUILTIN_SYM_ERROR, LanguageMetadata, LanguageTables, LexMode, ParseAction, ParseActionEntry,
    SymbolMetadata,
};

const LANGUAGE_VERSION_WITH_RESERVED_WORDS: u32 = 15;
const LANGUAGE_VERSION_WITH_PRIMARY_STATES: u32 = 14;

/// A grammar is immutable, process-long, and provided by a generated Rust crate.
#[derive(Clone, Copy)]
pub struct Language {
    pub(crate) tables: &'static LanguageTables,
}
impl From<&'static LanguageTables> for Language {
    fn from(tables: &'static LanguageTables) -> Self {
        Self { tables }
    }
}
impl std::fmt::Debug for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Language").field(&self.tables.name).finish()
    }
}
impl PartialEq for Language {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.tables, other.tables)
    }
}
impl Eq for Language {}
impl std::hash::Hash for Language {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::ptr::hash(self.tables, state);
    }
}
/// Entries borrow the host's header-free action run (not an allocated copy).
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct TableEntry {
    pub actions: &'static [ParseActionEntry],
    pub is_reusable: bool,
}
const LOOKUP_CACHE_SIZE: usize = 4096;

/// Parser-local memoization of immutable grammar lookups. The full state/symbol
/// key is checked on every hit, so direct-map collisions only cause a new scan.
/// Reset this cache whenever the parser's language changes.
pub(crate) struct LookupCache {
    entries: Box<[u64; LOOKUP_CACHE_SIZE]>,
}

impl Default for LookupCache {
    fn default() -> Self {
        Self {
            entries: Box::new([u64::MAX; LOOKUP_CACHE_SIZE]),
        }
    }
}

impl LookupCache {
    pub fn clear(&mut self) {
        self.entries.fill(u64::MAX);
    }

    fn lookup(&mut self, language: &Language, state: StateId, symbol: Symbol) -> u16 {
        // Dense rows are already a single indexed load; cache only compressed rows.
        if u32::from(state) < language.tables.large_state_count {
            return ts_language_lookup(language, state, symbol);
        }
        let key = (u32::from(state) << 16) | u32::from(symbol);
        let index = (usize::from(state) * 31 + usize::from(symbol) * 17) & (self.entries.len() - 1);
        let entry = &mut self.entries[index];
        // Bits 0..16 hold the value and 16..48 the key. The unused high bits
        // distinguish the empty sentinel from every possible state/symbol pair.
        if *entry >> 16 == u64::from(key) {
            return *entry as u16;
        }
        let value = ts_language_lookup(language, state, symbol);
        *entry = (u64::from(key) << 16) | u64::from(value);
        value
    }

    pub fn table_entry(
        &mut self,
        language: &Language,
        state: StateId,
        symbol: Symbol,
    ) -> TableEntry {
        if symbol == BUILTIN_SYM_ERROR || symbol == BUILTIN_SYM_ERROR_REPEAT {
            return TableEntry::default();
        }
        ts_assert!(u32::from(symbol) < language.tables.token_count);
        let index = self.lookup(language, state, symbol);
        let (is_reusable, actions) = language.tables.action_list(index as usize);
        TableEntry {
            actions,
            is_reusable,
        }
    }

    pub fn next_state(&mut self, language: &Language, state: StateId, symbol: Symbol) -> StateId {
        if symbol == BUILTIN_SYM_ERROR || symbol == BUILTIN_SYM_ERROR_REPEAT {
            0
        } else if u32::from(symbol) < language.tables.token_count {
            match self.actions(language, state, symbol).last() {
                Some(ParseActionEntry::Action(ParseAction::Shift {
                    state: next_state,
                    extra,
                    ..
                })) => {
                    if *extra {
                        state
                    } else {
                        *next_state
                    }
                }
                _ => 0,
            }
        } else {
            self.lookup(language, state, symbol)
        }
    }

    pub fn actions(
        &mut self,
        language: &Language,
        state: StateId,
        symbol: Symbol,
    ) -> &'static [ParseActionEntry] {
        self.table_entry(language, state, symbol).actions
    }

    pub fn has_actions(&mut self, language: &Language, state: StateId, symbol: Symbol) -> bool {
        self.lookup(language, state, symbol) != 0
    }

    pub fn has_reduce_action(
        &mut self,
        language: &Language,
        state: StateId,
        symbol: Symbol,
    ) -> bool {
        matches!(
            self.actions(language, state, symbol).first(),
            Some(ParseActionEntry::Action(ParseAction::Reduce { .. }))
        )
    }
}

/// Internal lookaheads are needed by error recovery. No public iterator API.
#[derive(Clone, Debug)]
pub(crate) struct LookaheadIterator {
    pub language: Language,
    pub data_index: i64,
    pub group_end: usize,
    pub state: StateId,
    pub table_value: u16,
    pub section_index: u16,
    pub group_count: u16,
    pub is_small_state: bool,
    pub actions: &'static [ParseActionEntry],
    pub symbol: Symbol,
    pub next_state: StateId,
}

pub(crate) fn ts_language_copy(language: &Language) -> Language {
    // Native grammars have static storage; wasm languages are outside this port.
    *language
}

pub(crate) fn ts_language_delete(_language: Language) {
    // Static native grammar tables are not reference-counted or freed.
}

pub(crate) fn ts_language_symbol_count(language: &Language) -> u32 {
    language.tables.symbol_count + language.tables.alias_count
}

pub(crate) fn ts_language_state_count(language: &Language) -> u32 {
    language.tables.state_count
}

pub(crate) fn ts_language_supertypes(language: &Language) -> &'static [Symbol] {
    if language.tables.abi_version >= LANGUAGE_VERSION_WITH_RESERVED_WORDS {
        &language.tables.supertype_symbols[..language.tables.supertype_count as usize]
    } else {
        &[]
    }
}

pub(crate) fn ts_language_subtypes(language: &Language, supertype: Symbol) -> &'static [Symbol] {
    if language.tables.abi_version < LANGUAGE_VERSION_WITH_RESERVED_WORDS
        || !ts_language_symbol_metadata(language, supertype).supertype
    {
        return &[];
    }
    let slice = language.tables.supertype_map_slices[supertype as usize];
    let start = slice.index as usize;
    &language.tables.supertype_map_entries[start..start + slice.length as usize]
}

pub(crate) fn ts_language_version(language: &Language) -> u32 {
    language.tables.abi_version
}

pub(crate) fn ts_language_abi_version(language: &Language) -> u32 {
    language.tables.abi_version
}

pub(crate) fn ts_language_metadata(language: &Language) -> Option<LanguageMetadata> {
    if language.tables.abi_version >= LANGUAGE_VERSION_WITH_RESERVED_WORDS {
        language.tables.metadata
    } else {
        None
    }
}

pub(crate) fn ts_language_name(language: &Language) -> Option<&'static str> {
    if language.tables.abi_version >= LANGUAGE_VERSION_WITH_RESERVED_WORDS {
        language.tables.name.as_deref()
    } else {
        None
    }
}

pub(crate) fn ts_language_field_count(language: &Language) -> u32 {
    language.tables.field_count
}

pub(crate) fn ts_language_table_entry(
    language: &Language,
    state: StateId,
    symbol: Symbol,
) -> TableEntry {
    if symbol == BUILTIN_SYM_ERROR || symbol == BUILTIN_SYM_ERROR_REPEAT {
        TableEntry::default()
    } else {
        ts_assert!(u32::from(symbol) < language.tables.token_count);
        let action_index = ts_language_lookup(language, state, symbol);
        let (is_reusable, actions) = language.tables.action_list(action_index as usize);
        TableEntry {
            actions,
            is_reusable,
        }
    }
}

pub(crate) fn ts_language_lex_mode_for_state(language: &Language, state: StateId) -> LexMode {
    let mut mode = language.tables.lex_modes[state as usize];
    if language.tables.abi_version < LANGUAGE_VERSION_WITH_RESERVED_WORDS {
        mode.reserved_word_set_id = 0;
    }
    mode
}

pub(crate) fn ts_language_is_reserved_word(
    language: &Language,
    state: StateId,
    symbol: Symbol,
) -> bool {
    let mode = ts_language_lex_mode_for_state(language, state);
    if mode.reserved_word_set_id > 0 {
        let size = language.tables.max_reserved_word_set_size as usize;
        let start = mode.reserved_word_set_id as usize * size;
        for &word in &language.tables.reserved_words[start..start + size] {
            // The equality check precedes the terminator check, even for symbol 0.
            if word == symbol {
                return true;
            }
            if word == 0 {
                break;
            }
        }
    }
    false
}

pub(crate) fn ts_language_symbol_metadata(language: &Language, symbol: Symbol) -> SymbolMetadata {
    match symbol {
        BUILTIN_SYM_ERROR => SymbolMetadata {
            visible: true,
            named: true,
            supertype: false,
        },
        BUILTIN_SYM_ERROR_REPEAT => SymbolMetadata::default(),
        _ => language.tables.symbol_metadata[symbol as usize],
    }
}

pub(crate) fn ts_language_public_symbol(language: &Language, symbol: Symbol) -> Symbol {
    if symbol == BUILTIN_SYM_ERROR {
        symbol
    } else {
        language.tables.public_symbol_map[symbol as usize]
    }
}

pub(crate) fn ts_language_next_state(
    language: &Language,
    state: StateId,
    symbol: Symbol,
) -> StateId {
    if symbol == BUILTIN_SYM_ERROR || symbol == BUILTIN_SYM_ERROR_REPEAT {
        0
    } else if u32::from(symbol) < language.tables.token_count {
        match ts_language_actions(language, state, symbol).last() {
            Some(ParseActionEntry::Action(ParseAction::Shift {
                state: next_state,
                extra,
                ..
            })) => {
                if *extra {
                    state
                } else {
                    *next_state
                }
            }
            _ => 0,
        }
    } else {
        ts_language_lookup(language, state, symbol)
    }
}

pub(crate) fn ts_language_symbol_name(language: &Language, symbol: Symbol) -> Option<&'static str> {
    match symbol {
        BUILTIN_SYM_ERROR => Some("ERROR"),
        BUILTIN_SYM_ERROR_REPEAT => Some("_ERROR"),
        _ if u32::from(symbol) < ts_language_symbol_count(language) => {
            Some(language.tables.symbol_names[symbol as usize].as_str())
        }
        _ => None,
    }
}

/// `strncmp(input, c_name, input.len())`, including the C string terminator.
/// In particular, the special ERROR lookup deliberately accepts a prefix.
fn compare_name_prefix(input: &[u8], c_name: &str) -> i32 {
    for (i, &byte) in input.iter().enumerate() {
        let other = c_name.as_bytes().get(i).copied().unwrap_or(0);
        let difference = i32::from(byte) - i32::from(other);
        if difference != 0 || byte == 0 {
            return difference;
        }
    }
    0
}

pub(crate) fn ts_language_symbol_for_name(language: &Language, name: &[u8], named: bool) -> Symbol {
    if named && compare_name_prefix(name, "ERROR") == 0 {
        return BUILTIN_SYM_ERROR;
    }
    let count = ts_language_symbol_count(language) as u16;
    for symbol in 0..count {
        let metadata = ts_language_symbol_metadata(language, symbol);
        if (!metadata.visible && !metadata.supertype) || metadata.named != named {
            continue;
        }
        let symbol_name = &language.tables.symbol_names[symbol as usize];
        if compare_name_prefix(name, symbol_name) == 0 && symbol_name.len() == name.len() {
            return language.tables.public_symbol_map[symbol as usize];
        }
    }
    0
}

pub(crate) fn ts_language_symbol_type(language: &Language, symbol: Symbol) -> SymbolType {
    let metadata = ts_language_symbol_metadata(language, symbol);
    if metadata.named && metadata.visible {
        SymbolType::Regular
    } else if metadata.visible {
        SymbolType::Anonymous
    } else if metadata.supertype {
        SymbolType::Supertype
    } else {
        SymbolType::Auxiliary
    }
}

pub(crate) fn ts_language_field_name_for_id(
    language: &Language,
    field_id: FieldId,
) -> Option<&'static str> {
    let count = ts_language_field_count(language);
    if count != 0 && u32::from(field_id) <= count {
        language.tables.field_names[field_id as usize].as_deref()
    } else {
        None
    }
}

pub(crate) fn ts_language_field_id_for_name(language: &Language, name: &[u8]) -> FieldId {
    let count = ts_language_field_count(language) as u16;
    for id in 1..=count {
        let field_name = language.tables.field_names[id as usize]
            .as_deref()
            .expect("field names after index zero are non-null");
        match compare_name_prefix(name, field_name) {
            0 if field_name.len() == name.len() => return id,
            // C switches on exactly -1, not on every negative comparison.
            -1 => return 0,
            _ => {}
        }
    }
    0
}

pub(crate) fn ts_lookahead_iterator_new(
    language: &Language,
    state: StateId,
) -> Option<LookaheadIterator> {
    if u32::from(state) >= language.tables.state_count {
        None
    } else {
        Some(ts_language_lookaheads(language, state))
    }
}

pub(crate) fn ts_lookahead_iterator_delete(_iterator: LookaheadIterator) {
    // The iterator's owned value replaces the C allocation.
}

pub(crate) fn ts_lookahead_iterator_reset_state(
    iterator: &mut LookaheadIterator,
    state: StateId,
) -> bool {
    if u32::from(state) >= iterator.language.tables.state_count {
        return false;
    }
    *iterator = ts_language_lookaheads(&iterator.language, state);
    true
}

pub(crate) fn ts_lookahead_iterator_language(iterator: &LookaheadIterator) -> &Language {
    &iterator.language
}

pub(crate) fn ts_lookahead_iterator_reset(
    iterator: &mut LookaheadIterator,
    language: &Language,
    state: StateId,
) -> bool {
    if u32::from(state) >= language.tables.state_count {
        return false;
    }
    *iterator = ts_language_lookaheads(language, state);
    true
}

pub(crate) fn ts_lookahead_iterator_next(iterator: &mut LookaheadIterator) -> bool {
    ts_lookahead_iterator__next(iterator)
}

pub(crate) fn ts_lookahead_iterator_current_symbol(iterator: &LookaheadIterator) -> Symbol {
    iterator.symbol
}

pub(crate) fn ts_lookahead_iterator_current_symbol_name(
    iterator: &LookaheadIterator,
) -> Option<&'static str> {
    ts_language_symbol_name(&iterator.language, iterator.symbol)
}

pub(crate) fn ts_language_actions(
    language: &Language,
    state: StateId,
    symbol: Symbol,
) -> &'static [ParseActionEntry] {
    ts_language_table_entry(language, state, symbol).actions
}

pub(crate) fn ts_language_has_reduce_action(
    language: &Language,
    state: StateId,
    symbol: Symbol,
) -> bool {
    matches!(
        ts_language_table_entry(language, state, symbol)
            .actions
            .first(),
        Some(ParseActionEntry::Action(ParseAction::Reduce { .. }))
    )
}

pub(crate) fn ts_language_lookup(language: &Language, state: StateId, symbol: Symbol) -> u16 {
    let tables = language.tables;
    if u32::from(state) >= tables.large_state_count {
        let mut index = tables.small_parse_table_map
            [(u32::from(state) - tables.large_state_count) as usize]
            as usize;
        let data = &tables.small_parse_table;
        let group_count = data[index];
        index += 1;
        for _ in 0..group_count {
            let value = data[index];
            let symbol_count = data[index + 1] as usize;
            index += 2;
            for _ in 0..symbol_count {
                let next_symbol = data[index];
                index += 1;
                if next_symbol == symbol {
                    return value;
                }
            }
        }
        0
    } else {
        tables.parse_table[state as usize * tables.symbol_count as usize + symbol as usize]
    }
}

pub(crate) fn ts_language_has_actions(language: &Language, state: StateId, symbol: Symbol) -> bool {
    ts_language_lookup(language, state, symbol) != 0
}

pub(crate) fn ts_language_lookaheads(language: &Language, state: StateId) -> LookaheadIterator {
    let tables = language.tables;
    let is_small_state = u32::from(state) >= tables.large_state_count;
    let (data_index, group_end, group_count) = if is_small_state {
        let index = tables.small_parse_table_map
            [(u32::from(state) - tables.large_state_count) as usize] as usize;
        (index as i64, index + 1, tables.small_parse_table[index])
    } else {
        // An integer cursor represents C's pointer one before the row, including
        // the first row. No out-of-bounds Rust reference is constructed.
        (i64::from(state) * i64::from(tables.symbol_count) - 1, 0, 0)
    };
    LookaheadIterator {
        language: *language,
        data_index,
        group_end,
        state: 0,
        table_value: 0,
        section_index: 0,
        group_count,
        is_small_state,
        actions: &[],
        symbol: Symbol::MAX,
        next_state: 0,
    }
}

pub(crate) fn ts_lookahead_iterator__next(iterator: &mut LookaheadIterator) -> bool {
    let tables = iterator.language.tables;
    if iterator.is_small_state {
        iterator.data_index += 1;
        if iterator.data_index as usize == iterator.group_end {
            if iterator.group_count == 0 {
                return false;
            }
            iterator.group_count -= 1;
            iterator.table_value = tables.small_parse_table[iterator.data_index as usize];
            iterator.data_index += 1;
            let symbol_count = tables.small_parse_table[iterator.data_index as usize] as usize;
            iterator.data_index += 1;
            iterator.group_end = iterator.data_index as usize + symbol_count;
            iterator.symbol = tables.small_parse_table[iterator.data_index as usize];
        } else {
            iterator.symbol = tables.small_parse_table[iterator.data_index as usize];
            return true;
        }
    } else {
        loop {
            iterator.data_index += 1;
            iterator.symbol = iterator.symbol.wrapping_add(1);
            if u32::from(iterator.symbol) >= tables.symbol_count {
                return false;
            }
            iterator.table_value = tables.parse_table[iterator.data_index as usize];
            if iterator.table_value != 0 {
                break;
            }
        }
    }

    if u32::from(iterator.symbol) < tables.token_count {
        iterator.actions = tables.action_list(iterator.table_value as usize).1;
        iterator.next_state = 0;
    } else {
        // C leaves the action pointer unchanged but sets its count to zero.
        iterator.actions = &[];
        iterator.next_state = iterator.table_value;
    }
    true
}

pub(crate) fn ts_language_alias_sequence(
    language: &Language,
    production_id: u32,
) -> &'static [Symbol] {
    if production_id == 0 {
        &[]
    } else {
        let length = language.tables.max_alias_sequence_length as usize;
        let start = production_id as usize * length;
        &language.tables.alias_sequences[start..start + length]
    }
}

pub(crate) fn ts_language_alias_at(
    language: &Language,
    production_id: u32,
    child_index: u32,
) -> Symbol {
    if production_id == 0 {
        0
    } else {
        language.tables.alias_sequences[production_id as usize
            * language.tables.max_alias_sequence_length as usize
            + child_index as usize]
    }
}

pub(crate) fn ts_language_field_map(
    language: &Language,
    production_id: u32,
) -> &'static [ts_port_tables::FieldMapEntry] {
    if language.tables.field_count == 0 {
        return &[];
    }
    let slice = language.tables.field_map_slices[production_id as usize];
    let start = slice.index as usize;
    &language.tables.field_map_entries[start..start + slice.length as usize]
}

pub(crate) fn ts_language_aliases_for_symbol(
    language: &Language,
    symbol: Symbol,
) -> &'static [Symbol] {
    let map = &language.tables.alias_map;
    let mut index = 0;
    loop {
        let next_symbol = map[index];
        index += 1;
        if next_symbol == 0 || next_symbol > symbol {
            break;
        }
        let count = map[index] as usize;
        index += 1;
        if next_symbol == symbol {
            return &map[index..index + count];
        }
        index += count;
    }
    &language.tables.public_symbol_map[symbol as usize..symbol as usize + 1]
}

pub(crate) fn ts_language_state_is_primary(language: &Language, state: StateId) -> bool {
    if language.tables.abi_version >= LANGUAGE_VERSION_WITH_PRIMARY_STATES {
        state == language.tables.primary_state_ids[state as usize]
    } else {
        true
    }
}

pub(crate) fn ts_language_enabled_external_tokens(
    language: &Language,
    external_scanner_state: u32,
) -> Option<&'static [bool]> {
    if external_scanner_state == 0 {
        None
    } else {
        let scanner = language
            .tables
            .external_scanner
            .as_ref()
            .expect("nonzero external scanner state requires a scanner");
        let count = language.tables.external_token_count as usize;
        let start = external_scanner_state as usize * count;
        Some(&scanner.states[start..start + count])
    }
}

pub(crate) fn ts_language_write_symbol_as_dot_string(
    language: &Language,
    output: &mut dyn std::io::Write,
    symbol: Symbol,
) -> std::io::Result<()> {
    let name = ts_language_symbol_name(language, symbol).expect("DOT symbol must have a name");
    for byte in name.bytes().take_while(|&byte| byte != 0) {
        match byte {
            b'"' | b'\\' => output.write_all(&[b'\\', byte])?,
            b'\n' => output.write_all(b"\\n")?,
            b'\t' => output.write_all(b"\\t")?,
            _ => output.write_all(&[byte])?,
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests;
