use crate::config::Config;
use crate::error::UserError;
use crate::helpers::subdirs;
use crate::runtime::steps::{self, Step};
use camino::Utf8PathBuf;
use std::process::ExitCode;

pub fn run(
  cmd: &str,
  args: &[String],
  config: Config,
  root_dir: &Utf8PathBuf,
) -> Result<(Config, Option<ExitCode>), UserError> {
  let mut steps = vec![];
  let dirs = match &config.folders {
    Some(config_folders) => config_folders.clone(),
    None => subdirs::all(root_dir)?,
  };
  for dir in dirs {
    steps.push(Step::Chdir { dir });
    steps.push(Step::Run {
      cmd: cmd.to_string(),
      args: args.to_owned(),
    });
  }
  Ok((
    Config {
      steps: steps::numbered(steps),
      root_dir: Some(root_dir.to_string()),
      ..config
    },
    None,
  ))
}
