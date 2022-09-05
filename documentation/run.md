# "run" command

The `run` command executes the given CLI command in every subdirectory.

If the command fails in one of the subdirectories, it ends and leaves you in the
failing directory. You can investigate the failure and then have three options:

- [abort](abort.md) to clear the job queue and return to the main directory
- [retry](retry.md) to retry the failed operation after you fixed the issue and
  continue executing the job queue
- [ignore](ignore.md) to skip the failed operation and continue executing the
  job queue

### example

Assuming you have the subfolders `sub1`, `sub2`, and `sub3`:

```
m run pwd

step 1: cd sub1

step 2: run "pwd"

  sub1

step 3: cd ../sub2

step 4: run "pwd"

  sub2

step 5: cd ../sub3

step 6: run "pwd"

  sub3

step 7: cd ..
```

### recipes

Update a dependency managed by npm:

```
m run -- npx npm-check-updates -u <dependency>
```
