// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

use peginator::PegParser;
use peginator_macro::peginate;

peginate!("@export Simple = c:char;");

#[test]
fn test_macro() {
    let s: Simple = Simple::parse("xyz").unwrap();
    assert_eq!(s.c, 'x');
}
