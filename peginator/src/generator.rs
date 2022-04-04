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

impl Default for Arity {
    fn default() -> Self {
        Arity::One
    }
}

#[derive(Debug, Default)]
struct StructField<'a> {
    type_names: HashSet<&'a str>,
    boxed: bool,
    arity: Arity,
}

type StructFields<'a> = HashMap<&'a str, StructField<'a>>;

struct ParsedRule<'a> {
    rule: &'a Rule,
    fields: StructFields<'a>,
}

/* This does not actually need to be a trait, I just wanted it for consistency */
trait FieldExtractor {
    fn extract_fields(&self) -> Result<StructFields>;
}

impl FieldExtractor for Choice {
    fn extract_fields(&self) -> Result<StructFields> {
        let mut all_fields = StructFields::new();
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
    fn extract_fields(&self) -> Result<StructFields> {
        let mut all_fields = StructFields::new();
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
    fn extract_fields(&self) -> Result<StructFields> {
        match self {
            DetailedExpression::Group(e) => e.extract_fields(),
            DetailedExpression::ClosureAtLeastOne(e) => e.extract_fields(),
            DetailedExpression::Closure(e) => e.extract_fields(),
            DetailedExpression::NegativeLookahead(_) => Ok(StructFields::new()),
            DetailedExpression::CharacterRange(_) => Ok(StructFields::new()),
            DetailedExpression::CharacterLiteral(_) => Ok(StructFields::new()),
            DetailedExpression::StringLiteral(_) => Ok(StructFields::new()),
            DetailedExpression::OverrideField(e) => e.extract_fields(),
            DetailedExpression::Field(e) => e.extract_fields(),
        }
    }
}

impl FieldExtractor for Group {
    fn extract_fields(&self) -> Result<StructFields> {
        self.body.extract_fields()
    }
}

impl FieldExtractor for Closure {
    fn extract_fields(&self) -> Result<StructFields> {
        Ok(set_arity_to_multiple(self.body.extract_fields()?))
    }
}

impl FieldExtractor for ClosureAtLeastOne {
    fn extract_fields(&self) -> Result<StructFields> {
        Ok(set_arity_to_multiple(self.body.extract_fields()?))
    }
}

fn set_arity_to_multiple(fields: StructFields) -> StructFields {
    let mut fields = fields;
    for value in fields.values_mut() {
        value.arity = Arity::Multiple;
    }
    fields
}

impl FieldExtractor for Field {
    fn extract_fields(&self) -> Result<StructFields> {
        match &self.name {
            Some(name) => Ok(StructFields::from([(
                name as &str,
                StructField {
                    type_names: HashSet::from([&self.typ as &str]),
                    ..Default::default()
                },
            )])),
            None => Ok(StructFields::new()),
        }
    }
}

impl FieldExtractor for OverrideField {
    fn extract_fields(&self) -> Result<StructFields> {
        Ok(StructFields::from([(
            "@",
            StructField {
                type_names: HashSet::from([&self.typ as &str]),
                ..Default::default()
            },
        )]))
    }
}

fn has_type_cycle<'a>(
    root_rule: &str,
    current_rule_name: &str,
    all_rules: &'a Vec<ParsedRule>,
    visited_rules: &mut HashSet<&'a str>,
) -> Result<bool> {
    if current_rule_name == "ANY_CHARACTER" {
        return Ok(false);
    }
    let current_rule = all_rules
        .iter()
        .find(|r| r.rule.name == current_rule_name)
        .ok_or_else(|| anyhow!("Unknown field type {}", current_rule_name))?;
    visited_rules.insert(&current_rule.rule.name);
    for field in current_rule.fields.values() {
        if let Arity::Multiple = field.arity {
            continue;
        }
        if field.boxed {
            continue;
        }
        for &type_name in &field.type_names {
            if type_name == root_rule {
                return Ok(true);
            }
            if visited_rules.contains(type_name) {
                continue;
            }
            if has_type_cycle(root_rule, type_name, all_rules, visited_rules)? {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

fn fields_that_have_cycles(rule: &ParsedRule, all_rules: &Vec<ParsedRule>) -> Result<Vec<String>> {
    let mut result = Vec::<String>::new();
    for (&field_name, field) in &rule.fields {
        if let Arity::Multiple = field.arity {
            continue;
        }

        let mut should_be_boxed = false;
        for type_name in &field.type_names {
            if has_type_cycle(&rule.rule.name, type_name, all_rules, &mut HashSet::new())? {
                should_be_boxed = true;
                break;
            }
        }
        if should_be_boxed {
            result.push(field_name.into())
        }
    }
    Ok(result)
}

fn generate_field_descriptors(grammar: &Grammar) -> Result<Vec<ParsedRule>> {
    let mut result = Vec::<ParsedRule>::new();
    for rule in &grammar.rules {
        result.push(ParsedRule {
            rule,
            fields: rule.definition.extract_fields()?,
        });
    }

    /* break cycles */
    for parsed_index in 0..result.len() {
        let cycle_names = fields_that_have_cycles(&result[parsed_index], &result)?;
        for field_name in cycle_names {
            let field = result[parsed_index]
                .fields
                .get_mut(&field_name as &str)
                .unwrap();
            field.boxed = true;
        }
    }
    Ok(result)
}

pub fn lets_debug(grammar: &Grammar) -> Result<()> {
    for parsed_rule in generate_field_descriptors(grammar)? {
        println!("{}:", parsed_rule.rule.name);
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
