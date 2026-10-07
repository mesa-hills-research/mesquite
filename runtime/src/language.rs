use crate::types::*;
use ts_port_tables::{LanguageMetadata, LanguageTables, LexMode, ParseActionEntry, SymbolMetadata};
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
    todo!("language: ts_language_copy")
}

pub(crate) fn ts_language_delete(language: Language) {
    todo!("language: ts_language_delete")
}

pub(crate) fn ts_language_symbol_count(language: &Language) -> u32 {
    todo!("language: ts_language_symbol_count")
}

pub(crate) fn ts_language_state_count(language: &Language) -> u32 {
    todo!("language: ts_language_state_count")
}

pub(crate) fn ts_language_supertypes(language: &Language) -> &'static [Symbol] {
    todo!("language: ts_language_supertypes")
}

pub(crate) fn ts_language_subtypes(language: &Language, supertype: Symbol) -> &'static [Symbol] {
    todo!("language: ts_language_subtypes")
}

pub(crate) fn ts_language_version(language: &Language) -> u32 {
    todo!("language: ts_language_version")
}

pub(crate) fn ts_language_abi_version(language: &Language) -> u32 {
    todo!("language: ts_language_abi_version")
}

pub(crate) fn ts_language_metadata(language: &Language) -> Option<LanguageMetadata> {
    todo!("language: ts_language_metadata")
}

pub(crate) fn ts_language_name(language: &Language) -> Option<&'static str> {
    todo!("language: ts_language_name")
}

pub(crate) fn ts_language_field_count(language: &Language) -> u32 {
    todo!("language: ts_language_field_count")
}

pub(crate) fn ts_language_table_entry(
    language: &Language,
    state: StateId,
    symbol: Symbol,
) -> TableEntry {
    todo!("language: ts_language_table_entry")
}

pub(crate) fn ts_language_lex_mode_for_state(language: &Language, state: StateId) -> LexMode {
    todo!("language: ts_language_lex_mode_for_state")
}

pub(crate) fn ts_language_is_reserved_word(
    language: &Language,
    state: StateId,
    symbol: Symbol,
) -> bool {
    todo!("language: ts_language_is_reserved_word")
}

pub(crate) fn ts_language_symbol_metadata(language: &Language, symbol: Symbol) -> SymbolMetadata {
    todo!("language: ts_language_symbol_metadata")
}

pub(crate) fn ts_language_public_symbol(language: &Language, symbol: Symbol) -> Symbol {
    todo!("language: ts_language_public_symbol")
}

pub(crate) fn ts_language_next_state(
    language: &Language,
    state: StateId,
    symbol: Symbol,
) -> StateId {
    todo!("language: ts_language_next_state")
}

pub(crate) fn ts_language_symbol_name(language: &Language, symbol: Symbol) -> Option<&'static str> {
    todo!("language: ts_language_symbol_name")
}

pub(crate) fn ts_language_symbol_for_name(language: &Language, name: &[u8], named: bool) -> Symbol {
    todo!("language: ts_language_symbol_for_name")
}

pub(crate) fn ts_language_symbol_type(language: &Language, symbol: Symbol) -> SymbolType {
    todo!("language: ts_language_symbol_type")
}

pub(crate) fn ts_language_field_name_for_id(
    language: &Language,
    field_id: FieldId,
) -> Option<&'static str> {
    todo!("language: ts_language_field_name_for_id")
}

pub(crate) fn ts_language_field_id_for_name(language: &Language, name: &[u8]) -> FieldId {
    todo!("language: ts_language_field_id_for_name")
}

pub(crate) fn ts_lookahead_iterator_new(
    language: &Language,
    state: StateId,
) -> Option<LookaheadIterator> {
    todo!("language: ts_lookahead_iterator_new")
}

pub(crate) fn ts_lookahead_iterator_delete(iterator: LookaheadIterator) {
    todo!("language: ts_lookahead_iterator_delete")
}

pub(crate) fn ts_lookahead_iterator_reset_state(
    iterator: &mut LookaheadIterator,
    state: StateId,
) -> bool {
    todo!("language: ts_lookahead_iterator_reset_state")
}

pub(crate) fn ts_lookahead_iterator_language(iterator: &LookaheadIterator) -> &Language {
    todo!("language: ts_lookahead_iterator_language")
}

pub(crate) fn ts_lookahead_iterator_reset(
    iterator: &mut LookaheadIterator,
    language: &Language,
    state: StateId,
) -> bool {
    todo!("language: ts_lookahead_iterator_reset")
}

pub(crate) fn ts_lookahead_iterator_next(iterator: &mut LookaheadIterator) -> bool {
    todo!("language: ts_lookahead_iterator_next")
}

pub(crate) fn ts_lookahead_iterator_current_symbol(iterator: &LookaheadIterator) -> Symbol {
    todo!("language: ts_lookahead_iterator_current_symbol")
}

pub(crate) fn ts_lookahead_iterator_current_symbol_name(
    iterator: &LookaheadIterator,
) -> Option<&'static str> {
    todo!("language: ts_lookahead_iterator_current_symbol_name")
}

pub(crate) fn ts_language_actions(
    language: &Language,
    state: StateId,
    symbol: Symbol,
) -> &'static [ParseActionEntry] {
    todo!("language: ts_language_actions")
}

pub(crate) fn ts_language_has_reduce_action(
    language: &Language,
    state: StateId,
    symbol: Symbol,
) -> bool {
    todo!("language: ts_language_has_reduce_action")
}

pub(crate) fn ts_language_lookup(language: &Language, state: StateId, symbol: Symbol) -> u16 {
    todo!("language: ts_language_lookup")
}

pub(crate) fn ts_language_has_actions(language: &Language, state: StateId, symbol: Symbol) -> bool {
    todo!("language: ts_language_has_actions")
}

pub(crate) fn ts_language_lookaheads(language: &Language, state: StateId) -> LookaheadIterator {
    todo!("language: ts_language_lookaheads")
}

pub(crate) fn ts_lookahead_iterator__next(iterator: &mut LookaheadIterator) -> bool {
    todo!("language: ts_lookahead_iterator__next")
}

pub(crate) fn ts_language_alias_sequence(
    language: &Language,
    production_id: u32,
) -> &'static [Symbol] {
    todo!("language: ts_language_alias_sequence")
}

pub(crate) fn ts_language_alias_at(
    language: &Language,
    production_id: u32,
    child_index: u32,
) -> Symbol {
    todo!("language: ts_language_alias_at")
}

pub(crate) fn ts_language_field_map(
    language: &Language,
    production_id: u32,
) -> &'static [ts_port_tables::FieldMapEntry] {
    todo!("language: ts_language_field_map")
}

pub(crate) fn ts_language_aliases_for_symbol(
    language: &Language,
    symbol: Symbol,
) -> &'static [Symbol] {
    todo!("language: ts_language_aliases_for_symbol")
}

pub(crate) fn ts_language_state_is_primary(language: &Language, state: StateId) -> bool {
    todo!("language: ts_language_state_is_primary")
}

pub(crate) fn ts_language_enabled_external_tokens(
    language: &Language,
    external_scanner_state: u32,
) -> Option<&'static [bool]> {
    todo!("language: ts_language_enabled_external_tokens")
}
pub(crate) fn ts_language_write_symbol_as_dot_string(
    language: &Language,
    output: &mut dyn std::io::Write,
    symbol: Symbol,
) -> std::io::Result<()> {
    todo!("language: ts_language_write_symbol_as_dot_string")
}
