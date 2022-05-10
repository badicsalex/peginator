use anyhow::Result;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::grammar::Grammar;

use super::common::{CodegenGrammar, CodegenRule, CodegenSettings};

impl CodegenGrammar for Grammar {
    fn generate_code(&self, settings: &CodegenSettings) -> Result<TokenStream> {
        let mut all_types = TokenStream::new();
        let mut all_parsers = TokenStream::new();
        let mut all_impls = TokenStream::new();
        let mut cache_entries = TokenStream::new();
        let peginator_crate = format_ident!("{}", settings.peginator_crate_name);
        for rule in &self.rules {
            let (exported, types, impls) = rule.generate_code(settings)?;
            all_types.extend(types);
            all_impls.extend(impls);
            let rule_ident = format_ident!("{}", rule.name);
            let internal_parser_name = format_ident!("parse_{}", rule.name);
            if exported {
                all_parsers.extend(quote!(
                    impl peginator_generated::PegParser for #rule_ident {
                        fn parse_advanced(
                            s: &str,
                            settings: &peginator_generated::ParseSettings)
                        -> Result<Self, peginator_generated::ParseError> {
                            Ok(peginator_generated::#internal_parser_name(
                                peginator_generated::ParseState::new(s, settings),
                                &mut Default::default(),
                            )?.result)
                        }
                    }
                ))
            }

            let cache_entry_ident = format_ident!("c_{}", rule.name);
            cache_entries.extend(quote!(pub #cache_entry_ident: CacheEntries<'a, #rule_ident>,));
        }
        Ok(quote!(
            #all_types
            #all_parsers
            #[allow(
                non_snake_case,
                unused_variables,
                unused_imports,
                unused_mut,
            )]
            mod peginator_generated {
                use super::*;
                pub use #peginator_crate::runtime::{ParseError, ParseSettings, ParseState, PegParser};
                use #peginator_crate::runtime::*;

                #[derive(Default)]
                pub struct ParseCache<'a> {
                    #cache_entries
                }
                #all_impls
            }
        ))
    }
}
