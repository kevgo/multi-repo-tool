use crate::runtime::Step;

pub fn exit(id: u32) -> Step {
    Step::Exit { id }
}
