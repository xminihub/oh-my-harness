# 1. Distribution: migrate Homebrew to the org tap

Date: 2026-11-18

Status: accepted. Follows `xminihub/limulin` issue #3, which moved `lim` to the
same arrangement and decided the org-wide tap that ADR-0001 there records. This
document is `oh-my-harness`'s side of that contract.

## Context

`oh-my-harness` owned a Homebrew formula it rendered itself. `.github/workflows/release.yml`
had an `update-homebrew-formula` job that `cat`'d `Formula/omh.rb` into the
per-product tap `xminihub/homebrew-oh-my-harness`, computing a sha256 from a
single darwin asset and pushing it with a content-write PAT
(`HOMEBREW_TAP_TOKEN`). Three problems, the same ones lim's issue #3 named:

- The formula pinned one target, `omh-macos-arm64.tar.gz`, while the release
  built binaries for Linux and Windows too — Intel Macs and every Linux user
  had no installable formula.
- The asset names carried no version (`omh-linux-amd64.tar.gz`), so republishing
  a version overwrote bytes the formula's pin no longer described.
- A PAT with write access to another repository pushed formula contents from
  this one, and the only thing proving the pinned URL matched a published asset
  was a heredoc that had to be edited in step with the release.

The org tap `xminihub/homebrew-tap` already renders, verifies and commits
formulas for every tool from `tools/<name>.json` + `templates/<name>.rb.tpl` +
`scripts/render.mjs`, gated by `brew style`/`brew fetch --all-platforms`/
`brew audit --strict --online`. `lim` is onboarded; `omh` is not.

## Decision

- **The tap owns the formula; this repository owns only the artifacts it
  pins.** `update-homebrew-formula` is deleted. A new `tap` job dispatches the
  tap's `bump.yml` with `name=omh`/`version=…`, polls that run to a conclusion
  with a timeout, and fails the release when the run fails. No token in this
  repository ever writes tap contents.
- **Per-target brew tarball assets, version in the name.** The release
  publishes `omh-<version>-<brew target>.tar.gz` for `darwin-arm64`,
  `darwin-x64`, `linux-x64` and `linux-arm64`, and `omh-<version>-win32-x64.zip`
  for Windows (never a brew target). The old unversioned `omh-linux-amd64.tar.gz`
  / `omh-macos-arm64.tar.gz` / `omh-windows-amd64.zip` names are retired;
  historical releases keep theirs. Asset naming is the cross-repo contract —
  `bump.yml` refuses to render a target with no matching asset.
- **Two build jobs cover four brew targets by cross-compilation.** A macOS job
  builds `aarch64-apple-darwin` (native on the Apple-Silicon runner) and
  `x86_64-apple-darwin` (against the universal SDK); a Linux job builds
  `x86_64-unknown-linux-gnu` (native) and `aarch64-unknown-linux-gnu` (via
  `gcc-aarch64-linux-gnu`). Windows stays one job.
- **Each tarball has a single member, `omh`** (`bin.install "omh"` reads
  exactly that). README/LICENSE are no longer bundled into the archive.
- **`omh` grew a `--version` flag** (`#[command(version)]` in `src/main.rs`).
  The tap template's `test do` block runs `omh --version` and matches the
  formula version against it; without the flag `brew test omh` fails.
- **Credentials are split by direction.** `HOMEBREW_TAP_TOKEN` and every
  reference to it are deleted; the `tap` job authenticates with the org-level
  `TAP_TRIGGER_TOKEN` (fine-grained PAT, Actions read/write on the tap) that
  lim's migration minted, so onboarding `omh` adds no new secret.
- **Onboarding lands in tap order.** The tap-side PR (`tools/omh.json` +
  `templates/omh.rb.tpl`) is a prerequisite: it must land before the first
  release that dispatches a bump for `omh`, or `bump.yml` errors "no
  tools/omh.json in this tap".
- **README points at the org tap**: `brew install xminihub/tap/omh`. The
  per-product tap `xminihub/homebrew-oh-my-harness` is left in place, not
  archived or deleted — matching lim.

## Out of scope

- Adding signing/notarization for the darwin binaries. `oh-my-harness` ships
  unsigned today; a brew-installed unsigned binary works with a Gatekeeper
  quarantine caveat, and signing is orthogonal to the tap migration.
- `install.sh`, `SHA256SUMS` and a latest-version file. The non-brew channel is
  `cargo install`, so those lim artifacts are not introduced here.
- The tap repository's own machinery (bump workflow, gates, backfill, audit CI)
  — owned by `xminihub/homebrew-tap`.

## Consequences

- A tool's release pipeline and its formula can no longer change in one commit;
  the contract between them is an asset name, checked by the tap's gates rather
  than a heredoc here.
- The formula is generated and never hand-edited; hand-editing `Formula/omh.rb`
  is futile because the next bump overwrites it.
- Linux and Intel-Mac users get a real brew path for the first time; a
  Homebrew-only macOS arm64 pin is no longer the only formula.
- A tap bump failure now stops a release, rather than a green release leaving a
  stale formula behind. Recovering from a failed bump means re-dispatching the
  tap's `bump.yml`, not editing anything here.
- `linux-arm64` costs a cross-linker in CI; if that build ever regresses, the
  release fails at the non-empty-asset guard before any bump is dispatched.

## Rejected alternatives

- **Keep the formula here and just add more platform blocks.** Leaves this
  repository rendering Ruby that duplicates the tap's job, and keeps the
  content-write PAT.
- **Publish a bare binary per target alongside each tarball** (lim's full
  artifact set). Unneeded: the only consumer that needs a raw download uses
  `cargo install`.
- **A bespoke deterministic tarball packer** (uid/gid/mtime pinned, as lim
  does). The tap re-checksums whatever asset is published at bump time, so
  byte-for-byte republish determinism is not required for correctness; plain
  `tar czf` keeps the Rust repository free of a packaging toolchain.
- **Rename only the brew assets, keep the old names too.** Doubles the published
  asset set and keeps a stale, version-less naming contract alive.
