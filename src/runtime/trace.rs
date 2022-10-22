// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.
use colored::*;

use super::{ParseResult, ParseState};

pub trait ParseTracer: Clone + Copy {
    fn run_traced<'a, T, F>(
        self,
        name: &'static str,
        state: ParseState<'a>,
        f: F,
    ) -> ParseResult<'a, T>
    where
        F: FnOnce(ParseState<'a>, Self) -> ParseResult<'a, T>;

    fn print_informative(&self, s: &str);

    fn new() -> Self;
}

#[derive(Debug, Clone, Copy)]
pub struct IndentedTracer {
    indentation_level: usize,
}

impl IndentedTracer {
    #[inline]
    pub fn indented(&self) -> Self {
        Self {
            indentation_level: self.indentation_level + 1,
        }
    }

    #[inline]
    pub fn print_trace_start(&self, state: &ParseState, name: &str) {
        let indentation = "    ".repeat(self.indentation_level);
        eprintln!("{indentation}{:?}", state.first_n_chars(50));
        eprintln!("{indentation}{}?", name.yellow());
    }

    #[inline]
    pub fn print_trace_result<T>(&self, result: &ParseResult<T>) {
        let indentation = "    ".repeat(self.indentation_level);
        match &result {
            Ok(ok_result) => {
                eprintln!("{indentation}{:?}", ok_result.state.first_n_chars(50));
                eprintln!("{indentation}{}", "Ok".green());
            }
            Err(err) => eprintln!(
                "{indentation}{} {}",
                "Error:".red(),
                err.specifics.to_string()
            ),
        };
    }
}

impl ParseTracer for IndentedTracer {
    #[inline(always)]
    fn run_traced<'a, T, F>(
        self,
        name: &'static str,
        state: ParseState<'a>,
        f: F,
    ) -> ParseResult<'a, T>
    where
        F: FnOnce(ParseState<'a>, Self) -> ParseResult<'a, T>,
    {
        self.print_trace_start(&state, name);
        let result = f(state, self.indented());
        self.print_trace_result(&result);
        result
    }

    #[inline]
    fn print_informative(&self, s: &str) {
        let indentation = "    ".repeat(self.indentation_level);
        eprintln!("{indentation}{}", s.cyan());
    }

    #[inline]
    fn new() -> Self {
        Self {
            indentation_level: 0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct NoopTracer {}

impl ParseTracer for NoopTracer {
    #[inline(always)]
    fn run_traced<'a, T, F>(
        self,
        _name: &'static str,
        state: ParseState<'a>,
        f: F,
    ) -> ParseResult<'a, T>
    where
        F: for<'b> FnOnce(ParseState<'a>, Self) -> ParseResult<'a, T>,
    {
        f(state, self)
    }

    #[inline(always)]
    fn print_informative(&self, _s: &str) {}

    #[inline]
    fn new() -> Self {
        Self {}
    }
}
