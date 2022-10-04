// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

use anyhow::Result;
use proc_macro2::TokenStream;
use quote::quote;

use super::common::{safe_ident, Arity, Codegen, CodegenSettings, FieldDescriptor};
use crate::grammar::{Grammar, Optional};

impl Codegen for Optional {
    fn generate_code_spec(
        &self,
        rule_fields: &[FieldDescriptor],
        grammar: &Grammar,
        settings: &CodegenSettings,
    ) -> Result<TokenStream> {
        let fields = self.body.get_filtered_rule_fields(rule_fields, grammar)?;
        let postprocess = if fields.is_empty() {
            quote!(
                .or_else(|err|
                    Ok(ParseOk{
                        result: (),
                        state: old_state.record_error(err),
                    })
                )
            )
        } else if fields.len() == 1 {
            quote!(
                .or_else(|err|
                    Ok(ParseOk{
                        result: Default::default(),
                        state: old_state.record_error(err),
                    })
                )
            )
        } else {
            let happy_case_fields: TokenStream = fields
                .iter()
                .map(|field| {
                    let name = safe_ident(field.name);
                    quote!(#name: result.#name,)
                })
                .collect();
            let unhappy_case_fields: TokenStream = fields
                .iter()
                .map(|field| {
                    let name = safe_ident(field.name);
                    quote!(#name: Default::default(),)
                })
                .collect();
            quote!(
                .map(|ok_result| ok_result.map(|result| Parsed{#happy_case_fields}))
                .or_else(|err|
                    Ok(ParseOk{
                        result: Parsed{#unhappy_case_fields},
                        state: old_state.record_error(err),
                    })
                )
            )
        };
        let body;
        let parse_call;
        if let Some(inline_body) = self.body.generate_inline_body(rule_fields, settings)? {
            body = TokenStream::new();
            parse_call = inline_body;
        } else {
            body = self.body.generate_code(rule_fields, grammar, settings)?;
            parse_call = quote!(optional::parse(state, tracer, cache));
        };
        Ok(quote!(
            mod optional{
                use super::*;
                #body
            }
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>
            ) -> ParseResult<'a, Parsed> {
                let old_state = state.clone();
                #parse_call #postprocess
            }
        ))
    }

    fn get_fields<'a>(&'a self, grammar: &'a Grammar) -> Result<Vec<FieldDescriptor<'a>>> {
        Ok(set_arity_to_optional(self.body.get_fields(grammar)?))
    }
}

fn set_arity_to_optional(fields: Vec<FieldDescriptor>) -> Vec<FieldDescriptor> {
    let mut fields = fields;
    for value in &mut fields {
        value.arity = match value.arity {
            Arity::One => Arity::Optional,
            Arity::Optional => Arity::Optional,
            Arity::Multiple => Arity::Multiple,
        }
    }
    fields
}
