# CONTEXT

Domain language for oh-my-harness. This is a glossary, not a spec: names here
are the names code, CI, issues, tests and ADRs must use.

## Product

- **oh-my-harness** — the project and the crates.io package name.
- **omh** — the command users type. One binary, one command: every artifact
  that installs a program installs it as `omh`.
- **harness** — the `.agents/` tree (`skills/`, `agents/`, `commands/`) that
  `omh install` populates, either in a project root or globally under `~/.agents`.

## Artifacts

- **target** — one of `darwin-arm64`, `darwin-x64`, `linux-x64`, `linux-arm64`,
  `win32-x64`. The vocabulary is the npm convention (ADR-0001): os
  `darwin`/`linux`/`win32`, cpu `arm64`/`x64`. The old `macos-arm64` /
  `linux-amd64` asset names are retired; historical release assets keep their
  old names.
- **binary** — a `cargo build --release` output for one Rust target triple,
  named `omh` (`omh.exe` on `win32-x64`). `<version>` is bare semver; the
  release _tag_ is `v<version>`.
- **release asset** — a file attached to a GitHub Release. The release
  publishes one **tarball asset** per **brew target** plus a Windows zip.
- **tarball asset** — `omh-<version>-<brew target>.tar.gz`, whose only member
  is the binary named `omh`. Built in CI with `tar czf <asset> omh`.
- **brew target** — the **target** subset the formula covers: `darwin-arm64`,
  `darwin-x64`, `linux-x64`, `linux-arm64`. `win32-x64` is never a brew
  target; Homebrew has no Windows.

## Channels

- **channel** — a way to install omh: **Homebrew** (`omh`, macOS and Linux), or
  **cargo** (`cargo install oh-my-harness`, all platforms). A change to one
  must not silently break another's naming.
- **source repository** — `xminihub/oh-my-harness` on GitHub: the code, the
  releases, the binaries.
- **tap** — what Homebrew calls `xminihub/tap`, i.e. `brew tap xminihub/tap`.
  Its Git repository is _not_ the source repository: it is
  `xminihub/homebrew-tap`, the org-wide tap shared by every xminihub tool. The
  old per-project tap `xminihub/homebrew-oh-my-harness` is superseded and stays
  untouched.
- **formula** — `Formula/omh.rb` in the tap. It is generated, never
  hand-edited: the source of truth is `tools/omh.json` plus
  `templates/omh.rb.tpl` in the tap, rendered and verified by that repository's
  `bump.yml`. This repository only publishes the tarball assets the formula
  pins.
