// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

pub trait CallHook<TU, TR> {
    fn call_as_extern(&self, s: &str, user_context: TU) -> Result<(TR, usize), &'static str>;
    fn call_as_check(&self, s: &str, user_context: TU) -> bool;
}

impl<TF, TU, TR> CallHook<TU, TR> for TF
where
    TF: Fn(&str) -> Result<(TR, usize), &'static str>,
{
    fn call_as_extern(&self, s: &str, _user_context: TU) -> Result<(TR, usize), &'static str> {
        self(s)
    }

    fn call_as_check(&self, s: &str, _user_context: TU) -> bool {
        self(s).is_ok()
    }
}

impl<TF, TU, TR> CallHook<TU, TR> for TF
where
    TF: Fn(&str, TU) -> Result<(TR, usize), &'static str>,
{
    fn call_as_extern(&self, s: &str, user_context: TU) -> Result<(TR, usize), &'static str> {
        self(s, user_context)
    }

    fn call_as_check(&self, s: &str, user_context: TU) -> bool {
        self(s, user_context).is_ok()
    }
}
