use crate::runtime::Step;
use std::process;

pub fn status(persisted_steps: &Vec<Step>) -> Vec<Step> {
    if persisted_steps.is_empty() {
        println!("I'm not doing anything right now.");
    } else {
        for step in persisted_steps {
            println!("{}", step);
        }
    }
    process::exit(0);
}
