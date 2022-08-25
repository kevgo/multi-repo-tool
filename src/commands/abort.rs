use crate::runtime::Step;

pub fn abort(previous_steps: &Option<Vec<Step>>) -> Vec<Step> {
    match previous_steps {
        Some(steps) => println!("aborting {} steps", steps.len()),
        None => println!("nothing to abort"),
    }
    vec![]
}
