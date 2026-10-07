//! Library crate for wordle-helper.
//!
//! Exposes the pure word-filtering/ranking logic so it can be shared by the
//! binary (`main.rs`) and by benchmarks/tests.

pub mod filter;
