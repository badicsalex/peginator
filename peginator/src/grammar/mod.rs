// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

/*
Grammar = {rules:Rule ";"} ;

Rule = {directives:Directive} name:Identifier [":" typ:Identifier] "=" definition:Choice;

Choice = choices:Sequence {"|" choices:Sequence};

Sequence = { parts:DelimitedExpression }+;

DelimitedExpression =
    @:Optional |
    @:ClosureAtLeastOne |
    @:Closure |
    @:NegativeLookahead |
    @:CharacterRange |
    @:Character |
    @:StringLiteral |
    @:OverrideField |
    @:Field
;

Optional:Expression = "[" body:Choice "]";

ClosureAtLeastOne:Expression = "{" body:Choice "}+";
Closure:Expression = "{" body:Choice "}";

NegativeLookahead:Expression = "!" expr:DelimitedExpression;

CharacterRange:Expression = from:CharacterLiteral ".." to:CharacterLiteral;

Character:Expression = chr:CharacterLiteral;

CharacterLiteral = "'" @:ANY_CHARACTER "'";

StringLiteral:Expression = '"' body:StringLiteralBody '"';

@string
StringLiteralBody = { "\\\"" | !'"' ANY_CHARACTER };

OverrideField:Expression = "@" ":" typ:Identifier;

Field:Expression = [name:Identifier ":"] typ:Identifier;

@string
Identifier = {'a'..'z' | 'A'..'Z' | '0'..'9'}+;

DirectiveExpression = @:StringDirective;
StringDirective:Directive = "@string";

*/

pub struct Grammar {
    rules: Vec<Rule>,
}

struct Rule {
    directives: Vec<Directive>,
    name: Identifier,
    typ: Option<Box<Identifier>>,
    definition: Choice,
}

struct Choice {
    choices: Vec<Sequence>,
}

struct Sequence {
    parts: Vec<Expression>,
}

enum Expression {
    Optional {
        body: Choice,
    },
    ClosureAtLeastOne {
        body: Choice,
    },
    Closure {
        body: Choice,
    },
    NegativeLookahead {
        expr: Box<Expression>,
    },
    CharacterRange {
        from: char,
        to: char,
    },
    Character {
        chr: char,
    },
    StringLiteral {
        body: StringLiteralBody,
    },
    OverrideField {
        typ: Identifier,
    },
    Field {
        name: Option<Box<Identifier>>,
        typ: Identifier,
    },
}

use Expression::*;

type StringLiteralBody = String;
type Identifier = String;

enum Directive {
    StringDirective,
}

fn simple_sequence(parts: Vec<Expression>) -> Choice {
    Choice {
        choices: vec![Sequence { parts }],
    }
}

fn simple_rule(name: &str, typ: Option<&str>, parts: Vec<Expression>) -> Rule {
    Rule {
        directives: vec![],
        name: name.into(),
        typ: typ.map(|s| Box::new(s.into())),
        definition: simple_sequence(parts),
    }
}

fn field(name: &str, typ: &str) -> Expression {
    Field {
        name: Some(Box::new(name.into())),
        typ: typ.into(),
    }
}

