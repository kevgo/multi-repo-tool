# Multi-repo tool

Mrt, pronounced _murt_, makes working with a large number of Git repositories
efficient by executing user-provided CLI commands in them.

### Installation

1. install [rustup](https://rustup.rs) and through that the current stable
   version of [Rust](https://www.rust-lang.org)

2. install mrt:

   ```
   cargo install --git --locked github.com/kevgo/mrt`
   ```

3. install the shell wrapper through which you will run mrt:

   ```
   ~/.cargo/bin/mrt activate | source
   ```

Now you can run mrt by calling the shell function `m`. No need to change the
`$PATH` environment variable. This also gives you auto-completion of mrt
arguments.

### Usage

Main operations:

- [clone](documentation/clone.md) all repositories of a Github organization to
  your local machine
- [run](documentation/run.md) a given CLI command in all subdirectories. _mrt_
  prints the outputs. If the command fails in a subdirectory, _mrt_ exits in
  this subdirectory to let you inspect the problem. Then you can
  [abort](documentation/abort.md) the entire command queue,
  [retry](documentation/retry.md) or [ignore](documentation/ignore.md) the last
  failed step.
- [walk](documentation/walk.md) through all subdirectories and interactively run
  commands in each. [next](documentation/next.md) goes to the next subdirectory.
  [abort](documentation/abort.md) stops the walk.
- [status](documentation/status.md) displays the current status of the command
  queue
- You can limit execution to a subset of folders
