use crate::runtime::Step;

pub fn activate() -> Vec<Step> {
    println!(
        r#"
function m -d "Fish shell stub for mrt" -w mrt
    mrt --activated $argv
    if test -e ./mrt.nextdir
        set --local nextdir (cat ./mrt.nextdir)
        echo "Found nextdir: $nextdir"
        rm ./mrt.nextdir
        cd $nextdir
    end
end
"#
    );
    vec![]
}
