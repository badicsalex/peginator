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
        let body = self.body.generate_code(rule_fields, grammar, settings)?;
        let fields = self.body.get_filtered_rule_fields(rule_fields, grammar)?;
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
                match optional::parse(state.clone(), tracer, cache) {
                Ok(ok_result) => Ok(ok_result.map(|result| Parsed{#happy_case_fields})),
                Err(err) => Ok(ParseOk{
                        result: Parsed{#unhappy_case_fields},
                        state,
                        farthest_error:Some(err),
                    })
                }
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
