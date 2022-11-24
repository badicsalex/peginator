// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

mod grammar;
use grammar::*;
use peginator_runtime::PegParser;

#[test]
fn test_string() {
    assert!(SkipWsString::parse("aaabbb").is_ok());
    assert!(SkipWsString::parse(" \n\taaa  bbb  ").is_ok());

    assert!(NoSkipWsString::parse("aaabbb").is_ok());
    assert!(NoSkipWsString::parse(" \n\taaa  bbb  ").is_err());
    assert!(NoSkipWsString::parse(" aaabbb").is_err());
    assert!(NoSkipWsString::parse("aaa bbb").is_err());
    assert!(NoSkipWsString::parse(" aaa bbb").is_err());
}

#[test]
fn test_char_range() {
    assert!(SkipWsCharRange::parse("ac").is_ok());
    assert!(SkipWsCharRange::parse(" \n\ta  c  ").is_ok());

    assert!(NoSkipWsCharRange::parse("ac").is_ok());
    assert!(NoSkipWsCharRange::parse(" \n\ta  c  ").is_err());
    assert!(NoSkipWsCharRange::parse(" ac").is_err());
    assert!(NoSkipWsCharRange::parse(" a c").is_err());
    assert!(NoSkipWsCharRange::parse("a  c").is_err());
}

#[test]
fn test_eoi() {
    assert!(SkipWsEoi::parse("a").is_ok());
    assert!(SkipWsEoi::parse("  a\n\t ").is_ok());

    assert!(NoSkipWsEoi::parse("a").is_ok());
    assert!(NoSkipWsEoi::parse("  a\n\t ").is_err());
    assert!(NoSkipWsEoi::parse("  a").is_err());
    assert!(NoSkipWsEoi::parse("a ").is_err());
}

#[test]
fn test_field() {
    assert!(SkipWsField::parse("aaabbbac").is_ok());
    assert!(SkipWsField::parse("  aaabbb  ac ").is_ok());
    assert!(SkipWsField::parse("  aaa bbb  ac ").is_err());
    assert!(SkipWsField::parse("  aaabbb  a c ").is_err());

    assert!(NoSkipWsField::parse("aaabbbac").is_ok());
    assert!(NoSkipWsField::parse("  aaabbb  ac ").is_err());
    assert!(NoSkipWsField::parse("aaabbb  ac").is_err());
    assert!(NoSkipWsField::parse(" aaabbbac").is_err());
    assert!(NoSkipWsField::parse("aaa bbbac").is_err());
    assert!(NoSkipWsField::parse("aaabbba c").is_err());
}

#[test]
fn test_override_field() {
    assert!(OverrideTester::parse("aaabbbaaabbbaaabbb").is_ok());
    assert!(OverrideTester::parse(" aaabbbaaabbbaaabbb").is_ok());
    assert!(OverrideTester::parse("aaa bbbaaabbbaaabbb").is_err());
    assert!(OverrideTester::parse("aaabbb aaabbbaaabbb").is_err());
    assert!(OverrideTester::parse("aaabbbaaa bbbaaabbb").is_err());
    assert!(OverrideTester::parse("aaabbbaaabbb aaabbb").is_ok());
    assert!(OverrideTester::parse("aaabbbaaabbbaaa bbb").is_err());

    assert!(OverrideTester::parse("acacac").is_ok());
    assert!(OverrideTester::parse(" acacac").is_ok());
    assert!(OverrideTester::parse("a cacac").is_err());
    assert!(OverrideTester::parse("ac acac").is_err());
    assert!(OverrideTester::parse("aca cac").is_err());
    assert!(OverrideTester::parse("acac ac").is_ok());
    assert!(OverrideTester::parse("acaca c").is_err());
}

#[test]
fn test_propagation() {
    assert!(SkipWsPropagation::parse("a.bbbc").is_ok());
    assert!(SkipWsPropagation::parse(" a . b b b c ").is_ok());

    assert!(NoSkipWsPropagation::parse("a.bbbc").is_ok());
    assert!(NoSkipWsPropagation::parse(" a.bbbc").is_err());
    assert!(NoSkipWsPropagation::parse("a .bbbc").is_err());
    assert!(NoSkipWsPropagation::parse("a. bbbc").is_err());
    assert!(NoSkipWsPropagation::parse("a.b bbc").is_err());
    assert!(NoSkipWsPropagation::parse("a.bb bc").is_err());
    assert!(NoSkipWsPropagation::parse("a.bbb c").is_err());
}

#[test]
fn test_opt_back() {
    assert!(OptBack::parse("abc").is_ok());
    assert!(OptBack::parse("a  \nbc").is_ok());
    assert!(OptBack::parse("ab c").is_err());
    assert!(OptBack::parse(" abc").is_err());
}
