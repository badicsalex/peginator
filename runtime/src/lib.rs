// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

//! This crate contains the code used by code generated by the [peginator]
//! crate. It contains some documented traits and types meant to be used by third-party
//! code, and a lot of public but undocumented functions and types that are only for
//! usage by generated code, and are subject to change even between minor versions.
//!
//! This crate should be the only "proper" dependency of your peginator-using project:
//!
//! ```toml
//! [dependencies]
//! peginator_runtime = "0.4"
//! ```
//!
//! The rest of the peginator crates are meant to be dev or build dependencies.
//!
//! Please see the documentation of [peginator] for more information.
//!
//! [peginator]: https://docs.rs/peginator

use std::collections::HashMap;

use nohash_hasher::BuildNoHashHasher;

mod builtin_parsers;
mod choice_helper;
mod error;
mod global;
mod parse_result;
mod peg_parser;
mod state;
mod trace;

#[doc(hidden)]
pub use builtin_parsers::{
    parse_Whitespace, parse_char, parse_character_literal, parse_character_literal_insensitive,
    parse_character_range, parse_end_of_input, parse_string_literal,
    parse_string_literal_insensitive,
};
#[doc(hidden)]
pub use choice_helper::ChoiceHelper;
pub use error::{ParseError, ParseErrorSpecifics, PrettyParseError};
#[doc(hidden)]
pub use global::ParseGlobal;
#[doc(hidden)]
pub use parse_result::{ParseOk, ParseResult, ParseResultExtras};
pub use peg_parser::{ParseSettings, PegParser, PegParserAdvanced};
#[doc(hidden)]
pub use state::ParseState;
#[doc(hidden)]
pub use trace::{IndentedTracer, NoopTracer, ParseTracer};

#[doc(hidden)]
pub type CacheEntries<'a, T> = HashMap<usize, ParseResult<'a, T>, BuildNoHashHasher<usize>>;

/// Helper trait to get the parse position of the parsed rule
///
/// Useful for creating general error reporting functions or
/// similar functionality where the position of multiple generated
/// types are used.
pub trait PegPosition {
    /// The parsed position of the rule in bytes (not characters)
    fn position(&self) -> &std::ops::Range<usize>;
}
