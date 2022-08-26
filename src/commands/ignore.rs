use crate::error::UserError;
use crate::runtime::Step;
use std::mem::drop;

pub fn ignore(previous_steps: Vec<Step>) -> Result<Vec<Step>, UserError> {
    if previous_steps.is_empty() {
        return Err(UserError::NothingToIgnore {});
    }
    let mut step_iter = previous_steps.into_iter();
    drop(step_iter.next());
    Ok(step_iter.collect())
}

#[cfg(test)]
mod tests {
    use crate::error::UserError;
    use crate::runtime::Step;

    #[test]
    fn content() {
        let give: Vec<Step> = vec![
            Step {
                id: 1,
                ..Step::default()
            },
            Step {
                id: 2,
                ..Step::default()
            },
        ];
        let want = Ok(vec![Step {
            id: 2,
            ..Step::default()
        }]);
        let have = super::ignore(give);
        assert_eq!(have, want);
    }

    #[test]
    fn empty() {
        let give: Vec<Step> = vec![];
        let want = UserError::NothingToIgnore {};
        let have = super::ignore(give);
        assert_eq!(have, Err(want));
    }
}
