use std::{ffi::OsStr, fs, path::PathBuf, process::Command};

use anyhow::Result;

use crate::{CodegenGrammar, Grammar, PegParser};

#[derive(Debug, Default)]
#[must_use]
pub struct Compile {
    source_path: PathBuf,
    destination_path: Option<PathBuf>,
    format: bool,
    recursive: bool,
}

impl Compile {
    pub fn directory<T: Into<PathBuf>>(filename: T) -> Self {
        Compile {
            source_path: filename.into(),
            recursive: true,
            ..Default::default()
        }
    }

    pub fn file<T: Into<PathBuf>>(filename: T) -> Self {
        Compile {
            source_path: filename.into(),
            ..Default::default()
        }
    }

    pub fn destination<T: Into<PathBuf>>(self, filename: T) -> Self {
        Compile {
            destination_path: Some(filename.into()),
            ..self
        }
    }

    pub fn format(self) -> Self {
        Compile {
            format: true,
            ..self
        }
    }

    fn run_on_single_file(source: &PathBuf, destination: &PathBuf, format: bool) -> Result<()> {
        let grammar = fs::read_to_string(source)?;
        let parsed_grammar = Grammar::parse(&grammar)?;
        let generated_code = parsed_grammar.generate_code(&Default::default())?;
        fs::write(destination, generated_code.to_string())?;
        if format {
            Command::new("rustfmt").arg(destination).status()?;
        }
        Ok(())
    }

    fn run_recursively(source: &PathBuf, format: bool) -> Result<()> {
        if source.is_dir() {
            source
                .read_dir()?
                .try_for_each(|c| Self::run_recursively(&c?.path(), format))
        } else if source.extension().and_then(OsStr::to_str) == Some("ebnf") {
            Self::run_on_single_file(source, &source.with_extension("rs"), format)
        } else {
            Ok(())
        }
    }

    pub fn run(self) -> Result<()> {
        if self.recursive {
            Self::run_recursively(&self.source_path, self.format)
        } else {
            Self::run_on_single_file(
                &self.source_path,
                &self
                    .destination_path
                    .unwrap_or_else(|| self.source_path.with_extension("rs")),
                self.format,
            )
        }
    }
}
