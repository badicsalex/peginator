// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

use anyhow::Result;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::grammar::{CharRule, CharRulePart};

impl CharRulePart {
    pub fn generate_parse_call(&self) -> Result<TokenStream> {
        match self {
            CharRulePart::CharRangePart(c) => {
                let char_literal: char = c.try_into()?;
                Ok(quote!(parse_character_literal(state.clone(), #char_literal)))
            }
            CharRulePart::CharacterRange(r) => {
                let from: char = (&r.from).try_into()?;
                let to: char = (&r.to).try_into()?;
                Ok(quote!(parse_character_range(state.clone(), #from, #to)))
            }
            CharRulePart::Identifier(ident) => {
                let parser_name = format_ident!("parse_{}", ident);
                Ok(quote!(#parser_name(state.clone(), tracer, cache)))
            }
        }
    }
}

impl CharRule {
    pub fn generate_code(&self) -> Result<TokenStream> {
        let name = &self.name;
        let rule_type = format_ident!("{}", self.name);
        let parser_name = format_ident!("parse_{}", self.name);
        let parser_calls = self
            .choices
            .iter()
            .map(|c| c.generate_parse_call())
            .collect::<Result<Vec<TokenStream>>>()?;

        Ok(quote!(
            #[inline]
            pub(super) fn #parser_name <'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>
            ) -> ParseResult<'a, #rule_type> {
                #(if let Ok(result) = #parser_calls { return Ok(result)})*
                Err(state.report_error(ParseErrorSpecifics::ExpectedCharacterClass { name: #name }))
            }
        ))
    }
}
