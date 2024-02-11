// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

mod generated;

use std::str::FromStr;

use anyhow::{Context, Result};
pub use generated::*;
use peginator::{ParseError, PegParser};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use super::common::{safe_ident, CodegenGrammar, CodegenRule, CodegenSettings};

impl CodegenGrammar for Grammar {
    fn generate_code(&self, settings: &CodegenSettings) -> Result<TokenStream> {
        let mut all_types = TokenStream::new();
        let mut all_parsers = TokenStream::new();
        let mut all_impls = TokenStream::new();
        let mut cache_entries = TokenStream::new();
        for rule_entry in &self.rules {
            match rule_entry {
                Grammar_rules::Rule(rule) => {
                    let flags = rule.flags();
                    let (types, impls) = rule
                        .generate_code(self, settings)
                        .with_context(|| format!("Error processing rule {}", rule.name))?;
                    all_types.extend(types);
                    all_impls.extend(impls);
                    let rule_ident = safe_ident(&rule.name);
                    let internal_parser_name = format_ident!("parse_{}", rule.name);
                    let user_context_type = &settings.user_context_type;
                    if flags.export {
                        all_parsers.extend(quote!(
                            impl peginator_generated::PegParserAdvanced<#user_context_type> for #rule_ident {
                                fn parse_advanced<TT: peginator_generated::ParseTracer>(
                                    s: &str,
                                    settings: &peginator_generated::ParseSettings,
                                    user_context: #user_context_type,
                                ) -> Result<Self, peginator_generated::ParseError> {
                                    Ok(peginator_generated::#internal_parser_name(
                                        peginator_generated::ParseState::new(s, settings),
                                        &mut peginator_generated
                                            ::ParseGlobal
                                            ::<TT, peginator_generated::ParseCache, #user_context_type>
                                            ::new(
                                                Default::default(),
                                                user_context,
                                        ),
                                    )?.result)
                                }
                            }
                        ))
                    }

                    if flags.memoize || flags.left_recursive {
                        let cache_entry_ident = format_ident!("c_{}", rule.name);
                        cache_entries
                            .extend(quote!(pub #cache_entry_ident: CacheEntries<'a, #rule_ident>,));
                    }
                }
                Grammar_rules::CharRule(rule) => {
                    let rule_ident = safe_ident(&rule.name);
                    all_types.extend(quote!(pub type #rule_ident = char;));
                    all_impls.extend(
                        rule.generate_code(settings).with_context(|| {
                            format!("Error processing @char rule {}", rule.name)
                        })?,
                    );
                }
                Grammar_rules::ExternRule(rule) => {
                    let (types, impls) = rule
                        .generate_code(settings)
                        .with_context(|| format!("Error processing @extern rule {}", rule.name))?;
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
                pub use peginator::{
                    ParseError, ParseSettings, ParseState, PegParser, IndentedTracer, ParseTracer,
                    PegPosition, ParseGlobal, PegParserAdvanced,
                };
                use peginator::*;

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

impl FromStr for Grammar {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        PegParser::parse(s)
    }
}
