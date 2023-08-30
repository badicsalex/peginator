// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

use anyhow::Result;
use proc_macro2::TokenStream;

use super::common::{CloneState, Codegen, CodegenSettings, FieldDescriptor};
use crate::grammar::{DelimitedExpression, Grammar, Group};

impl Codegen for DelimitedExpression {
    fn generate_code_spec(
        &self,
        rule_fields: &[FieldDescriptor],
        grammar: &Grammar,
        settings: &CodegenSettings,
    ) -> Result<TokenStream> {
        match self {
            DelimitedExpression::Group(a) => a.generate_code_spec(rule_fields, grammar, settings),
            DelimitedExpression::Optional(a) => {
                a.generate_code_spec(rule_fields, grammar, settings)
            }
            DelimitedExpression::Closure(a) => a.generate_code_spec(rule_fields, grammar, settings),
            DelimitedExpression::NegativeLookahead(a) => {
                a.generate_code_spec(rule_fields, grammar, settings)
            }
            DelimitedExpression::PositiveLookahead(a) => {
                a.generate_code_spec(rule_fields, grammar, settings)
            }
            DelimitedExpression::CharacterRange(a) => {
                a.generate_code_spec(rule_fields, grammar, settings)
            }
            DelimitedExpression::StringLiteral(a) => {
                a.generate_code_spec(rule_fields, grammar, settings)
            }
            DelimitedExpression::EndOfInput(a) => {
                a.generate_code_spec(rule_fields, grammar, settings)
            }
            DelimitedExpression::IncludeRule(a) => {
                a.generate_code_spec(rule_fields, grammar, settings)
            }
            DelimitedExpression::Field(a) => a.generate_code_spec(rule_fields, grammar, settings),
        }
    }

    fn generate_inline_body(
        &self,
        rule_fields: &[FieldDescriptor],
        grammar: &Grammar,
        settings: &CodegenSettings,
        clone_state: CloneState,
    ) -> Result<Option<TokenStream>> {
        match self {
            DelimitedExpression::CharacterRange(a) => {
                a.generate_inline_body(rule_fields, grammar, settings, clone_state)
            }
            DelimitedExpression::Closure(a) => {
                a.generate_inline_body(rule_fields, grammar, settings, clone_state)
            }
            DelimitedExpression::EndOfInput(a) => {
                a.generate_inline_body(rule_fields, grammar, settings, clone_state)
            }
            DelimitedExpression::Field(a) => {
                a.generate_inline_body(rule_fields, grammar, settings, clone_state)
            }
            DelimitedExpression::Group(a) => {
                a.generate_inline_body(rule_fields, grammar, settings, clone_state)
            }
            DelimitedExpression::IncludeRule(a) => {
                a.generate_inline_body(rule_fields, grammar, settings, clone_state)
            }
            DelimitedExpression::NegativeLookahead(a) => {
                a.generate_inline_body(rule_fields, grammar, settings, clone_state)
            }
            DelimitedExpression::Optional(a) => {
                a.generate_inline_body(rule_fields, grammar, settings, clone_state)
            }
            DelimitedExpression::PositiveLookahead(a) => {
                a.generate_inline_body(rule_fields, grammar, settings, clone_state)
            }
            DelimitedExpression::StringLiteral(a) => {
                a.generate_inline_body(rule_fields, grammar, settings, clone_state)
            }
        }
    }

    fn get_fields<'a>(&'a self, grammar: &'a Grammar) -> Result<Vec<FieldDescriptor<'a>>> {
        match self {
            DelimitedExpression::Group(a) => a.get_fields(grammar),
            DelimitedExpression::Optional(a) => a.get_fields(grammar),
            DelimitedExpression::Closure(a) => a.get_fields(grammar),
            DelimitedExpression::NegativeLookahead(a) => a.get_fields(grammar),
            DelimitedExpression::PositiveLookahead(a) => a.get_fields(grammar),
            DelimitedExpression::CharacterRange(a) => a.get_fields(grammar),
            DelimitedExpression::StringLiteral(a) => a.get_fields(grammar),
            DelimitedExpression::EndOfInput(a) => a.get_fields(grammar),
            DelimitedExpression::IncludeRule(a) => a.get_fields(grammar),
            DelimitedExpression::Field(a) => a.get_fields(grammar),
        }
    }
}

impl Codegen for Group {
    fn generate_code_spec(
        &self,
        rule_fields: &[FieldDescriptor],
        grammar: &Grammar,
        settings: &CodegenSettings,
    ) -> Result<TokenStream> {
        self.body.generate_code_spec(rule_fields, grammar, settings)
    }

    fn generate_inline_body(
        &self,
        rule_fields: &[FieldDescriptor],
        grammar: &Grammar,
        settings: &CodegenSettings,
        clone_state: CloneState,
    ) -> Result<Option<TokenStream>> {
        self.body
            .generate_inline_body(rule_fields, grammar, settings, clone_state)
    }

    fn get_fields<'a>(&'a self, grammar: &'a Grammar) -> Result<Vec<FieldDescriptor<'a>>> {
        self.body.get_fields(grammar)
    }
}
