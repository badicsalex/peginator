// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

use std::collections::HashMap;

use nohash_hasher::BuildNoHashHasher;

mod builtin_parsers;
mod error;
mod parse_result;
mod peg_parser;
mod state;
mod trace;

pub use builtin_parsers::{
    parse_Whitespace, parse_char, parse_character_literal, parse_character_range,
    parse_string_literal,
};
pub use error::{combine_errors, ParseError, ParseErrorSpecifics, PrettyParseError};
pub use parse_result::{ParseOk, ParseResult};
pub use peg_parser::{ParseSettings, PegParser};
pub use state::ParseState;
pub use trace::{IndentedTracer, NoopTracer, ParseTracer};

pub type CacheEntries<'a, T> = HashMap<usize, ParseResult<'a, T>, BuildNoHashHasher<usize>>;

pub trait PegPosition {
    fn position(&self) -> &std::ops::Range<usize>;
}
