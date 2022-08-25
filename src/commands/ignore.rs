use crate::runtime::Step;
use std::mem::drop;

pub fn ignore(previous_steps: Option<Vec<Step>>) -> Vec<Step> {
    match previous_steps {
        Some(steps) => {
            let mut step_iter = steps.into_iter();
            drop(step_iter.next());
            step_iter.collect()
        }
        None => vec![],
    }
}

#[cfg(test)]
mod tests {
    use crate::runtime::Step;

    #[test]
    fn content() {
        let give: Vec<Step> = vec![
            Step {
                id: 1,
                command: "cmd1".into(),
                args: vec![],
            },
            Step {
                id: 2,
                command: "cmd2".into(),
                args: vec![],
            },
        ];
        let want: Vec<Step> = vec![Step {
            id: 2,
            command: "cmd2".into(),
            args: vec![],
        }];
        let have = super::ignore(Some(give));
        assert_eq!(have, want);
    }

    #[test]
    fn empty() {
        let give: Vec<Step> = vec![];
        let want: Vec<Step> = vec![];
        let have = super::ignore(Some(give));
        assert_eq!(have, want);
    }
}
