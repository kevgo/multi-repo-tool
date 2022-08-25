use crate::runtime::Step;

/// provides the Step for cloning the given repo
pub fn clone(id: usize, url: String) -> Step {
    Step {
        id,
        command: "git".into(),
        args: vec!["clone".into(), url],
    }
}
