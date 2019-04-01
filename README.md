# Within

Little project to learn some rust basics. Inspired by [within](https://github.com/sjmulder/within) I found a reference to somewhere on the archlinux forums.
Runs a given script/command within one or multiple given directories.

### Usage:
```
Runs a command within list of directories

USAGE:
    within <DIRECTORY>... [--] <COMMAND>...

FLAGS:
    -h, --help       Print this help message.
    -V, --version    Show version information.

ARGS:
    <DIRECTORY>...    Sets DIRECTORY to execute COMMAND in
    <COMMAND>...      COMMAND to execute in DIRECTORY

EXAMPLE:
    within ~/test1 ~/test2 -- ls
```

### Installation:
Currently only possible by building the tool yourself with cargo.

Clone the repo with:
```
git clone https://github.com/texhnolyze/within.git
```

Move into cloned directory and build a release binary:
```
cargo build --release
```

Within binary will then be located in `target/release/within`.
