// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

use anyhow::{anyhow, bail, Result};
use proc_macro2::TokenStream;
use quote::quote;

use super::common::{generate_skip_ws, CloneState, Codegen, CodegenSettings, FieldDescriptor};
use crate::grammar::{
    CharacterRange, Grammar, HexaEscape, SimpleEscape, StringItem, StringLiteral, Utf8Escape,
};

impl TryFrom<&StringItem> for char {
    type Error = anyhow::Error;

    fn try_from(value: &StringItem) -> Result<Self, Self::Error> {
        match value {
            StringItem::HexaEscape(x) => Ok(x.into()),
            StringItem::SimpleEscape(x) => Ok(x.into()),
            StringItem::Utf8Escape(x) => x.try_into(),
            StringItem::char(x) => Ok(*x),
        }
    }
}

impl From<&HexaEscape> for char {
    fn from(value: &HexaEscape) -> Self {
        let c1i = value.c1.to_digit(16).unwrap() as u8;
        let c2i = value.c2.to_digit(16).unwrap() as u8;
        (c1i * 16 + c2i).into()
    }
}

impl From<&SimpleEscape> for char {
    fn from(value: &SimpleEscape) -> Self {
        match value {
            SimpleEscape::SimpleEscapeBackslash(_) => '\\',
            SimpleEscape::SimpleEscapeCarriageReturn(_) => '\r',
            SimpleEscape::SimpleEscapeDQuote(_) => '"',
            SimpleEscape::SimpleEscapeNewline(_) => '\n',
            SimpleEscape::SimpleEscapeQuote(_) => '\'',
            SimpleEscape::SimpleEscapeTab(_) => '\t',
        }
    }
}

impl TryFrom<&Utf8Escape> for char {
    type Error = anyhow::Error;

    fn try_from(value: &Utf8Escape) -> Result<Self, Self::Error> {
        let mut result: u32 = value.c1.to_digit(16).unwrap();
        if let Some(v) = &value.c2 {
            result = result * 16 + v.to_digit(16).unwrap();
        };
        if let Some(v) = &value.c3 {
            result = result * 16 + v.to_digit(16).unwrap();
        };
        if let Some(v) = &value.c4 {
            result = result * 16 + v.to_digit(16).unwrap();
        };
        if let Some(v) = &value.c5 {
            result = result * 16 + v.to_digit(16).unwrap();
        };
        if let Some(v) = &value.c6 {
            result = result * 16 + v.to_digit(16).unwrap();
        };
        char::from_u32(result).ok_or_else(|| anyhow!("Invalid utf-8 codepoint {:#x}", result))
    }
}

impl Codegen for CharacterRange {
    fn generate_inline_body(
        &self,
        _rule_fields: &[FieldDescriptor],
        settings: &CodegenSettings,
        clone_state: CloneState,
    ) -> Result<Option<TokenStream>> {
        let from: char = (&self.from).try_into()?;
        let to: char = (&self.to).try_into()?;
        let parse_call = generate_skip_ws(
            settings,
            "parse_character_range",
            quote!(#from, #to),
            clone_state,
        );
        Ok(Some(quote!(#parse_call .discard_result())))
    }

    fn get_fields(&self, _grammar: &Grammar) -> Result<Vec<FieldDescriptor>> {
        Ok(Vec::new())
    }
}

impl Codegen for StringLiteral {
    fn generate_inline_body(
        &self,
        _rule_fields: &[FieldDescriptor],
        settings: &CodegenSettings,
        clone_state: CloneState,
    ) -> Result<Option<TokenStream>> {
        let parser_name;
        let additional_params;
        let literal = &self
            .body
            .iter()
            .map(|item| -> Result<char> { item.try_into() })
            .collect::<Result<String>>()?;
        if self.insensitive.is_some() {
            if !literal.is_ascii() {
                bail!(
                    "Case insensitive matching only works for ascii strings. ({:?} was not ascii)",
                    literal
                );
            }
            let literal = literal.to_ascii_lowercase();
            if literal.chars().count() == 1 {
                let char_literal = literal.chars().next().unwrap();
                parser_name = "parse_character_literal_insensitive";
                additional_params = quote!(#char_literal);
            } else {
                parser_name = "parse_string_literal_insensitive";
                additional_params = quote!(#literal);
            }
        } else if literal.chars().count() == 1 {
            let char_literal = literal.chars().next().unwrap();
            parser_name = "parse_character_literal";
            additional_params = quote!(#char_literal);
        } else {
            parser_name = "parse_string_literal";
            additional_params = quote!(#literal);
        };
        let parse_call = generate_skip_ws(settings, parser_name, additional_params, clone_state);
        Ok(Some(quote!(#parse_call .discard_result())))
    }

    fn get_fields(&self, _grammar: &Grammar) -> Result<Vec<FieldDescriptor>> {
        Ok(Vec::new())
    }
}
