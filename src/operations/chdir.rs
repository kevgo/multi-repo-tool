use crate::runtime::Step;

pub fn chdir(id: usize, dir: String) -> Step {
    Step::Chdir { id, dir }
}
