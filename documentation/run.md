# "run" command

The `run` command executes the given CLI command in every subdirectory.

```
m run <command to run>
```

If the command fails in one of the subdirectories, it ends and leaves you in the
failing directory. You can investigate the failure and then do one of three
things:

- [abort](abort.md) to clear the job queue and return to the main directory
- [retry](retry.md) to retry the failed operation after you fixed the issue and
  continue executing the job queue
- [ignore](ignore.md) to skip the failed operation and continue executing the
  job queue
