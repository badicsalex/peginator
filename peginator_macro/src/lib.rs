// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

use peginator::peginator_compile;
use proc_macro::TokenStream;

#[proc_macro]
pub fn peginate(input: TokenStream) -> TokenStream {
    let param: syn::LitStr =
        syn::parse(input).expect("peginate!() expects a single string as a parameter");
    peginator_compile(&param.value())
        .expect("Error during grammar compilation")
        .into()
}
