// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

pub const GRAMMAR: &str = r###"

Grammar = {rules:Rule ";"} ;

Rule = {directives:DirectiveExpression} name:Identifier "=" definition:Choice;

Choice = choices:Sequence {"|" choices:Sequence};

Sequence = { parts:DelimitedExpression };

Group = "(" body:Choice ")";

Closure = "{" body:Choice "}" (at_least_one:AtLeastOneMarker|);
AtLeastOneMarker = '+';

NegativeLookahead = "!" expr:*DelimitedExpression;

CharacterRange = from:CharacterLiteral ".." to:CharacterLiteral;

CharacterLiteral = "'" @:char "'";

StringLiteral = '"' body:StringLiteralBody '"';

@string
@no_skip_ws
StringLiteralBody = { "\\\"" | !'"' char };

Field = (name:Identifier ":" (boxed:BoxMarker |)| ) typ:Identifier;

BoxMarker = '*';

OverrideField = "@" ":" typ:Identifier;

DelimitedExpression =
    @:Group |
    @:Closure |
    @:NegativeLookahead |
    @:CharacterRange |
    @:CharacterLiteral |
    @:StringLiteral |
    @:OverrideField |
    @:Field
;


@string
@no_skip_ws
Identifier = {'a'..'z' | 'A'..'Z' | '0'..'9' | '_'}+;

DirectiveExpression = @:StringDirective | @:NoSkipWsDirective;
StringDirective = "@string";
NoSkipWsDirective = "@no_skip_ws";

"###;

mod generated;

pub use generated::*;
