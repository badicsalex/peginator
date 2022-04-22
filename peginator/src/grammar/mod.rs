// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

pub const GRAMMAR: &str = r###"

Grammar = {rules:Rule ";"} ;

Rule = {directives:Directive} name:Identifier "=" definition:Choice;

Choice = choices:Sequence {"|" choices:Sequence};

Sequence = { parts:DelimitedExpression }+;

Group = "(" body:Choice ")";

Closure = "{" body:Choice "}" (at_least_one:AtLeastOneMarker|);
AtLeastOneMarker = "+"

NegativeLookahead = "!" expr:*DelimitedExpression;

CharacterRange = from:CharacterLiteral ".." to:CharacterLiteral;

CharacterLiteral = "'" @:char "'"

StringLiteral = '"' body:StringLiteralBody '"';

@string
@no_skip_ws
StringLiteralBody = { "\\\"" | !'"' char };

OverrideField = "@" ":" typ:Identifier;

Field = (name:Identifier ":" (boxed:BoxMarker |)| ) typ:Identifier;

BoxMarker = '*';

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

#[allow(non_snake_case, non_camel_case_types)]
pub mod test;

pub struct Grammar {
    pub rules: Vec<Rule>,
}

pub struct Rule {
    pub directives: Vec<DirectiveExpression>,
    pub name: Identifier,
    pub definition: Choice,
}

pub struct Choice {
    pub choices: Vec<Sequence>,
}

pub struct Sequence {
    pub parts: Vec<DelimitedExpression>,
}

pub enum DelimitedExpression {
    Group(Group),
    Closure(Closure),
    NegativeLookahead(NegativeLookahead),
    CharacterRange(CharacterRange),
    CharacterLiteral(CharacterLiteral),
    StringLiteral(StringLiteral),
    OverrideField(OverrideField),
    Field(Field),
}

macro_rules! detailed_expression_helper {
    ($TheType:ident) => {
        impl From<$TheType> for DelimitedExpression {
            fn from(v: $TheType) -> Self {
                DelimitedExpression::$TheType(v)
            }
        }

        impl From<$TheType> for Vec<DelimitedExpression> {
            fn from(v: $TheType) -> Self {
                vec![DelimitedExpression::$TheType(v)]
            }
        }
    };
}

detailed_expression_helper!(Group);
detailed_expression_helper!(Closure);
detailed_expression_helper!(NegativeLookahead);
detailed_expression_helper!(CharacterRange);
detailed_expression_helper!(StringLiteral);
detailed_expression_helper!(OverrideField);
detailed_expression_helper!(Field);

impl From<CharacterLiteral> for DelimitedExpression {
    fn from(v: CharacterLiteral) -> Self {
        DelimitedExpression::CharacterLiteral(v)
    }
}
pub struct Group {
    pub body: Choice,
}

pub struct Closure {
    pub body: Choice,
    pub at_least_one: Option<AtLeastOneMarker>,
}

pub struct AtLeastOneMarker;

pub struct NegativeLookahead {
    pub expr: Box<DelimitedExpression>,
}

pub struct CharacterRange {
    pub from: CharacterLiteral,
    pub to: CharacterLiteral,
}

pub type CharacterLiteral = char;

pub struct StringLiteral {
    pub body: StringLiteralBody,
}

pub type StringLiteralBody = String;

pub struct OverrideField {
    pub typ: Identifier,
}

pub struct Field {
    pub name: Option<Identifier>,
    pub boxed: Option<BoxMarker>,
    pub typ: Identifier,
}

pub struct BoxMarker;

pub type Identifier = String;

pub enum DirectiveExpression {
    StringDirective(StringDirective),
    NoSkipWsDirective(NoSkipWsDirective),
}

pub struct StringDirective {}
pub struct NoSkipWsDirective {}

fn simple_sequence(parts: Vec<DelimitedExpression>) -> Choice {
    Choice {
        choices: vec![Sequence { parts }],
    }
}

