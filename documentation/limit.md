# "limit" command

The `limit` command limits execution to a subset of directories. To determine
which folders to include, _mrt_ runs the given CLI command in all subfolders.
Folders in which the command exits with code `0` get included in
[walks](walk.md) and [runs](run.md). Use the ["status" command](status.md) to
see the list of included folders. When done with the folder subset, you can go
back to processing [all](all.md) folders.

```
m limit <command>
```

As an example, let's say some of your subfolders contain Python code, some
Node.js code. To limit work to the Node.js codebases:

```
m limit ls package.json
```

The `ls` command exits with code `1` if the given file doesn't exist.
