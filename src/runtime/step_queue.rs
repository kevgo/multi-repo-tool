use super::Step;
use crate::error::UserError;
use camino::{Utf8Path, Utf8PathBuf};
use std::fs::File;
use std::io::{BufReader, BufWriter, ErrorKind};
use std::{env, fs};

pub fn delete(config_path: &Utf8Path) {
    drop(fs::remove_file(config_path));
}

/// provides the full path to the config file
pub fn filepath() -> Utf8PathBuf {
    let fullpath = format!(
        "{}/.config/mrt.json",
        env::var("HOME").expect("cannot read environment variable $HOME")
    );
    Utf8PathBuf::from(fullpath)
}

pub fn load(config_path: &Utf8Path) -> Result<Vec<Step>, UserError> {
    let file = match File::open(filepath()) {
        Ok(file) => file,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => return Ok(vec![]),
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

pub fn save(config_path: &Utf8Path, steps: &Vec<Step>) -> Result<(), UserError> {
    let file = File::create(filepath()).map_err(|err| UserError::CannotWriteFile {
        filename: config_path.to_string(),
        guidance: err.to_string(),
    })?;
    let writer = BufWriter::new(file);
    match serde_json::to_writer_pretty(writer, steps) {
        Ok(_) => Ok(()),
        Err(e) => Err(UserError::CannotWriteFile {
            filename: config_path.to_string(),
            guidance: e.to_string(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use crate::runtime::step_queue::{self, delete};
    use crate::runtime::Step;
    use camino::Utf8PathBuf;

    #[test]
    fn persistence() {
        let steps1 = vec![
            Step::Chdir {
                id: 2,
                dir: "abc".into(),
            },
            Step::Run {
                id: 3,
                cmd: "git".into(),
                args: vec!["clone".into()],
            },
        ];
        let config_path = Utf8PathBuf::from("mrt_test.json");
        step_queue::save(&config_path, &steps1).unwrap();
        let steps2 = step_queue::load(&config_path).unwrap();
        assert_eq!(steps1, steps2);
        delete(&config_path);
    }
}
