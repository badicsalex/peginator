// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

use std::str::FromStr;

use peginator_codegen::{CodegenGrammar, CodegenSettings, Grammar};
use proc_macro::TokenStream;

/// Compile peginator grammar in-place
///
/// The parameter is the grammar as a string, and will generate the parser in-place.
///
/// The `PegParser` trait needs to be `use`-d separately.
#[proc_macro]
pub fn peginate(input: TokenStream) -> TokenStream {
    let param: syn::LitStr =
        syn::parse(input).expect("peginate!() expects a single string as a parameter");
    let parsed_grammar = Grammar::from_str(&param.value()).unwrap();
    parsed_grammar
        .generate_code(&CodegenSettings::default())
        .unwrap()
        .into()
}
