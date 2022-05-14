// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

use colored::*;

use super::{ParseError, ParseErrorSpecifics, ParseResult, ParseSettings};

#[derive(Debug, Clone)]
pub struct ParseState<'a> {
    partial_string: &'a str,
    start_index: usize,
    pub indentation_level: usize,
    tracing: bool,
}

impl<'a> ParseState<'a> {
    #[inline]
    pub fn new(s: &'a str, settings: &ParseSettings) -> ParseState<'a> {
        Self {
            partial_string: s,
            start_index: 0,
            indentation_level: 0,
            tracing: settings.tracing,
        }
    }

    #[inline]
    pub fn print_trace_start(&self, name: &str) {
        if self.tracing {
            let indentation = "    ".repeat(self.indentation_level);
            println!(
                "{}{:?}",
                indentation,
                self.s().chars().take(50).collect::<String>()
            );
            println!("{}{}?", indentation, name.yellow());
        }
    }

    #[inline]
    pub fn print_trace_cached(&self, name: &str) {
        if self.tracing {
            let indentation = "    ".repeat(self.indentation_level);
            println!(
                "{}{:?}",
                indentation,
                self.s().chars().take(50).collect::<String>()
            );
            println!(
                "{}{} {}",
                indentation,
                "Cached:".bright_yellow(),
                name.yellow()
            );
        }
    }

    #[inline]
    pub fn print_trace_result<T>(&self, result: &ParseResult<T>) {
        if self.tracing {
            let indentation = "    ".repeat(self.indentation_level);
            match &result {
                Ok(_) => println!("{}{}", indentation, "Ok".green()),
                Err(err) => println!(
                    "{}{} {}",
                    indentation,
                    "Error:".red(),
                    err.specifics_as_string()
                ),
            };
        }
    }

    #[inline]
    pub fn s(&self) -> &str {
        self.partial_string
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.s().is_empty()
    }

    /// Advance the parsing pointer n bytes
    ///
    /// # Safety
    /// Callers of this function are responsible that these preconditions are satisfied:
    ///    Indexes must lie on UTF-8 sequence boundaries.
    #[inline]
    #[warn(unsafe_op_in_unsafe_fn)]
    pub unsafe fn advance(self, length: usize) -> Self {
        // We are eschewing mainly the utf-8 codepoint check here,
        // because the caller can be sure that everything is fine.

        if length > self.partial_string.len() {
            // This should be optimized out in most cases
            panic!("String length overrun in advance()")
        };
        Self {
            start_index: self.start_index + length,
            // SAFETY:
            // Callers of this function are responsible that these preconditions are satisfied:
            //    Indexes must lie on UTF-8 sequence boundaries.
            //    The starting index must not exceed the ending index;
            //    Indexes must be within bounds of the original slice;
            partial_string: unsafe { self.partial_string.get_unchecked(length..) },
            ..self
        }
    }

    #[inline]
    pub fn slice_until(&self, other: &ParseState) -> &str {
        &self.partial_string[..(other.start_index - self.start_index)]
    }

    #[inline]
    pub fn range_until(&self, other: &ParseState) -> std::ops::Range<usize> {
        self.start_index..other.start_index
    }

    #[inline]
    pub fn skip_whitespace(self) -> Self {
        let mut result = self;
        while !result.is_empty() {
            if result.s().as_bytes()[0].is_ascii_whitespace() {
                // SAFETY:
                // Callers of this function are responsible that these preconditions are satisfied:
                //    Indexes must lie on UTF-8 sequence boundaries.
                //
                // The byte we are skipping is ASCII, so we are OK.
                result = unsafe { result.advance(1) };
            } else {
                break;
            }
        }
        result
    }

    #[inline]
    pub fn indent(self) -> Self {
        Self {
            indentation_level: self.indentation_level + 1,
            ..self
        }
    }

    #[inline]
    pub fn dedent(self) -> Self {
        Self {
            indentation_level: self.indentation_level - 1,
            ..self
        }
    }

    #[inline]
    pub fn cache_key(&self) -> usize {
        self.start_index
    }

    #[inline]
    pub fn report_error(self, specifics: ParseErrorSpecifics) -> ParseError {
        ParseError {
            position: self.start_index,
            specifics,
        }
    }
}
