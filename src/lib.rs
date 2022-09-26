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
//! Then run the `peginator-compile` command on it, and (optionally) rustfmt:
//!
//! ```text
//! peginator-compile your_grammar.ebnf >src/grammar.rs
//! rustfmt src/grammar.rs
//! ```
//!
//! Once you have compiled your grammar, you can import the types, and the
//! [`PegParser`] trait, and you can start parsing strings:
//! ```ignore
//! use crate::grammar::YourRootType;
//! use peginator::PegParser;
//!
//! let parse_result = YourRootType::parse("some string (maybe?)");
//! println!("{:?}", parse_result);
//! ```
//!
//! Alternatively, you can use a buildscript using the [`buildscript::Compile`] struct.
//!
#![doc = include_str!("../doc/syntax.md")]

#[doc(hidden)]
pub mod grammar;

pub mod buildscript;
#[doc(hidden)]
pub mod codegen;
#[doc(hidden)]
pub mod runtime;

pub use runtime::{
    ParseError, ParseErrorSpecifics, ParseSettings, PegParser, PegPosition, PrettyParseError,
};
