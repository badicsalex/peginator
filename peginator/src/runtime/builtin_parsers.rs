// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

use super::{ParseErrorSpecifics, ParseOk, ParseResult, ParseState};

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
    let state = unsafe { state.advance(result.len_utf8()) };
    Ok(ParseOk {
        result,
        state,
        farthest_error: None,
    })
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
        let state = unsafe { state.advance(s.len()) };
        Ok(ParseOk {
            result: (),
            state,
            farthest_error: None,
        })
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
            let state = unsafe { state.advance(1) };
            Ok(ParseOk {
                result: (),
                state,
                farthest_error: None,
            })
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
        let state = unsafe { state.advance(c.len_utf8()) };
        Ok(ParseOk {
            result: (),
            state,
            farthest_error: None,
        })
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
            let state = unsafe { state.advance(1) };
            Ok(ParseOk {
                result: (),
                state,
                farthest_error: None,
            })
        }
    } else {
        // utf-8 path
        let c = state.s().chars().next().ok_or_else(|| {
            state
                .clone()
                .report_error(ParseErrorSpecifics::ExpectedCharacterRange { from, to })
        })?;
        if c < from || c > to {
            Err(state.report_error(ParseErrorSpecifics::ExpectedCharacterRange { from, to }))
        } else {
            // SAFETY:
            // Callers of this function are responsible that these preconditions are satisfied:
            //    Indexes must lie on UTF-8 sequence boundaries.
            //
            // We are skipping a full character, so we should be OK.
            let state = unsafe { state.advance(c.len_utf8()) };
            Ok(ParseOk {
                result: (),
                state,
                farthest_error: None,
            })
        }
    }
}
