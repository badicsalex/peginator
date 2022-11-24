// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

mod char_rule;
mod choice;
mod closure;
mod common;
mod eoi;
mod extern_rule;
mod field;
mod grammar;
mod header;
mod include_rule;
mod lookahead;
mod misc;
mod optional;
mod rule;
mod sequence;
mod string;

pub use common::{CodegenGrammar, CodegenSettings};
pub use header::generate_source_header;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const BUILD_TIME: &str = env!("PEGINATOR_BUILD_TIME");
