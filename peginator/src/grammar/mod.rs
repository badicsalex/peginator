// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

/*
Grammar = {rules:Rule ";"} ;

Rule = {directives:Directive} name:Identifier "=" definition:Choice;

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

Optional = "[" body:Choice "]";

ClosureAtLeastOne = "{" body:Choice "}+";
Closure = "{" body:Choice "}";

NegativeLookahead = "!" expr:DelimitedExpression;

CharacterRange = from:CharacterLiteral ".." to:CharacterLiteral;

Character = chr:CharacterLiteral;

CharacterLiteral = "'" @:ANY_CHARACTER "'";

StringLiteral = '"' body:StringLiteralBody '"';

@string
StringLiteralBody = { "\\\"" | !'"' ANY_CHARACTER };

OverrideField = "@" ":" typ:Identifier;

Field = [name:Identifier ":"] typ:Identifier;

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
    definition: Choice,
}

struct Choice {
    choices: Vec<Sequence>,
}

struct Sequence {
    parts: Vec<DetailedExpression>,
}

enum DetailedExpression {
    Optional(Optional),
    ClosureAtLeastOne(ClosureAtLeastOne),
    Closure(Closure),
    NegativeLookahead(NegativeLookahead),
    CharacterRange(CharacterRange),
    Character(Character),
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

detailed_expression_helper!(Optional);
detailed_expression_helper!(ClosureAtLeastOne);
detailed_expression_helper!(Closure);
detailed_expression_helper!(NegativeLookahead);
detailed_expression_helper!(CharacterRange);
detailed_expression_helper!(Character);
detailed_expression_helper!(StringLiteral);
detailed_expression_helper!(OverrideField);
detailed_expression_helper!(Field);

struct Optional {
    body: Choice,
}

struct ClosureAtLeastOne {
    body: Choice,
}

struct Closure {
    body: Choice,
}

struct NegativeLookahead {
    expr: Box<DetailedExpression>,
}

struct CharacterRange {
    from: char,
    to: char,
}

struct Character {
    chr: char,
}

struct StringLiteral {
    body: StringLiteralBody,
}

struct OverrideField {
    typ: Identifier,
}

struct Field {
    name: Option<Box<Identifier>>,
    typ: Identifier,
}

type StringLiteralBody = String;
type Identifier = String;

type Directive = StringDirective;

struct StringDirective {}

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

fn bootstrap_parsinator_grammar() -> Grammar {
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
                                typ: "Optional".into(),
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
                "Optional",
                vec![
                    StringLiteral { body: "[".into() }.into(),
                    field("body", "Choice"),
                    StringLiteral { body: "]".into() }.into(),
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
                vec![
                    Character { chr: '"' }.into(),
                    field("body", "StringLiteralBody"),
                    Character { chr: '"' }.into(),
                ],
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
                                                expr: Box::new(Character { chr: '"' }.into()),
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
                    Optional {
                        body: simple_sequence(vec![
                            field("name", "Identifier"),
                            StringLiteral { body: "@".into() }.into(),
                        ]),
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
