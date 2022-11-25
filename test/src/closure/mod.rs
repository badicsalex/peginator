// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

mod grammar;
use grammar::*;
use peginator_runtime::PegParser;

#[test]
fn test_simple() {
    assert_eq!(Simple::parse("").unwrap(), Simple { f: vec![] });
    assert_eq!(Simple::parse("a").unwrap(), Simple { f: vec![FieldA] });
    assert_eq!(
        Simple::parse("aaa").unwrap(),
        Simple {
            f: vec![FieldA, FieldA, FieldA]
        }
    );

    assert!(SimplePlus::parse("").is_err());
    assert_eq!(
        SimplePlus::parse("a").unwrap(),
        SimplePlus { f: vec![FieldA] }
    );
    assert_eq!(
        SimplePlus::parse("aaa").unwrap(),
        SimplePlus {
            f: vec![FieldA, FieldA, FieldA]
        }
    );
}

#[test]
fn test_arities() {
    assert_eq!(Arity1::parse("").unwrap(), Arity1 { f: vec![] });
    assert_eq!(Arity1::parse(".").unwrap(), Arity1 { f: vec![] });
    assert_eq!(Arity1::parse(".a").unwrap(), Arity1 { f: vec![FieldA] });
    assert_eq!(Arity1::parse(".aa").unwrap(), Arity1 { f: vec![FieldA] });
    assert_eq!(Arity1::parse(".a..a").unwrap(), Arity1 { f: vec![FieldA] });
    assert_eq!(
        Arity1::parse(".a.a.a").unwrap(),
        Arity1 {
            f: vec![FieldA, FieldA, FieldA]
        }
    );
    assert_eq!(
        Arity1::parse(".a.aaa.a").unwrap(),
        Arity1 {
            f: vec![FieldA, FieldA]
        }
    );

    assert_eq!(ArityO::parse("").unwrap(), ArityO { f: vec![] });
    assert_eq!(ArityO::parse(".").unwrap(), ArityO { f: vec![] });
    assert_eq!(ArityO::parse(".a").unwrap(), ArityO { f: vec![FieldA] });
    assert_eq!(ArityO::parse(".aa").unwrap(), ArityO { f: vec![FieldA] });
    assert_eq!(
        ArityO::parse(".a..a").unwrap(),
        ArityO {
            f: vec![FieldA, FieldA]
        }
    );
    assert_eq!(
        ArityO::parse(".a.a.a").unwrap(),
        ArityO {
            f: vec![FieldA, FieldA, FieldA]
        }
    );
    assert_eq!(
        ArityO::parse(".a.aaa.a").unwrap(),
        ArityO {
            f: vec![FieldA, FieldA]
        }
    );
    assert_eq!(ArityM::parse("").unwrap(), ArityM { f: vec![] });
    assert_eq!(ArityM::parse(".").unwrap(), ArityM { f: vec![] });
    assert_eq!(ArityM::parse(".a").unwrap(), ArityM { f: vec![FieldA] });
    assert_eq!(
        ArityM::parse(".aa").unwrap(),
        ArityM {
            f: vec![FieldA, FieldA]
        }
    );
    assert_eq!(
        ArityM::parse(".a..a").unwrap(),
        ArityM {
            f: vec![FieldA, FieldA]
        }
    );
    assert_eq!(
        ArityM::parse(".a.a.a").unwrap(),
        ArityM {
            f: vec![FieldA, FieldA, FieldA]
        }
    );
    assert_eq!(
        ArityM::parse(".a.aaa.a").unwrap(),
        ArityM {
            f: vec![FieldA, FieldA, FieldA, FieldA, FieldA]
        }
    );

    assert_eq!(ArityX::parse("").unwrap(), ArityX {});
    assert_eq!(ArityX::parse(".").unwrap(), ArityX {});
    assert_eq!(ArityX::parse("...").unwrap(), ArityX {});
}

