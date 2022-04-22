// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

use peginator::codegen::lets_debug;
use peginator::grammar::bootstrap_parsinator_grammar;
use peginator::grammar::test::parse_Grammar;
use peginator::grammar::GRAMMAR;
use peginator::runtime::ParseState;

use anyhow::Result;

fn main() -> Result<()> {
    lets_debug(&bootstrap_parsinator_grammar())?;
    println!();
    println!();
    println!("{:#?}", parse_Grammar(ParseState::new(GRAMMAR))?.0);
    Ok(())
}
