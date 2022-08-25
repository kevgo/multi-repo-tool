use crate::runtime::Step;

pub fn ignore(previous_steps: Option<Vec<Step>>) -> Vec<Step> {
    match previous_steps {
        Some(steps) => {
            let mut step_iter = steps.into_iter();
            match step_iter.next() {
                Some(_) => step_iter.collect(),
                None => vec![],
            }
        }
        None => vec![],
    }
}
