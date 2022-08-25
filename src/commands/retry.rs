use crate::runtime::Step;

pub fn retry(previous_steps: Option<Vec<Step>>) -> Vec<Step> {
    match previous_steps {
        Some(steps) => steps,
        None => {
            println!("Nothing to retry");
            vec![]
        }
    }
}
