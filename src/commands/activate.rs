use crate::runtime::Step;

pub fn activate() -> Vec<Step> {
    println!(
        r#"
function m -d "Fish shell stub for mrt" -w mrt
    env MRT_ACTIVATED=true mrt $argv
    set file_path ~/.config/mrt.next_dir
    if test -e $file_path
        set --local nextdir (cat $file_path)
        echo "Found nextdir: $nextdir"
        rm $file_path
        cd $nextdir
    end
end
"#
    );
    vec![]
}
