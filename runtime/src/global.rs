// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

use super::ParseTracer;

#[derive(Debug, Clone)]
pub struct ParseGlobal<TT: ParseTracer, TC, TUD> {
    pub tracer: TT,
    pub cache: TC,
    pub user_defined: TUD,
}

impl<TT: ParseTracer, TC, TUD> ParseGlobal<TT, TC, TUD> {
    pub fn new(cache: TC, user_defined: TUD) -> Self {
        Self {
            tracer: TT::new(),
            cache,
            user_defined,
        }
    }
}
