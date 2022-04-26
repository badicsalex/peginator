// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

use colored::*;
use std::error::Error;

#[derive(Debug)]
pub struct ParseError;

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Error while parsing.")
    }
}

impl Error for ParseError {}

pub trait PegParser: Sized {
    fn parse(s: &str) -> Result<Self, ParseError> {
        Self::parse_advanced(s, &ParseSettings::default())
    }

    fn parse_advanced(s: &str, settings: &ParseSettings) -> Result<Self, ParseError>;
}

#[derive(Debug, Default)]
pub struct ParseSettings {
    pub tracing: bool,
}

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
    pub fn print_trace<F: Fn() -> ColoredString>(&self, f: F) {
        if self.tracing {
            let indentation = "    ".repeat(self.indentation_level);
            println!("{}{}", indentation, f())
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
}

pub type ParseResult<'a, T> = Result<(T, ParseState<'a>), ParseError>;

#[inline(always)]
pub fn parse_char(state: ParseState) -> ParseResult<char> {
    let result = state.s().chars().next().ok_or(ParseError)?;
    // SAFETY:
    // Callers of this function are responsible that these preconditions are satisfied:
    //    Indexes must lie on UTF-8 sequence boundaries.
    //
    // We are skipping a full character, so we should be OK.
    Ok((result, unsafe { state.advance(result.len_utf8()) }))
}

#[inline(always)]
pub fn parse_string_literal<'a>(state: ParseState<'a>, s: &str) -> ParseResult<'a, ()> {
    if !state.s().starts_with(s) {
        Err(ParseError)
    } else {
        // SAFETY:
        // Callers of this function are responsible that these preconditions are satisfied:
        //    Indexes must lie on UTF-8 sequence boundaries.
        //
        // We are skipping a correct string's length, so we should be OK.
        Ok(((), unsafe { state.advance(s.len()) }))
    }
}

#[inline(always)]
pub fn parse_character_literal(state: ParseState, c: char) -> ParseResult<()> {
    if c.is_ascii() {
        // fast path
        if state.is_empty() || state.s().as_bytes()[0] != c as u8 {
            Err(ParseError)
        } else {
            // SAFETY:
            // Callers of this function are responsible that these preconditions are satisfied:
            //    Indexes must lie on UTF-8 sequence boundaries.
            //
            // The byte we are skipping is ASCII, so we are OK.
            Ok(((), unsafe { state.advance(1) }))
        }
    } else if !state.s().starts_with(c) {
        // utf-8 path
        Err(ParseError)
    } else {
        // SAFETY:
        // Callers of this function are responsible that these preconditions are satisfied:
        //    Indexes must lie on UTF-8 sequence boundaries.
        //
        // We are skipping a full character, so we should be OK.
        Ok(((), unsafe { state.advance(c.len_utf8()) }))
    }
}

#[inline(always)]
pub fn parse_character_range(state: ParseState, from: char, to: char) -> ParseResult<()> {
    if from.is_ascii() && to.is_ascii() {
        // fast path
        if state.is_empty()
            || state.s().as_bytes()[0] < from as u8
            || state.s().as_bytes()[0] > to as u8
        {
            Err(ParseError)
        } else {
            // SAFETY:
            // Callers of this function are responsible that these preconditions are satisfied:
            //    Indexes must lie on UTF-8 sequence boundaries.
            //
            // The byte we are skipping is ASCII, so we are OK.
            Ok(((), unsafe { state.advance(1) }))
        }
    } else {
        // utf-8 path
        let result = state.s().chars().next().ok_or(ParseError)?;
        if result < from || result > to {
            Err(ParseError)
        } else {
            // SAFETY:
            // Callers of this function are responsible that these preconditions are satisfied:
            //    Indexes must lie on UTF-8 sequence boundaries.
            //
            // We are skipping a full character, so we should be OK.
            Ok(((), unsafe { state.advance(result.len_utf8()) }))
        }
    }
}

#[inline(always)]
pub fn run_rule_parser<'a, T, F>(
    f: F,
    name: &'static str,
    state: ParseState<'a>,
) -> ParseResult<'a, T>
where
    F: Fn(ParseState) -> ParseResult<T>,
{
    state.print_trace(|| format!("{}?", name).yellow());
    state.print_trace(|| format!("{:?}", state.s().chars().take(50).collect::<String>()).normal());
    let result = f(state.clone().indent());
    state.print_trace(|| {
        if result.is_ok() {
            "Ok".green()
        } else {
            "Err".red()
        }
    });
    result.map(|(result, state)| (result, state.dedent()))
}
