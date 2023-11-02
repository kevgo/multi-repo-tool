use crate::error::UserError;
use crate::helpers::{folder_list, subdirs};
use crate::runtime::steps::NumberedStep;
use camino::{Utf8Path, Utf8PathBuf};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::fs::File;
use std::io::{BufReader, BufWriter, ErrorKind};
use std::{env, fs};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Config {
    /// the root directory
    pub root_dir: Option<String>,
    /// steps to execute
    pub steps: Vec<NumberedStep>,
    /// folders to execute the steps in
    /// None --> all folders
    /// Some --> only the specified folders
    pub folders: Option<Vec<String>>,
}

impl Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let all_count = self
            .root_dir
            .as_ref()
            .map(|root_dir| subdirs::count(root_dir).expect("cannot list the root dir"));
        match &self.folders {
            Some(folders) => {
                match all_count {
                    Some(all) => writeln!(f, "Running only in {}/{} folders:", folders.len(), all),
                    None => writeln!(f, "Running only in {} folders:", folders.len()),
                }?;
                write!(f, "{}", folder_list::render(folders))?;
            }
            None => match all_count {
                Some(all) => writeln!(f, "Running in all {all} folders."),
                None => writeln!(f, "Running in all folders."),
            }?,
        }
        writeln!(f)?;
        if self.steps.is_empty() {
            writeln!(f, "I'm not doing anything right now.")
        } else {
            let max_id = self.steps.last().map_or(0, |step| step.id);
            for step in &self.steps {
                writeln!(f, "{}", &step.list(max_id)).unwrap();
            }
            Ok(())
        }
    }
}

pub fn delete(config_path: &Utf8Path) {
    let _ignore_failure = fs::remove_file(config_path);
}

/// provides the full path to the config file
pub fn filepath() -> Utf8PathBuf {
    let home_path = env::var("HOME").expect("cannot read environment variable $HOME");
    Utf8PathBuf::from(format!("{home_path}/.config/mrt.json"))
}

pub fn load(config_path: &Utf8Path) -> Result<Config, UserError> {
    let file = match File::open(config_path) {
        Ok(file) => file,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => return Ok(Config::default()),
            _ => {
                return Err(UserError::CannotReadPersistenceFile {
                    guidance: e.to_string(),
                    filename: config_path.to_string(),
                })
            }
        },
    };
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).map_err(|err| UserError::InvalidPersistenceFormat {
        filename: config_path.to_string(),
        guidance: err.to_string(),
    })
}

pub fn save(config_path: &Utf8Path, config: &Config) -> Result<(), UserError> {
    let file = File::create(config_path).map_err(|err| UserError::CannotWriteFile {
        filename: config_path.to_string(),
        guidance: err.to_string(),
    })?;
    let writer = BufWriter::new(file);
    match serde_json::to_writer_pretty(writer, config) {
        Ok(_) => Ok(()),
        Err(e) => Err(UserError::CannotWriteFile {
            filename: config_path.to_string(),
            guidance: e.to_string(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use crate::config::{self, Config};
    use crate::runtime::steps::{NumberedStep, Step};
    use big_s::S;
    use camino::Utf8PathBuf;
    use std::fs;

    #[test]
    fn persistence() {
        let config1 = Config {
            steps: vec![
                NumberedStep {
                    id: 2,
                    step: Step::Chdir { dir: S("abc") },
                },
                NumberedStep {
                    id: 3,
                    step: Step::Run {
                        cmd: S("git"),
                        args: vec![S("clone")],
                    },
                },
            ],
            folders: Some(vec![S("sub1"), S("sub2")]),
            root_dir: Some(S("rootdir")),
        };
        let config_path = Utf8PathBuf::from("mrt_test.json");
        config::save(&config_path, &config1).unwrap();
        let config2 = config::load(&config_path).unwrap();
        assert_eq!(config1, config2);
        fs::remove_file(&config_path).unwrap();
    }
}