fn bootstrap_parsinator_grammar() -> Grammar {
    Grammar {
        rules: vec![
            simple_rule(
                "Grammar",
                None,
                vec![Closure {
                    body: simple_sequence(vec![
                        field("rules", "Rule"),
                        StringLiteral { body: ";".into() },
                    ]),
                }],
            ),
            simple_rule(
                "Rule",
                None,
                vec![
                    Closure {
                        body: simple_sequence(vec![field("directives", "Directive")]),
                    },
                    field("name", "Identifier"),
                    Optional {
                        body: simple_sequence(vec![
                            StringLiteral { body: ":".into() },
                            field("typ", "Identifier"),
                        ]),
                    },
                    StringLiteral { body: "=".into() },
                    field("definition", "Choice"),
                ],
            ),
            simple_rule(
                "Choice",
                None,
                vec![
                    field("choices", "Sequence"),
                    Closure {
                        body: simple_sequence(vec![
                            StringLiteral { body: "|".into() },
                            field("choices", "Sequence"),
                        ]),
                    },
                ],
            ),
            simple_rule(
                "Sequence",
                None,
                vec![ClosureAtLeastOne {
                    body: simple_sequence(vec![field("parts", "DelimitedExpression")]),
                }],
            ),
            Rule {
                directives: vec![],
                name: "DelimitedExpression".into(),
                typ: None,
                definition: Choice {
                    choices: vec![
                        Sequence {
                            parts: vec![OverrideField {
                                typ: "Optional".into(),
                            }],
                        },
                        Sequence {
                            parts: vec![OverrideField {
                                typ: "ClosureAtLeastOne".into(),
                            }],
                        },
                        Sequence {
                            parts: vec![OverrideField {
                                typ: "Closure".into(),
                            }],
                        },
                        Sequence {
                            parts: vec![OverrideField {
                                typ: "NegativeLookahead".into(),
                            }],
                        },
                        Sequence {
                            parts: vec![OverrideField {
                                typ: "CharacterRange".into(),
                            }],
                        },
                        Sequence {
                            parts: vec![OverrideField {
                                typ: "Character".into(),
                            }],
                        },
                        Sequence {
                            parts: vec![OverrideField {
                                typ: "Literal".into(),
                            }],
                        },
                        Sequence {
                            parts: vec![OverrideField {
                                typ: "OverrideField".into(),
                            }],
                        },
                        Sequence {
                            parts: vec![OverrideField {
                                typ: "Field".into(),
                            }],
                        },
                    ],
                },
            },
            simple_rule(
                "Optional",
                Some("Expression"),
                vec![
                    StringLiteral { body: "[".into() },
                    field("body", "Choice"),
                    StringLiteral { body: "]".into() },
                ],
            ),
            simple_rule(
                "ClosureAtLeastOne",
                Some("Expression"),
                vec![
                    StringLiteral { body: "{".into() },
                    field("body", "Choice"),
                    StringLiteral { body: "}+".into() },
                ],
            ),
            simple_rule(
                "Closure",
                Some("Expression"),
                vec![
                    StringLiteral { body: "{".into() },
                    field("body", "Choice"),
                    StringLiteral { body: "}".into() },
                ],
            ),
            simple_rule(
                "NegativeLookahead",
                Some("Expression"),
                vec![
                    StringLiteral { body: "!".into() },
                    field("expr", "DelimitedExpression"),
                ],
            ),
            simple_rule(
                "CharacterRange",
                Some("Expression"),
                vec![
                    field("from", "CharacterLiteral"),
                    StringLiteral { body: "..".into() },
                    field("to", "CharacterLiteral"),
                ],
            ),
            simple_rule(
                "Character",
                Some("Expression"),
                vec![field("chr", "CharacterLiteral")],
            ),
            simple_rule(
                "CharacterLiteral",
                None,
                vec![
                    StringLiteral { body: "'".into() },
                    OverrideField {
                        typ: "ANY_CHARACTER".into(),
                    },
                    StringLiteral { body: "'".into() },
                ],
            ),
            simple_rule(
                "StringLiteral",
                Some("Expression"),
                vec![
                    Character { chr: '"' },
                    field("body", "StringLiteralBody"),
                    Character { chr: '"' },
                ],
            ),
            Rule {
                directives: vec![Directive::StringDirective],
                name: "StringLiteralBody".into(),
                typ: None,
                definition: Choice {
                    choices: vec![Sequence {
                        parts: vec![Closure {
                            body: Choice {
                                choices: vec![
                                    Sequence {
                                        parts: vec![StringLiteral {
                                            body: "\\\"".into(),
                                        }],
                                    },
                                    Sequence {
                                        parts: vec![
                                            NegativeLookahead {
                                                expr: Box::new(Character { chr: '"' }),
                                            },
                                            Field {
                                                name: None,
                                                typ: "ANY_CHARACTER".into(),
                                            },
                                        ],
                                    },
                                ],
                            },
                        }],
                    }],
                },
            },
            simple_rule(
                "Field",
                Some("Expression"),
                vec![
                    Optional {
                        body: simple_sequence(vec![
                            field("name", "Identifier"),
                            StringLiteral { body: "@".into() },
                        ]),
                    },
                    field("typ", "Identifier"),
                ],
            ),
            simple_rule(
                "OverrideField",
                Some("Expression"),
                vec![
                    StringLiteral { body: "@".into() },
                    StringLiteral { body: ":".into() },
                    field("typ", "Identifier"),
                ],
            ),
            Rule {
                directives: vec![Directive::StringDirective],
                name: "Identifier".into(),
                typ: None,
                definition: Choice {
                    choices: vec![Sequence {
                        parts: vec![ClosureAtLeastOne {
                            body: Choice {
                                choices: vec![
                                    Sequence {
                                        parts: vec![CharacterRange { from: 'a', to: 'z' }],
                                    },
                                    Sequence {
                                        parts: vec![CharacterRange { from: 'A', to: 'Z' }],
                                    },
                                    Sequence {
                                        parts: vec![CharacterRange { from: '0', to: '9' }],
                                    },
                                ],
                            },
                        }],
                    }],
                },
            },
            simple_rule(
                "DirectiveExpression",
                None,
                vec![OverrideField {
                    typ: "StringDirective".into(),
                }],
            ),
            simple_rule(
                "StringDirective",
                Some("Directive"),
                vec![StringLiteral {
                    body: "@string".into(),
                }],
            ),
        ],
    }
}
