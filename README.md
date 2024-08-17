# Clippy
Linux CLI tool for managing clipboard.

## Installation

1. Get the latest `.deb` package from the [releases](https://github.com/MartinGamesCZ/Clippy/releases) page.
2. Install the package using `sudo dpkg -i path_to_clippy.deb`.
3. You are now ready to use Clippy.

## Usage

Clippy can be used using the command `clippy`.

### Commands
- `clippy copy` - Copy the piped text to the clipboard.
- `clippy paste` - Paste the clipboard content to the terminal / pipe into another process/file.
- `clippy help` - Show help message (list of commands).
- `clippy` - Default command, action will be determined by the available pipes.

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

Or you can paste it into a file:

```bash
# Using paste command
clippy paste > file.txt

# You can also use the command without the paste argument
clippy > file.txt
```

This will paste the content of the clipboard into the file.

## Support us

This project is free and will always be. If you want to support us, you can donate to the project or give us a star on GitHub.
If you want to donate, contact me.

## Maintainers
- [Martin Petr](https://github.com/MartinGamesCZ)

## License

Use this tool as you wish, for free, even commercially. Just don't claim it as your own and don't blame me for anything. If you want to support me or the project, you can donate.
