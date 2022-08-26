use crate::runtime::Step;

pub fn abort(previous_steps: &Vec<Step>) -> Vec<Step> {
    if previous_steps.is_empty() {
        println!("nothing to abort");
    } else {
        println!("aborting {} steps", previous_steps.len());
    }
    vec![]
}
