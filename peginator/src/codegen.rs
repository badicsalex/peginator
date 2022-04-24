// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

use std::collections::BTreeSet;

use crate::grammar::*;

use anyhow::Result;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote};

fn quick_ident(s: &str) -> Ident {
    Ident::new(s, Span::call_site())
}

pub struct CodegenSettings {
    pub grammar_module_prefix: TokenStream,
    pub runtime_prefix: TokenStream,
    pub skip_whitespace: bool,
}

#[derive(Debug)]
pub enum Arity {
    One,
    Optional,
    Multiple,
}

#[derive(Debug)]
pub struct FieldDescriptor<'a> {
    pub name: &'a str,
    pub type_names: BTreeSet<&'a str>,
    pub arity: Arity,
    pub boxed: bool,
}

pub trait CodegenOuter {
    fn generate_code(&self, settings: &CodegenSettings) -> Result<TokenStream>;
}

impl CodegenOuter for Grammar {
    fn generate_code(&self, settings: &CodegenSettings) -> Result<TokenStream> {
        self.rules
            .iter()
            .map(|rule| rule.generate_code(settings))
            .collect()
    }
}
impl CodegenOuter for Rule {
    fn generate_code(&self, settings: &CodegenSettings) -> Result<TokenStream> {
        let mut string_flag = false;
        let mut skip_whitespace = settings.skip_whitespace;
        for directive in &self.directives {
            match directive {
                DirectiveExpression::StringDirective(_) => string_flag = true,
                DirectiveExpression::NoSkipWsDirective(_) => skip_whitespace = false,
            }
        }

        let settings = CodegenSettings {
            grammar_module_prefix: settings.grammar_module_prefix.clone(),
            runtime_prefix: settings.runtime_prefix.clone(),
            skip_whitespace,
        };

        let name = &self.name;
        let rule_mod = format_ident!("{}_impl", self.name);
        let rule_type = format_ident!("{}", self.name);
        let parser_name = format_ident!("parse_{}", self.name);
        let runtime_prefix = &settings.runtime_prefix;
        let choice_body = self.definition.generate_code(&settings)?;
        let fields = self.definition.get_fields()?;
        let outer_parser = quote!(
            pub fn #parser_name (state: ParseState) -> ParseResult<#rule_type> {
                run_rule_parser(#rule_mod::rule_parser, #name, state)
            }
        );
        if string_flag {
            Ok(quote!(
                mod #rule_mod{
                    use #runtime_prefix *;
                    #choice_body
                    pub fn rule_parser (state: ParseState) -> ParseResult<String> {
                        let (_, new_state) = parse(state.clone())?;
                        Ok((state.slice_until(&new_state).to_string(), new_state))
                    }
                }
                pub type #rule_type = String;
                #outer_parser
            ))
        } else if fields.len() == 1 && fields[0].name == "_override" {
            let override_type = generate_field_type(&fields[0], &settings);
            Ok(quote!(
                mod #rule_mod{
                    use #runtime_prefix *;
                    #choice_body
                    // Inside here, because an enum might need to be exported
                    pub type OverrideType = #override_type;
                    pub fn rule_parser (state: ParseState) -> ParseResult<OverrideType> {
                        let (result, new_state) = parse(state)?;
                        Ok((result._override, new_state))
                    }
                }
                pub use #rule_mod::OverrideType as #rule_type;
                #outer_parser
            ))
        } else {
            Ok(quote!(
                mod #rule_mod{
                    use #runtime_prefix *;
                    #choice_body
                    pub fn rule_parser (state: ParseState) -> ParseResult<Parsed> {
                        parse(state)
                    }
                }
                pub use #rule_mod::Parsed as #rule_type;
                #outer_parser
            ))
        }
    }
}

trait Codegen {
    fn generate_code_spec(&self, settings: &CodegenSettings) -> Result<TokenStream>;
    fn get_fields(&self) -> Result<Vec<FieldDescriptor>>;

    fn generate_code(&self, settings: &CodegenSettings) -> Result<TokenStream> {
        let spec_body = self.generate_code_spec(settings)?;
        let fields = self.get_fields()?;
        let parsed_type = generate_parsed_struct_type(&fields, settings);
        let enum_types: TokenStream = fields
            .iter()
            .filter(|f| f.type_names.len() > 1)
            .map(|f| generate_enum_type(f, settings))
            .collect();
        Ok(quote!(
            #spec_body
            #enum_types
            #parsed_type
        ))
    }
}

