<picture>
  <source media="(prefers-color-scheme: dark)" srcset="documentation/logo_800_dark.png">
  <source media="(prefers-color-scheme: light)" srcset="documentation/logo_800_light.png">
  <img alt="mrt logo" src="documentation/logo_800_light.png">
</picture>

`mrt`, pronounced _murt_, executes CLI commands in all/some Git repositories of
a GitHub organization.

### Installation

1. install [rustup](https://rustup.rs) and through that the current stable
   version of [Rust](https://www.rust-lang.org)

2. install mrt:

   ```
   cargo install --locked --git github.com/kevgo/mrt`
   ```

3. install the shell wrapper through which you will run mrt:

   ```
   ~/.cargo/bin/mrt activate | source
   ```

Now you can run mrt by calling the shell function `m`. No need to change the
`$PATH` environment variable. This also gives you auto-completion of mrt
arguments.

### Tutorial

Let's say you work at ACME Corp. ACME hosts its code on GitHub. One of the
vendors has released a critical security update to a library that is widely used
at ACME. Your job is to update this dependency company-wide. You estimate this
update needs to happen in around 200 Git repositories. For each repo, you need
to do these 7 steps:

- clone the repo onto your machine
- run `git checkout -b critical-update` (or since we use
  [Git Town](https://www.git-town.com): `git hack critical-update`)
- run `npm update <dependency>`
- run `git add -A && git commit -m "Update critical dependency"`
- submit a pull request

Five activities times two hundred repositories --> you are looking at doing one
thousand activities manually. You could automate some of them via Bash scripts
but how do you continue when one of the codebases encounters a problem?

_mrt_ enters the chat.

```
m clone acme
```

This [clones](documentation/clone.md) all 600 repos of `acme` organization on
GitHub onto your machine. To try out _mrt_, run this:

```
m run pwd
```

This runs `pwd` in all 600 repos and prints the output. We want to run commands
only in repositories that contain Node.js codebases.

```
m limit ls package.json
```

The ["limit" command](documentation/limit.md) limits subsequent _mrt_ activities
to repositories for which the given CLI command returns an exit code of `0`.
Node.js codebases contain a file `package.json`. The `ls` command returns exit
code `1` if the given file doesn't exist.

```
m run git hack critical-update
```

This creates a branch called `critical-update` in each of the 200 repos.

```
m run npm update <dependency>`
```

This runs `npm update <dependency>` in each of the 200 repos.

```
m run -- git add -A
m run -- git commit -m "Update critical dependency"
```

Time to submit 200 pull requests:

```
m run git new-pull-request
```

`git new-pull-request` is a [Git Town](https://www.git-town.com) command. Watch
200 browser tabs open in your browser.

All in all, we ran 7 CLI commands (instead of 1000) to roll out a dependency
update to 200 code repositories.

### Reference

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
