// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

use peginator::{generator::lets_debug, grammar::bootstrap_parsinator_grammar};

fn main() {
    println!("Hello, world!");
    lets_debug(&bootstrap_parsinator_grammar());
}
