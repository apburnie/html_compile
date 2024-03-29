//! # html_compile
//! A template engine for generating HTML strings within Rust for use with static websites.
//! HTML strings can be generated by inputting a concise syntax into provided macros or, alternatively, from Rust structs.
//! This engine prioritizes speed and should only be used with trusted HTML data.

pub mod compile;
pub mod types;
#[macro_use]
mod macros;
