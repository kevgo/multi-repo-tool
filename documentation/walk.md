# "walk" command

The `walk` command gives you a command prompt in all subdirectories. This allows
you to perform manual work in each subdirectory with minimal typing to change
directories and making sure you don't forget a subdirectory.

You can use these commands during directory traversal:

- [next](next.md) to go to the next directory
- [abort](abort.md) to stop the directory traversal

### example

Assuming you have the subfolders `sub1`, `sub2`, and `sub3`. Start the walk:

```
m walk
```

This takes you to folder `sub1`. Do your thing here, then run:

```
m next
```

This takes you to folder `sub2`. Do your thing here, then run:

```
m next
```

This takes you to folder `sub3`. Do your thing here, then run:

```
m next
```

This takes you back to the root folder.
