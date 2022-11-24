// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

mod grammar;
use grammar::*;
use peginator_runtime::{PegParser, PrettyParseError};

const PARSE_ME: &str = "fn example(&self, input:&str, rectified:&mut Rect) -> ExampleResult;";

#[test]
fn test() {
    match FunctionDef::parse_with_trace(PARSE_ME) {
        Ok(x) => println!("{:#?}", x),
        Err(e) => {
            println!("{}", PrettyParseError::from_parse_error(&e, PARSE_ME, None));
            panic!();
        }
    }
}
