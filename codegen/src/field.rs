// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

use anyhow::Result;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use super::common::{
    generate_skip_ws, safe_ident, Arity, CloneState, Codegen, CodegenSettings, FieldDescriptor,
};
use crate::{
    common::FieldProperties,
    grammar::{Field, Field_name, Grammar},
};

impl Codegen for Field {
    fn generate_inline_body(
        &self,
        rule_fields: &[FieldDescriptor],
        _grammar: &Grammar,
        settings: &CodegenSettings,
        clone_state: CloneState,
    ) -> Result<Option<TokenStream>> {
        let postprocess = if let Some(field_name) = &self.name {
            generate_postprocess_calls(
                match field_name {
                    Field_name::Identifier(field_name) => field_name,
                    Field_name::OverrideMarker(_) => "_override",
                },
                &self.typ,
                rule_fields,
            )
        } else {
            quote!(.discard_result())
        };
        let parser_call = generate_skip_ws(
            settings,
            &format!("parse_{}", self.typ),
            quote!(&mut *global),
            clone_state,
        );
        Ok(Some(quote!(#parser_call #postprocess)))
    }

    fn get_fields(&self, _grammar: &Grammar) -> Result<Vec<FieldDescriptor>> {
        if let Some(field_name) = &self.name {
            Ok(vec![FieldDescriptor {
                name: match field_name {
                    Field_name::Identifier(field_name) => field_name,
                    Field_name::OverrideMarker(_) => "_override",
                },
                types: [(
                    &self.typ as &str,
                    FieldProperties {
                        boxed: self.boxed.is_some(),
                    },
                )]
                .into(),
                arity: Arity::One,
            }])
        } else {
            Ok(Vec::new())
        }
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
    let enum_type_name = format_ident!("Parsed_{field_name}");
    let field_type_ident = safe_ident(field_type_name);

    let field_properties = field
        .types
        .get(field_type_name)
        .expect("Field type not found in field");
    let boxify = if field_properties.boxed {
        quote!(.map_inner(Box::new))
    } else {
        TokenStream::new()
    };
    let enumify = if field.types.len() > 1 {
        quote!(.map_inner(#enum_type_name::#field_type_ident))
    } else {
        TokenStream::new()
    };
    let aritify = match field.arity {
        Arity::One => TokenStream::new(),
        Arity::Optional => quote!(.map_inner(Some)),
        Arity::Multiple => quote!(.map_inner(|result| vec![result])),
    };
    quote!(
        #boxify
        #enumify
        #aritify
    )
}
