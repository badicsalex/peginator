// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

mod grammar;
use grammar::*;
use peginator::PegParser;

#[test]
fn test_positions() {
    let s = Root::parse("nice f:abcd f:z rulerino and some junk").unwrap();
    assert_eq!(
        s,
        Root {
            field: Field {
                s: Ident { position: 7..11 },
                position: 5..11
            },
            field2: Ident { position: 14..15 },
            position: 0..24
        }
    );
}
