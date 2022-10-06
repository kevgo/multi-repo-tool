# "limit" command

The `limit` command limits execution to a subset of directories. To determine
which folders to include, _mrt_ runs the given CLI command in all subfolders.
Folders in which the command exits with code `0` (success) get included in
[walks](walk.md) and [runs](run.md). Use the ["status" command](status.md) to
see the list of included folders. When done with the folder subset, you can go
back to processing [all](all.md) folders.

```
m limit <command>
```

A good CLI command to call is [has](https://github.com/kevgo/has).

### recipes

Limit to ...

- repos that contain a certain file or folder:

  ```
  m limit test -f  <file or folder name>
  m limit has file|folder <name>
  ```

- repos that contain a certain branch:

  ```
  m limit git show-ref --verify --quiet refs/heads/<branch name>
  m limit has branch <name>
  ```
