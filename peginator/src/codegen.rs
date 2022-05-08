// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

use std::collections::BTreeSet;

use crate::grammar::*;

use anyhow::{bail, Result};
use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote};

fn quick_ident(s: &str) -> Ident {
    Ident::new(s, Span::call_site())
}

pub struct CodegenSettings {
    pub skip_whitespace: bool,
    pub peginator_crate_name: String,
}

impl Default for CodegenSettings {
    fn default() -> Self {
        Self {
            skip_whitespace: true,
            peginator_crate_name: "peginator".into(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Arity {
    One,
    Optional,
    Multiple,
}

#[derive(Debug, Clone)]
pub struct FieldDescriptor<'a> {
    pub name: &'a str,
    pub type_names: BTreeSet<&'a str>,
    pub arity: Arity,
    pub boxed: bool,
}

pub trait CodegenGrammar {
    fn generate_code(&self, settings: &CodegenSettings) -> Result<TokenStream>;
}

impl CodegenGrammar for Grammar {
    fn generate_code(&self, settings: &CodegenSettings) -> Result<TokenStream> {
        let mut all_types = TokenStream::new();
        let mut all_parsers = TokenStream::new();
        let mut all_impls = TokenStream::new();
        let mut cache_entries = TokenStream::new();
        let peginator_crate = format_ident!("{}", settings.peginator_crate_name);
        for rule in &self.rules {
            let (exported, types, impls) = rule.generate_code(settings)?;
            all_types.extend(types);
            all_impls.extend(impls);
            let rule_ident = format_ident!("{}", rule.name);
            let internal_parser_name = format_ident!("parse_{}", rule.name);
            if exported {
                all_parsers.extend(quote!(
                    impl peginator_generated::PegParser for #rule_ident {
                        fn parse_advanced(
                            s: &str,
                            settings: &peginator_generated::ParseSettings)
                        -> Result<Self, peginator_generated::ParseError> {
                            Ok(peginator_generated::#internal_parser_name(
                                peginator_generated::ParseState::new(s, settings),
                                &mut Default::default(),
                            )?.0)
                        }
                    }
                ))
            }

            let cache_entry_ident = format_ident!("c_{}", rule.name);
            cache_entries.extend(quote!(pub #cache_entry_ident: CacheEntries<'a, #rule_ident>,));
        }
        Ok(quote!(
            #all_types
            #all_parsers
            #[allow(
                non_snake_case,
                unused_variables,
                unused_imports,
                unused_mut,
            )]
            mod peginator_generated {
                use super::*;
                pub use #peginator_crate::runtime::{ParseError, ParseSettings, ParseState, PegParser};
                use #peginator_crate::runtime::*;

                #[derive(Default)]
                pub struct ParseCache<'a> {
                    #cache_entries
                }
                #all_impls
            }
        ))
    }
}

trait CodegenRule {
    fn generate_code(&self, settings: &CodegenSettings)
        -> Result<(bool, TokenStream, TokenStream)>;
}

