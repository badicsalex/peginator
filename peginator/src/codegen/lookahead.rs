use anyhow::Result;
use proc_macro2::TokenStream;
use quote::quote;

use crate::grammar::NegativeLookahead;

use super::common::{Codegen, CodegenSettings, FieldDescriptor};

impl Codegen for NegativeLookahead {
    fn generate_code_spec(
        &self,
        rule_fields: &[FieldDescriptor],
        settings: &CodegenSettings,
    ) -> Result<TokenStream> {
        let body = self.expr.generate_code(rule_fields, settings)?;
        Ok(quote!(
            mod negative_lookahead{
                use super::*;
                #body
            }
            #[inline(always)]
            pub fn parse<'a>(state: ParseState<'a>, cache: &mut ParseCache<'a>) -> ParseResult<'a, Parsed> {
                match negative_lookahead::parse (state.clone(), cache) {
                    Ok(_) => Err(state.report_error(ParseErrorSpecifics::NegativeLookaheadFailed)),
                    Err(_) => Ok(ParseOk{result:Parsed, state, farthest_error:None}),
                }
            }
        ))
    }

    fn get_fields(&self) -> Result<Vec<FieldDescriptor>> {
        // TODO: check if body has fields. It should NOT, since capturing in a negative lookeahead
        // is not supported at all.
        Ok(Vec::new())
    }
}