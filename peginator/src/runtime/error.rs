// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

use colored::*;
use std::error::Error;

#[derive(Debug, Clone)]
pub enum ParseErrorSpecifics {
    ExpectedAnyCharacter,
    ExpectedCharacter { c: char },
    ExpectedCharacterRange { from: char, to: char },
    ExpectedString { s: &'static str },
    ExpectedCharacterClass { name: &'static str },
    ExpectedEoi,
    NegativeLookaheadFailed,
    // Special ones
    Other,
}

#[derive(Debug, Clone)]
pub struct ParseError {
    pub position: usize,
    pub specifics: ParseErrorSpecifics,
}

impl ParseError {
    #[inline]
    fn farther_than(&self, other: &Self) -> bool {
        self.position > other.position
    }

    pub fn specifics_as_string(&self) -> String {
        match self.specifics {
            ParseErrorSpecifics::ExpectedAnyCharacter => {
                "expected any character (found end of input)".to_string()
            }
            ParseErrorSpecifics::ExpectedCharacter { c } => format!("expected character '{}'", c),
            ParseErrorSpecifics::ExpectedCharacterRange { from, to } => {
                format!("expected character from range '{}'-'{}'", from, to)
            }
            ParseErrorSpecifics::ExpectedCharacterClass { name } => {
                format!("expected character from character class {}", name)
            }
            ParseErrorSpecifics::ExpectedString { s } => format!("expected string \"{}\"", s),
            ParseErrorSpecifics::ExpectedEoi => "expected end of input".to_string(),
            ParseErrorSpecifics::NegativeLookaheadFailed => {
                "negative lookahead condition failed".to_string()
            }
            ParseErrorSpecifics::Other => "Unknown error. Sorry :(".to_string(),
        }
    }
}

#[inline]
pub fn combine_errors(first: Option<ParseError>, second: Option<ParseError>) -> Option<ParseError> {
    match (first, second) {
        (None, None) => None,
        (None, Some(x)) => Some(x),
        (Some(x), None) => Some(x),
        (Some(a), Some(b)) => Some(if b.farther_than(&a) { b } else { a }),
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let specifics = self.specifics_as_string();
        write!(
            f,
            "Parse error on byte position {} while parsing: {}",
            self.position, specifics
        )
    }
}

impl Error for ParseError {}

#[derive(Debug, Clone)]
struct IndexedStringLine<'a> {
    pub s: &'a str,
    pub lineno: usize,
    pub start_offset: usize,
    pub end_offset: usize,
}

struct IndexedStringLineIterator<'a> {
    source: &'a str,
    lineno: usize,
    byte_offset: usize,
}

impl<'a> IndexedStringLineIterator<'a> {
    fn new(s: &'a str) -> IndexedStringLineIterator<'a> {
        IndexedStringLineIterator {
            source: s,
            lineno: 0,
            byte_offset: 0,
        }
    }
}

impl<'a> Iterator for IndexedStringLineIterator<'a> {
    type Item = IndexedStringLine<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.byte_offset >= self.source.bytes().len() {
            return None;
        }
        let next_offset = self.source[self.byte_offset..]
            .bytes()
            .position(|b| b == b'\n')
            .map(|p| p + self.byte_offset)
            .unwrap_or_else(|| self.source.bytes().len())
            + 1;
        let result = IndexedStringLine {
            s: &self.source[self.byte_offset..next_offset - 1],
            lineno: self.lineno,
            start_offset: self.byte_offset,
            end_offset: next_offset,
        };
        self.lineno += 1;
        self.byte_offset = next_offset;
        Some(result)
    }
}

#[derive(Debug, Clone)]
pub struct PrettyParseError {
    err_string: String,
}

impl PrettyParseError {
    pub fn from_parse_error(err: &ParseError, text: &str, source_file: Option<&str>) -> Self {
        let target_line = IndexedStringLineIterator::new(text)
            .find(|l| l.start_offset <= err.position && l.end_offset >= err.position)
            .unwrap();
        let character_position = target_line
            .s
            .char_indices()
            .map(|(cp, _c)| cp)
            .position(|cp| cp == err.position - target_line.start_offset)
            .unwrap_or(0);
        let position = if let Some(f) = source_file {
            format!(
                "{}:{:?}:{:?}",
                f,
                target_line.lineno + 1,
                character_position + 1
            )
        } else {
            format!(
                "Line {} character {}",
                target_line.lineno + 1,
                character_position + 1
            )
        };
        let err_string = format!(
            "{err}\n{arrow}{position}\n{pipe}\n{pipe}{the_line}\n{pipe}{caret:>caret_offset$}\n",
            err = err.specifics_as_string().bold().white(),
            position = position,
            the_line = target_line.s.trim_end(),
            caret = "^".bold().red(),
            caret_offset = character_position + 1,
            arrow = "--> ".bold().blue(),
            pipe = " |  ".bold().blue(),
        );
        Self { err_string }
    }
}

impl std::fmt::Display for PrettyParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.err_string)
    }
}

impl Error for PrettyParseError {}
