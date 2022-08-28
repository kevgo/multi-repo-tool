# Multi-repo tool

This tool allows executing CLI commands in multiple Git repositories.

### installation

- install the binary: `cargo install --git --locked github.com/kevgo/mrt`
- you need to run mrt through a install the shell wrapper

### Available operations

- [clone](documentation/clone.md) all repositories of a Github organization onto
  your local machine
- [run](
- [walk](documentation/walk.md)

### Dealing with failures

If one of the executed commands fails, mrt stops execution and allows you to do
one of these things:

- [abort](documentation/abort.md) to stop the entire command queue
- [retry](documentation/retry.md) to continue the command queue by retrying the
  failed command
- [ignore](documentation/ignore.md) to continue the command queue by skipping
  the failed command and executing the next one