#[test]
fn test_arities_plus() {
    assert!(Arity1P::parse("").is_err());
    assert!(Arity1P::parse(".").is_err());
    assert_eq!(Arity1P::parse(".a").unwrap(), Arity1P { f: vec![FieldA] });
    assert_eq!(Arity1P::parse(".aa").unwrap(), Arity1P { f: vec![FieldA] });
    assert_eq!(
        Arity1P::parse(".a..a").unwrap(),
        Arity1P { f: vec![FieldA] }
    );
    assert_eq!(
        Arity1P::parse(".a.a.a").unwrap(),
        Arity1P {
            f: vec![FieldA, FieldA, FieldA]
        }
    );
    assert_eq!(
        Arity1P::parse(".a.aaa.a").unwrap(),
        Arity1P {
            f: vec![FieldA, FieldA]
        }
    );

    assert!(ArityOP::parse("").is_err());
    assert_eq!(ArityOP::parse(".").unwrap(), ArityOP { f: vec![] });
    assert_eq!(ArityOP::parse(".a").unwrap(), ArityOP { f: vec![FieldA] });
    assert_eq!(ArityOP::parse(".aa").unwrap(), ArityOP { f: vec![FieldA] });
    assert_eq!(
        ArityOP::parse(".a..a").unwrap(),
        ArityOP {
            f: vec![FieldA, FieldA]
        }
    );
    assert_eq!(
        ArityOP::parse(".a.a.a").unwrap(),
        ArityOP {
            f: vec![FieldA, FieldA, FieldA]
        }
    );
    assert_eq!(
        ArityOP::parse(".a.aaa.a").unwrap(),
        ArityOP {
            f: vec![FieldA, FieldA]
        }
    );

    assert!(ArityMP::parse("").is_err());
    assert_eq!(ArityMP::parse(".").unwrap(), ArityMP { f: vec![] });
    assert_eq!(ArityMP::parse(".a").unwrap(), ArityMP { f: vec![FieldA] });
    assert_eq!(
        ArityMP::parse(".aa").unwrap(),
        ArityMP {
            f: vec![FieldA, FieldA]
        }
    );
    assert_eq!(
        ArityMP::parse(".a..a").unwrap(),
        ArityMP {
            f: vec![FieldA, FieldA]
        }
    );
    assert_eq!(
        ArityMP::parse(".a.a.a").unwrap(),
        ArityMP {
            f: vec![FieldA, FieldA, FieldA]
        }
    );
    assert_eq!(
        ArityMP::parse(".a.aaa.a").unwrap(),
        ArityMP {
            f: vec![FieldA, FieldA, FieldA, FieldA, FieldA]
        }
    );

    assert!(ArityXP::parse("").is_err());
    assert_eq!(ArityXP::parse(".").unwrap(), ArityXP {});
    assert_eq!(ArityXP::parse("...").unwrap(), ArityXP {});
}

#[test]
fn test_multi_field() {
    //MultiField = {f1:FieldA f2:FieldB [f3:FieldC] {f4:FieldD}};
    assert_eq!(
        MultiField::parse("").unwrap(),
        MultiField {
            f1: vec![],
            f2: vec![],
            f3: vec![],
            f4: vec![]
        }
    );
    assert_eq!(
        MultiField::parse("ab").unwrap(),
        MultiField {
            f1: vec![FieldA],
            f2: vec![FieldB],
            f3: vec![],
            f4: vec![]
        }
    );
    assert_eq!(
        MultiField::parse("abab").unwrap(),
        MultiField {
            f1: vec![FieldA, FieldA],
            f2: vec![FieldB, FieldB],
            f3: vec![],
            f4: vec![]
        }
    );
    assert_eq!(
        MultiField::parse("abcddd").unwrap(),
        MultiField {
            f1: vec![FieldA],
            f2: vec![FieldB],
            f3: vec![FieldC],
            f4: vec![FieldD, FieldD, FieldD]
        }
    );
    assert_eq!(
        MultiField::parse("abcddabddabccabc").unwrap(),
        MultiField {
            f1: vec![FieldA, FieldA, FieldA],
            f2: vec![FieldB, FieldB, FieldB],
            f3: vec![FieldC, FieldC],
            f4: vec![FieldD, FieldD, FieldD, FieldD]
        }
    );
}

#[test]
fn test_multi_part() {
    assert_eq!(
        MultiPart::parse("ac").unwrap(),
        MultiPart {
            f1: Some(FieldA),
            f2: None,
            f3: FieldC
        }
    );
}
