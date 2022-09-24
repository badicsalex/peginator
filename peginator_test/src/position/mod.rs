// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

mod grammar;
use grammar::*;
use peginator::{runtime::PegPosition, PegParser};

#[test]
fn test_positions() {
    let parsed_str = "nice f:abcd f:z rulerino and some junk";
    let s = Root::parse(parsed_str).unwrap();
    assert_eq!(parsed_str[s.field.s.position.clone()], *"abcd");
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
    assert_eq!(*s.position(), 0..24);
    assert_eq!(*s.field2.position(), 14..15);

    let s2 = Specials::parse("123aaaa").unwrap();
    assert_eq!(
        s2,
        Specials {
            n: Number {
                string: "123".into(),
                position: 0..3
            },
            e: EnumField::FieldA(FieldA { position: 3..7 })
        }
    );
    assert_eq!(*s2.e.position(), 3..7);
}
