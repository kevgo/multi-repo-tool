# Tutorial

A critical security update to a library that is widely used at your organization
has become available. You want to update this dependency company-wide. You
estimate this update needs to happen in around 200 Git repositories. For each
repo, you need to:

1. clone the repo onto your machine
2. run `git checkout -b critical-update` (or since we use
   [Git Town](https://www.git-town.com): `git hack critical-update`)
3. run `npx npm-check-updates -u <dependency>`
4. run `git add -A && git commit -m "Update critical dependency"`
5. submit a pull request to the team maintaining that codebase

Five activities times two hundred repositories --> you are looking at doing one
thousand activities manually. You could automate some of them via Bash scripts
but how do you continue when one of the codebases encounters a problem?

_mrt_ enters the chat.

```
m clone <org name>
```

`m` is the shell wrapper for _mrt_. This command [clones](clone.md) all 600
repos of your GitHub organization onto your machine.

If a clone fails, _mrt_ quits and lets you inspect the problem. When you are
ready to continue, you can run:

- `m retry` to retry the failed command
- `m abort` to step cloning
- `m ignore` to skip the failed clone and continue cloning the remaining
  repositories
- `m ignore-all` to skip all failed clones

```
m run git status
```

This runs `git status` in all 600 repos and prints the output. That's great but
let's limit the next couple of commands to repositories that contain Node.js
codebases.

```
m limit ls package.json
```

The ["limit" command](limit.md) limits _mrt_ activities to repositories for
which the given CLI command returns an exit code of `0`. Node.js codebases
contain a file `package.json`. The `ls` command returns exit code `1` if the
given file doesn't exist.

```
m run git hack critical-update
```

This creates a branch called `critical-update` in each of the 200 Node.js repos.

```
m run npx npm-check-updates -u <dependency>
```

This runs `npm update <dependency>` in each of the 200 repos.

```
m run git add -A
m run git commit -m "Update critical dependency"
```

Time to submit 200 pull requests:

```
m run git new-pull-request
```

`git new-pull-request` is a [Git Town](https://www.git-town.com) command. Watch
200 browser tabs open in your browser. Review each pull request and submit.

All in all, we ran 7 CLI commands (instead of 1000) to roll out a dependency
update to 200 code repositories.
