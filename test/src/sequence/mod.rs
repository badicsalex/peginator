// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

mod grammar;
use grammar::*;
use peginator::PegParser;

#[test]
fn test_merge() {
    assert_eq!(
        MergeSimple::parse("a a a empty a").unwrap(),
        MergeSimple {
            x: FieldA,
            y: FieldA,
            a: FieldA,
            b: FieldA
        }
    );
}

#[test]
fn test_multi_simple() {
    assert_eq!(
        MultiSimple::parse("xxy.yyz,zzzwww").unwrap(),
        MultiSimple {
            x: vec![FieldX, FieldX],
            y: vec![FieldY, FieldY, FieldY],
            z: vec![FieldZ, FieldZ, FieldZ, FieldZ],
            w: vec![FieldW, FieldW, FieldW],
        }
    )
}

#[test]
fn test_multi_opt() {
    assert_eq!(
        MultiOpt::parse("xxy.yyz,zzzwww").unwrap(),
        MultiOpt {
            x: vec![FieldX, FieldX],
            y: vec![FieldY, FieldY, FieldY],
            z: vec![FieldZ, FieldZ, FieldZ, FieldZ],
            w: vec![FieldW, FieldW, FieldW],
        }
    );
    assert_eq!(
        MultiOpt::parse("xxyyzzwww").unwrap(),
        MultiOpt {
            x: vec![FieldX, FieldX],
            y: vec![FieldY, FieldY],
            z: vec![FieldZ, FieldZ],
            w: vec![FieldW, FieldW, FieldW],
        }
    )
}
