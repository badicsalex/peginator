// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

mod grammar;
use grammar::*;
use peginator::{
    runtime::{NoopTracer, PegParserAdvanced},
    ParseSettings,
};

pub struct TheState {
    a: u32,
}

impl TheState {
    pub fn do_parse(&mut self, _s: &str) -> Result<(u32, usize), &'static str> {
        self.a += 1;
        Ok((self.a, 1))
    }
}

#[test]
fn test() {
    assert_eq!(
        Test::parse_advanced::<NoopTracer>("abc", &ParseSettings::default(), TheState { a: 42 })
            .unwrap(),
        Test { f1: 43, f2: 44 }
    );
}
