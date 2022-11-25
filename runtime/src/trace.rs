// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.
use colored::*;

use super::{ParseResult, ParseState};

pub trait ParseTracer: Clone + Copy {
    fn print_informative(&mut self, s: &str) {
        let _ = s;
    }
    fn print_trace_start(&mut self, state: &ParseState, name: &str) {
        let _ = (state, name);
    }
    fn print_trace_result<T>(&mut self, result: &ParseResult<T>) {
        let _ = result;
    }

    fn new() -> Self;
}

#[derive(Debug, Clone, Copy)]
pub struct IndentedTracer {
    indentation_level: usize,
}

impl ParseTracer for IndentedTracer {
    #[inline]
    fn print_informative(&mut self, s: &str) {
        let indentation = "    ".repeat(self.indentation_level);
        eprintln!("{indentation}{}", s.cyan());
    }

    #[inline]
    fn print_trace_start(&mut self, state: &ParseState, name: &str) {
        let indentation = "    ".repeat(self.indentation_level);
        eprintln!("{indentation}{:?}", state.first_n_chars(50));
        eprintln!("{indentation}{}?", name.yellow());
        self.indentation_level += 1;
    }

    #[inline]
    fn print_trace_result<T>(&mut self, result: &ParseResult<T>) {
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
        self.indentation_level -= 1;
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
    #[inline]
    fn new() -> Self {
        Self {}
    }
}
