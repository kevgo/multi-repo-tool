use std::env;
use std::fs::File;
use std::io::{BufReader, BufWriter, ErrorKind};

use crate::error::UserError;
use crate::runtime::Step;
use camino::{Utf8Path, Utf8PathBuf};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    /// the root directory
    pub root_dir: Option<String>,
    /// steps to execute
    pub steps: Vec<Step>,
    /// folders to execute the steps in
    /// None --> all folders
    /// Some --> only the specified folders
    pub folders: Option<Vec<String>>,
}

/// provides the full path to the config file
pub fn filepath() -> Utf8PathBuf {
    let home_path = env::var("HOME").expect("cannot read environment variable $HOME");
    Utf8PathBuf::from(format!("{}/.config/mrt.json", home_path))
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
    use crate::runtime::Step;
    use camino::Utf8PathBuf;
    use std::fs;

    #[test]
    fn persistence() {
        let config1 = Config {
            steps: vec![
                Step::Chdir {
                    id: 2,
                    dir: "abc".into(),
                },
                Step::Run {
                    id: 3,
                    cmd: "git".into(),
                    args: vec!["clone".into()],
                },
            ],
            folders: Some(vec!["sub1".into(), "sub2".into()]),
            root_dir: Some("rootdir".into()),
        };
        let config_path = Utf8PathBuf::from("mrt_test.json");
        config::save(&config_path, &config1).unwrap();
        let config2 = config::load(&config_path).unwrap();
        assert_eq!(config1, config2);
        fs::remove_file(&config_path).unwrap();
    }
}
