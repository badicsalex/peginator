// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

mod grammar;
use grammar::*;
use peginator::PegParser;

#[test]
fn test() {
    assert!(Simple::parse("").is_err());
    assert!(Simple::parse("a").is_ok());
    assert!(Simple::parse("b").is_err());
    assert!(Simple::parse("bc").is_ok());
    assert!(Simple::parse("de").is_ok());
    assert!(Simple::parse("df").is_ok());
    assert!(Simple::parse("ace").is_err());
}
