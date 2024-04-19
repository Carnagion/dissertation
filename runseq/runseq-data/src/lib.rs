//! Dataset parsing and conversion helpers for runway sequencing and de-icing problem instances.

#![deny(rust_2018_idioms)]
#![warn(missing_docs)]

#[cfg(feature = "furini")]
pub mod furini;

#[cfg(feature = "heathrow")]
pub mod heathrow;

#[cfg(feature = "xlsx")]
pub mod xlsx;
