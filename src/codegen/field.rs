// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

use anyhow::Result;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use super::common::{
    generate_skip_ws, safe_ident, Arity, CloneState, Codegen, CodegenSettings, FieldDescriptor,
};
use crate::grammar::{Field, Grammar, OverrideField};

impl Codegen for Field {
    fn generate_inline_body(
        &self,
        rule_fields: &[FieldDescriptor],
        _grammar: &Grammar,
        settings: &CodegenSettings,
        clone_state: CloneState,
    ) -> Result<Option<TokenStream>> {
        let postprocess = if let Some(field_name) = &self.name {
            generate_postprocess_calls(field_name, &self.typ, rule_fields)
        } else {
            quote!(.discard_result())
        };
        let parser_call = generate_skip_ws(
            settings,
            &format!("parse_{}", self.typ),
            quote!(tracer, cache),
            clone_state,
        );
        Ok(Some(quote!(#parser_call #postprocess)))
    }

    fn get_fields(&self, _grammar: &Grammar) -> Result<Vec<FieldDescriptor>> {
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
    fn generate_inline_body(
        &self,
        rule_fields: &[FieldDescriptor],
        _grammar: &Grammar,
        settings: &CodegenSettings,
        clone_state: CloneState,
    ) -> Result<Option<TokenStream>> {
        let postprocess = generate_postprocess_calls("_override", &self.typ, rule_fields);
        let parser_call = generate_skip_ws(
            settings,
            &format!("parse_{}", self.typ),
            quote!(tracer, cache),
            clone_state,
        );
        Ok(Some(quote!(#parser_call #postprocess)))
    }

    fn get_fields(&self, _grammar: &Grammar) -> Result<Vec<FieldDescriptor>> {
        Ok(vec![FieldDescriptor {
            name: "_override",
            type_names: [&self.typ as &str].into(),
            arity: Arity::One,
            boxed: false,
        }])
    }
}

fn generate_postprocess_calls(
    field_name: &str,
    field_type_name: &str,
    rule_fields: &[FieldDescriptor],
) -> TokenStream {
    let field = rule_fields
        .iter()
        .find(|f| f.name == field_name)
        .expect("Field not found in rule_fields");
    let enum_type_name = format_ident!("Parsed_{}", field_name);
    let field_type_ident = safe_ident(field_type_name);

    // Special cases for the most common cases
    if !field.boxed {
        if field.type_names.len() == 1 && field.arity == Arity::One {
            return TokenStream::new();
        }
        if field.type_names.len() > 1 && field.arity == Arity::One {
            return quote!(.map_inner(#enum_type_name::#field_type_ident));
        }
        if field.type_names.len() == 1 && field.arity == Arity::Optional {
            return quote!(.map_inner(Some));
        }
    }

    let enumified_field = if field.type_names.len() > 1 {
        quote!(#enum_type_name::#field_type_ident(result))
    } else {
        quote!(result)
    };
    let field_conversion = match field.arity {
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
    };
    quote!(
        .map_inner(|result| #field_conversion )
    )
}
