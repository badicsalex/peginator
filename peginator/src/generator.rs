// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

use std::collections::{HashMap, HashSet};

use crate::grammar::{
    Choice, Closure, ClosureAtLeastOne, DetailedExpression, Field, Grammar, Group, OverrideField,
    Rule, Sequence,
};

use anyhow::Result;

use proc_macro2::TokenStream;
use quote::quote;

#[derive(Debug)]
enum Arity {
    One,
    Optional,
    Multiple,
}

impl Arity {}

#[derive(Debug)]
struct StructField<'a> {
    type_names: HashSet<&'a str>,
    arity: Arity,
}

type StructFields<'a> = HashMap<&'a str, StructField<'a>>;

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
                    arity: Arity::One,
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
                arity: Arity::One,
            },
        )]))
    }
}

pub fn lets_debug(grammar: &Grammar) {
    for rule in &grammar.rules {
        println!("{}:", rule.name);
        let fields = rule.definition.extract_fields();
        println!("{:?}", fields);
        println!();
    }
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
