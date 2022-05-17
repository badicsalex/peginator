// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

fn main() {
    peginator::buildscript::Compile::directory("src")
        .format()
        .use_peginator_build_time()
        .run_exit_on_error()
}
