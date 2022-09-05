<picture>
  <source media="(prefers-color-scheme: dark)" srcset="documentation/logo_800_dark.png">
  <source media="(prefers-color-scheme: light)" srcset="documentation/logo_800_light.png">
  <img alt="mrt logo" src="documentation/logo_800_light.png">
</picture>

`mrt`, pronounced _murt_, is a multi-repo tool that makes executing CLI commands
in a large number of Git repositories efficient.

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

- [clone](documentation/clone.md) all repositories of a Github organization to
  your local machine
- [run](documentation/run.md) a given CLI command in all subdirectories and
  print the outputs. If a command fails, _mrt_ exits in the respective
  subdirectory to let you inspect/fix the problem. Then you can
  [abort](documentation/abort.md) the entire command queue,
  [retry](documentation/retry.md), [ignore](documentation/ignore.md) the failed
  step, or [ignore all](src/commands/ignore-all.rs) failed steps.
- [walk](documentation/walk.md) through all subdirectories and interactively run
  commands in each. When done with a folder, you can go to the
  [next](documentation/next.md) one or [abort](documentation/abort.md) the walk.
- [status](documentation/status.md) displays the current state of the command
  queue
- [limit](documentation/limit.md) execution to a subset of folders. When done
  with the folder subset, you can go back to processing
  [all](documentation/all.md) folders.
