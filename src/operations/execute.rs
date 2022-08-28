use crate::runtime::Step;

// provides the step for executing a CLI command in a subfolder
pub fn execute(id: u32, command: String, args: Vec<String>) -> Step {
    Step::Run { id, command, args }
}
