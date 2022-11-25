// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

mod grammar;
use grammar::*;
use peginator_runtime::{PegParser, PrettyParseError};

const PARSE_ME: &str = "result = (1 - 2 + 3) * (13 - 37 * 4 + 20);";

#[test]
fn test() {
    match Assignment::parse_with_trace(PARSE_ME) {
        Ok(x) => println!("{:#?}", x),
        Err(e) => {
            println!("{}", PrettyParseError::from_parse_error(&e, PARSE_ME, None));
            panic!();
        }
    }
}
