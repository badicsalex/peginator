// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

#[allow(non_camel_case_types)]
mod grammar;
use grammar::*;
use peginator::PegParser;

#[test]
fn test_macro() {
    let s = Simple::parse("type").unwrap();
    assert_eq!(s.r#type, Some(r#type));
}
