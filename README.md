# CMPS

**An intelligent touch!**

`cmps` is short for `compose`, an extendable cross-platform CLI tool to create or fill files with a default content.

## Usage

```
cmps [OPTIONS] [FILENAME] [EXTENSION]

Arguments:
  [FILENAME]
          The filename to compose, may point to a non-existing file, or an empty existing file

  [EXTENSION]
          The extension to use, overrides the extension in the filename (if any)

Options:
      --show <extension>
          Show the template for this extension and the path to the source file

      --stdout
          Write the template for this file to stdout. Does not create or modify the file.

          Useful for integrating with editors like (Neo-)vim.

  -f, --force
          Overwrite existing files. This will clear the file contents if no template is found

  -v, --verbose...
          Sets the level of verbosity (provide multiple times for higher levels)

  -h, --help
          Print help information (use `-h` for a summary)

  -V, --version
          Print version information
```

Custom templates can be placed in your config directory (see https://docs.rs/dirs/latest/dirs/fn.config_dir.html for platform specific details) under `cmps/templates`. The name of the template file must match the file extension to be modified.

E.g. creating a file `cmps/templates/py` with the contents
```
#!/usr/bin/env python3
# Author: John Doe
```
will insert the shebang and the author name when creating python files with compose.

## Local (per-project) configuration
If you want to use different templates for a specific project, you can create a `.cmps` directory to add additional templates or overwrite existing ones.
A good use-case for this is if you want to automatically insert a specific license in one of your projects, but not everywhere.

Like in the `config` directory, templates must be placed under a `templates` subfolder in this `.cmps` directory.

E.g.: `/my/important/project/.cmps/templates/py` would overwrite the template used for python files in the `/my/important/project/` directory and its subdirectories.

## Authors
[Björn Daase](https://github.com/BjoernDaase) and [Leon Matthes](https://github.com/LeonMatthes)
