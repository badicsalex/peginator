// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

use anyhow::{bail, Result};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

use super::common::{
    generate_derives, generate_enum_type, generate_field_type, generate_rule_parse_function,
    safe_ident, Arity, Codegen, CodegenRule, CodegenSettings, FieldDescriptor, PublicType,
    RecordPosition,
};
use crate::grammar::{DirectiveExpression, Grammar, Rule};

impl CodegenRule for Rule {
    fn generate_code(
        &self,
        grammar: &Grammar,
        settings: &CodegenSettings,
    ) -> Result<(TokenStream, TokenStream)> {
        let flags = self.flags();
        let settings = CodegenSettings {
            skip_whitespace: settings.skip_whitespace && !flags.no_skip_ws,
            ..settings.clone()
        };

        let fields = self.definition.get_fields(grammar)?;

        self.check_flags(&flags, &settings)?;

        let name = &self.name;
        let rule_mod = self.rule_module_ident();
        let rule_type = safe_ident(&self.name);
        let parser_name = format_ident!("parse_{name}");
        let choice_body = self.definition.generate_code(&fields, grammar, &settings)?;

        let (types, inner_decls, parse_body) = if flags.string {
            self.generate_string_rule(&settings)?
        } else if fields.len() == 1 && fields[0].name == "_override" {
            self.generate_override_rule(&fields, &settings)?
        } else {
            self.generate_normal_rule(&fields, grammar, &settings, flags.position.into())?
        };

        let rule_parser_call = self.generate_memoized_body(parse_body);
        let parse_body = quote!(
            global.tracer.print_trace_start(&state, #name);
            let result = { #rule_parser_call };
            global.tracer.print_trace_result(&result);
            result
        );

        let parse_function =
            generate_rule_parse_function(parser_name, rule_type, parse_body, &settings);
        let position_impls = self.generate_impl_position(&fields);

        Ok((
            types,
            quote!(
                mod #rule_mod{
                    use super::*;
                    #choice_body
                    #inner_decls
                }
                #parse_function
                #position_impls
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
    pub left_recursive: bool,
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
                DirectiveExpression::LeftrecDirective(_) => result.left_recursive = true,
                DirectiveExpression::CheckDirective(_) => (),
            }
        }
        result
    }

    fn check_flags(&self, flags: &RuleFlags, settings: &CodegenSettings) -> Result<()> {
        if flags.export && flags.string {
            bail!("@string rules cannot be @export-ed");
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
        settings: &CodegenSettings,
    ) -> Result<(TokenStream, TokenStream, TokenStream)> {
        let rule_mod = self.rule_module_ident();
        let rule_type_ident = safe_ident(&self.name);
        let check_calls = self.generate_check_calls(settings)?;
        let flags = self.flags();
        let type_decl = if flags.position {
            let derives = generate_derives(settings);
            quote!(
                #derives
                pub struct #rule_type_ident {
                    pub string: String,
                    pub position: std::ops::Range<usize>
                }
            )
        } else {
            quote!(pub type #rule_type_ident = String;)
        };
        let return_value = if flags.position {
            quote!(#rule_type_ident {
                string,
                position: state.range_until(new_state)
            })
        } else {
            quote!(string)
        };
        Ok((
            type_decl,
            quote!(),
            quote!(
                let result =
                    #rule_mod::parse(state.clone(), global)?
                    .map_with_state(
                        |_, new_state| {
                            let string = state.slice_until(new_state).to_string();
                            #return_value
                        }
                    );
                #check_calls
                Ok(result)
            ),
        ))
    }
    fn generate_override_rule(
        &self,
        fields: &[FieldDescriptor],
        settings: &CodegenSettings,
    ) -> Result<(TokenStream, TokenStream, TokenStream)> {
        let field = &fields[0];
        if field.type_names.len() <= 1 {
            let flags = self.flags();
            if flags.export {
                bail!("Simply overridden (containing '@:') rules cannot be @export-ed. Try the > operator instead.");
            }
            if flags.position {
                bail!("Simply overridden (containing '@:') rules cannot contain @position. Try the > operator instead.");
            }
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
    ) -> Result<(TokenStream, TokenStream, TokenStream)> {
        let rule_mod = self.rule_module_ident();
        let rule_type = safe_ident(&self.name);
        let override_type = generate_field_type(&self.name, field, settings);
        let check_calls = self.generate_check_calls(settings)?;
        Ok((
            quote!(
                pub type #rule_type = #override_type;
            ),
            quote!(
                use super::#rule_type as Parsed__override;
            ),
            quote!(
                let result = #rule_mod::parse(state, global)?;
                #check_calls
                Ok(result)
            ),
        ))
    }

    fn generate_override_rule_enum(
        &self,
        field: &FieldDescriptor,
        settings: &CodegenSettings,
    ) -> Result<(TokenStream, TokenStream, TokenStream)> {
        let rule_mod = self.rule_module_ident();
        let rule_type = safe_ident(&self.name);
        let enum_type = generate_enum_type(&self.name, field, settings);
        let check_calls = self.generate_check_calls(settings)?;
        Ok((
            quote!(
                #enum_type
            ),
            quote!(
                use super::#rule_type as Parsed__override;
            ),
            quote!(
                let result = #rule_mod::parse(state, global)?;
                #check_calls
                Ok(result)
            ),
        ))
    }

    fn generate_normal_rule(
        &self,
        fields: &[FieldDescriptor],
        grammar: &Grammar,
        settings: &CodegenSettings,
        record_position: RecordPosition,
    ) -> Result<(TokenStream, TokenStream, TokenStream)> {
        let rule_mod = self.rule_module_ident();
        let rule_type = safe_ident(&self.name);
        let parsed_enum_types: TokenStream = fields
            .iter()
            .filter(|f| f.type_names.len() > 1)
            .map(|f| generate_enum_type(&format!("{}_{}", self.name, f.name), f, settings))
            .collect();
        let parsed_struct_type = self.definition.generate_struct_type(
            fields,
            grammar,
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
        let field_assignments = if field_names.len() == 1 {
            quote!(#( #field_names:r, )*)
        } else {
            quote!(#( #field_names:r.#field_names, )*)
        };

        let check_calls = self.generate_check_calls(settings)?;

        let rule_parser_body = if record_position == RecordPosition::Yes {
            quote!(
                let result =
                    #rule_mod::parse(state.clone(), global)?
                    .map_with_state(
                        |r, new_state| super::#rule_type{
                            #field_assignments
                            position: state.range_until(new_state),
                        }
                    );
                #check_calls
                Ok(result)
            )
        } else {
            quote!(
                let result =
                    #rule_mod::parse(state, global)?
                    .map(
                        |r| super::#rule_type{
                            #field_assignments
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
            inner_enum_uses,
            rule_parser_body,
        ))
    }

    fn generate_check_calls(&self, settings: &CodegenSettings) -> Result<TokenStream> {
        let check_name_parts = self.directives.iter().filter_map(|d| {
            if let DirectiveExpression::CheckDirective(c) = d {
                Some(&c.function)
            } else {
                None
            }
        });
        let check_idents = check_name_parts.clone().map(|ps| {
            let part_idents = ps.iter().map(safe_ident);
            quote!(#(#part_idents)::*)
        });
        let check_names = check_name_parts.map(|ps| ps.join("::"));
        let user_context_param = if settings.has_user_context {
            quote!(, global.user_context)
        } else {
            quote!()
        };

        Ok(quote!(
            #(
                if !#check_idents(&result.result #user_context_param) {
                    return Err(result.state.report_error(
                        ParseErrorSpecifics::CheckFunctionFailed{function_name: #check_names}
                    ));
                }
            )*
        ))
    }

    fn generate_impl_position(&self, fields: &[FieldDescriptor]) -> TokenStream {
        let rule_type = safe_ident(&self.name);
        if self.flags().position {
            if fields.len() == 1 && fields[0].name == "_override" && fields[0].type_names.len() > 1
            {
                let cases = fields[0].type_names.iter().map(safe_ident);
                quote!(
                    impl PegPosition for #rule_type {
                        fn position(&self) -> &std::ops::Range<usize> {
                            match self{
                                #(Self::#cases(x) => x.position(),)*
                            }
                        }
                    }
                )
            } else {
                quote!(
                    impl PegPosition for #rule_type {
                        fn position(&self) -> &std::ops::Range<usize> {
                            &self.position
                        }
                    }
                )
            }
        } else {
            TokenStream::new()
        }
    }

    fn generate_memoized_body(&self, parse_body: TokenStream) -> TokenStream {
        let flags = self.flags();
        let cache_entry_ident = format_ident!("c_{}", self.name);
        if flags.left_recursive {
            quote!(
                let cache_key = state.cache_key();
                if let Some(cached) = global.cache.#cache_entry_ident.get(&cache_key) {
                    global.tracer.print_informative("Cache hit (left recursive)");
                    cached.clone()
                } else {
                    let mut best_result = Err(state.clone().report_error(ParseErrorSpecifics::LeftRecursionSentinel));
                    global.cache.#cache_entry_ident.insert(cache_key, best_result.clone());
                    loop {
                        global.tracer.print_informative("Starting new left recursive loop");
                        let state = state.clone();
                        let new_result = { #parse_body };
                        match (new_result, &best_result) {
                            (Ok(nro), Ok(bro)) => {
                                if nro.state.is_further_than(&bro.state) {
                                    best_result = Ok(nro);
                                    global.cache.#cache_entry_ident.insert(cache_key, best_result.clone());
                                } else {
                                    break;
                                }
                            }
                            (Ok(nro), Err(bre)) => {
                                best_result = Ok(nro);
                                global.cache.#cache_entry_ident.insert(cache_key, best_result.clone());
                            }
                            (Err(nre), Ok(bro)) => {
                                break;
                            }
                            (Err(nre), Err(bre)) => {
                                best_result = Err(nre);
                                global.cache.#cache_entry_ident.insert(cache_key, best_result.clone());
                                break;
                            }
                        }
                    }
                    best_result
                }
            )
        } else if flags.memoize {
            quote!(
                let cache_key = state.cache_key();
                if let Some(cached) = global.cache.#cache_entry_ident.get(&cache_key) {
                    global.tracer.print_informative("Cache hit");
                    cached.clone()
                } else {
                    let result = { #parse_body };
                    global.cache.#cache_entry_ident.insert(cache_key, result.clone());
                    result
                }
            )
        } else {
            parse_body
        }
    }

    fn rule_module_ident(&self) -> Ident {
        format_ident!("{}_impl", self.name)
    }
}
