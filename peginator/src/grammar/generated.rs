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
    pub from: CharRangePart,
    pub to: CharRangePart,
}
pub use StringItem as CharRangePart;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringLiteral {
    pub body: Vec<StringItem>,
}
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
pub enum StringItem__override {
    HexaEscape(HexaEscape),
    SimpleEscape(SimpleEscape),
    Utf8Escape(Utf8Escape),
    char(char),
}
pub use StringItem__override as StringItem;
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SimpleEscape__override {
    SimpleEscapeBackslash(SimpleEscapeBackslash),
    SimpleEscapeCarriageReturn(SimpleEscapeCarriageReturn),
    SimpleEscapeDQuote(SimpleEscapeDQuote),
    SimpleEscapeNewline(SimpleEscapeNewline),
    SimpleEscapeQuote(SimpleEscapeQuote),
    SimpleEscapeTab(SimpleEscapeTab),
}
pub use SimpleEscape__override as SimpleEscape;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimpleEscapeNewline;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimpleEscapeCarriageReturn;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimpleEscapeTab;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimpleEscapeBackslash;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimpleEscapeQuote;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimpleEscapeDQuote;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HexaEscape {
    pub c1: HexChar,
    pub c2: HexChar,
}
pub type HexChar = String;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Utf8Escape {
    pub c1: HexChar,
    pub c2: Option<HexChar>,
    pub c3: Option<HexChar>,
    pub c4: Option<HexChar>,
    pub c5: Option<HexChar>,
    pub c6: Option<HexChar>,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DirectiveExpression__override {
    ExportDirective(ExportDirective),
    NoSkipWsDirective(NoSkipWsDirective),
    PositionDirective(PositionDirective),
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
pub struct PositionDirective;
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
        pub c_CharRangePart: CacheEntries<'a, CharRangePart>,
        pub c_StringLiteral: CacheEntries<'a, StringLiteral>,
        pub c_Field: CacheEntries<'a, Field>,
        pub c_BoxMarker: CacheEntries<'a, BoxMarker>,
        pub c_OverrideField: CacheEntries<'a, OverrideField>,
        pub c_DelimitedExpression: CacheEntries<'a, DelimitedExpression>,
        pub c_Identifier: CacheEntries<'a, Identifier>,
        pub c_StringItem: CacheEntries<'a, StringItem>,
        pub c_SimpleEscape: CacheEntries<'a, SimpleEscape>,
        pub c_SimpleEscapeNewline: CacheEntries<'a, SimpleEscapeNewline>,
        pub c_SimpleEscapeCarriageReturn: CacheEntries<'a, SimpleEscapeCarriageReturn>,
        pub c_SimpleEscapeTab: CacheEntries<'a, SimpleEscapeTab>,
        pub c_SimpleEscapeBackslash: CacheEntries<'a, SimpleEscapeBackslash>,
        pub c_SimpleEscapeQuote: CacheEntries<'a, SimpleEscapeQuote>,
        pub c_SimpleEscapeDQuote: CacheEntries<'a, SimpleEscapeDQuote>,
        pub c_HexaEscape: CacheEntries<'a, HexaEscape>,
        pub c_HexChar: CacheEntries<'a, HexChar>,
        pub c_Utf8Escape: CacheEntries<'a, Utf8Escape>,
        pub c_DirectiveExpression: CacheEntries<'a, DirectiveExpression>,
        pub c_StringDirective: CacheEntries<'a, StringDirective>,
        pub c_NoSkipWsDirective: CacheEntries<'a, NoSkipWsDirective>,
        pub c_ExportDirective: CacheEntries<'a, ExportDirective>,
        pub c_PositionDirective: CacheEntries<'a, PositionDirective>,
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
                        let ok_result = parse_character_literal(state, ';')?;
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
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Parsed {
            pub rules: Vec<Rule>,
        }
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::Grammar> {
            let ok_result = parse(state, cache)?;
            Ok(ok_result.map(|r| super::Grammar { rules: r.rules }))
        }
    }
    #[inline]
    pub(super) fn parse_Grammar<'a>(
        state: ParseState<'a>,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, Grammar> {
        let cache_key = state.cache_key();
        if let Some(cached) = cache.c_Grammar.get(&cache_key) {
            state.print_trace_cached("Grammar");
            state.print_trace_result(&cached);
            cached.clone()
        } else {
            state.print_trace_start("Grammar");
            let result = Grammar_impl::rule_parser(state.clone().indent(), cache);
            state.print_trace_result(&result);
            let result = result.map(|result| ParseOk {
                state: result.state.dedent(),
                ..result
            });
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
                let ok_result = parse_character_literal(state, '=')?;
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
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Parsed {
            pub directives: Vec<DirectiveExpression>,
            pub name: Identifier,
            pub definition: Choice,
        }
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::Rule> {
            let ok_result = parse(state, cache)?;
            Ok(ok_result.map(|r| super::Rule {
                directives: r.directives,
                name: r.name,
                definition: r.definition,
            }))
        }
    }
    #[inline]
    pub(super) fn parse_Rule<'a>(
        state: ParseState<'a>,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, Rule> {
        let cache_key = state.cache_key();
        if let Some(cached) = cache.c_Rule.get(&cache_key) {
            state.print_trace_cached("Rule");
            state.print_trace_result(&cached);
            cached.clone()
        } else {
            state.print_trace_start("Rule");
            let result = Rule_impl::rule_parser(state.clone().indent(), cache);
            state.print_trace_result(&result);
            let result = result.map(|result| ParseOk {
                state: result.state.dedent(),
                ..result
            });
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
                        let ok_result = parse_character_literal(state, '|')?;
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
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Parsed {
            pub choices: Vec<Sequence>,
        }
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::Choice> {
            let ok_result = parse(state, cache)?;
            Ok(ok_result.map(|r| super::Choice { choices: r.choices }))
        }
    }
    #[inline]
    pub(super) fn parse_Choice<'a>(
        state: ParseState<'a>,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, Choice> {
        let cache_key = state.cache_key();
        if let Some(cached) = cache.c_Choice.get(&cache_key) {
            state.print_trace_cached("Choice");
            state.print_trace_result(&cached);
            cached.clone()
        } else {
            state.print_trace_start("Choice");
            let result = Choice_impl::rule_parser(state.clone().indent(), cache);
            state.print_trace_result(&result);
            let result = result.map(|result| ParseOk {
                state: result.state.dedent(),
                ..result
            });
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
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Parsed {
            pub parts: Vec<DelimitedExpression>,
        }
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::Sequence> {
            let ok_result = parse(state, cache)?;
            Ok(ok_result.map(|r| super::Sequence { parts: r.parts }))
        }
    }
    #[inline]
    pub(super) fn parse_Sequence<'a>(
        state: ParseState<'a>,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, Sequence> {
        let cache_key = state.cache_key();
        if let Some(cached) = cache.c_Sequence.get(&cache_key) {
            state.print_trace_cached("Sequence");
            state.print_trace_result(&cached);
            cached.clone()
        } else {
            state.print_trace_start("Sequence");
            let result = Sequence_impl::rule_parser(state.clone().indent(), cache);
            state.print_trace_result(&result);
            let result = result.map(|result| ParseOk {
                state: result.state.dedent(),
                ..result
            });
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
                let ok_result = parse_character_literal(state, '(')?;
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
                let ok_result = parse_character_literal(state, ')')?;
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
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Parsed {
            pub body: Choice,
        }
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::Group> {
            let ok_result = parse(state, cache)?;
            Ok(ok_result.map(|r| super::Group { body: r.body }))
        }
    }
    #[inline]
    pub(super) fn parse_Group<'a>(
        state: ParseState<'a>,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, Group> {
        let cache_key = state.cache_key();
        if let Some(cached) = cache.c_Group.get(&cache_key) {
            state.print_trace_cached("Group");
            state.print_trace_result(&cached);
            cached.clone()
        } else {
            state.print_trace_start("Group");
            let result = Group_impl::rule_parser(state.clone().indent(), cache);
            state.print_trace_result(&result);
            let result = result.map(|result| ParseOk {
                state: result.state.dedent(),
                ..result
            });
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
                let ok_result = parse_character_literal(state, '[')?;
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
                let ok_result = parse_character_literal(state, ']')?;
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
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Parsed {
            pub body: Choice,
        }
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::Optional> {
            let ok_result = parse(state, cache)?;
            Ok(ok_result.map(|r| super::Optional { body: r.body }))
        }
    }
    #[inline]
    pub(super) fn parse_Optional<'a>(
        state: ParseState<'a>,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, Optional> {
        let cache_key = state.cache_key();
        if let Some(cached) = cache.c_Optional.get(&cache_key) {
            state.print_trace_cached("Optional");
            state.print_trace_result(&cached);
            cached.clone()
        } else {
            state.print_trace_start("Optional");
            let result = Optional_impl::rule_parser(state.clone().indent(), cache);
            state.print_trace_result(&result);
            let result = result.map(|result| ParseOk {
                state: result.state.dedent(),
                ..result
            });
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
                let ok_result = parse_character_literal(state, '{')?;
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
                let ok_result = parse_character_literal(state, '}')?;
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
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Parsed {
            pub body: Choice,
            pub at_least_one: Option<AtLeastOneMarker>,
        }
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::Closure> {
            let ok_result = parse(state, cache)?;
            Ok(ok_result.map(|r| super::Closure {
                body: r.body,
                at_least_one: r.at_least_one,
            }))
        }
    }
    #[inline]
    pub(super) fn parse_Closure<'a>(
        state: ParseState<'a>,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, Closure> {
        let cache_key = state.cache_key();
        if let Some(cached) = cache.c_Closure.get(&cache_key) {
            state.print_trace_cached("Closure");
            state.print_trace_result(&cached);
            cached.clone()
        } else {
            state.print_trace_start("Closure");
            let result = Closure_impl::rule_parser(state.clone().indent(), cache);
            state.print_trace_result(&result);
            let result = result.map(|result| ParseOk {
                state: result.state.dedent(),
                ..result
            });
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
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Parsed;
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::AtLeastOneMarker> {
            let ok_result = parse(state, cache)?;
            Ok(ok_result.map(|r| super::AtLeastOneMarker {}))
        }
    }
    #[inline]
    pub(super) fn parse_AtLeastOneMarker<'a>(
        state: ParseState<'a>,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, AtLeastOneMarker> {
        let cache_key = state.cache_key();
        if let Some(cached) = cache.c_AtLeastOneMarker.get(&cache_key) {
            state.print_trace_cached("AtLeastOneMarker");
            state.print_trace_result(&cached);
            cached.clone()
        } else {
            state.print_trace_start("AtLeastOneMarker");
            let result = AtLeastOneMarker_impl::rule_parser(state.clone().indent(), cache);
            state.print_trace_result(&result);
            let result = result.map(|result| ParseOk {
                state: result.state.dedent(),
                ..result
            });
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
                let ok_result = parse_character_literal(state, '!')?;
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
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Parsed {
            pub expr: Box<DelimitedExpression>,
        }
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::NegativeLookahead> {
            let ok_result = parse(state, cache)?;
            Ok(ok_result.map(|r| super::NegativeLookahead { expr: r.expr }))
        }
    }
    #[inline]
    pub(super) fn parse_NegativeLookahead<'a>(
        state: ParseState<'a>,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, NegativeLookahead> {
        let cache_key = state.cache_key();
        if let Some(cached) = cache.c_NegativeLookahead.get(&cache_key) {
            state.print_trace_cached("NegativeLookahead");
            state.print_trace_result(&cached);
            cached.clone()
        } else {
            state.print_trace_start("NegativeLookahead");
            let result = NegativeLookahead_impl::rule_parser(state.clone().indent(), cache);
            state.print_trace_result(&result);
            let result = result.map(|result| ParseOk {
                state: result.state.dedent(),
                ..result
            });
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
                let ok_result = parse_CharRangePart(state, cache)?;
                Ok(ok_result.map(|result| Parsed { from: result }))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub from: CharRangePart,
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
                let ok_result = parse_CharRangePart(state, cache)?;
                Ok(ok_result.map(|result| Parsed { to: result }))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub to: CharRangePart,
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
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Parsed {
            pub from: CharRangePart,
            pub to: CharRangePart,
        }
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::CharacterRange> {
            let ok_result = parse(state, cache)?;
            Ok(ok_result.map(|r| super::CharacterRange {
                from: r.from,
                to: r.to,
            }))
        }
    }
    #[inline]
    pub(super) fn parse_CharacterRange<'a>(
        state: ParseState<'a>,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, CharacterRange> {
        let cache_key = state.cache_key();
        if let Some(cached) = cache.c_CharacterRange.get(&cache_key) {
            state.print_trace_cached("CharacterRange");
            state.print_trace_result(&cached);
            cached.clone()
        } else {
            state.print_trace_start("CharacterRange");
            let result = CharacterRange_impl::rule_parser(state.clone().indent(), cache);
            state.print_trace_result(&result);
            let result = result.map(|result| ParseOk {
                state: result.state.dedent(),
                ..result
            });
            cache.c_CharacterRange.insert(cache_key, result.clone());
            result
        }
    }
    mod CharRangePart_impl {
        use super::*;
        mod part_0 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let ok_result = parse_character_literal(state, '\'')?;
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
                let ok_result = parse_StringItem(state, cache)?;
                Ok(ok_result.map(|result| Parsed { _override: result }))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub _override: StringItem,
            }
        }
        mod part_2 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let ok_result = parse_character_literal(state, '\'')?;
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
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Parsed {
            pub _override: StringItem,
        }
        use super::CharRangePart as Parsed__override;
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::CharRangePart> {
            let ok_result = parse(state, cache)?;
            Ok(ok_result.map(|result| result._override))
        }
    }
    #[inline]
    pub(super) fn parse_CharRangePart<'a>(
        state: ParseState<'a>,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, CharRangePart> {
        let cache_key = state.cache_key();
        if let Some(cached) = cache.c_CharRangePart.get(&cache_key) {
            state.print_trace_cached("CharRangePart");
            state.print_trace_result(&cached);
            cached.clone()
        } else {
            state.print_trace_start("CharRangePart");
            let result = CharRangePart_impl::rule_parser(state.clone().indent(), cache);
            state.print_trace_result(&result);
            let result = result.map(|result| ParseOk {
                state: result.state.dedent(),
                ..result
            });
            cache.c_CharRangePart.insert(cache_key, result.clone());
            result
        }
    }
    mod StringLiteral_impl {
        use super::*;
        mod choice_0 {
            use super::*;
            mod part_0 {
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
            mod part_1 {
                use super::*;
                mod closure {
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
                                Ok(_) => Err(state
                                    .report_error(ParseErrorSpecifics::NegativeLookaheadFailed)),
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
                            let ok_result = parse_StringItem(state, cache)?;
                            Ok(ok_result.map(|result| Parsed { body: vec![result] }))
                        }
                        #[derive(Debug, Clone, PartialEq, Eq)]
                        pub struct Parsed {
                            pub body: Vec<StringItem>,
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
                            Err(err) => {
                                return Err(combine_errors(farthest_error, Some(err)).unwrap())
                            }
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
                            Err(err) => {
                                return Err(combine_errors(farthest_error, Some(err)).unwrap())
                            }
                        };
                        let mut body = result.body;
                        Ok(ParseOk {
                            result: Parsed { body },
                            state,
                            farthest_error,
                        })
                    }
                    #[derive(Debug, Clone, PartialEq, Eq)]
                    pub struct Parsed {
                        pub body: Vec<StringItem>,
                    }
                }
                #[inline(always)]
                pub fn parse<'a>(
                    state: ParseState<'a>,
                    cache: &mut ParseCache<'a>,
                ) -> ParseResult<'a, Parsed> {
                    let mut body: Vec<StringItem> = Vec::new();
                    let mut state = state;
                    let mut farthest_error: Option<ParseError> = None;
                    loop {
                        match closure::parse(state.clone(), cache) {
                            Ok(ParseOk {
                                result,
                                state: new_state,
                                farthest_error: new_farthest_error,
                            }) => {
                                body.extend(result.body);
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
                        result: Parsed { body },
                        state,
                        farthest_error,
                    })
                }
                #[derive(Debug, Clone, PartialEq, Eq)]
                pub struct Parsed {
                    pub body: Vec<StringItem>,
                }
            }
            mod part_2 {
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
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub body: Vec<StringItem>,
            }
        }
        mod choice_1 {
            use super::*;
            mod part_0 {
                use super::*;
                #[inline(always)]
                pub fn parse<'a>(
                    state: ParseState<'a>,
                    cache: &mut ParseCache<'a>,
                ) -> ParseResult<'a, Parsed> {
                    let ok_result = parse_character_literal(state, '\'')?;
                    Ok(ok_result.map(|_| Parsed))
                }
                #[derive(Debug, Clone, PartialEq, Eq)]
                pub struct Parsed;
            }
            mod part_1 {
                use super::*;
                mod closure {
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
                                let ok_result = parse_character_literal(state, '\'')?;
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
                                Ok(_) => Err(state
                                    .report_error(ParseErrorSpecifics::NegativeLookaheadFailed)),
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
                            let ok_result = parse_StringItem(state, cache)?;
                            Ok(ok_result.map(|result| Parsed { body: vec![result] }))
                        }
                        #[derive(Debug, Clone, PartialEq, Eq)]
                        pub struct Parsed {
                            pub body: Vec<StringItem>,
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
                            Err(err) => {
                                return Err(combine_errors(farthest_error, Some(err)).unwrap())
                            }
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
                            Err(err) => {
                                return Err(combine_errors(farthest_error, Some(err)).unwrap())
                            }
                        };
                        let mut body = result.body;
                        Ok(ParseOk {
                            result: Parsed { body },
                            state,
                            farthest_error,
                        })
                    }
                    #[derive(Debug, Clone, PartialEq, Eq)]
                    pub struct Parsed {
                        pub body: Vec<StringItem>,
                    }
                }
                #[inline(always)]
                pub fn parse<'a>(
                    state: ParseState<'a>,
                    cache: &mut ParseCache<'a>,
                ) -> ParseResult<'a, Parsed> {
                    let mut body: Vec<StringItem> = Vec::new();
                    let mut state = state;
                    let mut farthest_error: Option<ParseError> = None;
                    loop {
                        match closure::parse(state.clone(), cache) {
                            Ok(ParseOk {
                                result,
                                state: new_state,
                                farthest_error: new_farthest_error,
                            }) => {
                                body.extend(result.body);
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
                        result: Parsed { body },
                        state,
                        farthest_error,
                    })
                }
                #[derive(Debug, Clone, PartialEq, Eq)]
                pub struct Parsed {
                    pub body: Vec<StringItem>,
                }
            }
            mod part_2 {
                use super::*;
                #[inline(always)]
                pub fn parse<'a>(
                    state: ParseState<'a>,
                    cache: &mut ParseCache<'a>,
                ) -> ParseResult<'a, Parsed> {
                    let ok_result = parse_character_literal(state, '\'')?;
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
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub body: Vec<StringItem>,
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
                Ok(ok_result) => return Ok(ok_result.map(|result| Parsed { body: result.body })),
                Err(err) => farthest_error = combine_errors(farthest_error, Some(err)),
            }
            match choice_1::parse(state.clone(), cache) {
                Ok(ok_result) => return Ok(ok_result.map(|result| Parsed { body: result.body })),
                Err(err) => farthest_error = combine_errors(farthest_error, Some(err)),
            }
            Err(farthest_error.unwrap_or_else(|| state.report_error(ParseErrorSpecifics::Other)))
        }
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Parsed {
            pub body: Vec<StringItem>,
        }
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::StringLiteral> {
            let ok_result = parse(state, cache)?;
            Ok(ok_result.map(|r| super::StringLiteral { body: r.body }))
        }
    }
    #[inline]
    pub(super) fn parse_StringLiteral<'a>(
        state: ParseState<'a>,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, StringLiteral> {
        let cache_key = state.cache_key();
        if let Some(cached) = cache.c_StringLiteral.get(&cache_key) {
            state.print_trace_cached("StringLiteral");
            state.print_trace_result(&cached);
            cached.clone()
        } else {
            state.print_trace_start("StringLiteral");
            let result = StringLiteral_impl::rule_parser(state.clone().indent(), cache);
            state.print_trace_result(&result);
            let result = result.map(|result| ParseOk {
                state: result.state.dedent(),
                ..result
            });
            cache.c_StringLiteral.insert(cache_key, result.clone());
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
                        let ok_result = parse_character_literal(state, ':')?;
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
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Parsed {
            pub name: Option<Identifier>,
            pub boxed: Option<BoxMarker>,
            pub typ: Identifier,
        }
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::Field> {
            let ok_result = parse(state, cache)?;
            Ok(ok_result.map(|r| super::Field {
                name: r.name,
                boxed: r.boxed,
                typ: r.typ,
            }))
        }
    }
    #[inline]
    pub(super) fn parse_Field<'a>(
        state: ParseState<'a>,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, Field> {
        let cache_key = state.cache_key();
        if let Some(cached) = cache.c_Field.get(&cache_key) {
            state.print_trace_cached("Field");
            state.print_trace_result(&cached);
            cached.clone()
        } else {
            state.print_trace_start("Field");
            let result = Field_impl::rule_parser(state.clone().indent(), cache);
            state.print_trace_result(&result);
            let result = result.map(|result| ParseOk {
                state: result.state.dedent(),
                ..result
            });
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
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Parsed;
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::BoxMarker> {
            let ok_result = parse(state, cache)?;
            Ok(ok_result.map(|r| super::BoxMarker {}))
        }
    }
    #[inline]
    pub(super) fn parse_BoxMarker<'a>(
        state: ParseState<'a>,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, BoxMarker> {
        let cache_key = state.cache_key();
        if let Some(cached) = cache.c_BoxMarker.get(&cache_key) {
            state.print_trace_cached("BoxMarker");
            state.print_trace_result(&cached);
            cached.clone()
        } else {
            state.print_trace_start("BoxMarker");
            let result = BoxMarker_impl::rule_parser(state.clone().indent(), cache);
            state.print_trace_result(&result);
            let result = result.map(|result| ParseOk {
                state: result.state.dedent(),
                ..result
            });
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
                let ok_result = parse_character_literal(state, '@')?;
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
                let ok_result = parse_character_literal(state, ':')?;
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
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Parsed {
            pub typ: Identifier,
        }
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::OverrideField> {
            let ok_result = parse(state, cache)?;
            Ok(ok_result.map(|r| super::OverrideField { typ: r.typ }))
        }
    }
    #[inline]
    pub(super) fn parse_OverrideField<'a>(
        state: ParseState<'a>,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, OverrideField> {
        let cache_key = state.cache_key();
        if let Some(cached) = cache.c_OverrideField.get(&cache_key) {
            state.print_trace_cached("OverrideField");
            state.print_trace_result(&cached);
            cached.clone()
        } else {
            state.print_trace_start("OverrideField");
            let result = OverrideField_impl::rule_parser(state.clone().indent(), cache);
            state.print_trace_result(&result);
            let result = result.map(|result| ParseOk {
                state: result.state.dedent(),
                ..result
            });
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
        mod choice_6 {
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
        mod choice_7 {
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
        mod choice_8 {
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
            Err(farthest_error.unwrap_or_else(|| state.report_error(ParseErrorSpecifics::Other)))
        }
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Parsed {
            pub _override: Parsed__override,
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
            state.print_trace_cached("DelimitedExpression");
            state.print_trace_result(&cached);
            cached.clone()
        } else {
            state.print_trace_start("DelimitedExpression");
            let result = DelimitedExpression_impl::rule_parser(state.clone().indent(), cache);
            state.print_trace_result(&result);
            let result = result.map(|result| ParseOk {
                state: result.state.dedent(),
                ..result
            });
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
            Ok(ok_result.map_with_state(|_, new_state| state.slice_until(&new_state).to_string()))
        }
    }
    #[inline]
    pub(super) fn parse_Identifier<'a>(
        state: ParseState<'a>,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, Identifier> {
        let cache_key = state.cache_key();
        if let Some(cached) = cache.c_Identifier.get(&cache_key) {
            state.print_trace_cached("Identifier");
            state.print_trace_result(&cached);
            cached.clone()
        } else {
            state.print_trace_start("Identifier");
            let result = Identifier_impl::rule_parser(state.clone().indent(), cache);
            state.print_trace_result(&result);
            let result = result.map(|result| ParseOk {
                state: result.state.dedent(),
                ..result
            });
            cache.c_Identifier.insert(cache_key, result.clone());
            result
        }
    }
    mod StringItem_impl {
        use super::*;
        mod choice_0 {
            use super::*;
            mod part_0 {
                use super::*;
                #[inline(always)]
                pub fn parse<'a>(
                    state: ParseState<'a>,
                    cache: &mut ParseCache<'a>,
                ) -> ParseResult<'a, Parsed> {
                    let ok_result = parse_character_literal(state, '\\')?;
                    Ok(ok_result.map(|_| Parsed))
                }
                #[derive(Debug, Clone, PartialEq, Eq)]
                pub struct Parsed;
            }
            mod part_1 {
                use super::*;
                mod choice_0 {
                    use super::*;
                    #[inline(always)]
                    pub fn parse<'a>(
                        state: ParseState<'a>,
                        cache: &mut ParseCache<'a>,
                    ) -> ParseResult<'a, Parsed> {
                        let ok_result = parse_SimpleEscape(state, cache)?;
                        Ok(ok_result.map(|result| Parsed {
                            _override: Parsed__override::SimpleEscape(result),
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
                        let ok_result = parse_HexaEscape(state, cache)?;
                        Ok(ok_result.map(|result| Parsed {
                            _override: Parsed__override::HexaEscape(result),
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
                        let ok_result = parse_Utf8Escape(state, cache)?;
                        Ok(ok_result.map(|result| Parsed {
                            _override: Parsed__override::Utf8Escape(result),
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
                    Err(farthest_error
                        .unwrap_or_else(|| state.report_error(ParseErrorSpecifics::Other)))
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
                Ok(ParseOk {
                    result: Parsed { _override },
                    state,
                    farthest_error,
                })
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub _override: Parsed__override,
            }
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
                        let ok_result = parse_character_literal(state, '\\')?;
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
                            Err(state.report_error(ParseErrorSpecifics::NegativeLookaheadFailed))
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
                    Ok(ok_result.map(|result| Parsed {
                        _override: Parsed__override::char(result),
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
                Ok(ParseOk {
                    result: Parsed { _override },
                    state,
                    farthest_error,
                })
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
            Err(farthest_error.unwrap_or_else(|| state.report_error(ParseErrorSpecifics::Other)))
        }
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Parsed {
            pub _override: Parsed__override,
        }
        use super::StringItem as Parsed__override;
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::StringItem> {
            let ok_result = parse(state, cache)?;
            Ok(ok_result.map(|result| result._override))
        }
    }
    #[inline]
    pub(super) fn parse_StringItem<'a>(
        state: ParseState<'a>,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, StringItem> {
        let cache_key = state.cache_key();
        if let Some(cached) = cache.c_StringItem.get(&cache_key) {
            state.print_trace_cached("StringItem");
            state.print_trace_result(&cached);
            cached.clone()
        } else {
            state.print_trace_start("StringItem");
            let result = StringItem_impl::rule_parser(state.clone().indent(), cache);
            state.print_trace_result(&result);
            let result = result.map(|result| ParseOk {
                state: result.state.dedent(),
                ..result
            });
            cache.c_StringItem.insert(cache_key, result.clone());
            result
        }
    }
    mod SimpleEscape_impl {
        use super::*;
        mod choice_0 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let ok_result = parse_SimpleEscapeNewline(state, cache)?;
                Ok(ok_result.map(|result| Parsed {
                    _override: Parsed__override::SimpleEscapeNewline(result),
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
                let ok_result = parse_SimpleEscapeCarriageReturn(state, cache)?;
                Ok(ok_result.map(|result| Parsed {
                    _override: Parsed__override::SimpleEscapeCarriageReturn(result),
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
                let ok_result = parse_SimpleEscapeTab(state, cache)?;
                Ok(ok_result.map(|result| Parsed {
                    _override: Parsed__override::SimpleEscapeTab(result),
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
                let ok_result = parse_SimpleEscapeBackslash(state, cache)?;
                Ok(ok_result.map(|result| Parsed {
                    _override: Parsed__override::SimpleEscapeBackslash(result),
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
                let ok_result = parse_SimpleEscapeQuote(state, cache)?;
                Ok(ok_result.map(|result| Parsed {
                    _override: Parsed__override::SimpleEscapeQuote(result),
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
                let ok_result = parse_SimpleEscapeDQuote(state, cache)?;
                Ok(ok_result.map(|result| Parsed {
                    _override: Parsed__override::SimpleEscapeDQuote(result),
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
            Err(farthest_error.unwrap_or_else(|| state.report_error(ParseErrorSpecifics::Other)))
        }
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Parsed {
            pub _override: Parsed__override,
        }
        use super::SimpleEscape as Parsed__override;
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::SimpleEscape> {
            let ok_result = parse(state, cache)?;
            Ok(ok_result.map(|result| result._override))
        }
    }
    #[inline]
    pub(super) fn parse_SimpleEscape<'a>(
        state: ParseState<'a>,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, SimpleEscape> {
        let cache_key = state.cache_key();
        if let Some(cached) = cache.c_SimpleEscape.get(&cache_key) {
            state.print_trace_cached("SimpleEscape");
            state.print_trace_result(&cached);
            cached.clone()
        } else {
            state.print_trace_start("SimpleEscape");
            let result = SimpleEscape_impl::rule_parser(state.clone().indent(), cache);
            state.print_trace_result(&result);
            let result = result.map(|result| ParseOk {
                state: result.state.dedent(),
                ..result
            });
            cache.c_SimpleEscape.insert(cache_key, result.clone());
            result
        }
    }
    mod SimpleEscapeNewline_impl {
        use super::*;
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let ok_result = parse_character_literal(state, 'n')?;
            Ok(ok_result.map(|_| Parsed))
        }
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Parsed;
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::SimpleEscapeNewline> {
            let ok_result = parse(state, cache)?;
            Ok(ok_result.map(|r| super::SimpleEscapeNewline {}))
        }
    }
    #[inline]
    pub(super) fn parse_SimpleEscapeNewline<'a>(
        state: ParseState<'a>,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, SimpleEscapeNewline> {
        let cache_key = state.cache_key();
        if let Some(cached) = cache.c_SimpleEscapeNewline.get(&cache_key) {
            state.print_trace_cached("SimpleEscapeNewline");
            state.print_trace_result(&cached);
            cached.clone()
        } else {
            state.print_trace_start("SimpleEscapeNewline");
            let result = SimpleEscapeNewline_impl::rule_parser(state.clone().indent(), cache);
            state.print_trace_result(&result);
            let result = result.map(|result| ParseOk {
                state: result.state.dedent(),
                ..result
            });
            cache
                .c_SimpleEscapeNewline
                .insert(cache_key, result.clone());
            result
        }
    }
    mod SimpleEscapeCarriageReturn_impl {
        use super::*;
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let ok_result = parse_character_literal(state, 'r')?;
            Ok(ok_result.map(|_| Parsed))
        }
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Parsed;
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::SimpleEscapeCarriageReturn> {
            let ok_result = parse(state, cache)?;
            Ok(ok_result.map(|r| super::SimpleEscapeCarriageReturn {}))
        }
    }
    #[inline]
    pub(super) fn parse_SimpleEscapeCarriageReturn<'a>(
        state: ParseState<'a>,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, SimpleEscapeCarriageReturn> {
        let cache_key = state.cache_key();
        if let Some(cached) = cache.c_SimpleEscapeCarriageReturn.get(&cache_key) {
            state.print_trace_cached("SimpleEscapeCarriageReturn");
            state.print_trace_result(&cached);
            cached.clone()
        } else {
            state.print_trace_start("SimpleEscapeCarriageReturn");
            let result =
                SimpleEscapeCarriageReturn_impl::rule_parser(state.clone().indent(), cache);
            state.print_trace_result(&result);
            let result = result.map(|result| ParseOk {
                state: result.state.dedent(),
                ..result
            });
            cache
                .c_SimpleEscapeCarriageReturn
                .insert(cache_key, result.clone());
            result
        }
    }
    mod SimpleEscapeTab_impl {
        use super::*;
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let ok_result = parse_character_literal(state, 't')?;
            Ok(ok_result.map(|_| Parsed))
        }
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Parsed;
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::SimpleEscapeTab> {
            let ok_result = parse(state, cache)?;
            Ok(ok_result.map(|r| super::SimpleEscapeTab {}))
        }
    }
    #[inline]
    pub(super) fn parse_SimpleEscapeTab<'a>(
        state: ParseState<'a>,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, SimpleEscapeTab> {
        let cache_key = state.cache_key();
        if let Some(cached) = cache.c_SimpleEscapeTab.get(&cache_key) {
            state.print_trace_cached("SimpleEscapeTab");
            state.print_trace_result(&cached);
            cached.clone()
        } else {
            state.print_trace_start("SimpleEscapeTab");
            let result = SimpleEscapeTab_impl::rule_parser(state.clone().indent(), cache);
            state.print_trace_result(&result);
            let result = result.map(|result| ParseOk {
                state: result.state.dedent(),
                ..result
            });
            cache.c_SimpleEscapeTab.insert(cache_key, result.clone());
            result
        }
    }
    mod SimpleEscapeBackslash_impl {
        use super::*;
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let ok_result = parse_character_literal(state, '\\')?;
            Ok(ok_result.map(|_| Parsed))
        }
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Parsed;
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::SimpleEscapeBackslash> {
            let ok_result = parse(state, cache)?;
            Ok(ok_result.map(|r| super::SimpleEscapeBackslash {}))
        }
    }
    #[inline]
    pub(super) fn parse_SimpleEscapeBackslash<'a>(
        state: ParseState<'a>,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, SimpleEscapeBackslash> {
        let cache_key = state.cache_key();
        if let Some(cached) = cache.c_SimpleEscapeBackslash.get(&cache_key) {
            state.print_trace_cached("SimpleEscapeBackslash");
            state.print_trace_result(&cached);
            cached.clone()
        } else {
            state.print_trace_start("SimpleEscapeBackslash");
            let result = SimpleEscapeBackslash_impl::rule_parser(state.clone().indent(), cache);
            state.print_trace_result(&result);
            let result = result.map(|result| ParseOk {
                state: result.state.dedent(),
                ..result
            });
            cache
                .c_SimpleEscapeBackslash
                .insert(cache_key, result.clone());
            result
        }
    }
    mod SimpleEscapeQuote_impl {
        use super::*;
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let ok_result = parse_character_literal(state, '\'')?;
            Ok(ok_result.map(|_| Parsed))
        }
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Parsed;
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::SimpleEscapeQuote> {
            let ok_result = parse(state, cache)?;
            Ok(ok_result.map(|r| super::SimpleEscapeQuote {}))
        }
    }
    #[inline]
    pub(super) fn parse_SimpleEscapeQuote<'a>(
        state: ParseState<'a>,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, SimpleEscapeQuote> {
        let cache_key = state.cache_key();
        if let Some(cached) = cache.c_SimpleEscapeQuote.get(&cache_key) {
            state.print_trace_cached("SimpleEscapeQuote");
            state.print_trace_result(&cached);
            cached.clone()
        } else {
            state.print_trace_start("SimpleEscapeQuote");
            let result = SimpleEscapeQuote_impl::rule_parser(state.clone().indent(), cache);
            state.print_trace_result(&result);
            let result = result.map(|result| ParseOk {
                state: result.state.dedent(),
                ..result
            });
            cache.c_SimpleEscapeQuote.insert(cache_key, result.clone());
            result
        }
    }
    mod SimpleEscapeDQuote_impl {
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
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::SimpleEscapeDQuote> {
            let ok_result = parse(state, cache)?;
            Ok(ok_result.map(|r| super::SimpleEscapeDQuote {}))
        }
    }
    #[inline]
    pub(super) fn parse_SimpleEscapeDQuote<'a>(
        state: ParseState<'a>,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, SimpleEscapeDQuote> {
        let cache_key = state.cache_key();
        if let Some(cached) = cache.c_SimpleEscapeDQuote.get(&cache_key) {
            state.print_trace_cached("SimpleEscapeDQuote");
            state.print_trace_result(&cached);
            cached.clone()
        } else {
            state.print_trace_start("SimpleEscapeDQuote");
            let result = SimpleEscapeDQuote_impl::rule_parser(state.clone().indent(), cache);
            state.print_trace_result(&result);
            let result = result.map(|result| ParseOk {
                state: result.state.dedent(),
                ..result
            });
            cache.c_SimpleEscapeDQuote.insert(cache_key, result.clone());
            result
        }
    }
    mod HexaEscape_impl {
        use super::*;
        mod part_0 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let ok_result = parse_character_literal(state, 'x')?;
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
                let ok_result = parse_HexChar(state, cache)?;
                Ok(ok_result.map(|result| Parsed { c1: result }))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub c1: HexChar,
            }
        }
        mod part_2 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let ok_result = parse_HexChar(state, cache)?;
                Ok(ok_result.map(|result| Parsed { c2: result }))
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub c2: HexChar,
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
            let mut c1 = result.c1;
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
            let mut c2 = result.c2;
            Ok(ParseOk {
                result: Parsed { c1, c2 },
                state,
                farthest_error,
            })
        }
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Parsed {
            pub c1: HexChar,
            pub c2: HexChar,
        }
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::HexaEscape> {
            let ok_result = parse(state, cache)?;
            Ok(ok_result.map(|r| super::HexaEscape { c1: r.c1, c2: r.c2 }))
        }
    }
    #[inline]
    pub(super) fn parse_HexaEscape<'a>(
        state: ParseState<'a>,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, HexaEscape> {
        let cache_key = state.cache_key();
        if let Some(cached) = cache.c_HexaEscape.get(&cache_key) {
            state.print_trace_cached("HexaEscape");
            state.print_trace_result(&cached);
            cached.clone()
        } else {
            state.print_trace_start("HexaEscape");
            let result = HexaEscape_impl::rule_parser(state.clone().indent(), cache);
            state.print_trace_result(&result);
            let result = result.map(|result| ParseOk {
                state: result.state.dedent(),
                ..result
            });
            cache.c_HexaEscape.insert(cache_key, result.clone());
            result
        }
    }
    mod HexChar_impl {
        use super::*;
        mod choice_0 {
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
        mod choice_1 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let ok_result = parse_character_range(state, 'a', 'f')?;
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
                let ok_result = parse_character_range(state, 'A', 'F')?;
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
            Err(farthest_error.unwrap_or_else(|| state.report_error(ParseErrorSpecifics::Other)))
        }
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Parsed;
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, String> {
            let ok_result = parse(state.clone(), cache)?;
            Ok(ok_result.map_with_state(|_, new_state| state.slice_until(&new_state).to_string()))
        }
    }
    #[inline]
    pub(super) fn parse_HexChar<'a>(
        state: ParseState<'a>,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, HexChar> {
        let cache_key = state.cache_key();
        if let Some(cached) = cache.c_HexChar.get(&cache_key) {
            state.print_trace_cached("HexChar");
            state.print_trace_result(&cached);
            cached.clone()
        } else {
            state.print_trace_start("HexChar");
            let result = HexChar_impl::rule_parser(state.clone().indent(), cache);
            state.print_trace_result(&result);
            let result = result.map(|result| ParseOk {
                state: result.state.dedent(),
                ..result
            });
            cache.c_HexChar.insert(cache_key, result.clone());
            result
        }
    }
    mod Utf8Escape_impl {
        use super::*;
        mod choice_0 {
            use super::*;
            mod part_0 {
                use super::*;
                #[inline(always)]
                pub fn parse<'a>(
                    state: ParseState<'a>,
                    cache: &mut ParseCache<'a>,
                ) -> ParseResult<'a, Parsed> {
                    let ok_result = parse_character_literal(state, 'u')?;
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
                    let ok_result = parse_character_literal(state, '{')?;
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
                    let ok_result = parse_HexChar(state, cache)?;
                    Ok(ok_result.map(|result| Parsed { c1: result }))
                }
                #[derive(Debug, Clone, PartialEq, Eq)]
                pub struct Parsed {
                    pub c1: HexChar,
                }
            }
            mod part_3 {
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
                            let ok_result = parse_HexChar(state, cache)?;
                            Ok(ok_result.map(|result| Parsed { c2: Some(result) }))
                        }
                        #[derive(Debug, Clone, PartialEq, Eq)]
                        pub struct Parsed {
                            pub c2: Option<HexChar>,
                        }
                    }
                    mod part_1 {
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
                                    let ok_result = parse_HexChar(state, cache)?;
                                    Ok(ok_result.map(|result| Parsed { c3: Some(result) }))
                                }
                                #[derive(Debug, Clone, PartialEq, Eq)]
                                pub struct Parsed {
                                    pub c3: Option<HexChar>,
                                }
                            }
                            mod part_1 {
                                use super::*;
                                mod optional {
                                    use super::*;
                                    mod part_0 {
                                        use super::*;
                                        #[inline(always)]
                                        pub fn parse<'a>(
                                            state: ParseState<'a>,
                                            cache: &mut ParseCache<'a>,
                                        ) -> ParseResult<'a, Parsed>
                                        {
                                            let ok_result = parse_HexChar(state, cache)?;
                                            Ok(ok_result.map(|result| Parsed { c4: Some(result) }))
                                        }
                                        #[derive(Debug, Clone, PartialEq, Eq)]
                                        pub struct Parsed {
                                            pub c4: Option<HexChar>,
                                        }
                                    }
                                    mod part_1 {
                                        use super::*;
                                        mod optional {
                                            use super::*;
                                            mod part_0 {
                                                use super::*;
                                                #[inline(always)]
                                                pub fn parse<'a>(
                                                    state: ParseState<'a>,
                                                    cache: &mut ParseCache<'a>,
                                                ) -> ParseResult<'a, Parsed>
                                                {
                                                    let ok_result = parse_HexChar(state, cache)?;
                                                    Ok(ok_result
                                                        .map(|result| Parsed { c5: Some(result) }))
                                                }
                                                #[derive(Debug, Clone, PartialEq, Eq)]
                                                pub struct Parsed {
                                                    pub c5: Option<HexChar>,
                                                }
                                            }
                                            mod part_1 {
                                                use super::*;
                                                mod optional {
                                                    use super::*;
                                                    #[inline(always)]
                                                    pub fn parse<'a>(
                                                        state: ParseState<'a>,
                                                        cache: &mut ParseCache<'a>,
                                                    ) -> ParseResult<'a, Parsed>
                                                    {
                                                        let ok_result =
                                                            parse_HexChar(state, cache)?;
                                                        Ok(ok_result.map(|result| Parsed {
                                                            c6: Some(result),
                                                        }))
                                                    }
                                                    #[derive(Debug, Clone, PartialEq, Eq)]
                                                    pub struct Parsed {
                                                        pub c6: Option<HexChar>,
                                                    }
                                                }
                                                #[inline(always)]
                                                pub fn parse<'a>(
                                                    state: ParseState<'a>,
                                                    cache: &mut ParseCache<'a>,
                                                ) -> ParseResult<'a, Parsed>
                                                {
                                                    match optional::parse(state.clone(), cache) {
                                                        Ok(ok_result) => {
                                                            Ok(ok_result.map(|result| Parsed {
                                                                c6: result.c6,
                                                            }))
                                                        }
                                                        Err(err) => Ok(ParseOk {
                                                            result: Parsed {
                                                                c6: Default::default(),
                                                            },
                                                            state,
                                                            farthest_error: Some(err),
                                                        }),
                                                    }
                                                }
                                                #[derive(Debug, Clone, PartialEq, Eq)]
                                                pub struct Parsed {
                                                    pub c6: Option<HexChar>,
                                                }
                                            }
                                            #[inline(always)]
                                            pub fn parse<'a>(
                                                state: ParseState<'a>,
                                                cache: &mut ParseCache<'a>,
                                            ) -> ParseResult<'a, Parsed>
                                            {
                                                let mut state = state;
                                                let mut farthest_error: Option<ParseError> = None;
                                                let result = match part_0::parse(state, cache) {
                                                    Ok(ParseOk {
                                                        result,
                                                        state: new_state,
                                                        farthest_error: new_farthest_error,
                                                    }) => {
                                                        farthest_error = combine_errors(
                                                            farthest_error,
                                                            new_farthest_error,
                                                        );
                                                        state = new_state;
                                                        result
                                                    }
                                                    Err(err) => {
                                                        return Err(combine_errors(
                                                            farthest_error,
                                                            Some(err),
                                                        )
                                                        .unwrap())
                                                    }
                                                };
                                                let mut c5 = result.c5;
                                                let result = match part_1::parse(state, cache) {
                                                    Ok(ParseOk {
                                                        result,
                                                        state: new_state,
                                                        farthest_error: new_farthest_error,
                                                    }) => {
                                                        farthest_error = combine_errors(
                                                            farthest_error,
                                                            new_farthest_error,
                                                        );
                                                        state = new_state;
                                                        result
                                                    }
                                                    Err(err) => {
                                                        return Err(combine_errors(
                                                            farthest_error,
                                                            Some(err),
                                                        )
                                                        .unwrap())
                                                    }
                                                };
                                                let mut c6 = result.c6;
                                                Ok(ParseOk {
                                                    result: Parsed { c5, c6 },
                                                    state,
                                                    farthest_error,
                                                })
                                            }
                                            #[derive(Debug, Clone, PartialEq, Eq)]
                                            pub struct Parsed {
                                                pub c5: Option<HexChar>,
                                                pub c6: Option<HexChar>,
                                            }
                                        }
                                        #[inline(always)]
                                        pub fn parse<'a>(
                                            state: ParseState<'a>,
                                            cache: &mut ParseCache<'a>,
                                        ) -> ParseResult<'a, Parsed>
                                        {
                                            match optional::parse(state.clone(), cache) {
                                                Ok(ok_result) => {
                                                    Ok(ok_result.map(|result| Parsed {
                                                        c5: result.c5,
                                                        c6: result.c6,
                                                    }))
                                                }
                                                Err(err) => Ok(ParseOk {
                                                    result: Parsed {
                                                        c5: Default::default(),
                                                        c6: Default::default(),
                                                    },
                                                    state,
                                                    farthest_error: Some(err),
                                                }),
                                            }
                                        }
                                        #[derive(Debug, Clone, PartialEq, Eq)]
                                        pub struct Parsed {
                                            pub c5: Option<HexChar>,
                                            pub c6: Option<HexChar>,
                                        }
                                    }
                                    #[inline(always)]
                                    pub fn parse<'a>(
                                        state: ParseState<'a>,
                                        cache: &mut ParseCache<'a>,
                                    ) -> ParseResult<'a, Parsed>
                                    {
                                        let mut state = state;
                                        let mut farthest_error: Option<ParseError> = None;
                                        let result = match part_0::parse(state, cache) {
                                            Ok(ParseOk {
                                                result,
                                                state: new_state,
                                                farthest_error: new_farthest_error,
                                            }) => {
                                                farthest_error = combine_errors(
                                                    farthest_error,
                                                    new_farthest_error,
                                                );
                                                state = new_state;
                                                result
                                            }
                                            Err(err) => {
                                                return Err(combine_errors(
                                                    farthest_error,
                                                    Some(err),
                                                )
                                                .unwrap())
                                            }
                                        };
                                        let mut c4 = result.c4;
                                        let result = match part_1::parse(state, cache) {
                                            Ok(ParseOk {
                                                result,
                                                state: new_state,
                                                farthest_error: new_farthest_error,
                                            }) => {
                                                farthest_error = combine_errors(
                                                    farthest_error,
                                                    new_farthest_error,
                                                );
                                                state = new_state;
                                                result
                                            }
                                            Err(err) => {
                                                return Err(combine_errors(
                                                    farthest_error,
                                                    Some(err),
                                                )
                                                .unwrap())
                                            }
                                        };
                                        let mut c5 = result.c5;
                                        let mut c6 = result.c6;
                                        Ok(ParseOk {
                                            result: Parsed { c4, c5, c6 },
                                            state,
                                            farthest_error,
                                        })
                                    }
                                    #[derive(Debug, Clone, PartialEq, Eq)]
                                    pub struct Parsed {
                                        pub c4: Option<HexChar>,
                                        pub c5: Option<HexChar>,
                                        pub c6: Option<HexChar>,
                                    }
                                }
                                #[inline(always)]
                                pub fn parse<'a>(
                                    state: ParseState<'a>,
                                    cache: &mut ParseCache<'a>,
                                ) -> ParseResult<'a, Parsed> {
                                    match optional::parse(state.clone(), cache) {
                                        Ok(ok_result) => Ok(ok_result.map(|result| Parsed {
                                            c4: result.c4,
                                            c5: result.c5,
                                            c6: result.c6,
                                        })),
                                        Err(err) => Ok(ParseOk {
                                            result: Parsed {
                                                c4: Default::default(),
                                                c5: Default::default(),
                                                c6: Default::default(),
                                            },
                                            state,
                                            farthest_error: Some(err),
                                        }),
                                    }
                                }
                                #[derive(Debug, Clone, PartialEq, Eq)]
                                pub struct Parsed {
                                    pub c4: Option<HexChar>,
                                    pub c5: Option<HexChar>,
                                    pub c6: Option<HexChar>,
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
                                        farthest_error =
                                            combine_errors(farthest_error, new_farthest_error);
                                        state = new_state;
                                        result
                                    }
                                    Err(err) => {
                                        return Err(
                                            combine_errors(farthest_error, Some(err)).unwrap()
                                        )
                                    }
                                };
                                let mut c3 = result.c3;
                                let result = match part_1::parse(state, cache) {
                                    Ok(ParseOk {
                                        result,
                                        state: new_state,
                                        farthest_error: new_farthest_error,
                                    }) => {
                                        farthest_error =
                                            combine_errors(farthest_error, new_farthest_error);
                                        state = new_state;
                                        result
                                    }
                                    Err(err) => {
                                        return Err(
                                            combine_errors(farthest_error, Some(err)).unwrap()
                                        )
                                    }
                                };
                                let mut c4 = result.c4;
                                let mut c5 = result.c5;
                                let mut c6 = result.c6;
                                Ok(ParseOk {
                                    result: Parsed { c3, c4, c5, c6 },
                                    state,
                                    farthest_error,
                                })
                            }
                            #[derive(Debug, Clone, PartialEq, Eq)]
                            pub struct Parsed {
                                pub c3: Option<HexChar>,
                                pub c4: Option<HexChar>,
                                pub c5: Option<HexChar>,
                                pub c6: Option<HexChar>,
                            }
                        }
                        #[inline(always)]
                        pub fn parse<'a>(
                            state: ParseState<'a>,
                            cache: &mut ParseCache<'a>,
                        ) -> ParseResult<'a, Parsed> {
                            match optional::parse(state.clone(), cache) {
                                Ok(ok_result) => Ok(ok_result.map(|result| Parsed {
                                    c3: result.c3,
                                    c4: result.c4,
                                    c5: result.c5,
                                    c6: result.c6,
                                })),
                                Err(err) => Ok(ParseOk {
                                    result: Parsed {
                                        c3: Default::default(),
                                        c4: Default::default(),
                                        c5: Default::default(),
                                        c6: Default::default(),
                                    },
                                    state,
                                    farthest_error: Some(err),
                                }),
                            }
                        }
                        #[derive(Debug, Clone, PartialEq, Eq)]
                        pub struct Parsed {
                            pub c3: Option<HexChar>,
                            pub c4: Option<HexChar>,
                            pub c5: Option<HexChar>,
                            pub c6: Option<HexChar>,
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
                            Err(err) => {
                                return Err(combine_errors(farthest_error, Some(err)).unwrap())
                            }
                        };
                        let mut c2 = result.c2;
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
                            Err(err) => {
                                return Err(combine_errors(farthest_error, Some(err)).unwrap())
                            }
                        };
                        let mut c3 = result.c3;
                        let mut c4 = result.c4;
                        let mut c5 = result.c5;
                        let mut c6 = result.c6;
                        Ok(ParseOk {
                            result: Parsed { c2, c3, c4, c5, c6 },
                            state,
                            farthest_error,
                        })
                    }
                    #[derive(Debug, Clone, PartialEq, Eq)]
                    pub struct Parsed {
                        pub c2: Option<HexChar>,
                        pub c3: Option<HexChar>,
                        pub c4: Option<HexChar>,
                        pub c5: Option<HexChar>,
                        pub c6: Option<HexChar>,
                    }
                }
                #[inline(always)]
                pub fn parse<'a>(
                    state: ParseState<'a>,
                    cache: &mut ParseCache<'a>,
                ) -> ParseResult<'a, Parsed> {
                    match optional::parse(state.clone(), cache) {
                        Ok(ok_result) => Ok(ok_result.map(|result| Parsed {
                            c2: result.c2,
                            c3: result.c3,
                            c4: result.c4,
                            c5: result.c5,
                            c6: result.c6,
                        })),
                        Err(err) => Ok(ParseOk {
                            result: Parsed {
                                c2: Default::default(),
                                c3: Default::default(),
                                c4: Default::default(),
                                c5: Default::default(),
                                c6: Default::default(),
                            },
                            state,
                            farthest_error: Some(err),
                        }),
                    }
                }
                #[derive(Debug, Clone, PartialEq, Eq)]
                pub struct Parsed {
                    pub c2: Option<HexChar>,
                    pub c3: Option<HexChar>,
                    pub c4: Option<HexChar>,
                    pub c5: Option<HexChar>,
                    pub c6: Option<HexChar>,
                }
            }
            mod part_4 {
                use super::*;
                #[inline(always)]
                pub fn parse<'a>(
                    state: ParseState<'a>,
                    cache: &mut ParseCache<'a>,
                ) -> ParseResult<'a, Parsed> {
                    let ok_result = parse_character_literal(state, '}')?;
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
                let mut c1 = result.c1;
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
                let mut c2 = result.c2;
                let mut c3 = result.c3;
                let mut c4 = result.c4;
                let mut c5 = result.c5;
                let mut c6 = result.c6;
                match part_4::parse(state, cache) {
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
                    result: Parsed {
                        c1,
                        c2,
                        c3,
                        c4,
                        c5,
                        c6,
                    },
                    state,
                    farthest_error,
                })
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub c1: HexChar,
                pub c2: Option<HexChar>,
                pub c3: Option<HexChar>,
                pub c4: Option<HexChar>,
                pub c5: Option<HexChar>,
                pub c6: Option<HexChar>,
            }
        }
        mod choice_1 {
            use super::*;
            mod part_0 {
                use super::*;
                #[inline(always)]
                pub fn parse<'a>(
                    state: ParseState<'a>,
                    cache: &mut ParseCache<'a>,
                ) -> ParseResult<'a, Parsed> {
                    let ok_result = parse_character_literal(state, 'u')?;
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
                    let ok_result = parse_HexChar(state, cache)?;
                    Ok(ok_result.map(|result| Parsed { c1: result }))
                }
                #[derive(Debug, Clone, PartialEq, Eq)]
                pub struct Parsed {
                    pub c1: HexChar,
                }
            }
            mod part_2 {
                use super::*;
                #[inline(always)]
                pub fn parse<'a>(
                    state: ParseState<'a>,
                    cache: &mut ParseCache<'a>,
                ) -> ParseResult<'a, Parsed> {
                    let ok_result = parse_HexChar(state, cache)?;
                    Ok(ok_result.map(|result| Parsed { c2: Some(result) }))
                }
                #[derive(Debug, Clone, PartialEq, Eq)]
                pub struct Parsed {
                    pub c2: Option<HexChar>,
                }
            }
            mod part_3 {
                use super::*;
                #[inline(always)]
                pub fn parse<'a>(
                    state: ParseState<'a>,
                    cache: &mut ParseCache<'a>,
                ) -> ParseResult<'a, Parsed> {
                    let ok_result = parse_HexChar(state, cache)?;
                    Ok(ok_result.map(|result| Parsed { c3: Some(result) }))
                }
                #[derive(Debug, Clone, PartialEq, Eq)]
                pub struct Parsed {
                    pub c3: Option<HexChar>,
                }
            }
            mod part_4 {
                use super::*;
                #[inline(always)]
                pub fn parse<'a>(
                    state: ParseState<'a>,
                    cache: &mut ParseCache<'a>,
                ) -> ParseResult<'a, Parsed> {
                    let ok_result = parse_HexChar(state, cache)?;
                    Ok(ok_result.map(|result| Parsed { c4: Some(result) }))
                }
                #[derive(Debug, Clone, PartialEq, Eq)]
                pub struct Parsed {
                    pub c4: Option<HexChar>,
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
                let mut c1 = result.c1;
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
                let mut c2 = result.c2;
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
                let mut c3 = result.c3;
                let result = match part_4::parse(state, cache) {
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
                let mut c4 = result.c4;
                Ok(ParseOk {
                    result: Parsed { c1, c2, c3, c4 },
                    state,
                    farthest_error,
                })
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub c1: HexChar,
                pub c2: Option<HexChar>,
                pub c3: Option<HexChar>,
                pub c4: Option<HexChar>,
            }
        }
        mod choice_2 {
            use super::*;
            mod part_0 {
                use super::*;
                #[inline(always)]
                pub fn parse<'a>(
                    state: ParseState<'a>,
                    cache: &mut ParseCache<'a>,
                ) -> ParseResult<'a, Parsed> {
                    let ok_result = parse_character_literal(state, 'U')?;
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
                    let ok_result = parse_character_literal(state, '0')?;
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
                    let ok_result = parse_character_literal(state, '0')?;
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
                    let ok_result = parse_HexChar(state, cache)?;
                    Ok(ok_result.map(|result| Parsed { c1: result }))
                }
                #[derive(Debug, Clone, PartialEq, Eq)]
                pub struct Parsed {
                    pub c1: HexChar,
                }
            }
            mod part_4 {
                use super::*;
                #[inline(always)]
                pub fn parse<'a>(
                    state: ParseState<'a>,
                    cache: &mut ParseCache<'a>,
                ) -> ParseResult<'a, Parsed> {
                    let ok_result = parse_HexChar(state, cache)?;
                    Ok(ok_result.map(|result| Parsed { c2: Some(result) }))
                }
                #[derive(Debug, Clone, PartialEq, Eq)]
                pub struct Parsed {
                    pub c2: Option<HexChar>,
                }
            }
            mod part_5 {
                use super::*;
                #[inline(always)]
                pub fn parse<'a>(
                    state: ParseState<'a>,
                    cache: &mut ParseCache<'a>,
                ) -> ParseResult<'a, Parsed> {
                    let ok_result = parse_HexChar(state, cache)?;
                    Ok(ok_result.map(|result| Parsed { c3: Some(result) }))
                }
                #[derive(Debug, Clone, PartialEq, Eq)]
                pub struct Parsed {
                    pub c3: Option<HexChar>,
                }
            }
            mod part_6 {
                use super::*;
                #[inline(always)]
                pub fn parse<'a>(
                    state: ParseState<'a>,
                    cache: &mut ParseCache<'a>,
                ) -> ParseResult<'a, Parsed> {
                    let ok_result = parse_HexChar(state, cache)?;
                    Ok(ok_result.map(|result| Parsed { c4: Some(result) }))
                }
                #[derive(Debug, Clone, PartialEq, Eq)]
                pub struct Parsed {
                    pub c4: Option<HexChar>,
                }
            }
            mod part_7 {
                use super::*;
                #[inline(always)]
                pub fn parse<'a>(
                    state: ParseState<'a>,
                    cache: &mut ParseCache<'a>,
                ) -> ParseResult<'a, Parsed> {
                    let ok_result = parse_HexChar(state, cache)?;
                    Ok(ok_result.map(|result| Parsed { c5: Some(result) }))
                }
                #[derive(Debug, Clone, PartialEq, Eq)]
                pub struct Parsed {
                    pub c5: Option<HexChar>,
                }
            }
            mod part_8 {
                use super::*;
                #[inline(always)]
                pub fn parse<'a>(
                    state: ParseState<'a>,
                    cache: &mut ParseCache<'a>,
                ) -> ParseResult<'a, Parsed> {
                    let ok_result = parse_HexChar(state, cache)?;
                    Ok(ok_result.map(|result| Parsed { c6: Some(result) }))
                }
                #[derive(Debug, Clone, PartialEq, Eq)]
                pub struct Parsed {
                    pub c6: Option<HexChar>,
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
                let mut c1 = result.c1;
                let result = match part_4::parse(state, cache) {
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
                let mut c2 = result.c2;
                let result = match part_5::parse(state, cache) {
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
                let mut c3 = result.c3;
                let result = match part_6::parse(state, cache) {
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
                let mut c4 = result.c4;
                let result = match part_7::parse(state, cache) {
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
                let mut c5 = result.c5;
                let result = match part_8::parse(state, cache) {
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
                let mut c6 = result.c6;
                Ok(ParseOk {
                    result: Parsed {
                        c1,
                        c2,
                        c3,
                        c4,
                        c5,
                        c6,
                    },
                    state,
                    farthest_error,
                })
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Parsed {
                pub c1: HexChar,
                pub c2: Option<HexChar>,
                pub c3: Option<HexChar>,
                pub c4: Option<HexChar>,
                pub c5: Option<HexChar>,
                pub c6: Option<HexChar>,
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
                        c1: result.c1,
                        c2: result.c2,
                        c3: result.c3,
                        c4: result.c4,
                        c5: result.c5,
                        c6: result.c6,
                    }))
                }
                Err(err) => farthest_error = combine_errors(farthest_error, Some(err)),
            }
            match choice_1::parse(state.clone(), cache) {
                Ok(ok_result) => {
                    return Ok(ok_result.map(|result| Parsed {
                        c1: result.c1,
                        c2: result.c2,
                        c3: result.c3,
                        c4: result.c4,
                        c5: None,
                        c6: None,
                    }))
                }
                Err(err) => farthest_error = combine_errors(farthest_error, Some(err)),
            }
            match choice_2::parse(state.clone(), cache) {
                Ok(ok_result) => {
                    return Ok(ok_result.map(|result| Parsed {
                        c1: result.c1,
                        c2: result.c2,
                        c3: result.c3,
                        c4: result.c4,
                        c5: result.c5,
                        c6: result.c6,
                    }))
                }
                Err(err) => farthest_error = combine_errors(farthest_error, Some(err)),
            }
            Err(farthest_error.unwrap_or_else(|| state.report_error(ParseErrorSpecifics::Other)))
        }
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Parsed {
            pub c1: HexChar,
            pub c2: Option<HexChar>,
            pub c3: Option<HexChar>,
            pub c4: Option<HexChar>,
            pub c5: Option<HexChar>,
            pub c6: Option<HexChar>,
        }
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::Utf8Escape> {
            let ok_result = parse(state, cache)?;
            Ok(ok_result.map(|r| super::Utf8Escape {
                c1: r.c1,
                c2: r.c2,
                c3: r.c3,
                c4: r.c4,
                c5: r.c5,
                c6: r.c6,
            }))
        }
    }
    #[inline]
    pub(super) fn parse_Utf8Escape<'a>(
        state: ParseState<'a>,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, Utf8Escape> {
        let cache_key = state.cache_key();
        if let Some(cached) = cache.c_Utf8Escape.get(&cache_key) {
            state.print_trace_cached("Utf8Escape");
            state.print_trace_result(&cached);
            cached.clone()
        } else {
            state.print_trace_start("Utf8Escape");
            let result = Utf8Escape_impl::rule_parser(state.clone().indent(), cache);
            state.print_trace_result(&result);
            let result = result.map(|result| ParseOk {
                state: result.state.dedent(),
                ..result
            });
            cache.c_Utf8Escape.insert(cache_key, result.clone());
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
        mod choice_3 {
            use super::*;
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let state = state.skip_whitespace();
                let ok_result = parse_PositionDirective(state, cache)?;
                Ok(ok_result.map(|result| Parsed {
                    _override: Parsed__override::PositionDirective(result),
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
            Err(farthest_error.unwrap_or_else(|| state.report_error(ParseErrorSpecifics::Other)))
        }
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Parsed {
            pub _override: Parsed__override,
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
            state.print_trace_cached("DirectiveExpression");
            state.print_trace_result(&cached);
            cached.clone()
        } else {
            state.print_trace_start("DirectiveExpression");
            let result = DirectiveExpression_impl::rule_parser(state.clone().indent(), cache);
            state.print_trace_result(&result);
            let result = result.map(|result| ParseOk {
                state: result.state.dedent(),
                ..result
            });
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
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Parsed;
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::StringDirective> {
            let ok_result = parse(state, cache)?;
            Ok(ok_result.map(|r| super::StringDirective {}))
        }
    }
    #[inline]
    pub(super) fn parse_StringDirective<'a>(
        state: ParseState<'a>,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, StringDirective> {
        let cache_key = state.cache_key();
        if let Some(cached) = cache.c_StringDirective.get(&cache_key) {
            state.print_trace_cached("StringDirective");
            state.print_trace_result(&cached);
            cached.clone()
        } else {
            state.print_trace_start("StringDirective");
            let result = StringDirective_impl::rule_parser(state.clone().indent(), cache);
            state.print_trace_result(&result);
            let result = result.map(|result| ParseOk {
                state: result.state.dedent(),
                ..result
            });
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
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Parsed;
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::NoSkipWsDirective> {
            let ok_result = parse(state, cache)?;
            Ok(ok_result.map(|r| super::NoSkipWsDirective {}))
        }
    }
    #[inline]
    pub(super) fn parse_NoSkipWsDirective<'a>(
        state: ParseState<'a>,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, NoSkipWsDirective> {
        let cache_key = state.cache_key();
        if let Some(cached) = cache.c_NoSkipWsDirective.get(&cache_key) {
            state.print_trace_cached("NoSkipWsDirective");
            state.print_trace_result(&cached);
            cached.clone()
        } else {
            state.print_trace_start("NoSkipWsDirective");
            let result = NoSkipWsDirective_impl::rule_parser(state.clone().indent(), cache);
            state.print_trace_result(&result);
            let result = result.map(|result| ParseOk {
                state: result.state.dedent(),
                ..result
            });
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
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Parsed;
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::ExportDirective> {
            let ok_result = parse(state, cache)?;
            Ok(ok_result.map(|r| super::ExportDirective {}))
        }
    }
    #[inline]
    pub(super) fn parse_ExportDirective<'a>(
        state: ParseState<'a>,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, ExportDirective> {
        let cache_key = state.cache_key();
        if let Some(cached) = cache.c_ExportDirective.get(&cache_key) {
            state.print_trace_cached("ExportDirective");
            state.print_trace_result(&cached);
            cached.clone()
        } else {
            state.print_trace_start("ExportDirective");
            let result = ExportDirective_impl::rule_parser(state.clone().indent(), cache);
            state.print_trace_result(&result);
            let result = result.map(|result| ParseOk {
                state: result.state.dedent(),
                ..result
            });
            cache.c_ExportDirective.insert(cache_key, result.clone());
            result
        }
    }
    mod PositionDirective_impl {
        use super::*;
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let state = state.skip_whitespace();
            let ok_result = parse_string_literal(state, "@position")?;
            Ok(ok_result.map(|_| Parsed))
        }
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Parsed;
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::PositionDirective> {
            let ok_result = parse(state, cache)?;
            Ok(ok_result.map(|r| super::PositionDirective {}))
        }
    }
    #[inline]
    pub(super) fn parse_PositionDirective<'a>(
        state: ParseState<'a>,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, PositionDirective> {
        let cache_key = state.cache_key();
        if let Some(cached) = cache.c_PositionDirective.get(&cache_key) {
            state.print_trace_cached("PositionDirective");
            state.print_trace_result(&cached);
            cached.clone()
        } else {
            state.print_trace_start("PositionDirective");
            let result = PositionDirective_impl::rule_parser(state.clone().indent(), cache);
            state.print_trace_result(&result);
            let result = result.map(|result| ParseOk {
                state: result.state.dedent(),
                ..result
            });
            cache.c_PositionDirective.insert(cache_key, result.clone());
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
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Parsed;
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::EndOfInput> {
            let ok_result = parse(state, cache)?;
            Ok(ok_result.map(|r| super::EndOfInput {}))
        }
    }
    #[inline]
    pub(super) fn parse_EndOfInput<'a>(
        state: ParseState<'a>,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, EndOfInput> {
        let cache_key = state.cache_key();
        if let Some(cached) = cache.c_EndOfInput.get(&cache_key) {
            state.print_trace_cached("EndOfInput");
            state.print_trace_result(&cached);
            cached.clone()
        } else {
            state.print_trace_start("EndOfInput");
            let result = EndOfInput_impl::rule_parser(state.clone().indent(), cache);
            state.print_trace_result(&result);
            let result = result.map(|result| ParseOk {
                state: result.state.dedent(),
                ..result
            });
            cache.c_EndOfInput.insert(cache_key, result.clone());
            result
        }
    }
}
