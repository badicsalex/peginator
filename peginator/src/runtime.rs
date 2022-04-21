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