fn generate_field_type(field: &FieldDescriptor, settings: &CodegenSettings) -> TokenStream {
    let prefix = &settings.grammar_module_prefix;
    let field_inner_type_ident: TokenStream = if field.type_names.len() > 1 {
        let ident = format_ident!("E_{}", field.name);
        quote!(#ident)
    } else {
        let type_name = field.type_names.iter().next().unwrap();
        let ident = format_ident!("{}", type_name);
        if type_name == &"char" {
            quote!(char)
        } else {
            quote!(#prefix #ident)
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

fn generate_enum_type(field: &FieldDescriptor, settings: &CodegenSettings) -> TokenStream {
    let prefix = &settings.grammar_module_prefix;
    let ident = format_ident!("E_{}", field.name);
    let type_idents: Vec<Ident> = field.type_names.iter().map(|n| quick_ident(n)).collect();
    quote!(
        #[derive(Debug)]
        pub enum #ident {
            #(#type_idents(#prefix #type_idents),)*
        }
    )
}

fn generate_parsed_struct_type(
    fields: &[FieldDescriptor],
    settings: &CodegenSettings,
) -> TokenStream {
    if fields.is_empty() {
        quote!(
            pub type Parsed = ();
        )
    } else {
        let field_names: Vec<Ident> = fields
            .iter()
            .map(|f| Ident::new(f.name, Span::call_site()))
            .collect();
        let field_types: Vec<TokenStream> = fields
            .iter()
            .map(|f| generate_field_type(f, settings))
            .collect();
        quote!(
            #[derive(Debug)]
            pub struct Parsed {
                #( pub #field_names: #field_types, )*
            }
        )
    }
}

impl Codegen for Choice {
    fn generate_code_spec(&self, settings: &CodegenSettings) -> Result<TokenStream> {
        if self.choices.len() < 2 {
            return self.choices[0].generate_code_spec(settings);
        }
        let runtime_prefix = &settings.runtime_prefix;
        let choice_bodies = self
            .choices
            .iter()
            .enumerate()
            .map(|(num, choice)| -> Result<TokenStream> {
                let choice_mod = format_ident!("choice_{}", num);
                let sequence_body = choice.generate_code(settings)?;
                Ok(quote!(
                    mod #choice_mod{
                    use #runtime_prefix *;
                        #sequence_body
                    }
                ))
            })
            .collect::<Result<TokenStream>>()?;
        let parse_function = self.generate_parse_function(settings)?;
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
    fn generate_parse_function(&self, settings: &CodegenSettings) -> Result<TokenStream> {
        let fields = self.get_fields()?;
        let calls = self
            .choices
            .iter()
            .enumerate()
            .map(|(num, choice)| {
                let choice_mod = format_ident!("choice_{}", num);
                if fields.is_empty() {
                    Ok(quote!(
                        if let Ok((_, new_state)) = #choice_mod::parse(state.clone()) {
                            return Ok(((), new_state));
                        }
                    ))
                } else {
                    let inner_fields = choice.get_fields()?;
                    let field_assignments =
                        Self::generate_field_assignments(&fields, &inner_fields);
                    Ok(quote!(
                        if let Ok((result, new_state)) = #choice_mod::parse(state.clone()) {
                            return Ok((
                                Parsed{
                                    #field_assignments
                                },
                                new_state
                            ));
                        }
                    ))
                }
            })
            .collect::<Result<TokenStream>>()?;
        Ok(quote!(
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                #calls
                Err(ParseError)
            }
        ))
    }

    fn generate_field_assignments(
        my_fields: &[FieldDescriptor],
        inner_fields: &[FieldDescriptor],
    ) -> TokenStream {
        my_fields
            .iter()
            .map(|my_field| {
                let name = format_ident!("{}", my_field.name);
                let inner_field_opt = inner_fields.iter().find(|f| f.name == my_field.name);
                let value =
                    if let Some(inner_field) = inner_field_opt {
                        let enum_converted = Self::generate_enum_conversion(my_field, inner_field);
                        match (&my_field.arity, &inner_field.arity) {
                            (Arity::One, Arity::One) => quote!(#enum_converted),
                            (Arity::Optional, Arity::Optional) => quote!(#enum_converted),
                            (Arity::Multiple, Arity::Multiple) => quote!(#enum_converted),
                            (Arity::Optional, Arity::One) => quote!(Some(#enum_converted)),
                            (Arity::Multiple, Arity::One) => quote!(vec![#enum_converted]),
                            (Arity::Multiple, Arity::Optional) => quote!(
                                // TODO: enum conversion
                                if let Some(result_inner) = result.#name {
                                    vec![result_inner]
                                } else {
                                    vec![]
                                }),
                            _ => panic!("Invalid Arity combination in choice inner => outer mapping")
                        }
                    } else {
                        match my_field.arity {
                            Arity::One => panic!("Not found Arity::One field in choice inner fields. This should never happen"),
                            Arity::Optional => quote!(None),
                            Arity::Multiple => quote!(Vec::new()),
                        }
                    };
                quote!(#name: #value,)
            })
            .collect()
    }

    fn generate_enum_conversion(
        my_field: &FieldDescriptor,
        inner_field: &FieldDescriptor,
    ) -> TokenStream {
        let name = format_ident!("{}", my_field.name);
        let enum_type = format_ident!("E_{}", my_field.name);
        if my_field.type_names.len() < 2 {
            quote!( result.#name)
        } else if inner_field.type_names.len() < 2 {
            let inner_type = format_ident!("{}", inner_field.type_names.iter().next().unwrap());
            quote!(#enum_type::#inner_type(result.#name))
        } else {
            let matches: TokenStream = inner_field
                .type_names
                .iter()
                .map(|type_name| {
                    let type_ident = format_ident!("{}", type_name);
                    quote!(
                        #type_ident(a) => #enum_type::#type_ident(a),
                    )
                })
                .collect();
            quote!(
                match result.#name {
                    #matches
                }
            )
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

impl Codegen for Sequence {
    fn generate_code_spec(&self, settings: &CodegenSettings) -> Result<TokenStream> {
        if self.parts.is_empty() {
            return Ok(quote!(
                pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                    Ok(((), state))
                }
            ));
        }
        if self.parts.len() < 2 {
            return self.parts[0].generate_code_spec(settings);
        }
        let runtime_prefix = &settings.runtime_prefix;
        let part_bodies = self
            .parts
            .iter()
            .enumerate()
            .map(|(num, part)| -> Result<TokenStream> {
                let part_mod = format_ident!("part_{}", num);
                let part_body = part.generate_code(settings)?;
                Ok(quote!(
                    mod #part_mod{
                        use #runtime_prefix *;
                        #part_body
                    }
                ))
            })
            .collect::<Result<TokenStream>>()?;
        let parse_function = self.generate_parse_function(settings)?;
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
    fn generate_parse_function(&self, settings: &CodegenSettings) -> Result<TokenStream> {
        let fields = self.get_fields()?;
        let declarations: TokenStream = fields
            .iter()
            .map(|f| {
                let typ = generate_field_type(f, settings);
                let name_ident = format_ident!("{}", f.name);
                match f.arity {
                    Arity::One => quote!(),
                    Arity::Optional => quote!(let mut #name_ident: #typ = None;),
                    Arity::Multiple => quote!(let mut #name_ident: #typ = Vec::new();),
                }
            })
            .collect();
        let calls = self
            .parts
            .iter()
            .enumerate()
            .map(|(num, part)| {
                let part_mod = format_ident!("part_{}", num);
                let inner_fields = part.get_fields()?;
                if inner_fields.is_empty() {
                    Ok(quote!(
                        let (_, state) =  #part_mod::parse(state)?;
                    ))
                } else {
                    let field_assignments =
                        Self::generate_field_assignments(&fields, &inner_fields);
                    Ok(quote!(
                        let (result, state) =  #part_mod::parse(state)?;
                        #field_assignments
                    ))
                }
            })
            .collect::<Result<TokenStream>>()?;
        let field_names: Vec<Ident> = fields.iter().map(|f| format_ident!("{}", f.name)).collect();
        let parse_result = if fields.is_empty() {
            quote!(())
        } else {
            quote!(Parsed{ #( #field_names,)* })
        };
        Ok(quote!(
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                #declarations
                #calls
                Ok((#parse_result, state))
            }
        ))
    }
    fn generate_field_assignments(
        my_fields: &[FieldDescriptor],
        inner_fields: &[FieldDescriptor],
    ) -> TokenStream {
        inner_fields
            .iter()
            .map(|inner_field| {
                let my_field = my_fields
                    .iter()
                    .find(|f| f.name == inner_field.name)
                    .unwrap();
                let name = format_ident!("{}", my_field.name);
                // TODO: Enum conversion
                match (&my_field.arity, &inner_field.arity) {
                    (Arity::One, Arity::One) => quote!(let #name = result.#name;),
                    (Arity::Optional, Arity::Optional) => quote!(#name = #name.or(result.#name);),
                    (Arity::Multiple, Arity::Multiple) => quote!(#name.extend(result.#name);),
                    (Arity::Optional, Arity::One) => quote!(#name = Some(result.#name);),
                    (Arity::Multiple, Arity::One) => quote!(#name.push(result.#name);),
                    (Arity::Multiple, Arity::Optional) => quote!(
                        if let Some(result_inner) = result.#name {
                            vec![result_inner]
                        } else {
                            vec![]
                        }
                    ),
                    _ => panic!("Invalid Arity combination in sequence inner => outer mapping"),
                }
            })
            .collect()
    }
}

impl Codegen for DelimitedExpression {
    fn generate_code_spec(&self, settings: &CodegenSettings) -> Result<TokenStream> {
        match self {
            DelimitedExpression::Group(a) => a.generate_code_spec(settings),
            DelimitedExpression::Optional(a) => a.generate_code_spec(settings),
            DelimitedExpression::Closure(a) => a.generate_code_spec(settings),
            DelimitedExpression::NegativeLookahead(a) => a.generate_code_spec(settings),
            DelimitedExpression::CharacterRange(a) => a.generate_code_spec(settings),
            DelimitedExpression::CharacterLiteral(a) => a.generate_code_spec(settings),
            DelimitedExpression::StringLiteral(a) => a.generate_code_spec(settings),
            DelimitedExpression::OverrideField(a) => a.generate_code_spec(settings),
            DelimitedExpression::Field(a) => a.generate_code_spec(settings),
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
            DelimitedExpression::OverrideField(a) => a.get_fields(),
            DelimitedExpression::Field(a) => a.get_fields(),
        }
    }
}

impl Codegen for Group {
    fn generate_code_spec(&self, settings: &CodegenSettings) -> Result<TokenStream> {
        self.body.generate_code_spec(settings)
    }

    fn get_fields(&self) -> Result<Vec<FieldDescriptor>> {
        self.body.get_fields()
    }
}

impl Codegen for Optional {
    fn generate_code_spec(&self, settings: &CodegenSettings) -> Result<TokenStream> {
        let body = self.body.generate_code(settings)?;
        let inner_fields = self.body.get_fields()?;
        let runtime_prefix = &settings.runtime_prefix;
        let happy_case_fields: TokenStream = inner_fields
            .iter()
            .map(|inner_field| {
                let name = format_ident!("{}", inner_field.name);
                // TODO: enum conversion
                let value = match &inner_field.arity {
                    Arity::One => quote!(Some(result.#name)),
                    Arity::Optional => quote!(result.#name),
                    Arity::Multiple => quote!(result.#name),
                };
                quote!(#name: #value,)
            })
            .collect();
        let unhappy_case_fields: TokenStream = inner_fields
            .iter()
            .map(|inner_field| {
                let name = format_ident!("{}", inner_field.name);
                // TODO: enum conversion
                let value = match &inner_field.arity {
                    Arity::One => quote!(None),
                    Arity::Optional => quote!(None),
                    Arity::Multiple => quote!(Vec::new()),
                };
                quote!(#name: #value,)
            })
            .collect();
        Ok(quote!(
            mod optional{
                use #runtime_prefix *;
                #body
            }
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                if let Ok((result, new_state)) = optional::parse(state.clone()) {
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
    fn generate_code_spec(&self, settings: &CodegenSettings) -> Result<TokenStream> {
        let closure_body = self.body.generate_code(settings)?;
        let runtime_prefix = &settings.runtime_prefix;
        let fields = self.get_fields()?;
        let inner_fields = self.body.get_fields()?;
        let declarations: TokenStream = fields
            .iter()
            .map(|f| {
                let typ = generate_field_type(f, settings);
                let name_ident = format_ident!("{}", f.name);
                quote!(let mut #name_ident: #typ = Vec::new();)
            })
            .collect();
        let assignments: TokenStream = inner_fields
            .iter()
            .map(|inner_field| {
                let name = format_ident!("{}", inner_field.name);
                // TODO: Enum conversion
                match &inner_field.arity {
                    Arity::One => quote!(#name.push(result.#name);),
                    Arity::Optional => quote!(
                        if let Some(result_inner) = result.#name {
                            vec![result_inner]
                        } else {
                            vec![]
                        }
                    ),
                    Arity::Multiple => quote!(#name.extend(result.#name);),
                }
            })
            .collect();
        let field_names: Vec<Ident> = fields.iter().map(|f| format_ident!("{}", f.name)).collect();
        let parse_result = if fields.is_empty() {
            quote!(())
        } else {
            quote!(Parsed{ #( #field_names,)* })
        };
        let at_least_one_body = if self.at_least_one.is_some() {
            quote!(
                let (result, new_state) = closure::parse(state)?;
                #assignments
                state = new_state;
            )
        } else {
            quote!()
        };

        Ok(quote!(
            mod closure{
                use #runtime_prefix *;
                #closure_body
            }
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                let mut state = state;
                #declarations
                #at_least_one_body
                while let Ok((result, new_state)) = closure::parse(state.clone()) {
                    #assignments
                    state = new_state;
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
    fn generate_code_spec(&self, settings: &CodegenSettings) -> Result<TokenStream> {
        let body = self.expr.generate_code(settings)?;
        let runtime_prefix = &settings.runtime_prefix;
        Ok(quote!(
            mod negative_lookahead{
                use #runtime_prefix *;
                #body
            }
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                match negative_lookahead::parse (state.clone()) {
                    Ok(_) => Err(ParseError),
                    Err(_) => Ok(((), state)),
                }
            }
        ))
    }

    fn get_fields(&self) -> Result<Vec<FieldDescriptor>> {
        Ok(Vec::new())
    }
}

impl Codegen for CharacterRange {
    fn generate_code_spec(&self, settings: &CodegenSettings) -> Result<TokenStream> {
        let from = &self.from;
        let to = &self.to;
        let skip_ws = if settings.skip_whitespace {
            quote!(let state = state.skip_whitespace();)
        } else {
            quote!()
        };
        Ok(quote!(
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                #skip_ws
                parse_character_range(state, #from, #to)
            }
        ))
    }

    fn get_fields(&self) -> Result<Vec<FieldDescriptor>> {
        Ok(Vec::new())
    }
}

impl Codegen for CharacterLiteral {
    fn generate_code_spec(&self, settings: &CodegenSettings) -> Result<TokenStream> {
        let literal = &self;
        let skip_ws = if settings.skip_whitespace {
            quote!(let state = state.skip_whitespace();)
        } else {
            quote!()
        };
        Ok(quote!(
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                #skip_ws
                parse_character_literal(state, #literal)
            }
        ))
    }

    fn get_fields(&self) -> Result<Vec<FieldDescriptor>> {
        Ok(Vec::new())
    }
}

impl Codegen for StringLiteral {
    fn generate_code_spec(&self, settings: &CodegenSettings) -> Result<TokenStream> {
        let literal = &self.body;
        let skip_ws = if settings.skip_whitespace {
            quote!(let state = state.skip_whitespace();)
        } else {
            quote!()
        };
        Ok(quote!(
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                #skip_ws
                parse_string_literal(state, #literal)
            }
        ))
    }

    fn get_fields(&self) -> Result<Vec<FieldDescriptor>> {
        Ok(Vec::new())
    }
}

impl Codegen for OverrideField {
    fn generate_code_spec(&self, settings: &CodegenSettings) -> Result<TokenStream> {
        let empty_prefix = TokenStream::new();
        let prefix = if self.typ == "char" {
            &empty_prefix
        } else {
            &settings.grammar_module_prefix
        };
        let parser_name = format_ident!("parse_{}", self.typ);
        let skip_ws = if settings.skip_whitespace {
            quote!(let state = state.skip_whitespace();)
        } else {
            quote!()
        };
        Ok(quote!(
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                #skip_ws
                let (_override, state) = #prefix #parser_name (state)?;
                Ok((Parsed{ _override }, state))
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
    fn generate_code_spec(&self, settings: &CodegenSettings) -> Result<TokenStream> {
        let empty_prefix = TokenStream::new();
        let prefix = if self.typ == "char" {
            &empty_prefix
        } else {
            &settings.grammar_module_prefix
        };
        let parser_name = format_ident!("parse_{}", self.typ);
        let skip_ws = if settings.skip_whitespace {
            quote!(let state = state.skip_whitespace();)
        } else {
            quote!()
        };
        if let Some(field_name) = &self.name {
            let field_ident = format_ident!("{}", field_name);
            let maybe_boxed = if self.boxed.is_some() {
                quote!(#field_ident: Box::new(#field_ident))
            } else {
                quote!(#field_ident)
            };
            Ok(quote!(
                pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                    #skip_ws
                    let (#field_ident, state) = #prefix #parser_name (state)?;
                    Ok((Parsed{ #maybe_boxed }, state))
                }
            ))
        } else {
            Ok(quote!(
                pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                    #skip_ws
                    let (_, state) = #prefix #parser_name (state)?;
                    Ok(((), state))
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

pub fn lets_debug(grammar: &Grammar) -> Result<()> {
    let settings = CodegenSettings {
        grammar_module_prefix: quote!(crate::grammar::generated::),
        runtime_prefix: quote!(crate::runtime::),
        skip_whitespace: true,
    };
    println!("use crate::runtime::*;");
    println!("{}", grammar.generate_code(&settings)?);
    Ok(())
}
