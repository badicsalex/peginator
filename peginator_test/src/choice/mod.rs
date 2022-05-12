// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

mod grammar;
use grammar::*;
use peginator::PegParser;

use crate::test_utils::assert_type_eq;

#[test]
fn test_simple() {
    let s = Simple::parse("a").unwrap();
    assert_eq!(s.f, Simple_f::FieldA(FieldA));

    assert!(Simple::parse("x").is_err())
}

#[test]
fn test_types() {
    assert_type_eq!(Arity11, f, FieldA);
    assert_type_eq!(Arity1O, f, Option<FieldA>);
    assert_type_eq!(Arity1M, f, Vec<FieldA>);
    assert_type_eq!(ArityO1, f, Option<FieldA>);
    assert_type_eq!(ArityOO, f, Option<FieldA>);
    assert_type_eq!(ArityOM, f, Vec<FieldA>);
    assert_type_eq!(ArityM1, f, Vec<FieldA>);
    assert_type_eq!(ArityMO, f, Vec<FieldA>);
    assert_type_eq!(ArityMM, f, Vec<FieldA>);

    assert_type_eq!(DefaultO, f, Option<FieldA>);
    assert_type_eq!(DefaultM, f, Vec<FieldA>);
}

#[test]
fn test_arities_1() {
    assert_eq!(Arity11::parse("1a").unwrap(), Arity11 { f: FieldA });
    assert_eq!(Arity11::parse("2a").unwrap(), Arity11 { f: FieldA });
    assert!(Arity11::parse("1").is_err());
    assert!(Arity11::parse("2").is_err());
    assert!(Arity11::parse("3a").is_err());

    assert_eq!(Arity1O::parse("1a").unwrap(), Arity1O { f: Some(FieldA) });
    assert_eq!(Arity1O::parse("2a").unwrap(), Arity1O { f: Some(FieldA) });
    assert_eq!(Arity1O::parse("2").unwrap(), Arity1O { f: None });
    assert!(Arity1O::parse("1").is_err());
    assert!(Arity1O::parse("3a").is_err());

    assert_eq!(Arity1M::parse("1a").unwrap(), Arity1M { f: vec![FieldA] });
    assert_eq!(Arity1M::parse("2a").unwrap(), Arity1M { f: vec![FieldA] });
    assert_eq!(
        Arity1M::parse("2aaa").unwrap(),
        Arity1M {
            f: vec![FieldA, FieldA, FieldA]
        }
    );
    assert_eq!(Arity1M::parse("2").unwrap(), Arity1M { f: vec![] });
    assert!(Arity1M::parse("1").is_err());
    assert!(Arity1M::parse("3a").is_err());

    assert_eq!(Arity1X::parse("1a").unwrap(), Arity1X { f: Some(FieldA) });
    assert_eq!(Arity1X::parse("2a").unwrap(), Arity1X { f: None });
    assert_eq!(Arity1X::parse("2").unwrap(), Arity1X { f: None });
    assert!(Arity1X::parse("1").is_err());
    assert!(Arity1X::parse("3a").is_err());
}

