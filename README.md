# Multi-repo tool

This tool allows executing CLI commands in multiple Git repositories.

### Starting a workflow

Use

- [clone](documentation/clone.md) all repositoriess of a Github organization
  onto your local machine

### Dealing with failures

Once a workflow steps fails, use one of these commands:

- [abort](documentation/abort.md) to stop the entire workflow
- [retry](documentation/retry.md) to try the currently failed workflow command
  by retrying the failed command
- [ignore](documentation/ignore.md) to continue the currently failed workflow by
  skipping the failed command and executing the next one
