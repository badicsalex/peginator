// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

use anyhow::Result;
use proc_macro2::TokenStream;
use quote::quote;

use crate::grammar::EndOfInput;

use super::common::{Codegen, CodegenSettings, FieldDescriptor};

impl Codegen for EndOfInput {
    fn generate_code_spec(
        &self,
        _rule_fields: &[FieldDescriptor],
        settings: &CodegenSettings,
    ) -> Result<TokenStream> {
        let skip_ws = if settings.skip_whitespace {
            quote!(let state = state.skip_whitespace();)
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

    fn get_fields(&self) -> Result<Vec<FieldDescriptor>> {
        Ok(Vec::new())
    }
}
