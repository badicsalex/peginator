// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

use super::{ParseError, ParseState};

#[derive(Debug, Clone)]
pub struct ParseOk<'a, T> {
    pub result: T,
    pub state: ParseState<'a>,
}

impl<'a, T> ParseOk<'a, T> {
    #[inline]
    pub fn map<T2, F>(self, f: F) -> ParseOk<'a, T2>
    where
        F: Fn(T) -> T2,
    {
        ParseOk::<T2> {
            result: f(self.result),
            state: self.state,
        }
    }

    #[inline]
    pub fn map_with_state<T2, F>(self, f: F) -> ParseOk<'a, T2>
    where
        F: Fn(T, &ParseState) -> T2,
    {
        ParseOk::<T2> {
            result: f(self.result, &self.state),
            state: self.state,
        }
    }
}

pub type ParseResult<'a, T> = Result<ParseOk<'a, T>, ParseError>;
