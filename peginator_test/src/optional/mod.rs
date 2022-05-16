// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

mod grammar;
use grammar::*;
use peginator::PegParser;

#[test]
fn test() {
    assert_eq!(
        Tester::parse("").unwrap(),
        Tester {
            simple: None,
            optional: None,
            multi: vec!()
        }
    );
    assert_eq!(
        Tester::parse(",.").unwrap(),
        Tester {
            simple: None,
            optional: None,
            multi: vec!()
        }
    );
    assert_eq!(
        Tester::parse("a,a.aaa").unwrap(),
        Tester {
            simple: Some(FieldA),
            optional: Some(FieldA),
            multi: vec![FieldA, FieldA, FieldA]
        }
    );
    assert_eq!(
        Tester::parse("a,aa.aaa").unwrap(),
        Tester {
            simple: Some(FieldA),
            optional: Some(FieldA),
            multi: vec![],
        }
    );
}