impl CodegenRule for Rule {
    fn generate_code(
        &self,
        settings: &CodegenSettings,
    ) -> Result<(bool, TokenStream, TokenStream)> {
        let mut string_flag = false;
        let mut skip_whitespace = settings.skip_whitespace;
        let mut export = false;
        for directive in &self.directives {
            match directive {
                DirectiveExpression::StringDirective(_) => string_flag = true,
                DirectiveExpression::NoSkipWsDirective(_) => skip_whitespace = false,
                DirectiveExpression::ExportDirective(_) => export = true,
            }
        }

        let settings = CodegenSettings {
            skip_whitespace,
            ..Default::default()
        };

        let name = &self.name;
        let rule_mod = format_ident!("{}_impl", self.name);
        let rule_type = format_ident!("{}", self.name);
        let parser_name = format_ident!("parse_{}", self.name);
        let cache_entry_ident = format_ident!("c_{}", self.name);
        let fields = self.definition.get_fields()?;
        let choice_body = self.definition.generate_code_spec(&fields, &settings)?;
        let outer_parser = quote!(
            #[inline]
            pub(super) fn #parser_name <'a>(state: ParseState<'a>, cache: &mut ParseCache<'a>) -> ParseResult<'a, #rule_type> {
                let cache_key = state.cache_key();
                if let Some(cached) = cache.#cache_entry_ident.get(&cache_key) {
                    cached.clone()
                } else {
                    let result = run_rule_parser(#rule_mod::rule_parser, #name, state, cache);
                    cache.#cache_entry_ident.insert(cache_key, result.clone());
                    result
                }
            }
        );
        if string_flag {
            if export {
                bail!("@string rules cannot be @export-ed");
            };
            let parsed_types = self
                .definition
                .generate_struct_type(&fields, &settings, "Parsed")?;
            Ok((
                false,
                quote!(pub type #rule_type = String;),
                quote!(
                    mod #rule_mod{
                        use super::*;
                        #choice_body
                        #parsed_types
                        #[inline(always)]
                        pub fn rule_parser <'a>(state: ParseState<'a>, cache: &mut ParseCache<'a>) -> ParseResult<'a, String> {
                            let (_, new_state) = parse(state.clone(), cache)?;
                            Ok((state.slice_until(&new_state).to_string(), new_state))
                        }
                    }
                    #outer_parser
                ),
            ))
        } else if fields.len() == 1 && fields[0].name == "_override" {
            if export {
                bail!("Overridden (containing '@:') rules cannot be @export-ed");
            };
            let field = &fields[0];
            if field.type_names.len() <= 1 {
                // Simple case
                let override_type = generate_field_type(&self.name, field, &settings);
                Ok((
                    false,
                    quote!(
                        pub use #override_type as #rule_type;
                    ),
                    quote!(
                        mod #rule_mod{
                            use super::*;
                            #choice_body
                            pub struct Parsed {
                                _override: super::#rule_type,
                            }
                            use super::#rule_type as Parsed__override;
                            #[inline(always)]
                            pub fn rule_parser <'a>(state: ParseState<'a>, cache: &mut ParseCache<'a>) -> ParseResult<'a, super::#rule_type> {
                                let (result, new_state) = parse(state, cache)?;
                                Ok((result._override, new_state))
                            }
                        }
                        #outer_parser
                    ),
                ))
            } else {
                // Enum case
                let override_type = generate_field_type(&self.name, field, &settings);
                let enum_type = generate_enum_type(&self.name, field, &settings);
                Ok((
                    false,
                    quote!(
                        #enum_type
                        pub use #override_type as #rule_type;
                    ),
                    quote!(
                        mod #rule_mod{
                            use super::*;
                            #choice_body
                            pub struct Parsed {
                                _override: super::#rule_type,
                            }
                            use super::#rule_type as Parsed__override;
                            #[inline(always)]
                            pub fn rule_parser <'a>(state: ParseState<'a>, cache: &mut ParseCache<'a>) -> ParseResult<'a, super::#rule_type> {
                                let (result, new_state) = parse(state, cache)?;
                                Ok((result._override, new_state))
                            }
                        }
                        #outer_parser
                    ),
                ))
            }
        } else {
            let parsed_enum_types: TokenStream = fields
                .iter()
                .filter(|f| f.type_names.len() > 1)
                .map(|f| generate_enum_type(&self.name, f, &settings))
                .collect();
            let parsed_struct_type = self
                .definition
                .generate_struct_type(&fields, &settings, &self.name)?;
            let used_types = self
                .definition
                .generate_use_super_as_parsed(&settings, &self.name)?;
            Ok((
                export,
                quote!(
                    #parsed_struct_type
                    #parsed_enum_types
                ),
                quote!(
                    mod #rule_mod{
                        use super::*;
                        #choice_body
                        #used_types
                        #[inline(always)]
                        pub fn rule_parser <'a>(state: ParseState<'a>, cache: &mut ParseCache<'a>) -> ParseResult<'a, Parsed> {
                            parse(state, cache)
                        }
                    }
                    #outer_parser
                ),
            ))
        }
    }
}

trait Codegen {
    fn generate_code_spec(
        &self,
        rule_fields: &[FieldDescriptor],
        settings: &CodegenSettings,
    ) -> Result<TokenStream>;
    fn get_fields(&self) -> Result<Vec<FieldDescriptor>>;

    fn generate_code(
        &self,
        rule_fields: &[FieldDescriptor],
        settings: &CodegenSettings,
    ) -> Result<TokenStream> {
        let spec_body = self.generate_code_spec(rule_fields, settings)?;
        let parsed_type = self.generate_struct_type(rule_fields, settings, "Parsed")?;
        Ok(quote!(
            #spec_body
            #parsed_type
        ))
    }

