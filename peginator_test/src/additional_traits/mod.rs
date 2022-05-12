// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

mod grammar;
use grammar::*;
use peginator::PegParser;

trait CustomTrait {
    fn numeric(self) -> i32;
}

impl CustomTrait for Tag1 {
    fn numeric(self) -> i32 {
        1
    }
}

impl CustomTrait for Tag2 {
    fn numeric(self) -> i32 {
        2
    }
}

#[test]
fn test_macro() {
    let s = Simple::parse("1").unwrap();
    if let Simple_c::Tag1(parsed) = s.c {
        assert_eq!(parsed.numeric(), 1);
    } else {
        panic!("Wat.");
    }
}
