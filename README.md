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
  - [abort](documentation/abort.md) the currently running queue
  - [retry](documentation/retry.md) the last failed step and continue the queue
  - [ignore](documentation/ignore.md) the last failed step and continue the
    queue
- [walk](documentation/walk.md) through all subdirectories and open a shell
  prompt in each
  - [next](documentation/next.md) goes to the next subdirectory
- [status](documentation/status.md) displays the current status of the command
  queue

### Dealing with failures

If one of the executed commands fails, mrt stops execution and allows you to do
one of these things:
