// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

mod grammar;
use grammar::*;
use peginator::PegParser;

#[test]
fn test_simple() {
    assert!(String1::parse("almafa").is_ok());
    assert!(String1::parse("almafa2").is_ok());
    assert!(String1::parse("blmafa").is_err());
    assert!(String1::parse("almbfa").is_err());
    assert!(String1::parse("almafb").is_err());
    assert!(String1::parse("almafű").is_err());

    assert!(String2::parse("almafa").is_ok());
    assert!(String2::parse("blmafa").is_err());

    assert!(Char1::parse("a").is_ok());
    assert!(Char1::parse("b").is_err());
    assert!(Char1::parse("ű").is_err());

    assert!(Char2::parse("a").is_ok());
    assert!(Char2::parse("b").is_err());
    assert!(Char2::parse("ű").is_err());
}

#[test]
fn test_sensitivity() {
    assert!(String1::parse("AlMaFa").is_ok());
    assert!(String1::parse("alMafa2").is_ok());
    assert!(String1::parse("bLMAfa").is_err());
    assert!(String1::parse("aLMBfa").is_err());
    assert!(String1::parse("aLMAfb").is_err());
    assert!(String1::parse("aLMAfŰ").is_err());

    assert!(String2::parse("alMAFA").is_ok());
    assert!(String2::parse("blMAFA").is_err());

    assert!(Char1::parse("A").is_ok());
    assert!(Char1::parse("B").is_err());
    assert!(Char1::parse("Ű").is_err());

    assert!(Char2::parse("A").is_ok());
    assert!(Char2::parse("B").is_err());
    assert!(Char2::parse("Ű").is_err());
}
