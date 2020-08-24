# `shmod`

`shmod` is a tool for playing with [Numeric] ([Octal]), and [Symbolic] Unix
file-system permission notations.

A user can give either a numeric or symbolic representation of a permission and
see the corresponding symbolic or numeric permission. For example:

```shell
$ shmod 1750
1750: rwxr-x--T

$ shmod rwxr-xr-x
755: rwxr-xr-x
```

This tool may be of use to people new to Unix type permissions who want to
experiment with various permission notations to see how the octal and symbolic
representations correspond to each other.

<!-- links -->
[Numeric]: https://en.wikipedia.org/wiki/File-system_permissions#Numeric_notation
[Octal]: https://en.wikipedia.org/wiki/Octal
[Symbolic]: https://en.wikipedia.org/wiki/File-system_permissions#Symbolic_notation
