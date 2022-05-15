// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

use super::{IndentedTracer, NoopTracer, ParseError, ParseTracer};

pub trait PegParser: Sized {
    fn parse(s: &str) -> Result<Self, ParseError> {
        Self::parse_advanced::<NoopTracer>(s, &ParseSettings::default())
    }
    fn parse_with_trace(s: &str) -> Result<Self, ParseError> {
        Self::parse_advanced::<IndentedTracer>(s, &ParseSettings::default())
    }

    fn parse_advanced<T: ParseTracer>(
        s: &str,
        settings: &ParseSettings,
    ) -> Result<Self, ParseError>;
}

#[derive(Debug, Default)]
pub struct ParseSettings {}