fn simple_rule(name: &str, parts: Vec<DelimitedExpression>) -> Rule {
    Rule {
        directives: vec![],
        name: name.into(),
        definition: simple_sequence(parts),
    }
}

fn field(name: &str, typ: &str) -> DelimitedExpression {
    Field {
        name: Some(name.into()),
        boxed: None,
        typ: typ.into(),
    }
    .into()
}

pub fn bootstrap_parsinator_grammar() -> Grammar {
    Grammar {
        rules: vec![
            simple_rule(
                "Grammar",
                Closure {
                    body: simple_sequence(vec![
                        field("rules", "Rule"),
                        StringLiteral { body: ";".into() }.into(),
                    ]),
                    at_least_one: None,
                }
                .into(),
            ),
            simple_rule(
                "Rule",
                vec![
                    Closure {
                        body: simple_sequence(vec![field("directives", "DirectiveExpression")]),
                        at_least_one: None,
                    }
                    .into(),
                    field("name", "Identifier"),
                    StringLiteral { body: "=".into() }.into(),
                    field("definition", "Choice"),
                ],
            ),
            simple_rule(
                "Choice",
                vec![
                    field("choices", "Sequence"),
                    Closure {
                        body: simple_sequence(vec![
                            StringLiteral { body: "|".into() }.into(),
                            field("choices", "Sequence"),
                        ]),
                        at_least_one: None,
                    }
                    .into(),
                ],
            ),
            simple_rule(
                "Sequence",
                Closure {
                    body: simple_sequence(vec![field("parts", "DelimitedExpression")]),
                    at_least_one: Some(AtLeastOneMarker),
                }
                .into(),
            ),
            simple_rule(
                "Group",
                vec![
                    StringLiteral { body: "(".into() }.into(),
                    field("body", "Choice"),
                    StringLiteral { body: ")".into() }.into(),
                ],
            ),
            simple_rule(
                "Closure",
                vec![
                    StringLiteral { body: "{".into() }.into(),
                    field("body", "Choice"),
                    StringLiteral { body: "}".into() }.into(),
                    Group {
                        body: Choice {
                            choices: vec![
                                Sequence {
                                    parts: vec![field("at_least_one", "AtLeastOneMarker")],
                                },
                                Sequence { parts: vec![] },
                            ],
                        },
                    }
                    .into(),
                ],
            ),
            simple_rule("AtLeastOneMarker", vec!['+'.into()]),
            simple_rule(
                "NegativeLookahead",
                vec![
                    StringLiteral { body: "!".into() }.into(),
                    Field {
                        name: Some("expr".into()),
                        boxed: Some(BoxMarker),
                        typ: "DelimitedExpression".into(),
                    }
                    .into(),
                ],
            ),
            simple_rule(
                "CharacterRange",
                vec![
                    field("from", "CharacterLiteral"),
                    StringLiteral { body: "..".into() }.into(),
                    field("to", "CharacterLiteral"),
                ],
            ),
            simple_rule(
                "CharacterLiteral",
                vec![
                    StringLiteral { body: "'".into() }.into(),
                    OverrideField { typ: "char".into() }.into(),
                    StringLiteral { body: "'".into() }.into(),
                ],
            ),
            simple_rule(
                "StringLiteral",
                vec!['"'.into(), field("body", "StringLiteralBody"), '"'.into()],
            ),
            Rule {
                directives: vec![
                    DirectiveExpression::StringDirective(StringDirective {}),
                    DirectiveExpression::NoSkipWsDirective(NoSkipWsDirective {}),
                ],
                name: "StringLiteralBody".into(),
                definition: Choice {
                    choices: vec![Sequence {
                        parts: Closure {
                            body: Choice {
                                choices: vec![
                                    Sequence {
                                        parts: StringLiteral {
                                            body: "\\\"".into(),
                                        }
                                        .into(),
                                    },
                                    Sequence {
                                        parts: vec![
                                            NegativeLookahead {
                                                expr: Box::new('"'.into()),
                                            }
                                            .into(),
                                            Field {
                                                name: None,
                                                boxed: None,
                                                typ: "char".into(),
                                            }
                                            .into(),
                                        ],
                                    },
                                ],
                            },
                            at_least_one: None,
                        }
                        .into(),
                    }],
                },
            },
            simple_rule(
                "Field",
                vec![
                    Group {
                        body: Choice {
                            choices: vec![
                                Sequence {
                                    parts: vec![
                                        field("name", "Identifier"),
                                        StringLiteral { body: ":".into() }.into(),
                                        Group {
                                            body: Choice {
                                                choices: vec![
                                                    Sequence {
                                                        parts: vec![field("boxed", "BoxMarker")],
                                                    },
                                                    Sequence { parts: vec![] },
                                                ],
                                            },
                                        }
                                        .into(),
                                    ],
                                },
                                Sequence { parts: vec![] },
                            ],
                        },
                    }
                    .into(),
                    field("typ", "Identifier"),
                ],
            ),
            simple_rule("BoxMarker", vec!['*'.into()]),
            simple_rule(
                "OverrideField",
                vec![
                    StringLiteral { body: "@".into() }.into(),
                    StringLiteral { body: ":".into() }.into(),
                    field("typ", "Identifier"),
                ],
            ),
            Rule {
                directives: vec![],
                name: "DelimitedExpression".into(),
                definition: Choice {
                    choices: vec![
                        Sequence {
                            parts: OverrideField {
                                typ: "Group".into(),
                            }
                            .into(),
                        },
                        Sequence {
                            parts: OverrideField {
                                typ: "Closure".into(),
                            }
                            .into(),
                        },
                        Sequence {
                            parts: OverrideField {
                                typ: "NegativeLookahead".into(),
                            }
                            .into(),
                        },
                        Sequence {
                            parts: OverrideField {
                                typ: "CharacterRange".into(),
                            }
                            .into(),
                        },
                        Sequence {
                            parts: OverrideField {
                                typ: "CharacterLiteral".into(),
                            }
                            .into(),
                        },
                        Sequence {
                            parts: OverrideField {
                                typ: "StringLiteral".into(),
                            }
                            .into(),
                        },
                        Sequence {
                            parts: OverrideField {
                                typ: "OverrideField".into(),
                            }
                            .into(),
                        },
                        Sequence {
                            parts: OverrideField {
                                typ: "Field".into(),
                            }
                            .into(),
                        },
                    ],
                },
            },
            Rule {
                directives: vec![
                    DirectiveExpression::StringDirective(StringDirective {}),
                    DirectiveExpression::NoSkipWsDirective(NoSkipWsDirective {}),
                ],
                name: "Identifier".into(),
                definition: Choice {
                    choices: vec![Sequence {
                        parts: Closure {
                            body: Choice {
                                choices: vec![
                                    Sequence {
                                        parts: CharacterRange { from: 'a', to: 'z' }.into(),
                                    },
                                    Sequence {
                                        parts: CharacterRange { from: 'A', to: 'Z' }.into(),
                                    },
                                    Sequence {
                                        parts: CharacterRange { from: '0', to: '9' }.into(),
                                    },
                                    Sequence {
                                        parts: vec!['_'.into()],
                                    },
                                ],
                            },
                            at_least_one: Some(AtLeastOneMarker),
                        }
                        .into(),
                    }],
                },
            },
            Rule {
                directives: vec![],
                name: "DirectiveExpression".into(),
                definition: Choice {
                    choices: vec![
                        Sequence {
                            parts: OverrideField {
                                typ: "StringDirective".into(),
                            }
                            .into(),
                        },
                        Sequence {
                            parts: OverrideField {
                                typ: "NoSkipWsDirective".into(),
                            }
                            .into(),
                        },
                    ],
                },
            },
            simple_rule(
                "StringDirective",
                StringLiteral {
                    body: "@string".into(),
                }
                .into(),
            ),
            simple_rule(
                "NoSkipWsDirective",
                StringLiteral {
                    body: "@no_skip_ws".into(),
                }
                .into(),
            ),
        ],
    }
}
