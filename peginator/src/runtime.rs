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

#[derive(Debug, Clone)]
pub struct ParseState<'a> {
    full_string: &'a str,
    start_index: usize,
    pub indentation_level: usize,
    tracing: bool,
}

impl<'a> ParseState<'a> {
    pub fn new(s: &'a str) -> ParseState<'a> {
        Self {
            full_string: s,
            start_index: 0,
            indentation_level: 0,
            tracing: false,
        }
    }
    pub fn enable_tracing(self, tracing: bool) -> Self {
        Self { tracing, ..self }
    }

    pub fn print_trace<F: Fn() -> ColoredString>(&self, f: F) {
        if self.tracing {
            let indentation = "    ".repeat(self.indentation_level);
            println!("{}{}", indentation, f())
        }
    }

    pub fn s(&self) -> &str {
        &self.full_string[self.start_index..]
    }

    pub fn is_empty(&self) -> bool {
        self.s().is_empty()
    }

    pub fn advance(self, length: usize) -> Self {
        Self {
            start_index: self.start_index + length,
            ..self
        }
    }
    pub fn slice_until(&self, other: &ParseState) -> &str {
        &self.full_string[self.start_index..other.start_index]
    }
    pub fn skip_whitespace(self) -> Self {
        let mut result = self;
        while let Some(ch) = result.s().chars().next() {
            if ch.is_whitespace() {
                result.start_index += ch.len_utf8();
            } else {
                break;
            }
        }
        result
    }
    pub fn indent(self) -> Self {
        Self {
            indentation_level: self.indentation_level + 1,
            ..self
        }
    }
    pub fn dedent(self) -> Self {
        Self {
            indentation_level: self.indentation_level - 1,
            ..self
        }
    }
}

pub type ParseResult<'a, T> = Result<(T, ParseState<'a>), ParseError>;

pub fn parse_char(state: ParseState) -> ParseResult<char> {
    let result = state.s().chars().next().ok_or(ParseError)?;
    Ok((result, state.advance(result.len_utf8())))
}

pub fn parse_string_literal<'a>(state: ParseState<'a>, s: &str) -> ParseResult<'a, ()> {
    if !state.s().starts_with(s) {
        Err(ParseError)
    } else {
        Ok(((), state.advance(s.len())))
    }
}

pub fn parse_character_literal(state: ParseState, c: char) -> ParseResult<()> {
    let result = state.s().chars().next().ok_or(ParseError)?;
    if result != c {
        Err(ParseError)
    } else {
        Ok(((), state.advance(result.len_utf8())))
    }
}

pub fn parse_character_range(state: ParseState, from: char, to: char) -> ParseResult<()> {
    let result = state.s().chars().next().ok_or(ParseError)?;
    if result < from || result > to {
        Err(ParseError)
    } else {
        Ok(((), state.advance(result.len_utf8())))
    }
}

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
