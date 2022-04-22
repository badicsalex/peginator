// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

use peginator::codegen::lets_debug;
use peginator::grammar::parse_Grammar;
use peginator::grammar::GRAMMAR;

use anyhow::Result;
use peginator::runtime::ParseState;

fn main() -> Result<()> {
    let grammar = parse_Grammar(ParseState::new(GRAMMAR))?.0;
    lets_debug(&grammar)?;
    Ok(())
}
