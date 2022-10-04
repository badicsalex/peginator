// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

use anyhow::Result;
use proc_macro2::TokenStream;
use quote::quote;

use super::common::{generate_skip_ws, Codegen, CodegenSettings, FieldDescriptor};
use crate::grammar::{EndOfInput, Grammar};

impl Codegen for EndOfInput {
    fn generate_inline_body(
        &self,
        _rule_fields: &[FieldDescriptor],
        settings: &CodegenSettings,
    ) -> Result<Option<TokenStream>> {
        Ok(Some(generate_skip_ws(
            settings,
            quote!(parse_end_of_input(state)),
        )))
    }

    fn get_fields(&self, _grammar: &Grammar) -> Result<Vec<FieldDescriptor>> {
        Ok(Vec::new())
    }
}
