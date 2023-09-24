# shiori-cli

Manage your [Shiori](https://github.com/go-shiori/shiori) bookmarks conveniently from the CLI

## Available features

| Feature          | State |
| ---------------- | :---: |
| Log in           |  ✅   |
| Log out          |  ✅   |
| Get bookmarks    |  ❌   |
| Add bookmarks    |  ✅   |
| Edit bookmarks   |  ❌   |
| Delete bookmarks |  ❌   |
| Get tags         |  ✅   |
| Rename tags      |  ❌   |

## Installation

Just download and extract the appropriate binary for your system (if yours is missing, let me know in an [issue](https://github.com/cbe/shiori-cli/issues/new)) from the [latest release](https://github.com/cbe/shiori-cli/releases/latest) and put it in your PATH.

Alternatively you can build it yourself with

```sh
cargo build --release
```

## Usage

By simply running `shiori-cli` or `shiori-cli help` you get an overview of the available commands, running `shiori-cli help <COMMAND>` will print more information about the given command and it's available arguments.

Before you will be able to do anything useful with this tool you need to sign in to any Shiori instance with `shiori-cli login`.
