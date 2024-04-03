#![deny(rust_2018_idioms)]

#[cfg(feature = "furini")]
pub mod furini;

#[cfg(feature = "heathrow")]
pub mod heathrow;

#[cfg(feature = "xlsx")]
pub mod xlsx;
