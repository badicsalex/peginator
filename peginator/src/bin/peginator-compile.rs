// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

use std::fs;

use anyhow::Result;
use clap::Parser;

use peginator::CodegenGrammar;
use peginator::CodegenSettings;
use peginator::Grammar;
use peginator::ParseSettings;
use peginator::PegParser;

/// Compile EBNF grammar into rust parser code.
#[derive(Parser, Debug)]
#[clap(version, about)]
struct Args {
    /// Module path of the built-in peginator code
    #[clap(short, long, default_value_t = String::from("peginator"))]
    peginator_crate_name: String,

    /// Print the parsed AST and exit
    #[clap(short, long)]
    ast_only: bool,

    /// Trace rule matching
    #[clap(short, long)]
    trace: bool,

    grammar_file: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let grammar = fs::read_to_string(args.grammar_file)?;
    let parsed_grammar = Grammar::parse_advanced(
        &grammar,
        &ParseSettings {
            tracing: args.trace,
        },
    )?;
    if args.ast_only {
        println!("{:#?}", parsed_grammar);
        return Ok(());
    }

    let settings = CodegenSettings {
        skip_whitespace: true,
        peginator_crate_name: args.peginator_crate_name,
    };
    let generated_code = parsed_grammar.generate_code(&settings)?;
    println!("{}", generated_code);
    Ok(())
}
