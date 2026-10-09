#![forbid(unsafe_code)]
//! `pattern_core` — pure Rust library for canonical millimetre geometry,
//! unit conversion and length formatting.
//!
//! This crate compiles to both native and `wasm32-unknown-unknown`.  It has no
//! filesystem, network, clock or randomness dependencies.

pub mod geom;
pub mod units;
