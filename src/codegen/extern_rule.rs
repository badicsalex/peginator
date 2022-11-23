// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

use anyhow::Result;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use super::common::{generate_rule_parse_function, safe_ident};
use crate::grammar::ExternRule;

impl ExternRule {
    pub fn generate_code(&self) -> Result<(TokenStream, TokenStream)> {
        let result_type = if self.directive.type_parts.is_empty() {
            quote!(String)
        } else {
            let part_idents = self.directive.type_parts.iter().map(safe_ident);
            quote!(#(#part_idents)::*)
        };
        let name_idents = self.directive.name_parts.iter().map(safe_ident);
        let function_ident = quote!(#(#name_idents)::*);

        let rule_type = safe_ident(&self.name);
        let parser_name = format_ident!("parse_{}", self.name);

        let parse_body = quote!(
            match #function_ident(state.s()) {
                Ok((result, advance)) => {
                    Ok(ParseOk {
                        result: result.into(),
                        state: state.advance_safe(advance),
                    })
                },
                Err(error_string) => {
                    Err(state.report_error(ParseErrorSpecifics::ExternRuleFailed {
                        error_string,
                    }))
                }
            }
        );
        Ok((
            quote!(pub type #rule_type = #result_type;),
            generate_rule_parse_function(parser_name, rule_type, parse_body),
        ))
    }
}