    fn generate_struct_type(
        &self,
        rule_fields: &[FieldDescriptor],
        settings: &CodegenSettings,
        type_name: &str,
    ) -> Result<TokenStream> {
        let fields = self.get_filtered_rule_fields(rule_fields)?;
        Ok(generate_parsed_struct_type(type_name, &fields, settings))
    }

    fn generate_use_super_as_parsed(
        &self,
        _settings: &CodegenSettings,
        type_name: &str,
    ) -> Result<TokenStream> {
        let fields = self.get_fields()?;
        let type_ident = format_ident!("{}", type_name);
        let enum_types: TokenStream = fields
            .iter()
            .filter(|f| f.type_names.len() > 1)
            .map(|f| {
                let outer_name = format_ident!("{}_{}", type_name, f.name);
                let inner_name = format_ident!("Parsed_{}", f.name);
                quote!(use super::#outer_name as #inner_name;)
            })
            .collect();
        Ok(quote!(
            use super::#type_ident as Parsed;
            #enum_types
        ))
    }

    fn get_filtered_rule_fields<'a>(
        &self,
        rule_fields: &[FieldDescriptor<'a>],
    ) -> Result<Vec<FieldDescriptor<'a>>> {
        let fields = self.get_fields()?;
        Ok(rule_fields
            .iter()
            .filter(|rf| fields.iter().any(|f| f.name == rf.name))
            .cloned()
            .collect())
    }
}

