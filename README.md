# frec

`frec` is a frecency-based tool for managing file weights, inspired by [zoxide](https://github.com/ajeetdsouza/zoxide).

[![license](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

## Getting Started

![frec-v0.1.0.mp4](https://github.com/user-attachments/assets/c158fcc1-a43a-4120-bbcc-53b14b3c683c)

```bash
# Add a file to track (first time: rank=1, within 1 hour: *4 = score 4)
frec add test.md

# Query files (sorted by score, higher = more frequent)
frec query

# Query with scores shown
frec query -s

# Add again to increase rank
frec add test.md
frec query -s   # score: 8 (rank=2, *4)

# Use e command (after running: eval "$(frec init zsh)")
e test.md       # open file and track it
e               # interactive fzf selection
```

## Table of Contents

- [Introduction](#introduction)
- [Install](#install)
- [Usage](#usage)
- [Features](#features)
- [Shell Integration](#shell-interaction)
- [Yazi Integration](#yazi-integration)
- [License](#license)

## Introduction

What if your terminal could remember not just where you've been, but **what you've done**?

[zoxide](https://github.com/ajeetdsouza/zoxide) changed how we navigate directories—its frecency algorithm learns your habits and makes jumping between folders feel like magic.

But here's the thing: **why should this magic be limited to `cd`?**

The frecency algorithm—tracking both *frequency* and *recency*—is surprisingly universal. It works for anything you use repeatedly. That's what `frec` does: it extracts this intelligence and applies it to arbitrary strings, especially file paths.

Pipe it into fzf, and suddenly you have:

- **Smart file editing** — your most-used files appear first
- **Project hopping** — jump between projects you actually work on
- **Command recall** — surface commands you use most often
- **Anything you can dream up**

### Meet frec_editor

`frec_editor` is a shell script that proves what's possible with frec + fzf.
Here's a rough idea of how it works:

```bash
frec query --table=editor | fzf | xargs -I{} frec add --table=editor {} && $EDITOR {}
```

By combining frec with fzf via a pipe, you can intelligently choose which files to edit based on your personal usage patterns.

## Install

```bash
# Install frec
cargo install frec

# Install shell integration and frec_editor
frec install
eval "$(frec init zsh)"  # or bash
```

## Usage

### Add a path

Add or update a file's weight in the database:

```bash
frec add ~/test.md
frec add --table=editor ~/vimrc
frec add --table=project ~/code/main.rs
```

### Query paths

Query files from the database:

```bash
frec query                          # list all files in default table
frec query test                     # filter by keyword
frec query -s                       # show scores
frec query --table=editor           # query from specific table
frec query --table=editor -s        # show scores from specific table
```

### Shell initialization

Generate shell initialization script:

```bash
source <(frec init bash)
source <(frec init zsh)
```

## Features

### Multi-table Management

`frec` supports multiple tables to organize different types of files:

- `default` table: for general files
- Custom tables: `--table=editor`, `--table=project`, etc.

### Frecency Algorithm

The ranking algorithm considers both frequency and recency (same as zoxide):

- **rank**: number of times the file has been accessed via `frec add`

| Last access time | Frecency |
|------------------|-----------|
| Within 1 hour    | rank * 4  |
| Within 1 day     | rank * 2  |
| Within 1 week    | rank / 2  |
| Otherwise        | rank / 4  |

### JSON Storage

All data is stored in `~/.local/share/frec/db.json`:

```json
{
  "tables": {
    "default": {
      "entries": {
        "/home/yang/test.md": {
          "rank": 3,
          "last_accessed": 1234567890
        }
      }
    }
  }
}
```

## Shell Interaction

### Zsh

Add the following to your `~/.zshrc`:

```bash
source <(frec init zsh)
```

Then you can:
- Press `Ctrl+e` to open interactive file selection with fzf
- Or type `e` and press Enter

After selecting a file, it will be added to the `editor` table and opened with `$EDITOR`.

### Bash

Add the following to your `~/.bashrc`:

```bash
source <(frec init bash)
```

Press `Ctrl+e` to open interactive file selection with fzf.

### Usage

The `e` command supports two modes:

- **Interactive mode** (no arguments): Press `Ctrl+e` or type `e` and press Enter to open fzf for file selection
- **Direct mode** (with path): `e ~/code.cpp` directly opens the file and trains its weight

Examples:

```bash
e                # interactive file selection with fzf
e ~/test.md      # directly open file and increase its weight
e $HOME/main.rs  # support variable expansion
```

Each time you open a file through `e`, its frequency score increases, making it rank higher in future interactive selections.

## Yazi Integration

You can integrate frec with Yazi file manager to track file access frequency.

### Setup

1. Ensure `frec_editor` is in your PATH (already installed via `frec install`)

2. Add the following to your `~/.config/yazi/yazi.toml`:

```toml
[opener]
edit = [
    { run = 'frec_editor "$1"', block = true, desc = "Edit with frec", for = "linux" },
]
```

3. Restart Yazi

Now when you open a file in Yazi, it will automatically be added to the `editor` table with increased frequency score.

## License

[MIT](./LICENSE) © m1dsolo
