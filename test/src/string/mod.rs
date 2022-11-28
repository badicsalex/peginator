// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

mod grammar;
use grammar::*;
use peginator::PegParser;

#[test]
fn test_simple() {
    assert!(String1::parse("almafa").is_ok());
    assert!(String1::parse("almafa2").is_ok());
    assert!(String1::parse("blmafa").is_err());
    assert!(String1::parse("almbfa").is_err());
    assert!(String1::parse("almafb").is_err());

    assert!(String2::parse("almafa").is_ok());
    assert!(String2::parse("blmafa").is_err());

    assert!(Char1::parse("a").is_ok());
    assert!(Char1::parse("b").is_err());

    assert!(Char2::parse("a").is_ok());
    assert!(Char2::parse("b").is_err());

    assert!(UtfChar::parse("☃").is_ok());
    assert!(UtfChar::parse("a").is_err());
    assert!(UtfChar::parse("☄").is_err());
}

#[test]
fn test_char_range() {
    assert!(CharRange::parse("b").is_ok());
    assert!(CharRange::parse("d").is_ok());
    assert!(CharRange::parse("g").is_ok());
    assert!(CharRange::parse("a").is_err());
    assert!(CharRange::parse("h").is_err());
    assert!(CharRange::parse("á").is_err());

    assert!(UtfRange::parse("←").is_err());
    assert!(UtfRange::parse("↑").is_ok());
    assert!(UtfRange::parse("→").is_ok());
    assert!(UtfRange::parse("↓").is_ok());
    assert!(UtfRange::parse("↔").is_err());
    assert!(UtfRange::parse("a").is_err());

    assert!(HalfRange::parse("a").is_err());
    assert!(HalfRange::parse("w").is_err());
    assert!(HalfRange::parse("x").is_ok());
    assert!(HalfRange::parse("y").is_ok());
    assert!(HalfRange::parse("}").is_ok());
    assert!(HalfRange::parse("á").is_ok());
    assert!(HalfRange::parse("\u{0749}").is_ok());
    assert!(HalfRange::parse("\u{074A}").is_ok());
    assert!(HalfRange::parse("\u{074B}").is_err());
    assert!(HalfRange::parse("☃").is_err());
}

#[test]
fn test_spaces() {
    assert!(Spaces::parse("ab cd ef").is_ok());
    assert!(Spaces::parse("  ab cd ef").is_ok());
    assert!(Spaces::parse("ab cd ef  ").is_ok());

    assert!(Spaces::parse("ab cdef").is_err());
    assert!(Spaces::parse("abcd ef").is_err());
    assert!(Spaces::parse("abcdef").is_err());
    assert!(Spaces::parse("ab c d ef").is_err());
    assert!(Spaces::parse("abc d ef").is_err());
    assert!(Spaces::parse("ab cde f").is_err());
}

#[test]
fn test_escapes() {
    assert!(SimpleEscapes1::parse(".\n\t\r\\'\"").is_ok());
    assert!(SimpleEscapes1::parse(".\n\t\r\\'").is_err());

    assert!(SimpleEscapes2::parse(".\n\t\r\\'\"").is_ok());
    assert!(SimpleEscapes2::parse(".\n\t\r\\'").is_err());

    assert!(HexEscape::parse("aUb").is_ok());
    assert!(HexEscape::parse("aU").is_err());
    assert!(HexEscape::parse("aVb").is_err());

    assert!(UnicodeEscapeSimple::parse("Á0 ﹖0 \u{10abc}d").is_ok());
    assert!(UnicodeEscapeSimple::parse("Á0 ﹖0 \u{10abc}").is_err());
    assert!(UnicodeEscapeSimple::parse("0 ﹖0 \u{10abc}d").is_err());
    assert!(UnicodeEscapeSimple::parse("Á0 ﹖ 0 \u{10abc}d").is_err());

    assert!(UnicodeEscapeRust::parse("(\u{7}\u{7}ÁÁűű‱‱𝐐𝐐\u{101337})").is_ok());
    assert!(UnicodeEscapeRust::parse("(\u{7} \u{7} Á Á ű ű ‱ ‱ 𝐐 𝐐 \u{101337} )").is_ok());
    assert!(UnicodeEscapeRust::parse("(\u{7}\u{7}ÁÁűű‱‱𝐐𝐐\u{101338})").is_err());
    assert!(UnicodeEscapeRust::parse("(\u{7}\u{7}ÁÁŰű‱‱𝐐𝐐\u{101337})").is_err());
    assert!(UnicodeEscapeRust::parse("(\u{6}\u{7}ÁÁűű‱‱𝐐𝐐\u{101337})").is_err());
}

#[test]
fn test_unescaped() {
    assert!(Unescaped::parse("a\n 🫃🏿_").is_ok());
    assert!(Unescaped::parse("a\n 🫃🏿 _").is_err());
    assert!(Unescaped::parse("a\n\t🫃🏿_").is_err());
    assert!(Unescaped::parse("b\n 🫃🏿_").is_err());
    assert!(Unescaped::parse("a\n 🫃🏿>").is_err());
}
