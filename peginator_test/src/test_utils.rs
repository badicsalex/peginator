// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

macro_rules! assert_type_eq (
    ($parent:ident, $field:ident, $type:ty) => {
        let _ = |a:$parent| -> $type {a.$field};
    }
);

pub(crate) use assert_type_eq;
