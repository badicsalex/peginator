use anyhow::Result;
use proc_macro2::TokenStream;
use quote::quote;

use crate::grammar::{CharacterLiteral, CharacterRange, StringLiteral};

use super::common::{Codegen, CodegenSettings, FieldDescriptor};

impl Codegen for CharacterRange {
    fn generate_code_spec(
        &self,
        _rule_fields: &[FieldDescriptor],
        settings: &CodegenSettings,
    ) -> Result<TokenStream> {
        let from = &self.from;
        let to = &self.to;
        let skip_ws = if settings.skip_whitespace {
            quote!(let state = state.skip_whitespace();)
        } else {
            quote!()
        };
        Ok(quote!(
            #[inline(always)]
            pub fn parse<'a>(state: ParseState<'a>, cache: &mut ParseCache<'a>) -> ParseResult<'a, Parsed> {
                #skip_ws
                let ok_result = parse_character_range(state, #from, #to)?;
                Ok(ok_result.map(|_| Parsed))
            }
        ))
    }

    fn get_fields(&self) -> Result<Vec<FieldDescriptor>> {
        Ok(Vec::new())
    }
}

impl Codegen for CharacterLiteral {
    fn generate_code_spec(
        &self,
        _rule_fields: &[FieldDescriptor],
        settings: &CodegenSettings,
    ) -> Result<TokenStream> {
        let literal = &self;
        let skip_ws = if settings.skip_whitespace {
            quote!(let state = state.skip_whitespace();)
        } else {
            quote!()
        };
        Ok(quote!(
            #[inline(always)]
            pub fn parse<'a>(state: ParseState<'a>, cache: &mut ParseCache<'a>) -> ParseResult<'a, Parsed> {
                #skip_ws
                let ok_result = parse_character_literal(state, #literal)?;
                Ok(ok_result.map(|_| Parsed))
            }
        ))
    }

    fn get_fields(&self) -> Result<Vec<FieldDescriptor>> {
        Ok(Vec::new())
    }
}

impl Codegen for StringLiteral {
    fn generate_code_spec(
        &self,
        _rule_fields: &[FieldDescriptor],
        settings: &CodegenSettings,
    ) -> Result<TokenStream> {
        let literal = &self.body;
        let skip_ws = if settings.skip_whitespace {
            quote!(let state = state.skip_whitespace();)
        } else {
            quote!()
        };
        Ok(quote!(
            #[inline(always)]
            pub fn parse<'a>(state: ParseState<'a>, cache: &mut ParseCache<'a>) -> ParseResult<'a, Parsed> {
                #skip_ws
                let ok_result = parse_string_literal(state, #literal)?;
                Ok(ok_result.map(|_| Parsed))
            }
        ))
    }

    fn get_fields(&self) -> Result<Vec<FieldDescriptor>> {
        Ok(Vec::new())
    }
}
