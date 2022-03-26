// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

#[allow(dead_code)]
pub mod grammar;

use quote::quote;

use proc_macro2::TokenStream;

pub fn peginator_compile(peg_grammar: &str) -> TokenStream {
    quote!(println!("The grammar was {:?}", #peg_grammar))
}
