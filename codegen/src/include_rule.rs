// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

use anyhow::Result;
use proc_macro2::TokenStream;

use super::common::{Codegen, CodegenSettings, FieldDescriptor};
use crate::grammar::{Choice, Grammar, Grammar_rules, IncludeRule};

impl Codegen for IncludeRule {
    fn generate_code_spec(
        &self,
        rule_fields: &[FieldDescriptor],
        grammar: &Grammar,
        settings: &CodegenSettings,
    ) -> Result<TokenStream> {
        self.included_rule_definition(grammar)?
            .generate_code_spec(rule_fields, grammar, settings)
    }

    fn get_fields<'a>(&'a self, grammar: &'a Grammar) -> Result<Vec<FieldDescriptor<'a>>> {
        self.included_rule_definition(grammar)?.get_fields(grammar)
    }
}

impl IncludeRule {
    fn included_rule_definition<'a>(&'a self, grammar: &'a Grammar) -> Result<&'a Choice> {
        Ok(&grammar
            .rules
            .iter()
            .find_map(|r| {
                if let Grammar_rules::Rule(r) = r {
                    if r.name == self.rule {
                        Some(r)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "Could not find normal (not char or extern) rule named {}",
                    self.rule
                )
            })?
            .definition)
    }
}
