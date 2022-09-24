// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

use anyhow::{bail, Result};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

use super::common::{
    generate_enum_type, generate_field_type, Arity, Codegen, CodegenRule, CodegenSettings,
    FieldDescriptor, PublicType, RecordPosition,
};
use crate::{
    codegen::utils::safe_ident,
    grammar::{DirectiveExpression, Rule},
};

impl CodegenRule for Rule {
    fn generate_code(&self, settings: &CodegenSettings) -> Result<(TokenStream, TokenStream)> {
        let flags = self.flags();
        let settings = CodegenSettings {
            skip_whitespace: settings.skip_whitespace && !flags.no_skip_ws,
            ..settings.clone()
        };

        let fields = self.definition.get_fields()?;

        self.check_flags(&flags, &settings)?;

        let name = &self.name;
        let rule_mod = format_ident!("{}_impl", self.name);
        let rule_type = safe_ident(&self.name);
        let parser_name = format_ident!("parse_{}", self.name);
        let cache_entry_ident = format_ident!("c_{}", self.name);
        let choice_body = self.definition.generate_code(&fields, &settings)?;

        let (types, inner_code) = if flags.string {
            self.generate_string_rule(&settings)?
        } else if fields.len() == 1 && fields[0].name == "_override" {
            self.generate_override_rule(&fields, &settings)?
        } else {
            self.generate_normal_rule(&fields, &settings, flags.position.into())?
        };

        let rule_parser_call = if flags.memoize {
            quote!(
                let cache_key = state.cache_key();
                if let Some(cached) = cache.#cache_entry_ident.get(&cache_key) {
                    cached.clone()
                } else {
                    let result = #rule_mod::rule_parser(state, tracer, cache);
                    cache.#cache_entry_ident.insert(cache_key, result.clone());
                    result
                }
            )
        } else {
            quote!(#rule_mod::rule_parser(state, tracer, cache))
        };

        Ok((
            types,
            quote!(
                mod #rule_mod{
                    use super::*;
                    #choice_body
                    #inner_code
                }
                #[inline]
                pub(super) fn #parser_name <'a>(
                    state: ParseState<'a>,
                    tracer: impl ParseTracer,
                    cache: &mut ParseCache<'a>
                ) -> ParseResult<'a, #rule_type> {
                    tracer.run_traced(
                        #name, state,
                        |state, tracer| {
                            #rule_parser_call
                        },
                    )
                }
            ),
        ))
    }
}

#[derive(Debug, Default)]
pub struct RuleFlags {
    pub no_skip_ws: bool,
    pub export: bool,
    pub string: bool,
    pub position: bool,
    pub memoize: bool,
}

impl Rule {
    pub fn flags(&self) -> RuleFlags {
        let mut result = RuleFlags::default();
        for directive in &self.directives {
            match directive {
                DirectiveExpression::StringDirective(_) => result.string = true,
                DirectiveExpression::NoSkipWsDirective(_) => result.no_skip_ws = true,
                DirectiveExpression::ExportDirective(_) => result.export = true,
                DirectiveExpression::PositionDirective(_) => result.position = true,
                DirectiveExpression::MemoizeDirective(_) => result.memoize = true,
                DirectiveExpression::CheckDirective(_) => (),
            }
        }
        result
    }

    fn check_flags(&self, flags: &RuleFlags, settings: &CodegenSettings) -> Result<()> {
        if flags.export && flags.string {
            bail!("@string rules cannot be @export-ed");
        }
        if flags.position && flags.string {
            bail!("@string rules cannot contain @position");
        }
        if self.name == "Whitespace" && !flags.no_skip_ws {
            bail!("The 'Whitespace' rule (and all called rules) must be @no_skip_ws to prevent recursion");
        }
        if flags.memoize && !settings.derives.contains(&"Clone".into()) {
            bail!("@memoize can only be used if 'Clone' is in the derives set");
        }
        Ok(())
    }

