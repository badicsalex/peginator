// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

use crate::grammar::Grammar;

pub mod parser;
pub mod types;
use anyhow::Result;
use proc_macro2::{Ident, Span};

fn quick_ident(s: &str) -> Ident {
    Ident::new(s, Span::call_site())
}

pub fn lets_debug(grammar: &Grammar) -> Result<()> {
    let ast_structs = types::extract_ast_structs(grammar)?;
    let types_code = types::generate_types(&ast_structs)?;
    let parser_code = parser::generate_parsers(grammar, &ast_structs)?;
    println!("use crate::runtime::*;");
    println!("{}", types_code);
    println!("{}", parser_code);
    Ok(())
}