#[test]
fn test_arities_o() {
    assert_eq!(ArityO1::parse("1a").unwrap(), ArityO1 { f: Some(FieldA) });
    assert_eq!(ArityO1::parse("1").unwrap(), ArityO1 { f: None });
    assert_eq!(ArityO1::parse("2a").unwrap(), ArityO1 { f: Some(FieldA) });
    assert!(ArityO1::parse("2").is_err());
    assert!(ArityO1::parse("3a").is_err());

    assert_eq!(ArityOO::parse("1a").unwrap(), ArityOO { f: Some(FieldA) });
    assert_eq!(ArityOO::parse("1").unwrap(), ArityOO { f: None });
    assert_eq!(ArityOO::parse("2a").unwrap(), ArityOO { f: Some(FieldA) });
    assert_eq!(ArityOO::parse("2").unwrap(), ArityOO { f: None });
    assert!(ArityOO::parse("3a").is_err());

    assert_eq!(ArityOM::parse("1a").unwrap(), ArityOM { f: vec![FieldA] });
    // Not an error, because no $ at end of grammar
    assert_eq!(ArityOM::parse("1aaa").unwrap(), ArityOM { f: vec![FieldA] });
    assert_eq!(ArityOM::parse("1").unwrap(), ArityOM { f: vec![] });
    assert_eq!(ArityOM::parse("2a").unwrap(), ArityOM { f: vec![FieldA] });
    assert_eq!(
        ArityOM::parse("2aaa").unwrap(),
        ArityOM {
            f: vec![FieldA, FieldA, FieldA]
        }
    );
    assert_eq!(ArityOM::parse("2").unwrap(), ArityOM { f: vec![] });
    assert!(ArityOM::parse("3a").is_err());

    assert_eq!(ArityOX::parse("1a").unwrap(), ArityOX { f: Some(FieldA) });
    assert_eq!(ArityOX::parse("1").unwrap(), ArityOX { f: None });
    assert_eq!(ArityOX::parse("2a").unwrap(), ArityOX { f: None });
    assert_eq!(ArityOX::parse("2").unwrap(), ArityOX { f: None });
    assert!(ArityOX::parse("3a").is_err());
}

#[test]
fn test_arities_m() {
    assert_eq!(ArityM1::parse("1a").unwrap(), ArityM1 { f: vec![FieldA] });
    assert_eq!(ArityM1::parse("1").unwrap(), ArityM1 { f: vec![] });
    assert_eq!(
        ArityM1::parse("1aaa").unwrap(),
        ArityM1 {
            f: vec![FieldA, FieldA, FieldA]
        }
    );
    assert_eq!(ArityM1::parse("2a").unwrap(), ArityM1 { f: vec![FieldA] });
    assert!(ArityM1::parse("2").is_err());
    assert!(ArityM1::parse("3a").is_err());

    assert_eq!(ArityMO::parse("1a").unwrap(), ArityMO { f: vec![FieldA] });
    assert_eq!(ArityMO::parse("1").unwrap(), ArityMO { f: vec![] });
    assert_eq!(
        ArityMO::parse("1aaa").unwrap(),
        ArityMO {
            f: vec![FieldA, FieldA, FieldA]
        }
    );
    assert_eq!(ArityMO::parse("2a").unwrap(), ArityMO { f: vec![FieldA] });
    // Not an error, because no $ at end of grammar
    assert_eq!(ArityMO::parse("2aaa").unwrap(), ArityMO { f: vec![FieldA] });
    assert_eq!(ArityMO::parse("2").unwrap(), ArityMO { f: vec![] });
    assert!(ArityMO::parse("3a").is_err());

    assert_eq!(ArityMM::parse("1a").unwrap(), ArityMM { f: vec![FieldA] });
    assert_eq!(
        ArityMM::parse("1aaa").unwrap(),
        ArityMM {
            f: vec![FieldA, FieldA, FieldA]
        }
    );
    assert_eq!(ArityMM::parse("1").unwrap(), ArityMM { f: vec![] });
    assert_eq!(ArityMM::parse("2a").unwrap(), ArityMM { f: vec![FieldA] });
    assert_eq!(
        ArityMM::parse("2aaa").unwrap(),
        ArityMM {
            f: vec![FieldA, FieldA, FieldA]
        }
    );
    assert_eq!(ArityMM::parse("2").unwrap(), ArityMM { f: vec![] });
    assert!(ArityMM::parse("3a").is_err());

    assert_eq!(ArityMX::parse("1a").unwrap(), ArityMX { f: vec![FieldA] });
    assert_eq!(ArityMX::parse("1").unwrap(), ArityMX { f: vec![] });
    assert_eq!(
        ArityMX::parse("1aaa").unwrap(),
        ArityMX {
            f: vec![FieldA, FieldA, FieldA]
        }
    );
    // Not an error, because no $ at end of grammar
    assert_eq!(ArityMX::parse("2a").unwrap(), ArityMX { f: vec![] });
    assert_eq!(ArityMX::parse("2aaa").unwrap(), ArityMX { f: vec![] });
    assert_eq!(ArityMX::parse("2").unwrap(), ArityMX { f: vec![] });
    assert!(ArityMX::parse("3a").is_err());
}

