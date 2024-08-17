# Clippy
Linux CLI tool for managing clipboard.

## Installation

1. Get the latest `.deb` package from the [releases](/MartinGamesCZ/Clippy/releases) page.
2. Install the package using `sudo dpkg -i path_to_clippy.deb`.
3. You are now ready to use Clippy.

## Usage

Clippy can be used using the command `clippy`.

### Commands
- `clippy copy` - Copy the piped text to the clipboard.
- `clippy paste` - Paste the clipboard content to the terminal / pipe into another process/file.

### Pipes
You can pipe clippy to other files or processes.

#### Copy

For example, you can pipe output of a `pwd` command into clippy:

```bash
# Using copy command
pwd | clippy copy

# You can also use the command without the copy argument
pwd | clippy
```

This will copy the current working directory to the clipboard.

#### Paste

You can also pipe clippy to other commands:

```bash
# Using paste command
clippy paste | xargs ls -l

# You can also use the command without the paste argument
clippy | xargs ls -l
```

This will list the content of the path in the clipboard.

You can also use it as an argument for a command:

```bash
# Using paste command
ls -l $(clippy paste)

# You can also use the command without the paste argument
ls -l $(clippy)
```

This will list the content of the path in the clipboard.

## Donations

If you would like to donate or support the project, please contact me.

## Maintainers
- [Martin Petr](https://github.com/MartinGamesCZ)
