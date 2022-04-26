// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

mod grammar;

mod codegen;

pub mod runtime;

pub use codegen::{CodegenGrammar, CodegenSettings};
pub use grammar::{parse_Grammar, parse_Grammar_advanced};
pub use runtime::ParseSettings;

use proc_macro2::TokenStream;
use quote::quote;

use anyhow::Result;

pub fn peginator_compile(peg_grammar: &str) -> Result<TokenStream> {
    let parsed_grammar = parse_Grammar_advanced(
        peg_grammar,
        &ParseSettings {
            ..Default::default()
        },
    )?;
    let settings = CodegenSettings {
        skip_whitespace: true,
    };
    let generated_code = parsed_grammar.generate_code(&settings)?;
    Ok(quote!(
        use peginator::runtime::*;
        #generated_code
    ))
}
