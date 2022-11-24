// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

mod grammar;
use grammar::*;
use peginator_runtime::PegParser;

trait Calculate {
    fn calculate(&self) -> i64;
}

impl Calculate for Number {
    fn calculate(&self) -> i64 {
        self.parse().unwrap()
    }
}

impl Calculate for Group {
    fn calculate(&self) -> i64 {
        self.body.calculate()
    }
}

impl Calculate for Add {
    fn calculate(&self) -> i64 {
        self.left.calculate() + self.right.calculate()
    }
}

impl Calculate for Sub {
    fn calculate(&self) -> i64 {
        self.left.calculate() - self.right.calculate()
    }
}
impl Calculate for Mul {
    fn calculate(&self) -> i64 {
        self.left.calculate() * self.right.calculate()
    }
}
impl Calculate for Div {
    fn calculate(&self) -> i64 {
        self.left.calculate() / self.right.calculate()
    }
}

// ==== Boilerplate. This could be replaced with enum_dispatch ====
impl Calculate for Expression {
    fn calculate(&self) -> i64 {
        match self {
            Expression::Add(x) => x.calculate(),
            Expression::Sub(x) => x.calculate(),
            Expression::Term(x) => x.calculate(),
        }
    }
}
impl Calculate for Term {
    fn calculate(&self) -> i64 {
        match self {
            Term::Div(x) => x.calculate(),
            Term::Factor(x) => x.calculate(),
            Term::Mul(x) => x.calculate(),
        }
    }
}
impl Calculate for Factor {
    fn calculate(&self) -> i64 {
        match self {
            Factor::Group(x) => x.calculate(),
            Factor::Number(x) => x.calculate(),
        }
    }
}
// ============

#[test]
fn test() {
    assert_eq!(
        Expression::parse_with_trace("1 - 2 + 3").unwrap(),
        Expression::Add(Add {
            left: Box::new(Expression::Sub(Sub {
                left: Box::new(Expression::Term(Term::Factor(Factor::Number("1".into())))),
                right: Term::Factor(Factor::Number("2".into())),
            })),
            right: Term::Factor(Factor::Number("3".into())),
        })
    );
    assert_eq!(
        Expression::parse_with_trace("(1 - 2) * 3 / 4 + 5").unwrap(),
        Expression::Add(Add {
            left: Box::new(Expression::Term(Term::Div(Div {
                left: Box::new(Term::Mul(Mul {
                    left: Box::new(Term::Factor(Factor::Group(Group {
                        body: Box::new(Expression::Sub(Sub {
                            left: Box::new(Expression::Term(Term::Factor(Factor::Number(
                                "1".into()
                            )))),
                            right: Term::Factor(Factor::Number("2".into()))
                        }))
                    }))),
                    right: Factor::Number("3".into())
                })),
                right: Factor::Number("4".into())
            }))),
            right: Term::Factor(Factor::Number("5".into()))
        })
    );
    assert_eq!(
        Expression::parse("(15-27) * 3 / 4 + 123 - (5*7 + 2)")
            .unwrap()
            .calculate(),
        77
    );
}
