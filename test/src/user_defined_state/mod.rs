// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

#[allow(clippy::useless_conversion)]
mod grammar;
use grammar::*;
use peginator::{NoopTracer, ParseSettings, PegParserAdvanced};

pub struct TheState {
    retval: u32,
    a_count: u32,
}

impl TheState {
    pub fn do_parse(&mut self, _s: &str) -> Result<(u32, usize), &'static str> {
        self.retval += 1;
        Ok((self.retval, 1))
    }
}

pub fn stateful_parser(s: &str, state: &mut TheState) -> Result<(u32, usize), &'static str> {
    state.do_parse(s)
}
pub fn stateful_checker(_a: &ManyAs, state: &mut TheState) -> bool {
    if state.a_count > 0 {
        state.a_count -= 1;
        true
    } else {
        false
    }
}

#[test]
fn test() {
    let mut state = TheState {
        retval: 42,
        a_count: 4,
    };
    assert_eq!(
        Test::parse_advanced::<NoopTracer>("ab a a a a a a", &ParseSettings::default(), &mut state)
            .unwrap(),
        Test {
            f1: 43,
            f2: 44,
            f3: vec![ManyAs, ManyAs, ManyAs, ManyAs]
        }
    );
    assert_eq!(state.retval, 44);
}
