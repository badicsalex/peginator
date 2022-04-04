// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

use std::collections::{HashMap, HashSet};

use crate::grammar::{
    Choice, Closure, ClosureAtLeastOne, DetailedExpression, Field, Grammar, Group, OverrideField,
    Rule, Sequence,
};

use anyhow::{anyhow, Result};

use proc_macro2::TokenStream;
use quote::quote;

#[derive(Debug)]
enum Arity {
    One,
    Optional,
    Multiple,
}

#[derive(Debug)]
struct ASTStructField<'a> {
    name: &'a str,
    type_names: Vec<&'a str>,
    arity: Arity,
    boxed: bool,
}

struct ASTStruct<'a> {
    name: &'a str,
    fields: Vec<ASTStructField<'a>>,
}

struct ExtractedFieldParams<'a> {
    type_names: HashSet<&'a str>,
    arity: Arity,
}

type ExtractedFields<'a> = HashMap<&'a str, ExtractedFieldParams<'a>>;

/* This does not actually need to be a trait, I just wanted it for consistency */
trait FieldExtractor {
    fn extract_fields(&self) -> Result<ExtractedFields>;
}

impl FieldExtractor for Choice {
    fn extract_fields(&self) -> Result<ExtractedFields> {
        let mut all_fields = ExtractedFields::new();
        let mut first_iteration = true;
        for choice in &self.choices {
            let new_fields = choice.extract_fields()?;

            if !first_iteration {
                for (field_name, field_value) in &mut all_fields {
                    if !new_fields.contains_key(field_name) {
                        field_value.arity = Arity::Optional;
                    }
                }
            }
            first_iteration = false;

            for (new_field_name, new_field) in new_fields {
                if let Some(original) = all_fields.get_mut(new_field_name) {
                    original.arity = combine_arities_for_choice(&original.arity, &new_field.arity);
                    original.type_names.extend(&new_field.type_names);
                } else {
                    all_fields.insert(new_field_name, new_field);
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

impl FieldExtractor for Sequence {
    fn extract_fields(&self) -> Result<ExtractedFields> {
        let mut all_fields = ExtractedFields::new();
        for part in &self.parts {
            let new_fields = part.extract_fields()?;
            for (new_field_name, new_field) in new_fields {
                if let Some(original) = all_fields.get_mut(new_field_name) {
                    original.arity = Arity::Multiple;
                    original.type_names.extend(&new_field.type_names);
                } else {
                    all_fields.insert(new_field_name, new_field);
                }
            }
        }
        Ok(all_fields)
    }
}

impl FieldExtractor for DetailedExpression {
    fn extract_fields(&self) -> Result<ExtractedFields> {
        match self {
            DetailedExpression::Group(e) => e.extract_fields(),
            DetailedExpression::ClosureAtLeastOne(e) => e.extract_fields(),
            DetailedExpression::Closure(e) => e.extract_fields(),
            DetailedExpression::NegativeLookahead(_) => Ok(ExtractedFields::new()),
            DetailedExpression::CharacterRange(_) => Ok(ExtractedFields::new()),
            DetailedExpression::CharacterLiteral(_) => Ok(ExtractedFields::new()),
            DetailedExpression::StringLiteral(_) => Ok(ExtractedFields::new()),
            DetailedExpression::OverrideField(e) => e.extract_fields(),
            DetailedExpression::Field(e) => e.extract_fields(),
        }
    }
}

impl FieldExtractor for Group {
    fn extract_fields(&self) -> Result<ExtractedFields> {
        self.body.extract_fields()
    }
}

impl FieldExtractor for Closure {
    fn extract_fields(&self) -> Result<ExtractedFields> {
        Ok(set_arity_to_multiple(self.body.extract_fields()?))
    }
}

impl FieldExtractor for ClosureAtLeastOne {
    fn extract_fields(&self) -> Result<ExtractedFields> {
        Ok(set_arity_to_multiple(self.body.extract_fields()?))
    }
}

fn set_arity_to_multiple(fields: ExtractedFields) -> ExtractedFields {
    let mut fields = fields;
    for value in fields.values_mut() {
        value.arity = Arity::Multiple;
    }
    fields
}

impl FieldExtractor for Field {
    fn extract_fields(&self) -> Result<ExtractedFields> {
        match &self.name {
            Some(name) => Ok(ExtractedFields::from([(
                name as &str,
                ExtractedFieldParams {
                    type_names: HashSet::from([&self.typ as &str]),
                    arity: Arity::One,
                },
            )])),
            None => Ok(ExtractedFields::new()),
        }
    }
}

impl FieldExtractor for OverrideField {
    fn extract_fields(&self) -> Result<ExtractedFields> {
        Ok(ExtractedFields::from([(
            "@",
            ExtractedFieldParams {
                type_names: HashSet::from([&self.typ as &str]),
                arity: Arity::One,
            },
        )]))
    }
}

fn has_type_cycle<'a>(
    root_ast_struct: &str,
    current_ast_struct_name: &str,
    all_ast_structs: &'a Vec<ASTStruct>,
    visited_ast_structs: &mut HashSet<&'a str>,
) -> Result<bool> {
    if current_ast_struct_name == "ANY_CHARACTER" {
        return Ok(false);
    }
    let current_ast_struct = all_ast_structs
        .iter()
        .find(|s| s.name == current_ast_struct_name)
        .ok_or_else(|| anyhow!("Unknown field type {}", current_ast_struct_name))?;
    visited_ast_structs.insert(current_ast_struct.name);
    for field in &current_ast_struct.fields {
        if let Arity::Multiple = field.arity {
            continue;
        }
        if field.boxed {
            continue;
        }
        for &type_name in &field.type_names {
            if type_name == root_ast_struct {
                return Ok(true);
            }
            if visited_ast_structs.contains(type_name) {
                continue;
            }
            if has_type_cycle(
                root_ast_struct,
                type_name,
                all_ast_structs,
                visited_ast_structs,
            )? {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

fn fields_that_have_cycles(
    ast_struct: &ASTStruct,
    all_ast_structs: &Vec<ASTStruct>,
) -> Result<Vec<String>> {
    let mut result = Vec::<String>::new();
    for field in &ast_struct.fields {
        if let Arity::Multiple = field.arity {
            continue;
        }

        let mut should_be_boxed = false;
        for type_name in &field.type_names {
            if has_type_cycle(
                ast_struct.name,
                type_name,
                all_ast_structs,
                &mut HashSet::new(),
            )? {
                should_be_boxed = true;
                break;
            }
        }
        if should_be_boxed {
            result.push(field.name.into())
        }
    }
    Ok(result)
}

fn generate_field_descriptors(grammar: &Grammar) -> Result<Vec<ASTStruct>> {
    let mut result = Vec::<ASTStruct>::new();
    for rule in &grammar.rules {
        let mut fields: Vec<ASTStructField> = rule
            .definition
            .extract_fields()?
            .into_iter()
            .map(|(field_name, desc)| {
                let mut type_names: Vec<&str> = desc.type_names.iter().copied().collect();
                type_names.sort_unstable();
                ASTStructField {
                    name: field_name,
                    type_names,
                    arity: desc.arity,
                    boxed: false,
                }
            })
            .collect();
        fields.sort_unstable_by_key(|f| f.name);
        result.push(ASTStruct {
            name: &rule.name,
            fields,
        });
    }

    /* break cycles */
    for parsed_index in 0..result.len() {
        let cycle_names = fields_that_have_cycles(&result[parsed_index], &result)?;
        for field_name in cycle_names {
            let field = result[parsed_index]
                .fields
                .iter_mut()
                .find(|f| f.name == field_name)
                .unwrap();
            field.boxed = true;
        }
    }
    Ok(result)
}

pub fn lets_debug(grammar: &Grammar) -> Result<()> {
    for parsed_rule in generate_field_descriptors(grammar)? {
        println!("{}:", parsed_rule.name);
        println!("{:?}", parsed_rule.fields);
        println!();
    }
    Ok(())
}

fn generate_single_type(rule: &Rule) -> Result<TokenStream> {
    let struct_name = &rule.name;
    let fields = rule.definition.extract_fields()?;
    let result = quote!(
        pub struct #struct_name;
    );
    Ok(result)
}
fn generate_types(grammar: &Grammar) -> Result<TokenStream> {
    let mut result = TokenStream::new();
    for rule in &grammar.rules {
        result.extend(generate_single_type(rule)?)
    }
    Ok(result)
}
