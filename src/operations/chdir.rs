use crate::runtime::Step;

pub fn chdir(id: u32, dir: String) -> Step {
    Step::Chdir { id, dir }
}
