use crate::config::Config;
use crate::error::UserError;
use std::mem::drop;

pub fn ignore(config: Config) -> Result<Config, UserError> {
    if config.steps.is_empty() {
        return Err(UserError::NothingToIgnore {});
    }
    let mut step_iter = config.steps.into_iter();
    drop(step_iter.next());
    Ok(Config {
        steps: step_iter.collect(),
        ..config
    })
}

#[cfg(test)]
mod tests {
    use crate::config::Config;
    use crate::error::UserError;
    use crate::runtime::Step;

    #[test]
    fn content() {
        let give = Config {
            steps: vec![
                Step::Chdir {
                    id: 1,
                    dir: "one".into(),
                },
                Step::Chdir {
                    id: 2,
                    dir: "two".into(),
                },
            ],
            ..Config::default()
        };
        let want = Ok(Config {
            steps: vec![Step::Chdir {
                id: 2,
                dir: "two".into(),
            }],
            ..Config::default()
        });
        let have = super::ignore(give);
        assert_eq!(have, want);
    }

    #[test]
    fn empty() {
        let give = Config {
            steps: vec![],
            ..Config::default()
        };
        let want = UserError::NothingToIgnore {};
        let have = super::ignore(give);
        assert_eq!(have, Err(want));
    }
}
