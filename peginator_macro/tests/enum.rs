// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

use peginator_macro::peginate;

use peginator::PegParser;

peginate!(
    r##"
@export
Simple = (a:EnumVal1 | a:EnumVal2) | (a:EnumVal3 | a:EnumVal4);

@export
WithOpt = (a:EnumVal1 | a:EnumVal2) | (a:EnumVal3 | a:EnumVal4|);

@export
WithMult = {a:EnumVal1 | a:EnumVal2} | (a:EnumVal3 | a:EnumVal4);

@export
WithOptMult = {a:EnumVal1 | a:EnumVal2} | (a:EnumVal3 | a:EnumVal4|);

EnumVal1 = '1';
EnumVal2 = '2';
EnumVal3 = '3';
EnumVal4 = '4';

"##
);

#[test]
fn test_simple() {
    let s: Simple = Simple::parse("1").unwrap();
    assert!(matches!(s.a, Simple_a::EnumVal1(..)));
    let s: Simple = Simple::parse("2").unwrap();
    assert!(matches!(s.a, Simple_a::EnumVal2(..)));
    let s: Simple = Simple::parse("3").unwrap();
    assert!(matches!(s.a, Simple_a::EnumVal3(..)));
    let s: Simple = Simple::parse("4").unwrap();
    assert!(matches!(s.a, Simple_a::EnumVal4(..)));
}

#[test]
fn test_with_opt() {
    let s: WithOpt = WithOpt::parse("1").unwrap();
    assert!(matches!(s.a, Some(WithOpt_a::EnumVal1(..))));
    let s: WithOpt = WithOpt::parse("2").unwrap();
    assert!(matches!(s.a, Some(WithOpt_a::EnumVal2(..))));
    let s: WithOpt = WithOpt::parse("3").unwrap();
    assert!(matches!(s.a, Some(WithOpt_a::EnumVal3(..))));
    let s: WithOpt = WithOpt::parse("4").unwrap();
    assert!(matches!(s.a, Some(WithOpt_a::EnumVal4(..))));

    let s: WithOpt = WithOpt::parse("5").unwrap();
    assert!(matches!(s.a, None));
}
