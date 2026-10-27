---
id: builder
type: module-design
status: draft
title: Recipe builder and launcher (`crates/vivy`)
parent: architecture
depends-on: [process]
---

## Responsibility

Turn one recipe into one built agent, then start it.

`recipe.rs` parses and checks the recipe before Cargo sees it: unknown fields
(quartered with the offending line), duplicate components or plugin types, empty
components, malformed names and type paths, a plugin path outside its component's
crate, two sources for one crate, features for an unselected crate. Whether a type
exists and is a plugin is left to rustc, which names the missing type at the
generated line.

`generate.rs` writes `.generated/<name>/Cargo.toml` and `src/main.rs` in Rig's own
launcher shape — the harness plugins, then `rig_harness::load::<T>` per plugin in
recipe order, then `rig_harness::run`. Generated files are marked and rewritten
only when their content changes. Every generated package shares one target
directory (`CARGO_TARGET_DIR`, else `.generated/target`), so changing the
selection rebuilds only what changed.

Reproducibility is a file pair: `<recipe>.Cargo.lock` beside the recipe is copied
into the package before a build, and the resolved lock is written back after it.
The recipe and its lock are the pin; nothing about a built agent depends on when
it was built.

`launch.rs` runs the built binary and takes Pi's session options itself
(`--no-session`, `--fork`, `--session`, `--export`), handing the rest of the
command line to the species; it follows Rig's session-switch protocol, including
its reload exit code. The checkout path is baked in at compile time, so a moved
tree needs `cargo clean -p vivy` before rebuilding.

## Boundary

Depends on `process` (credential filtering for the cargo and describe children),
`rig-harness-protocol`, and **nothing from `plugins/`**. Plugin crates reach a
build only as paths written into the generated manifest; they are never Rust
dependencies of the builder. That is what allows a plugin the recipe omits to be
absent from the built agent's graph rather than merely unused in it.

Owns the recipe's meaning, the generated package's shape, and the launch/session
translation. It does not own plugin behavior, the agent's terminal, its session
store, or runtime settings — those belong to Rig and to the selected plugins.

Invariants: plugin crates are built with default features off, so only what
selected components ask for compiles in; the recipe is the one selection source
(Rig's `plugins.toml` path is not used); a recipe error is reported before
compiling, and every other failure is left to the tool that detects it.
