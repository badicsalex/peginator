// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

use anyhow::{bail, Result};
use proc_macro2::TokenStream;
use quote::quote;

use super::common::{generate_inner_parse_function, Codegen, CodegenSettings, FieldDescriptor};
use crate::grammar::{Grammar, NegativeLookahead, PositiveLookahead};

impl Codegen for NegativeLookahead {
    fn generate_code_spec(
        &self,
        rule_fields: &[FieldDescriptor],
        grammar: &Grammar,
        settings: &CodegenSettings,
    ) -> Result<TokenStream> {
        let body = self.expr.generate_code(rule_fields, grammar, settings)?;
        let parse_body = quote!(match negative_lookahead::parse(state.clone(), global) {
            Ok(_) => Err(state.report_error(ParseErrorSpecifics::NegativeLookaheadFailed)),
            Err(_) => Ok(ParseOk { result: (), state }),
        });
        let parse_function = generate_inner_parse_function(parse_body, settings);
        Ok(quote!(
            mod negative_lookahead{
                use super::*;
                #body
            }
            #parse_function
        ))
    }

    fn get_fields(&self, grammar: &Grammar) -> Result<Vec<FieldDescriptor>> {
        if !self.expr.get_fields(grammar)?.is_empty() {
            bail!("The body of negative lookaheads should not contain named fields")
        }
        Ok(Vec::new())
    }
}

impl Codegen for PositiveLookahead {
    fn generate_code_spec(
        &self,
        rule_fields: &[FieldDescriptor],
        grammar: &Grammar,
        settings: &CodegenSettings,
    ) -> Result<TokenStream> {
        let body = self.expr.generate_code(rule_fields, grammar, settings)?;
        let parse_body = quote!(
            positive_lookahead::parse (state.clone(), global)?;
            Ok(ParseOk{result:(), state})
        );
        let parse_function = generate_inner_parse_function(parse_body, settings);
        Ok(quote!(
            mod positive_lookahead{
                use super::*;
                #body
            }
            #parse_function
        ))
    }

    fn get_fields(&self, grammar: &Grammar) -> Result<Vec<FieldDescriptor>> {
        if !self.expr.get_fields(grammar)?.is_empty() {
            bail!("The body of positive lookaheads should not contain named fields")
        }
        Ok(Vec::new())
    }
}
