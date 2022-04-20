// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

use peginator::{codegen::lets_debug, grammar::bootstrap_parsinator_grammar};

use anyhow::Result;

fn main() -> Result<()> {
    println!("Hello, world!");
    lets_debug(&bootstrap_parsinator_grammar())?;
    Ok(())
}
