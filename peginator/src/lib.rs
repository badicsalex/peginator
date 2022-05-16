// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

use build_time::build_time_utc;

mod grammar;

mod codegen;

pub mod buildscript;
pub mod runtime;

pub use codegen::{generate_source_header, CodegenGrammar, CodegenSettings};
pub use grammar::Grammar;
pub use runtime::{ParseError, ParseSettings, PegParser, PrettyParseError};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const BUILD_TIME: &str = build_time_utc!();
