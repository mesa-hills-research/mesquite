//! Rust owns representation/layout; these replace host.h's preprocessor probes.
pub(crate) const TS_BIG_ENDIAN: bool = cfg!(target_endian = "big");
pub(crate) const TS_PTR_SIZE: usize = usize::BITS as usize;
