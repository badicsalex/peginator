// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

mod grammar;
use grammar::*;
use peginator::PegParser;

#[test]
fn test_skips() {
    assert_eq!(AnyButB::parse("a").unwrap().c, 'a');
    assert!(AnyButB::parse("b").is_err());

    assert_eq!(AnyButASDF::parse("x").unwrap().c, 'x');
    assert!(AnyButASDF::parse("s").is_err());
}

fn split_last(s: &str) -> (String, String) {
    let result = SplitLast::parse_with_trace(s).unwrap();
    (
        result.list_rest.iter().collect(),
        result.list_last.iter().collect(),
    )
}

#[test]
fn test_split_last() {
    assert_eq!(
        split_last("abc, def, ghi"),
        ("abc, def".to_string(), " ghi".to_string())
    );
    assert_eq!(split_last(",,,,"), (",,,".to_string(), "".to_string()));
    assert!(SplitLast::parse_with_trace("asd").is_err());
}

#[test]
fn test_list() {
    let result = List::parse("1*2, 2*3!").unwrap();
    assert_eq!(
        result,
        List {
            parts: vec![ListPart { a: '1', b: '2' }, ListPart { a: '2', b: '3' }]
        }
    );
    assert!(List::parse("1*2, 2*3, 5*6.").is_ok());
    assert!(List::parse("1*2! 2*3!").is_err());
    assert!(List::parse("1*2, 2*3,").is_err());
    assert!(List::parse("1*2,").is_err());
    assert!(List::parse("1*2").is_err());
}

#[test]
fn test_intersection() {
    assert!(Intersection::parse("d").is_err());
    assert!(Intersection::parse("e").is_ok());
    assert!(Intersection::parse("f").is_ok());
    assert!(Intersection::parse("g").is_err());
}