fn generate_field_type(
    parent_type: &str,
    field: &FieldDescriptor,
    _settings: &CodegenSettings,
) -> TokenStream {
    let field_inner_type_ident: TokenStream = if field.type_names.len() > 1 {
        let ident = format_ident!("{}_{}", parent_type, field.name);
        quote!(#ident)
    } else {
        let type_name = field.type_names.iter().next().unwrap();
        let ident = format_ident!("{}", type_name);
        if type_name == &"char" {
            quote!(char)
        } else {
            quote!(#ident)
        }
    };
    match field.arity {
        Arity::One => {
            if field.boxed {
                quote!(Box<#field_inner_type_ident>)
            } else {
                quote!(#field_inner_type_ident)
            }
        }
        Arity::Optional => {
            if field.boxed {
                quote!(Option<Box<#field_inner_type_ident>>)
            } else {
                quote!(Option<#field_inner_type_ident>)
            }
        }
        Arity::Multiple => quote!(Vec<#field_inner_type_ident>),
    }
}

fn generate_enum_type(
    parent_type: &str,
    field: &FieldDescriptor,
    _settings: &CodegenSettings,
) -> TokenStream {
    let ident = format_ident!("{}_{}", parent_type, field.name);
    let type_idents: Vec<Ident> = field.type_names.iter().map(|n| quick_ident(n)).collect();
    quote!(
        #[allow(non_camel_case_types)]
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub enum #ident {
            #(#type_idents(#type_idents),)*
        }
    )
}

fn generate_parsed_struct_type(
    type_name: &str,
    fields: &[FieldDescriptor],
    settings: &CodegenSettings,
) -> TokenStream {
    let type_ident = format_ident!("{}", type_name);
    if fields.is_empty() {
        quote!(
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct #type_ident;
        )
    } else {
        let field_names: Vec<Ident> = fields
            .iter()
            .map(|f| Ident::new(f.name, Span::call_site()))
            .collect();
        let field_types: Vec<TokenStream> = fields
            .iter()
            .map(|f| generate_field_type(type_name, f, settings))
            .collect();
        quote!(
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct #type_ident {
                #( pub #field_names: #field_types, )*
            }
        )
    }
}

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
                    if !new_fields.iter().any(|f| f.name == field.name) {
                        field.arity = Arity::Optional;
                    }
                }
            }
            first_iteration = false;

            for new_field in new_fields {
                if let Some(original) = all_fields.iter_mut().find(|f| f.name == new_field.name) {
                    original.arity = combine_arities_for_choice(&original.arity, &new_field.arity);
                    // TODO: remove duplicates
                    original.type_names.extend(&new_field.type_names);
                } else {
                    all_fields.push(new_field);
                }
            }
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
                        match #choice_mod::parse(state.clone(), cache) {
                            Ok((_, new_state)) => return Ok((Parsed, new_state)),
                            Err(err) => state = state.record_error(err),
                        }
                    ))
                } else {
                    let field_assignments: TokenStream = fields
                        .iter()
                        .map(|field| {
                            let name = format_ident!("{}", field.name);
                            let inner_exists = inner_fields
                                .iter()
                                .any(|inner_field| inner_field.name == field.name);
                            let value = if inner_exists {
                                quote!(result.#name)
                            } else {
                                match field.arity {
                                    Arity::One => {
                                        panic!("Outer field cannot be One if inner does not exist")
                                    }
                                    Arity::Optional => quote!(None),
                                    Arity::Multiple => quote!(Vec::new()),
                                }
                            };
                            quote!(#name: #value,)
                        })
                        .collect();
                    Ok(quote!(
                        match #choice_mod::parse(state.clone(), cache) {
                            Ok((result, new_state)) => return Ok((
                                Parsed{
                                    #field_assignments
                                },
                                new_state
                            )),
                            Err(err) => state = state.record_error(err),
                        }
                    ))
                }
            })
            .collect::<Result<TokenStream>>()?;
        Ok(quote!(
            #[inline(always)]
            pub fn parse<'a>(state: ParseState<'a>, cache: &mut ParseCache<'a>) -> ParseResult<'a, Parsed> {
                let mut state = state;
                #calls
                Err(state.report_farthest_error())
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
                    cache: &mut ParseCache<'a>,
                ) -> ParseResult<'a, Parsed> {
                    Ok((Parsed, state))
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
        let mut fields_seen = BTreeSet::<&str>::new();
        for (num, part) in self.parts.iter().enumerate() {
            let part_mod = format_ident!("part_{}", num);
            let inner_fields = part.get_filtered_rule_fields(rule_fields)?;
            let call = if inner_fields.is_empty() {
                quote!(
                    let (_, state) =  #part_mod::parse(state, cache)?;
                )
            } else {
                let mut field_assignments = TokenStream::new();
                for field in &inner_fields {
                    let name = format_ident!("{}", field.name);
                    let field_assignment = if !fields_seen.contains(field.name) {
                        fields_seen.insert(field.name);
                        quote!(let mut #name = result.#name;)
                    } else {
                        match field.arity {
                            Arity::One => quote!(let #name = result.#name;),
                            Arity::Optional => quote!(#name = #name.or(result.#name);),
                            Arity::Multiple => quote!(#name.extend(result.#name);),
                        }
                    };
                    field_assignments.extend(field_assignment);
                }
                quote!(
                    let (result, state) =  #part_mod::parse(state, cache)?;
                    #field_assignments
                )
            };
            calls.extend(call);
        }
        let field_names: Vec<Ident> = fields.iter().map(|f| format_ident!("{}", f.name)).collect();
        let parse_result = quote!(Parsed{ #( #field_names,)* });
        Ok(quote!(
            #[inline(always)]
            pub fn parse<'a>(state: ParseState<'a>, cache: &mut ParseCache<'a>) -> ParseResult<'a, Parsed> {
                #calls
                Ok((#parse_result, state))
            }
        ))
    }
}

impl Codegen for DelimitedExpression {
    fn generate_code_spec(
        &self,
        rule_fields: &[FieldDescriptor],
        settings: &CodegenSettings,
    ) -> Result<TokenStream> {
        match self {
            DelimitedExpression::Group(a) => a.generate_code_spec(rule_fields, settings),
            DelimitedExpression::Optional(a) => a.generate_code_spec(rule_fields, settings),
            DelimitedExpression::Closure(a) => a.generate_code_spec(rule_fields, settings),
            DelimitedExpression::NegativeLookahead(a) => {
                a.generate_code_spec(rule_fields, settings)
            }
            DelimitedExpression::CharacterRange(a) => a.generate_code_spec(rule_fields, settings),
            DelimitedExpression::CharacterLiteral(a) => a.generate_code_spec(rule_fields, settings),
            DelimitedExpression::StringLiteral(a) => a.generate_code_spec(rule_fields, settings),
            DelimitedExpression::EndOfInput(a) => a.generate_code_spec(rule_fields, settings),
            DelimitedExpression::OverrideField(a) => a.generate_code_spec(rule_fields, settings),
            DelimitedExpression::Field(a) => a.generate_code_spec(rule_fields, settings),
        }
    }

    fn get_fields(&self) -> Result<Vec<FieldDescriptor>> {
        match self {
            DelimitedExpression::Group(a) => a.get_fields(),
            DelimitedExpression::Optional(a) => a.get_fields(),
            DelimitedExpression::Closure(a) => a.get_fields(),
            DelimitedExpression::NegativeLookahead(a) => a.get_fields(),
            DelimitedExpression::CharacterRange(a) => a.get_fields(),
            DelimitedExpression::CharacterLiteral(a) => a.get_fields(),
            DelimitedExpression::StringLiteral(a) => a.get_fields(),
            DelimitedExpression::EndOfInput(a) => a.get_fields(),
            DelimitedExpression::OverrideField(a) => a.get_fields(),
            DelimitedExpression::Field(a) => a.get_fields(),
        }
    }
}

impl Codegen for Group {
    fn generate_code_spec(
        &self,
        rule_fields: &[FieldDescriptor],
        settings: &CodegenSettings,
    ) -> Result<TokenStream> {
        self.body.generate_code_spec(rule_fields, settings)
    }

    fn get_fields(&self) -> Result<Vec<FieldDescriptor>> {
        self.body.get_fields()
    }
}

impl Codegen for Optional {
    fn generate_code_spec(
        &self,
        rule_fields: &[FieldDescriptor],
        settings: &CodegenSettings,
    ) -> Result<TokenStream> {
        let body = self.body.generate_code(rule_fields, settings)?;
        let fields = self.body.get_filtered_rule_fields(rule_fields)?;
        let happy_case_fields: TokenStream = fields
            .iter()
            .map(|field| {
                let name = format_ident!("{}", field.name);
                quote!(#name: result.#name,)
            })
            .collect();
        let unhappy_case_fields: TokenStream = fields
            .iter()
            .map(|field| {
                let name = format_ident!("{}", field.name);
                quote!(#name: Default::default(),)
            })
            .collect();
        Ok(quote!(
            mod optional{
                use super::*;
                #body
            }
            #[inline(always)]
            pub fn parse<'a>(state: ParseState<'a>, cache: &mut ParseCache<'a>) -> ParseResult<'a, Parsed> {
                if let Ok((result, new_state)) = optional::parse(state.clone(), cache) {
                    Ok((Parsed{
                        #happy_case_fields
                    }, new_state))
                } else {
                    Ok((Parsed{
                        #unhappy_case_fields
                    }, state))
                }
            }
        ))
    }

    fn get_fields(&self) -> Result<Vec<FieldDescriptor>> {
        Ok(set_arity_to_optional(self.body.get_fields()?))
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
impl Codegen for Closure {
    fn generate_code_spec(
        &self,
        rule_fields: &[FieldDescriptor],
        settings: &CodegenSettings,
    ) -> Result<TokenStream> {
        let closure_body = self.body.generate_code(rule_fields, settings)?;
        let fields = self.body.get_filtered_rule_fields(rule_fields)?;
        let declarations: TokenStream = fields
            .iter()
            .map(|f| {
                let typ = generate_field_type("Parsed", f, settings);
                let name_ident = format_ident!("{}", f.name);
                quote!(let mut #name_ident: #typ = Vec::new();)
            })
            .collect();
        let assignments: TokenStream = fields
            .iter()
            .map(|field| {
                let name = format_ident!("{}", field.name);
                quote!(#name.extend(result.#name);)
            })
            .collect();
        let field_names: Vec<Ident> = fields.iter().map(|f| format_ident!("{}", f.name)).collect();
        let parse_result = quote!(Parsed{ #( #field_names,)* });
        let at_least_one_body = if self.at_least_one.is_some() {
            quote!(
                let (result, new_state) = closure::parse(state, cache)?;
                #assignments
                state = new_state;
            )
        } else {
            quote!()
        };

        Ok(quote!(
            mod closure{
                use super::*;
                #closure_body
            }
            #[inline(always)]
            pub fn parse<'a>(state: ParseState<'a>, cache: &mut ParseCache<'a>) -> ParseResult<'a, Parsed> {
                let mut state = state;
                #declarations
                #at_least_one_body
                loop {
                    match closure::parse(state.clone(), cache) {
                        Ok((result, new_state)) => {
                            #assignments
                            state = new_state;
                        },
                        Err(err) => {
                            state = state.record_error(err);
                            break;
                        }
                    }
                }
                Ok((#parse_result, state))
            }
        ))
    }

    fn get_fields(&self) -> Result<Vec<FieldDescriptor>> {
        Ok(set_arity_to_multiple(self.body.get_fields()?))
    }
}

fn set_arity_to_multiple(fields: Vec<FieldDescriptor>) -> Vec<FieldDescriptor> {
    let mut fields = fields;
    for value in &mut fields {
        value.arity = Arity::Multiple;
    }
    fields
}

impl Codegen for NegativeLookahead {
    fn generate_code_spec(
        &self,
        rule_fields: &[FieldDescriptor],
        settings: &CodegenSettings,
    ) -> Result<TokenStream> {
        let body = self.expr.generate_code(rule_fields, settings)?;
        Ok(quote!(
            mod negative_lookahead{
                use super::*;
                #body
            }
            #[inline(always)]
            pub fn parse<'a>(state: ParseState<'a>, cache: &mut ParseCache<'a>) -> ParseResult<'a, Parsed> {
                match negative_lookahead::parse (state.clone(), cache) {
                    Ok(_) => Err(state.report_error(ParseErrorSpecifics::NegativeLookaheadFailed)),
                    Err(_) => Ok((Parsed, state)),
                }
            }
        ))
    }

    fn get_fields(&self) -> Result<Vec<FieldDescriptor>> {
        // TODO: check if body has fields. It should NOT, since capturing in a negative lookeahead
        // is not supported at all.
        Ok(Vec::new())
    }
}

impl Codegen for CharacterRange {
    fn generate_code_spec(
        &self,
        _rule_fields: &[FieldDescriptor],
        settings: &CodegenSettings,
    ) -> Result<TokenStream> {
        let from = &self.from;
        let to = &self.to;
        let skip_ws = if settings.skip_whitespace {
            quote!(let state = state.skip_whitespace();)
        } else {
            quote!()
        };
        Ok(quote!(
            #[inline(always)]
            pub fn parse<'a>(state: ParseState<'a>, cache: &mut ParseCache<'a>) -> ParseResult<'a, Parsed> {
                #skip_ws
                let (_, state) = parse_character_range(state, #from, #to)?;
                Ok((Parsed, state))
            }
        ))
    }

    fn get_fields(&self) -> Result<Vec<FieldDescriptor>> {
        Ok(Vec::new())
    }
}

impl Codegen for CharacterLiteral {
    fn generate_code_spec(
        &self,
        _rule_fields: &[FieldDescriptor],
        settings: &CodegenSettings,
    ) -> Result<TokenStream> {
        let literal = &self;
        let skip_ws = if settings.skip_whitespace {
            quote!(let state = state.skip_whitespace();)
        } else {
            quote!()
        };
        Ok(quote!(
            #[inline(always)]
            pub fn parse<'a>(state: ParseState<'a>, cache: &mut ParseCache<'a>) -> ParseResult<'a, Parsed> {
                #skip_ws
                let (_, state) = parse_character_literal(state, #literal)?;
                Ok((Parsed, state))
            }
        ))
    }

    fn get_fields(&self) -> Result<Vec<FieldDescriptor>> {
        Ok(Vec::new())
    }
}

impl Codegen for StringLiteral {
    fn generate_code_spec(
        &self,
        _rule_fields: &[FieldDescriptor],
        settings: &CodegenSettings,
    ) -> Result<TokenStream> {
        let literal = &self.body;
        let skip_ws = if settings.skip_whitespace {
            quote!(let state = state.skip_whitespace();)
        } else {
            quote!()
        };
        Ok(quote!(
            #[inline(always)]
            pub fn parse<'a>(state: ParseState<'a>, cache: &mut ParseCache<'a>) -> ParseResult<'a, Parsed> {
                #skip_ws
                let (_, state) = parse_string_literal(state, #literal)?;
                Ok((Parsed, state))
            }
        ))
    }

    fn get_fields(&self) -> Result<Vec<FieldDescriptor>> {
        Ok(Vec::new())
    }
}

impl Codegen for EndOfInput {
    fn generate_code_spec(
        &self,
        _rule_fields: &[FieldDescriptor],
        settings: &CodegenSettings,
    ) -> Result<TokenStream> {
        let skip_ws = if settings.skip_whitespace {
            quote!(let state = state.skip_whitespace();)
        } else {
            quote!()
        };
        Ok(quote!(
            #[inline(always)]
            pub fn parse<'a>(state: ParseState<'a>, cache: &mut ParseCache<'a>) -> ParseResult<'a, Parsed> {
                #skip_ws
                if state.is_empty() {
                    Ok((Parsed, state))
                } else {
                    Err(state.report_error(ParseErrorSpecifics::ExpectedEoi))
                }
            }
        ))
    }

    fn get_fields(&self) -> Result<Vec<FieldDescriptor>> {
        Ok(Vec::new())
    }
}

impl Codegen for OverrideField {
    fn generate_code_spec(
        &self,
        rule_fields: &[FieldDescriptor],
        settings: &CodegenSettings,
    ) -> Result<TokenStream> {
        // TODO: CONVERT
        let parser_name = format_ident!("parse_{}", self.typ);
        let skip_ws = if settings.skip_whitespace {
            quote!(let state = state.skip_whitespace();)
        } else {
            quote!()
        };
        let field_conversion = generate_field_converter("_override", &self.typ, rule_fields);
        Ok(quote!(
            #[inline(always)]
            pub fn parse<'a>(state: ParseState<'a>, cache: &mut ParseCache<'a>) -> ParseResult<'a, Parsed> {
                #skip_ws
                let (result, state) = #parser_name (state, cache)?;
                Ok((Parsed{ _override: #field_conversion }, state))
            }
        ))
    }

    fn get_fields(&self) -> Result<Vec<FieldDescriptor>> {
        Ok(vec![FieldDescriptor {
            name: "_override",
            type_names: [&self.typ as &str].into(),
            arity: Arity::One,
            boxed: false,
        }])
    }
}

impl Codegen for Field {
    fn generate_code_spec(
        &self,
        rule_fields: &[FieldDescriptor],
        settings: &CodegenSettings,
    ) -> Result<TokenStream> {
        let parser_name = format_ident!("parse_{}", self.typ);
        let skip_ws = if settings.skip_whitespace {
            quote!(let state = state.skip_whitespace();)
        } else {
            quote!()
        };
        if let Some(field_name) = &self.name {
            let field_ident = format_ident!("{}", field_name);
            let field_conversion = generate_field_converter(field_name, &self.typ, rule_fields);
            Ok(quote!(
                #[inline(always)]
                pub fn parse<'a>(state: ParseState<'a>, cache: &mut ParseCache<'a>) -> ParseResult<'a, Parsed> {
                    #skip_ws
                    let (result, state) = #parser_name (state, cache)?;
                    Ok((Parsed{ #field_ident: #field_conversion }, state))
                }
            ))
        } else {
            Ok(quote!(
                #[inline(always)]
                pub fn parse<'a>(state: ParseState<'a>, cache: &mut ParseCache<'a>) -> ParseResult<'a, Parsed> {
                    #skip_ws
                    let (_, state) = #parser_name (state, cache)?;
                    Ok((Parsed, state))
                }
            ))
        }
    }

    fn get_fields(&self) -> Result<Vec<FieldDescriptor>> {
        if let Some(field_name) = &self.name {
            Ok(vec![FieldDescriptor {
                name: field_name,
                type_names: [&self.typ as &str].into(),
                arity: Arity::One,
                boxed: self.boxed.is_some(),
            }])
        } else {
            Ok(Vec::new())
        }
    }
}

fn generate_field_converter(
    field_name: &str,
    field_type_name: &str,
    rule_fields: &[FieldDescriptor],
) -> TokenStream {
    let field = rule_fields
        .iter()
        .find(|f| f.name == field_name)
        .expect("Field not found in rule_fields");
    let enumified_field = if field.type_names.len() > 1 {
        let enum_type_name = format_ident!("Parsed_{}", field_name);
        let field_type_ident = format_ident!("{}", field_type_name);
        quote!(#enum_type_name::#field_type_ident(result))
    } else {
        quote!(result)
    };
    match field.arity {
        Arity::One => {
            if field.boxed {
                quote!(Box::new(#enumified_field))
            } else {
                quote!(#enumified_field)
            }
        }
        Arity::Optional => {
            if field.boxed {
                quote!(Some(Box::new(#enumified_field)))
            } else {
                quote!(Some(#enumified_field))
            }
        }
        Arity::Multiple => quote!(vec![#enumified_field]),
    }
}
