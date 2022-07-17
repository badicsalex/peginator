// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

mod grammar;
use std::fmt::Debug;

use grammar::*;
use peginator::PegParser;

impl Clone for Simple {
    fn clone(&self) -> Self {
        Self { a: Field }
    }
}

impl Debug for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Haha, no")
    }
}

impl PartialEq for Field {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

#[test]
fn test() {
    let s = Simple::parse("a").unwrap();
    assert_eq!(s.a, Field);
}
