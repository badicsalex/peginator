// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

#[derive(Debug)]
pub struct ParseError {
    msg: String,
}

impl ParseError {
    fn new(s: &str) -> Self {
        Self { msg: s.to_string() }
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Error while parsing: {}", self.msg)
    }
}

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

    pub fn advance(&mut self, length: usize) {
        self.start_index += length
    }
}

pub type ParseResult<'a, T> = Result<(T, ParseState<'a>), ParseError>;
pub type InnerParseResult<'a> = Result<ParseState<'a>, ParseError>;
pub type PossibleParseError = Result<(), ParseError>;

pub struct Builtins;

impl Builtins {
    pub fn start_choice() -> Builtins {
        Builtins
    }
    pub fn start_sequence() -> Builtins {
        Builtins
    }
    pub fn choice<'a, F: FnOnce() -> InnerParseResult<'a>>(f: F) -> Builtins {
        Builtins
    }
    pub fn part<'a, F: FnOnce() -> InnerParseResult<'a>>(f: F) -> Builtins {
        Builtins
    }
    pub fn negative_lookahead<'a, F: FnOnce() -> InnerParseResult<'a>>(f: F) -> Builtins {
        Builtins
    }
    pub fn closure<'a, F: FnOnce() -> InnerParseResult<'a>>(f: F) -> Builtins {
        Builtins
    }
    pub fn closure_plus<'a, F: FnOnce() -> InnerParseResult<'a>>(f: F) -> Builtins {
        Builtins
    }
    pub fn end() -> Builtins {
        Builtins
    }
}
