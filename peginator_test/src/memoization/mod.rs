// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

mod grammar;
use grammar::*;
use ntest::timeout;
use peginator::PegParser;

#[test]
#[timeout(1000)]
fn test_memoization() {
    let s = Root::parse(&format!("{:a>100}{:c>100}.", "", ""))
        .unwrap()
        .parsed;
    let expected: Option<Box<Recursive>> =
        (0..101).fold(None, |r, _| Some(Box::new(Recursive { inner: r })));
    assert_eq!(s, *expected.unwrap());
}
