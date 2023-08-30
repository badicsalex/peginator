// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

#[allow(clippy::module_inception)]
mod grammar;
use grammar::*;
use peginator::PegParser;

#[test]
fn test_enums() {
    match EnumTest::parse("a(123;456)").unwrap().a {
        EnumTest_a::Point(parsed) => assert_eq!(
            parsed,
            Point {
                x: "123".to_string(),
                y: "456".to_string()
            }
        ),
        _ => panic!("Invalid parse"),
    }

    match EnumTestComplex::parse("b dontcare: 123 456").unwrap().a {
        EnumTestComplex_a::NoFieldName(parsed) => assert_eq!(
            parsed,
            NoFieldName {
                f: "456".to_string()
            }
        ),
        _ => panic!("Invalid parse"),
    }
}

#[test]
fn test_overrides() {
    let result = OverrideTest::parse("(1;2) (3;4)").unwrap();
    assert_eq!(
        result.s,
        Point {
            x: "1".to_string(),
            y: "2".to_string()
        }
    );
    match result.e {
        EnumOverride::Point(p) => assert_eq!(
            p,
            Point {
                x: "3".to_string(),
                y: "4".to_string()
            }
        ),
        _ => panic!("Invalid parse"),
    }
    assert_eq!(result.o, None);
    assert_eq!(result.v, vec![]);
    let result = OverrideTest::parse("(1;2) (3;4) (5;6) (7;8) (9;10)").unwrap();
    assert_eq!(
        result.o,
        Some(Point {
            x: "5".to_string(),
            y: "6".to_string()
        })
    );
    assert_eq!(
        result.v,
        vec![
            Point {
                x: "7".to_string(),
                y: "8".to_string()
            },
            Point {
                x: "9".to_string(),
                y: "10".to_string()
            }
        ]
    );
}
