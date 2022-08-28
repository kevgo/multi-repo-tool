function m -d "Fish shell stub for mrt"
  mrt $argv
  if test -e ./mrt_nextdir
    set -f nextdir (cat ./mrt_nextdir)
    rm ./mrt_nextdir
    cd $nextdir
  end
end
