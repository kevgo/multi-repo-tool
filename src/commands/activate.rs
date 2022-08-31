use crate::runtime::Step;

pub fn activate() -> Vec<Step> {
    // define wrapper shell function
    println!(
        r#"
function m -d "Fish shell wrapper for mrt"
    env MRT_WRAPPED=true mrt $argv
    set --local next_dir_path ~/.config/mrt.next_dir
    if test -e $next_dir_path
        set --local nextdir (cat $next_dir_path)
        rm $next_dir_path
        cd $nextdir
    end
end
"#
    );
    // disable auto-completing filenames
    println!("complete -c m -f");
    // auto-complete the built-in commands
    println!("complete -c m -a 'walk' -d 'manually iterate the subfolders'");
    println!("complete -c m -a 'run' -d 'execute the given command in all subfolders'");
    println!("complete -c m -a 'clone' -d 'clone all repos in the given GitHub org'");
    println!("complete -c m -a 'abort' -d 'abort a failed run'");
    println!("complete -c m -a 'retry' -d 'retry a failed step in a run'");
    println!("complete -c m -a 'ignore' -d 'ignore the currently failing step in a run'");
    println!("complete -c m -a 'next' -d 'go to the next subdir in walk'");
    vec![]
}