#[test]
fn test_arities_x() {
    assert_eq!(ArityX1::parse("1a").unwrap(), ArityX1 { f: None });
    assert_eq!(ArityX1::parse("1").unwrap(), ArityX1 { f: None });
    assert_eq!(ArityX1::parse("2a").unwrap(), ArityX1 { f: Some(FieldA) });
    assert!(ArityX1::parse("2").is_err());
    assert!(ArityX1::parse("3a").is_err());

    assert_eq!(ArityXO::parse("1a").unwrap(), ArityXO { f: None });
    assert_eq!(ArityXO::parse("1").unwrap(), ArityXO { f: None });
    assert_eq!(ArityXO::parse("2a").unwrap(), ArityXO { f: Some(FieldA) });
    assert_eq!(ArityXO::parse("2").unwrap(), ArityXO { f: None });
    assert!(ArityXO::parse("3a").is_err());

    assert_eq!(ArityXM::parse("1a").unwrap(), ArityXM { f: vec![] });
    // Not an error, because no $ at end of grammar
    assert_eq!(ArityXM::parse("1aaa").unwrap(), ArityXM { f: vec![] });
    assert_eq!(ArityXM::parse("1").unwrap(), ArityXM { f: vec![] });
    assert_eq!(ArityXM::parse("2a").unwrap(), ArityXM { f: vec![FieldA] });
    assert_eq!(
        ArityXM::parse("2aaa").unwrap(),
        ArityXM {
            f: vec![FieldA, FieldA, FieldA]
        }
    );
    assert_eq!(ArityXM::parse("2").unwrap(), ArityXM { f: vec![] });
    assert!(ArityXM::parse("3a").is_err());

    assert_eq!(ArityXX::parse("1a").unwrap(), ArityXX {});
    assert_eq!(ArityXX::parse("1").unwrap(), ArityXX {});
    assert_eq!(ArityXX::parse("2a").unwrap(), ArityXX {});
    assert_eq!(ArityXX::parse("2").unwrap(), ArityXX {});
    assert!(ArityXX::parse("3a").is_err());
}

#[test]
fn test_defaults() {
    assert_eq!(DefaultO::parse("1").unwrap(), DefaultO { f: None });
    assert_eq!(DefaultO::parse("1a").unwrap(), DefaultO { f: Some(FieldA) });
    assert_eq!(DefaultO::parse("2").unwrap(), DefaultO { f: None });

    assert_eq!(DefaultM::parse("1").unwrap(), DefaultM { f: vec![] });
    assert_eq!(
        DefaultM::parse("1aaa").unwrap(),
        DefaultM {
            f: vec![FieldA, FieldA, FieldA]
        }
    );
    assert_eq!(DefaultM::parse("2").unwrap(), DefaultM { f: vec![] });
}

#[test]
fn test_multiple_fields() {
    // MultipleFields = f1:FieldA '1' f2:FieldB | f2:FieldA '2' f3:FieldB;
    assert_eq!(
        MultipleFields::parse("a1b").unwrap(),
        MultipleFields {
            f1: Some(FieldA),
            f2: MultipleFields_f2::FieldB(FieldB),
            f3: None
        }
    );
    assert_eq!(
        MultipleFields::parse("a2b").unwrap(),
        MultipleFields {
            f1: None,
            f2: MultipleFields_f2::FieldA(FieldA),
            f3: Some(FieldB)
        }
    );
}
