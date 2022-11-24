// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

mod grammar;
use grammar::*;
use peginator_runtime::PegParser;

#[test]
fn test() {
    assert!(Simple::parse("ab").is_ok());
    assert!(Simple::parse("a\nb").is_err());
    assert!(Simple::parse(" a   b ").is_ok());
    assert!(Simple::parse("x a xx b x").is_ok());
    assert!(Simple::parse("# ayy, his is some good stuff\na b # haha\n").is_ok());
    assert!(Simple::parse("# ayy, this is some good stuff\na b # haha").is_err());
}
