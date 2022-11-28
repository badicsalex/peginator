// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

mod grammar;
use grammar::*;
use peginator::PegParser;

#[test]
fn test() {
    assert_eq!(Test::parse("ua").unwrap().u.unwrap(), UnitRule);
    assert!(Test::parse("fa").is_err());

    assert_eq!(
        Test::parse("c(123,456)").unwrap().c.unwrap(),
        ComplexRule {
            x: "123".to_string(),
            y: "456".to_string()
        }
    );
    assert!(Test::parse("c(23,456)").is_err());

    assert_eq!(Test::parse("ss123").unwrap().s.unwrap(), "123".to_string());
    assert!(Test::parse("ss23").is_err());

    assert_eq!(
        Test::parse("d0xabc").unwrap().d.unwrap(),
        DualNumber::HexNumber("0xabc".to_string())
    );
    assert!(Test::parse("d0xbc").is_err());

    assert_eq!(Test::parse("lű").unwrap().l.unwrap(), 'ű');
    assert!(Test::parse("lŰ").is_err());
}

pub fn unit_checker(_: &UnitRule) -> bool {
    true
}

pub fn unit_checker_failing(_: &FailingUnitRule) -> bool {
    false
}

pub fn complex_checker(cr: &ComplexRule) -> bool {
    cr.x.contains('1')
}

pub fn override_check(sn: &SmallNumber) -> bool {
    sn.contains('1')
}

pub fn enum_override_check(dn: &DualNumber) -> bool {
    match dn {
        DualNumber::HexNumber(hn) => hn.contains('a'),
        DualNumber::Number(n) => n.contains('1'),
    }
}
