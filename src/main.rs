// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

//! # Table of contents
//! * [Description](#description)
//! * [Quickstart](#quickstart)
//! * [Integration](#integration)
//! * [Syntax](#peginator-syntax)
//!   * [Rules](#rules)
//!   * [Expressions](#expressions)
//!   * [Fields](#fields)
//!   * [Directives](#directives)
//! * [Whitespace skipping](#whitespace-skipping)
//!
//!
//!
//!
//! # Description
//!
//! PEG parser generator for creating ASTs in Rust
//!
//! Peginator is a PEG (Parsing Expression Grammar) parser generator written in Rust.
//! It is specifically made to parse into ASTs (Abstract Syntax Trees), as opposed to
//! most, streaming-style parsers out there.
//!
//! It generates both the tree structure and the parsing code that can create that tree from any
//! `&str`. The generated parsing code is deliberately very simple straightforward Rust
//! code, which is usually optimized very well by the compiler.
//!
//! There is an opt-in memoization feature that makes it a proper packrat parser that can parse
//! any input in linear time and space.
//!
//! #### About PEGs
//!
//! This documentation describes how peginator implements PEGs. A basic understanding of PEGs
//! are assumed. There are good introductions on
//! [wikipedia](https://en.wikipedia.org/wiki/Parsing_expression_grammar) or in the
//! [docs of other parser generators](https://pest.rs/book/grammars/syntax.html).
//!
//! Peginator is bootstrapped using its own
//! [syntax and grammar file](https://github.com/badicsalex/peginator/blob/master/grammar.ebnf),
//! which is somewhat easy-to-read.
//!
//! # Quickstart
//!
//! The grammars for peginator are written in a syntax similar to EBNF
//! (extended Backus-Naur form):
//!
//! ```ebnf
//! @export
//! FunctionDef = 'fn' name:Ident '(' param_list:ParamList ')' [ '->' return_value:Type ];
//!
//! ParamList = self_param:SelfParam {',' params:Param} | params:Param  {',' params:Param} | ;
//!
//! Param = name:Ident ':' typ: Type;
//!
//! SelfParam = [ref_type:ReferenceMarker] 'self';
//!
//! Type = [ref_type:ReferenceMarker] typename:Ident;
//!
//! ReferenceMarker = @:MutableReference | @:ImmutableReference;
//!
//! ImmutableReference = '&';
//! MutableReference = '&' 'mut';
//!
//! @string
//! @no_skip_ws
//! Ident = {'a'..'z' | 'A'..'Z' | '_' | '0'..'9'};
//! ```
//!
//! Based on the above grammar, peginator will generate the following types:
//!
//! ```ignore
//! pub struct FunctionDef {
//!     pub name: Ident,
//!     pub param_list: ParamList,
//!     pub return_value: Option<Type>,
//! }
//! pub struct ParamList {
//!     pub self_param: Option<SelfParam>,
//!     pub params: Vec<Param>,
//! }
//! pub struct Param {
//!     pub name: Ident,
//!     pub typ: Type,
//! }
//! pub struct SelfParam {
//!     pub ref_type: Option<ReferenceMarker>,
//! }
//! pub struct Type {
//!     pub ref_type: Option<ReferenceMarker>,
//!     pub typename: Ident,
//! }
//! pub enum ReferenceMarker {
//!     ImmutableReference(ImmutableReference),
//!     MutableReference(MutableReference),
//! }
//! pub struct ImmutableReference;
//! pub struct MutableReference;
//! pub type Ident = String;
//!
//! impl PegParser for FunctionDef { /* omitted */ }
//! ```
//!
//! Parsing then looks like this:
//! ```ignore
//! FunctionDef::parse("fn example(&self, input:&str, rectified:&mut Rect) -> ExampleResult;")
//! ```
//!
//! Which results in the folowing structure:
//! ```ignore
//! FunctionDef {
//!     name: "example",
//!     param_list: ParamList {
//!         self_param: Some(SelfParam {
//!             ref_type: Some(ImmutableReference(ImmutableReference)),
//!         }),
//!         params: [
//!             Param {
//!                 name: "input",
//!                 typ: Type {
//!                     ref_type: Some(ImmutableReference(ImmutableReference)),
//!                     typename: "str",
//!                 },
//!             },
//!             Param {
//!                 name: "rectified",
//!                 typ: Type {
//!                     ref_type: Some(MutableReference(MutableReference)),
//!                     typename: "Rect",
//!                 },
//!             },
//!         ],
//!     },
//!     return_value: Some(Type {
//!         ref_type: None,
//!         typename: "ExampleResult",
//!     }),
//! }
//! ```
//!
//! # Integration
//!
//! To use peginator grammars in your project, you should write the grammar and save it in the
//! project directory with the `.ebnf` extension.
//!
//! Then run the `peginator` command on it, and (optionally) rustfmt:
//!
//! ```text
//! peginator your_grammar.ebnf | rustfmt >src/grammar.rs
//! ```
//!
//! Make sure you also add the `peginator_runtime` crate as a dependency in your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! peginator_runtime = "0.4"
//! ```
//!
//! Once you have compiled your grammar, you can import the types, and the
//! [PegParser] trait, and you can start parsing strings:
//! ```ignore
//! use crate::grammar::YourRootType;
//! use peginator_runtime::PegParser;
//!
//! let parse_result = YourRootType::parse("some string (maybe?)");
//! println!("{:?}", parse_result);
//! ```
//!
//! Alternatively, you can use a buildscript using the [`peginator_codegen::Compile`]
//! struct by adding `peginator_codegen` as a build dependency in your `Cargo.toml`:
//!
//! ```toml
//! [build-dependencies]
//! peginator_codegen = "0.4"
//! ```
//!
//! And then in your `build.rs`:
//!
//! ```ignore
//! use peginator_codegen::Compile;
//!
//! fn main() {
//!     let out = format!("{}/grammar.rs", std::env::var("OUT_DIR").unwrap());
//!
//!     peginator_codegen::Compile::file("grammar.ebnf")
//!         .destination(out)
//!         .format()
//!         .run_exit_on_error();
//!
//!     println!("cargo:rerun-if-changed=grammar.ebnf");
//! }
//! ```
//!
//! For additional information, see the documentation of the [peginator_codegen] and
//! [peginator_runtime] crates.
//!
//! [peginator_codegen]: https://docs.rs/peginator_codegen/latest/peginator_codegen
//! [peginator_runtime]: https://docs.rs/peginator_runtime/latest/peginator_runtime
//! [PegParser]: https://docs.rs/peginator_runtime/latest/peginator_runtime/trait.PegParser.html
#![doc = include_str!("../doc/syntax.md")]

use std::fs;

use anyhow::Result;
use clap::Parser;
use colored::*;
use peginator_codegen::grammar::Grammar;
use peginator_codegen::runtime::{PegParser, PrettyParseError};
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
