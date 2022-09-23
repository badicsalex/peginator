// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.
use build_time::build_time_utc;

mod char_rule;
mod choice;
mod closure;
mod common;
mod eoi;
mod extern_rule;
mod field;
mod grammar;
mod header;
mod lookahead;
mod misc;
mod optional;
mod rule;
mod sequence;
mod string;

pub use crate::grammar::Grammar;
pub use common::{CodegenGrammar, CodegenSettings};
pub use header::generate_source_header;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const BUILD_TIME: &str = build_time_utc!();
