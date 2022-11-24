// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

mod grammar;
use grammar::*;
use peginator_runtime::PegParser;

#[test]
fn test() {
    assert_eq!(
        Test::parse("r###\"éáűú\"## \"###other").unwrap(),
        Test {
            a: "éáűú\"## ".into(),
            b: TheStruct { s: "other".into() }
        }
    );
}

pub fn raw_string_parser(s: &str) -> Result<(&str, usize), &'static str> {
    let sb = s.as_bytes();
    if *sb.get(0).ok_or("expected 'r', found end of string")? != b'r' {
        return Err("expected 'r'");
    }
    let num_hashes = sb[1..]
        .iter()
        .position(|c| *c != b'#')
        .ok_or("expected '#' or '\"', found end of string ")?;
    let hashes = &sb[1..1 + num_hashes];
    if *sb
        .get(1 + num_hashes)
        .ok_or("expected '\"', found end of string")?
        != b'"'
    {
        return Err("expected '\"'");
    }
    for i in num_hashes + 2..sb.len() {
        if sb[i] == b'"' && sb[i + 1..].starts_with(hashes) {
            return Ok((&s[num_hashes + 2..i], i + 1 + num_hashes));
        }
    }
    Err("could not find end of raw string marker")
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheStruct {
    s: String,
}

pub fn struct_maker(s: &str) -> Result<(TheStruct, usize), &'static str> {
    Ok((TheStruct { s: s.into() }, s.len()))
}
