use std::{
    ffi::OsStr,
    fs::{self, File},
    io::Read,
    path::PathBuf,
    process::Command,
};

use anyhow::Result;
use colored::*;

use crate::{generate_source_header, CodegenGrammar, Grammar, PegParser, PrettyParseError};

#[derive(Debug, Default)]
#[must_use]
pub struct Compile {
    source_path: PathBuf,
    destination_path: Option<PathBuf>,
    format: bool,
    recursive: bool,
    use_peginator_build_time: bool,
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

    pub fn use_peginator_build_time(self) -> Self {
        Compile {
            use_peginator_build_time: true,
            ..self
        }
    }

    fn run_on_single_file(&self, source: &PathBuf, destination: &PathBuf) -> Result<()> {
        let grammar = fs::read_to_string(source)?;
        let source_header = generate_source_header(&grammar, self.use_peginator_build_time);
        if let Ok(f) = File::open(destination) {
            let mut existing_header = String::new();
            if f.take(source_header.bytes().count() as u64)
                .read_to_string(&mut existing_header)
                .is_ok()
                && source_header == existing_header
            {
                return Ok(());
            }
        };

        let parsed_grammar = Grammar::parse(&grammar)
            .map_err(|err| PrettyParseError::from_parse_error(&err, &grammar, source.to_str()))?;
        let generated_code = format!(
            "{}\n{}",
            source_header,
            parsed_grammar.generate_code(&Default::default())?
        );
        fs::write(destination, generated_code.to_string())?;
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

    pub fn run_exit_on_error(self) {
        colored::control::set_override(true);
        let result = self.run();
        if let Err(error) = result {
            // I absolutely hate how this is a global, because there is no way to know if it was forced
            // already. Thankfully we are exiting right after this.
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
