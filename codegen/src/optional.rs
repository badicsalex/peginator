// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

use anyhow::Result;
use proc_macro2::TokenStream;
use quote::quote;

use super::common::{
    generate_inner_parse_function, safe_ident, Arity, CloneState, Codegen, CodegenSettings,
    FieldDescriptor,
};
use crate::grammar::{Grammar, Optional};

impl Codegen for Optional {
    fn generate_code_spec(
        &self,
        rule_fields: &[FieldDescriptor],
        grammar: &Grammar,
        settings: &CodegenSettings,
    ) -> Result<TokenStream> {
        let postprocess = self.generate_postprocess_calls(rule_fields, grammar)?;
        let body;
        let parse_call;
        if let Some(inline_body) =
            self.body
                .generate_inline_body(rule_fields, grammar, settings, CloneState::Yes)?
        {
            body = TokenStream::new();
            parse_call = inline_body;
        } else {
            let inner_body = self.body.generate_code(rule_fields, grammar, settings)?;
            body = quote!(mod optional{
                use super::*;
                #inner_body
            });
            parse_call = quote!(optional::parse(state.clone(), global));
        };
        let parse_body = quote!(#parse_call #postprocess);
        let parse_function = generate_inner_parse_function(parse_body, settings);
        Ok(quote!(
            #body
            #parse_function
        ))
    }

    fn generate_inline_body(
        &self,
        rule_fields: &[FieldDescriptor],
        grammar: &Grammar,
        settings: &CodegenSettings,
        clone_state: CloneState,
    ) -> Result<Option<TokenStream>> {
        if clone_state == CloneState::Yes {
            Ok(None)
        } else if let Some(inline_body) =
            self.body
                .generate_inline_body(rule_fields, grammar, settings, CloneState::Yes)?
        {
            let postprocess = self.generate_postprocess_calls(rule_fields, grammar)?;
            Ok(Some(quote!(#inline_body #postprocess)))
        } else {
            Ok(None)
        }
    }

    fn get_fields<'a>(&'a self, grammar: &'a Grammar) -> Result<Vec<FieldDescriptor<'a>>> {
        Ok(set_arity_to_optional(self.body.get_fields(grammar)?))
    }
}

impl Optional {
    fn generate_postprocess_calls(
        &self,
        rule_fields: &[FieldDescriptor],
        grammar: &Grammar,
    ) -> Result<TokenStream> {
        let fields = self.body.get_filtered_rule_fields(rule_fields, grammar)?;
        let result = if fields.is_empty() {
            quote!(
                .or_else(|err|
                    Ok(ParseOk{
                        result: (),
                        state: state.record_error(err),
                    })
                )
            )
        } else if fields.len() == 1 {
            quote!(
                .or_else(|err|
                    Ok(ParseOk{
                        result: Default::default(),
                        state: state.record_error(err),
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
                .map_inner(|result| Parsed{#happy_case_fields})
                .or_else(|err|
                    Ok(ParseOk{
                        result: Parsed{#unhappy_case_fields},
                        state: state.record_error(err),
                    })
                )
            )
        };
        Ok(result)
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
