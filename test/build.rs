// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

fn main() {
    peginator_codegen::Compile::directory("src")
        .format()
        .derives(vec![
            "Debug".into(),
            "Clone".into(),
            "PartialEq".into(),
            "Eq".into(),
        ])
        .run_exit_on_error();
    peginator_codegen::Compile::file("src/custom_derives_empty/grammar.not_ebnf")
        .format()
        .derives(vec![])
        .prefix("pub struct ImJustHereToConfuse;".into())
        .run_exit_on_error();
    peginator_codegen::Compile::file("src/user_defined_state/grammar.not_ebnf")
        .format()
        .derives(vec![
            "Debug".into(),
            "Clone".into(),
            "PartialEq".into(),
            "Eq".into(),
        ])
        .user_defined_type("crate::user_defined_state::TheState")
        .run_exit_on_error();
}
