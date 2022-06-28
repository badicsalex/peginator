// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

use anyhow::{bail, Result};
use proc_macro2::TokenStream;
use quote::quote;

use crate::grammar::{NegativeLookahead, PositiveLookahead};

use super::common::{Codegen, CodegenSettings, FieldDescriptor};

impl Codegen for NegativeLookahead {
    fn generate_code_spec(
        &self,
        rule_fields: &[FieldDescriptor],
        settings: &CodegenSettings,
    ) -> Result<TokenStream> {
        let body = self.expr.generate_code(rule_fields, settings)?;
        Ok(quote!(
            mod negative_lookahead{
                use super::*;
                #body
            }
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>
            ) -> ParseResult<'a, Parsed> {
                match negative_lookahead::parse (state.clone(), tracer, cache) {
                    Ok(_) => Err(state.report_error(ParseErrorSpecifics::NegativeLookaheadFailed)),
                    Err(_) => Ok(ParseOk{result:Parsed, state, farthest_error:None}),
                }
            }
        ))
    }

    fn get_fields(&self) -> Result<Vec<FieldDescriptor>> {
        if !self.expr.get_fields()?.is_empty() {
            bail!("The body of negative lookaheads should not contain named fields")
        }
        Ok(Vec::new())
    }
}

impl Codegen for PositiveLookahead {
    fn generate_code_spec(
        &self,
        rule_fields: &[FieldDescriptor],
        settings: &CodegenSettings,
    ) -> Result<TokenStream> {
        let body = self.expr.generate_code(rule_fields, settings)?;
        Ok(quote!(
            mod positive_lookahead{
                use super::*;
                #body
            }
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>
            ) -> ParseResult<'a, Parsed> {
                positive_lookahead::parse (state.clone(), tracer, cache)?;
                Ok(ParseOk{result:Parsed, state, farthest_error:None})
            }
        ))
    }

    fn get_fields(&self) -> Result<Vec<FieldDescriptor>> {
        if !self.expr.get_fields()?.is_empty() {
            bail!("The body of positive lookaheads should not contain named fields")
        }
        Ok(Vec::new())
    }
}
