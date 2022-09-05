# "clone" command

The `clone` command downloads an entire Github organization into the current
folder of your local machine.

If you run into rate limits with the GitHub API,
[create a personal access token](https://github.com/settings/tokens/new) and
create an environment variable `GITHUB_TOKEN` containing this token.

### example

To clone all repositories of the Github organization https://github.com/ory:

```
m clone ory
```
