// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

#[derive(Debug)]
pub struct ParseError;

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Error while parsing.")
    }
}

#[derive(Debug, Clone)]
pub struct ParseState<'a> {
    full_string: &'a str,
    start_index: usize,
}

impl ParseState<'_> {
    pub fn s(&self) -> &str {
        &self.full_string[self.start_index..]
    }

    pub fn is_empty(&self) -> bool {
        self.s().is_empty()
    }

    pub fn advance(self, length: usize) -> Self {
        Self {
            full_string: self.full_string,
            start_index: self.start_index + length,
        }
    }
}

pub type ParseResult<'a, T> = Result<(T, ParseState<'a>), ParseError>;

pub fn parse_char<'a>(state: ParseState<'a>) -> ParseResult<'a, char> {
    todo!()
}

pub fn parse_string_literal<'a>(state: ParseState<'a>, str: &str) -> ParseResult<'a, ()> {
    todo!();
}

pub fn parse_character_literal<'a>(state: ParseState<'a>, c: char) -> ParseResult<'a, ()> {
    todo!();
}

pub fn parse_character_range<'a>(
    state: ParseState<'a>,
    from: char,
    to: char,
) -> ParseResult<'a, ()> {
    todo!();
}

pub fn parse_closure<'a, T, F: Fn(ParseState) -> ParseResult<T>>(
    state: ParseState<'a>,
    closure: F,
    min_num: i64,
) -> ParseResult<'a, Vec<T>> {
    todo!();
}
