// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

mod grammar;
use grammar::*;
use peginator::PegParser;

#[test]
fn test_macro() {
    let s = Simple::parse("type").unwrap();
    if let Some(t) = s.r#type {
        assert_eq!(t, r#type)
    } else {
        panic!("Wat.");
    }
}
