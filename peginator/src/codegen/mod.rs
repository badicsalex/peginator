// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

use crate::grammar::Grammar;

pub mod parser;
pub mod types;
use anyhow::Result;

pub fn lets_debug(grammar: &Grammar) -> Result<()> {
    let ast_structs = types::extract_ast_structs(grammar)?;
    for ast_struct in &ast_structs {
        println!("{}:", ast_struct.name);
        println!("{:?}", ast_struct.fields);
        println!();
    }
    Ok(())
}
