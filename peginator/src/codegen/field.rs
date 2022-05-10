use anyhow::Result;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::grammar::{Field, OverrideField};

use super::common::{Arity, Codegen, CodegenSettings, FieldDescriptor};

impl Codegen for Field {
    fn generate_code_spec(
        &self,
        rule_fields: &[FieldDescriptor],
        settings: &CodegenSettings,
    ) -> Result<TokenStream> {
        let parser_name = format_ident!("parse_{}", self.typ);
        let skip_ws = if settings.skip_whitespace {
            quote!(let state = state.skip_whitespace();)
        } else {
            quote!()
        };
        if let Some(field_name) = &self.name {
            let field_ident = format_ident!("{}", field_name);
            let field_conversion = generate_field_converter(field_name, &self.typ, rule_fields);
            Ok(quote!(
                #[inline(always)]
                pub fn parse<'a>(state: ParseState<'a>, cache: &mut ParseCache<'a>) -> ParseResult<'a, Parsed> {
                    #skip_ws
                    let ok_result = #parser_name (state, cache)?;
                    Ok(ok_result.map(|result| Parsed{ #field_ident: #field_conversion }))
                }
            ))
        } else {
            Ok(quote!(
                #[inline(always)]
                pub fn parse<'a>(state: ParseState<'a>, cache: &mut ParseCache<'a>) -> ParseResult<'a, Parsed> {
                    #skip_ws
                    let ok_result = #parser_name (state, cache)?;
                    Ok(ok_result.map(|_| Parsed))
                }
            ))
        }
    }

    fn get_fields(&self) -> Result<Vec<FieldDescriptor>> {
        if let Some(field_name) = &self.name {
            Ok(vec![FieldDescriptor {
                name: field_name,
                type_names: [&self.typ as &str].into(),
                arity: Arity::One,
                boxed: self.boxed.is_some(),
            }])
        } else {
            Ok(Vec::new())
        }
    }
}

impl Codegen for OverrideField {
    fn generate_code_spec(
        &self,
        rule_fields: &[FieldDescriptor],
        settings: &CodegenSettings,
    ) -> Result<TokenStream> {
        // TODO: CONVERT
        let parser_name = format_ident!("parse_{}", self.typ);
        let skip_ws = if settings.skip_whitespace {
            quote!(let state = state.skip_whitespace();)
        } else {
            quote!()
        };
        let field_conversion = generate_field_converter("_override", &self.typ, rule_fields);
        Ok(quote!(
            #[inline(always)]
            pub fn parse<'a>(state: ParseState<'a>, cache: &mut ParseCache<'a>) -> ParseResult<'a, Parsed> {
                #skip_ws
                let ok_result = #parser_name (state, cache)?;
                Ok(ok_result.map(|result| Parsed{ _override: #field_conversion }))
            }
        ))
    }

    fn get_fields(&self) -> Result<Vec<FieldDescriptor>> {
        Ok(vec![FieldDescriptor {
            name: "_override",
            type_names: [&self.typ as &str].into(),
            arity: Arity::One,
            boxed: false,
        }])
    }
}

fn generate_field_converter(
    field_name: &str,
    field_type_name: &str,
    rule_fields: &[FieldDescriptor],
) -> TokenStream {
    let field = rule_fields
        .iter()
        .find(|f| f.name == field_name)
        .expect("Field not found in rule_fields");
    let enumified_field = if field.type_names.len() > 1 {
        let enum_type_name = format_ident!("Parsed_{}", field_name);
        let field_type_ident = format_ident!("{}", field_type_name);
        quote!(#enum_type_name::#field_type_ident(result))
    } else {
        quote!(result)
    };
    match field.arity {
        Arity::One => {
            if field.boxed {
                quote!(Box::new(#enumified_field))
            } else {
                quote!(#enumified_field)
            }
        }
        Arity::Optional => {
            if field.boxed {
                quote!(Some(Box::new(#enumified_field)))
            } else {
                quote!(Some(#enumified_field))
            }
        }
        Arity::Multiple => quote!(vec![#enumified_field]),
    }
}
