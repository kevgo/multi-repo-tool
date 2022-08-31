# Multi-repo tool

Mrt, pronounced _murt_, allows executing CLI commands in multiple Git
repositories. Execution can happen automated or manual.

### Installation

1. install the binary:

   ```
   cargo install --git --locked github.com/kevgo/mrt`
   ```

2. install the shell wrapper that you need to run the binary through:

   ```
   ~/.cargo/bin/mrt activate | source
   ```

Now you can run mrt by calling `m`. No need to change the `$PATH` environment
variable. This also gives you auto-completion in your shell.

### Usage

Mrt provides three main operations:

- [clone](documentation/clone.md) all repositories of a Github organization onto
  your local machine
- [run](documentation/run.md) a CLI command in all subdirectories
  - [abort](documentation/abort.md) to stop the queue
  - [retry](documentation/retry.md) to continue the command queue by retrying
    the failed command
  - [ignore](documentation/ignore.md) to continue the command queue by skipping
    the failed command and executing the next one
- [walk](documentation/walk.md) through all subdirectories and manually do
  something in each
  - [next](documentation/next.md)
- [status](doc

### Dealing with failures

If one of the executed commands fails, mrt stops execution and allows you to do
one of these things:
