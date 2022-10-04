// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

use anyhow::Result;
use proc_macro2::{Ident, TokenStream};
use quote::quote;

use super::common::{
    generate_field_type, safe_ident, Arity, CloneState, Codegen, CodegenSettings, FieldDescriptor,
};
use crate::grammar::{Closure, Grammar};

impl Codegen for Closure {
    fn generate_code_spec(
        &self,
        rule_fields: &[FieldDescriptor],
        grammar: &Grammar,
        settings: &CodegenSettings,
    ) -> Result<TokenStream> {
        let closure_body;
        let parse_call;
        if let Some(inline_body) =
            self.body
                .generate_inline_body(rule_fields, grammar, settings, CloneState::Yes)?
        {
            closure_body = TokenStream::new();
            parse_call = inline_body;
        } else {
            closure_body = self.body.generate_code(rule_fields, grammar, settings)?;
            parse_call = quote!(closure::parse(state.clone(), tracer, cache));
        };

        let fields = self.body.get_filtered_rule_fields(rule_fields, grammar)?;
        let declarations: TokenStream = fields
            .iter()
            .map(|f| {
                let typ = generate_field_type("Parsed", f, settings);
                let name_ident = safe_ident(f.name);
                quote!(let mut #name_ident: #typ = Vec::new();)
            })
            .collect();
        let assignments: TokenStream = fields
            .iter()
            .map(|field| {
                let name = safe_ident(field.name);
                if fields.len() == 1 {
                    quote!(#name.extend(__result);)
                } else {
                    quote!(#name.extend(__result.#name);)
                }
            })
            .collect();
        let field_names: Vec<Ident> = fields.iter().map(|f| safe_ident(f.name)).collect();
        let parse_result = if field_names.is_empty() {
            quote!(())
        } else if field_names.len() == 1 {
            quote!(#( #field_names)*)
        } else {
            quote!(Parsed{ #( #field_names,)* })
        };
        let at_least_one_body = if self.at_least_one.is_some() {
            quote!(
                let ParseOk{result:__result, mut state, ..} = #parse_call?;
                #assignments
            )
        } else {
            quote!(
                let mut state = state;
            )
        };

        Ok(quote!(
            mod closure{
                use super::*;
                #closure_body
            }
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>
            ) -> ParseResult<'a, Parsed> {
                #declarations
                #at_least_one_body
                loop {
                    match #parse_call {
                        Ok(ParseOk{result: __result, state:new_state, ..}) => {
                            #assignments
                            state = new_state;
                        },
                        Err(err) => {
                            state = state.record_error(err);
                            break;
                        }
                    }
                }
                Ok(ParseOk{result:#parse_result, state})
            }
        ))
    }

    fn get_fields<'a>(&'a self, grammar: &'a Grammar) -> Result<Vec<FieldDescriptor<'a>>> {
        Ok(set_arity_to_multiple(self.body.get_fields(grammar)?))
    }
}

fn set_arity_to_multiple(fields: Vec<FieldDescriptor>) -> Vec<FieldDescriptor> {
    let mut fields = fields;
    for value in &mut fields {
        value.arity = Arity::Multiple;
    }
    fields
}
