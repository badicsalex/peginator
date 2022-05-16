use std::collections::HashSet;

use anyhow::Result;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

use crate::grammar::Sequence;

use super::common::{Arity, Codegen, CodegenSettings, FieldDescriptor};

impl Codegen for Sequence {
    fn generate_code_spec(
        &self,
        rule_fields: &[FieldDescriptor],
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
                    Ok(ParseOk {
                        result: Parsed,
                        state,
                        farthest_error: None,
                    })
                }
            ));
        }
        if self.parts.len() < 2 {
            return self.parts[0].generate_code_spec(rule_fields, settings);
        }
        let part_bodies = self
            .parts
            .iter()
            .enumerate()
            .map(|(num, part)| -> Result<TokenStream> {
                let part_mod = format_ident!("part_{}", num);
                let part_body = part.generate_code(rule_fields, settings)?;
                Ok(quote!(
                    mod #part_mod{
                        use super::*;
                        #part_body
                    }
                ))
            })
            .collect::<Result<TokenStream>>()?;
        let parse_function = self.generate_parse_function(rule_fields, settings)?;
        Ok(quote!(
            #part_bodies
            #parse_function
        ))
    }

    fn get_fields(&self) -> Result<Vec<FieldDescriptor>> {
        let mut all_fields = Vec::<FieldDescriptor>::new();
        for part in &self.parts {
            let new_fields = part.get_fields()?;
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
        _settings: &CodegenSettings,
    ) -> Result<TokenStream> {
        let fields = self.get_filtered_rule_fields(rule_fields)?;
        let mut calls = TokenStream::new();
        let mut fields_seen = HashSet::<&str>::new();
        for (num, part) in self.parts.iter().enumerate() {
            let part_mod = format_ident!("part_{}", num);
            let inner_fields = part.get_filtered_rule_fields(rule_fields)?;
            let call = if inner_fields.is_empty() {
                quote!(
                    match #part_mod::parse(state, tracer.clone(), cache) {
                        Ok(ParseOk{
                            result:_,
                            state:new_state,
                            farthest_error:new_farthest_error
                        }) => {
                            farthest_error = combine_errors(farthest_error, new_farthest_error);
                            state = new_state;
                        },
                        Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
                    }
                )
            } else {
                let mut field_assignments = TokenStream::new();
                for field in &inner_fields {
                    let name = format_ident!("{}", field.name);
                    let field_assignment = if !fields_seen.contains(field.name) {
                        fields_seen.insert(field.name);
                        if field.arity == Arity::Multiple {
                            quote!(let mut #name = result.#name;)
                        } else {
                            quote!(let #name = result.#name;)
                        }
                    } else {
                        assert_eq!(field.arity, Arity::Multiple);
                        quote!(#name.extend(result.#name);)
                    };
                    field_assignments.extend(field_assignment);
                }
                quote!(
                    let result = match #part_mod::parse(state, tracer.clone(), cache) {
                        Ok(ParseOk{
                            result,
                            state:new_state,
                            farthest_error:new_farthest_error
                        }) => {
                            farthest_error = combine_errors(farthest_error, new_farthest_error);
                            state = new_state;
                            result
                        }
                        Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
                    };
                    #field_assignments
                )
            };
            calls.extend(call);
        }
        let field_names: Vec<Ident> = fields.iter().map(|f| format_ident!("{}", f.name)).collect();
        let parse_result = quote!(Parsed{ #( #field_names,)* });
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
                Ok(ParseOk{result:#parse_result, state, farthest_error})
            }
        ))
    }
}
