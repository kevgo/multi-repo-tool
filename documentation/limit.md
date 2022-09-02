# "limit" command

The `limit` command limits execution to a subset of directories. _mrt_ runs the
given CLI command in all subfolders. If the command exits with code `0`, mrt
will include this folder in [walks](walk.md) and [runs](run.md). Use the
["status" command](status.md) to see the list of folders. When done with the
folder subset, you can go back to processing [all](all.md) folders.

```
m limit <command>
```
