// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

mod grammar;
use grammar::*;
use peginator_runtime::PegParser;

#[test]
fn test_hex() {
    assert_eq!(
        HexNumTester::parse("123ABFZ1").unwrap().num,
        "123ABF".to_string()
    );
    assert_eq!(
        HexNumTester::parse("123 ABFZ1").unwrap().num,
        "123".to_string()
    );
    assert!(HexNumTester::parse("X").is_err());
}

#[test]
fn test_weird() {
    assert_eq!(
        WeirdTest::parse(".axz").unwrap(),
        WeirdTest { a: 'a', w: 'x' }
    );
    assert_eq!(
        WeirdTest::parse(".a2z").unwrap(),
        WeirdTest { a: 'a', w: '2' }
    );
    assert_eq!(
        WeirdTest::parse(".aaz").unwrap(),
        WeirdTest { a: 'a', w: 'a' }
    );
    assert_eq!(
        WeirdTest::parse(".a✀z").unwrap(),
        WeirdTest { a: 'a', w: '✀' }
    );
    assert!(WeirdTest::parse(".b✀z").is_err());
    assert!(WeirdTest::parse(".a0z").is_err());
    assert!(WeirdTest::parse(".a").is_err());
    assert!(WeirdTest::parse(".a ✀z").is_err());
    assert!(WeirdTest::parse(". a✀z").is_err());
}
