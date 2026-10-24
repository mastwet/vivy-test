# vivy-test

A Rust coding-agent experiment: **one YAML recipe selects components, one build
assembles a runnable agent, and model access goes through a gateway first.**

Use Rig's runtime and terminal and Pi's coding-agent workflow as the reference.
The Windows-native species includes file/search tools, Git Bash, sessions,
prompt templates, and an optional auxiliary Python workbench. Current model
integration checks use a scripted loopback gateway; a real-provider smoke is
still pending.

## Build and run on Windows

Prerequisites: Rust 1.97.1 and MSVC C++ build tools, Git for Windows, and Python
3.11+ when the recipe includes Python. Run these commands from this checkout:

```powershell
# Short paths avoid MSVC path/command-line limits in native dependencies.
$env:CARGO_TARGET_DIR = 'C:\vt'
$env:CARGO_PROFILE_DEV_DEBUG = '0'
cargo run -p vivy -- build recipes/coding-agent.yaml

& C:\vt\debug\vivy.exe                    # interactive terminal
& C:\vt\debug\vivy.exe -p 'Fix this bug'   # one task
& C:\vt\debug\vivy.exe --resume <id>       # resume a saved session
& C:\vt\debug\vivy.exe --json              # Rig's JSON protocol
```

`vivy` starts the already-built baseline species. Build explicitly after a
recipe or source change. This is a checkout-local development tool, with no
installer or automatic rebuild. Keep `CARGO_TARGET_DIR` consistent between build
and launch. Launching preserves the current working directory; resuming uses
the session's saved directory. Add the build directory to your PATH if desired.

## Gateway and runtime settings

Edit `RIG_HOME/settings.toml` (default `~/.rig/settings.toml`):

```toml
[gateway]
base_url = "https://gateway.example/v1"
model = "model-id-as-the-gateway-names-it"

# Optional overrides; defaults discover Git for Windows and use `python`.
# [shell]
# executable = "C:/Program Files/Git/bin/bash.exe"
# [python]
# executable = "C:/path/to/python.exe"
```

```powershell
$env:RIG_SECRET_GATEWAY = '...'  # never store the key in this repository
& C:\vt\debug\vivy.exe
```

An explicit `--model` takes precedence; a resumed session keeps its model;
otherwise the configured gateway model is selected. Its raw model ID is sent
unchanged. Build and execution children do not inherit `RIG_SECRET_*`.

`/new`, `/resume <id>`, `/session`, and `/quit` use Rig's terminal and session
mechanisms. A session switch in print mode returns Rig's exit code 75 rather
than replaying the prompt. Rig's JSON interface is not Pi RPC compatibility.

## Optional Python workbench

The recipe's `python` component adds `python_repl(code, timeout_secs?)` alongside
the existing tools. Variables/imports and top-level `await` persist across calls
for one active agent. The default timeout is 60 seconds.

Cancellation or timeout stops the worker and its descendants and resets
variables. Ordinary Python exceptions retain existing variables. Exit, session
switching, and resumed processes start a fresh namespace. There are no state
snapshots, automatic package installations, or background task service. Python
and shell run with the user's permissions; neither is a security sandbox.

Remove the Python component and rebuild to exclude its crate and embedded worker.
All component selection remains in [the recipe](recipes/coding-agent.yaml).

## Project notes

- [AGENTS.md](AGENTS.md): lean development rules.
- [DESIGN.md](docs/DESIGN.md): assembly direction, implementation and verification.
- [PI-BASELINE.md](docs/PI-BASELINE.md): wider functional coverage target.
- [HANDOFF.md](docs/HANDOFF.md): reference clones and source navigation.

Clone Pi and Rig into ignored `.workspace/` as described in the handoff. **Do
not clone, vendor, depend on, or modify the original Vivy repository.** It is
under separate recovery work. Its assembly ideas are summarized in this project.

Keep the experiment small: no species hashes, standalone plugin SDK,
attestation, plugin marketplace, Studio lifecycle, or Pi TypeScript extension
compatibility. Ordinary Cargo locking and source pins remain dependency
management. Pi's wider feature set is an incremental target, not a first-run gate.
