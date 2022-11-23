// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

use std::collections::HashSet;

use anyhow::Result;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

use super::common::{
    generate_inner_parse_function, safe_ident, Arity, CloneState, Codegen, CodegenSettings,
    FieldDescriptor,
};
use crate::grammar::{Grammar, Sequence};

impl Codegen for Sequence {
    fn generate_code_spec(
        &self,
        rule_fields: &[FieldDescriptor],
        grammar: &Grammar,
        settings: &CodegenSettings,
    ) -> Result<TokenStream> {
        if self.parts.is_empty() {
            return Ok(generate_inner_parse_function(
                quote!(Ok(ParseOk { result: (), state })),
                settings,
            ));
        }
        if self.parts.len() < 2 {
            return self.parts[0].generate_code_spec(rule_fields, grammar, settings);
        }
        let part_bodies = self
            .parts
            .iter()
            .enumerate()
            .filter(|(_, part)| {
                part.generate_inline_body(rule_fields, grammar, settings, CloneState::No)
                    .ok()
                    .flatten()
                    .is_none()
            })
            .map(|(num, part)| -> Result<TokenStream> {
                let part_mod = format_ident!("part_{num}");
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

    fn generate_inline_body(
        &self,
        rule_fields: &[FieldDescriptor],
        grammar: &Grammar,
        settings: &CodegenSettings,
        clone_state: CloneState,
    ) -> Result<Option<TokenStream>> {
        if self.parts.is_empty() {
            let state = match clone_state {
                CloneState::No => quote!(state),
                CloneState::Yes => quote!(state: state.clone()),
            };
            Ok(Some(quote!(Ok(ParseOk { result: (), #state }))))
        } else if self.parts.len() < 2 {
            self.parts[0].generate_inline_body(rule_fields, grammar, settings, clone_state)
        } else {
            Ok(None)
        }
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
        settings: &CodegenSettings,
    ) -> Result<TokenStream> {
        let fields = self.get_filtered_rule_fields(rule_fields, grammar)?;
        let mut calls = TokenStream::new();
        let mut fields_seen = HashSet::<&str>::new();
        for (num, part) in self.parts.iter().enumerate() {
            let inner_fields = part.get_filtered_rule_fields(rule_fields, grammar)?;
            let part_mod = format_ident!("part_{num}");
            let parse_call = if let Some(inline_body) =
                part.generate_inline_body(rule_fields, grammar, settings, CloneState::No)?
            {
                inline_body
            } else {
                quote!(#part_mod::parse(state, global))
            };
            let call = if inner_fields.is_empty() {
                quote!(
                    let ParseOk{state, ..} = #parse_call?;
                )
            } else {
                let mut field_assignments = TokenStream::new();
                let mut extend_calls = TokenStream::new();
                if inner_fields.len() == 1 {
                    let field = &inner_fields[0];
                    let field_name = &field.name;
                    let name = safe_ident(field_name);
                    if !fields_seen.contains(field_name) {
                        fields_seen.insert(field_name);
                        if field.arity == Arity::Multiple {
                            field_assignments.extend(quote!(mut #name))
                        } else {
                            field_assignments.extend(quote!(#name))
                        }
                    } else {
                        assert_eq!(field.arity, Arity::Multiple);
                        let extend_name = format_ident!("extend_{field_name}_with");
                        field_assignments.extend(quote!(#extend_name));
                        extend_calls.extend(quote!(#name.extend(#extend_name);));
                    }
                } else {
                    for field in &inner_fields {
                        let field_name = &field.name;
                        let name = safe_ident(field_name);
                        if !fields_seen.contains(field_name) {
                            fields_seen.insert(field_name);
                            if field.arity == Arity::Multiple {
                                field_assignments.extend(quote!(mut #name,))
                            } else {
                                field_assignments.extend(quote!(#name,))
                            }
                        } else {
                            assert_eq!(field.arity, Arity::Multiple);
                            let extend_name = format_ident!("extend_{field_name}_with");
                            field_assignments.extend(quote!(#name: #extend_name,));
                            extend_calls.extend(quote!(#name.extend(#extend_name);));
                        }
                    }
                    field_assignments = quote!( #part_mod::Parsed { #field_assignments } )
                }
                quote!(
                    let ParseOk{result: #field_assignments, state} = #parse_call?;
                    #extend_calls
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
        let parse_body = quote!(
            #calls
            Ok(ParseOk{result:#parse_result, state})
        );
        Ok(generate_inner_parse_function(parse_body, settings))
    }
}
