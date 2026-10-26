# Provenance

Canonical source repository: https://github.com/anreonyr/sqware.git
Local source checkout: `source/`
Public source commit: `378d4d826fff70851dfcf08ace8cded4a55d0d2b`
Public commit branch: `codex/shuyra-execution-extraction` (PR35)
Local extraction branch: `codex/shuyra-schedule-extraction` (not published)
Extraction date: 2026-10-26

The source `crates/schedule` tree had 13 tracked files. Nine production Rust
files were copied byte-for-byte; one root manifest and three owner files (the
test `.gitignore`, test `Cargo.toml` manifest, and test `lib.rs`) were relocated
and minimally adapted as listed below. The three owner files already include
the owner manifest, so the totals are 9 production Rust + 1 root manifest + 3
owner files. No other production implementation, target artifact, or old root
lockfile was copied. `rust-toolchain.toml` was copied byte-for-byte from the
same public SHA; both lockfiles were generated normally for the new independent
workspaces, and no prior owner lock existed.

| # | Source path | Destination | Source SHA-256 | Destination SHA-256 | Adaptation |
|---|---|---|---|---|---|
| 1 | `crates/schedule/Cargo.toml` | `Cargo.toml` | `7e12bae651f69075e8f587fd0606335524fe8bce1fd539a823eb300a973e6309` | `feb5fdd152b9e8f70ad1758f8cff8345596e3f81465c1d5af26b3dc85c6d9830` | Explicit `name`/`version = "0.1.0"`/`edition = "2024"`; added `[workspace] resolver = "3"`; kept `lib test/bench/doctest = false`; no dependencies/features/build script. |
| 2 | `crates/schedule/src/compose.rs` | `src/compose.rs` | `529cfa39b821bb11fb804dc2fbbc3375f918d49dffa35f545d258bc4032eaf92` | same | Byte-identical. |
| 3 | `crates/schedule/src/dispatch.rs` | `src/dispatch.rs` | `66aab710f40eb31d024f2044c8accd02101a2524feb5f16d2ae7207ab8d10ceb` | same | Byte-identical. |
| 4 | `crates/schedule/src/graph.rs` | `src/graph.rs` | `b03e90d43cdd9d4ad80bf4b9e0080a0905dcb7dd546db8dc780223ce6bf7be8b` | same | Byte-identical. |
| 5 | `crates/schedule/src/lib.rs` | `src/lib.rs` | `c5d4b42f9b8a15356c4cb1e3a183925c6bc8b7845e7b2ee3de8039d2187a35a3` | same | Byte-identical. |
| 6 | `crates/schedule/src/mod.rs` | `src/mod.rs` | `76d270473b9eb84493ab681c4752301f239dc83ff27ef24e83bb9b208e63a8d6` | same | Byte-identical. |
| 7 | `crates/schedule/src/plan.rs` | `src/plan.rs` | `88dc48b098f07a853efe87dca1f78175f9f4fe00b37e4afd7ec8bfad1c77fa19` | same | Byte-identical. |
| 8 | `crates/schedule/src/resource.rs` | `src/resource.rs` | `f2312716601f7ade5dc3ba197f3b661d56f6eaac55e89ccaea6ff76cfc731b08` | same | Byte-identical. |
| 9 | `crates/schedule/src/sequence.rs` | `src/sequence.rs` | `f3c72051648903b8a2a5a30e25588f0570759ced143599f849ea67eac586d565` | same | Byte-identical. |
| 10 | `crates/schedule/src/system.rs` | `src/system.rs` | `c123380aed1bbb8c2c418b6e76198b66f6071cf79226e578666bcdc988023c85` | same | Byte-identical. |
| 11 | `crates/schedule/src/tests/.gitignore` | `tests/host/.gitignore` | `07d64eb09a56c853b7853a47276799b38d7d6d25a4314b0c1abc10432c214e17` | `306fd52e74fca6746e12acc750f232de15e71da917756a1270d74c91f5eb7368` | Kept `/target/`; removed `/Cargo.lock` so the host lock is tracked. |
| 12 | `crates/schedule/src/tests/Cargo.toml` | `tests/host/Cargo.toml` | `2a3c56bc0197270b87f14459640e87548119905a6f537f9cf77bbb43b8377e74` | `7af55d611e50f91dff94c1e9beef7ba07ec4ff217145e486a4bf4499e994ff07` | Kept independent `[workspace]`, `schedule-tests`, and `[lib] path = "lib.rs"`; added `syn = { version = "2", features = ["full", "visit"] }` as a dev-dependency only. |
| 13 | `crates/schedule/src/tests/lib.rs` | `tests/host/lib.rs` | `d5c78ee7e5e8546f9c9c656ca3e8197f64c8f4cb6169778335a866f67dab3aaa` | `664092ba909cf9b8ad037ca9bb02d6f4093f3f23ce76ed4cb9acddd075b2946a` | Only `#[path]` changed from `../mod.rs` to `../../src/mod.rs`, plus `#[cfg(test)] mod shape;`. Original 36 test bodies and the thread-local allocation counter are unchanged. |

## Owners and licensing

Git history for `crates/schedule` and the migrated shape source
(`programs/tests/system-shape/lib.rs`) records two existing identities:

- `anreonyr <anreonyr@gmail.com>`
- `anreonyr <88045926+anreonyr@users.noreply.github.com>`

The `crates/schedule` subtree history uses the first identity; the shape source
file's history includes both. These identities are recorded from the existing
source history, which remains available at the canonical repository URL above.
The source package and workspace declare no `authors` field; no new author
identity was added to any manifest. The source repository contains no `LICENSE`
file, and no license text or license metadata was added here.

## Scope

This provider is a library plus host-only mechanism tests. No unit, server, or
executable was added. The migrated `tests/host/shape.rs` test comes from the
same public SHA's
`programs/tests/system-shape/lib.rs::schedule_mechanism_has_no_domain_dependencies`,
its `References` visitor, and the recursive `references` helper. Its two
assertions and `UseTree` behavior are preserved; only repository-root
location/path adaptation and blank-line layout differ. The test scans the entire
`src` tree with `syn` and checks the two original predicates: no `protocol::`
path and no `protocol` in the root manifest.
