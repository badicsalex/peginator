// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

//! This crate contains the code used for generating [`peginator`] parsing code from a
//! grammar file. Unless you are using [`Compile`] in a buildscript, you
//! probably want to see the [`peginator`] crate documentation instead.
//!
//! To integrate [`peginator`] using a buildscript, first add `peginator_codegen` as
//! a build dependency in your `Cargo.toml`:
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
//! See the documentation of [`Compile`] for more advanced options.
//!
//! [`peginator`]: https://docs.rs/peginator

mod grammar;

mod buildscript;
mod char_rule;
mod choice;
mod closure;
mod common;
mod eoi;
mod extern_rule;
mod field;
mod header;
mod include_rule;
mod lookahead;
mod misc;
mod optional;
mod rule;
mod sequence;
mod string;

pub use buildscript::Compile;
#[doc(hidden)]
pub use common::{CodegenGrammar, CodegenSettings};
#[doc(hidden)]
pub use grammar::Grammar;
#[doc(hidden)]
pub use header::generate_source_header;

#[doc(hidden)]
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[doc(hidden)]
pub const BUILD_TIME: &str = env!("PEGINATOR_BUILD_TIME");
