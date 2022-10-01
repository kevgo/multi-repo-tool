use crate::config::Config;
use std::process::ExitCode;

pub fn activate() -> (Config, Option<ExitCode>) {
    // define wrapper shell function
    println!(
        r#"
function m -d "Fish shell wrapper for mrt"
    set cmd $argv[1]
    set args $argv[2..-1]
    if test "$cmd" = "--"
        set cmd $argv[2]
        set args $argv[3..-1]
    end
    env MRT_WRAPPED=true mrt $cmd -- $args
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
    println!("complete -c m -a 'status' -d 'display the current status of the work queue'");
    println!("complete -c m -a 'abort' -d 'abort a failed run'");
    println!("complete -c m -a 'retry' -d 'retry a failed step in a run'");
    println!("complete -c m -a 'ignore' -d 'ignore the currently failing step in a run'");
    println!("complete -c m -a 'ignore-all' -d 'ignore all failing steps in a run'");
    println!("complete -c m -a 'next' -d 'go to the next subdir in walk'");
    (Config::default(), Some(ExitCode::SUCCESS))
}
