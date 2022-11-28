// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

//! This crate contains the code used for generating parsing code from a
//! grammar file. Unless you are using [`Compile`] in a buildscript, you
//! probably want to see the [peginator] crate documentation instead.
//!
//! [peginator]: https://docs.rs/peginator/latest/peginator

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
pub use common::{CodegenGrammar, CodegenSettings};
pub use header::generate_source_header;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const BUILD_TIME: &str = env!("PEGINATOR_BUILD_TIME");
