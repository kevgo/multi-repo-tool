use crate::runtime::Step;

/// prints Fish commands that set up autocompletion
pub fn fish() -> Vec<Step> {
    // disable completing filenames
    println!("complete -c mrt -f");
    // complete the built-in commands
    println!("complete -c mrt -a 'walk' -d 'manually iterate the subfolders'");
    println!("complete -c mrt -a 'run' -d 'execute the given command in all subfolders'");
    println!("complete -c mrt -a 'clone' -d 'clone all repos in the given GitHub org'");
    println!("complete -c mrt -a 'abort' -d 'abort a failed run'");
    println!("complete -c mrt -a 'retry' -d 'retry a failed step in a run'");
    println!("complete -c mrt -a 'ignore' -d 'ignore the currently failing step in a run'");
    println!("complete -c mrt -a 'next' -d 'go to the next subdir in walk'");
    vec![]
}
