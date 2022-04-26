use crate::runtime::*;
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
pub fn parse_Grammar(s: &str) -> Result<Grammar, ParseError> {
    parse_Grammar_advanced(s, &ParseSettings::default())
}
pub fn parse_Grammar_advanced(s: &str, settings: &ParseSettings) -> Result<Grammar, ParseError> {
    Ok(parse_Grammar_internal(ParseState::new(s, settings))?.0)
}
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
                    let (rules, state) = parse_Rule_internal(state)?;
                    Ok((Parsed { rules }, state))
                }
                #[derive(Debug)]
                pub struct Parsed {
                    pub rules: Rule,
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
                let rules = result.rules;
                let (_, state) = part_1::parse(state)?;
                Ok((Parsed { rules }, state))
            }
            #[derive(Debug)]
            pub struct Parsed {
                pub rules: Rule,
            }
        }
        #[inline(always)]
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let mut state = state;
            let mut rules: Vec<Rule> = Vec::new();
            while let Ok((result, new_state)) = closure::parse(state.clone()) {
                rules.push(result.rules);
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
        let mut rules: Vec<Rule> = Vec::new();
        let (result, state) = part_0::parse(state)?;
        rules.extend(result.rules);
        let (_, state) = part_1::parse(state)?;
        Ok((Parsed { rules }, state))
    }
    use super::Grammar as Parsed;
    #[inline(always)]
    pub fn rule_parser(state: ParseState) -> ParseResult<Parsed> {
        parse(state)
    }
}
fn parse_Grammar_internal(state: ParseState) -> ParseResult<Grammar> {
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
                let (directives, state) = parse_DirectiveExpression_internal(state)?;
                Ok((Parsed { directives }, state))
            }
            #[derive(Debug)]
            pub struct Parsed {
                pub directives: DirectiveExpression,
            }
        }
        #[inline(always)]
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let mut state = state;
            let mut directives: Vec<DirectiveExpression> = Vec::new();
            while let Ok((result, new_state)) = closure::parse(state.clone()) {
                directives.push(result.directives);
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
            let (name, state) = parse_Identifier_internal(state)?;
            Ok((Parsed { name }, state))
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
            let (definition, state) = parse_Choice_internal(state)?;
            Ok((Parsed { definition }, state))
        }
        #[derive(Debug)]
        pub struct Parsed {
            pub definition: Choice,
        }
    }
    #[inline(always)]
    pub fn parse(state: ParseState) -> ParseResult<Parsed> {
        let mut directives: Vec<DirectiveExpression> = Vec::new();
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
    use super::Rule as Parsed;
    #[inline(always)]
    pub fn rule_parser(state: ParseState) -> ParseResult<Parsed> {
        parse(state)
    }
}
fn parse_Rule_internal(state: ParseState) -> ParseResult<Rule> {
    run_rule_parser(Rule_impl::rule_parser, "Rule", state)
}
mod Choice_impl {
    use super::*;
    mod part_0 {
        use super::*;
        #[inline(always)]
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let state = state.skip_whitespace();
            let (choices, state) = parse_Sequence_internal(state)?;
            Ok((Parsed { choices }, state))
        }
        #[derive(Debug)]
        pub struct Parsed {
            pub choices: Sequence,
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
                    let (choices, state) = parse_Sequence_internal(state)?;
                    Ok((Parsed { choices }, state))
                }
                #[derive(Debug)]
                pub struct Parsed {
                    pub choices: Sequence,
                }
            }
            #[inline(always)]
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                let (_, state) = part_0::parse(state)?;
                let (result, state) = part_1::parse(state)?;
                let choices = result.choices;
                Ok((Parsed { choices }, state))
            }
            #[derive(Debug)]
            pub struct Parsed {
                pub choices: Sequence,
            }
        }
        #[inline(always)]
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let mut state = state;
            let mut choices: Vec<Sequence> = Vec::new();
            while let Ok((result, new_state)) = closure::parse(state.clone()) {
                choices.push(result.choices);
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
        let mut choices: Vec<Sequence> = Vec::new();
        let (result, state) = part_0::parse(state)?;
        choices.push(result.choices);
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
fn parse_Choice_internal(state: ParseState) -> ParseResult<Choice> {
    run_rule_parser(Choice_impl::rule_parser, "Choice", state)
}
mod Sequence_impl {
    use super::*;
    mod closure {
        use super::*;
        #[inline(always)]
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let state = state.skip_whitespace();
            let (parts, state) = parse_DelimitedExpression_internal(state)?;
            Ok((Parsed { parts }, state))
        }
        #[derive(Debug)]
        pub struct Parsed {
            pub parts: DelimitedExpression,
        }
    }
    #[inline(always)]
    pub fn parse(state: ParseState) -> ParseResult<Parsed> {
        let mut state = state;
        let mut parts: Vec<DelimitedExpression> = Vec::new();
        while let Ok((result, new_state)) = closure::parse(state.clone()) {
            parts.push(result.parts);
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
fn parse_Sequence_internal(state: ParseState) -> ParseResult<Sequence> {
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
            let (body, state) = parse_Choice_internal(state)?;
            Ok((Parsed { body }, state))
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
        let body = result.body;
        let (_, state) = part_2::parse(state)?;
        Ok((Parsed { body }, state))
    }
    use super::Group as Parsed;
    #[inline(always)]
    pub fn rule_parser(state: ParseState) -> ParseResult<Parsed> {
        parse(state)
    }
}
fn parse_Group_internal(state: ParseState) -> ParseResult<Group> {
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
            let (body, state) = parse_Choice_internal(state)?;
            Ok((Parsed { body }, state))
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
        let body = result.body;
        let (_, state) = part_2::parse(state)?;
        Ok((Parsed { body }, state))
    }
    use super::Optional as Parsed;
    #[inline(always)]
    pub fn rule_parser(state: ParseState) -> ParseResult<Parsed> {
        parse(state)
    }
}
fn parse_Optional_internal(state: ParseState) -> ParseResult<Optional> {
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
            let (body, state) = parse_Choice_internal(state)?;
            Ok((Parsed { body }, state))
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
                let (at_least_one, state) = parse_AtLeastOneMarker_internal(state)?;
                Ok((Parsed { at_least_one }, state))
            }
            #[derive(Debug)]
            pub struct Parsed {
                pub at_least_one: AtLeastOneMarker,
            }
        }
        #[inline(always)]
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            if let Ok((result, new_state)) = optional::parse(state.clone()) {
                Ok((
                    Parsed {
                        at_least_one: Some(result.at_least_one),
                    },
                    new_state,
                ))
            } else {
                Ok((Parsed { at_least_one: None }, state))
            }
        }
        #[derive(Debug)]
        pub struct Parsed {
            pub at_least_one: Option<AtLeastOneMarker>,
        }
    }
    #[inline(always)]
    pub fn parse(state: ParseState) -> ParseResult<Parsed> {
        let mut at_least_one: Option<AtLeastOneMarker> = None;
        let (_, state) = part_0::parse(state)?;
        let (result, state) = part_1::parse(state)?;
        let body = result.body;
        let (_, state) = part_2::parse(state)?;
        let (result, state) = part_3::parse(state)?;
        at_least_one = at_least_one.or(result.at_least_one);
        Ok((Parsed { body, at_least_one }, state))
    }
    use super::Closure as Parsed;
    #[inline(always)]
    pub fn rule_parser(state: ParseState) -> ParseResult<Parsed> {
        parse(state)
    }
}
fn parse_Closure_internal(state: ParseState) -> ParseResult<Closure> {
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
fn parse_AtLeastOneMarker_internal(state: ParseState) -> ParseResult<AtLeastOneMarker> {
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
            let (expr, state) = parse_DelimitedExpression_internal(state)?;
            Ok((
                Parsed {
                    expr: Box::new(expr),
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
        let expr = result.expr;
        Ok((Parsed { expr }, state))
    }
    use super::NegativeLookahead as Parsed;
    #[inline(always)]
    pub fn rule_parser(state: ParseState) -> ParseResult<Parsed> {
        parse(state)
    }
}
fn parse_NegativeLookahead_internal(state: ParseState) -> ParseResult<NegativeLookahead> {
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
            let (from, state) = parse_CharacterLiteral_internal(state)?;
            Ok((Parsed { from }, state))
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
            let (to, state) = parse_CharacterLiteral_internal(state)?;
            Ok((Parsed { to }, state))
        }
        #[derive(Debug)]
        pub struct Parsed {
            pub to: CharacterLiteral,
        }
    }
    #[inline(always)]
    pub fn parse(state: ParseState) -> ParseResult<Parsed> {
        let (result, state) = part_0::parse(state)?;
        let from = result.from;
        let (_, state) = part_1::parse(state)?;
        let (result, state) = part_2::parse(state)?;
        let to = result.to;
        Ok((Parsed { from, to }, state))
    }
    use super::CharacterRange as Parsed;
    #[inline(always)]
    pub fn rule_parser(state: ParseState) -> ParseResult<Parsed> {
        parse(state)
    }
}
fn parse_CharacterRange_internal(state: ParseState) -> ParseResult<CharacterRange> {
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
            let (_override, state) = parse_char_internal(state)?;
            Ok((Parsed { _override }, state))
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
        let _override = result._override;
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
fn parse_CharacterLiteral_internal(state: ParseState) -> ParseResult<CharacterLiteral> {
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
            let (body, state) = parse_StringLiteralBody_internal(state)?;
            Ok((Parsed { body }, state))
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
        let body = result.body;
        let (_, state) = part_2::parse(state)?;
        Ok((Parsed { body }, state))
    }
    use super::StringLiteral as Parsed;
    #[inline(always)]
    pub fn rule_parser(state: ParseState) -> ParseResult<Parsed> {
        parse(state)
    }
}
fn parse_StringLiteral_internal(state: ParseState) -> ParseResult<StringLiteral> {
    run_rule_parser(StringLiteral_impl::rule_parser, "StringLiteral", state)
}
mod StringLiteralBody_impl {
    use super::*;
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
                    let (_, state) = parse_char_internal(state)?;
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
fn parse_StringLiteralBody_internal(state: ParseState) -> ParseResult<StringLiteralBody> {
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
                    let (name, state) = parse_Identifier_internal(state)?;
                    Ok((Parsed { name }, state))
                }
                #[derive(Debug)]
                pub struct Parsed {
                    pub name: Identifier,
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
                        let (boxed, state) = parse_BoxMarker_internal(state)?;
                        Ok((Parsed { boxed }, state))
                    }
                    #[derive(Debug)]
                    pub struct Parsed {
                        pub boxed: BoxMarker,
                    }
                }
                #[inline(always)]
                pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                    if let Ok((result, new_state)) = optional::parse(state.clone()) {
                        Ok((
                            Parsed {
                                boxed: Some(result.boxed),
                            },
                            new_state,
                        ))
                    } else {
                        Ok((Parsed { boxed: None }, state))
                    }
                }
                #[derive(Debug)]
                pub struct Parsed {
                    pub boxed: Option<BoxMarker>,
                }
            }
            #[inline(always)]
            pub fn parse(state: ParseState) -> ParseResult<Parsed> {
                let mut boxed: Option<BoxMarker> = None;
                let (result, state) = part_0::parse(state)?;
                let name = result.name;
                let (_, state) = part_1::parse(state)?;
                let (result, state) = part_2::parse(state)?;
                boxed = boxed.or(result.boxed);
                Ok((Parsed { name, boxed }, state))
            }
            #[derive(Debug)]
            pub struct Parsed {
                pub name: Identifier,
                pub boxed: Option<BoxMarker>,
            }
        }
        #[inline(always)]
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            if let Ok((result, new_state)) = optional::parse(state.clone()) {
                Ok((
                    Parsed {
                        name: Some(result.name),
                        boxed: result.boxed,
                    },
                    new_state,
                ))
            } else {
                Ok((
                    Parsed {
                        name: None,
                        boxed: None,
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
            let (typ, state) = parse_Identifier_internal(state)?;
            Ok((Parsed { typ }, state))
        }
        #[derive(Debug)]
        pub struct Parsed {
            pub typ: Identifier,
        }
    }
    #[inline(always)]
    pub fn parse(state: ParseState) -> ParseResult<Parsed> {
        let mut name: Option<Identifier> = None;
        let mut boxed: Option<BoxMarker> = None;
        let (result, state) = part_0::parse(state)?;
        name = name.or(result.name);
        boxed = boxed.or(result.boxed);
        let (result, state) = part_1::parse(state)?;
        let typ = result.typ;
        Ok((Parsed { name, boxed, typ }, state))
    }
    use super::Field as Parsed;
    #[inline(always)]
    pub fn rule_parser(state: ParseState) -> ParseResult<Parsed> {
        parse(state)
    }
}
fn parse_Field_internal(state: ParseState) -> ParseResult<Field> {
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
fn parse_BoxMarker_internal(state: ParseState) -> ParseResult<BoxMarker> {
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
            let (typ, state) = parse_Identifier_internal(state)?;
            Ok((Parsed { typ }, state))
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
        let typ = result.typ;
        Ok((Parsed { typ }, state))
    }
    use super::OverrideField as Parsed;
    #[inline(always)]
    pub fn rule_parser(state: ParseState) -> ParseResult<Parsed> {
        parse(state)
    }
}
fn parse_OverrideField_internal(state: ParseState) -> ParseResult<OverrideField> {
    run_rule_parser(OverrideField_impl::rule_parser, "OverrideField", state)
}
mod DelimitedExpression_impl {
    use super::*;
    mod choice_0 {
        use super::*;
        #[inline(always)]
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let state = state.skip_whitespace();
            let (_override, state) = parse_Group_internal(state)?;
            Ok((Parsed { _override }, state))
        }
        #[derive(Debug)]
        pub struct Parsed {
            pub _override: Group,
        }
    }
    mod choice_1 {
        use super::*;
        #[inline(always)]
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let state = state.skip_whitespace();
            let (_override, state) = parse_Optional_internal(state)?;
            Ok((Parsed { _override }, state))
        }
        #[derive(Debug)]
        pub struct Parsed {
            pub _override: Optional,
        }
    }
    mod choice_2 {
        use super::*;
        #[inline(always)]
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let state = state.skip_whitespace();
            let (_override, state) = parse_Closure_internal(state)?;
            Ok((Parsed { _override }, state))
        }
        #[derive(Debug)]
        pub struct Parsed {
            pub _override: Closure,
        }
    }
    mod choice_3 {
        use super::*;
        #[inline(always)]
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let state = state.skip_whitespace();
            let (_override, state) = parse_NegativeLookahead_internal(state)?;
            Ok((Parsed { _override }, state))
        }
        #[derive(Debug)]
        pub struct Parsed {
            pub _override: NegativeLookahead,
        }
    }
    mod choice_4 {
        use super::*;
        #[inline(always)]
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let state = state.skip_whitespace();
            let (_override, state) = parse_CharacterRange_internal(state)?;
            Ok((Parsed { _override }, state))
        }
        #[derive(Debug)]
        pub struct Parsed {
            pub _override: CharacterRange,
        }
    }
    mod choice_5 {
        use super::*;
        #[inline(always)]
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let state = state.skip_whitespace();
            let (_override, state) = parse_CharacterLiteral_internal(state)?;
            Ok((Parsed { _override }, state))
        }
        #[derive(Debug)]
        pub struct Parsed {
            pub _override: CharacterLiteral,
        }
    }
    mod choice_6 {
        use super::*;
        #[inline(always)]
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let state = state.skip_whitespace();
            let (_override, state) = parse_StringLiteral_internal(state)?;
            Ok((Parsed { _override }, state))
        }
        #[derive(Debug)]
        pub struct Parsed {
            pub _override: StringLiteral,
        }
    }
    mod choice_7 {
        use super::*;
        #[inline(always)]
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let state = state.skip_whitespace();
            let (_override, state) = parse_EndOfInput_internal(state)?;
            Ok((Parsed { _override }, state))
        }
        #[derive(Debug)]
        pub struct Parsed {
            pub _override: EndOfInput,
        }
    }
    mod choice_8 {
        use super::*;
        #[inline(always)]
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let state = state.skip_whitespace();
            let (_override, state) = parse_OverrideField_internal(state)?;
            Ok((Parsed { _override }, state))
        }
        #[derive(Debug)]
        pub struct Parsed {
            pub _override: OverrideField,
        }
    }
    mod choice_9 {
        use super::*;
        #[inline(always)]
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let state = state.skip_whitespace();
            let (_override, state) = parse_Field_internal(state)?;
            Ok((Parsed { _override }, state))
        }
        #[derive(Debug)]
        pub struct Parsed {
            pub _override: Field,
        }
    }
    #[inline(always)]
    pub fn parse(state: ParseState) -> ParseResult<Parsed> {
        if let Ok((result, new_state)) = choice_0::parse(state.clone()) {
            return Ok((
                Parsed {
                    _override: Parsed__override::Group(result._override),
                },
                new_state,
            ));
        }
        if let Ok((result, new_state)) = choice_1::parse(state.clone()) {
            return Ok((
                Parsed {
                    _override: Parsed__override::Optional(result._override),
                },
                new_state,
            ));
        }
        if let Ok((result, new_state)) = choice_2::parse(state.clone()) {
            return Ok((
                Parsed {
                    _override: Parsed__override::Closure(result._override),
                },
                new_state,
            ));
        }
        if let Ok((result, new_state)) = choice_3::parse(state.clone()) {
            return Ok((
                Parsed {
                    _override: Parsed__override::NegativeLookahead(result._override),
                },
                new_state,
            ));
        }
        if let Ok((result, new_state)) = choice_4::parse(state.clone()) {
            return Ok((
                Parsed {
                    _override: Parsed__override::CharacterRange(result._override),
                },
                new_state,
            ));
        }
        if let Ok((result, new_state)) = choice_5::parse(state.clone()) {
            return Ok((
                Parsed {
                    _override: Parsed__override::CharacterLiteral(result._override),
                },
                new_state,
            ));
        }
        if let Ok((result, new_state)) = choice_6::parse(state.clone()) {
            return Ok((
                Parsed {
                    _override: Parsed__override::StringLiteral(result._override),
                },
                new_state,
            ));
        }
        if let Ok((result, new_state)) = choice_7::parse(state.clone()) {
            return Ok((
                Parsed {
                    _override: Parsed__override::EndOfInput(result._override),
                },
                new_state,
            ));
        }
        if let Ok((result, new_state)) = choice_8::parse(state.clone()) {
            return Ok((
                Parsed {
                    _override: Parsed__override::OverrideField(result._override),
                },
                new_state,
            ));
        }
        if let Ok((result, new_state)) = choice_9::parse(state.clone()) {
            return Ok((
                Parsed {
                    _override: Parsed__override::Field(result._override),
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
fn parse_DelimitedExpression_internal(state: ParseState) -> ParseResult<DelimitedExpression> {
    run_rule_parser(
        DelimitedExpression_impl::rule_parser,
        "DelimitedExpression",
        state,
    )
}
mod Identifier_impl {
    use super::*;
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
fn parse_Identifier_internal(state: ParseState) -> ParseResult<Identifier> {
    run_rule_parser(Identifier_impl::rule_parser, "Identifier", state)
}
mod DirectiveExpression_impl {
    use super::*;
    mod choice_0 {
        use super::*;
        #[inline(always)]
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let state = state.skip_whitespace();
            let (_override, state) = parse_StringDirective_internal(state)?;
            Ok((Parsed { _override }, state))
        }
        #[derive(Debug)]
        pub struct Parsed {
            pub _override: StringDirective,
        }
    }
    mod choice_1 {
        use super::*;
        #[inline(always)]
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let state = state.skip_whitespace();
            let (_override, state) = parse_NoSkipWsDirective_internal(state)?;
            Ok((Parsed { _override }, state))
        }
        #[derive(Debug)]
        pub struct Parsed {
            pub _override: NoSkipWsDirective,
        }
    }
    mod choice_2 {
        use super::*;
        #[inline(always)]
        pub fn parse(state: ParseState) -> ParseResult<Parsed> {
            let state = state.skip_whitespace();
            let (_override, state) = parse_ExportDirective_internal(state)?;
            Ok((Parsed { _override }, state))
        }
        #[derive(Debug)]
        pub struct Parsed {
            pub _override: ExportDirective,
        }
    }
    #[inline(always)]
    pub fn parse(state: ParseState) -> ParseResult<Parsed> {
        if let Ok((result, new_state)) = choice_0::parse(state.clone()) {
            return Ok((
                Parsed {
                    _override: Parsed__override::StringDirective(result._override),
                },
                new_state,
            ));
        }
        if let Ok((result, new_state)) = choice_1::parse(state.clone()) {
            return Ok((
                Parsed {
                    _override: Parsed__override::NoSkipWsDirective(result._override),
                },
                new_state,
            ));
        }
        if let Ok((result, new_state)) = choice_2::parse(state.clone()) {
            return Ok((
                Parsed {
                    _override: Parsed__override::ExportDirective(result._override),
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
fn parse_DirectiveExpression_internal(state: ParseState) -> ParseResult<DirectiveExpression> {
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
fn parse_StringDirective_internal(state: ParseState) -> ParseResult<StringDirective> {
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
fn parse_NoSkipWsDirective_internal(state: ParseState) -> ParseResult<NoSkipWsDirective> {
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
fn parse_ExportDirective_internal(state: ParseState) -> ParseResult<ExportDirective> {
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
fn parse_EndOfInput_internal(state: ParseState) -> ParseResult<EndOfInput> {
    run_rule_parser(EndOfInput_impl::rule_parser, "EndOfInput", state)
}
