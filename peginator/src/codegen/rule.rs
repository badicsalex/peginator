use anyhow::{bail, Result};
use proc_macro2::{Ident, TokenStream};
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
        let flags = self.flags();
        let settings = CodegenSettings {
            skip_whitespace: settings.skip_whitespace && !flags.no_skip_ws,
            ..Default::default()
        };

        let fields = self.definition.get_fields()?;
        let is_override = fields.len() == 1 && fields[0].name == "_override";

        Self::check_flags(&flags, is_override)?;

        let name = &self.name;
        let rule_mod = format_ident!("{}_impl", self.name);
        let rule_type = format_ident!("{}", self.name);
        let parser_name = format_ident!("parse_{}", self.name);
        let cache_entry_ident = format_ident!("c_{}", self.name);
        let choice_body = self.definition.generate_code(&fields, &settings)?;

        let (types, inner_code) = if flags.string {
            self.generate_string_rule(&settings)?
        } else if is_override {
            self.generate_override_rule(&fields, &settings)?
        } else {
            self.generate_normal_rule(&fields, &settings, flags.position)?
        };

        Ok((
            flags.export,
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
                        state.print_trace_cached(#name);
                        state.print_trace_result(&cached);
                        cached.clone()
                    } else {
                        state.print_trace_start(#name);
                        let result = #rule_mod::rule_parser(state.clone().indent(), cache);
                        state.print_trace_result(&result);
                        let result = result.map(|result| ParseOk {
                            state: result.state.dedent(),
                            ..result
                        });
                        cache.#cache_entry_ident.insert(cache_key, result.clone());
                        result
                    }
                }
            ),
        ))
    }
}

#[derive(Debug, Default)]
struct RuleFlags {
    pub no_skip_ws: bool,
    pub export: bool,
    pub string: bool,
    pub position: bool,
}

impl Rule {
    fn flags(&self) -> RuleFlags {
        let mut result = RuleFlags::default();
        for directive in &self.directives {
            match directive {
                DirectiveExpression::StringDirective(_) => result.string = true,
                DirectiveExpression::NoSkipWsDirective(_) => result.no_skip_ws = true,
                DirectiveExpression::ExportDirective(_) => result.export = true,
                DirectiveExpression::PositionDirective(_) => result.position = true,
            }
        }
        result
    }

    fn check_flags(flags: &RuleFlags, is_override: bool) -> Result<()> {
        if flags.export && is_override {
            bail!("Overridden (containing '@:') rules cannot be @export-ed");
        }
        if flags.export && flags.string {
            bail!("@string rules cannot be @export-ed");
        }
        if flags.position && is_override {
            bail!("Overridden (containing '@:') rules cannot contain @position");
        }
        if flags.position && flags.string {
            bail!("@string rules cannot contain @position");
        }
        Ok(())
    }

    fn generate_string_rule(
        &self,
        _settings: &CodegenSettings,
    ) -> Result<(TokenStream, TokenStream)> {
        let rule_type = format_ident!("{}", self.name);
        Ok((
            quote!(pub type #rule_type = String;),
            quote!(
                #[inline(always)]
                pub fn rule_parser<'a>(
                    state: ParseState<'a>,
                    cache: &mut ParseCache<'a>,
                ) -> ParseResult<'a, String> {
                    let ok_result = parse(state.clone(), cache)?;
                    Ok(ok_result
                        .map_with_state(|_, new_state| state.slice_until(&new_state).to_string()))
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
        record_position: bool,
    ) -> Result<(TokenStream, TokenStream)> {
        let rule_type = format_ident!("{}", self.name);
        let parsed_enum_types: TokenStream = fields
            .iter()
            .filter(|f| f.type_names.len() > 1)
            .map(|f| generate_enum_type(&self.name, f, settings))
            .collect();
        let parsed_struct_type =
            self.definition
                .generate_struct_type(fields, settings, &self.name, record_position)?;
        let inner_enum_uses: TokenStream = fields
            .iter()
            .filter(|f| f.type_names.len() > 1)
            .map(|f| {
                let outer_name = format_ident!("{}_{}", self.name, f.name);
                let inner_name = format_ident!("Parsed_{}", f.name);
                quote!(use super::#outer_name as #inner_name;)
            })
            .collect();
        let field_names: Vec<Ident> = fields.iter().map(|f| format_ident!("{}", f.name)).collect();

        let rule_parser_body = if record_position {
            quote!(
                let ok_result = parse(state.clone(), cache)?;
                Ok(ok_result.map_with_state(
                    |r, new_state| super::#rule_type{
                        #( #field_names:r.#field_names,)*
                        position: state.range_until(new_state),
                    }
                ))
            )
        } else {
            quote!(
                let ok_result = parse(state, cache)?;
                Ok(ok_result.map(
                    |r| super::#rule_type{
                        #( #field_names:r.#field_names,)*
                    }
                ))
            )
        };
        Ok((
            quote!(
                #parsed_struct_type
                #parsed_enum_types
            ),
            quote!(
                #inner_enum_uses
                #[inline(always)]
                pub fn rule_parser <'a>(state: ParseState<'a>, cache: &mut ParseCache<'a>) -> ParseResult<'a, super::#rule_type> {
                    #rule_parser_body
                }
            ),
        ))
    }
}
