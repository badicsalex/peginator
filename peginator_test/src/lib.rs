// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

#![cfg(test)]
pub(crate) mod test_utils;

mod additional_traits;
mod char_rule;
pub mod check;
mod choice;
mod closure;
mod custom_derives_empty;
mod custom_whitespace;
mod enums;
mod eoi;
pub mod extern_directive;
mod field;
mod fndef_example;
mod include_rule;
mod lookahead;
mod memoization;
mod operator_example;
mod optional;
mod palindrome;
mod position;
mod precedence;
mod rust_keywords;
mod sequence;
mod simple;
mod skip_ws;
mod string;
