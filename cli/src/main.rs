// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

use std::fs;

use anyhow::Result;
use clap::Parser;
use colored::*;
use peginator::{PegParser, PrettyParseError};
use peginator_codegen::Grammar;
use peginator_codegen::{generate_source_header, CodegenGrammar, CodegenSettings};

/// Compile EBNF grammar into rust parser code.
#[derive(Parser, Debug)]
#[clap(version, about)]
struct Args {
    /// Print the parsed AST and exit
    #[clap(short, long)]
    ast_only: bool,

    /// Trace rule matching
    #[clap(short, long)]
    trace: bool,

    /// Use a custom set of derives for the generated types
    #[clap(short, long)]
    derives: Vec<String>,

    grammar_file: String,
}

fn main_wrap() -> Result<()> {
    let args = Args::parse();
    let grammar = fs::read_to_string(&args.grammar_file)?;
    let parsed_grammar = if args.trace {
        Grammar::parse_with_trace(&grammar)
    } else {
        Grammar::parse(&grammar)
    }
    .map_err(|err| PrettyParseError::from_parse_error(&err, &grammar, Some(&args.grammar_file)))?;
    if args.ast_only {
        println!("{:#?}", parsed_grammar);
        return Ok(());
    }

    let derives = if args.derives.is_empty() {
        CodegenSettings::default().derives
    } else {
        args.derives
    };

    let settings = CodegenSettings {
        derives,
        ..Default::default()
    };
    let generated_code = parsed_grammar.generate_code(&settings)?;
    println!("{}", generate_source_header(&grammar));
    println!("{}", generated_code);
    Ok(())
}

fn main() {
    if let Err(e) = main_wrap() {
        println!("{}: {}", "Error".red().bold(), e)
    }
}
