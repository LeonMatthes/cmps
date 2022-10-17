# CMPS

**An intelligent touch!**

`cmps` is short for `compose`, an extendable cross-platform CLI tool to create or fill files with a default content.

## Usage

```
cmps [FLAGS] [OPTIONS] <FILENAME> [EXTENSION]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Sets the level of verbosity (provide multiple times for higher levels)

OPTIONS:
        --show <extension>    Show the template for this extension and the path to the source file.

ARGS:
    <FILENAME>     The filename to compose, may point to a non-existing file, or an empty existing file.
    <EXTENSION>    The extension to use, overrides the extension in the filename (if any).
```

Custom templates can be placed in your config directory (see https://docs.rs/dirs/latest/dirs/fn.config_dir.html for platform specific details) under `cmps/templates`. The name of the template file must match the file extension to be modified.

E.g. creating a file `cmps/templates/py` with the contents
```
#!/usr/bin/env python3
# Author: John Doe
```
will insert the shebang and the author name when creating python files with compose.

## Authors
[Björn Daase](https://github.com/BjoernDaase) and [Leon Matthes](https://github.com/LeonMatthes)
