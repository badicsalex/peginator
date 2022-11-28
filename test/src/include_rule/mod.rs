// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

mod grammar;
use grammar::*;
use peginator::PegParser;

#[test]
fn test() {
    assert_eq!(
        Test::parse("a(123,456)").unwrap(),
        Test {
            a: Field,
            x: Number,
            y: Number
        }
    );
    assert_eq!(
        MultiTest::parse("a(123,456)(123,456)[123,456][123,456]").unwrap(),
        MultiTest {
            a: Field,
            x: vec![Number, Number],
            y: vec![Number, Number, Number, Number],
            z: vec![Number, Number]
        }
    );
    assert_eq!(
        OverrideTest::parse("123").unwrap(),
        OverrideTest {
            o: Overridden::Number(Number)
        }
    );
}
