use crate::runtime::Step;

pub fn retry(previous_steps: Vec<Step>) -> Vec<Step> {
    if previous_steps.is_empty() {
        println!("Nothing to retry");
        vec![]
    } else {
        previous_steps
    }
}
