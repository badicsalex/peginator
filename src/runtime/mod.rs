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
    parse_Whitespace, parse_char, parse_character_literal, parse_character_literal_insensitive,
    parse_character_range, parse_string_literal, parse_string_literal_insensitive,
};
pub use error::{combine_errors, ParseError, ParseErrorSpecifics, PrettyParseError};
#[cfg(feature = "lsp-document")]
pub use lsp_document::{IndexedText, Pos, TextAdapter, TextMap};
#[cfg(feature = "lsp-types")]
pub use lsp_types::{Position, Range};
pub use parse_result::{ParseOk, ParseResult};
pub use peg_parser::{ParseSettings, PegParser};
pub use state::ParseState;
pub use trace::{IndentedTracer, NoopTracer, ParseTracer};

pub type CacheEntries<'a, T> = HashMap<usize, ParseResult<'a, T>, BuildNoHashHasher<usize>>;

/// Helper trait to get the parse position of the parsed rule
///
/// Useful for creating general error reporting functions or
/// similar functionality where the position of multiple generated
/// types are used.
pub trait PegPosition {
    /// The parsed position of the rule in bytes (not characters)
    fn position(&self) -> &std::ops::Range<usize>;
    /// Get start offset of the token
    fn offset_start(&self) -> usize {
        self.position().start
    }
    /// Get end offset of the token
    fn offset_end(&self) -> usize {
        self.position().start
    }
    /// Get range in lsp-form
    #[cfg(feature = "lsp")]
    fn lsp_range<T>(&self, text: &IndexedText<String>) -> Option<Range> {
        let start = self.lsp_start(&text)?;
        let end = self.lsp_end(&text)?;
        Some(Range { start, end })
    }
    /// Get start position in lsp-form
    #[cfg(feature = "lsp")]
    fn lsp_start<T>(&self, text: &IndexedText<String>) -> Option<Position> {
        let pos = text.offset_to_pos(self.offset_start())?;
        text.pos_to_lsp_pos(&pos)
    }
    /// Get end position in lsp-form
    #[cfg(feature = "lsp")]
    fn lsp_end<T>(&self, text: &IndexedText<String>) -> Option<Position> {
        let pos = text.offset_to_pos(self.offset_end())?;
        text.pos_to_lsp_pos(&pos)
    }
}
