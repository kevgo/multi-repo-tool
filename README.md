# Multi-repo tool

This tool allows working with multiple Git repositories.

### Start a workflow

- [clone](documentation/clone.md) a Github organization

### Workflow failures

Once a workflow steps fails, use one of these commands:

- [abort](documentation/abort.md) to stop the entire workflow
- [retry](documentation/retry.md) to try the currently failed workflow command
  by retrying the failed command
- [ignore](documentation/ignore.md) to continue the currently failed workflow by
  skipping the failed command and executing the next one
