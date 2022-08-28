# "run" command

The `run` command executes the given CLI command in every subdirectory. If the
command fails, it ends in the failing directory. You can investigate the failure
and then do one of three things:

- **abort:** stop execution and return to the main directory
- **retry:** retry the failed operation after you fixed the issue
- **ignore:** skip the failed operation and continue executing the CLI command
  in the next subdirectory
