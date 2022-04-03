// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

/*
Grammar = {rules:Rule ";"} ;

Rule = {directives:Directive} name:Identifier "=" definition:Choice;

Choice = choices:Sequence {"|" choices:Sequence};

Sequence = { parts:DelimitedExpression }+;

DelimitedExpression =
    @:Group |
    @:ClosureAtLeastOne |
    @:Closure |
    @:NegativeLookahead |
    @:CharacterRange |
    @:Character |
    @:StringLiteral |
    @:OverrideField |
    @:Field
;

Group = "(" body:Choice ")";

ClosureAtLeastOne = "{" body:Choice "}+";
Closure = "{" body:Choice "}";

NegativeLookahead = "!" expr:DelimitedExpression;

CharacterRange = from:CharacterLiteral ".." to:CharacterLiteral;

CharacterLiteral = "'" @:ANY_CHARACTER "'"

StringLiteral = '"' body:StringLiteralBody '"';

@string
StringLiteralBody = { "\\\"" | !'"' ANY_CHARACTER };

OverrideField = "@" ":" typ:Identifier;

Field = (name:Identifier ":" | ) typ:Identifier;

@string
Identifier = {'a'..'z' | 'A'..'Z' | '0'..'9'}+;

DirectiveExpression = @:StringDirective;
StringDirective:Directive = "@string";

*/

pub struct Grammar {
    pub rules: Vec<Rule>,
}

pub struct Rule {
    pub directives: Vec<Directive>,
    pub name: Identifier,
    pub definition: Choice,
}

pub struct Choice {
    pub choices: Vec<Sequence>,
}

pub struct Sequence {
    pub parts: Vec<DetailedExpression>,
}

pub enum DetailedExpression {
    Group(Group),
    ClosureAtLeastOne(ClosureAtLeastOne),
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
        impl From<$TheType> for DetailedExpression {
            fn from(v: $TheType) -> Self {
                DetailedExpression::$TheType(v)
            }
        }

        impl From<$TheType> for Vec<DetailedExpression> {
            fn from(v: $TheType) -> Self {
                vec![DetailedExpression::$TheType(v)]
            }
        }
    };
}

detailed_expression_helper!(Group);
detailed_expression_helper!(ClosureAtLeastOne);
detailed_expression_helper!(Closure);
detailed_expression_helper!(NegativeLookahead);
detailed_expression_helper!(CharacterRange);
detailed_expression_helper!(StringLiteral);
detailed_expression_helper!(OverrideField);
detailed_expression_helper!(Field);

impl From<CharacterLiteral> for DetailedExpression {
    fn from(v: CharacterLiteral) -> Self {
        DetailedExpression::CharacterLiteral(v)
    }
}
pub struct Group {
    pub body: Choice,
}

pub struct ClosureAtLeastOne {
    pub body: Choice,
}

pub struct Closure {
    pub body: Choice,
}

pub struct NegativeLookahead {
    pub expr: Box<DetailedExpression>,
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
    pub name: Option<Box<Identifier>>,
    pub typ: Identifier,
}

pub type Identifier = String;

pub type Directive = StringDirective;

pub struct StringDirective {}

fn simple_sequence(parts: Vec<DetailedExpression>) -> Choice {
    Choice {
        choices: vec![Sequence { parts }],
    }
}

fn simple_rule(name: &str, parts: Vec<DetailedExpression>) -> Rule {
    Rule {
        directives: vec![],
        name: name.into(),
        definition: simple_sequence(parts),
    }
}

fn field(name: &str, typ: &str) -> DetailedExpression {
    Field {
        name: Some(Box::new(name.into())),
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
                }
                .into(),
            ),
            simple_rule(
                "Rule",
                vec![
                    Closure {
                        body: simple_sequence(vec![field("directives", "Directive")]),
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
                    }
                    .into(),
                ],
            ),
            simple_rule(
                "Sequence",
                ClosureAtLeastOne {
                    body: simple_sequence(vec![field("parts", "DelimitedExpression")]),
                }
                .into(),
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
                                typ: "ClosureAtLeastOne".into(),
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
                                typ: "Character".into(),
                            }
                            .into(),
                        },
                        Sequence {
                            parts: OverrideField {
                                typ: "Literal".into(),
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
            simple_rule(
                "Group",
                vec![
                    StringLiteral { body: "(".into() }.into(),
                    field("body", "Choice"),
                    StringLiteral { body: ")".into() }.into(),
                ],
            ),
            simple_rule(
                "ClosureAtLeastOne",
                vec![
                    StringLiteral { body: "{".into() }.into(),
                    field("body", "Choice"),
                    StringLiteral { body: "}+".into() }.into(),
                ],
            ),
            simple_rule(
                "Closure",
                vec![
                    StringLiteral { body: "{".into() }.into(),
                    field("body", "Choice"),
                    StringLiteral { body: "}".into() }.into(),
                ],
            ),
            simple_rule(
                "NegativeLookahead",
                vec![
                    StringLiteral { body: "!".into() }.into(),
                    field("expr", "DelimitedExpression"),
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
            simple_rule("Character", vec![field("chr", "CharacterLiteral")]),
            simple_rule(
                "CharacterLiteral",
                vec![
                    StringLiteral { body: "'".into() }.into(),
                    OverrideField {
                        typ: "ANY_CHARACTER".into(),
                    }
                    .into(),
                    StringLiteral { body: "'".into() }.into(),
                ],
            ),
            simple_rule(
                "StringLiteral",
                vec!['"'.into(), field("body", "StringLiteralBody"), '"'.into()],
            ),
            Rule {
                directives: vec![StringDirective {}],
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
                                                typ: "ANY_CHARACTER".into(),
                                            }
                                            .into(),
                                        ],
                                    },
                                ],
                            },
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
                                        StringLiteral { body: "@".into() }.into(),
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
            simple_rule(
                "OverrideField",
                vec![
                    StringLiteral { body: "@".into() }.into(),
                    StringLiteral { body: ":".into() }.into(),
                    field("typ", "Identifier"),
                ],
            ),
            Rule {
                directives: vec![StringDirective {}],
                name: "Identifier".into(),
                definition: Choice {
                    choices: vec![Sequence {
                        parts: ClosureAtLeastOne {
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
                                ],
                            },
                        }
                        .into(),
                    }],
                },
            },
            simple_rule(
                "DirectiveExpression",
                OverrideField {
                    typ: "StringDirective".into(),
                }
                .into(),
            ),
            simple_rule(
                "StringDirective",
                StringLiteral {
                    body: "@string".into(),
                }
                .into(),
            ),
        ],
    }
}
