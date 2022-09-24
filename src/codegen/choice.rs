// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

use anyhow::Result;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use super::common::{Arity, Codegen, CodegenSettings, FieldDescriptor};
use crate::{codegen::utils::safe_ident, grammar::Choice};

impl Codegen for Choice {
    fn generate_code_spec(
        &self,
        rule_fields: &[FieldDescriptor],
        settings: &CodegenSettings,
    ) -> Result<TokenStream> {
        if self.choices.len() < 2 {
            return self.choices[0].generate_code_spec(rule_fields, settings);
        }
        let choice_bodies = self
            .choices
            .iter()
            .enumerate()
            .map(|(num, choice)| -> Result<TokenStream> {
                let choice_mod = format_ident!("choice_{}", num);
                let sequence_body = choice.generate_code(rule_fields, settings)?;
                Ok(quote!(
                    mod #choice_mod{
                    use super::*;
                        #sequence_body
                    }
                ))
            })
            .collect::<Result<TokenStream>>()?;
        let parse_function = self.generate_parse_function(rule_fields, settings)?;
        Ok(quote!(
            #choice_bodies
            #parse_function
        ))
    }

    fn get_fields(&self) -> Result<Vec<FieldDescriptor>> {
        let mut all_fields = Vec::<FieldDescriptor>::new();
        let mut first_iteration = true;
        for choice in &self.choices {
            let new_fields = choice.get_fields()?;

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
    fn generate_parse_function(
        &self,
        rule_fields: &[FieldDescriptor],
        _settings: &CodegenSettings,
    ) -> Result<TokenStream> {
        let fields = self.get_filtered_rule_fields(rule_fields)?;
        let calls = self
            .choices
            .iter()
            .enumerate()
            .map(|(num, choice)| {
                let inner_fields = choice.get_fields().unwrap();
                let choice_mod = format_ident!("choice_{}", num);
                if fields.is_empty() {
                    Ok(quote!(
                        match #choice_mod::parse(state.clone(), tracer, cache) {
                            Ok(ok_result) => return Ok(ok_result.map(|result| Parsed)),
                            Err(err) => farthest_error = combine_errors(farthest_error, Some(err)),
                        }
                    ))
                } else {
                    let field_assignments: TokenStream = fields
                        .iter()
                        .map(|field| {
                            let name = safe_ident(field.name);
                            let inner_exists = inner_fields
                                .iter()
                                .any(|inner_field| inner_field.name == field.name);
                            let value = if inner_exists {
                                quote!(result.#name)
                            } else {
                                match field.arity {
                                    Arity::One => {
                                        panic!("Outer field ({:?}) cannot be One if inner does not exist", field)
                                    }
                                    Arity::Optional => quote!(None),
                                    Arity::Multiple => quote!(Vec::new()),
                                }
                            };
                            quote!(#name: #value,)
                        })
                        .collect();
                    Ok(quote!(
                        match #choice_mod::parse(state.clone(), tracer, cache) {
                            Ok(ok_result) => return Ok(
                                    ok_result.map(|result|
                                        Parsed{
                                            #field_assignments
                                        }
                                    )
                                ),
                            Err(err) => farthest_error = combine_errors(farthest_error, Some(err)),
                        }
                    ))
                }
            })
            .collect::<Result<TokenStream>>()?;
        Ok(quote!(
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>
            ) -> ParseResult<'a, Parsed> {
                let mut state = state;
                let mut farthest_error: Option<ParseError> = None;
                #calls
                Err(farthest_error.unwrap_or_else(|| state.report_error(ParseErrorSpecifics::Other)))
            }
        ))
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
