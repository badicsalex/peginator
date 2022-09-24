// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

use crate::codegen::safe_ident;
use crate::grammar::ExternRule;
use anyhow::Result;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

impl ExternRule {
    pub fn generate_code(&self) -> Result<(TokenStream, TokenStream)> {
        let result_type = if self.directive.type_parts.is_empty() {
            quote!(String)
        } else {
            let part_idents = self.directive.type_parts.iter().map(|p| safe_ident(p));
            quote!(#(#part_idents)::*)
        };
        let name_idents = self.directive.name_parts.iter().map(|p| safe_ident(p));
        let function_ident = quote!(#(#name_idents)::*);
        let function_name = self.directive.name_parts.join("::");

        let rule_ident = safe_ident(&self.name);
        let parser_name = format_ident!("parse_{}", self.name);

        Ok((
            quote!(pub type #rule_ident = #result_type;),
            quote!(
            #[inline]
                pub(super) fn #parser_name <'a>(
                    state: ParseState<'a>,
                    tracer: impl ParseTracer,
                    cache: &mut ParseCache<'a>
                ) -> ParseResult<'a, #rule_ident> {
                    match #function_ident(state.s()) {
                        Ok((result, advance)) => {
                            Ok(ParseOk {
                                result: result.into(),
                                state: state.advance_safe(advance),
                                farthest_error: None,
                            })
                        },
                        Err(error_string) => {
                            Err(state.report_error(ParseErrorSpecifics::ExternRuleFailed {
                                function_name: #function_name,
                                error_string,
                            }))
                        }
                    }
                }
            ),
        ))
    }
}
