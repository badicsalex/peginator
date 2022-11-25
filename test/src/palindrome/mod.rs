// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

mod grammar;
use grammar::*;
use peginator_runtime::PegParser;

fn to_palindrome_struct(s: &str) -> Palindrome {
    let result: Option<Box<Palindrome>> = s.chars().rev().fold(None, |p, c| {
        Some(Box::new(Palindrome {
            letter: match c {
                'a' => Palindrome_letter::A(A),
                _ => Palindrome_letter::B(B),
            },
            inner: p,
        }))
    });
    *result.unwrap()
}

#[test]
fn test_parsing() {
    let s = Palindrome::parse("aabbaaaabbaabbaaabab").unwrap();
    assert_eq!(s, to_palindrome_struct("aabbaa"));
    let s = Palindrome::parse("aabbaaaabbaaaabbaaabab").unwrap();
    assert_eq!(s, to_palindrome_struct("aabbaaaab"));
}
