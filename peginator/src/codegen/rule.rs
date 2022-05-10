use anyhow::{bail, Result};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::grammar::{DirectiveExpression, Rule};

use super::common::{
    generate_enum_type, generate_field_type, Codegen, CodegenRule, CodegenSettings, FieldDescriptor,
};

impl CodegenRule for Rule {
    fn generate_code(
        &self,
        settings: &CodegenSettings,
    ) -> Result<(bool, TokenStream, TokenStream)> {
        let mut string_flag = false;
        let mut skip_whitespace = settings.skip_whitespace;
        let mut export = false;
        for directive in &self.directives {
            match directive {
                DirectiveExpression::StringDirective(_) => string_flag = true,
                DirectiveExpression::NoSkipWsDirective(_) => skip_whitespace = false,
                DirectiveExpression::ExportDirective(_) => export = true,
            }
        }

        let settings = CodegenSettings {
            skip_whitespace,
            ..Default::default()
        };

        let fields = self.definition.get_fields()?;
        let is_override = fields.len() == 1 && fields[0].name == "_override";

        if export && is_override {
            bail!("Overridden (containing '@:') rules cannot be @export-ed");
        }
        if export && string_flag {
            bail!("@string rules cannot be @export-ed");
        }

        let name = &self.name;
        let rule_mod = format_ident!("{}_impl", self.name);
        let rule_type = format_ident!("{}", self.name);
        let parser_name = format_ident!("parse_{}", self.name);
        let cache_entry_ident = format_ident!("c_{}", self.name);
        let choice_body = self.definition.generate_code_spec(&fields, &settings)?;
        let (types, inner_code) = if string_flag {
            self.generate_string_rule(&fields, &settings)?
        } else if is_override {
            self.generate_override_rule(&fields, &settings)?
        } else {
            self.generate_normal_rule(&fields, &settings)?
        };
        Ok((
            export,
            types,
            quote!(
                mod #rule_mod{
                    use super::*;
                    #choice_body
                    #inner_code
                }
                #[inline]
                pub(super) fn #parser_name <'a>(state: ParseState<'a>, cache: &mut ParseCache<'a>) -> ParseResult<'a, #rule_type> {
                    let cache_key = state.cache_key();
                    if let Some(cached) = cache.#cache_entry_ident.get(&cache_key) {
                        cached.clone()
                    } else {
                        let result = run_rule_parser(#rule_mod::rule_parser, #name, state, cache);
                        cache.#cache_entry_ident.insert(cache_key, result.clone());
                        result
                    }
                }
            ),
        ))
    }
}

impl Rule {
    fn generate_string_rule(
        &self,
        fields: &[FieldDescriptor],
        settings: &CodegenSettings,
    ) -> Result<(TokenStream, TokenStream)> {
        let rule_type = format_ident!("{}", self.name);
        let parsed_types = self
            .definition
            .generate_struct_type(fields, settings, "Parsed")?;
        Ok((
            quote!(pub type #rule_type = String;),
            quote!(
                #parsed_types
                #[inline(always)]
                pub fn rule_parser <'a>(state: ParseState<'a>, cache: &mut ParseCache<'a>) -> ParseResult<'a, String> {
                    let ok_result = parse(state.clone(), cache)?;
                    let new_state = ok_result.state.clone();
                    Ok(ok_result.map(|_| state.slice_until(&new_state).to_string()))
                }
            ),
        ))
    }
    fn generate_override_rule(
        &self,
        fields: &[FieldDescriptor],
        settings: &CodegenSettings,
    ) -> Result<(TokenStream, TokenStream)> {
        let field = &fields[0];
        if field.type_names.len() <= 1 {
            self.generate_override_rule_simple(field, settings)
        } else {
            self.generate_override_rule_enum(field, settings)
        }
    }

    fn generate_override_rule_simple(
        &self,
        field: &FieldDescriptor,
        settings: &CodegenSettings,
    ) -> Result<(TokenStream, TokenStream)> {
        let rule_type = format_ident!("{}", self.name);
        let override_type = generate_field_type(&self.name, field, settings);
        Ok((
            quote!(
                pub use #override_type as #rule_type;
            ),
            quote!(
                pub struct Parsed {
                    _override: super::#rule_type,
                }
                use super::#rule_type as Parsed__override;
                #[inline(always)]
                pub fn rule_parser <'a>(state: ParseState<'a>, cache: &mut ParseCache<'a>) -> ParseResult<'a, super::#rule_type> {
                    let ok_result = parse(state, cache)?;
                    Ok(ok_result.map(|result| result._override))
                }
            ),
        ))
    }

    fn generate_override_rule_enum(
        &self,
        field: &FieldDescriptor,
        settings: &CodegenSettings,
    ) -> Result<(TokenStream, TokenStream)> {
        let rule_type = format_ident!("{}", self.name);
        let override_type = generate_field_type(&self.name, field, settings);
        let enum_type = generate_enum_type(&self.name, field, settings);
        Ok((
            quote!(
                #enum_type
                pub use #override_type as #rule_type;
            ),
            quote!(
                pub struct Parsed {
                    _override: super::#rule_type,
                }
                use super::#rule_type as Parsed__override;
                #[inline(always)]
                pub fn rule_parser <'a>(state: ParseState<'a>, cache: &mut ParseCache<'a>) -> ParseResult<'a, super::#rule_type> {
                    let ok_result = parse(state, cache)?;
                    Ok(ok_result.map(|result| result._override))
                }
            ),
        ))
    }

    fn generate_normal_rule(
        &self,
        fields: &[FieldDescriptor],
        settings: &CodegenSettings,
    ) -> Result<(TokenStream, TokenStream)> {
        let parsed_enum_types: TokenStream = fields
            .iter()
            .filter(|f| f.type_names.len() > 1)
            .map(|f| generate_enum_type(&self.name, f, settings))
            .collect();
        let parsed_struct_type = self
            .definition
            .generate_struct_type(fields, settings, &self.name)?;
        let used_types = self
            .definition
            .generate_use_super_as_parsed(settings, &self.name)?;
        Ok((
            quote!(
                #parsed_struct_type
                #parsed_enum_types
            ),
            quote!(
                #used_types
                #[inline(always)]
                pub fn rule_parser <'a>(state: ParseState<'a>, cache: &mut ParseCache<'a>) -> ParseResult<'a, Parsed> {
                    parse(state, cache)
                }
            ),
        ))
    }
}
