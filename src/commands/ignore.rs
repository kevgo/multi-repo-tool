use crate::runtime::Step;

pub fn ignore(previous_steps: Option<Vec<Step>>) -> Vec<Step> {
    match previous_steps {
        Some(steps) => {
            let mut step_iter = steps.into_iter();
            match step_iter.next() {
                Some(_) => Vec::from_iter(step_iter),
                None => return vec![],
            }
        }
        None => vec![],
    }
}
