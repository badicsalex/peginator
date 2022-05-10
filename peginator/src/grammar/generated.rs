#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grammar {
    pub rules: Vec<Rule>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rule {
    pub directives: Vec<DirectiveExpression>,
    pub name: Identifier,
    pub definition: Choice,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Choice {
    pub choices: Vec<Sequence>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sequence {
    pub parts: Vec<DelimitedExpression>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Group {
    pub body: Choice,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Optional {
    pub body: Choice,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Closure {
    pub body: Choice,
    pub at_least_one: Option<AtLeastOneMarker>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AtLeastOneMarker;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NegativeLookahead {
    pub expr: Box<DelimitedExpression>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharacterRange {
    pub from: CharacterLiteral,
    pub to: CharacterLiteral,
}
pub use char as CharacterLiteral;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringLiteral {
    pub body: StringLiteralBody,
}
pub type StringLiteralBody = String;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Field {
    pub name: Option<Identifier>,
    pub boxed: Option<BoxMarker>,
    pub typ: Identifier,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoxMarker;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OverrideField {
    pub typ: Identifier,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq)]
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DirectiveExpression__override {
    ExportDirective(ExportDirective),
    NoSkipWsDirective(NoSkipWsDirective),
    StringDirective(StringDirective),
}
pub use DirectiveExpression__override as DirectiveExpression;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringDirective;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NoSkipWsDirective;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportDirective;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EndOfInput;
impl peginator_generated::PegParser for Grammar {
    fn parse_advanced(
        s: &str,
        settings: &peginator_generated::ParseSettings,
    ) -> Result<Self, peginator_generated::ParseError> {
        Ok(peginator_generated::parse_Grammar(
            peginator_generated::ParseState::new(s, settings),
            &mut Default::default(),
        )?
        .result)
    }
}
#[allow(non_snake_case, unused_variables, unused_imports, unused_mut)]
mod peginator_generated {
    use super::*;
    use crate::runtime::*;
    pub use crate::runtime::{ParseError, ParseSettings, ParseState, PegParser};
    #[derive(Default)]
    pub struct ParseCache<'a> {
        pub c_Grammar: CacheEntries<'a, Grammar>,
        pub c_Rule: CacheEntries<'a, Rule>,
        pub c_Choice: CacheEntries<'a, Choice>,
        pub c_Sequence: CacheEntries<'a, Sequence>,
        pub c_Group: CacheEntries<'a, Group>,
        pub c_Optional: CacheEntries<'a, Optional>,
        pub c_Closure: CacheEntries<'a, Closure>,
        pub c_AtLeastOneMarker: CacheEntries<'a, AtLeastOneMarker>,
        pub c_NegativeLookahead: CacheEntries<'a, NegativeLookahead>,
        pub c_CharacterRange: CacheEntries<'a, CharacterRange>,
        pub c_CharacterLiteral: CacheEntries<'a, CharacterLiteral>,
        pub c_StringLiteral: CacheEntries<'a, StringLiteral>,
        pub c_StringLiteralBody: CacheEntries<'a, StringLiteralBody>,
        pub c_Field: CacheEntries<'a, Field>,
        pub c_BoxMarker: CacheEntries<'a, BoxMarker>,
        pub c_OverrideField: CacheEntries<'a, OverrideField>,
        pub c_DelimitedExpression: CacheEntries<'a, DelimitedExpression>,
        pub c_Identifier: CacheEntries<'a, Identifier>,
        pub c_DirectiveExpression: CacheEntries<'a, DirectiveExpression>,
        pub c_StringDirective: CacheEntries<'a, StringDirective>,
        pub c_NoSkipWsDirective: CacheEntries<'a, NoSkipWsDirective>,
        pub c_ExportDirective: CacheEntries<'a, ExportDirective>,
        pub c_EndOfInput: CacheEntries<'a, EndOfInput>,
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
                    pub fn parse<'a>(
                        state: ParseState<'a>,
                        cache: &mut ParseCache<'a>,
                    ) -> ParseResult<'a, Parsed> {
                        let state = state.skip_whitespace();
                        let ok_result = parse_Rule(state, cache)?;
                        Ok(ok_result.map(|result| Parsed {
                            rules: vec![result],
                        }))
                    }
                    #[derive(Debug, Clone, PartialEq, Eq)]
                    pub struct Parsed {
                        pub rules: Vec<Rule>,
                    }
                }
                mod part_1 {
                    use super::*;
                    #[inline(always)]
                    pub fn parse<'a>(
                        state: ParseState<'a>,
                        cache: &mut ParseCache<'a>,
                    ) -> ParseResult<'a, Parsed> {
                        let state = state.skip_whitespace();
                        let ok_result = parse_string_literal(state, ";")?;
                        Ok(ok_result.map(|_| Parsed))
                    }
                    #[derive(Debug, Clone, PartialEq, Eq)]
                    pub struct Parsed;
                }
                #[inline(always)]
                pub fn parse<'a>(
                    state: ParseState<'a>,
                    cache: &mut ParseCache<'a>,
                ) -> ParseResult<'a, Parsed> {
                    let mut state = state;
                    let mut farthest_error: Option<ParseError> = None;
                    let result = match part_0::parse(state, cache) {
                        Ok(ParseOk {
                            result,
                            state: new_state,
                            farthest_error: new_farthest_error,
                        }) => {
                            farthest_error = combine_errors(farthest_error, new_farthest_error);
                            state = new_state;
                            result
                        }
                        Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
                    };
                    let mut rules = result.rules;
                    match part_1::parse(state, cache) {
                        Ok(ParseOk {
                            result: _,
                            state: new_state,
                            farthest_error: new_farthest_error,
                        }) => {
                            farthest_error = combine_errors(farthest_error, new_farthest_error);
                            state = new_state;
                        }
                        Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
                    }
                    Ok(ParseOk {
                        result: Parsed { rules },
                        state,
                        farthest_error,
                    })
                }
                #[derive(Debug, Clone, PartialEq, Eq)]
                pub struct Parsed {
                    pub rules: Vec<Rule>,
                }
            }
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let mut rules: Vec<Rule> = Vec::new();
                let mut state = state;
                let mut farthest_error: Option<ParseError> = None;
                loop {
                    match closure::parse(state.clone(), cache) {
                        Ok(ParseOk {
                            result,
                            state: new_state,
                            farthest_error: new_farthest_error,
                        }) => {
                            rules.extend(result.rules);
                            state = new_state;
                            farthest_error = combine_errors(farthest_error, new_farthest_error);
                        }
                        Err(err) => {
                            farthest_error = combine_errors(farthest_error, Some(err));
                            break;
                        }
                    }
                }
                Ok(ParseOk {
                    result: Parsed { rules },
                    state,
                    farthest_error,
                })
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub rules: Vec<Rule>,
            }
        }
        mod part_1 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                if state.is_empty() {
                    Ok(ParseOk {
                        result: Parsed,
                        state,
                        farthest_error: None,
                    })
                } else {
                    Err(state.report_error(ParseErrorSpecifics::ExpectedEoi))
                }
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed;
        }
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let mut state = state;
            let mut farthest_error: Option<ParseError> = None;
            let result = match part_0::parse(state, cache) {
                Ok(ParseOk {
                    result,
                    state: new_state,
                    farthest_error: new_farthest_error,
                }) => {
                    farthest_error = combine_errors(farthest_error, new_farthest_error);
                    state = new_state;
                    result
                }
                Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
            };
            let mut rules = result.rules;
            match part_1::parse(state, cache) {
                Ok(ParseOk {
                    result: _,
                    state: new_state,
                    farthest_error: new_farthest_error,
                }) => {
                    farthest_error = combine_errors(farthest_error, new_farthest_error);
                    state = new_state;
                }
                Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
            }
            Ok(ParseOk {
                result: Parsed { rules },
                state,
                farthest_error,
            })
        }
        use super::Grammar as Parsed;
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            parse(state, cache)
        }
    }
    #[inline]
    pub(super) fn parse_Grammar<'a>(
        state: ParseState<'a>,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, Grammar> {
        let cache_key = state.cache_key();
        if let Some(cached) = cache.c_Grammar.get(&cache_key) {
            cached.clone()
        } else {
            let result = run_rule_parser(Grammar_impl::rule_parser, "Grammar", state, cache);
            cache.c_Grammar.insert(cache_key, result.clone());
            result
        }
    }
    mod Rule_impl {
        use super::*;
        mod part_0 {
            use super::*;
            mod closure {
                use super::*;
                #[inline(always)]
                pub fn parse<'a>(
                    state: ParseState<'a>,
                    cache: &mut ParseCache<'a>,
                ) -> ParseResult<'a, Parsed> {
                    let state = state.skip_whitespace();
                    let ok_result = parse_DirectiveExpression(state, cache)?;
                    Ok(ok_result.map(|result| Parsed {
                        directives: vec![result],
                    }))
                }
                #[derive(Debug, Clone, PartialEq, Eq)]
                pub struct Parsed {
                    pub directives: Vec<DirectiveExpression>,
                }
            }
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let mut directives: Vec<DirectiveExpression> = Vec::new();
                let mut state = state;
                let mut farthest_error: Option<ParseError> = None;
                loop {
                    match closure::parse(state.clone(), cache) {
                        Ok(ParseOk {
                            result,
                            state: new_state,
                            farthest_error: new_farthest_error,
                        }) => {
                            directives.extend(result.directives);
                            state = new_state;
                            farthest_error = combine_errors(farthest_error, new_farthest_error);
                        }
                        Err(err) => {
                            farthest_error = combine_errors(farthest_error, Some(err));
                            break;
                        }
                    }
                }
                Ok(ParseOk {
                    result: Parsed { directives },
                    state,
                    farthest_error,
                })
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub directives: Vec<DirectiveExpression>,
            }
        }
        mod part_1 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_Identifier(state, cache)?;
                Ok(ok_result.map(|result| Parsed { name: result }))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub name: Identifier,
            }
        }
        mod part_2 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_string_literal(state, "=")?;
                Ok(ok_result.map(|_| Parsed))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed;
        }
        mod part_3 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_Choice(state, cache)?;
                Ok(ok_result.map(|result| Parsed { definition: result }))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub definition: Choice,
            }
        }
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let mut state = state;
            let mut farthest_error: Option<ParseError> = None;
            let result = match part_0::parse(state, cache) {
                Ok(ParseOk {
                    result,
                    state: new_state,
                    farthest_error: new_farthest_error,
                }) => {
                    farthest_error = combine_errors(farthest_error, new_farthest_error);
                    state = new_state;
                    result
                }
                Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
            };
            let mut directives = result.directives;
            let result = match part_1::parse(state, cache) {
                Ok(ParseOk {
                    result,
                    state: new_state,
                    farthest_error: new_farthest_error,
                }) => {
                    farthest_error = combine_errors(farthest_error, new_farthest_error);
                    state = new_state;
                    result
                }
                Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
            };
            let mut name = result.name;
            match part_2::parse(state, cache) {
                Ok(ParseOk {
                    result: _,
                    state: new_state,
                    farthest_error: new_farthest_error,
                }) => {
                    farthest_error = combine_errors(farthest_error, new_farthest_error);
                    state = new_state;
                }
                Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
            }
            let result = match part_3::parse(state, cache) {
                Ok(ParseOk {
                    result,
                    state: new_state,
                    farthest_error: new_farthest_error,
                }) => {
                    farthest_error = combine_errors(farthest_error, new_farthest_error);
                    state = new_state;
                    result
                }
                Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
            };
            let mut definition = result.definition;
            Ok(ParseOk {
                result: Parsed {
                    directives,
                    name,
                    definition,
                },
                state,
                farthest_error,
            })
        }
        use super::Rule as Parsed;
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            parse(state, cache)
        }
    }
    #[inline]
    pub(super) fn parse_Rule<'a>(
        state: ParseState<'a>,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, Rule> {
        let cache_key = state.cache_key();
        if let Some(cached) = cache.c_Rule.get(&cache_key) {
            cached.clone()
        } else {
            let result = run_rule_parser(Rule_impl::rule_parser, "Rule", state, cache);
            cache.c_Rule.insert(cache_key, result.clone());
            result
        }
    }
    mod Choice_impl {
        use super::*;
        mod part_0 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_Sequence(state, cache)?;
                Ok(ok_result.map(|result| Parsed {
                    choices: vec![result],
                }))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
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
                    pub fn parse<'a>(
                        state: ParseState<'a>,
                        cache: &mut ParseCache<'a>,
                    ) -> ParseResult<'a, Parsed> {
                        let state = state.skip_whitespace();
                        let ok_result = parse_string_literal(state, "|")?;
                        Ok(ok_result.map(|_| Parsed))
                    }
                    #[derive(Debug, Clone, PartialEq, Eq)]
                    pub struct Parsed;
                }
                mod part_1 {
                    use super::*;
                    #[inline(always)]
                    pub fn parse<'a>(
                        state: ParseState<'a>,
                        cache: &mut ParseCache<'a>,
                    ) -> ParseResult<'a, Parsed> {
                        let state = state.skip_whitespace();
                        let ok_result = parse_Sequence(state, cache)?;
                        Ok(ok_result.map(|result| Parsed {
                            choices: vec![result],
                        }))
                    }
                    #[derive(Debug, Clone, PartialEq, Eq)]
                    pub struct Parsed {
                        pub choices: Vec<Sequence>,
                    }
                }
                #[inline(always)]
                pub fn parse<'a>(
                    state: ParseState<'a>,
                    cache: &mut ParseCache<'a>,
                ) -> ParseResult<'a, Parsed> {
                    let mut state = state;
                    let mut farthest_error: Option<ParseError> = None;
                    match part_0::parse(state, cache) {
                        Ok(ParseOk {
                            result: _,
                            state: new_state,
                            farthest_error: new_farthest_error,
                        }) => {
                            farthest_error = combine_errors(farthest_error, new_farthest_error);
                            state = new_state;
                        }
                        Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
                    }
                    let result = match part_1::parse(state, cache) {
                        Ok(ParseOk {
                            result,
                            state: new_state,
                            farthest_error: new_farthest_error,
                        }) => {
                            farthest_error = combine_errors(farthest_error, new_farthest_error);
                            state = new_state;
                            result
                        }
                        Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
                    };
                    let mut choices = result.choices;
                    Ok(ParseOk {
                        result: Parsed { choices },
                        state,
                        farthest_error,
                    })
                }
                #[derive(Debug, Clone, PartialEq, Eq)]
                pub struct Parsed {
                    pub choices: Vec<Sequence>,
                }
            }
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let mut choices: Vec<Sequence> = Vec::new();
                let mut state = state;
                let mut farthest_error: Option<ParseError> = None;
                loop {
                    match closure::parse(state.clone(), cache) {
                        Ok(ParseOk {
                            result,
                            state: new_state,
                            farthest_error: new_farthest_error,
                        }) => {
                            choices.extend(result.choices);
                            state = new_state;
                            farthest_error = combine_errors(farthest_error, new_farthest_error);
                        }
                        Err(err) => {
                            farthest_error = combine_errors(farthest_error, Some(err));
                            break;
                        }
                    }
                }
                Ok(ParseOk {
                    result: Parsed { choices },
                    state,
                    farthest_error,
                })
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub choices: Vec<Sequence>,
            }
        }
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let mut state = state;
            let mut farthest_error: Option<ParseError> = None;
            let result = match part_0::parse(state, cache) {
                Ok(ParseOk {
                    result,
                    state: new_state,
                    farthest_error: new_farthest_error,
                }) => {
                    farthest_error = combine_errors(farthest_error, new_farthest_error);
                    state = new_state;
                    result
                }
                Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
            };
            let mut choices = result.choices;
            let result = match part_1::parse(state, cache) {
                Ok(ParseOk {
                    result,
                    state: new_state,
                    farthest_error: new_farthest_error,
                }) => {
                    farthest_error = combine_errors(farthest_error, new_farthest_error);
                    state = new_state;
                    result
                }
                Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
            };
            choices.extend(result.choices);
            Ok(ParseOk {
                result: Parsed { choices },
                state,
                farthest_error,
            })
        }
        use super::Choice as Parsed;
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            parse(state, cache)
        }
    }
    #[inline]
    pub(super) fn parse_Choice<'a>(
        state: ParseState<'a>,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, Choice> {
        let cache_key = state.cache_key();
        if let Some(cached) = cache.c_Choice.get(&cache_key) {
            cached.clone()
        } else {
            let result = run_rule_parser(Choice_impl::rule_parser, "Choice", state, cache);
            cache.c_Choice.insert(cache_key, result.clone());
            result
        }
    }
    mod Sequence_impl {
        use super::*;
        mod closure {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_DelimitedExpression(state, cache)?;
                Ok(ok_result.map(|result| Parsed {
                    parts: vec![result],
                }))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub parts: Vec<DelimitedExpression>,
            }
        }
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let mut parts: Vec<DelimitedExpression> = Vec::new();
            let mut state = state;
            let mut farthest_error: Option<ParseError> = None;
            loop {
                match closure::parse(state.clone(), cache) {
                    Ok(ParseOk {
                        result,
                        state: new_state,
                        farthest_error: new_farthest_error,
                    }) => {
                        parts.extend(result.parts);
                        state = new_state;
                        farthest_error = combine_errors(farthest_error, new_farthest_error);
                    }
                    Err(err) => {
                        farthest_error = combine_errors(farthest_error, Some(err));
                        break;
                    }
                }
            }
            Ok(ParseOk {
                result: Parsed { parts },
                state,
                farthest_error,
            })
        }
        use super::Sequence as Parsed;
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            parse(state, cache)
        }
    }
    #[inline]
    pub(super) fn parse_Sequence<'a>(
        state: ParseState<'a>,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, Sequence> {
        let cache_key = state.cache_key();
        if let Some(cached) = cache.c_Sequence.get(&cache_key) {
            cached.clone()
        } else {
            let result = run_rule_parser(Sequence_impl::rule_parser, "Sequence", state, cache);
            cache.c_Sequence.insert(cache_key, result.clone());
            result
        }
    }
    mod Group_impl {
        use super::*;
        mod part_0 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_string_literal(state, "(")?;
                Ok(ok_result.map(|_| Parsed))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed;
        }
        mod part_1 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_Choice(state, cache)?;
                Ok(ok_result.map(|result| Parsed { body: result }))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub body: Choice,
            }
        }
        mod part_2 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_string_literal(state, ")")?;
                Ok(ok_result.map(|_| Parsed))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed;
        }
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let mut state = state;
            let mut farthest_error: Option<ParseError> = None;
            match part_0::parse(state, cache) {
                Ok(ParseOk {
                    result: _,
                    state: new_state,
                    farthest_error: new_farthest_error,
                }) => {
                    farthest_error = combine_errors(farthest_error, new_farthest_error);
                    state = new_state;
                }
                Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
            }
            let result = match part_1::parse(state, cache) {
                Ok(ParseOk {
                    result,
                    state: new_state,
                    farthest_error: new_farthest_error,
                }) => {
                    farthest_error = combine_errors(farthest_error, new_farthest_error);
                    state = new_state;
                    result
                }
                Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
            };
            let mut body = result.body;
            match part_2::parse(state, cache) {
                Ok(ParseOk {
                    result: _,
                    state: new_state,
                    farthest_error: new_farthest_error,
                }) => {
                    farthest_error = combine_errors(farthest_error, new_farthest_error);
                    state = new_state;
                }
                Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
            }
            Ok(ParseOk {
                result: Parsed { body },
                state,
                farthest_error,
            })
        }
        use super::Group as Parsed;
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            parse(state, cache)
        }
    }
    #[inline]
    pub(super) fn parse_Group<'a>(
        state: ParseState<'a>,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, Group> {
        let cache_key = state.cache_key();
        if let Some(cached) = cache.c_Group.get(&cache_key) {
            cached.clone()
        } else {
            let result = run_rule_parser(Group_impl::rule_parser, "Group", state, cache);
            cache.c_Group.insert(cache_key, result.clone());
            result
        }
    }
    mod Optional_impl {
        use super::*;
        mod part_0 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_string_literal(state, "[")?;
                Ok(ok_result.map(|_| Parsed))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed;
        }
        mod part_1 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_Choice(state, cache)?;
                Ok(ok_result.map(|result| Parsed { body: result }))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub body: Choice,
            }
        }
        mod part_2 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_string_literal(state, "]")?;
                Ok(ok_result.map(|_| Parsed))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed;
        }
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let mut state = state;
            let mut farthest_error: Option<ParseError> = None;
            match part_0::parse(state, cache) {
                Ok(ParseOk {
                    result: _,
                    state: new_state,
                    farthest_error: new_farthest_error,
                }) => {
                    farthest_error = combine_errors(farthest_error, new_farthest_error);
                    state = new_state;
                }
                Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
            }
            let result = match part_1::parse(state, cache) {
                Ok(ParseOk {
                    result,
                    state: new_state,
                    farthest_error: new_farthest_error,
                }) => {
                    farthest_error = combine_errors(farthest_error, new_farthest_error);
                    state = new_state;
                    result
                }
                Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
            };
            let mut body = result.body;
            match part_2::parse(state, cache) {
                Ok(ParseOk {
                    result: _,
                    state: new_state,
                    farthest_error: new_farthest_error,
                }) => {
                    farthest_error = combine_errors(farthest_error, new_farthest_error);
                    state = new_state;
                }
                Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
            }
            Ok(ParseOk {
                result: Parsed { body },
                state,
                farthest_error,
            })
        }
        use super::Optional as Parsed;
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            parse(state, cache)
        }
    }
    #[inline]
    pub(super) fn parse_Optional<'a>(
        state: ParseState<'a>,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, Optional> {
        let cache_key = state.cache_key();
        if let Some(cached) = cache.c_Optional.get(&cache_key) {
            cached.clone()
        } else {
            let result = run_rule_parser(Optional_impl::rule_parser, "Optional", state, cache);
            cache.c_Optional.insert(cache_key, result.clone());
            result
        }
    }
    mod Closure_impl {
        use super::*;
        mod part_0 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_string_literal(state, "{")?;
                Ok(ok_result.map(|_| Parsed))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed;
        }
        mod part_1 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_Choice(state, cache)?;
                Ok(ok_result.map(|result| Parsed { body: result }))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub body: Choice,
            }
        }
        mod part_2 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_string_literal(state, "}")?;
                Ok(ok_result.map(|_| Parsed))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed;
        }
        mod part_3 {
            use super::*;
            mod optional {
                use super::*;
                #[inline(always)]
                pub fn parse<'a>(
                    state: ParseState<'a>,
                    cache: &mut ParseCache<'a>,
                ) -> ParseResult<'a, Parsed> {
                    let state = state.skip_whitespace();
                    let ok_result = parse_AtLeastOneMarker(state, cache)?;
                    Ok(ok_result.map(|result| Parsed {
                        at_least_one: Some(result),
                    }))
                }
                #[derive(Debug, Clone, PartialEq, Eq)]
                pub struct Parsed {
                    pub at_least_one: Option<AtLeastOneMarker>,
                }
            }
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                match optional::parse(state.clone(), cache) {
                    Ok(ok_result) => Ok(ok_result.map(|result| Parsed {
                        at_least_one: result.at_least_one,
                    })),
                    Err(err) => Ok(ParseOk {
                        result: Parsed {
                            at_least_one: Default::default(),
                        },
                        state,
                        farthest_error: Some(err),
                    }),
                }
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub at_least_one: Option<AtLeastOneMarker>,
            }
        }
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let mut state = state;
            let mut farthest_error: Option<ParseError> = None;
            match part_0::parse(state, cache) {
                Ok(ParseOk {
                    result: _,
                    state: new_state,
                    farthest_error: new_farthest_error,
                }) => {
                    farthest_error = combine_errors(farthest_error, new_farthest_error);
                    state = new_state;
                }
                Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
            }
            let result = match part_1::parse(state, cache) {
                Ok(ParseOk {
                    result,
                    state: new_state,
                    farthest_error: new_farthest_error,
                }) => {
                    farthest_error = combine_errors(farthest_error, new_farthest_error);
                    state = new_state;
                    result
                }
                Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
            };
            let mut body = result.body;
            match part_2::parse(state, cache) {
                Ok(ParseOk {
                    result: _,
                    state: new_state,
                    farthest_error: new_farthest_error,
                }) => {
                    farthest_error = combine_errors(farthest_error, new_farthest_error);
                    state = new_state;
                }
                Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
            }
            let result = match part_3::parse(state, cache) {
                Ok(ParseOk {
                    result,
                    state: new_state,
                    farthest_error: new_farthest_error,
                }) => {
                    farthest_error = combine_errors(farthest_error, new_farthest_error);
                    state = new_state;
                    result
                }
                Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
            };
            let mut at_least_one = result.at_least_one;
            Ok(ParseOk {
                result: Parsed { body, at_least_one },
                state,
                farthest_error,
            })
        }
        use super::Closure as Parsed;
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            parse(state, cache)
        }
    }
    #[inline]
    pub(super) fn parse_Closure<'a>(
        state: ParseState<'a>,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, Closure> {
        let cache_key = state.cache_key();
        if let Some(cached) = cache.c_Closure.get(&cache_key) {
            cached.clone()
        } else {
            let result = run_rule_parser(Closure_impl::rule_parser, "Closure", state, cache);
            cache.c_Closure.insert(cache_key, result.clone());
            result
        }
    }
    mod AtLeastOneMarker_impl {
        use super::*;
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let state = state.skip_whitespace();
            let ok_result = parse_character_literal(state, '+')?;
            Ok(ok_result.map(|_| Parsed))
        }
        use super::AtLeastOneMarker as Parsed;
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            parse(state, cache)
        }
    }
    #[inline]
    pub(super) fn parse_AtLeastOneMarker<'a>(
        state: ParseState<'a>,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, AtLeastOneMarker> {
        let cache_key = state.cache_key();
        if let Some(cached) = cache.c_AtLeastOneMarker.get(&cache_key) {
            cached.clone()
        } else {
            let result = run_rule_parser(
                AtLeastOneMarker_impl::rule_parser,
                "AtLeastOneMarker",
                state,
                cache,
            );
            cache.c_AtLeastOneMarker.insert(cache_key, result.clone());
            result
        }
    }
    mod NegativeLookahead_impl {
        use super::*;
        mod part_0 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_string_literal(state, "!")?;
                Ok(ok_result.map(|_| Parsed))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed;
        }
        mod part_1 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_DelimitedExpression(state, cache)?;
                Ok(ok_result.map(|result| Parsed {
                    expr: Box::new(result),
                }))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub expr: Box<DelimitedExpression>,
            }
        }
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let mut state = state;
            let mut farthest_error: Option<ParseError> = None;
            match part_0::parse(state, cache) {
                Ok(ParseOk {
                    result: _,
                    state: new_state,
                    farthest_error: new_farthest_error,
                }) => {
                    farthest_error = combine_errors(farthest_error, new_farthest_error);
                    state = new_state;
                }
                Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
            }
            let result = match part_1::parse(state, cache) {
                Ok(ParseOk {
                    result,
                    state: new_state,
                    farthest_error: new_farthest_error,
                }) => {
                    farthest_error = combine_errors(farthest_error, new_farthest_error);
                    state = new_state;
                    result
                }
                Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
            };
            let mut expr = result.expr;
            Ok(ParseOk {
                result: Parsed { expr },
                state,
                farthest_error,
            })
        }
        use super::NegativeLookahead as Parsed;
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            parse(state, cache)
        }
    }
    #[inline]
    pub(super) fn parse_NegativeLookahead<'a>(
        state: ParseState<'a>,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, NegativeLookahead> {
        let cache_key = state.cache_key();
        if let Some(cached) = cache.c_NegativeLookahead.get(&cache_key) {
            cached.clone()
        } else {
            let result = run_rule_parser(
                NegativeLookahead_impl::rule_parser,
                "NegativeLookahead",
                state,
                cache,
            );
            cache.c_NegativeLookahead.insert(cache_key, result.clone());
            result
        }
    }
    mod CharacterRange_impl {
        use super::*;
        mod part_0 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_CharacterLiteral(state, cache)?;
                Ok(ok_result.map(|result| Parsed { from: result }))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub from: CharacterLiteral,
            }
        }
        mod part_1 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_string_literal(state, "..")?;
                Ok(ok_result.map(|_| Parsed))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed;
        }
        mod part_2 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_CharacterLiteral(state, cache)?;
                Ok(ok_result.map(|result| Parsed { to: result }))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub to: CharacterLiteral,
            }
        }
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let mut state = state;
            let mut farthest_error: Option<ParseError> = None;
            let result = match part_0::parse(state, cache) {
                Ok(ParseOk {
                    result,
                    state: new_state,
                    farthest_error: new_farthest_error,
                }) => {
                    farthest_error = combine_errors(farthest_error, new_farthest_error);
                    state = new_state;
                    result
                }
                Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
            };
            let mut from = result.from;
            match part_1::parse(state, cache) {
                Ok(ParseOk {
                    result: _,
                    state: new_state,
                    farthest_error: new_farthest_error,
                }) => {
                    farthest_error = combine_errors(farthest_error, new_farthest_error);
                    state = new_state;
                }
                Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
            }
            let result = match part_2::parse(state, cache) {
                Ok(ParseOk {
                    result,
                    state: new_state,
                    farthest_error: new_farthest_error,
                }) => {
                    farthest_error = combine_errors(farthest_error, new_farthest_error);
                    state = new_state;
                    result
                }
                Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
            };
            let mut to = result.to;
            Ok(ParseOk {
                result: Parsed { from, to },
                state,
                farthest_error,
            })
        }
        use super::CharacterRange as Parsed;
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            parse(state, cache)
        }
    }
    #[inline]
    pub(super) fn parse_CharacterRange<'a>(
        state: ParseState<'a>,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, CharacterRange> {
        let cache_key = state.cache_key();
        if let Some(cached) = cache.c_CharacterRange.get(&cache_key) {
            cached.clone()
        } else {
            let result = run_rule_parser(
                CharacterRange_impl::rule_parser,
                "CharacterRange",
                state,
                cache,
            );
            cache.c_CharacterRange.insert(cache_key, result.clone());
            result
        }
    }
    mod CharacterLiteral_impl {
        use super::*;
        mod part_0 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_string_literal(state, "'")?;
                Ok(ok_result.map(|_| Parsed))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed;
        }
        mod part_1 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_char(state, cache)?;
                Ok(ok_result.map(|result| Parsed { _override: result }))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub _override: char,
            }
        }
        mod part_2 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_string_literal(state, "'")?;
                Ok(ok_result.map(|_| Parsed))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed;
        }
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let mut state = state;
            let mut farthest_error: Option<ParseError> = None;
            match part_0::parse(state, cache) {
                Ok(ParseOk {
                    result: _,
                    state: new_state,
                    farthest_error: new_farthest_error,
                }) => {
                    farthest_error = combine_errors(farthest_error, new_farthest_error);
                    state = new_state;
                }
                Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
            }
            let result = match part_1::parse(state, cache) {
                Ok(ParseOk {
                    result,
                    state: new_state,
                    farthest_error: new_farthest_error,
                }) => {
                    farthest_error = combine_errors(farthest_error, new_farthest_error);
                    state = new_state;
                    result
                }
                Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
            };
            let mut _override = result._override;
            match part_2::parse(state, cache) {
                Ok(ParseOk {
                    result: _,
                    state: new_state,
                    farthest_error: new_farthest_error,
                }) => {
                    farthest_error = combine_errors(farthest_error, new_farthest_error);
                    state = new_state;
                }
                Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
            }
            Ok(ParseOk {
                result: Parsed { _override },
                state,
                farthest_error,
            })
        }
        pub struct Parsed {
            _override: super::CharacterLiteral,
        }
        use super::CharacterLiteral as Parsed__override;
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::CharacterLiteral> {
            let ok_result = parse(state, cache)?;
            Ok(ok_result.map(|result| result._override))
        }
    }
    #[inline]
    pub(super) fn parse_CharacterLiteral<'a>(
        state: ParseState<'a>,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, CharacterLiteral> {
        let cache_key = state.cache_key();
        if let Some(cached) = cache.c_CharacterLiteral.get(&cache_key) {
            cached.clone()
        } else {
            let result = run_rule_parser(
                CharacterLiteral_impl::rule_parser,
                "CharacterLiteral",
                state,
                cache,
            );
            cache.c_CharacterLiteral.insert(cache_key, result.clone());
            result
        }
    }
    mod StringLiteral_impl {
        use super::*;
        mod part_0 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_character_literal(state, '"')?;
                Ok(ok_result.map(|_| Parsed))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed;
        }
        mod part_1 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_StringLiteralBody(state, cache)?;
                Ok(ok_result.map(|result| Parsed { body: result }))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub body: StringLiteralBody,
            }
        }
        mod part_2 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_character_literal(state, '"')?;
                Ok(ok_result.map(|_| Parsed))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed;
        }
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let mut state = state;
            let mut farthest_error: Option<ParseError> = None;
            match part_0::parse(state, cache) {
                Ok(ParseOk {
                    result: _,
                    state: new_state,
                    farthest_error: new_farthest_error,
                }) => {
                    farthest_error = combine_errors(farthest_error, new_farthest_error);
                    state = new_state;
                }
                Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
            }
            let result = match part_1::parse(state, cache) {
                Ok(ParseOk {
                    result,
                    state: new_state,
                    farthest_error: new_farthest_error,
                }) => {
                    farthest_error = combine_errors(farthest_error, new_farthest_error);
                    state = new_state;
                    result
                }
                Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
            };
            let mut body = result.body;
            match part_2::parse(state, cache) {
                Ok(ParseOk {
                    result: _,
                    state: new_state,
                    farthest_error: new_farthest_error,
                }) => {
                    farthest_error = combine_errors(farthest_error, new_farthest_error);
                    state = new_state;
                }
                Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
            }
            Ok(ParseOk {
                result: Parsed { body },
                state,
                farthest_error,
            })
        }
        use super::StringLiteral as Parsed;
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            parse(state, cache)
        }
    }
    #[inline]
    pub(super) fn parse_StringLiteral<'a>(
        state: ParseState<'a>,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, StringLiteral> {
        let cache_key = state.cache_key();
        if let Some(cached) = cache.c_StringLiteral.get(&cache_key) {
            cached.clone()
        } else {
            let result = run_rule_parser(
                StringLiteral_impl::rule_parser,
                "StringLiteral",
                state,
                cache,
            );
            cache.c_StringLiteral.insert(cache_key, result.clone());
            result
        }
    }
    mod StringLiteralBody_impl {
        use super::*;
        mod closure {
            use super::*;
            mod choice_0 {
                use super::*;
                #[inline(always)]
                pub fn parse<'a>(
                    state: ParseState<'a>,
                    cache: &mut ParseCache<'a>,
                ) -> ParseResult<'a, Parsed> {
                    let ok_result = parse_string_literal(state, "\\\\\\\"")?;
                    Ok(ok_result.map(|_| Parsed))
                }
                #[derive(Debug, Clone, PartialEq, Eq)]
                pub struct Parsed;
            }
            mod choice_1 {
                use super::*;
                mod part_0 {
                    use super::*;
                    mod negative_lookahead {
                        use super::*;
                        #[inline(always)]
                        pub fn parse<'a>(
                            state: ParseState<'a>,
                            cache: &mut ParseCache<'a>,
                        ) -> ParseResult<'a, Parsed> {
                            let ok_result = parse_character_literal(state, '"')?;
                            Ok(ok_result.map(|_| Parsed))
                        }
                        #[derive(Debug, Clone, PartialEq, Eq)]
                        pub struct Parsed;
                    }
                    #[inline(always)]
                    pub fn parse<'a>(
                        state: ParseState<'a>,
                        cache: &mut ParseCache<'a>,
                    ) -> ParseResult<'a, Parsed> {
                        match negative_lookahead::parse(state.clone(), cache) {
                            Ok(_) => {
                                Err(state
                                    .report_error(ParseErrorSpecifics::NegativeLookaheadFailed))
                            }
                            Err(_) => Ok(ParseOk {
                                result: Parsed,
                                state,
                                farthest_error: None,
                            }),
                        }
                    }
                    #[derive(Debug, Clone, PartialEq, Eq)]
                    pub struct Parsed;
                }
                mod part_1 {
                    use super::*;
                    #[inline(always)]
                    pub fn parse<'a>(
                        state: ParseState<'a>,
                        cache: &mut ParseCache<'a>,
                    ) -> ParseResult<'a, Parsed> {
                        let ok_result = parse_char(state, cache)?;
                        Ok(ok_result.map(|_| Parsed))
                    }
                    #[derive(Debug, Clone, PartialEq, Eq)]
                    pub struct Parsed;
                }
                #[inline(always)]
                pub fn parse<'a>(
                    state: ParseState<'a>,
                    cache: &mut ParseCache<'a>,
                ) -> ParseResult<'a, Parsed> {
                    let mut state = state;
                    let mut farthest_error: Option<ParseError> = None;
                    match part_0::parse(state, cache) {
                        Ok(ParseOk {
                            result: _,
                            state: new_state,
                            farthest_error: new_farthest_error,
                        }) => {
                            farthest_error = combine_errors(farthest_error, new_farthest_error);
                            state = new_state;
                        }
                        Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
                    }
                    match part_1::parse(state, cache) {
                        Ok(ParseOk {
                            result: _,
                            state: new_state,
                            farthest_error: new_farthest_error,
                        }) => {
                            farthest_error = combine_errors(farthest_error, new_farthest_error);
                            state = new_state;
                        }
                        Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
                    }
                    Ok(ParseOk {
                        result: Parsed {},
                        state,
                        farthest_error,
                    })
                }
                #[derive(Debug, Clone, PartialEq, Eq)]
                pub struct Parsed;
            }
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let mut state = state;
                let mut farthest_error: Option<ParseError> = None;
                match choice_0::parse(state.clone(), cache) {
                    Ok(ok_result) => return Ok(ok_result.map(|result| Parsed)),
                    Err(err) => farthest_error = combine_errors(farthest_error, Some(err)),
                }
                match choice_1::parse(state.clone(), cache) {
                    Ok(ok_result) => return Ok(ok_result.map(|result| Parsed)),
                    Err(err) => farthest_error = combine_errors(farthest_error, Some(err)),
                }
                Err(farthest_error
                    .unwrap_or_else(|| state.report_error(ParseErrorSpecifics::Other)))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed;
        }
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let mut state = state;
            let mut farthest_error: Option<ParseError> = None;
            loop {
                match closure::parse(state.clone(), cache) {
                    Ok(ParseOk {
                        result,
                        state: new_state,
                        farthest_error: new_farthest_error,
                    }) => {
                        state = new_state;
                        farthest_error = combine_errors(farthest_error, new_farthest_error);
                    }
                    Err(err) => {
                        farthest_error = combine_errors(farthest_error, Some(err));
                        break;
                    }
                }
            }
            Ok(ParseOk {
                result: Parsed {},
                state,
                farthest_error,
            })
        }
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Parsed;
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, String> {
            let ok_result = parse(state.clone(), cache)?;
            let new_state = ok_result.state.clone();
            Ok(ok_result.map(|_| state.slice_until(&new_state).to_string()))
        }
    }
    #[inline]
    pub(super) fn parse_StringLiteralBody<'a>(
        state: ParseState<'a>,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, StringLiteralBody> {
        let cache_key = state.cache_key();
        if let Some(cached) = cache.c_StringLiteralBody.get(&cache_key) {
            cached.clone()
        } else {
            let result = run_rule_parser(
                StringLiteralBody_impl::rule_parser,
                "StringLiteralBody",
                state,
                cache,
            );
            cache.c_StringLiteralBody.insert(cache_key, result.clone());
            result
        }
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
                    pub fn parse<'a>(
                        state: ParseState<'a>,
                        cache: &mut ParseCache<'a>,
                    ) -> ParseResult<'a, Parsed> {
                        let state = state.skip_whitespace();
                        let ok_result = parse_Identifier(state, cache)?;
                        Ok(ok_result.map(|result| Parsed { name: Some(result) }))
                    }
                    #[derive(Debug, Clone, PartialEq, Eq)]
                    pub struct Parsed {
                        pub name: Option<Identifier>,
                    }
                }
                mod part_1 {
                    use super::*;
                    #[inline(always)]
                    pub fn parse<'a>(
                        state: ParseState<'a>,
                        cache: &mut ParseCache<'a>,
                    ) -> ParseResult<'a, Parsed> {
                        let state = state.skip_whitespace();
                        let ok_result = parse_string_literal(state, ":")?;
                        Ok(ok_result.map(|_| Parsed))
                    }
                    #[derive(Debug, Clone, PartialEq, Eq)]
                    pub struct Parsed;
                }
                mod part_2 {
                    use super::*;
                    mod optional {
                        use super::*;
                        #[inline(always)]
                        pub fn parse<'a>(
                            state: ParseState<'a>,
                            cache: &mut ParseCache<'a>,
                        ) -> ParseResult<'a, Parsed> {
                            let state = state.skip_whitespace();
                            let ok_result = parse_BoxMarker(state, cache)?;
                            Ok(ok_result.map(|result| Parsed {
                                boxed: Some(result),
                            }))
                        }
                        #[derive(Debug, Clone, PartialEq, Eq)]
                        pub struct Parsed {
                            pub boxed: Option<BoxMarker>,
                        }
                    }
                    #[inline(always)]
                    pub fn parse<'a>(
                        state: ParseState<'a>,
                        cache: &mut ParseCache<'a>,
                    ) -> ParseResult<'a, Parsed> {
                        match optional::parse(state.clone(), cache) {
                            Ok(ok_result) => Ok(ok_result.map(|result| Parsed {
                                boxed: result.boxed,
                            })),
                            Err(err) => Ok(ParseOk {
                                result: Parsed {
                                    boxed: Default::default(),
                                },
                                state,
                                farthest_error: Some(err),
                            }),
                        }
                    }
                    #[derive(Debug, Clone, PartialEq, Eq)]
                    pub struct Parsed {
                        pub boxed: Option<BoxMarker>,
                    }
                }
                #[inline(always)]
                pub fn parse<'a>(
                    state: ParseState<'a>,
                    cache: &mut ParseCache<'a>,
                ) -> ParseResult<'a, Parsed> {
                    let mut state = state;
                    let mut farthest_error: Option<ParseError> = None;
                    let result = match part_0::parse(state, cache) {
                        Ok(ParseOk {
                            result,
                            state: new_state,
                            farthest_error: new_farthest_error,
                        }) => {
                            farthest_error = combine_errors(farthest_error, new_farthest_error);
                            state = new_state;
                            result
                        }
                        Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
                    };
                    let mut name = result.name;
                    match part_1::parse(state, cache) {
                        Ok(ParseOk {
                            result: _,
                            state: new_state,
                            farthest_error: new_farthest_error,
                        }) => {
                            farthest_error = combine_errors(farthest_error, new_farthest_error);
                            state = new_state;
                        }
                        Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
                    }
                    let result = match part_2::parse(state, cache) {
                        Ok(ParseOk {
                            result,
                            state: new_state,
                            farthest_error: new_farthest_error,
                        }) => {
                            farthest_error = combine_errors(farthest_error, new_farthest_error);
                            state = new_state;
                            result
                        }
                        Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
                    };
                    let mut boxed = result.boxed;
                    Ok(ParseOk {
                        result: Parsed { name, boxed },
                        state,
                        farthest_error,
                    })
                }
                #[derive(Debug, Clone, PartialEq, Eq)]
                pub struct Parsed {
                    pub name: Option<Identifier>,
                    pub boxed: Option<BoxMarker>,
                }
            }
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                match optional::parse(state.clone(), cache) {
                    Ok(ok_result) => Ok(ok_result.map(|result| Parsed {
                        name: result.name,
                        boxed: result.boxed,
                    })),
                    Err(err) => Ok(ParseOk {
                        result: Parsed {
                            name: Default::default(),
                            boxed: Default::default(),
                        },
                        state,
                        farthest_error: Some(err),
                    }),
                }
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub name: Option<Identifier>,
                pub boxed: Option<BoxMarker>,
            }
        }
        mod part_1 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_Identifier(state, cache)?;
                Ok(ok_result.map(|result| Parsed { typ: result }))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub typ: Identifier,
            }
        }
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let mut state = state;
            let mut farthest_error: Option<ParseError> = None;
            let result = match part_0::parse(state, cache) {
                Ok(ParseOk {
                    result,
                    state: new_state,
                    farthest_error: new_farthest_error,
                }) => {
                    farthest_error = combine_errors(farthest_error, new_farthest_error);
                    state = new_state;
                    result
                }
                Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
            };
            let mut name = result.name;
            let mut boxed = result.boxed;
            let result = match part_1::parse(state, cache) {
                Ok(ParseOk {
                    result,
                    state: new_state,
                    farthest_error: new_farthest_error,
                }) => {
                    farthest_error = combine_errors(farthest_error, new_farthest_error);
                    state = new_state;
                    result
                }
                Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
            };
            let mut typ = result.typ;
            Ok(ParseOk {
                result: Parsed { name, boxed, typ },
                state,
                farthest_error,
            })
        }
        use super::Field as Parsed;
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            parse(state, cache)
        }
    }
    #[inline]
    pub(super) fn parse_Field<'a>(
        state: ParseState<'a>,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, Field> {
        let cache_key = state.cache_key();
        if let Some(cached) = cache.c_Field.get(&cache_key) {
            cached.clone()
        } else {
            let result = run_rule_parser(Field_impl::rule_parser, "Field", state, cache);
            cache.c_Field.insert(cache_key, result.clone());
            result
        }
    }
    mod BoxMarker_impl {
        use super::*;
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let state = state.skip_whitespace();
            let ok_result = parse_character_literal(state, '*')?;
            Ok(ok_result.map(|_| Parsed))
        }
        use super::BoxMarker as Parsed;
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            parse(state, cache)
        }
    }
    #[inline]
    pub(super) fn parse_BoxMarker<'a>(
        state: ParseState<'a>,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, BoxMarker> {
        let cache_key = state.cache_key();
        if let Some(cached) = cache.c_BoxMarker.get(&cache_key) {
            cached.clone()
        } else {
            let result = run_rule_parser(BoxMarker_impl::rule_parser, "BoxMarker", state, cache);
            cache.c_BoxMarker.insert(cache_key, result.clone());
            result
        }
    }
    mod OverrideField_impl {
        use super::*;
        mod part_0 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_string_literal(state, "@")?;
                Ok(ok_result.map(|_| Parsed))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed;
        }
        mod part_1 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_string_literal(state, ":")?;
                Ok(ok_result.map(|_| Parsed))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed;
        }
        mod part_2 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_Identifier(state, cache)?;
                Ok(ok_result.map(|result| Parsed { typ: result }))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub typ: Identifier,
            }
        }
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let mut state = state;
            let mut farthest_error: Option<ParseError> = None;
            match part_0::parse(state, cache) {
                Ok(ParseOk {
                    result: _,
                    state: new_state,
                    farthest_error: new_farthest_error,
                }) => {
                    farthest_error = combine_errors(farthest_error, new_farthest_error);
                    state = new_state;
                }
                Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
            }
            match part_1::parse(state, cache) {
                Ok(ParseOk {
                    result: _,
                    state: new_state,
                    farthest_error: new_farthest_error,
                }) => {
                    farthest_error = combine_errors(farthest_error, new_farthest_error);
                    state = new_state;
                }
                Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
            }
            let result = match part_2::parse(state, cache) {
                Ok(ParseOk {
                    result,
                    state: new_state,
                    farthest_error: new_farthest_error,
                }) => {
                    farthest_error = combine_errors(farthest_error, new_farthest_error);
                    state = new_state;
                    result
                }
                Err(err) => return Err(combine_errors(farthest_error, Some(err)).unwrap()),
            };
            let mut typ = result.typ;
            Ok(ParseOk {
                result: Parsed { typ },
                state,
                farthest_error,
            })
        }
        use super::OverrideField as Parsed;
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            parse(state, cache)
        }
    }
    #[inline]
    pub(super) fn parse_OverrideField<'a>(
        state: ParseState<'a>,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, OverrideField> {
        let cache_key = state.cache_key();
        if let Some(cached) = cache.c_OverrideField.get(&cache_key) {
            cached.clone()
        } else {
            let result = run_rule_parser(
                OverrideField_impl::rule_parser,
                "OverrideField",
                state,
                cache,
            );
            cache.c_OverrideField.insert(cache_key, result.clone());
            result
        }
    }
    mod DelimitedExpression_impl {
        use super::*;
        mod choice_0 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_Group(state, cache)?;
                Ok(ok_result.map(|result| Parsed {
                    _override: Parsed__override::Group(result),
                }))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub _override: Parsed__override,
            }
        }
        mod choice_1 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_Optional(state, cache)?;
                Ok(ok_result.map(|result| Parsed {
                    _override: Parsed__override::Optional(result),
                }))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub _override: Parsed__override,
            }
        }
        mod choice_2 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_Closure(state, cache)?;
                Ok(ok_result.map(|result| Parsed {
                    _override: Parsed__override::Closure(result),
                }))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub _override: Parsed__override,
            }
        }
        mod choice_3 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_NegativeLookahead(state, cache)?;
                Ok(ok_result.map(|result| Parsed {
                    _override: Parsed__override::NegativeLookahead(result),
                }))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub _override: Parsed__override,
            }
        }
        mod choice_4 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_CharacterRange(state, cache)?;
                Ok(ok_result.map(|result| Parsed {
                    _override: Parsed__override::CharacterRange(result),
                }))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub _override: Parsed__override,
            }
        }
        mod choice_5 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_CharacterLiteral(state, cache)?;
                Ok(ok_result.map(|result| Parsed {
                    _override: Parsed__override::CharacterLiteral(result),
                }))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub _override: Parsed__override,
            }
        }
        mod choice_6 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_StringLiteral(state, cache)?;
                Ok(ok_result.map(|result| Parsed {
                    _override: Parsed__override::StringLiteral(result),
                }))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub _override: Parsed__override,
            }
        }
        mod choice_7 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_EndOfInput(state, cache)?;
                Ok(ok_result.map(|result| Parsed {
                    _override: Parsed__override::EndOfInput(result),
                }))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub _override: Parsed__override,
            }
        }
        mod choice_8 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_OverrideField(state, cache)?;
                Ok(ok_result.map(|result| Parsed {
                    _override: Parsed__override::OverrideField(result),
                }))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub _override: Parsed__override,
            }
        }
        mod choice_9 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_Field(state, cache)?;
                Ok(ok_result.map(|result| Parsed {
                    _override: Parsed__override::Field(result),
                }))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub _override: Parsed__override,
            }
        }
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let mut state = state;
            let mut farthest_error: Option<ParseError> = None;
            match choice_0::parse(state.clone(), cache) {
                Ok(ok_result) => {
                    return Ok(ok_result.map(|result| Parsed {
                        _override: result._override,
                    }))
                }
                Err(err) => farthest_error = combine_errors(farthest_error, Some(err)),
            }
            match choice_1::parse(state.clone(), cache) {
                Ok(ok_result) => {
                    return Ok(ok_result.map(|result| Parsed {
                        _override: result._override,
                    }))
                }
                Err(err) => farthest_error = combine_errors(farthest_error, Some(err)),
            }
            match choice_2::parse(state.clone(), cache) {
                Ok(ok_result) => {
                    return Ok(ok_result.map(|result| Parsed {
                        _override: result._override,
                    }))
                }
                Err(err) => farthest_error = combine_errors(farthest_error, Some(err)),
            }
            match choice_3::parse(state.clone(), cache) {
                Ok(ok_result) => {
                    return Ok(ok_result.map(|result| Parsed {
                        _override: result._override,
                    }))
                }
                Err(err) => farthest_error = combine_errors(farthest_error, Some(err)),
            }
            match choice_4::parse(state.clone(), cache) {
                Ok(ok_result) => {
                    return Ok(ok_result.map(|result| Parsed {
                        _override: result._override,
                    }))
                }
                Err(err) => farthest_error = combine_errors(farthest_error, Some(err)),
            }
            match choice_5::parse(state.clone(), cache) {
                Ok(ok_result) => {
                    return Ok(ok_result.map(|result| Parsed {
                        _override: result._override,
                    }))
                }
                Err(err) => farthest_error = combine_errors(farthest_error, Some(err)),
            }
            match choice_6::parse(state.clone(), cache) {
                Ok(ok_result) => {
                    return Ok(ok_result.map(|result| Parsed {
                        _override: result._override,
                    }))
                }
                Err(err) => farthest_error = combine_errors(farthest_error, Some(err)),
            }
            match choice_7::parse(state.clone(), cache) {
                Ok(ok_result) => {
                    return Ok(ok_result.map(|result| Parsed {
                        _override: result._override,
                    }))
                }
                Err(err) => farthest_error = combine_errors(farthest_error, Some(err)),
            }
            match choice_8::parse(state.clone(), cache) {
                Ok(ok_result) => {
                    return Ok(ok_result.map(|result| Parsed {
                        _override: result._override,
                    }))
                }
                Err(err) => farthest_error = combine_errors(farthest_error, Some(err)),
            }
            match choice_9::parse(state.clone(), cache) {
                Ok(ok_result) => {
                    return Ok(ok_result.map(|result| Parsed {
                        _override: result._override,
                    }))
                }
                Err(err) => farthest_error = combine_errors(farthest_error, Some(err)),
            }
            Err(farthest_error.unwrap_or_else(|| state.report_error(ParseErrorSpecifics::Other)))
        }
        pub struct Parsed {
            _override: super::DelimitedExpression,
        }
        use super::DelimitedExpression as Parsed__override;
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::DelimitedExpression> {
            let ok_result = parse(state, cache)?;
            Ok(ok_result.map(|result| result._override))
        }
    }
    #[inline]
    pub(super) fn parse_DelimitedExpression<'a>(
        state: ParseState<'a>,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, DelimitedExpression> {
        let cache_key = state.cache_key();
        if let Some(cached) = cache.c_DelimitedExpression.get(&cache_key) {
            cached.clone()
        } else {
            let result = run_rule_parser(
                DelimitedExpression_impl::rule_parser,
                "DelimitedExpression",
                state,
                cache,
            );
            cache
                .c_DelimitedExpression
                .insert(cache_key, result.clone());
            result
        }
    }
    mod Identifier_impl {
        use super::*;
        mod closure {
            use super::*;
            mod choice_0 {
                use super::*;
                #[inline(always)]
                pub fn parse<'a>(
                    state: ParseState<'a>,
                    cache: &mut ParseCache<'a>,
                ) -> ParseResult<'a, Parsed> {
                    let ok_result = parse_character_range(state, 'a', 'z')?;
                    Ok(ok_result.map(|_| Parsed))
                }
                #[derive(Debug, Clone, PartialEq, Eq)]
                pub struct Parsed;
            }
            mod choice_1 {
                use super::*;
                #[inline(always)]
                pub fn parse<'a>(
                    state: ParseState<'a>,
                    cache: &mut ParseCache<'a>,
                ) -> ParseResult<'a, Parsed> {
                    let ok_result = parse_character_range(state, 'A', 'Z')?;
                    Ok(ok_result.map(|_| Parsed))
                }
                #[derive(Debug, Clone, PartialEq, Eq)]
                pub struct Parsed;
            }
            mod choice_2 {
                use super::*;
                #[inline(always)]
                pub fn parse<'a>(
                    state: ParseState<'a>,
                    cache: &mut ParseCache<'a>,
                ) -> ParseResult<'a, Parsed> {
                    let ok_result = parse_character_range(state, '0', '9')?;
                    Ok(ok_result.map(|_| Parsed))
                }
                #[derive(Debug, Clone, PartialEq, Eq)]
                pub struct Parsed;
            }
            mod choice_3 {
                use super::*;
                #[inline(always)]
                pub fn parse<'a>(
                    state: ParseState<'a>,
                    cache: &mut ParseCache<'a>,
                ) -> ParseResult<'a, Parsed> {
                    let ok_result = parse_character_literal(state, '_')?;
                    Ok(ok_result.map(|_| Parsed))
                }
                #[derive(Debug, Clone, PartialEq, Eq)]
                pub struct Parsed;
            }
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let mut state = state;
                let mut farthest_error: Option<ParseError> = None;
                match choice_0::parse(state.clone(), cache) {
                    Ok(ok_result) => return Ok(ok_result.map(|result| Parsed)),
                    Err(err) => farthest_error = combine_errors(farthest_error, Some(err)),
                }
                match choice_1::parse(state.clone(), cache) {
                    Ok(ok_result) => return Ok(ok_result.map(|result| Parsed)),
                    Err(err) => farthest_error = combine_errors(farthest_error, Some(err)),
                }
                match choice_2::parse(state.clone(), cache) {
                    Ok(ok_result) => return Ok(ok_result.map(|result| Parsed)),
                    Err(err) => farthest_error = combine_errors(farthest_error, Some(err)),
                }
                match choice_3::parse(state.clone(), cache) {
                    Ok(ok_result) => return Ok(ok_result.map(|result| Parsed)),
                    Err(err) => farthest_error = combine_errors(farthest_error, Some(err)),
                }
                Err(farthest_error
                    .unwrap_or_else(|| state.report_error(ParseErrorSpecifics::Other)))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed;
        }
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let ParseOk {
                result,
                mut state,
                mut farthest_error,
            } = closure::parse(state, cache)?;
            loop {
                match closure::parse(state.clone(), cache) {
                    Ok(ParseOk {
                        result,
                        state: new_state,
                        farthest_error: new_farthest_error,
                    }) => {
                        state = new_state;
                        farthest_error = combine_errors(farthest_error, new_farthest_error);
                    }
                    Err(err) => {
                        farthest_error = combine_errors(farthest_error, Some(err));
                        break;
                    }
                }
            }
            Ok(ParseOk {
                result: Parsed {},
                state,
                farthest_error,
            })
        }
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Parsed;
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, String> {
            let ok_result = parse(state.clone(), cache)?;
            let new_state = ok_result.state.clone();
            Ok(ok_result.map(|_| state.slice_until(&new_state).to_string()))
        }
    }
    #[inline]
    pub(super) fn parse_Identifier<'a>(
        state: ParseState<'a>,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, Identifier> {
        let cache_key = state.cache_key();
        if let Some(cached) = cache.c_Identifier.get(&cache_key) {
            cached.clone()
        } else {
            let result = run_rule_parser(Identifier_impl::rule_parser, "Identifier", state, cache);
            cache.c_Identifier.insert(cache_key, result.clone());
            result
        }
    }
    mod DirectiveExpression_impl {
        use super::*;
        mod choice_0 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_StringDirective(state, cache)?;
                Ok(ok_result.map(|result| Parsed {
                    _override: Parsed__override::StringDirective(result),
                }))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub _override: Parsed__override,
            }
        }
        mod choice_1 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_NoSkipWsDirective(state, cache)?;
                Ok(ok_result.map(|result| Parsed {
                    _override: Parsed__override::NoSkipWsDirective(result),
                }))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub _override: Parsed__override,
            }
        }
        mod choice_2 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_ExportDirective(state, cache)?;
                Ok(ok_result.map(|result| Parsed {
                    _override: Parsed__override::ExportDirective(result),
                }))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub _override: Parsed__override,
            }
        }
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let mut state = state;
            let mut farthest_error: Option<ParseError> = None;
            match choice_0::parse(state.clone(), cache) {
                Ok(ok_result) => {
                    return Ok(ok_result.map(|result| Parsed {
                        _override: result._override,
                    }))
                }
                Err(err) => farthest_error = combine_errors(farthest_error, Some(err)),
            }
            match choice_1::parse(state.clone(), cache) {
                Ok(ok_result) => {
                    return Ok(ok_result.map(|result| Parsed {
                        _override: result._override,
                    }))
                }
                Err(err) => farthest_error = combine_errors(farthest_error, Some(err)),
            }
            match choice_2::parse(state.clone(), cache) {
                Ok(ok_result) => {
                    return Ok(ok_result.map(|result| Parsed {
                        _override: result._override,
                    }))
                }
                Err(err) => farthest_error = combine_errors(farthest_error, Some(err)),
            }
            Err(farthest_error.unwrap_or_else(|| state.report_error(ParseErrorSpecifics::Other)))
        }
        pub struct Parsed {
            _override: super::DirectiveExpression,
        }
        use super::DirectiveExpression as Parsed__override;
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::DirectiveExpression> {
            let ok_result = parse(state, cache)?;
            Ok(ok_result.map(|result| result._override))
        }
    }
    #[inline]
    pub(super) fn parse_DirectiveExpression<'a>(
        state: ParseState<'a>,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, DirectiveExpression> {
        let cache_key = state.cache_key();
        if let Some(cached) = cache.c_DirectiveExpression.get(&cache_key) {
            cached.clone()
        } else {
            let result = run_rule_parser(
                DirectiveExpression_impl::rule_parser,
                "DirectiveExpression",
                state,
                cache,
            );
            cache
                .c_DirectiveExpression
                .insert(cache_key, result.clone());
            result
        }
    }
    mod StringDirective_impl {
        use super::*;
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let state = state.skip_whitespace();
            let ok_result = parse_string_literal(state, "@string")?;
            Ok(ok_result.map(|_| Parsed))
        }
        use super::StringDirective as Parsed;
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            parse(state, cache)
        }
    }
    #[inline]
    pub(super) fn parse_StringDirective<'a>(
        state: ParseState<'a>,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, StringDirective> {
        let cache_key = state.cache_key();
        if let Some(cached) = cache.c_StringDirective.get(&cache_key) {
            cached.clone()
        } else {
            let result = run_rule_parser(
                StringDirective_impl::rule_parser,
                "StringDirective",
                state,
                cache,
            );
            cache.c_StringDirective.insert(cache_key, result.clone());
            result
        }
    }
    mod NoSkipWsDirective_impl {
        use super::*;
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let state = state.skip_whitespace();
            let ok_result = parse_string_literal(state, "@no_skip_ws")?;
            Ok(ok_result.map(|_| Parsed))
        }
        use super::NoSkipWsDirective as Parsed;
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            parse(state, cache)
        }
    }
    #[inline]
    pub(super) fn parse_NoSkipWsDirective<'a>(
        state: ParseState<'a>,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, NoSkipWsDirective> {
        let cache_key = state.cache_key();
        if let Some(cached) = cache.c_NoSkipWsDirective.get(&cache_key) {
            cached.clone()
        } else {
            let result = run_rule_parser(
                NoSkipWsDirective_impl::rule_parser,
                "NoSkipWsDirective",
                state,
                cache,
            );
            cache.c_NoSkipWsDirective.insert(cache_key, result.clone());
            result
        }
    }
    mod ExportDirective_impl {
        use super::*;
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let state = state.skip_whitespace();
            let ok_result = parse_string_literal(state, "@export")?;
            Ok(ok_result.map(|_| Parsed))
        }
        use super::ExportDirective as Parsed;
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            parse(state, cache)
        }
    }
    #[inline]
    pub(super) fn parse_ExportDirective<'a>(
        state: ParseState<'a>,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, ExportDirective> {
        let cache_key = state.cache_key();
        if let Some(cached) = cache.c_ExportDirective.get(&cache_key) {
            cached.clone()
        } else {
            let result = run_rule_parser(
                ExportDirective_impl::rule_parser,
                "ExportDirective",
                state,
                cache,
            );
            cache.c_ExportDirective.insert(cache_key, result.clone());
            result
        }
    }
    mod EndOfInput_impl {
        use super::*;
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let state = state.skip_whitespace();
            let ok_result = parse_character_literal(state, '$')?;
            Ok(ok_result.map(|_| Parsed))
        }
        use super::EndOfInput as Parsed;
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            parse(state, cache)
        }
    }
    #[inline]
    pub(super) fn parse_EndOfInput<'a>(
        state: ParseState<'a>,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, EndOfInput> {
        let cache_key = state.cache_key();
        if let Some(cached) = cache.c_EndOfInput.get(&cache_key) {
            cached.clone()
        } else {
            let result = run_rule_parser(EndOfInput_impl::rule_parser, "EndOfInput", state, cache);
            cache.c_EndOfInput.insert(cache_key, result.clone());
            result
        }
    }
}
