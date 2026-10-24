# vivy-test

A Rust coding-agent experiment: **one YAML recipe selects components, one build
assembles a runnable agent, and model access goes through a gateway first.**

The purpose is to test Vivy's assembled-species idea with substantially less
engineering overhead. Use Rig for reusable implementation and Pi coding-agent
as the functional reference. Favor a working agent and short iteration cycles.

## Status

The first slice exists: a recipe builder (`crates/vivy`), the baseline recipe
(`recipes/coding-agent.yaml`) and one Vivy plugin (`plugins/vivy-gateway`).
It reuses Rig's harness and plugins; see [DESIGN.md](docs/DESIGN.md) for the
route, what has been run, and what is still unverified. It is built and run on
Linux; Windows has not been tried.

## Use

```sh
cargo run -p vivy -- build recipes/coding-agent.yaml   # prints the agent binary
```

Runtime settings are Rig's `RIG_HOME/settings.toml` (default `~/.rig`); the
gateway reads its table there and its key from the agent secret variable:

```toml
[gateway]
base_url = "https://gateway.example/v1"
model = "model-id-as-the-gateway-names-it"
```

```sh
export RIG_SECRET_GATEWAY=...    # never stored in the repository
.generated/target/debug/coding-agent --model gateway/<model>            # terminal
.generated/target/debug/coding-agent --model gateway/<model> -p "task"  # one prompt
```

Change the recipe and rebuild to change what is compiled in.

## Start here

1. Read [AGENTS.md](AGENTS.md) for the short working rules.
2. Read [the decisions and assembly explanation](docs/DESIGN.md).
3. Use [the Pi baseline](docs/PI-BASELINE.md) to track functional coverage.
4. Follow [the handoff](docs/HANDOFF.md) for reference clones and next actions.

**Clone Pi and Rig into this repository's ignored `.workspace/` directory.
Do not clone, vendor, depend on, or modify the original Vivy repository.**
That project is undergoing separate recovery work. This experiment must be
understandable and buildable without it; its useful assembly ideas are summarized
here. Reference-clone commands and inspected revisions are in the handoff.

## What matters

- A recipe changes which optional plugin crates enter the generated Cargo
  dependency graph and which constructors run.
- Rig components can be exposed as small Rust/Vivy plugins, using existing
  upstream interfaces where practical.
- One model gateway supplies the first working model integration.
- The first usable agent can take a task, stream a response, read/edit files,
  execute a command, and continue a conversation.
- Pi's wider capabilities remain the coverage target, delivered incrementally
  with Rust equivalents. Existing Pi TypeScript extension compatibility is not
  required.

Species hashes, a standalone plugin SDK, attestation, a plugin marketplace, and
Vivy's production governance machinery are excluded from the initial experiment.
Ordinary Cargo locking and source revision pins are still useful dependency
management; they are not a species identity system.

No gateway vendor or operating-system compromise has been selected. The current
development environment is Windows; Rig's inspected launcher documents Linux and
macOS support. Resolve that practical issue before building a large abstraction.
