// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

mod grammar;
use grammar::*;
use peginator::PegParser;

#[test]
fn test() {
    assert!(Simple::parse("").is_err());
    assert!(Simple::parse("hmm").is_ok());
    assert!(Simple::parse("   hmm").is_ok());
    assert!(Simple::parse("   hmm   ").is_ok());
    assert!(Simple::parse("hmm   ").is_ok());
    assert!(Simple::parse("hmmhmm").is_err());

    assert!(Choice::parse("").is_err());
    assert!(Choice::parse("hmm").is_ok());
    assert!(Choice::parse("   hmm   ").is_ok());
    assert!(Choice::parse("hmmhmm").is_err());
    assert!(Choice::parse("thonk").is_ok());
    assert!(Choice::parse("thonkhmm").is_ok());

    assert_eq!(Optional::parse("well").unwrap(), Optional { f: None });
    assert_eq!(
        Optional::parse("well hmm").unwrap(),
        Optional { f: Some(Hmm) }
    );
    assert_eq!(
        Optional::parse("well hmm hmm").unwrap(),
        Optional { f: None }
    );

    assert!(NoSkipWs::parse("").is_err());
    assert!(NoSkipWs::parse("hmm").is_ok());
    assert!(NoSkipWs::parse("   hmm   ").is_err());
    assert!(NoSkipWs::parse("hmm   ").is_err());
    assert!(NoSkipWs::parse("hmmhmm").is_err());
}
