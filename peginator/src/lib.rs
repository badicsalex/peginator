// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

mod grammar;

mod codegen;

pub mod buildscript;
pub mod runtime;

pub use codegen::{CodegenGrammar, CodegenSettings};
pub use grammar::Grammar;
pub use runtime::{ParseError, ParseSettings, PegParser, PrettyParseError};

use proc_macro2::TokenStream;

use anyhow::Result;

pub fn peginator_compile(peg_grammar: &str) -> Result<TokenStream> {
    let parsed_grammar = Grammar::parse_advanced(peg_grammar, &ParseSettings::default())?;
    parsed_grammar.generate_code(&CodegenSettings::default())
}