    fn generate_string_rule(
        &self,
        _settings: &CodegenSettings,
    ) -> Result<(TokenStream, TokenStream)> {
        let rule_type = safe_ident(&self.name);
        let check_calls = self.generate_check_calls()?;
        Ok((
            quote!(pub type #rule_type = String;),
            quote!(
                #[inline(always)]
                pub fn rule_parser<'a>(
                    state: ParseState<'a>,
                    tracer: impl ParseTracer,
                    cache: &mut ParseCache<'a>,
                ) -> ParseResult<'a, String> {
                    let result =
                        parse(state.clone(), tracer, cache)?
                        .map_with_state(
                            |_, new_state|
                            state.slice_until(new_state).to_string()
                        );
                    #check_calls
                    Ok(result)
                }
            ),
        ))
    }
    fn generate_override_rule(
        &self,
        fields: &[FieldDescriptor],
        settings: &CodegenSettings,
    ) -> Result<(TokenStream, TokenStream)> {
        let field = &fields[0];
        let flags = self.flags();
        if flags.export {
            bail!("Overridden (containing '@:') rules cannot be @export-ed");
        }
        if flags.position {
            bail!("Overridden (containing '@:') rules cannot contain @position");
        }
        if field.type_names.len() <= 1 {
            self.generate_override_rule_simple(field, settings)
        } else {
            if field.arity != Arity::One {
                bail!("Enum '@:' fields have to be used exactly once in all choice branches, and must not be used in closures or optional parts.");
            }
            self.generate_override_rule_enum(field, settings)
        }
    }

    fn generate_override_rule_simple(
        &self,
        field: &FieldDescriptor,
        settings: &CodegenSettings,
    ) -> Result<(TokenStream, TokenStream)> {
        let rule_type = safe_ident(&self.name);
        let override_type = generate_field_type(&self.name, field, settings);
        let check_calls = self.generate_check_calls()?;
        Ok((
            quote!(
                pub type #rule_type = #override_type;
            ),
            quote!(
                use super::#rule_type as Parsed__override;
                #[inline(always)]
                pub fn rule_parser <'a>(
                    state: ParseState<'a>,
                    tracer: impl ParseTracer,
                    cache: &mut ParseCache<'a>
                ) -> ParseResult<'a, super::#rule_type> {
                    let result =
                        parse(state, tracer, cache)?
                        .map(|result| result._override);
                    #check_calls
                    Ok(result)
                }
            ),
        ))
    }

    fn generate_override_rule_enum(
        &self,
        field: &FieldDescriptor,
        settings: &CodegenSettings,
    ) -> Result<(TokenStream, TokenStream)> {
        let rule_type = safe_ident(&self.name);
        let enum_type = generate_enum_type(&self.name, field, settings);
        let check_calls = self.generate_check_calls()?;
        Ok((
            quote!(
                #enum_type
            ),
            quote!(
                use super::#rule_type as Parsed__override;
                #[inline(always)]
                pub fn rule_parser <'a>(
                    state: ParseState<'a>,
                    tracer: impl ParseTracer,
                    cache: &mut ParseCache<'a>
                ) -> ParseResult<'a, super::#rule_type> {
                    let result =
                        parse(state, tracer, cache)?
                        .map(|result| result._override);
                    #check_calls
                    Ok(result)
                }
            ),
        ))
    }

    fn generate_normal_rule(
        &self,
        fields: &[FieldDescriptor],
        settings: &CodegenSettings,
        record_position: RecordPosition,
    ) -> Result<(TokenStream, TokenStream)> {
        let rule_type = safe_ident(&self.name);
        let parsed_enum_types: TokenStream = fields
            .iter()
            .filter(|f| f.type_names.len() > 1)
            .map(|f| generate_enum_type(&format!("{}_{}", self.name, f.name), f, settings))
            .collect();
        let parsed_struct_type = self.definition.generate_struct_type(
            fields,
            settings,
            &self.name,
            record_position,
            PublicType::Yes,
        )?;
        let inner_enum_uses: TokenStream = fields
            .iter()
            .filter(|f| f.type_names.len() > 1)
            .map(|f| {
                let outer_name = format_ident!("{}_{}", self.name, f.name);
                let inner_name = format_ident!("Parsed_{}", f.name);
                quote!(use super::#outer_name as #inner_name;)
            })
            .collect();
        let field_names: Vec<Ident> = fields.iter().map(|f| safe_ident(f.name)).collect();

        let check_calls = self.generate_check_calls()?;

        let rule_parser_body = if record_position == RecordPosition::Yes {
            quote!(
                let result =
                    parse(state.clone(), tracer, cache)?
                    .map_with_state(
                        |r, new_state| super::#rule_type{
                            #( #field_names:r.#field_names,)*
                            position: state.range_until(new_state),
                        }
                    );
                #check_calls
                Ok(result)
            )
        } else {
            quote!(
                let result =
                    parse(state, tracer, cache)?
                    .map(
                        |r| super::#rule_type{
                            #( #field_names:r.#field_names,)*
                        }
                    );
                #check_calls
                Ok(result)
            )
        };
        Ok((
            quote!(
                #parsed_struct_type
                #parsed_enum_types
            ),
            quote!(
                #inner_enum_uses
                #[inline(always)]
                pub fn rule_parser <'a>(
                    state: ParseState<'a>,
                    tracer: impl ParseTracer,
                    cache: &mut ParseCache<'a>
                ) -> ParseResult<'a, super::#rule_type> {
                    #rule_parser_body
                }
            ),
        ))
    }

    fn generate_check_calls(&self) -> Result<TokenStream> {
        let check_name_parts = self.directives.iter().filter_map(|d| {
            if let DirectiveExpression::CheckDirective(c) = d {
                Some(&c.name_parts)
            } else {
                None
            }
        });
        let check_idents = check_name_parts.clone().map(|ps| {
            let part_idents = ps.iter().map(safe_ident);
            quote!(#(#part_idents)::*)
        });
        let check_names = check_name_parts.map(|ps| ps.join("::"));

        Ok(quote!(
            #(
                if !#check_idents(&result.result) {
                    let check_function_error = result.state.report_error(
                        ParseErrorSpecifics::CheckFunctionFailed{function_name: #check_names}
                    );
                    let farthest_error = combine_errors(
                        result.farthest_error,
                        Some(check_function_error)
                    ).unwrap();
                    return Err(farthest_error);
                }
            )*
        ))
    }
}
