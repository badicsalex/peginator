use crate::runtime::*;
mod Grammar_impl {
    use crate::runtime::*;
    mod closure {
        use crate::runtime::*;
        mod part_0 {
            use crate::runtime::*;
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                let (rules, state) = crate::grammar::test::parse_Rule(state)?;
                Ok((Parsed { rules }, state))
            }
            #[derive(Debug)]
            pub struct Parsed {
                pub rules: crate::grammar::test::Rule,
            }
        }
        mod part_1 {
            use crate::runtime::*;
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                parse_string_literal(state, ";")
            }
            pub type Parsed = ();
        }
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let state = state.skip_whitespace();
            let (result, state) = part_0::parse(state)?;
            let rules = result.rules;
            let state = state.skip_whitespace();
            let (_, state) = part_1::parse(state)?;
            let state = state.skip_whitespace();
            Ok((Parsed { rules }, state))
        }
        #[derive(Debug)]
        pub struct Parsed {
            pub rules: crate::grammar::test::Rule,
        }
    }
    pub fn parse(state: ParseState) -> ParseResult<Parsed> {
        let mut state = state;
        let mut rules: Vec<crate::grammar::test::Rule> = Vec::new();
        while let Ok((result, new_state)) = closure::parse(state.clone()) {
            rules.push(result.rules);
            state = new_state;
        }
        Ok((Parsed { rules }, state))
    }
    #[derive(Debug)]
    pub struct Parsed {
        pub rules: Vec<crate::grammar::test::Rule>,
    }
}
pub use Grammar_impl::parse as parse_Grammar;
pub use Grammar_impl::Parsed as Grammar;
mod Rule_impl {
    use crate::runtime::*;
    mod part_0 {
        use crate::runtime::*;
        mod closure {
            use crate::runtime::*;
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                let (directives, state) = crate::grammar::test::parse_DirectiveExpression(state)?;
                Ok((Parsed { directives }, state))
            }
            #[derive(Debug)]
            pub struct Parsed {
                pub directives: crate::grammar::test::DirectiveExpression,
            }
        }
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let mut state = state;
            let mut directives: Vec<crate::grammar::test::DirectiveExpression> = Vec::new();
            while let Ok((result, new_state)) = closure::parse(state.clone()) {
                directives.push(result.directives);
                state = new_state;
            }
            Ok((Parsed { directives }, state))
        }
        #[derive(Debug)]
        pub struct Parsed {
            pub directives: Vec<crate::grammar::test::DirectiveExpression>,
        }
    }
    mod part_1 {
        use crate::runtime::*;
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let (name, state) = crate::grammar::test::parse_Identifier(state)?;
            Ok((Parsed { name }, state))
        }
        #[derive(Debug)]
        pub struct Parsed {
            pub name: crate::grammar::test::Identifier,
        }
    }
    mod part_2 {
        use crate::runtime::*;
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            parse_string_literal(state, "=")
        }
        pub type Parsed = ();
    }
    mod part_3 {
        use crate::runtime::*;
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let (definition, state) = crate::grammar::test::parse_Choice(state)?;
            Ok((Parsed { definition }, state))
        }
        #[derive(Debug)]
        pub struct Parsed {
            pub definition: crate::grammar::test::Choice,
        }
    }
    pub fn parse(state: ParseState) -> ParseResult<Parsed> {
        let mut directives: Vec<crate::grammar::test::DirectiveExpression> = Vec::new();
        let state = state.skip_whitespace();
        let (result, state) = part_0::parse(state)?;
        directives.extend(result.directives);
        let state = state.skip_whitespace();
        let (result, state) = part_1::parse(state)?;
        let name = result.name;
        let state = state.skip_whitespace();
        let (_, state) = part_2::parse(state)?;
        let state = state.skip_whitespace();
        let (result, state) = part_3::parse(state)?;
        let definition = result.definition;
        let state = state.skip_whitespace();
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
        pub directives: Vec<crate::grammar::test::DirectiveExpression>,
        pub name: crate::grammar::test::Identifier,
        pub definition: crate::grammar::test::Choice,
    }
}
pub use Rule_impl::parse as parse_Rule;
pub use Rule_impl::Parsed as Rule;
mod Choice_impl {
    use crate::runtime::*;
    mod part_0 {
        use crate::runtime::*;
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let (choices, state) = crate::grammar::test::parse_Sequence(state)?;
            Ok((Parsed { choices }, state))
        }
        #[derive(Debug)]
        pub struct Parsed {
            pub choices: crate::grammar::test::Sequence,
        }
    }
    mod part_1 {
        use crate::runtime::*;
        mod closure {
            use crate::runtime::*;
            mod part_0 {
                use crate::runtime::*;
                pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                    parse_string_literal(state, "|")
                }
                pub type Parsed = ();
            }
            mod part_1 {
                use crate::runtime::*;
                pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                    let (choices, state) = crate::grammar::test::parse_Sequence(state)?;
                    Ok((Parsed { choices }, state))
                }
                #[derive(Debug)]
                pub struct Parsed {
                    pub choices: crate::grammar::test::Sequence,
                }
            }
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                let state = state.skip_whitespace();
                let (_, state) = part_0::parse(state)?;
                let state = state.skip_whitespace();
                let (result, state) = part_1::parse(state)?;
                let choices = result.choices;
                let state = state.skip_whitespace();
                Ok((Parsed { choices }, state))
            }
            #[derive(Debug)]
            pub struct Parsed {
                pub choices: crate::grammar::test::Sequence,
            }
        }
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let mut state = state;
            let mut choices: Vec<crate::grammar::test::Sequence> = Vec::new();
            while let Ok((result, new_state)) = closure::parse(state.clone()) {
                choices.push(result.choices);
                state = new_state;
            }
            Ok((Parsed { choices }, state))
        }
        #[derive(Debug)]
        pub struct Parsed {
            pub choices: Vec<crate::grammar::test::Sequence>,
        }
    }
    pub fn parse(state: ParseState) -> ParseResult<Parsed> {
        let mut choices: Vec<crate::grammar::test::Sequence> = Vec::new();
        let state = state.skip_whitespace();
        let (result, state) = part_0::parse(state)?;
        choices.push(result.choices);
        let state = state.skip_whitespace();
        let (result, state) = part_1::parse(state)?;
        choices.extend(result.choices);
        let state = state.skip_whitespace();
        Ok((Parsed { choices }, state))
    }
    #[derive(Debug)]
    pub struct Parsed {
        pub choices: Vec<crate::grammar::test::Sequence>,
    }
}
pub use Choice_impl::parse as parse_Choice;
pub use Choice_impl::Parsed as Choice;
mod Sequence_impl {
    use crate::runtime::*;
    mod closure {
        use crate::runtime::*;
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let (parts, state) = crate::grammar::test::parse_DelimitedExpression(state)?;
            Ok((Parsed { parts }, state))
        }
        #[derive(Debug)]
        pub struct Parsed {
            pub parts: crate::grammar::test::DelimitedExpression,
        }
    }
    pub fn parse(state: ParseState) -> ParseResult<Parsed> {
        let mut state = state;
        let mut parts: Vec<crate::grammar::test::DelimitedExpression> = Vec::new();
        let (result, new_state) = closure::parse(state)?;
        parts.push(result.parts);
        state = new_state;
        while let Ok((result, new_state)) = closure::parse(state.clone()) {
            parts.push(result.parts);
            state = new_state;
        }
        Ok((Parsed { parts }, state))
    }
    #[derive(Debug)]
    pub struct Parsed {
        pub parts: Vec<crate::grammar::test::DelimitedExpression>,
    }
}
pub use Sequence_impl::parse as parse_Sequence;
pub use Sequence_impl::Parsed as Sequence;
mod Group_impl {
    use crate::runtime::*;
    mod part_0 {
        use crate::runtime::*;
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            parse_string_literal(state, "(")
        }
        pub type Parsed = ();
    }
    mod part_1 {
        use crate::runtime::*;
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let (body, state) = crate::grammar::test::parse_Choice(state)?;
            Ok((Parsed { body }, state))
        }
        #[derive(Debug)]
        pub struct Parsed {
            pub body: crate::grammar::test::Choice,
        }
    }
    mod part_2 {
        use crate::runtime::*;
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            parse_string_literal(state, ")")
        }
        pub type Parsed = ();
    }
    pub fn parse(state: ParseState) -> ParseResult<Parsed> {
        let state = state.skip_whitespace();
        let (_, state) = part_0::parse(state)?;
        let state = state.skip_whitespace();
        let (result, state) = part_1::parse(state)?;
        let body = result.body;
        let state = state.skip_whitespace();
        let (_, state) = part_2::parse(state)?;
        let state = state.skip_whitespace();
        Ok((Parsed { body }, state))
    }
    #[derive(Debug)]
    pub struct Parsed {
        pub body: crate::grammar::test::Choice,
    }
}
pub use Group_impl::parse as parse_Group;
pub use Group_impl::Parsed as Group;
mod Closure_impl {
    use crate::runtime::*;
    mod part_0 {
        use crate::runtime::*;
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            parse_string_literal(state, "{")
        }
        pub type Parsed = ();
    }
    mod part_1 {
        use crate::runtime::*;
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let (body, state) = crate::grammar::test::parse_Choice(state)?;
            Ok((Parsed { body }, state))
        }
        #[derive(Debug)]
        pub struct Parsed {
            pub body: crate::grammar::test::Choice,
        }
    }
    mod part_2 {
        use crate::runtime::*;
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            parse_string_literal(state, "}")
        }
        pub type Parsed = ();
    }
    mod part_3 {
        use crate::runtime::*;
        mod choice_0 {
            use crate::runtime::*;
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                let (at_least_one, state) = crate::grammar::test::parse_AtLeastOneMarker(state)?;
                Ok((Parsed { at_least_one }, state))
            }
            #[derive(Debug)]
            pub struct Parsed {
                pub at_least_one: crate::grammar::test::AtLeastOneMarker,
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
            pub at_least_one: Option<crate::grammar::test::AtLeastOneMarker>,
        }
    }
    pub fn parse(state: ParseState) -> ParseResult<Parsed> {
        let mut at_least_one: Option<crate::grammar::test::AtLeastOneMarker> = None;
        let state = state.skip_whitespace();
        let (_, state) = part_0::parse(state)?;
        let state = state.skip_whitespace();
        let (result, state) = part_1::parse(state)?;
        let body = result.body;
        let state = state.skip_whitespace();
        let (_, state) = part_2::parse(state)?;
        let state = state.skip_whitespace();
        let (result, state) = part_3::parse(state)?;
        at_least_one = at_least_one.or(result.at_least_one);
        let state = state.skip_whitespace();
        Ok((Parsed { body, at_least_one }, state))
    }
    #[derive(Debug)]
    pub struct Parsed {
        pub body: crate::grammar::test::Choice,
        pub at_least_one: Option<crate::grammar::test::AtLeastOneMarker>,
    }
}
pub use Closure_impl::parse as parse_Closure;
pub use Closure_impl::Parsed as Closure;
mod AtLeastOneMarker_impl {
    use crate::runtime::*;
    pub fn parse(state: ParseState) -> ParseResult<Parsed> {
        parse_character_literal(state, '+')
    }
    pub type Parsed = ();
}
pub use AtLeastOneMarker_impl::parse as parse_AtLeastOneMarker;
pub use AtLeastOneMarker_impl::Parsed as AtLeastOneMarker;
mod NegativeLookahead_impl {
    use crate::runtime::*;
    mod part_0 {
        use crate::runtime::*;
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            parse_string_literal(state, "!")
        }
        pub type Parsed = ();
    }
    mod part_1 {
        use crate::runtime::*;
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let (expr, state) = crate::grammar::test::parse_DelimitedExpression(state)?;
            Ok((
                Parsed {
                    expr: Box::new(expr),
                },
                state,
            ))
        }
        #[derive(Debug)]
        pub struct Parsed {
            pub expr: Box<crate::grammar::test::DelimitedExpression>,
        }
    }
    pub fn parse(state: ParseState) -> ParseResult<Parsed> {
        let state = state.skip_whitespace();
        let (_, state) = part_0::parse(state)?;
        let state = state.skip_whitespace();
        let (result, state) = part_1::parse(state)?;
        let expr = result.expr;
        let state = state.skip_whitespace();
        Ok((Parsed { expr }, state))
    }
    #[derive(Debug)]
    pub struct Parsed {
        pub expr: Box<crate::grammar::test::DelimitedExpression>,
    }
}
pub use NegativeLookahead_impl::parse as parse_NegativeLookahead;
pub use NegativeLookahead_impl::Parsed as NegativeLookahead;
mod CharacterRange_impl {
    use crate::runtime::*;
    mod part_0 {
        use crate::runtime::*;
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let (from, state) = crate::grammar::test::parse_CharacterLiteral(state)?;
            Ok((Parsed { from }, state))
        }
        #[derive(Debug)]
        pub struct Parsed {
            pub from: crate::grammar::test::CharacterLiteral,
        }
    }
    mod part_1 {
        use crate::runtime::*;
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            parse_string_literal(state, "..")
        }
        pub type Parsed = ();
    }
    mod part_2 {
        use crate::runtime::*;
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let (to, state) = crate::grammar::test::parse_CharacterLiteral(state)?;
            Ok((Parsed { to }, state))
        }
        #[derive(Debug)]
        pub struct Parsed {
            pub to: crate::grammar::test::CharacterLiteral,
        }
    }
    pub fn parse(state: ParseState) -> ParseResult<Parsed> {
        let state = state.skip_whitespace();
        let (result, state) = part_0::parse(state)?;
        let from = result.from;
        let state = state.skip_whitespace();
        let (_, state) = part_1::parse(state)?;
        let state = state.skip_whitespace();
        let (result, state) = part_2::parse(state)?;
        let to = result.to;
        let state = state.skip_whitespace();
        Ok((Parsed { from, to }, state))
    }
    #[derive(Debug)]
    pub struct Parsed {
        pub from: crate::grammar::test::CharacterLiteral,
        pub to: crate::grammar::test::CharacterLiteral,
    }
}
pub use CharacterRange_impl::parse as parse_CharacterRange;
pub use CharacterRange_impl::Parsed as CharacterRange;
mod CharacterLiteral_impl {
    use crate::runtime::*;
    mod part_0 {
        use crate::runtime::*;
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            parse_string_literal(state, "'")
        }
        pub type Parsed = ();
    }
    mod part_1 {
        use crate::runtime::*;
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
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
            parse_string_literal(state, "'")
        }
        pub type Parsed = ();
    }
    pub fn parse(state: ParseState) -> ParseResult<Parsed> {
        let state = state.skip_whitespace();
        let (_, state) = part_0::parse(state)?;
        let state = state.skip_whitespace();
        let (result, state) = part_1::parse(state)?;
        let _override = result._override;
        let state = state.skip_whitespace();
        let (_, state) = part_2::parse(state)?;
        let state = state.skip_whitespace();
        Ok((Parsed { _override }, state))
    }
    #[derive(Debug)]
    pub struct Parsed {
        pub _override: char,
    }
    pub type OverrideType = char;
}
pub use CharacterLiteral_impl::OverrideType as CharacterLiteral;
pub fn parse_CharacterLiteral(state: ParseState) -> ParseResult<CharacterLiteral> {
    let (result, new_state) = CharacterLiteral_impl::parse(state)?;
    Ok((result._override, new_state))
}
mod StringLiteral_impl {
    use crate::runtime::*;
    mod part_0 {
        use crate::runtime::*;
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            parse_character_literal(state, '"')
        }
        pub type Parsed = ();
    }
    mod part_1 {
        use crate::runtime::*;
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let (body, state) = crate::grammar::test::parse_StringLiteralBody(state)?;
            Ok((Parsed { body }, state))
        }
        #[derive(Debug)]
        pub struct Parsed {
            pub body: crate::grammar::test::StringLiteralBody,
        }
    }
    mod part_2 {
        use crate::runtime::*;
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            parse_character_literal(state, '"')
        }
        pub type Parsed = ();
    }
    pub fn parse(state: ParseState) -> ParseResult<Parsed> {
        let state = state.skip_whitespace();
        let (_, state) = part_0::parse(state)?;
        let state = state.skip_whitespace();
        let (result, state) = part_1::parse(state)?;
        let body = result.body;
        let state = state.skip_whitespace();
        let (_, state) = part_2::parse(state)?;
        let state = state.skip_whitespace();
        Ok((Parsed { body }, state))
    }
    #[derive(Debug)]
    pub struct Parsed {
        pub body: crate::grammar::test::StringLiteralBody,
    }
}
pub use StringLiteral_impl::parse as parse_StringLiteral;
pub use StringLiteral_impl::Parsed as StringLiteral;
mod StringLiteralBody_impl {
    use crate::runtime::*;
    mod closure {
        use crate::runtime::*;
        mod choice_0 {
            use crate::runtime::*;
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                parse_string_literal(state, "\\\"")
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
}
pub type StringLiteralBody = String;
pub fn parse_StringLiteralBody(state: ParseState) -> ParseResult<StringLiteralBody> {
    let (_, new_state) = StringLiteralBody_impl::parse(state.clone())?;
    Ok((state.slice_until(&new_state).to_string(), new_state))
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
                    let (name, state) = crate::grammar::test::parse_Identifier(state)?;
                    Ok((Parsed { name }, state))
                }
                #[derive(Debug)]
                pub struct Parsed {
                    pub name: crate::grammar::test::Identifier,
                }
            }
            mod part_1 {
                use crate::runtime::*;
                pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                    parse_string_literal(state, ":")
                }
                pub type Parsed = ();
            }
            mod part_2 {
                use crate::runtime::*;
                mod choice_0 {
                    use crate::runtime::*;
                    pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                        let (boxed, state) = crate::grammar::test::parse_BoxMarker(state)?;
                        Ok((Parsed { boxed }, state))
                    }
                    #[derive(Debug)]
                    pub struct Parsed {
                        pub boxed: crate::grammar::test::BoxMarker,
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
                    pub boxed: Option<crate::grammar::test::BoxMarker>,
                }
            }
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                let mut boxed: Option<crate::grammar::test::BoxMarker> = None;
                let state = state.skip_whitespace();
                let (result, state) = part_0::parse(state)?;
                let name = result.name;
                let state = state.skip_whitespace();
                let (_, state) = part_1::parse(state)?;
                let state = state.skip_whitespace();
                let (result, state) = part_2::parse(state)?;
                boxed = boxed.or(result.boxed);
                let state = state.skip_whitespace();
                Ok((Parsed { name, boxed }, state))
            }
            #[derive(Debug)]
            pub struct Parsed {
                pub name: crate::grammar::test::Identifier,
                pub boxed: Option<crate::grammar::test::BoxMarker>,
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
            pub name: Option<crate::grammar::test::Identifier>,
            pub boxed: Option<crate::grammar::test::BoxMarker>,
        }
    }
    mod part_1 {
        use crate::runtime::*;
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let (typ, state) = crate::grammar::test::parse_Identifier(state)?;
            Ok((Parsed { typ }, state))
        }
        #[derive(Debug)]
        pub struct Parsed {
            pub typ: crate::grammar::test::Identifier,
        }
    }
    pub fn parse(state: ParseState) -> ParseResult<Parsed> {
        let mut name: Option<crate::grammar::test::Identifier> = None;
        let mut boxed: Option<crate::grammar::test::BoxMarker> = None;
        let state = state.skip_whitespace();
        let (result, state) = part_0::parse(state)?;
        name = name.or(result.name);
        boxed = boxed.or(result.boxed);
        let state = state.skip_whitespace();
        let (result, state) = part_1::parse(state)?;
        let typ = result.typ;
        let state = state.skip_whitespace();
        Ok((Parsed { name, boxed, typ }, state))
    }
    #[derive(Debug)]
    pub struct Parsed {
        pub name: Option<crate::grammar::test::Identifier>,
        pub boxed: Option<crate::grammar::test::BoxMarker>,
        pub typ: crate::grammar::test::Identifier,
    }
}
pub use Field_impl::parse as parse_Field;
pub use Field_impl::Parsed as Field;
mod BoxMarker_impl {
    use crate::runtime::*;
    pub fn parse(state: ParseState) -> ParseResult<Parsed> {
        parse_character_literal(state, '*')
    }
    pub type Parsed = ();
}
pub use BoxMarker_impl::parse as parse_BoxMarker;
pub use BoxMarker_impl::Parsed as BoxMarker;
mod OverrideField_impl {
    use crate::runtime::*;
    mod part_0 {
        use crate::runtime::*;
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            parse_string_literal(state, "@")
        }
        pub type Parsed = ();
    }
    mod part_1 {
        use crate::runtime::*;
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            parse_string_literal(state, ":")
        }
        pub type Parsed = ();
    }
    mod part_2 {
        use crate::runtime::*;
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let (typ, state) = crate::grammar::test::parse_Identifier(state)?;
            Ok((Parsed { typ }, state))
        }
        #[derive(Debug)]
        pub struct Parsed {
            pub typ: crate::grammar::test::Identifier,
        }
    }
    pub fn parse(state: ParseState) -> ParseResult<Parsed> {
        let state = state.skip_whitespace();
        let (_, state) = part_0::parse(state)?;
        let state = state.skip_whitespace();
        let (_, state) = part_1::parse(state)?;
        let state = state.skip_whitespace();
        let (result, state) = part_2::parse(state)?;
        let typ = result.typ;
        let state = state.skip_whitespace();
        Ok((Parsed { typ }, state))
    }
    #[derive(Debug)]
    pub struct Parsed {
        pub typ: crate::grammar::test::Identifier,
    }
}
pub use OverrideField_impl::parse as parse_OverrideField;
pub use OverrideField_impl::Parsed as OverrideField;
mod DelimitedExpression_impl {
    use crate::runtime::*;
    mod choice_0 {
        use crate::runtime::*;
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let (_override, state) = crate::grammar::test::parse_Group(state)?;
            Ok((Parsed { _override }, state))
        }
        #[derive(Debug)]
        pub struct Parsed {
            pub _override: crate::grammar::test::Group,
        }
    }
    mod choice_1 {
        use crate::runtime::*;
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let (_override, state) = crate::grammar::test::parse_Closure(state)?;
            Ok((Parsed { _override }, state))
        }
        #[derive(Debug)]
        pub struct Parsed {
            pub _override: crate::grammar::test::Closure,
        }
    }
    mod choice_2 {
        use crate::runtime::*;
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let (_override, state) = crate::grammar::test::parse_NegativeLookahead(state)?;
            Ok((Parsed { _override }, state))
        }
        #[derive(Debug)]
        pub struct Parsed {
            pub _override: crate::grammar::test::NegativeLookahead,
        }
    }
    mod choice_3 {
        use crate::runtime::*;
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let (_override, state) = crate::grammar::test::parse_CharacterRange(state)?;
            Ok((Parsed { _override }, state))
        }
        #[derive(Debug)]
        pub struct Parsed {
            pub _override: crate::grammar::test::CharacterRange,
        }
    }
    mod choice_4 {
        use crate::runtime::*;
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let (_override, state) = crate::grammar::test::parse_CharacterLiteral(state)?;
            Ok((Parsed { _override }, state))
        }
        #[derive(Debug)]
        pub struct Parsed {
            pub _override: crate::grammar::test::CharacterLiteral,
        }
    }
    mod choice_5 {
        use crate::runtime::*;
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let (_override, state) = crate::grammar::test::parse_StringLiteral(state)?;
            Ok((Parsed { _override }, state))
        }
        #[derive(Debug)]
        pub struct Parsed {
            pub _override: crate::grammar::test::StringLiteral,
        }
    }
    mod choice_6 {
        use crate::runtime::*;
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let (_override, state) = crate::grammar::test::parse_OverrideField(state)?;
            Ok((Parsed { _override }, state))
        }
        #[derive(Debug)]
        pub struct Parsed {
            pub _override: crate::grammar::test::OverrideField,
        }
    }
    mod choice_7 {
        use crate::runtime::*;
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let (_override, state) = crate::grammar::test::parse_Field(state)?;
            Ok((Parsed { _override }, state))
        }
        #[derive(Debug)]
        pub struct Parsed {
            pub _override: crate::grammar::test::Field,
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
        CharacterLiteral(crate::grammar::test::CharacterLiteral),
        CharacterRange(crate::grammar::test::CharacterRange),
        Closure(crate::grammar::test::Closure),
        Field(crate::grammar::test::Field),
        Group(crate::grammar::test::Group),
        NegativeLookahead(crate::grammar::test::NegativeLookahead),
        OverrideField(crate::grammar::test::OverrideField),
        StringLiteral(crate::grammar::test::StringLiteral),
    }
    #[derive(Debug)]
    pub struct Parsed {
        pub _override: E__override,
    }
    pub type OverrideType = E__override;
}
pub use DelimitedExpression_impl::OverrideType as DelimitedExpression;
pub fn parse_DelimitedExpression(state: ParseState) -> ParseResult<DelimitedExpression> {
    let (result, new_state) = DelimitedExpression_impl::parse(state)?;
    Ok((result._override, new_state))
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
}
pub type Identifier = String;
pub fn parse_Identifier(state: ParseState) -> ParseResult<Identifier> {
    let (_, new_state) = Identifier_impl::parse(state.clone())?;
    Ok((state.slice_until(&new_state).to_string(), new_state))
}
mod DirectiveExpression_impl {
    use crate::runtime::*;
    mod choice_0 {
        use crate::runtime::*;
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let (_override, state) = crate::grammar::test::parse_StringDirective(state)?;
            Ok((Parsed { _override }, state))
        }
        #[derive(Debug)]
        pub struct Parsed {
            pub _override: crate::grammar::test::StringDirective,
        }
    }
    mod choice_1 {
        use crate::runtime::*;
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let (_override, state) = crate::grammar::test::parse_NoSkipWsDirective(state)?;
            Ok((Parsed { _override }, state))
        }
        #[derive(Debug)]
        pub struct Parsed {
            pub _override: crate::grammar::test::NoSkipWsDirective,
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
        NoSkipWsDirective(crate::grammar::test::NoSkipWsDirective),
        StringDirective(crate::grammar::test::StringDirective),
    }
    #[derive(Debug)]
    pub struct Parsed {
        pub _override: E__override,
    }
    pub type OverrideType = E__override;
}
pub use DirectiveExpression_impl::OverrideType as DirectiveExpression;
pub fn parse_DirectiveExpression(state: ParseState) -> ParseResult<DirectiveExpression> {
    let (result, new_state) = DirectiveExpression_impl::parse(state)?;
    Ok((result._override, new_state))
}
mod StringDirective_impl {
    use crate::runtime::*;
    pub fn parse(state: ParseState) -> ParseResult<Parsed> {
        parse_string_literal(state, "@string")
    }
    pub type Parsed = ();
}
pub use StringDirective_impl::parse as parse_StringDirective;
pub use StringDirective_impl::Parsed as StringDirective;
mod NoSkipWsDirective_impl {
    use crate::runtime::*;
    pub fn parse(state: ParseState) -> ParseResult<Parsed> {
        parse_string_literal(state, "@no_skip_ws")
    }
    pub type Parsed = ();
}
pub use NoSkipWsDirective_impl::parse as parse_NoSkipWsDirective;
pub use NoSkipWsDirective_impl::Parsed as NoSkipWsDirective;

