use crate::runtime::Step;

// provides the step for executing a CLI command in a subfolder
pub fn execute(id: usize, command: String, args: Vec<String>) -> Vec<Step> {
    vec![Step::Run { id, command, args }]
}
