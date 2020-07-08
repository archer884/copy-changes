# copy-changes

`copy-changes` is a simple utility intended to save time (and SSD writes) when iterating on certain kinds of projects.

```shell
~/src/copy-changes master
❯ copy-changes --help
copy-changes 0.1.2

USAGE:
    copy-changes [FLAGS] <from> <to>

FLAGS:
    -f, --force
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose

ARGS:
    <from>
    <to>
```

The basic idea is you tell the program to copy files from one location to another and it will copy only those which either A) do not exist in the "to" location, or B) those files which have been updated in the "from" location. To see which files would be copies *without performing the copy,* run the program with the `--verbose` flag set. To actually force the copy (thereby overwriting files in the destination), set the `--force` flag.

## Installation

Assuming you have set up cargo, you may install using the following command:

```shell
$ cargo install --git https://github.com/archer884/copy-changes.git
```

To install cargo, [follow these instructions](https://rustup.rs/).
