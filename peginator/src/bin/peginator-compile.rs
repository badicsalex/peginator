// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

use std::fs;

use anyhow::Result;
use clap::Parser;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use peginator::codegen::CodegenOuter;
use peginator::codegen::CodegenSettings;
use peginator::grammar::parse_Grammar;
use peginator::runtime::ParseState;

/// Compile EBNF grammar into rust parser code.
#[derive(Parser, Debug)]
#[clap(version, about)]
struct Args {
    /// Module path of the generated parser code
    #[clap(short, long, default_value_t = String::from("crate::grammar::generated"))]
    grammar_module_prefix: String,

    /// Module path of the built-in peginator code
    #[clap(short, long, default_value_t = String::from("peginator::runtime"))]
    runtime_module_prefix: String,

    /// Print the parsed AST and exit
    #[clap(short, long)]
    ast_only: bool,

    grammar_file: String,
}

fn ident_with_scope(s: &str) -> TokenStream {
    let idents = s.split("::").map(|p| format_ident!("{}", p));
    quote!( #(#idents ::)* )
}

fn main() -> Result<()> {
    let args = Args::parse();
    let grammar = fs::read_to_string(args.grammar_file)?;
    let parsed_grammar = parse_Grammar(ParseState::new(&grammar))?.0;
    if args.ast_only {
        println!("{:#?}", parsed_grammar);
        return Ok(());
    }

    let settings = CodegenSettings {
        grammar_module_prefix: ident_with_scope(&args.grammar_module_prefix),
        runtime_prefix: ident_with_scope(&args.runtime_module_prefix),
        skip_whitespace: true,
    };
    let generated_code = parsed_grammar.generate_code(&settings)?;
    println!("use {}::*;", args.runtime_module_prefix);
    println!("{}", generated_code);
    Ok(())
}