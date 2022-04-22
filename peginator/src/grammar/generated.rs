use crate::runtime::*;
mod Grammar_impl {
    use crate::runtime::*;
    mod closure {
        use crate::runtime::*;
        mod part_0 {
            use crate::runtime::*;
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                let state = state.skip_whitespace();
                let (rules, state) = crate::grammar::generated::parse_Rule(state)?;
                Ok((Parsed { rules }, state))
            }
            #[derive(Debug)]
            pub struct Parsed {
                pub rules: crate::grammar::generated::Rule,
            }
        }
        mod part_1 {
            use crate::runtime::*;
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                let state = state.skip_whitespace();
                parse_string_literal(state, ";")
            }
            pub type Parsed = ();
        }
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let (result, state) = part_0::parse(state)?;
            let rules = result.rules;
            let (_, state) = part_1::parse(state)?;
            Ok((Parsed { rules }, state))
        }
        #[derive(Debug)]
        pub struct Parsed {
            pub rules: crate::grammar::generated::Rule,
        }
    }
    pub fn parse(state: ParseState) -> ParseResult<Parsed> {
        let mut state = state;
        let mut rules: Vec<crate::grammar::generated::Rule> = Vec::new();
        while let Ok((result, new_state)) = closure::parse(state.clone()) {
            rules.push(result.rules);
            state = new_state;
        }
        Ok((Parsed { rules }, state))
    }
    #[derive(Debug)]
    pub struct Parsed {
        pub rules: Vec<crate::grammar::generated::Rule>,
    }
    pub fn rule_parser(state: ParseState) -> ParseResult<Parsed> {
        parse(state)
    }
}
pub use Grammar_impl::Parsed as Grammar;
pub fn parse_Grammar(state: ParseState) -> ParseResult<Grammar> {
    run_rule_parser(Grammar_impl::rule_parser, "Grammar", state)
}
mod Rule_impl {
    use crate::runtime::*;
    mod part_0 {
        use crate::runtime::*;
        mod closure {
            use crate::runtime::*;
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                let state = state.skip_whitespace();
                let (directives, state) =
                    crate::grammar::generated::parse_DirectiveExpression(state)?;
                Ok((Parsed { directives }, state))
            }
            #[derive(Debug)]
            pub struct Parsed {
                pub directives: crate::grammar::generated::DirectiveExpression,
            }
        }
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let mut state = state;
            let mut directives: Vec<crate::grammar::generated::DirectiveExpression> = Vec::new();
            while let Ok((result, new_state)) = closure::parse(state.clone()) {
                directives.push(result.directives);
                state = new_state;
            }
            Ok((Parsed { directives }, state))
        }
        #[derive(Debug)]
        pub struct Parsed {
            pub directives: Vec<crate::grammar::generated::DirectiveExpression>,
        }
    }
    mod part_1 {
        use crate::runtime::*;
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let state = state.skip_whitespace();
            let (name, state) = crate::grammar::generated::parse_Identifier(state)?;
            Ok((Parsed { name }, state))
        }
        #[derive(Debug)]
        pub struct Parsed {
            pub name: crate::grammar::generated::Identifier,
        }
    }
    mod part_2 {
        use crate::runtime::*;
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let state = state.skip_whitespace();
            parse_string_literal(state, "=")
        }
        pub type Parsed = ();
    }
    mod part_3 {
        use crate::runtime::*;
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let state = state.skip_whitespace();
            let (definition, state) = crate::grammar::generated::parse_Choice(state)?;
            Ok((Parsed { definition }, state))
        }
        #[derive(Debug)]
        pub struct Parsed {
            pub definition: crate::grammar::generated::Choice,
        }
    }
    pub fn parse(state: ParseState) -> ParseResult<Parsed> {
        let mut directives: Vec<crate::grammar::generated::DirectiveExpression> = Vec::new();
        let (result, state) = part_0::parse(state)?;
        directives.extend(result.directives);
        let (result, state) = part_1::parse(state)?;
        let name = result.name;
        let (_, state) = part_2::parse(state)?;
        let (result, state) = part_3::parse(state)?;
        let definition = result.definition;
        Ok((
            Parsed {
                directives,
                name,
                definition,
            },
            state,
        ))
    }
    #[derive(Debug)]
    pub struct Parsed {
        pub directives: Vec<crate::grammar::generated::DirectiveExpression>,
        pub name: crate::grammar::generated::Identifier,
        pub definition: crate::grammar::generated::Choice,
    }
    pub fn rule_parser(state: ParseState) -> ParseResult<Parsed> {
        parse(state)
    }
}
pub use Rule_impl::Parsed as Rule;
pub fn parse_Rule(state: ParseState) -> ParseResult<Rule> {
    run_rule_parser(Rule_impl::rule_parser, "Rule", state)
}
mod Choice_impl {
    use crate::runtime::*;
    mod part_0 {
        use crate::runtime::*;
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let state = state.skip_whitespace();
            let (choices, state) = crate::grammar::generated::parse_Sequence(state)?;
            Ok((Parsed { choices }, state))
        }
        #[derive(Debug)]
        pub struct Parsed {
            pub choices: crate::grammar::generated::Sequence,
        }
    }
    mod part_1 {
        use crate::runtime::*;
        mod closure {
            use crate::runtime::*;
            mod part_0 {
                use crate::runtime::*;
                pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                    let state = state.skip_whitespace();
                    parse_string_literal(state, "|")
                }
                pub type Parsed = ();
            }
            mod part_1 {
                use crate::runtime::*;
                pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                    let state = state.skip_whitespace();
                    let (choices, state) = crate::grammar::generated::parse_Sequence(state)?;
                    Ok((Parsed { choices }, state))
                }
                #[derive(Debug)]
                pub struct Parsed {
                    pub choices: crate::grammar::generated::Sequence,
                }
            }
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                let (_, state) = part_0::parse(state)?;
                let (result, state) = part_1::parse(state)?;
                let choices = result.choices;
                Ok((Parsed { choices }, state))
            }
            #[derive(Debug)]
            pub struct Parsed {
                pub choices: crate::grammar::generated::Sequence,
            }
        }
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let mut state = state;
            let mut choices: Vec<crate::grammar::generated::Sequence> = Vec::new();
            while let Ok((result, new_state)) = closure::parse(state.clone()) {
                choices.push(result.choices);
                state = new_state;
            }
            Ok((Parsed { choices }, state))
        }
        #[derive(Debug)]
        pub struct Parsed {
            pub choices: Vec<crate::grammar::generated::Sequence>,
        }
    }
    pub fn parse(state: ParseState) -> ParseResult<Parsed> {
        let mut choices: Vec<crate::grammar::generated::Sequence> = Vec::new();
        let (result, state) = part_0::parse(state)?;
        choices.push(result.choices);
        let (result, state) = part_1::parse(state)?;
        choices.extend(result.choices);
        Ok((Parsed { choices }, state))
    }
    #[derive(Debug)]
    pub struct Parsed {
        pub choices: Vec<crate::grammar::generated::Sequence>,
    }
    pub fn rule_parser(state: ParseState) -> ParseResult<Parsed> {
        parse(state)
    }
}
pub use Choice_impl::Parsed as Choice;
pub fn parse_Choice(state: ParseState) -> ParseResult<Choice> {
    run_rule_parser(Choice_impl::rule_parser, "Choice", state)
}
mod Sequence_impl {
    use crate::runtime::*;
    mod closure {
        use crate::runtime::*;
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let state = state.skip_whitespace();
            let (parts, state) = crate::grammar::generated::parse_DelimitedExpression(state)?;
            Ok((Parsed { parts }, state))
        }
        #[derive(Debug)]
        pub struct Parsed {
            pub parts: crate::grammar::generated::DelimitedExpression,
        }
    }
    pub fn parse(state: ParseState) -> ParseResult<Parsed> {
        let mut state = state;
        let mut parts: Vec<crate::grammar::generated::DelimitedExpression> = Vec::new();
        while let Ok((result, new_state)) = closure::parse(state.clone()) {
            parts.push(result.parts);
            state = new_state;
        }
        Ok((Parsed { parts }, state))
    }
    #[derive(Debug)]
    pub struct Parsed {
        pub parts: Vec<crate::grammar::generated::DelimitedExpression>,
    }
    pub fn rule_parser(state: ParseState) -> ParseResult<Parsed> {
        parse(state)
    }
}
pub use Sequence_impl::Parsed as Sequence;
pub fn parse_Sequence(state: ParseState) -> ParseResult<Sequence> {
    run_rule_parser(Sequence_impl::rule_parser, "Sequence", state)
}
mod Group_impl {
    use crate::runtime::*;
    mod part_0 {
        use crate::runtime::*;
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let state = state.skip_whitespace();
            parse_string_literal(state, "(")
        }
        pub type Parsed = ();
    }
    mod part_1 {
        use crate::runtime::*;
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let state = state.skip_whitespace();
            let (body, state) = crate::grammar::generated::parse_Choice(state)?;
            Ok((Parsed { body }, state))
        }
        #[derive(Debug)]
        pub struct Parsed {
            pub body: crate::grammar::generated::Choice,
        }
    }
    mod part_2 {
        use crate::runtime::*;
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let state = state.skip_whitespace();
            parse_string_literal(state, ")")
        }
        pub type Parsed = ();
    }
    pub fn parse(state: ParseState) -> ParseResult<Parsed> {
        let (_, state) = part_0::parse(state)?;
        let (result, state) = part_1::parse(state)?;
        let body = result.body;
        let (_, state) = part_2::parse(state)?;
        Ok((Parsed { body }, state))
    }
    #[derive(Debug)]
    pub struct Parsed {
        pub body: crate::grammar::generated::Choice,
    }
    pub fn rule_parser(state: ParseState) -> ParseResult<Parsed> {
        parse(state)
    }
}
pub use Group_impl::Parsed as Group;
pub fn parse_Group(state: ParseState) -> ParseResult<Group> {
    run_rule_parser(Group_impl::rule_parser, "Group", state)
}
mod Closure_impl {
    use crate::runtime::*;
    mod part_0 {
        use crate::runtime::*;
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let state = state.skip_whitespace();
            parse_string_literal(state, "{")
        }
        pub type Parsed = ();
    }
    mod part_1 {
        use crate::runtime::*;
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let state = state.skip_whitespace();
            let (body, state) = crate::grammar::generated::parse_Choice(state)?;
            Ok((Parsed { body }, state))
        }
        #[derive(Debug)]
        pub struct Parsed {
            pub body: crate::grammar::generated::Choice,
        }
    }
    mod part_2 {
        use crate::runtime::*;
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let state = state.skip_whitespace();
            parse_string_literal(state, "}")
        }
        pub type Parsed = ();
    }
    mod part_3 {
        use crate::runtime::*;
        mod choice_0 {
            use crate::runtime::*;
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                let state = state.skip_whitespace();
                let (at_least_one, state) =
                    crate::grammar::generated::parse_AtLeastOneMarker(state)?;
                Ok((Parsed { at_least_one }, state))
            }
            #[derive(Debug)]
            pub struct Parsed {
                pub at_least_one: crate::grammar::generated::AtLeastOneMarker,
            }
        }
        mod choice_1 {
            use crate::runtime::*;
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                Ok(((), state))
            }
            pub type Parsed = ();
        }
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            if let Ok((result, new_state)) = choice_0::parse(state.clone()) {
                return Ok((
                    Parsed {
                        at_least_one: Some(result.at_least_one),
                    },
                    new_state,
                ));
            }
            if let Ok((result, new_state)) = choice_1::parse(state.clone()) {
                return Ok((Parsed { at_least_one: None }, new_state));
            }
            Err(ParseError)
        }
        #[derive(Debug)]
        pub struct Parsed {
            pub at_least_one: Option<crate::grammar::generated::AtLeastOneMarker>,
        }
    }
    pub fn parse(state: ParseState) -> ParseResult<Parsed> {
        let mut at_least_one: Option<crate::grammar::generated::AtLeastOneMarker> = None;
        let (_, state) = part_0::parse(state)?;
        let (result, state) = part_1::parse(state)?;
        let body = result.body;
        let (_, state) = part_2::parse(state)?;
        let (result, state) = part_3::parse(state)?;
        at_least_one = at_least_one.or(result.at_least_one);
        Ok((Parsed { body, at_least_one }, state))
    }
    #[derive(Debug)]
    pub struct Parsed {
        pub body: crate::grammar::generated::Choice,
        pub at_least_one: Option<crate::grammar::generated::AtLeastOneMarker>,
    }
    pub fn rule_parser(state: ParseState) -> ParseResult<Parsed> {
        parse(state)
    }
}
pub use Closure_impl::Parsed as Closure;
pub fn parse_Closure(state: ParseState) -> ParseResult<Closure> {
    run_rule_parser(Closure_impl::rule_parser, "Closure", state)
}
mod AtLeastOneMarker_impl {
    use crate::runtime::*;
    pub fn parse(state: ParseState) -> ParseResult<Parsed> {
        let state = state.skip_whitespace();
        parse_character_literal(state, '+')
    }
    pub type Parsed = ();
    pub fn rule_parser(state: ParseState) -> ParseResult<Parsed> {
        parse(state)
    }
}
pub use AtLeastOneMarker_impl::Parsed as AtLeastOneMarker;
pub fn parse_AtLeastOneMarker(state: ParseState) -> ParseResult<AtLeastOneMarker> {
    run_rule_parser(
        AtLeastOneMarker_impl::rule_parser,
        "AtLeastOneMarker",
        state,
    )
}
mod NegativeLookahead_impl {
    use crate::runtime::*;
    mod part_0 {
        use crate::runtime::*;
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let state = state.skip_whitespace();
            parse_string_literal(state, "!")
        }
        pub type Parsed = ();
    }
    mod part_1 {
        use crate::runtime::*;
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let state = state.skip_whitespace();
            let (expr, state) = crate::grammar::generated::parse_DelimitedExpression(state)?;
            Ok((
                Parsed {
                    expr: Box::new(expr),
                },
                state,
            ))
        }
        #[derive(Debug)]
        pub struct Parsed {
            pub expr: Box<crate::grammar::generated::DelimitedExpression>,
        }
    }
    pub fn parse(state: ParseState) -> ParseResult<Parsed> {
        let (_, state) = part_0::parse(state)?;
        let (result, state) = part_1::parse(state)?;
        let expr = result.expr;
        Ok((Parsed { expr }, state))
    }
    #[derive(Debug)]
    pub struct Parsed {
        pub expr: Box<crate::grammar::generated::DelimitedExpression>,
    }
    pub fn rule_parser(state: ParseState) -> ParseResult<Parsed> {
        parse(state)
    }
}
pub use NegativeLookahead_impl::Parsed as NegativeLookahead;
pub fn parse_NegativeLookahead(state: ParseState) -> ParseResult<NegativeLookahead> {
    run_rule_parser(
        NegativeLookahead_impl::rule_parser,
        "NegativeLookahead",
        state,
    )
}
mod CharacterRange_impl {
    use crate::runtime::*;
    mod part_0 {
        use crate::runtime::*;
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let state = state.skip_whitespace();
            let (from, state) = crate::grammar::generated::parse_CharacterLiteral(state)?;
            Ok((Parsed { from }, state))
        }
        #[derive(Debug)]
        pub struct Parsed {
            pub from: crate::grammar::generated::CharacterLiteral,
        }
    }
    mod part_1 {
        use crate::runtime::*;
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let state = state.skip_whitespace();
            parse_string_literal(state, "..")
        }
        pub type Parsed = ();
    }
    mod part_2 {
        use crate::runtime::*;
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let state = state.skip_whitespace();
            let (to, state) = crate::grammar::generated::parse_CharacterLiteral(state)?;
            Ok((Parsed { to }, state))
        }
        #[derive(Debug)]
        pub struct Parsed {
            pub to: crate::grammar::generated::CharacterLiteral,
        }
    }
    pub fn parse(state: ParseState) -> ParseResult<Parsed> {
        let (result, state) = part_0::parse(state)?;
        let from = result.from;
        let (_, state) = part_1::parse(state)?;
        let (result, state) = part_2::parse(state)?;
        let to = result.to;
        Ok((Parsed { from, to }, state))
    }
    #[derive(Debug)]
    pub struct Parsed {
        pub from: crate::grammar::generated::CharacterLiteral,
        pub to: crate::grammar::generated::CharacterLiteral,
    }
    pub fn rule_parser(state: ParseState) -> ParseResult<Parsed> {
        parse(state)
    }
}
pub use CharacterRange_impl::Parsed as CharacterRange;
pub fn parse_CharacterRange(state: ParseState) -> ParseResult<CharacterRange> {
    run_rule_parser(CharacterRange_impl::rule_parser, "CharacterRange", state)
}
mod CharacterLiteral_impl {
    use crate::runtime::*;
    mod part_0 {
        use crate::runtime::*;
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let state = state.skip_whitespace();
            parse_string_literal(state, "'")
        }
        pub type Parsed = ();
    }
    mod part_1 {
        use crate::runtime::*;
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let state = state.skip_whitespace();
            let (_override, state) = parse_char(state)?;
            Ok((Parsed { _override }, state))
        }
        #[derive(Debug)]
        pub struct Parsed {
            pub _override: char,
        }
    }
    mod part_2 {
        use crate::runtime::*;
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let state = state.skip_whitespace();
            parse_string_literal(state, "'")
        }
        pub type Parsed = ();
    }
    pub fn parse(state: ParseState) -> ParseResult<Parsed> {
        let (_, state) = part_0::parse(state)?;
        let (result, state) = part_1::parse(state)?;
        let _override = result._override;
        let (_, state) = part_2::parse(state)?;
        Ok((Parsed { _override }, state))
    }
    #[derive(Debug)]
    pub struct Parsed {
        pub _override: char,
    }
    pub type OverrideType = char;
    pub fn rule_parser(state: ParseState) -> ParseResult<OverrideType> {
        let (result, new_state) = parse(state)?;
        Ok((result._override, new_state))
    }
}
pub use CharacterLiteral_impl::OverrideType as CharacterLiteral;
pub fn parse_CharacterLiteral(state: ParseState) -> ParseResult<CharacterLiteral> {
    run_rule_parser(
        CharacterLiteral_impl::rule_parser,
        "CharacterLiteral",
        state,
    )
}
mod StringLiteral_impl {
    use crate::runtime::*;
    mod part_0 {
        use crate::runtime::*;
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let state = state.skip_whitespace();
            parse_character_literal(state, '"')
        }
        pub type Parsed = ();
    }
    mod part_1 {
        use crate::runtime::*;
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let state = state.skip_whitespace();
            let (body, state) = crate::grammar::generated::parse_StringLiteralBody(state)?;
            Ok((Parsed { body }, state))
        }
        #[derive(Debug)]
        pub struct Parsed {
            pub body: crate::grammar::generated::StringLiteralBody,
        }
    }
    mod part_2 {
        use crate::runtime::*;
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let state = state.skip_whitespace();
            parse_character_literal(state, '"')
        }
        pub type Parsed = ();
    }
    pub fn parse(state: ParseState) -> ParseResult<Parsed> {
        let (_, state) = part_0::parse(state)?;
        let (result, state) = part_1::parse(state)?;
        let body = result.body;
        let (_, state) = part_2::parse(state)?;
        Ok((Parsed { body }, state))
    }
    #[derive(Debug)]
    pub struct Parsed {
        pub body: crate::grammar::generated::StringLiteralBody,
    }
    pub fn rule_parser(state: ParseState) -> ParseResult<Parsed> {
        parse(state)
    }
}
pub use StringLiteral_impl::Parsed as StringLiteral;
pub fn parse_StringLiteral(state: ParseState) -> ParseResult<StringLiteral> {
    run_rule_parser(StringLiteral_impl::rule_parser, "StringLiteral", state)
}
mod StringLiteralBody_impl {
    use crate::runtime::*;
    mod closure {
        use crate::runtime::*;
        mod choice_0 {
            use crate::runtime::*;
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                parse_string_literal(state, "\\\\\\\"")
            }
            pub type Parsed = ();
        }
        mod choice_1 {
            use crate::runtime::*;
            mod part_0 {
                use crate::runtime::*;
                mod negative_lookahead {
                    use crate::runtime::*;
                    pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                        parse_character_literal(state, '"')
                    }
                    pub type Parsed = ();
                }
                pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                    match negative_lookahead::parse(state.clone()) {
                        Ok(_) => Err(ParseError),
                        Err(_) => Ok(((), state)),
                    }
                }
                pub type Parsed = ();
            }
            mod part_1 {
                use crate::runtime::*;
                pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                    let (_, state) = parse_char(state)?;
                    Ok(((), state))
                }
                pub type Parsed = ();
            }
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                let (_, state) = part_0::parse(state)?;
                let (_, state) = part_1::parse(state)?;
                Ok(((), state))
            }
            pub type Parsed = ();
        }
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
    pub fn parse(state: ParseState) -> ParseResult<Parsed> {
        let mut state = state;
        while let Ok((result, new_state)) = closure::parse(state.clone()) {
            state = new_state;
        }
        Ok(((), state))
    }
    pub type Parsed = ();
    pub fn rule_parser(state: ParseState) -> ParseResult<String> {
        let (_, new_state) = parse(state.clone())?;
        Ok((state.slice_until(&new_state).to_string(), new_state))
    }
}
pub type StringLiteralBody = String;
pub fn parse_StringLiteralBody(state: ParseState) -> ParseResult<StringLiteralBody> {
    run_rule_parser(
        StringLiteralBody_impl::rule_parser,
        "StringLiteralBody",
        state,
    )
}
mod Field_impl {
    use crate::runtime::*;
    mod part_0 {
        use crate::runtime::*;
        mod choice_0 {
            use crate::runtime::*;
            mod part_0 {
                use crate::runtime::*;
                pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                    let state = state.skip_whitespace();
                    let (name, state) = crate::grammar::generated::parse_Identifier(state)?;
                    Ok((Parsed { name }, state))
                }
                #[derive(Debug)]
                pub struct Parsed {
                    pub name: crate::grammar::generated::Identifier,
                }
            }
            mod part_1 {
                use crate::runtime::*;
                pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                    let state = state.skip_whitespace();
                    parse_string_literal(state, ":")
                }
                pub type Parsed = ();
            }
            mod part_2 {
                use crate::runtime::*;
                mod choice_0 {
                    use crate::runtime::*;
                    pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                        let state = state.skip_whitespace();
                        let (boxed, state) = crate::grammar::generated::parse_BoxMarker(state)?;
                        Ok((Parsed { boxed }, state))
                    }
                    #[derive(Debug)]
                    pub struct Parsed {
                        pub boxed: crate::grammar::generated::BoxMarker,
                    }
                }
                mod choice_1 {
                    use crate::runtime::*;
                    pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                        Ok(((), state))
                    }
                    pub type Parsed = ();
                }
                pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                    if let Ok((result, new_state)) = choice_0::parse(state.clone()) {
                        return Ok((
                            Parsed {
                                boxed: Some(result.boxed),
                            },
                            new_state,
                        ));
                    }
                    if let Ok((result, new_state)) = choice_1::parse(state.clone()) {
                        return Ok((Parsed { boxed: None }, new_state));
                    }
                    Err(ParseError)
                }
                #[derive(Debug)]
                pub struct Parsed {
                    pub boxed: Option<crate::grammar::generated::BoxMarker>,
                }
            }
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                let mut boxed: Option<crate::grammar::generated::BoxMarker> = None;
                let (result, state) = part_0::parse(state)?;
                let name = result.name;
                let (_, state) = part_1::parse(state)?;
                let (result, state) = part_2::parse(state)?;
                boxed = boxed.or(result.boxed);
                Ok((Parsed { name, boxed }, state))
            }
            #[derive(Debug)]
            pub struct Parsed {
                pub name: crate::grammar::generated::Identifier,
                pub boxed: Option<crate::grammar::generated::BoxMarker>,
            }
        }
        mod choice_1 {
            use crate::runtime::*;
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                Ok(((), state))
            }
            pub type Parsed = ();
        }
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            if let Ok((result, new_state)) = choice_0::parse(state.clone()) {
                return Ok((
                    Parsed {
                        name: Some(result.name),
                        boxed: result.boxed,
                    },
                    new_state,
                ));
            }
            if let Ok((result, new_state)) = choice_1::parse(state.clone()) {
                return Ok((
                    Parsed {
                        name: None,
                        boxed: None,
                    },
                    new_state,
                ));
            }
            Err(ParseError)
        }
        #[derive(Debug)]
        pub struct Parsed {
            pub name: Option<crate::grammar::generated::Identifier>,
            pub boxed: Option<crate::grammar::generated::BoxMarker>,
        }
    }
    mod part_1 {
        use crate::runtime::*;
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let state = state.skip_whitespace();
            let (typ, state) = crate::grammar::generated::parse_Identifier(state)?;
            Ok((Parsed { typ }, state))
        }
        #[derive(Debug)]
        pub struct Parsed {
            pub typ: crate::grammar::generated::Identifier,
        }
    }
    pub fn parse(state: ParseState) -> ParseResult<Parsed> {
        let mut name: Option<crate::grammar::generated::Identifier> = None;
        let mut boxed: Option<crate::grammar::generated::BoxMarker> = None;
        let (result, state) = part_0::parse(state)?;
        name = name.or(result.name);
        boxed = boxed.or(result.boxed);
        let (result, state) = part_1::parse(state)?;
        let typ = result.typ;
        Ok((Parsed { name, boxed, typ }, state))
    }
    #[derive(Debug)]
    pub struct Parsed {
        pub name: Option<crate::grammar::generated::Identifier>,
        pub boxed: Option<crate::grammar::generated::BoxMarker>,
        pub typ: crate::grammar::generated::Identifier,
    }
    pub fn rule_parser(state: ParseState) -> ParseResult<Parsed> {
        parse(state)
    }
}
pub use Field_impl::Parsed as Field;
pub fn parse_Field(state: ParseState) -> ParseResult<Field> {
    run_rule_parser(Field_impl::rule_parser, "Field", state)
}
mod BoxMarker_impl {
    use crate::runtime::*;
    pub fn parse(state: ParseState) -> ParseResult<Parsed> {
        let state = state.skip_whitespace();
        parse_character_literal(state, '*')
    }
    pub type Parsed = ();
    pub fn rule_parser(state: ParseState) -> ParseResult<Parsed> {
        parse(state)
    }
}
pub use BoxMarker_impl::Parsed as BoxMarker;
pub fn parse_BoxMarker(state: ParseState) -> ParseResult<BoxMarker> {
    run_rule_parser(BoxMarker_impl::rule_parser, "BoxMarker", state)
}
mod OverrideField_impl {
    use crate::runtime::*;
    mod part_0 {
        use crate::runtime::*;
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let state = state.skip_whitespace();
            parse_string_literal(state, "@")
        }
        pub type Parsed = ();
    }
    mod part_1 {
        use crate::runtime::*;
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let state = state.skip_whitespace();
            parse_string_literal(state, ":")
        }
        pub type Parsed = ();
    }
    mod part_2 {
        use crate::runtime::*;
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let state = state.skip_whitespace();
            let (typ, state) = crate::grammar::generated::parse_Identifier(state)?;
            Ok((Parsed { typ }, state))
        }
        #[derive(Debug)]
        pub struct Parsed {
            pub typ: crate::grammar::generated::Identifier,
        }
    }
    pub fn parse(state: ParseState) -> ParseResult<Parsed> {
        let (_, state) = part_0::parse(state)?;
        let (_, state) = part_1::parse(state)?;
        let (result, state) = part_2::parse(state)?;
        let typ = result.typ;
        Ok((Parsed { typ }, state))
    }
    #[derive(Debug)]
    pub struct Parsed {
        pub typ: crate::grammar::generated::Identifier,
    }
    pub fn rule_parser(state: ParseState) -> ParseResult<Parsed> {
        parse(state)
    }
}
pub use OverrideField_impl::Parsed as OverrideField;
pub fn parse_OverrideField(state: ParseState) -> ParseResult<OverrideField> {
    run_rule_parser(OverrideField_impl::rule_parser, "OverrideField", state)
}
mod DelimitedExpression_impl {
    use crate::runtime::*;
    mod choice_0 {
        use crate::runtime::*;
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let state = state.skip_whitespace();
            let (_override, state) = crate::grammar::generated::parse_Group(state)?;
            Ok((Parsed { _override }, state))
        }
        #[derive(Debug)]
        pub struct Parsed {
            pub _override: crate::grammar::generated::Group,
        }
    }
    mod choice_1 {
        use crate::runtime::*;
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let state = state.skip_whitespace();
            let (_override, state) = crate::grammar::generated::parse_Closure(state)?;
            Ok((Parsed { _override }, state))
        }
        #[derive(Debug)]
        pub struct Parsed {
            pub _override: crate::grammar::generated::Closure,
        }
    }
    mod choice_2 {
        use crate::runtime::*;
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let state = state.skip_whitespace();
            let (_override, state) = crate::grammar::generated::parse_NegativeLookahead(state)?;
            Ok((Parsed { _override }, state))
        }
        #[derive(Debug)]
        pub struct Parsed {
            pub _override: crate::grammar::generated::NegativeLookahead,
        }
    }
    mod choice_3 {
        use crate::runtime::*;
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let state = state.skip_whitespace();
            let (_override, state) = crate::grammar::generated::parse_CharacterRange(state)?;
            Ok((Parsed { _override }, state))
        }
        #[derive(Debug)]
        pub struct Parsed {
            pub _override: crate::grammar::generated::CharacterRange,
        }
    }
    mod choice_4 {
        use crate::runtime::*;
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let state = state.skip_whitespace();
            let (_override, state) = crate::grammar::generated::parse_CharacterLiteral(state)?;
            Ok((Parsed { _override }, state))
        }
        #[derive(Debug)]
        pub struct Parsed {
            pub _override: crate::grammar::generated::CharacterLiteral,
        }
    }
    mod choice_5 {
        use crate::runtime::*;
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let state = state.skip_whitespace();
            let (_override, state) = crate::grammar::generated::parse_StringLiteral(state)?;
            Ok((Parsed { _override }, state))
        }
        #[derive(Debug)]
        pub struct Parsed {
            pub _override: crate::grammar::generated::StringLiteral,
        }
    }
    mod choice_6 {
        use crate::runtime::*;
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let state = state.skip_whitespace();
            let (_override, state) = crate::grammar::generated::parse_OverrideField(state)?;
            Ok((Parsed { _override }, state))
        }
        #[derive(Debug)]
        pub struct Parsed {
            pub _override: crate::grammar::generated::OverrideField,
        }
    }
    mod choice_7 {
        use crate::runtime::*;
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let state = state.skip_whitespace();
            let (_override, state) = crate::grammar::generated::parse_Field(state)?;
            Ok((Parsed { _override }, state))
        }
        #[derive(Debug)]
        pub struct Parsed {
            pub _override: crate::grammar::generated::Field,
        }
    }
    pub fn parse(state: ParseState) -> ParseResult<Parsed> {
        if let Ok((result, new_state)) = choice_0::parse(state.clone()) {
            return Ok((
                Parsed {
                    _override: E__override::Group(result._override),
                },
                new_state,
            ));
        }
        if let Ok((result, new_state)) = choice_1::parse(state.clone()) {
            return Ok((
                Parsed {
                    _override: E__override::Closure(result._override),
                },
                new_state,
            ));
        }
        if let Ok((result, new_state)) = choice_2::parse(state.clone()) {
            return Ok((
                Parsed {
                    _override: E__override::NegativeLookahead(result._override),
                },
                new_state,
            ));
        }
        if let Ok((result, new_state)) = choice_3::parse(state.clone()) {
            return Ok((
                Parsed {
                    _override: E__override::CharacterRange(result._override),
                },
                new_state,
            ));
        }
        if let Ok((result, new_state)) = choice_4::parse(state.clone()) {
            return Ok((
                Parsed {
                    _override: E__override::CharacterLiteral(result._override),
                },
                new_state,
            ));
        }
        if let Ok((result, new_state)) = choice_5::parse(state.clone()) {
            return Ok((
                Parsed {
                    _override: E__override::StringLiteral(result._override),
                },
                new_state,
            ));
        }
        if let Ok((result, new_state)) = choice_6::parse(state.clone()) {
            return Ok((
                Parsed {
                    _override: E__override::OverrideField(result._override),
                },
                new_state,
            ));
        }
        if let Ok((result, new_state)) = choice_7::parse(state.clone()) {
            return Ok((
                Parsed {
                    _override: E__override::Field(result._override),
                },
                new_state,
            ));
        }
        Err(ParseError)
    }
    #[derive(Debug)]
    pub enum E__override {
        CharacterLiteral(crate::grammar::generated::CharacterLiteral),
        CharacterRange(crate::grammar::generated::CharacterRange),
        Closure(crate::grammar::generated::Closure),
        Field(crate::grammar::generated::Field),
        Group(crate::grammar::generated::Group),
        NegativeLookahead(crate::grammar::generated::NegativeLookahead),
        OverrideField(crate::grammar::generated::OverrideField),
        StringLiteral(crate::grammar::generated::StringLiteral),
    }
    #[derive(Debug)]
    pub struct Parsed {
        pub _override: E__override,
    }
    pub type OverrideType = E__override;
    pub fn rule_parser(state: ParseState) -> ParseResult<OverrideType> {
        let (result, new_state) = parse(state)?;
        Ok((result._override, new_state))
    }
}
pub use DelimitedExpression_impl::OverrideType as DelimitedExpression;
pub fn parse_DelimitedExpression(state: ParseState) -> ParseResult<DelimitedExpression> {
    run_rule_parser(
        DelimitedExpression_impl::rule_parser,
        "DelimitedExpression",
        state,
    )
}
mod Identifier_impl {
    use crate::runtime::*;
    mod closure {
        use crate::runtime::*;
        mod choice_0 {
            use crate::runtime::*;
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                parse_character_range(state, 'a', 'z')
            }
            pub type Parsed = ();
        }
        mod choice_1 {
            use crate::runtime::*;
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                parse_character_range(state, 'A', 'Z')
            }
            pub type Parsed = ();
        }
        mod choice_2 {
            use crate::runtime::*;
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                parse_character_range(state, '0', '9')
            }
            pub type Parsed = ();
        }
        mod choice_3 {
            use crate::runtime::*;
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                parse_character_literal(state, '_')
            }
            pub type Parsed = ();
        }
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
    pub fn rule_parser(state: ParseState) -> ParseResult<String> {
        let (_, new_state) = parse(state.clone())?;
        Ok((state.slice_until(&new_state).to_string(), new_state))
    }
}
pub type Identifier = String;
pub fn parse_Identifier(state: ParseState) -> ParseResult<Identifier> {
    run_rule_parser(Identifier_impl::rule_parser, "Identifier", state)
}
mod DirectiveExpression_impl {
    use crate::runtime::*;
    mod choice_0 {
        use crate::runtime::*;
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let state = state.skip_whitespace();
            let (_override, state) = crate::grammar::generated::parse_StringDirective(state)?;
            Ok((Parsed { _override }, state))
        }
        #[derive(Debug)]
        pub struct Parsed {
            pub _override: crate::grammar::generated::StringDirective,
        }
    }
    mod choice_1 {
        use crate::runtime::*;
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let state = state.skip_whitespace();
            let (_override, state) = crate::grammar::generated::parse_NoSkipWsDirective(state)?;
            Ok((Parsed { _override }, state))
        }
        #[derive(Debug)]
        pub struct Parsed {
            pub _override: crate::grammar::generated::NoSkipWsDirective,
        }
    }
    pub fn parse(state: ParseState) -> ParseResult<Parsed> {
        if let Ok((result, new_state)) = choice_0::parse(state.clone()) {
            return Ok((
                Parsed {
                    _override: E__override::StringDirective(result._override),
                },
                new_state,
            ));
        }
        if let Ok((result, new_state)) = choice_1::parse(state.clone()) {
            return Ok((
                Parsed {
                    _override: E__override::NoSkipWsDirective(result._override),
                },
                new_state,
            ));
        }
        Err(ParseError)
    }
    #[derive(Debug)]
    pub enum E__override {
        NoSkipWsDirective(crate::grammar::generated::NoSkipWsDirective),
        StringDirective(crate::grammar::generated::StringDirective),
    }
    #[derive(Debug)]
    pub struct Parsed {
        pub _override: E__override,
    }
    pub type OverrideType = E__override;
    pub fn rule_parser(state: ParseState) -> ParseResult<OverrideType> {
        let (result, new_state) = parse(state)?;
        Ok((result._override, new_state))
    }
}
pub use DirectiveExpression_impl::OverrideType as DirectiveExpression;
pub fn parse_DirectiveExpression(state: ParseState) -> ParseResult<DirectiveExpression> {
    run_rule_parser(
        DirectiveExpression_impl::rule_parser,
        "DirectiveExpression",
        state,
    )
}
mod StringDirective_impl {
    use crate::runtime::*;
    pub fn parse(state: ParseState) -> ParseResult<Parsed> {
        let state = state.skip_whitespace();
        parse_string_literal(state, "@string")
    }
    pub type Parsed = ();
    pub fn rule_parser(state: ParseState) -> ParseResult<Parsed> {
        parse(state)
    }
}
pub use StringDirective_impl::Parsed as StringDirective;
pub fn parse_StringDirective(state: ParseState) -> ParseResult<StringDirective> {
    run_rule_parser(StringDirective_impl::rule_parser, "StringDirective", state)
}
mod NoSkipWsDirective_impl {
    use crate::runtime::*;
    pub fn parse(state: ParseState) -> ParseResult<Parsed> {
        let state = state.skip_whitespace();
        parse_string_literal(state, "@no_skip_ws")
    }
    pub type Parsed = ();
    pub fn rule_parser(state: ParseState) -> ParseResult<Parsed> {
        parse(state)
    }
}
pub use NoSkipWsDirective_impl::Parsed as NoSkipWsDirective;
pub fn parse_NoSkipWsDirective(state: ParseState) -> ParseResult<NoSkipWsDirective> {
    run_rule_parser(
        NoSkipWsDirective_impl::rule_parser,
        "NoSkipWsDirective",
        state,
    )
}
