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
    pub fn enable_tracing(self, tracing: bool) -> Self {
        Self { tracing, ..self }
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

    #[inline]
    pub fn advance(self, length: usize) -> Self {
        Self {
            start_index: self.start_index + length,
            partial_string: &self.partial_string[length..],
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
        while let Some(ch) = result.s().chars().next() {
            if ch.is_whitespace() {
                result = result.advance(ch.len_utf8());
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
pub fn parse_char_internal(state: ParseState) -> ParseResult<char> {
    let result = state.s().chars().next().ok_or(ParseError)?;
    Ok((result, state.advance(result.len_utf8())))
}

#[inline(always)]
pub fn parse_string_literal<'a>(state: ParseState<'a>, s: &str) -> ParseResult<'a, ()> {
    if !state.s().starts_with(s) {
        Err(ParseError)
    } else {
        Ok(((), state.advance(s.len())))
    }
}

#[inline(always)]
pub fn parse_character_literal(state: ParseState, c: char) -> ParseResult<()> {
    let result = state.s().chars().next().ok_or(ParseError)?;
    if result != c {
        Err(ParseError)
    } else {
        Ok(((), state.advance(result.len_utf8())))
    }
}

#[inline(always)]
pub fn parse_character_range(state: ParseState, from: char, to: char) -> ParseResult<()> {
    let result = state.s().chars().next().ok_or(ParseError)?;
    if result < from || result > to {
        Err(ParseError)
    } else {
        Ok(((), state.advance(result.len_utf8())))
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
