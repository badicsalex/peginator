#[derive(Debug)]
pub struct Grammar {
    pub rules: Vec<Rule>,
}
#[derive(Debug)]
pub struct Rule {
    pub directives: Vec<DirectiveExpression>,
    pub name: Identifier,
    pub definition: Choice,
}
#[derive(Debug)]
pub struct Choice {
    pub choices: Vec<Sequence>,
}
#[derive(Debug)]
pub struct Sequence {
    pub parts: Vec<DelimitedExpression>,
}
#[derive(Debug)]
pub struct Group {
    pub body: Choice,
}
#[derive(Debug)]
pub struct Optional {
    pub body: Choice,
}
#[derive(Debug)]
pub struct Closure {
    pub body: Choice,
    pub at_least_one: Option<AtLeastOneMarker>,
}
pub type AtLeastOneMarker = ();
#[derive(Debug)]
pub struct NegativeLookahead {
    pub expr: Box<DelimitedExpression>,
}
#[derive(Debug)]
pub struct CharacterRange {
    pub from: CharacterLiteral,
    pub to: CharacterLiteral,
}
pub use char as CharacterLiteral;
#[derive(Debug)]
pub struct StringLiteral {
    pub body: StringLiteralBody,
}
pub type StringLiteralBody = String;
#[derive(Debug)]
pub struct Field {
    pub name: Option<Identifier>,
    pub boxed: Option<BoxMarker>,
    pub typ: Identifier,
}
pub type BoxMarker = ();
#[derive(Debug)]
pub struct OverrideField {
    pub typ: Identifier,
}
#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum DelimitedExpression__override {
    CharacterLiteral(CharacterLiteral),
    CharacterRange(CharacterRange),
    Closure(Closure),
    EndOfInput(EndOfInput),
    Field(Field),
    Group(Group),
    NegativeLookahead(NegativeLookahead),
    Optional(Optional),
    OverrideField(OverrideField),
    StringLiteral(StringLiteral),
}
pub use DelimitedExpression__override as DelimitedExpression;
pub type Identifier = String;
#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum DirectiveExpression__override {
    ExportDirective(ExportDirective),
    NoSkipWsDirective(NoSkipWsDirective),
    StringDirective(StringDirective),
}
pub use DirectiveExpression__override as DirectiveExpression;
pub type StringDirective = ();
pub type NoSkipWsDirective = ();
pub type ExportDirective = ();
pub type EndOfInput = ();
impl peginator_generated::PegParser for Grammar {
    fn parse_advanced(
        s: &str,
        settings: &peginator_generated::ParseSettings,
    ) -> Result<Self, peginator_generated::ParseError> {
        Ok(
            peginator_generated::parse_Grammar(peginator_generated::ParseState::new(s, settings))?
                .0,
        )
    }
}
#[allow(non_snake_case, unused_variables, unused_imports, unused_mut)]
mod peginator_generated {
    use super::*;
    use crate::runtime::*;
    pub use crate::runtime::{ParseError, ParseSettings, ParseState, PegParser};
    mod Grammar_impl {
        use super::*;
        mod part_0 {
            use super::*;
            mod closure {
                use super::*;
                mod part_0 {
                    use super::*;
                    #[inline(always)]
                    pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                        let state = state.skip_whitespace();
                        let (result, state) = parse_Rule(state)?;
                        Ok((
                            Parsed {
                                rules: vec![result],
                            },
                            state,
                        ))
                    }
                    #[derive(Debug)]
                    pub struct Parsed {
                        pub rules: Vec<Rule>,
                    }
                }
                mod part_1 {
                    use super::*;
                    #[inline(always)]
                    pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                        let state = state.skip_whitespace();
                        parse_string_literal(state, ";")
                    }
                    pub type Parsed = ();
                }
                #[inline(always)]
                pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                    let (result, state) = part_0::parse(state)?;
                    let mut rules = result.rules;
                    let (_, state) = part_1::parse(state)?;
                    Ok((Parsed { rules }, state))
                }
                #[derive(Debug)]
                pub struct Parsed {
                    pub rules: Vec<Rule>,
                }
            }
            #[inline(always)]
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                let mut state = state;
                let mut rules: Vec<Rule> = Vec::new();
                while let Ok((result, new_state)) = closure::parse(state.clone()) {
                    rules.extend(result.rules);
                    state = new_state;
                }
                Ok((Parsed { rules }, state))
            }
            #[derive(Debug)]
            pub struct Parsed {
                pub rules: Vec<Rule>,
            }
        }
        mod part_1 {
            use super::*;
            #[inline(always)]
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                let state = state.skip_whitespace();
                if state.is_empty() {
                    Ok(((), state))
                } else {
                    Err(ParseError)
                }
            }
            pub type Parsed = ();
        }
        #[inline(always)]
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let (result, state) = part_0::parse(state)?;
            let mut rules = result.rules;
            let (_, state) = part_1::parse(state)?;
            Ok((Parsed { rules }, state))
        }
        use super::Grammar as Parsed;
        #[inline(always)]
        pub fn rule_parser(state: ParseState) -> ParseResult<Parsed> {
            parse(state)
        }
    }
    #[inline]
    pub(super) fn parse_Grammar(state: ParseState) -> ParseResult<Grammar> {
        run_rule_parser(Grammar_impl::rule_parser, "Grammar", state)
    }
    mod Rule_impl {
        use super::*;
        mod part_0 {
            use super::*;
            mod closure {
                use super::*;
                #[inline(always)]
                pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                    let state = state.skip_whitespace();
                    let (result, state) = parse_DirectiveExpression(state)?;
                    Ok((
                        Parsed {
                            directives: vec![result],
                        },
                        state,
                    ))
                }
                #[derive(Debug)]
                pub struct Parsed {
                    pub directives: Vec<DirectiveExpression>,
                }
            }
            #[inline(always)]
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                let mut state = state;
                let mut directives: Vec<DirectiveExpression> = Vec::new();
                while let Ok((result, new_state)) = closure::parse(state.clone()) {
                    directives.extend(result.directives);
                    state = new_state;
                }
                Ok((Parsed { directives }, state))
            }
            #[derive(Debug)]
            pub struct Parsed {
                pub directives: Vec<DirectiveExpression>,
            }
        }
        mod part_1 {
            use super::*;
            #[inline(always)]
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                let state = state.skip_whitespace();
                let (result, state) = parse_Identifier(state)?;
                Ok((Parsed { name: result }, state))
            }
            #[derive(Debug)]
            pub struct Parsed {
                pub name: Identifier,
            }
        }
        mod part_2 {
            use super::*;
            #[inline(always)]
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                let state = state.skip_whitespace();
                parse_string_literal(state, "=")
            }
            pub type Parsed = ();
        }
        mod part_3 {
            use super::*;
            #[inline(always)]
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                let state = state.skip_whitespace();
                let (result, state) = parse_Choice(state)?;
                Ok((Parsed { definition: result }, state))
            }
            #[derive(Debug)]
            pub struct Parsed {
                pub definition: Choice,
            }
        }
        #[inline(always)]
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let (result, state) = part_0::parse(state)?;
            let mut directives = result.directives;
            let (result, state) = part_1::parse(state)?;
            let mut name = result.name;
            let (_, state) = part_2::parse(state)?;
            let (result, state) = part_3::parse(state)?;
            let mut definition = result.definition;
            Ok((
                Parsed {
                    directives,
                    name,
                    definition,
                },
                state,
            ))
        }
        use super::Rule as Parsed;
        #[inline(always)]
        pub fn rule_parser(state: ParseState) -> ParseResult<Parsed> {
            parse(state)
        }
    }
    #[inline]
    pub(super) fn parse_Rule(state: ParseState) -> ParseResult<Rule> {
        run_rule_parser(Rule_impl::rule_parser, "Rule", state)
    }
    mod Choice_impl {
        use super::*;
        mod part_0 {
            use super::*;
            #[inline(always)]
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                let state = state.skip_whitespace();
                let (result, state) = parse_Sequence(state)?;
                Ok((
                    Parsed {
                        choices: vec![result],
                    },
                    state,
                ))
            }
            #[derive(Debug)]
            pub struct Parsed {
                pub choices: Vec<Sequence>,
            }
        }
        mod part_1 {
            use super::*;
            mod closure {
                use super::*;
                mod part_0 {
                    use super::*;
                    #[inline(always)]
                    pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                        let state = state.skip_whitespace();
                        parse_string_literal(state, "|")
                    }
                    pub type Parsed = ();
                }
                mod part_1 {
                    use super::*;
                    #[inline(always)]
                    pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                        let state = state.skip_whitespace();
                        let (result, state) = parse_Sequence(state)?;
                        Ok((
                            Parsed {
                                choices: vec![result],
                            },
                            state,
                        ))
                    }
                    #[derive(Debug)]
                    pub struct Parsed {
                        pub choices: Vec<Sequence>,
                    }
                }
                #[inline(always)]
                pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                    let (_, state) = part_0::parse(state)?;
                    let (result, state) = part_1::parse(state)?;
                    let mut choices = result.choices;
                    Ok((Parsed { choices }, state))
                }
                #[derive(Debug)]
                pub struct Parsed {
                    pub choices: Vec<Sequence>,
                }
            }
            #[inline(always)]
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                let mut state = state;
                let mut choices: Vec<Sequence> = Vec::new();
                while let Ok((result, new_state)) = closure::parse(state.clone()) {
                    choices.extend(result.choices);
                    state = new_state;
                }
                Ok((Parsed { choices }, state))
            }
            #[derive(Debug)]
            pub struct Parsed {
                pub choices: Vec<Sequence>,
            }
        }
        #[inline(always)]
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let (result, state) = part_0::parse(state)?;
            let mut choices = result.choices;
            let (result, state) = part_1::parse(state)?;
            choices.extend(result.choices);
            Ok((Parsed { choices }, state))
        }
        use super::Choice as Parsed;
        #[inline(always)]
        pub fn rule_parser(state: ParseState) -> ParseResult<Parsed> {
            parse(state)
        }
    }
    #[inline]
    pub(super) fn parse_Choice(state: ParseState) -> ParseResult<Choice> {
        run_rule_parser(Choice_impl::rule_parser, "Choice", state)
    }
    mod Sequence_impl {
        use super::*;
        mod closure {
            use super::*;
            #[inline(always)]
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                let state = state.skip_whitespace();
                let (result, state) = parse_DelimitedExpression(state)?;
                Ok((
                    Parsed {
                        parts: vec![result],
                    },
                    state,
                ))
            }
            #[derive(Debug)]
            pub struct Parsed {
                pub parts: Vec<DelimitedExpression>,
            }
        }
        #[inline(always)]
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let mut state = state;
            let mut parts: Vec<DelimitedExpression> = Vec::new();
            while let Ok((result, new_state)) = closure::parse(state.clone()) {
                parts.extend(result.parts);
                state = new_state;
            }
            Ok((Parsed { parts }, state))
        }
        use super::Sequence as Parsed;
        #[inline(always)]
        pub fn rule_parser(state: ParseState) -> ParseResult<Parsed> {
            parse(state)
        }
    }
    #[inline]
    pub(super) fn parse_Sequence(state: ParseState) -> ParseResult<Sequence> {
        run_rule_parser(Sequence_impl::rule_parser, "Sequence", state)
    }
    mod Group_impl {
        use super::*;
        mod part_0 {
            use super::*;
            #[inline(always)]
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                let state = state.skip_whitespace();
                parse_string_literal(state, "(")
            }
            pub type Parsed = ();
        }
        mod part_1 {
            use super::*;
            #[inline(always)]
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                let state = state.skip_whitespace();
                let (result, state) = parse_Choice(state)?;
                Ok((Parsed { body: result }, state))
            }
            #[derive(Debug)]
            pub struct Parsed {
                pub body: Choice,
            }
        }
        mod part_2 {
            use super::*;
            #[inline(always)]
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                let state = state.skip_whitespace();
                parse_string_literal(state, ")")
            }
            pub type Parsed = ();
        }
        #[inline(always)]
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let (_, state) = part_0::parse(state)?;
            let (result, state) = part_1::parse(state)?;
            let mut body = result.body;
            let (_, state) = part_2::parse(state)?;
            Ok((Parsed { body }, state))
        }
        use super::Group as Parsed;
        #[inline(always)]
        pub fn rule_parser(state: ParseState) -> ParseResult<Parsed> {
            parse(state)
        }
    }
    #[inline]
    pub(super) fn parse_Group(state: ParseState) -> ParseResult<Group> {
        run_rule_parser(Group_impl::rule_parser, "Group", state)
    }
    mod Optional_impl {
        use super::*;
        mod part_0 {
            use super::*;
            #[inline(always)]
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                let state = state.skip_whitespace();
                parse_string_literal(state, "[")
            }
            pub type Parsed = ();
        }
        mod part_1 {
            use super::*;
            #[inline(always)]
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                let state = state.skip_whitespace();
                let (result, state) = parse_Choice(state)?;
                Ok((Parsed { body: result }, state))
            }
            #[derive(Debug)]
            pub struct Parsed {
                pub body: Choice,
            }
        }
        mod part_2 {
            use super::*;
            #[inline(always)]
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                let state = state.skip_whitespace();
                parse_string_literal(state, "]")
            }
            pub type Parsed = ();
        }
        #[inline(always)]
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let (_, state) = part_0::parse(state)?;
            let (result, state) = part_1::parse(state)?;
            let mut body = result.body;
            let (_, state) = part_2::parse(state)?;
            Ok((Parsed { body }, state))
        }
        use super::Optional as Parsed;
        #[inline(always)]
        pub fn rule_parser(state: ParseState) -> ParseResult<Parsed> {
            parse(state)
        }
    }
    #[inline]
    pub(super) fn parse_Optional(state: ParseState) -> ParseResult<Optional> {
        run_rule_parser(Optional_impl::rule_parser, "Optional", state)
    }
    mod Closure_impl {
        use super::*;
        mod part_0 {
            use super::*;
            #[inline(always)]
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                let state = state.skip_whitespace();
                parse_string_literal(state, "{")
            }
            pub type Parsed = ();
        }
        mod part_1 {
            use super::*;
            #[inline(always)]
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                let state = state.skip_whitespace();
                let (result, state) = parse_Choice(state)?;
                Ok((Parsed { body: result }, state))
            }
            #[derive(Debug)]
            pub struct Parsed {
                pub body: Choice,
            }
        }
        mod part_2 {
            use super::*;
            #[inline(always)]
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                let state = state.skip_whitespace();
                parse_string_literal(state, "}")
            }
            pub type Parsed = ();
        }
        mod part_3 {
            use super::*;
            mod optional {
                use super::*;
                #[inline(always)]
                pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                    let state = state.skip_whitespace();
                    let (result, state) = parse_AtLeastOneMarker(state)?;
                    Ok((
                        Parsed {
                            at_least_one: Some(result),
                        },
                        state,
                    ))
                }
                #[derive(Debug)]
                pub struct Parsed {
                    pub at_least_one: Option<AtLeastOneMarker>,
                }
            }
            #[inline(always)]
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                if let Ok((result, new_state)) = optional::parse(state.clone()) {
                    Ok((
                        Parsed {
                            at_least_one: result.at_least_one,
                        },
                        new_state,
                    ))
                } else {
                    Ok((
                        Parsed {
                            at_least_one: Default::default(),
                        },
                        state,
                    ))
                }
            }
            #[derive(Debug)]
            pub struct Parsed {
                pub at_least_one: Option<AtLeastOneMarker>,
            }
        }
        #[inline(always)]
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let (_, state) = part_0::parse(state)?;
            let (result, state) = part_1::parse(state)?;
            let mut body = result.body;
            let (_, state) = part_2::parse(state)?;
            let (result, state) = part_3::parse(state)?;
            let mut at_least_one = result.at_least_one;
            Ok((Parsed { body, at_least_one }, state))
        }
        use super::Closure as Parsed;
        #[inline(always)]
        pub fn rule_parser(state: ParseState) -> ParseResult<Parsed> {
            parse(state)
        }
    }
    #[inline]
    pub(super) fn parse_Closure(state: ParseState) -> ParseResult<Closure> {
        run_rule_parser(Closure_impl::rule_parser, "Closure", state)
    }
    mod AtLeastOneMarker_impl {
        use super::*;
        #[inline(always)]
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let state = state.skip_whitespace();
            parse_character_literal(state, '+')
        }
        use super::AtLeastOneMarker as Parsed;
        #[inline(always)]
        pub fn rule_parser(state: ParseState) -> ParseResult<Parsed> {
            parse(state)
        }
    }
    #[inline]
    pub(super) fn parse_AtLeastOneMarker(state: ParseState) -> ParseResult<AtLeastOneMarker> {
        run_rule_parser(
            AtLeastOneMarker_impl::rule_parser,
            "AtLeastOneMarker",
            state,
        )
    }
    mod NegativeLookahead_impl {
        use super::*;
        mod part_0 {
            use super::*;
            #[inline(always)]
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                let state = state.skip_whitespace();
                parse_string_literal(state, "!")
            }
            pub type Parsed = ();
        }
        mod part_1 {
            use super::*;
            #[inline(always)]
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                let state = state.skip_whitespace();
                let (result, state) = parse_DelimitedExpression(state)?;
                Ok((
                    Parsed {
                        expr: Box::new(result),
                    },
                    state,
                ))
            }
            #[derive(Debug)]
            pub struct Parsed {
                pub expr: Box<DelimitedExpression>,
            }
        }
        #[inline(always)]
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let (_, state) = part_0::parse(state)?;
            let (result, state) = part_1::parse(state)?;
            let mut expr = result.expr;
            Ok((Parsed { expr }, state))
        }
        use super::NegativeLookahead as Parsed;
        #[inline(always)]
        pub fn rule_parser(state: ParseState) -> ParseResult<Parsed> {
            parse(state)
        }
    }
    #[inline]
    pub(super) fn parse_NegativeLookahead(state: ParseState) -> ParseResult<NegativeLookahead> {
        run_rule_parser(
            NegativeLookahead_impl::rule_parser,
            "NegativeLookahead",
            state,
        )
    }
    mod CharacterRange_impl {
        use super::*;
        mod part_0 {
            use super::*;
            #[inline(always)]
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                let state = state.skip_whitespace();
                let (result, state) = parse_CharacterLiteral(state)?;
                Ok((Parsed { from: result }, state))
            }
            #[derive(Debug)]
            pub struct Parsed {
                pub from: CharacterLiteral,
            }
        }
        mod part_1 {
            use super::*;
            #[inline(always)]
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                let state = state.skip_whitespace();
                parse_string_literal(state, "..")
            }
            pub type Parsed = ();
        }
        mod part_2 {
            use super::*;
            #[inline(always)]
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                let state = state.skip_whitespace();
                let (result, state) = parse_CharacterLiteral(state)?;
                Ok((Parsed { to: result }, state))
            }
            #[derive(Debug)]
            pub struct Parsed {
                pub to: CharacterLiteral,
            }
        }
        #[inline(always)]
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let (result, state) = part_0::parse(state)?;
            let mut from = result.from;
            let (_, state) = part_1::parse(state)?;
            let (result, state) = part_2::parse(state)?;
            let mut to = result.to;
            Ok((Parsed { from, to }, state))
        }
        use super::CharacterRange as Parsed;
        #[inline(always)]
        pub fn rule_parser(state: ParseState) -> ParseResult<Parsed> {
            parse(state)
        }
    }
    #[inline]
    pub(super) fn parse_CharacterRange(state: ParseState) -> ParseResult<CharacterRange> {
        run_rule_parser(CharacterRange_impl::rule_parser, "CharacterRange", state)
    }
    mod CharacterLiteral_impl {
        use super::*;
        mod part_0 {
            use super::*;
            #[inline(always)]
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                let state = state.skip_whitespace();
                parse_string_literal(state, "'")
            }
            pub type Parsed = ();
        }
        mod part_1 {
            use super::*;
            #[inline(always)]
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                let state = state.skip_whitespace();
                let (result, state) = parse_char(state)?;
                Ok((Parsed { _override: result }, state))
            }
            #[derive(Debug)]
            pub struct Parsed {
                pub _override: char,
            }
        }
        mod part_2 {
            use super::*;
            #[inline(always)]
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                let state = state.skip_whitespace();
                parse_string_literal(state, "'")
            }
            pub type Parsed = ();
        }
        #[inline(always)]
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let (_, state) = part_0::parse(state)?;
            let (result, state) = part_1::parse(state)?;
            let mut _override = result._override;
            let (_, state) = part_2::parse(state)?;
            Ok((Parsed { _override }, state))
        }
        pub struct Parsed {
            _override: super::CharacterLiteral,
        }
        use super::CharacterLiteral as Parsed__override;
        #[inline(always)]
        pub fn rule_parser(state: ParseState) -> ParseResult<super::CharacterLiteral> {
            let (result, new_state) = parse(state)?;
            Ok((result._override, new_state))
        }
    }
    #[inline]
    pub(super) fn parse_CharacterLiteral(state: ParseState) -> ParseResult<CharacterLiteral> {
        run_rule_parser(
            CharacterLiteral_impl::rule_parser,
            "CharacterLiteral",
            state,
        )
    }
    mod StringLiteral_impl {
        use super::*;
        mod part_0 {
            use super::*;
            #[inline(always)]
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                let state = state.skip_whitespace();
                parse_character_literal(state, '"')
            }
            pub type Parsed = ();
        }
        mod part_1 {
            use super::*;
            #[inline(always)]
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                let state = state.skip_whitespace();
                let (result, state) = parse_StringLiteralBody(state)?;
                Ok((Parsed { body: result }, state))
            }
            #[derive(Debug)]
            pub struct Parsed {
                pub body: StringLiteralBody,
            }
        }
        mod part_2 {
            use super::*;
            #[inline(always)]
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                let state = state.skip_whitespace();
                parse_character_literal(state, '"')
            }
            pub type Parsed = ();
        }
        #[inline(always)]
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let (_, state) = part_0::parse(state)?;
            let (result, state) = part_1::parse(state)?;
            let mut body = result.body;
            let (_, state) = part_2::parse(state)?;
            Ok((Parsed { body }, state))
        }
        use super::StringLiteral as Parsed;
        #[inline(always)]
        pub fn rule_parser(state: ParseState) -> ParseResult<Parsed> {
            parse(state)
        }
    }
    #[inline]
    pub(super) fn parse_StringLiteral(state: ParseState) -> ParseResult<StringLiteral> {
        run_rule_parser(StringLiteral_impl::rule_parser, "StringLiteral", state)
    }
    mod StringLiteralBody_impl {
        use super::*;
        mod closure {
            use super::*;
            mod choice_0 {
                use super::*;
                #[inline(always)]
                pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                    parse_string_literal(state, "\\\\\\\"")
                }
                pub type Parsed = ();
            }
            mod choice_1 {
                use super::*;
                mod part_0 {
                    use super::*;
                    mod negative_lookahead {
                        use super::*;
                        #[inline(always)]
                        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                            parse_character_literal(state, '"')
                        }
                        pub type Parsed = ();
                    }
                    #[inline(always)]
                    pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                        match negative_lookahead::parse(state.clone()) {
                            Ok(_) => Err(ParseError),
                            Err(_) => Ok(((), state)),
                        }
                    }
                    pub type Parsed = ();
                }
                mod part_1 {
                    use super::*;
                    #[inline(always)]
                    pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                        let (_, state) = parse_char(state)?;
                        Ok(((), state))
                    }
                    pub type Parsed = ();
                }
                #[inline(always)]
                pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                    let (_, state) = part_0::parse(state)?;
                    let (_, state) = part_1::parse(state)?;
                    Ok(((), state))
                }
                pub type Parsed = ();
            }
            #[inline(always)]
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                if let Ok((_, new_state)) = choice_0::parse(state.clone()) {
                    return Ok(((), new_state));
                }
                if let Ok((_, new_state)) = choice_1::parse(state.clone()) {
                    return Ok(((), new_state));
                }
                Err(ParseError)
            }
            pub type Parsed = ();
        }
        #[inline(always)]
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let mut state = state;
            while let Ok((result, new_state)) = closure::parse(state.clone()) {
                state = new_state;
            }
            Ok(((), state))
        }
        pub type Parsed = ();
        #[inline(always)]
        pub fn rule_parser(state: ParseState) -> ParseResult<String> {
            let (_, new_state) = parse(state.clone())?;
            Ok((state.slice_until(&new_state).to_string(), new_state))
        }
    }
    #[inline]
    pub(super) fn parse_StringLiteralBody(state: ParseState) -> ParseResult<StringLiteralBody> {
        run_rule_parser(
            StringLiteralBody_impl::rule_parser,
            "StringLiteralBody",
            state,
        )
    }
    mod Field_impl {
        use super::*;
        mod part_0 {
            use super::*;
            mod optional {
                use super::*;
                mod part_0 {
                    use super::*;
                    #[inline(always)]
                    pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                        let state = state.skip_whitespace();
                        let (result, state) = parse_Identifier(state)?;
                        Ok((Parsed { name: Some(result) }, state))
                    }
                    #[derive(Debug)]
                    pub struct Parsed {
                        pub name: Option<Identifier>,
                    }
                }
                mod part_1 {
                    use super::*;
                    #[inline(always)]
                    pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                        let state = state.skip_whitespace();
                        parse_string_literal(state, ":")
                    }
                    pub type Parsed = ();
                }
                mod part_2 {
                    use super::*;
                    mod optional {
                        use super::*;
                        #[inline(always)]
                        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                            let state = state.skip_whitespace();
                            let (result, state) = parse_BoxMarker(state)?;
                            Ok((
                                Parsed {
                                    boxed: Some(result),
                                },
                                state,
                            ))
                        }
                        #[derive(Debug)]
                        pub struct Parsed {
                            pub boxed: Option<BoxMarker>,
                        }
                    }
                    #[inline(always)]
                    pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                        if let Ok((result, new_state)) = optional::parse(state.clone()) {
                            Ok((
                                Parsed {
                                    boxed: result.boxed,
                                },
                                new_state,
                            ))
                        } else {
                            Ok((
                                Parsed {
                                    boxed: Default::default(),
                                },
                                state,
                            ))
                        }
                    }
                    #[derive(Debug)]
                    pub struct Parsed {
                        pub boxed: Option<BoxMarker>,
                    }
                }
                #[inline(always)]
                pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                    let (result, state) = part_0::parse(state)?;
                    let mut name = result.name;
                    let (_, state) = part_1::parse(state)?;
                    let (result, state) = part_2::parse(state)?;
                    let mut boxed = result.boxed;
                    Ok((Parsed { name, boxed }, state))
                }
                #[derive(Debug)]
                pub struct Parsed {
                    pub name: Option<Identifier>,
                    pub boxed: Option<BoxMarker>,
                }
            }
            #[inline(always)]
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                if let Ok((result, new_state)) = optional::parse(state.clone()) {
                    Ok((
                        Parsed {
                            name: result.name,
                            boxed: result.boxed,
                        },
                        new_state,
                    ))
                } else {
                    Ok((
                        Parsed {
                            name: Default::default(),
                            boxed: Default::default(),
                        },
                        state,
                    ))
                }
            }
            #[derive(Debug)]
            pub struct Parsed {
                pub name: Option<Identifier>,
                pub boxed: Option<BoxMarker>,
            }
        }
        mod part_1 {
            use super::*;
            #[inline(always)]
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                let state = state.skip_whitespace();
                let (result, state) = parse_Identifier(state)?;
                Ok((Parsed { typ: result }, state))
            }
            #[derive(Debug)]
            pub struct Parsed {
                pub typ: Identifier,
            }
        }
        #[inline(always)]
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let (result, state) = part_0::parse(state)?;
            let mut name = result.name;
            let mut boxed = result.boxed;
            let (result, state) = part_1::parse(state)?;
            let mut typ = result.typ;
            Ok((Parsed { name, boxed, typ }, state))
        }
        use super::Field as Parsed;
        #[inline(always)]
        pub fn rule_parser(state: ParseState) -> ParseResult<Parsed> {
            parse(state)
        }
    }
    #[inline]
    pub(super) fn parse_Field(state: ParseState) -> ParseResult<Field> {
        run_rule_parser(Field_impl::rule_parser, "Field", state)
    }
    mod BoxMarker_impl {
        use super::*;
        #[inline(always)]
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let state = state.skip_whitespace();
            parse_character_literal(state, '*')
        }
        use super::BoxMarker as Parsed;
        #[inline(always)]
        pub fn rule_parser(state: ParseState) -> ParseResult<Parsed> {
            parse(state)
        }
    }
    #[inline]
    pub(super) fn parse_BoxMarker(state: ParseState) -> ParseResult<BoxMarker> {
        run_rule_parser(BoxMarker_impl::rule_parser, "BoxMarker", state)
    }
    mod OverrideField_impl {
        use super::*;
        mod part_0 {
            use super::*;
            #[inline(always)]
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                let state = state.skip_whitespace();
                parse_string_literal(state, "@")
            }
            pub type Parsed = ();
        }
        mod part_1 {
            use super::*;
            #[inline(always)]
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                let state = state.skip_whitespace();
                parse_string_literal(state, ":")
            }
            pub type Parsed = ();
        }
        mod part_2 {
            use super::*;
            #[inline(always)]
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                let state = state.skip_whitespace();
                let (result, state) = parse_Identifier(state)?;
                Ok((Parsed { typ: result }, state))
            }
            #[derive(Debug)]
            pub struct Parsed {
                pub typ: Identifier,
            }
        }
        #[inline(always)]
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let (_, state) = part_0::parse(state)?;
            let (_, state) = part_1::parse(state)?;
            let (result, state) = part_2::parse(state)?;
            let mut typ = result.typ;
            Ok((Parsed { typ }, state))
        }
        use super::OverrideField as Parsed;
        #[inline(always)]
        pub fn rule_parser(state: ParseState) -> ParseResult<Parsed> {
            parse(state)
        }
    }
    #[inline]
    pub(super) fn parse_OverrideField(state: ParseState) -> ParseResult<OverrideField> {
        run_rule_parser(OverrideField_impl::rule_parser, "OverrideField", state)
    }
    mod DelimitedExpression_impl {
        use super::*;
        mod choice_0 {
            use super::*;
            #[inline(always)]
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                let state = state.skip_whitespace();
                let (result, state) = parse_Group(state)?;
                Ok((
                    Parsed {
                        _override: Parsed__override::Group(result),
                    },
                    state,
                ))
            }
            #[derive(Debug)]
            pub struct Parsed {
                pub _override: Parsed__override,
            }
        }
        mod choice_1 {
            use super::*;
            #[inline(always)]
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                let state = state.skip_whitespace();
                let (result, state) = parse_Optional(state)?;
                Ok((
                    Parsed {
                        _override: Parsed__override::Optional(result),
                    },
                    state,
                ))
            }
            #[derive(Debug)]
            pub struct Parsed {
                pub _override: Parsed__override,
            }
        }
        mod choice_2 {
            use super::*;
            #[inline(always)]
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                let state = state.skip_whitespace();
                let (result, state) = parse_Closure(state)?;
                Ok((
                    Parsed {
                        _override: Parsed__override::Closure(result),
                    },
                    state,
                ))
            }
            #[derive(Debug)]
            pub struct Parsed {
                pub _override: Parsed__override,
            }
        }
        mod choice_3 {
            use super::*;
            #[inline(always)]
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                let state = state.skip_whitespace();
                let (result, state) = parse_NegativeLookahead(state)?;
                Ok((
                    Parsed {
                        _override: Parsed__override::NegativeLookahead(result),
                    },
                    state,
                ))
            }
            #[derive(Debug)]
            pub struct Parsed {
                pub _override: Parsed__override,
            }
        }
        mod choice_4 {
            use super::*;
            #[inline(always)]
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                let state = state.skip_whitespace();
                let (result, state) = parse_CharacterRange(state)?;
                Ok((
                    Parsed {
                        _override: Parsed__override::CharacterRange(result),
                    },
                    state,
                ))
            }
            #[derive(Debug)]
            pub struct Parsed {
                pub _override: Parsed__override,
            }
        }
        mod choice_5 {
            use super::*;
            #[inline(always)]
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                let state = state.skip_whitespace();
                let (result, state) = parse_CharacterLiteral(state)?;
                Ok((
                    Parsed {
                        _override: Parsed__override::CharacterLiteral(result),
                    },
                    state,
                ))
            }
            #[derive(Debug)]
            pub struct Parsed {
                pub _override: Parsed__override,
            }
        }
        mod choice_6 {
            use super::*;
            #[inline(always)]
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                let state = state.skip_whitespace();
                let (result, state) = parse_StringLiteral(state)?;
                Ok((
                    Parsed {
                        _override: Parsed__override::StringLiteral(result),
                    },
                    state,
                ))
            }
            #[derive(Debug)]
            pub struct Parsed {
                pub _override: Parsed__override,
            }
        }
        mod choice_7 {
            use super::*;
            #[inline(always)]
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                let state = state.skip_whitespace();
                let (result, state) = parse_EndOfInput(state)?;
                Ok((
                    Parsed {
                        _override: Parsed__override::EndOfInput(result),
                    },
                    state,
                ))
            }
            #[derive(Debug)]
            pub struct Parsed {
                pub _override: Parsed__override,
            }
        }
        mod choice_8 {
            use super::*;
            #[inline(always)]
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                let state = state.skip_whitespace();
                let (result, state) = parse_OverrideField(state)?;
                Ok((
                    Parsed {
                        _override: Parsed__override::OverrideField(result),
                    },
                    state,
                ))
            }
            #[derive(Debug)]
            pub struct Parsed {
                pub _override: Parsed__override,
            }
        }
        mod choice_9 {
            use super::*;
            #[inline(always)]
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                let state = state.skip_whitespace();
                let (result, state) = parse_Field(state)?;
                Ok((
                    Parsed {
                        _override: Parsed__override::Field(result),
                    },
                    state,
                ))
            }
            #[derive(Debug)]
            pub struct Parsed {
                pub _override: Parsed__override,
            }
        }
        #[inline(always)]
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            if let Ok((result, new_state)) = choice_0::parse(state.clone()) {
                return Ok((
                    Parsed {
                        _override: result._override,
                    },
                    new_state,
                ));
            }
            if let Ok((result, new_state)) = choice_1::parse(state.clone()) {
                return Ok((
                    Parsed {
                        _override: result._override,
                    },
                    new_state,
                ));
            }
            if let Ok((result, new_state)) = choice_2::parse(state.clone()) {
                return Ok((
                    Parsed {
                        _override: result._override,
                    },
                    new_state,
                ));
            }
            if let Ok((result, new_state)) = choice_3::parse(state.clone()) {
                return Ok((
                    Parsed {
                        _override: result._override,
                    },
                    new_state,
                ));
            }
            if let Ok((result, new_state)) = choice_4::parse(state.clone()) {
                return Ok((
                    Parsed {
                        _override: result._override,
                    },
                    new_state,
                ));
            }
            if let Ok((result, new_state)) = choice_5::parse(state.clone()) {
                return Ok((
                    Parsed {
                        _override: result._override,
                    },
                    new_state,
                ));
            }
            if let Ok((result, new_state)) = choice_6::parse(state.clone()) {
                return Ok((
                    Parsed {
                        _override: result._override,
                    },
                    new_state,
                ));
            }
            if let Ok((result, new_state)) = choice_7::parse(state.clone()) {
                return Ok((
                    Parsed {
                        _override: result._override,
                    },
                    new_state,
                ));
            }
            if let Ok((result, new_state)) = choice_8::parse(state.clone()) {
                return Ok((
                    Parsed {
                        _override: result._override,
                    },
                    new_state,
                ));
            }
            if let Ok((result, new_state)) = choice_9::parse(state.clone()) {
                return Ok((
                    Parsed {
                        _override: result._override,
                    },
                    new_state,
                ));
            }
            Err(ParseError)
        }
        pub struct Parsed {
            _override: super::DelimitedExpression,
        }
        use super::DelimitedExpression as Parsed__override;
        #[inline(always)]
        pub fn rule_parser(state: ParseState) -> ParseResult<super::DelimitedExpression> {
            let (result, new_state) = parse(state)?;
            Ok((result._override, new_state))
        }
    }
    #[inline]
    pub(super) fn parse_DelimitedExpression(state: ParseState) -> ParseResult<DelimitedExpression> {
        run_rule_parser(
            DelimitedExpression_impl::rule_parser,
            "DelimitedExpression",
            state,
        )
    }
    mod Identifier_impl {
        use super::*;
        mod closure {
            use super::*;
            mod choice_0 {
                use super::*;
                #[inline(always)]
                pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                    parse_character_range(state, 'a', 'z')
                }
                pub type Parsed = ();
            }
            mod choice_1 {
                use super::*;
                #[inline(always)]
                pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                    parse_character_range(state, 'A', 'Z')
                }
                pub type Parsed = ();
            }
            mod choice_2 {
                use super::*;
                #[inline(always)]
                pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                    parse_character_range(state, '0', '9')
                }
                pub type Parsed = ();
            }
            mod choice_3 {
                use super::*;
                #[inline(always)]
                pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                    parse_character_literal(state, '_')
                }
                pub type Parsed = ();
            }
            #[inline(always)]
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                if let Ok((_, new_state)) = choice_0::parse(state.clone()) {
                    return Ok(((), new_state));
                }
                if let Ok((_, new_state)) = choice_1::parse(state.clone()) {
                    return Ok(((), new_state));
                }
                if let Ok((_, new_state)) = choice_2::parse(state.clone()) {
                    return Ok(((), new_state));
                }
                if let Ok((_, new_state)) = choice_3::parse(state.clone()) {
                    return Ok(((), new_state));
                }
                Err(ParseError)
            }
            pub type Parsed = ();
        }
        #[inline(always)]
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let mut state = state;
            let (result, new_state) = closure::parse(state)?;
            state = new_state;
            while let Ok((result, new_state)) = closure::parse(state.clone()) {
                state = new_state;
            }
            Ok(((), state))
        }
        pub type Parsed = ();
        #[inline(always)]
        pub fn rule_parser(state: ParseState) -> ParseResult<String> {
            let (_, new_state) = parse(state.clone())?;
            Ok((state.slice_until(&new_state).to_string(), new_state))
        }
    }
    #[inline]
    pub(super) fn parse_Identifier(state: ParseState) -> ParseResult<Identifier> {
        run_rule_parser(Identifier_impl::rule_parser, "Identifier", state)
    }
    mod DirectiveExpression_impl {
        use super::*;
        mod choice_0 {
            use super::*;
            #[inline(always)]
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                let state = state.skip_whitespace();
                let (result, state) = parse_StringDirective(state)?;
                Ok((
                    Parsed {
                        _override: Parsed__override::StringDirective(result),
                    },
                    state,
                ))
            }
            #[derive(Debug)]
            pub struct Parsed {
                pub _override: Parsed__override,
            }
        }
        mod choice_1 {
            use super::*;
            #[inline(always)]
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                let state = state.skip_whitespace();
                let (result, state) = parse_NoSkipWsDirective(state)?;
                Ok((
                    Parsed {
                        _override: Parsed__override::NoSkipWsDirective(result),
                    },
                    state,
                ))
            }
            #[derive(Debug)]
            pub struct Parsed {
                pub _override: Parsed__override,
            }
        }
        mod choice_2 {
            use super::*;
            #[inline(always)]
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                let state = state.skip_whitespace();
                let (result, state) = parse_ExportDirective(state)?;
                Ok((
                    Parsed {
                        _override: Parsed__override::ExportDirective(result),
                    },
                    state,
                ))
            }
            #[derive(Debug)]
            pub struct Parsed {
                pub _override: Parsed__override,
            }
        }
        #[inline(always)]
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            if let Ok((result, new_state)) = choice_0::parse(state.clone()) {
                return Ok((
                    Parsed {
                        _override: result._override,
                    },
                    new_state,
                ));
            }
            if let Ok((result, new_state)) = choice_1::parse(state.clone()) {
                return Ok((
                    Parsed {
                        _override: result._override,
                    },
                    new_state,
                ));
            }
            if let Ok((result, new_state)) = choice_2::parse(state.clone()) {
                return Ok((
                    Parsed {
                        _override: result._override,
                    },
                    new_state,
                ));
            }
            Err(ParseError)
        }
        pub struct Parsed {
            _override: super::DirectiveExpression,
        }
        use super::DirectiveExpression as Parsed__override;
        #[inline(always)]
        pub fn rule_parser(state: ParseState) -> ParseResult<super::DirectiveExpression> {
            let (result, new_state) = parse(state)?;
            Ok((result._override, new_state))
        }
    }
    #[inline]
    pub(super) fn parse_DirectiveExpression(state: ParseState) -> ParseResult<DirectiveExpression> {
        run_rule_parser(
            DirectiveExpression_impl::rule_parser,
            "DirectiveExpression",
            state,
        )
    }
    mod StringDirective_impl {
        use super::*;
        #[inline(always)]
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let state = state.skip_whitespace();
            parse_string_literal(state, "@string")
        }
        use super::StringDirective as Parsed;
        #[inline(always)]
        pub fn rule_parser(state: ParseState) -> ParseResult<Parsed> {
            parse(state)
        }
    }
    #[inline]
    pub(super) fn parse_StringDirective(state: ParseState) -> ParseResult<StringDirective> {
        run_rule_parser(StringDirective_impl::rule_parser, "StringDirective", state)
    }
    mod NoSkipWsDirective_impl {
        use super::*;
        #[inline(always)]
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let state = state.skip_whitespace();
            parse_string_literal(state, "@no_skip_ws")
        }
        use super::NoSkipWsDirective as Parsed;
        #[inline(always)]
        pub fn rule_parser(state: ParseState) -> ParseResult<Parsed> {
            parse(state)
        }
    }
    #[inline]
    pub(super) fn parse_NoSkipWsDirective(state: ParseState) -> ParseResult<NoSkipWsDirective> {
        run_rule_parser(
            NoSkipWsDirective_impl::rule_parser,
            "NoSkipWsDirective",
            state,
        )
    }
    mod ExportDirective_impl {
        use super::*;
        #[inline(always)]
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let state = state.skip_whitespace();
            parse_string_literal(state, "@export")
        }
        use super::ExportDirective as Parsed;
        #[inline(always)]
        pub fn rule_parser(state: ParseState) -> ParseResult<Parsed> {
            parse(state)
        }
    }
    #[inline]
    pub(super) fn parse_ExportDirective(state: ParseState) -> ParseResult<ExportDirective> {
        run_rule_parser(ExportDirective_impl::rule_parser, "ExportDirective", state)
    }
    mod EndOfInput_impl {
        use super::*;
        #[inline(always)]
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let state = state.skip_whitespace();
            parse_character_literal(state, '$')
        }
        use super::EndOfInput as Parsed;
        #[inline(always)]
        pub fn rule_parser(state: ParseState) -> ParseResult<Parsed> {
            parse(state)
        }
    }
    #[inline]
    pub(super) fn parse_EndOfInput(state: ParseState) -> ParseResult<EndOfInput> {
        run_rule_parser(EndOfInput_impl::rule_parser, "EndOfInput", state)
    }
}
