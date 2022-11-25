// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

use std::error::Error;

use colored::*;

/// The type and specifics of the atomic match, used by [`ParseError`].
#[derive(Debug, Clone)]
pub enum ParseErrorSpecifics {
    /// Expected any character, but found end of input.
    ExpectedAnyCharacter,
    /// Expected a specific character.
    ExpectedCharacter {
        c: char,
    },
    /// Expected a character from a specific range.
    ExpectedCharacterRange {
        from: char,
        to: char,
    },
    /// Expected a specific string.
    ExpectedString {
        s: &'static str,
    },
    /// Expected to match a @char rule.
    ExpectedCharacterClass {
        name: &'static str,
    },
    /// Expected the end of file, but found additional characters.
    ExpectedEoi,
    /// A negative lookahead (`!`) rule part failed.
    NegativeLookaheadFailed,
    /// A custom check function failed
    CheckFunctionFailed {
        function_name: &'static str,
    },
    /// A custom extern rule failed
    ExternRuleFailed {
        error_string: &'static str,
    },
    LeftRecursionSentinel,

    /// An unknown error happened. Usually means there is a problem with peginator itself.
    Other,
}

impl ToString for ParseErrorSpecifics {
    fn to_string(&self) -> String {
        match self {
            ParseErrorSpecifics::ExpectedAnyCharacter => {
                "expected any character (found end of input)".to_string()
            }
            ParseErrorSpecifics::ExpectedCharacter { c } => format!("expected character '{c}'"),
            ParseErrorSpecifics::ExpectedCharacterRange { from, to } => {
                format!("expected character from range '{from}'-'{to}'")
            }
            ParseErrorSpecifics::ExpectedCharacterClass { name } => {
                format!("expected character from character class {name}")
            }
            ParseErrorSpecifics::ExpectedString { s } => format!("expected string \"{s}\""),
            ParseErrorSpecifics::ExpectedEoi => "expected end of input".to_string(),
            ParseErrorSpecifics::NegativeLookaheadFailed => {
                "negative lookahead condition failed".to_string()
            }
            ParseErrorSpecifics::CheckFunctionFailed { function_name } => {
                format!("check function '{function_name}' failed")
            }
            ParseErrorSpecifics::ExternRuleFailed { error_string } => {
                format!("extern function failed with '{error_string}'")
            }
            ParseErrorSpecifics::LeftRecursionSentinel => {
                "Left recursion sentinel reached, will probably retry.".to_string()
            }
            ParseErrorSpecifics::Other => "Unknown error. Sorry :(".to_string(),
        }
    }
}

/// An error happened during parsing (compact version).
///
/// During parsing, the parser records the furthest it got without encountering a match failure. The
/// error will contain both this furthest position and the first unmatched atomic matcher at this
/// position.
///
/// Convert to [`PrettyParseError`] before showing it to a user.
#[derive(Debug, Clone)]
pub struct ParseError {
    /// The byte-position of the furthest match failure.
    pub position: usize,
    /// The atomic match that was unsuccessful at the furthest parsing position.
    pub specifics: ParseErrorSpecifics,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let specifics = self.specifics.to_string();
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

/// An error happened during parsing (pretty version).
///
/// Converted from [`ParseError`], produces a very pretty, colored error message when printed with
/// regular [`std::fmt::Display`].
#[derive(Debug, Clone)]
pub struct PrettyParseError {
    err_string: String,
}

impl PrettyParseError {
    /// Convert from [`ParseError`]
    ///
    /// The parsed `text` needs to be supplied to show the context of the error.
    ///
    /// The `source_file` parameter is used to print the error with the same format `rustc` does.
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
            err = err.specifics.to_string().bold().white(),
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
