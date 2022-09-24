// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

use std::collections::BTreeSet;

use anyhow::Result;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote};

use crate::codegen::safe_ident;

#[derive(Debug, Clone)]
pub struct CodegenSettings {
    pub skip_whitespace: bool,
    pub peginator_crate_name: String,
    pub derives: Vec<String>,
}

impl Default for CodegenSettings {
    fn default() -> Self {
        Self {
            skip_whitespace: true,
            peginator_crate_name: "peginator".into(),
            derives: vec!["Debug".into(), "Clone".into()],
        }
    }
}

pub trait CodegenGrammar {
    fn generate_code(&self, settings: &CodegenSettings) -> Result<TokenStream>;
}

pub trait CodegenRule {
    fn generate_code(&self, settings: &CodegenSettings) -> Result<(TokenStream, TokenStream)>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Arity {
    One,
    Optional,
    Multiple,
}

#[derive(Debug, Clone)]
pub struct FieldDescriptor<'a> {
    pub name: &'a str,
    pub type_names: BTreeSet<&'a str>,
    pub arity: Arity,
    pub boxed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordPosition {
    No,
    Yes,
}

impl From<bool> for RecordPosition {
    fn from(b: bool) -> Self {
        if b {
            Self::Yes
        } else {
            Self::No
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PublicType {
    No,
    Yes,
}

pub trait Codegen {
    /// Generate code that's related to the parse function and the child parsers and types.
    ///
    /// Should not generate any types that are related to this parser.
    fn generate_code_spec(
        &self,
        rule_fields: &[FieldDescriptor],
        settings: &CodegenSettings,
    ) -> Result<TokenStream>;

    /// Get all fields that are generated by this parser
    fn get_fields(&self) -> Result<Vec<FieldDescriptor>>;

    /// Generate all parse code and types that are related to this parser
    ///
    /// Calls generate_code_spec and also generates related types
    fn generate_code(
        &self,
        rule_fields: &[FieldDescriptor],
        settings: &CodegenSettings,
    ) -> Result<TokenStream> {
        let spec_body = self.generate_code_spec(rule_fields, settings)?;
        let parsed_type = self.generate_struct_type(
            rule_fields,
            settings,
            "Parsed",
            RecordPosition::No,
            PublicType::No,
        )?;
        Ok(quote!(
            #spec_body
            #parsed_type
        ))
    }

    fn generate_struct_type(
        &self,
        rule_fields: &[FieldDescriptor],
        settings: &CodegenSettings,
        type_name: &str,
        record_position: RecordPosition,
        public_type: PublicType,
    ) -> Result<TokenStream> {
        let fields = self.get_filtered_rule_fields(rule_fields)?;
        Ok(generate_parsed_struct_type(
            type_name,
            &fields,
            settings,
            record_position,
            public_type,
        ))
    }

    fn get_filtered_rule_fields<'a>(
        &self,
        rule_fields: &[FieldDescriptor<'a>],
    ) -> Result<Vec<FieldDescriptor<'a>>> {
        let fields = self.get_fields()?;
        Ok(rule_fields
            .iter()
            .filter(|rf| fields.iter().any(|f| f.name == rf.name))
            .cloned()
            .collect())
    }
}

fn generate_derives(settings: &CodegenSettings) -> TokenStream {
    if settings.derives.is_empty() {
        return quote!();
    }
    let derive_idents: Vec<Ident> = settings
        .derives
        .iter()
        .map(|f| Ident::new(f, Span::call_site()))
        .collect();
    quote!(#[derive( #( #derive_idents, )*)])
}

fn generate_parsed_struct_type(
    type_name: &str,
    fields: &[FieldDescriptor],
    settings: &CodegenSettings,
    record_position: RecordPosition,
    public_type: PublicType,
) -> TokenStream {
    let type_ident = safe_ident(type_name);
    let derives = if public_type == PublicType::Yes {
        generate_derives(settings)
    } else {
        quote!()
    };

    if fields.is_empty() && record_position == RecordPosition::No {
        quote!(
            #derives
            pub struct #type_ident;
        )
    } else {
        let field_names: Vec<Ident> = fields
            .iter()
            .map(|f| {
                let mut ident = safe_ident(f.name);
                ident.set_span(Span::call_site());
                ident
            })
            .collect();
        let field_types: Vec<TokenStream> = fields
            .iter()
            .map(|f| generate_field_type(type_name, f, settings))
            .collect();
        let position_field = if record_position == RecordPosition::Yes {
            quote!(pub position: std::ops::Range<usize>,)
        } else {
            quote!()
        };
        quote!(
            #derives
            pub struct #type_ident {
                #( pub #field_names: #field_types, )*
                #position_field
            }
        )
    }
}

pub fn generate_field_type(
    parent_type: &str,
    field: &FieldDescriptor,
    _settings: &CodegenSettings,
) -> TokenStream {
    let field_inner_type_ident: TokenStream = if field.type_names.len() > 1 {
        let ident = format_ident!("{}_{}", parent_type, field.name);
        quote!(#ident)
    } else {
        let type_name = field.type_names.iter().next().unwrap();
        let ident = safe_ident(type_name);
        if type_name == &"char" {
            quote!(char)
        } else {
            quote!(#ident)
        }
    };
    match field.arity {
        Arity::One => {
            if field.boxed {
                quote!(Box<#field_inner_type_ident>)
            } else {
                quote!(#field_inner_type_ident)
            }
        }
        Arity::Optional => {
            if field.boxed {
                quote!(Option<Box<#field_inner_type_ident>>)
            } else {
                quote!(Option<#field_inner_type_ident>)
            }
        }
        Arity::Multiple => quote!(Vec<#field_inner_type_ident>),
    }
}

pub fn generate_enum_type(
    name: &str,
    field: &FieldDescriptor,
    settings: &CodegenSettings,
) -> TokenStream {
    let ident = safe_ident(name);
    let derives = generate_derives(settings);
    let type_idents: Vec<Ident> = field.type_names.iter().map(|n| safe_ident(n)).collect();
    quote!(
        #[allow(non_camel_case_types)]
        #derives
        pub enum #ident {
            #(#type_idents(#type_idents),)*
        }
    )
}
