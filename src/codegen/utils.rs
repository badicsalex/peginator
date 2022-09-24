// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

use proc_macro2::Ident;
use quote::format_ident;

pub fn safe_ident(name: &str) -> Ident {
    for keyword in RUST_KEYWORDS {
        if name == keyword {
            return format_ident!("r#{}", name);
        }
    }
    format_ident!("{}", name)
}

/// https://doc.rust-lang.org/reference/keywords.html
pub const RUST_KEYWORDS: [&'static str; 50] = [
    // "crate" can't be r#crate
    "as", "break", "const", "continue", "else", "enum", "extern", "false", "fn", "for", "if",
    "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return", "self",
    "Self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use", "where",
    "while", "async", "await", "dyn", "abstract", "become", "box", "do", "final", "macro",
    "override", "priv", "typeof", "unsized", "virtual", "yield", "try",
];
