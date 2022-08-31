use crate::runtime::Step;

pub fn activate() -> Vec<Step> {
    println!(
        r#"
function m -d "Fish shell stub for mrt" -w mrt
    env MRT_ACTIVATED=true mrt $argv
    set next_dir_path ~/.config/mrt.next_dir
    if test -e $next_dir_path
        set --local nextdir (cat $next_dir_path)
        rm $next_dir_path
        cd $nextdir
    end
end
"#
    );
    vec![]
}
