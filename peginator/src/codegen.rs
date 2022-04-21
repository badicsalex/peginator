// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

use std::collections::BTreeSet;

use crate::grammar::*;

use anyhow::{anyhow, Result};
use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote};

fn quick_ident(s: &str) -> Ident {
    Ident::new(s, Span::call_site())
}

struct CodegenSettings {
    grammar_module_prefix: TokenStream,
    runtime_prefix: TokenStream,
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

trait CodegenOuter {
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
        let rule_mod = format_ident!("{}_impl", self.name);
        let rule_type = format_ident!("{}", self.name);
        let parser_name = format_ident!("parse_{}", self.name);
        let runtime_prefix = &settings.runtime_prefix;
        let choice_body = self.definition.generate_code(settings)?;
        Ok(quote!(
            mod #rule_mod{
                use #runtime_prefix *;
                #choice_body
            }
            pub use #rule_mod::Parsed as #rule_type;
            pub use #rule_mod::parse as #parser_name;
        ))
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
            pub struct Parsed {
                #( #field_names: #field_types, )*
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
        Ok(quote!(
            #choice_bodies
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                // TODO
                Err(ParseError)
            }
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
            return Ok(quote!());
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
        Ok(quote!(
            #part_bodies
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                // TODO
                Err(ParseError)
            }
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

impl Codegen for DelimitedExpression {
    fn generate_code_spec(&self, settings: &CodegenSettings) -> Result<TokenStream> {
        match self {
            DelimitedExpression::Group(a) => a.generate_code_spec(settings),
            DelimitedExpression::ClosureAtLeastOne(a) => a.generate_code_spec(settings),
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
            DelimitedExpression::ClosureAtLeastOne(a) => a.get_fields(),
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

impl Codegen for Closure {
    fn generate_code_spec(&self, settings: &CodegenSettings) -> Result<TokenStream> {
        let closure_body = self.body.generate_code(settings)?;
        let runtime_prefix = &settings.runtime_prefix;
        Ok(quote!(
            mod closure{
                use #runtime_prefix *;
                #closure_body
            }
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                // TODO: turn the Vec<Parsed> inside-out
                parse_closure(state, closure::parse, 1);
                Err(ParseError)
            }
        ))
    }

    fn get_fields(&self) -> Result<Vec<FieldDescriptor>> {
        Ok(set_arity_to_multiple(self.body.get_fields()?))
    }
}

impl Codegen for ClosureAtLeastOne {
    fn generate_code_spec(&self, settings: &CodegenSettings) -> Result<TokenStream> {
        let closure_body = self.body.generate_code(settings)?;
        let runtime_prefix = &settings.runtime_prefix;
        Ok(quote!(
            mod closure{
                use #runtime_prefix *;
                #closure_body
            }
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                // TODO: turn the Vec<Parsed> inside-out
                parse_closure(state, closure::parse, 1);
                Err(ParseError)
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
    fn generate_code_spec(&self, _settings: &CodegenSettings) -> Result<TokenStream> {
        let from = &self.from;
        let to = &self.to;
        Ok(quote!(
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                parse_character_range(state, #from, #to)
            }
        ))
    }

    fn get_fields(&self) -> Result<Vec<FieldDescriptor>> {
        Ok(Vec::new())
    }
}

impl Codegen for CharacterLiteral {
    fn generate_code_spec(&self, _settings: &CodegenSettings) -> Result<TokenStream> {
        let literal = &self;
        Ok(quote!(
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                parse_character_literal(state, #literal)
            }
        ))
    }

    fn get_fields(&self) -> Result<Vec<FieldDescriptor>> {
        Ok(Vec::new())
    }
}

impl Codegen for StringLiteral {
    fn generate_code_spec(&self, _settings: &CodegenSettings) -> Result<TokenStream> {
        let literal = &self.body;
        Ok(quote!(
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
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
        Ok(quote!(
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
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
        if let Some(field_name) = &self.name {
            let field_ident = format_ident!("{}", field_name);
            let maybe_boxed = if (self.boxed.is_some()) {
                quote!(#field_ident: Box::new(#field_ident))
            } else {
                quote!(#field_ident)
            };
            Ok(quote!(
                pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                    let (#field_ident, state) = #prefix #parser_name (state)?;
                    Ok((Parsed{ #maybe_boxed }, state))
                }
            ))
        } else {
            Ok(quote!(
                pub fn parse(state: ParseState) -> ParseResult<Parsed> {
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
        grammar_module_prefix: quote!(crate::grammar::test::),
        runtime_prefix: quote!(crate::runtime::),
    };
    println!("{}", grammar.generate_code(&settings)?);
    Ok(())
}
