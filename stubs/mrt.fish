function m -d "Fish shell stub for mrt"
    mrt $argv
    if test -e ./mrt.nextdir
        set --local nextdir (cat ./mrt.nextdir)
        echo "Found nextdir: $nextdir"
        rm ./mrt.nextdir
        cd $nextdir
    end
end
