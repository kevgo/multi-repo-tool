# Multi-repo tool

This tool allows executing CLI commands in multiple Git repositories.

### installation

1. install the binary:

   ```
   cargo install --git --locked github.com/kevgo/mrt`
   ```

2. install the shell wrapper that you need to run the binary through:

   ```
   curl https://raw.githubusercontent.com/kevgo/mrt/main/Cargo.lock [dir in your $PATH]
   ```

### Available operations

- [clone](documentation/clone.md) all repositories of a Github organization onto
  your local machine
- [run](documentation/run.md) a CLI command in all subdirectories
- [walk](documentation/walk.md) through all subdirectories and manually do
  something in each

### Dealing with failures

If one of the executed commands fails, mrt stops execution and allows you to do
one of these things:

- [abort](documentation/abort.md) to stop the entire command queue
- [retry](documentation/retry.md) to continue the command queue by retrying the
  failed command
- [ignore](documentation/ignore.md) to continue the command queue by skipping
  the failed command and executing the next one
