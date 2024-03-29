// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

use super::{ParseError, ParseErrorSpecifics, ParseSettings};

#[derive(Debug, Clone)]
pub struct ParseState<'a> {
    partial_string: &'a str,
    start_index: usize,
    farthest_error: Option<ParseError>,
}

impl<'a> ParseState<'a> {
    #[inline]
    pub fn new(s: &'a str, _settings: &ParseSettings) -> ParseState<'a> {
        Self {
            partial_string: s,
            start_index: 0,
            farthest_error: None,
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

    /// Advance the parsing pointer n chars. Panics if length indexes into a character
    #[inline]
    pub fn advance_safe(self, length: usize) -> Self {
        if length > self.partial_string.len() {
            // This should be optimized out in most cases
            panic!("String length overrun in advance()")
        };
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
    pub fn range_until(&self, other: &ParseState) -> std::ops::Range<usize> {
        self.start_index..other.start_index
    }

    #[inline]
    pub fn cache_key(&self) -> usize {
        self.start_index
    }

    #[inline]
    pub fn report_error(self, specifics: ParseErrorSpecifics) -> ParseError {
        let position = self.start_index;
        self.record_error(ParseError {
            position,
            specifics,
        })
        .report_farthest_error()
    }

    #[inline]
    pub fn record_error(mut self, error: ParseError) -> Self {
        match &mut self.farthest_error {
            Some(farthest_error) => {
                if farthest_error.position <= error.position {
                    *farthest_error = error
                }
            }
            None => self.farthest_error = Some(error),
        }
        self
    }

    #[inline]
    pub fn report_farthest_error(self) -> ParseError {
        self.farthest_error.unwrap_or(ParseError {
            position: self.start_index,
            specifics: ParseErrorSpecifics::Other,
        })
    }

    pub fn first_n_chars(&self, n: usize) -> String {
        self.s().chars().take(n).collect()
    }

    #[inline]
    pub fn is_further_than(&self, other: &Self) -> bool {
        self.start_index > other.start_index
    }
}
