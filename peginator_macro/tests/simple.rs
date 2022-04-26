// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

use peginator_macro::peginate;

peginate!("Simple = c:char;");

#[test]
fn test_macro() {
    let s: Simple = parse_Simple("xyz").unwrap();
    assert!(s.c == 'x');
}
