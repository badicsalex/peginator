// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

use anyhow::Result;
use proc_macro2::TokenStream;

use super::common::{Codegen, CodegenSettings, FieldDescriptor};
use crate::grammar::{DelimitedExpression, Group};

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
            DelimitedExpression::PositiveLookahead(a) => {
                a.generate_code_spec(rule_fields, settings)
            }
            DelimitedExpression::CharacterRange(a) => a.generate_code_spec(rule_fields, settings),
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
            DelimitedExpression::PositiveLookahead(a) => a.get_fields(),
            DelimitedExpression::CharacterRange(a) => a.get_fields(),
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
