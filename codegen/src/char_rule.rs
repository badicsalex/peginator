// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

use anyhow::Result;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use super::{
    common::{generate_rule_parse_function, safe_ident},
    CodegenSettings,
};
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
                Ok(quote!(#parser_name(state.clone(), global)))
            }
        }
    }
}

impl CharRule {
    pub fn generate_code(&self, settings: &CodegenSettings) -> Result<TokenStream> {
        let name = &self.name;
        let rule_type = safe_ident(&self.name);
        let parser_name = format_ident!("parse_{}", self.name);
        let parser_calls = self
            .choices
            .iter()
            .map(|c| c.generate_parse_call())
            .collect::<Result<Vec<TokenStream>>>()?;
        let check_calls = self.generate_check_calls()?;

        let parse_body = quote!(
            #check_calls
            #(if let Ok(result) = #parser_calls { return Ok(result)})*
            Err(state.report_error(ParseErrorSpecifics::ExpectedCharacterClass { name: #name }))
        );
        Ok(generate_rule_parse_function(
            parser_name,
            rule_type,
            parse_body,
            settings,
        ))
    }

    fn generate_check_calls(&self) -> Result<TokenStream> {
        if self.directives.is_empty() {
            return Ok(TokenStream::new());
        }
        let name = &self.name;
        let check_idents = self.directives.iter().map(|d| {
            let part_idents = d.function.iter().map(safe_ident);
            quote!(#(#part_idents)::*)
        });

        Ok(quote!(
            if let Some(c) = state.s().chars().next() {
                #(
                    if !#check_idents(c) {
                        return Err(state.report_error(ParseErrorSpecifics::ExpectedCharacterClass { name: #name }))
                    }
                )*
            } else {
                return Err(state.report_error(ParseErrorSpecifics::ExpectedCharacterClass { name: #name }))
            }
        ))
    }
}
