// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

use peginator::grammar::parse_Grammar;
use peginator::grammar::GRAMMAR;
use peginator::runtime::ParseState;

use anyhow::Result;

fn main() -> Result<()> {
    println!("{:#?}", parse_Grammar(ParseState::new(GRAMMAR))?.0);
    Ok(())
}
