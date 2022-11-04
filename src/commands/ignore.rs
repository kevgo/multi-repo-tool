use crate::config::Config;
use crate::error::UserError;
use std::mem::drop;
use std::process::ExitCode;

pub fn ignore(config: Config) -> Result<(Config, Option<ExitCode>), UserError> {
    if config.steps.is_empty() {
        return Err(UserError::NothingToIgnore {});
    }
    let mut step_iter = config.steps.into_iter();
    drop(step_iter.next());
    drop(step_iter.next());
    Ok((
        Config {
            steps: step_iter.collect(),
            ..config
        },
        None,
    ))
}

#[cfg(test)]
mod tests {
    use big_s::S;

    use crate::config::Config;
    use crate::error::UserError;
    use crate::runtime::steps::{NumberedStep, Step};

    #[test]
    fn content() {
        let give = Config {
            steps: vec![
                NumberedStep {
                    id: 1,
                    step: Step::Chdir { dir: S("one") },
                },
                NumberedStep {
                    id: 2,
                    step: Step::Run {
                        cmd: S("pwd"),
                        args: vec![],
                    },
                },
                NumberedStep {
                    id: 3,
                    step: Step::Chdir { dir: S("two") },
                },
                NumberedStep {
                    id: 4,
                    step: Step::Run {
                        cmd: S("pwd"),
                        args: vec![],
                    },
                },
            ],
            ..Config::default()
        };
        let want = Config {
            steps: vec![
                NumberedStep {
                    id: 3,
                    step: Step::Chdir { dir: S("two") },
                },
                NumberedStep {
                    id: 4,
                    step: Step::Run {
                        cmd: S("pwd"),
                        args: vec![],
                    },
                },
            ],
            ..Config::default()
        };
        match super::ignore(give) {
            Ok((config, exit_code)) => {
                assert_eq!(config, want);
                assert!(exit_code.is_none());
            }
            Err(err) => panic!("{}", err.messages().0),
        }
    }

    #[test]
    fn empty() {
        let give = Config {
            steps: vec![],
            ..Config::default()
        };
        let want = UserError::NothingToIgnore;
        match super::ignore(give) {
            Ok(data) => panic!("{:?}", data),
            Err(err) => assert_eq!(err, want),
        }
    }
}
