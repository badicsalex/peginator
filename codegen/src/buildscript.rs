// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

//! Buildscript helpers for peginator
//!
//! See [`Compile`] for examples.

use std::{
    ffi::OsStr,
    fs::{self, File},
    io::Read,
    path::PathBuf,
    process::Command,
    str::FromStr,
};

use anyhow::Result;
use colored::*;
use peginator::PrettyParseError;

use crate::{generate_source_header, grammar::Grammar, CodegenGrammar, CodegenSettings};

/// Compiles peginator grammars into rust code with a builder interface.
///
/// It only recompiles files if it detects (based on the generated file header in the `.rs` file)
/// change in either the peginator library, or the grammar file.
///
/// It is meant to be used as `peginator_codegen::Compile`, hence the generic name.
///
/// Example `build.rs` for using a single grammar file and putting the result in the target directory:
/// ```no_run
///# #[allow(clippy::needless_doctest_main)]
/// fn main() {
///     peginator_codegen::Compile::file("my_grammar.ebnf")
///        .destination(format!("{}/my_grammar.rs", std::env::var("OUT_DIR").unwrap()))
///        .format()
///        .run_exit_on_error();
///     println!("cargo:rerun-if-changed=my_grammar.ebnf");
/// }
/// ```
///
/// Importing this grammar:
/// ```ignore
/// mod grammar { include!(concat!(env!("OUT_DIR"), "/my_grammar.rs")); }
/// ```
///
/// Example `build.rs` for multiple grammar files in the src directory, putting compiled files next to
/// their grammar definitions:
/// ```no_run
///# #[allow(clippy::needless_doctest_main)]
/// fn main() {
///     peginator_codegen::Compile::directory("src")
///        .format()
///        .run_exit_on_error()
/// }
/// ```
///
#[derive(Debug)]
#[must_use]
pub struct Compile {
    source_path: PathBuf,
    destination_path: Option<PathBuf>,
    format: bool,
    recursive: bool,
    settings: CodegenSettings,
    prefix: String,
}

impl Compile {
    fn default() -> Self {
        Compile {
            source_path: PathBuf::new(),
            destination_path: None,
            format: false,
            recursive: false,
            settings: Default::default(),
            prefix: String::new(),
        }
    }
    /// Run compilation on a whole directory run.
    ///
    /// The whole directory is recursively searched for files with the `.ebnf` extension, and
    /// compiled to rust code with the same filename but with `.rs` extension.
    pub fn directory<T: Into<PathBuf>>(filename: T) -> Self {
        Compile {
            source_path: filename.into(),
            recursive: true,
            ..Self::default()
        }
    }

    /// Run compilation on a single file.
    ///
    /// The file will be compiled. If destination is not given, it will be the same filename in the
    /// same directory, but with `.rs` extension.
    pub fn file<T: Into<PathBuf>>(filename: T) -> Self {
        Compile {
            source_path: filename.into(),
            ..Self::default()
        }
    }

    /// Destination file name.
    ///
    /// This option only used if running on a single file.
    pub fn destination<T: Into<PathBuf>>(self, filename: T) -> Self {
        Compile {
            destination_path: Some(filename.into()),
            ..self
        }
    }

    /// Format .rs files with rustfmt after grammar compilation.
    pub fn format(self) -> Self {
        Compile {
            format: true,
            ..self
        }
    }

    /// Use a specific set of derives when declaring structs.
    ///
    /// The default set is `#[derive(Debug, Clone)]`
    pub fn derives(mut self, derives: Vec<String>) -> Self {
        self.settings.derives = derives;
        self
    }

    /// Set a prefix (code) that will be pasted into the file between the header and the generated code
    ///
    /// Useful for `use` declarations or maybe custom structs.
    pub fn prefix(self, prefix: String) -> Self {
        Compile { prefix, ..self }
    }

    /// Set the type of global.user_defined. Experimental feature, may change without notice.
    pub fn user_defined_type(mut self, user_defined_type: &str) -> Self {
        self.settings.set_user_defined_type(user_defined_type);
        self
    }

    fn run_on_single_file(&self, source: &PathBuf, destination: &PathBuf) -> Result<()> {
        let grammar = fs::read_to_string(source)?;
        let source_header = format!("{}\n{}", generate_source_header(&grammar), self.prefix);
        if let Ok(f) = File::open(destination) {
            let mut existing_header = String::new();
            if f.take(source_header.len() as u64)
                .read_to_string(&mut existing_header)
                .is_ok()
                && source_header == existing_header
            {
                return Ok(());
            }
        };

        let parsed_grammar = Grammar::from_str(&grammar)
            .map_err(|err| PrettyParseError::from_parse_error(&err, &grammar, source.to_str()))?;
        let generated_code = format!(
            "{}\n{}",
            source_header,
            parsed_grammar.generate_code(&self.settings)?
        );
        fs::write(destination, &generated_code)?;
        if self.format {
            Command::new("rustfmt").arg(destination).status()?;
        };
        Ok(())
    }

    fn run_recursively(&self, source: &PathBuf) -> Result<()> {
        if source.is_dir() {
            source
                .read_dir()?
                .try_for_each(|c| self.run_recursively(&c?.path()))
        } else if source.extension().and_then(OsStr::to_str) == Some("ebnf") {
            self.run_on_single_file(source, &source.with_extension("rs"))
        } else {
            Ok(())
        }
    }

    /// Run the compilation, returning an error.
    ///
    /// In case of a parse error, [`PrettyParseError`] is thrown,
    /// which will print a pretty error with `format!` or `print!`.
    pub fn run(self) -> Result<()> {
        if self.recursive {
            self.run_recursively(&self.source_path)
        } else {
            self.run_on_single_file(
                &self.source_path.clone(),
                &self
                    .destination_path
                    .clone()
                    .unwrap_or_else(|| self.source_path.with_extension("rs")),
            )
        }
    }

    /// Run the compilation, and exit with an exit code in case of an error.
    ///
    /// It also makes sure to pretty-print the error, should one occur.
    pub fn run_exit_on_error(self) {
        // I absolutely hate how this is a global, because there is no way to know if it was forced
        // already. Thankfully we are exiting right after this.
        colored::control::set_override(true);
        let result = self.run();
        if let Err(error) = result {
            eprintln!(
                "{red_error}{colon} {error}",
                red_error = "error".red().bold(),
                colon = ":".bold().white(),
                error = error
            );
            std::process::exit(1);
        }
    }
}
