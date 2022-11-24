// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

mod grammar;
use grammar::*;
use peginator::PegParser;

#[test]
fn test_simple() {
    let s = Simple::parse("1").unwrap();
    assert_eq!(s.a, Simple_a::EnumVal1(EnumVal1));
    let s = Simple::parse("2").unwrap();
    assert_eq!(s.a, Simple_a::EnumVal2(EnumVal2));
    let s = Simple::parse("3").unwrap();
    assert_eq!(s.a, Simple_a::EnumVal3(EnumVal3));
    let s = Simple::parse("4").unwrap();
    assert_eq!(s.a, Simple_a::EnumVal4(EnumVal4));

    assert!(Simple::parse("5").is_err())
}

#[test]
fn test_with_opt() {
    let s = WithOpt::parse("1").unwrap();
    assert_eq!(s.a, Some(WithOpt_a::EnumVal1(EnumVal1)));
    let s = WithOpt::parse("2").unwrap();
    assert_eq!(s.a, Some(WithOpt_a::EnumVal2(EnumVal2)));
    let s = WithOpt::parse("3").unwrap();
    assert_eq!(s.a, Some(WithOpt_a::EnumVal3(EnumVal3)));
    let s = WithOpt::parse("4").unwrap();
    assert_eq!(s.a, Some(WithOpt_a::EnumVal4(EnumVal4)));

    let s = WithOpt::parse("5").unwrap();
    assert_eq!(s.a, None);
}

#[test]
fn test_with_mult() {
    let s = WithMult::parse("1122").unwrap();
    assert_eq!(
        s.a,
        vec![
            WithMult_a::EnumVal1(EnumVal1),
            WithMult_a::EnumVal1(EnumVal1),
            WithMult_a::EnumVal2(EnumVal2),
            WithMult_a::EnumVal2(EnumVal2),
        ]
    );
    let s = WithMult::parse("21").unwrap();
    assert_eq!(
        s.a,
        vec![
            WithMult_a::EnumVal2(EnumVal2),
            WithMult_a::EnumVal1(EnumVal1),
        ]
    );
    let s = WithMult::parse("3").unwrap();
    assert_eq!(s.a, vec![WithMult_a::EnumVal3(EnumVal3)]);

    let s = WithMult::parse("4").unwrap();
    assert_eq!(s.a, vec![WithMult_a::EnumVal4(EnumVal4)]);

    assert!(WithMult::parse("5").is_err())
}

#[test]
fn test_with_mult_opt() {
    let s = WithMultOpt::parse("1122").unwrap();
    assert_eq!(
        s.a,
        vec![
            WithMultOpt_a::EnumVal1(EnumVal1),
            WithMultOpt_a::EnumVal1(EnumVal1),
            WithMultOpt_a::EnumVal2(EnumVal2),
            WithMultOpt_a::EnumVal2(EnumVal2),
        ]
    );
    let s = WithMultOpt::parse("21").unwrap();
    assert_eq!(
        s.a,
        vec![
            WithMultOpt_a::EnumVal2(EnumVal2),
            WithMultOpt_a::EnumVal1(EnumVal1),
        ]
    );
    let s = WithMultOpt::parse("3").unwrap();
    assert_eq!(s.a, vec![WithMultOpt_a::EnumVal3(EnumVal3)]);

    let s = WithMultOpt::parse("4").unwrap();
    assert_eq!(s.a, vec![WithMultOpt_a::EnumVal4(EnumVal4)]);

    let s = WithMultOpt::parse("5").unwrap();
    assert_eq!(s.a, vec![]);
}

#[test]
fn test_one_opt() {
    let s = OneOpt::parse("1").unwrap();
    assert_eq!(s.a, vec![OneOpt_a::EnumVal1(EnumVal1)]);
    let s = OneOpt::parse("12").unwrap();
    assert_eq!(
        s.a,
        vec![OneOpt_a::EnumVal1(EnumVal1), OneOpt_a::EnumVal2(EnumVal2),]
    );

    assert!(OneOpt::parse("2").is_err())
}
