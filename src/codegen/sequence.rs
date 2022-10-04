// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

use std::collections::HashSet;

use anyhow::Result;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

use super::common::{safe_ident, Arity, Codegen, CodegenSettings, FieldDescriptor};
use crate::grammar::{Grammar, Sequence};

impl Codegen for Sequence {
    fn generate_code_spec(
        &self,
        rule_fields: &[FieldDescriptor],
        grammar: &Grammar,
        settings: &CodegenSettings,
    ) -> Result<TokenStream> {
        if self.parts.is_empty() {
            return Ok(quote!(
                #[inline(always)]
                pub fn parse<'a>(
                    state: ParseState<'a>,
                    tracer: impl ParseTracer,
                    cache: &mut ParseCache<'a>,
                ) -> ParseResult<'a, Parsed> {
                    Ok(ParseOk { result: (), state })
                }
            ));
        }
        if self.parts.len() < 2 {
            return self.parts[0].generate_code_spec(rule_fields, grammar, settings);
        }
        let part_bodies = self
            .parts
            .iter()
            .enumerate()
            .map(|(num, part)| -> Result<TokenStream> {
                let part_mod = format_ident!("part_{}", num);
                let part_body = part.generate_code(rule_fields, grammar, settings)?;
                Ok(quote!(
                    mod #part_mod{
                        use super::*;
                        #part_body
                    }
                ))
            })
            .collect::<Result<TokenStream>>()?;
        let parse_function = self.generate_parse_function(rule_fields, grammar, settings)?;
        Ok(quote!(
            #part_bodies
            #parse_function
        ))
    }

    fn get_fields<'a>(&'a self, grammar: &'a Grammar) -> Result<Vec<FieldDescriptor<'a>>> {
        let mut all_fields = Vec::<FieldDescriptor>::new();
        for part in &self.parts {
            let new_fields = part.get_fields(grammar)?;
            for new_field in new_fields {
                if let Some(original) = all_fields.iter_mut().find(|f| f.name == new_field.name) {
                    original.arity = Arity::Multiple;
                    original.type_names.extend(&new_field.type_names);
                } else {
                    all_fields.push(new_field);
                }
            }
        }
        Ok(all_fields)
    }
}

impl Sequence {
    fn generate_parse_function(
        &self,
        rule_fields: &[FieldDescriptor],
        grammar: &Grammar,
        _settings: &CodegenSettings,
    ) -> Result<TokenStream> {
        let fields = self.get_filtered_rule_fields(rule_fields, grammar)?;
        let mut calls = TokenStream::new();
        let mut fields_seen = HashSet::<&str>::new();
        for (num, part) in self.parts.iter().enumerate() {
            let part_mod = format_ident!("part_{}", num);
            let inner_fields = part.get_filtered_rule_fields(rule_fields, grammar)?;
            let call = if inner_fields.is_empty() {
                quote!(
                    let ParseOk{state, ..} = #part_mod::parse(state, tracer, cache)?;
                )
            } else {
                let mut field_assignments = TokenStream::new();
                for field in &inner_fields {
                    let name = safe_ident(field.name);
                    let source = if inner_fields.len() == 1 {
                        quote!(__result)
                    } else {
                        quote!(__result.#name)
                    };
                    let field_assignment = if !fields_seen.contains(field.name) {
                        fields_seen.insert(field.name);
                        if field.arity == Arity::Multiple {
                            quote!(let mut #name = #source;)
                        } else {
                            quote!(let #name = #source;)
                        }
                    } else {
                        assert_eq!(field.arity, Arity::Multiple);
                        quote!(#name.extend(#source);)
                    };
                    field_assignments.extend(field_assignment);
                }
                quote!(
                    let ParseOk{result:__result, state, ..} = #part_mod::parse(state, tracer, cache)?;
                    #field_assignments
                )
            };
            calls.extend(call);
        }
        let field_names: Vec<Ident> = fields.iter().map(|f| safe_ident(f.name)).collect();
        let parse_result = if field_names.is_empty() {
            quote!(())
        } else if field_names.len() == 1 {
            quote!(#( #field_names )* )
        } else {
            quote!(Parsed{ #( #field_names,)* })
        };
        Ok(quote!(
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>
            ) -> ParseResult<'a, Parsed> {
                #calls
                Ok(ParseOk{result:#parse_result, state})
            }
        ))
    }
}
