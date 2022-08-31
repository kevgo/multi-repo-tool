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
   curl https://raw.githubusercontent.com/kevgo/mrt/main/stubs/mrt.fish -o <somewhere in your $PATH>
   ```

To set up auto-completion for Fish shell, add this to your
`~/.config/fish/config.fish`:

```
if test -f ~/.cargo/bin/mrt
  ~/.cargo/bin/mrt completions | source
end
```

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

### Dealing with failures

If one of the executed commands fails, mrt stops execution and allows you to do
one of these things:

```

```
