// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

mod grammar;
use grammar::*;
use peginator::PegParser;

#[test]
fn test() {
    let s = Simple::parse("a").unwrap();
    assert_eq!(s.a, Field);
}
