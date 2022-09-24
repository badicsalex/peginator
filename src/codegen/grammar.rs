// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

use anyhow::Result;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use super::common::{safe_ident, CodegenGrammar, CodegenRule, CodegenSettings};
use crate::grammar::{Grammar, Grammar_rules};

impl CodegenGrammar for Grammar {
    fn generate_code(&self, settings: &CodegenSettings) -> Result<TokenStream> {
        let mut all_types = TokenStream::new();
        let mut all_parsers = TokenStream::new();
        let mut all_impls = TokenStream::new();
        let mut cache_entries = TokenStream::new();
        let peginator_crate = safe_ident(&settings.peginator_crate_name);
        for rule_entry in &self.rules {
            match rule_entry {
                Grammar_rules::Rule(rule) => {
                    let flags = rule.flags();
                    let (types, impls) = rule.generate_code(self, settings)?;
                    all_types.extend(types);
                    all_impls.extend(impls);
                    let rule_ident = safe_ident(&rule.name);
                    let internal_parser_name = format_ident!("parse_{}", rule.name);
                    if flags.export {
                        all_parsers.extend(quote!(
                            impl peginator_generated::PegParser for #rule_ident {
                                fn parse_advanced<T: peginator_generated::ParseTracer>(
                                    s: &str,
                                    settings: &peginator_generated::ParseSettings)
                                -> Result<Self, peginator_generated::ParseError> {
                                    Ok(peginator_generated::#internal_parser_name(
                                        peginator_generated::ParseState::new(s, settings),
                                        T::new(),
                                        &mut Default::default(),
                                    )?.result)
                                }
                            }
                        ))
                    }

                    if flags.memoize {
                        let cache_entry_ident = format_ident!("c_{}", rule.name);
                        cache_entries
                            .extend(quote!(pub #cache_entry_ident: CacheEntries<'a, #rule_ident>,));
                    }
                }
                Grammar_rules::CharRule(char_rule) => {
                    let rule_ident = safe_ident(&char_rule.name);
                    all_types.extend(quote!(pub type #rule_ident = char;));
                    all_impls.extend(char_rule.generate_code());
                }
                Grammar_rules::ExternRule(extern_rule) => {
                    let (types, impls) = extern_rule.generate_code()?;
                    all_types.extend(types);
                    all_impls.extend(impls);
                }
            }
        }
        Ok(quote!(
            #all_types
            #all_parsers
            #[allow(
                non_snake_case,
                unused_variables,
                unused_imports,
                unused_mut,
                dead_code,
            )]
            mod peginator_generated {
                use super::*;
                pub use #peginator_crate::runtime::{
                    ParseError, ParseSettings, ParseState, PegParser, IndentedTracer, ParseTracer
                };
                use #peginator_crate::runtime::*;

                #[derive(Default)]
                pub struct ParseCache<'a> {
                    #cache_entries
                    _please_dont_complain: std::marker::PhantomData<&'a ()>,
                }
                #all_impls
            }
        ))
    }
}
