// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

use std::collections::HashMap;

use anyhow::{anyhow, bail, Result};
use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote};

use crate::grammar::{
    CharacterLiteral, CharacterRange, Choice, Closure, ClosureAtLeastOne, DelimitedExpression,
    Field, Grammar, Group, NegativeLookahead, OverrideField, Rule, Sequence, StringDirective,
    StringLiteral,
};

use crate::codegen::types::{generate_field_type, ASTStruct, Arity};

use super::quick_ident;

trait TopLevelParserGenerator {
    fn generate_parser(&self, ast_structs: &HashMap<&str, &ASTStruct>) -> Result<TokenStream>;
}

impl TopLevelParserGenerator for Grammar {
    fn generate_parser(&self, ast_structs: &HashMap<&str, &ASTStruct>) -> Result<TokenStream> {
        let mut result = TokenStream::new();
        for rule in &self.rules {
            result.extend(rule.generate_parser(ast_structs)?)
        }
        Ok(result)
    }
}

impl TopLevelParserGenerator for Rule {
    fn generate_parser(&self, ast_structs: &HashMap<&str, &ASTStruct>) -> Result<TokenStream> {
        let mytype = ast_structs[&self.name as &str];
        let type_name_ident = quick_ident(mytype.name);

        let field_parser = self.definition.generate_parser(&self, ast_structs)?;
        let field_declarations: TokenStream = mytype
            .fields
            .iter()
            .flat_map(|f| -> TokenStream {
                let ident = quick_ident(if f.name == "@" { "result" } else { f.name });
                let field_type = generate_field_type(mytype.name, f);
                match f.arity {
                    Arity::One => quote!(let #ident: #field_type;),
                    Arity::Optional => quote!(let #ident: #field_type = None;),
                    Arity::Multiple => quote!(let #ident: #field_type = Vec::new();),
                }
            })
            .collect();
        let result_constructor = if mytype.is_overridden() {
            quote!(result)
        } else if mytype.string_rule {
            quote!(String::new())
        } else {
            let field_idents: Vec<Ident> =
                mytype.fields.iter().map(|f| quick_ident(f.name)).collect();

            quote!(
                #type_name_ident {
                    #(#field_idents, )*
                }
            )
        };
        let rule_parser_ident = format_ident!("parse_{}", self.name);
        Ok(quote!(
            fn #rule_parser_ident(state: ParseState) -> ParseResult<#type_name_ident>{
                #field_declarations;
                #field_parser;
                Ok((#result_constructor, state))
            }
        ))
    }
}

trait ParserGenerator {
    fn generate_parser(
        &self,
        current_rule: &Rule,
        ast_structs: &HashMap<&str, &ASTStruct>,
    ) -> Result<TokenStream>;
}

impl ParserGenerator for Choice {
    fn generate_parser(
        &self,
        current_rule: &Rule,
        ast_structs: &HashMap<&str, &ASTStruct>,
    ) -> Result<TokenStream> {
        if self.choices.len() == 1 {
            self.choices[0].generate_parser(current_rule, ast_structs)
        } else {
            let mut result = TokenStream::new();
            for choice in &self.choices {
                let choice_body = choice.generate_parser(current_rule, ast_structs)?;
                result.extend(quote!(
                    .choice(|| {#choice_body})
                ))
            }
            Ok(quote!(Builtins::start_choice() #result .end()))
        }
    }
}

impl ParserGenerator for Sequence {
    fn generate_parser(
        &self,
        current_rule: &Rule,
        ast_structs: &HashMap<&str, &ASTStruct>,
    ) -> Result<TokenStream> {
        if self.parts.len() == 1 {
            self.parts[0].generate_parser(current_rule, ast_structs)
        } else {
            let mut result = TokenStream::new();
            for part in &self.parts {
                let sequence_body = part.generate_parser(current_rule, ast_structs)?;
                result.extend(quote!(
                    .part(|| {#sequence_body})
                ))
            }
            Ok(quote!(Builtins::start_sequence() #result .end()))
        }
    }
}

impl ParserGenerator for DelimitedExpression {
    fn generate_parser(
        &self,
        current_rule: &Rule,
        ast_structs: &HashMap<&str, &ASTStruct>,
    ) -> Result<TokenStream> {
        match self {
            DelimitedExpression::Group(s) => s.generate_parser(current_rule, ast_structs),
            DelimitedExpression::ClosureAtLeastOne(s) => {
                s.generate_parser(current_rule, ast_structs)
            }
            DelimitedExpression::Closure(s) => s.generate_parser(current_rule, ast_structs),
            DelimitedExpression::NegativeLookahead(s) => {
                s.generate_parser(current_rule, ast_structs)
            }
            DelimitedExpression::CharacterRange(s) => s.generate_parser(current_rule, ast_structs),
            DelimitedExpression::CharacterLiteral(s) => {
                s.generate_parser(current_rule, ast_structs)
            }
            DelimitedExpression::StringLiteral(s) => s.generate_parser(current_rule, ast_structs),
            DelimitedExpression::OverrideField(s) => s.generate_parser(current_rule, ast_structs),
            DelimitedExpression::Field(s) => s.generate_parser(current_rule, ast_structs),
        }
    }
}

impl ParserGenerator for Group {
    fn generate_parser(
        &self,
        current_rule: &Rule,
        ast_structs: &HashMap<&str, &ASTStruct>,
    ) -> Result<TokenStream> {
        self.body.generate_parser(current_rule, ast_structs)
    }
}

impl ParserGenerator for ClosureAtLeastOne {
    fn generate_parser(
        &self,
        current_rule: &Rule,
        ast_structs: &HashMap<&str, &ASTStruct>,
    ) -> Result<TokenStream> {
        let body = self.body.generate_parser(current_rule, ast_structs)?;
        Ok(quote!(Builtins::closure_plus(||{#body})))
    }
}

impl ParserGenerator for Closure {
    fn generate_parser(
        &self,
        current_rule: &Rule,
        ast_structs: &HashMap<&str, &ASTStruct>,
    ) -> Result<TokenStream> {
        let body = self.body.generate_parser(current_rule, ast_structs)?;
        Ok(quote!(Builtins::closure(||{#body})))
    }
}

impl ParserGenerator for NegativeLookahead {
    fn generate_parser(
        &self,
        current_rule: &Rule,
        ast_structs: &HashMap<&str, &ASTStruct>,
    ) -> Result<TokenStream> {
        let body = self.expr.generate_parser(current_rule, ast_structs)?;
        Ok(quote!(Builtins::negative_lookahead(||{#body})))
    }
}

impl ParserGenerator for CharacterRange {
    fn generate_parser(
        &self,
        current_rule: &Rule,
        ast_structs: &HashMap<&str, &ASTStruct>,
    ) -> Result<TokenStream> {
        Ok(quote!("Character range"))
    }
}

impl ParserGenerator for CharacterLiteral {
    fn generate_parser(
        &self,
        current_rule: &Rule,
        ast_structs: &HashMap<&str, &ASTStruct>,
    ) -> Result<TokenStream> {
        Ok(quote!("Character literal"))
    }
}

impl ParserGenerator for StringLiteral {
    fn generate_parser(
        &self,
        current_rule: &Rule,
        ast_structs: &HashMap<&str, &ASTStruct>,
    ) -> Result<TokenStream> {
        Ok(quote!("String literal"))
    }
}

impl ParserGenerator for OverrideField {
    fn generate_parser(
        &self,
        current_rule: &Rule,
        ast_structs: &HashMap<&str, &ASTStruct>,
    ) -> Result<TokenStream> {
        Ok(quote!("Override field"))
    }
}

impl ParserGenerator for Field {
    fn generate_parser(
        &self,
        current_rule: &Rule,
        ast_structs: &HashMap<&str, &ASTStruct>,
    ) -> Result<TokenStream> {
        let current_type = &ast_structs[&current_rule.name as &str];
        let rule_parser_ident = format_ident!("parse_{}", self.typ);
        if let Some(field_name) = &self.name {
            let current_field = current_type
                .fields
                .iter()
                .find(|f| f.name == field_name)
                .unwrap();
            let field_name_ident = quick_ident(field_name);
            let commit_field = match (&current_field.arity, current_field.boxed) {
                (Arity::One, false) => quote!(#field_name_ident = tmp;),
                (Arity::One, true) => quote!(#field_name_ident = Box::new(tmp);),
                (Arity::Optional, false) => quote!(#field_name_ident = Some(tmp);),
                (Arity::Optional, true) => quote!(#field_name_ident = Some(Box::new(tmp));),
                (Arity::Multiple, _) => quote!(#field_name_ident.append(tmp);),
            };
            Ok(quote!(
                let (tmp, new_state) = #rule_parser_ident(state)?;
                state = new_state;
                #commit_field
            ))
        } else {
            Ok(quote!((_, state) = #rule_parser_ident(state)?;))
        }
    }
}

pub fn generate_parsers(grammar: &Grammar, ast_structs: &[ASTStruct]) -> Result<TokenStream> {
    grammar.generate_parser(&HashMap::from_iter(ast_structs.iter().map(|f| (f.name, f))))
}
