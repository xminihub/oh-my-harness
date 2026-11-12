[![CI](https://github.com/xminihub/oh-my-harness/actions/workflows/ci.yml/badge.svg)](https://github.com/xminihub/oh-my-harness/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/oh-my-harness.svg)](https://crates.io/crates/oh-my-harness)
[![Crates.io Downloads](https://img.shields.io/crates/d/oh-my-harness.svg)](https://crates.io/crates/oh-my-harness)
[![Homebrew](https://img.shields.io/github/v/release/xminihub/oh-my-harness?label=homebrew&logo=homebrew)](https://github.com/xminihub/homebrew-oh-my-harness)

# oh-my-harness

## Installation

### Homebrew (macOS)

```sh
brew install xminihub/homebrew-oh-my-harness/omh
```

### Cargo (all platforms)

```sh
cargo install oh-my-harness
```

## Usage

```sh
omh install [repo] [-d <project-dir>] [-g|--global] [--force]
```

`omh install` installs everything a repo ships into your harness's `.agents` directories:

- `skills/<name>/` folders → `<project>/.agents/skills/` (or `~/.agents/skills/` with `-g`)
- top-level `agents/*.md` files → `<project>/.agents/agents/` (or `~/.agents/agents/` with `-g`)
- top-level `commands/*.md` files → `<project>/.agents/commands/` (or `~/.agents/commands/` with `-g`)

Installs are project-level by default, rooted at the current directory or the project passed to `-d`; `-g`/`--global` installs into `~/.agents` instead (`-g` and `-d` conflict).
`repo` is a git URL or local directory, and defaults to the oh-my-harness repo. Existing skills, agents, and commands are skipped unless `--force` overwrites them. A repo must ship at least one of `skills/`, `agents/`, or `commands/`; a missing directory just installs nothing of that kind.
