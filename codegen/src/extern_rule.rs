// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

use anyhow::Result;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use super::{
    common::{generate_rule_parse_function, safe_ident},
    CodegenSettings,
};
use crate::grammar::ExternRule;

impl ExternRule {
    pub fn generate_code(&self, settings: &CodegenSettings) -> Result<(TokenStream, TokenStream)> {
        let return_type = if let Some(return_type) = &self.directive.return_type {
            let part_idents = return_type.iter().map(safe_ident);
            quote!(#(#part_idents)::*)
        } else {
            quote!(String)
        };
        let name_idents = self.directive.function.iter().map(safe_ident);
        let function_ident = quote!(#(#name_idents)::*);

        let rule_type = safe_ident(&self.name);
        let parser_name = format_ident!("parse_{}", self.name);
        let user_context_param = if settings.has_user_context {
            quote!(, global.user_context)
        } else {
            quote!()
        };

        let parse_body = quote!(
            match #function_ident(state.s() #user_context_param) {
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
            quote!(pub type #rule_type = #return_type;),
            generate_rule_parse_function(parser_name, rule_type, parse_body, settings),
        ))
    }
}
