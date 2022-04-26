// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

mod grammar;

mod codegen;

mod runtime;

pub use codegen::{CodegenGrammar, CodegenSettings};
pub use grammar::parse_Grammar;
pub use runtime::ParseSettings;

use proc_macro2::TokenStream;

use anyhow::Result;

pub fn peginator_compile(peg_grammar: &str) -> Result<TokenStream> {
    let parsed_grammar = parse_Grammar(
        peg_grammar,
        &ParseSettings {
            ..Default::default()
        },
    )?;
    let settings = CodegenSettings {
        skip_whitespace: true,
    };
    parsed_grammar.generate_code(&settings)
}
