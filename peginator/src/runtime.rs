// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

use colored::*;
use std::{collections::HashMap, error::Error};

#[derive(Debug, Clone)]
pub enum ParseErrorSpecifics {
    ExpectedAnyCharacter,
    ExpectedCharacter { c: char },
    ExpectedCharacterRange { from: char, to: char },
    ExpectedString { s: &'static str },
    ExpectedEoi,
    NegativeLookaheadFailed,
    // Special ones
    Other,
    MisusedFarthestError,
}

#[derive(Debug, Clone)]
pub struct ParseError {
    position: usize,
    specifics: ParseErrorSpecifics,
}

impl ParseError {
    #[inline]
    fn farther_than(&self, other: &Self) -> bool {
        self.position > other.position
    }

    fn specifics_as_string(&self) -> String {
        match self.specifics {
            ParseErrorSpecifics::ExpectedAnyCharacter => {
                "expected any character (found end of input)".to_string()
            }
            ParseErrorSpecifics::ExpectedCharacter { c } => format!("expected character '{}'", c),
            ParseErrorSpecifics::ExpectedCharacterRange { from, to } => {
                format!("expected character from range '{}'-'{}'", from, to)
            }
            ParseErrorSpecifics::ExpectedString { s } => format!("expected string \"{}\"", s),
            ParseErrorSpecifics::ExpectedEoi => "expected end of input".to_string(),
            ParseErrorSpecifics::NegativeLookaheadFailed => {
                "negative lookahead condition failed".to_string()
            }
            ParseErrorSpecifics::Other => "Unknown error. Sorry :(".to_string(),
            ParseErrorSpecifics::MisusedFarthestError => {
                "misused 'farthest_error' on ParseState. Serious internal error in peginator"
                    .to_string()
            }
        }
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
    // Note: Storing this in the state is incorrect if memoization is applied,
    // since only the first parsing result will be stored, even if subsequent
    // calls have an even further error.
    //
    // It should be part of the ParseResult instead, but that would require
    // rewriting half the world, so this is a big TODO for now.
    //
    // Especially since this is just a diagnostic heuristic.
    farthest_error: Option<ParseError>,
}

impl<'a> ParseState<'a> {
    #[inline]
    pub fn new(s: &'a str, settings: &ParseSettings) -> ParseState<'a> {
        Self {
            partial_string: s,
            start_index: 0,
            indentation_level: 0,
            tracing: settings.tracing,
            farthest_error: None,
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

    #[inline]
    pub fn cache_key(&self) -> usize {
        self.start_index
    }

    #[inline]
    pub fn record_error(self, err: ParseError) -> Self {
        if let Some(current_farthest_error) = &self.farthest_error {
            // We are overwriting the current error with the more recent one, because the more
            // recent error tends to be more specific, especially in case of sequences
            if !current_farthest_error.farther_than(&err) {
                Self {
                    farthest_error: Some(err),
                    ..self
                }
            } else {
                self
            }
        } else {
            Self {
                farthest_error: Some(err),
                ..self
            }
        }
    }

    #[inline]
    pub fn report_error(self, specifics: ParseErrorSpecifics) -> ParseError {
        let new_error = ParseError {
            position: self.start_index,
            specifics,
        };
        self.record_error(new_error).report_farthest_error()
    }

    #[inline]
    pub fn report_farthest_error(self) -> ParseError {
        match self.farthest_error {
            Some(err) => err,
            None => self.report_error(ParseErrorSpecifics::MisusedFarthestError),
        }
    }
}

pub type ParseResult<'a, T> = Result<(T, ParseState<'a>), ParseError>;

pub type CacheEntries<'a, T> = HashMap<usize, ParseResult<'a, T>>;

/// Hand-written 'rule parser' for parsing a single cahracter.
///
/// Should always look just like all the other generated parse functions.
#[inline(always)]
pub fn parse_char<'a, _CT>(state: ParseState<'a>, _cache: &_CT) -> ParseResult<'a, char> {
    let result = state.s().chars().next().ok_or_else(|| {
        state
            .clone()
            .report_error(ParseErrorSpecifics::ExpectedAnyCharacter)
    })?;
    // SAFETY:
    // Callers of this function are responsible that these preconditions are satisfied:
    //    Indexes must lie on UTF-8 sequence boundaries.
    //
    // We are skipping a full character, so we should be OK.
    Ok((result, unsafe { state.advance(result.len_utf8()) }))
}

#[inline(always)]
pub fn parse_string_literal<'a>(state: ParseState<'a>, s: &'static str) -> ParseResult<'a, ()> {
    if !state.s().starts_with(s) {
        Err(state.report_error(ParseErrorSpecifics::ExpectedString { s }))
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
            Err(state.report_error(ParseErrorSpecifics::ExpectedCharacter { c }))
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
        Err(state.report_error(ParseErrorSpecifics::ExpectedCharacter { c }))
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
            Err(state.report_error(ParseErrorSpecifics::ExpectedCharacterRange { from, to }))
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
        let result = state.s().chars().next().ok_or_else(|| {
            state
                .clone()
                .report_error(ParseErrorSpecifics::ExpectedCharacterRange { from, to })
        })?;
        if result < from || result > to {
            Err(state.report_error(ParseErrorSpecifics::ExpectedCharacterRange { from, to }))
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
pub fn run_rule_parser<'a, T, F, CT>(
    f: F,
    name: &'static str,
    state: ParseState<'a>,
    cache: &mut CT,
) -> ParseResult<'a, T>
where
    F: for<'b> Fn(ParseState<'a>, &'b mut CT) -> ParseResult<'a, T>,
{
    state.print_trace(|| format!("{}?", name).yellow());
    state.print_trace(|| format!("{:?}", state.s().chars().take(50).collect::<String>()).normal());
    let result = f(state.clone().indent(), cache);
    state.print_trace(|| match &result {
        Ok(_) => "Ok".green(),
        Err(err) => format!("Error: {}", err.specifics_as_string()).red(),
    });
    result.map(|(result, state)| (result, state.dedent()))
}
