// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

use anyhow::Result;
use proc_macro2::TokenStream;
use quote::quote;

use super::common::{Codegen, CodegenSettings, FieldDescriptor};
use crate::grammar::{EndOfInput, Grammar};

impl Codegen for EndOfInput {
    fn generate_code_spec(
        &self,
        _rule_fields: &[FieldDescriptor],
        _grammar: &Grammar,
        settings: &CodegenSettings,
    ) -> Result<TokenStream> {
        let skip_ws = if settings.skip_whitespace {
            quote!(let ParseOk{state, ..} = parse_Whitespace(state, tracer, cache)?;)
        } else {
            quote!()
        };
        Ok(quote!(
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>
            ) -> ParseResult<'a, Parsed> {
                #skip_ws
                if state.is_empty() {
                    Ok(ParseOk{result:Parsed, state, farthest_error:None})
                } else {
                    Err(state.report_error(ParseErrorSpecifics::ExpectedEoi))
                }
            }
        ))
    }

    fn get_fields(&self, _grammar: &Grammar) -> Result<Vec<FieldDescriptor>> {
        Ok(Vec::new())
    }
}
