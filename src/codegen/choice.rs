// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

use anyhow::Result;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use super::common::{safe_ident, Arity, CloneState, Codegen, CodegenSettings, FieldDescriptor};
use crate::grammar::{Choice, Grammar};

impl Codegen for Choice {
    fn generate_code_spec(
        &self,
        rule_fields: &[FieldDescriptor],
        grammar: &Grammar,
        settings: &CodegenSettings,
    ) -> Result<TokenStream> {
        if self.choices.len() < 2 {
            return self.choices[0].generate_code_spec(rule_fields, grammar, settings);
        }
        let choice_bodies = self
            .choices
            .iter()
            .enumerate()
            .filter(|(_, choice)| {
                choice
                    .generate_inline_body(rule_fields, grammar, settings, CloneState::Yes)
                    .ok()
                    .flatten()
                    .is_none()
            })
            .map(|(num, choice)| -> Result<TokenStream> {
                let choice_mod = format_ident!("choice_{}", num);
                let sequence_body = choice.generate_code(rule_fields, grammar, settings)?;
                Ok(quote!(
                    mod #choice_mod{
                    use super::*;
                        #sequence_body
                    }
                ))
            })
            .collect::<Result<TokenStream>>()?;
        let parse_body =
            self.generate_parse_body(rule_fields, grammar, settings, CloneState::No)?;
        Ok(quote!(
            #choice_bodies
            #[inline(always)]
            pub fn parse<'a>(
                mut state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>
            ) -> ParseResult<'a, Parsed> {
                #parse_body
            }
        ))
    }

    fn generate_inline_body(
        &self,
        rule_fields: &[FieldDescriptor],
        grammar: &Grammar,
        settings: &CodegenSettings,
        clone_state: CloneState,
    ) -> Result<Option<TokenStream>> {
        if self.choices.len() < 2 {
            self.choices[0].generate_inline_body(rule_fields, grammar, settings, clone_state)
        } else if self.choices.iter().all(|c| {
            c.generate_inline_body(rule_fields, grammar, settings, CloneState::No)
                .ok()
                .flatten()
                .is_some()
        }) && self.get_filtered_rule_fields(rule_fields, grammar)?.len() <= 1
        {
            Ok(Some(self.generate_parse_body(
                rule_fields,
                grammar,
                settings,
                clone_state,
            )?))
        } else {
            Ok(None)
        }
    }

    fn get_fields<'a>(&'a self, grammar: &'a Grammar) -> Result<Vec<FieldDescriptor<'a>>> {
        let mut all_fields = Vec::<FieldDescriptor>::new();
        let mut first_iteration = true;
        for choice in &self.choices {
            let new_fields = choice.get_fields(grammar)?;

            if !first_iteration {
                for field in &mut all_fields {
                    if field.arity == Arity::One && !new_fields.iter().any(|f| f.name == field.name)
                    {
                        field.arity = Arity::Optional;
                    }
                }
            }

            for new_field in new_fields {
                if let Some(original) = all_fields.iter_mut().find(|f| f.name == new_field.name) {
                    original.arity = combine_arities_for_choice(&original.arity, &new_field.arity);
                    original.type_names.extend(&new_field.type_names);
                } else if first_iteration || new_field.arity != Arity::One {
                    all_fields.push(new_field);
                } else {
                    all_fields.push(FieldDescriptor {
                        arity: Arity::Optional,
                        ..new_field
                    });
                }
            }

            first_iteration = false;
        }
        Ok(all_fields)
    }
}

impl Choice {
    fn generate_parse_body(
        &self,
        rule_fields: &[FieldDescriptor],
        grammar: &Grammar,
        settings: &CodegenSettings,
        clone_state: CloneState,
    ) -> Result<TokenStream> {
        let fields = self.get_filtered_rule_fields(rule_fields, grammar)?;
        let calls = self
            .choices
            .iter()
            .enumerate()
            .map(|(num, choice)| {
                let parse_call = if let Some(inline_body) = choice
                    .generate_inline_body(rule_fields, grammar, settings, CloneState::No)
                    .unwrap()
                {
                    inline_body
                } else {
                    let choice_mod = format_ident!("choice_{}", num);
                    quote!(#choice_mod::parse(state, tracer, cache))
                };
                let inner_fields = choice.get_fields(grammar).unwrap();
                let postprocess = Self::generate_result_converter(&fields, &inner_fields);
                quote!(
                    .choice(|state| #parse_call #postprocess)
                )
            })
            .collect::<TokenStream>();
        let state = match clone_state {
            CloneState::No => quote!(state),
            CloneState::Yes => quote!(state.clone()),
        };
        Ok(quote!(
            ChoiceHelper::new(#state)
                #calls
                .end()
        ))
    }

    fn generate_result_converter(
        fields: &[FieldDescriptor],
        inner_fields: &[FieldDescriptor],
    ) -> TokenStream {
        if fields.is_empty() {
            TokenStream::new()
        } else if fields.len() == 1 {
            if inner_fields.is_empty() {
                let default = Self::generate_default_field(&fields[0]);
                quote!(.map_inner(|_| #default))
            } else {
                TokenStream::new()
            }
        } else {
            let field_assignments: TokenStream = fields
                .iter()
                .map(|field| {
                    let name = safe_ident(field.name);
                    let inner_exists = inner_fields
                        .iter()
                        .any(|inner_field| inner_field.name == field.name);
                    let value = if inner_exists {
                        if inner_fields.len() == 1 {
                            quote!(r)
                        } else {
                            quote!(r.#name)
                        }
                    } else {
                        Self::generate_default_field(field)
                    };
                    quote!(#name: #value,)
                })
                .collect();
            quote!(.map_inner(|r| Parsed{ #field_assignments }))
        }
    }

    fn generate_default_field(field: &FieldDescriptor) -> TokenStream {
        match field.arity {
            Arity::One => {
                panic!(
                    "Outer field ({:?}) cannot be One if inner does not exist",
                    field
                )
            }
            Arity::Optional => quote!(None),
            Arity::Multiple => quote!(Vec::new()),
        }
    }
}

fn combine_arities_for_choice(left: &Arity, right: &Arity) -> Arity {
    match (left, right) {
        (Arity::One, Arity::One) => Arity::One,
        (Arity::One, Arity::Optional) => Arity::Optional,
        (Arity::One, Arity::Multiple) => Arity::Multiple,
        (Arity::Optional, Arity::One) => Arity::Optional,
        (Arity::Optional, Arity::Optional) => Arity::Optional,
        (Arity::Optional, Arity::Multiple) => Arity::Multiple,
        (Arity::Multiple, Arity::One) => Arity::Multiple,
        (Arity::Multiple, Arity::Optional) => Arity::Multiple,
        (Arity::Multiple, Arity::Multiple) => Arity::Multiple,
    }
}
