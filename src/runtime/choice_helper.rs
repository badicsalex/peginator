// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

use super::{ParseOk, ParseResult, ParseState};

pub struct ChoiceHelper<'a, T> {
    state: ParseState<'a>,
    result: Option<ParseOk<'a, T>>,
}

impl<'a, T> ChoiceHelper<'a, T> {
    #[inline]
    pub fn new(state: ParseState<'a>) -> Self {
        Self {
            state,
            result: None,
        }
    }

    #[inline]
    pub fn choice(mut self, parse_fn: impl FnOnce(ParseState<'a>) -> ParseResult<'a, T>) -> Self {
        if self.result.is_none() {
            match parse_fn(self.state.clone()) {
                Ok(ok_result) => self.result = Some(ok_result),
                Err(err) => self.state = self.state.record_error(err),
            }
        }
        self
    }

    #[inline]
    pub fn end(self) -> ParseResult<'a, T> {
        match self.result {
            Some(ok) => Ok(ok),
            None => Err(self.state.report_farthest_error()),
        }
    }
}
