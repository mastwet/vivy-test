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

### One-click Windows start

Double-click `start.cmd` to open the interactive coding terminal in this
checkout with the Windows console set to UTF-8. It uses the existing
`C:\vt\debug` build and does not build automatically. If `Desktop\keys.txt` or
`Desktop\key.txt` exists, it reads three non-empty lines (gateway URL, model
ID, API key). It writes only the URL and model to the ignored
`.workspace\launch-home\settings.toml`, and passes the
key through `RIG_SECRET_GATEWAY` in process memory. Sessions use that isolated
`RIG_HOME`. Without a Desktop key file, the launcher uses the existing
`RIG_HOME\settings.toml` and `RIG_SECRET_GATEWAY` environment variable.

## Gateway and runtime settings

Edit `RIG_HOME/settings.toml` (default `~/.rig/settings.toml`):

```toml
[gateway]
base_url = "https://gateway.example/v1"
model = "model-id-as-the-gateway-names-it"
# Optional model facts, which Rig acts on:
dialect = "openrouter"            # Rig's option mapping for this gateway (needed for /effort)
context_window = 131072           # compaction keeps requests inside it
max_output_tokens = 8192
image_input = true                # images reach the model
reasoning = ["low", "medium", "high"]   # /effort levels; add "off" if it can stop
input_price = 0.5                 # USD per million tokens, both or neither
output_price = 1.5

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

Pi's session options work too: `--no-session` keeps no session, `--fork <id>`
continues a copy of one, `--session <id|path>` resumes one; `/clone` copies the
running session, `/import <file.jsonl>` loads an `/export` file into an empty
session, and `/tree` shows a session's branches. Project trust works as in Pi: the project's own skills, prompt templates and
`.rig/SYSTEM.md`/`APPEND_SYSTEM.md` load only once it is trusted (`--approve`
for one run, `/trust` or the start question to remember; `[trust] default` in
`settings.toml` is `ask`, `always` or `never`).

In the terminal view, `/resume`
alone opens the session picker; `/thinking` and `--thinking <level>` are Pi's
names for `/effort`, Shift+Tab cycles the reasoning level and Alt+P the model
(list more ids on the gateway with `models = [...]`), and `--export <id>
[output]` exports a session from the command line; Alt+Up then
Enter rewinds to an earlier message, and the branch left behind is summarized
into the fork (Pi's branch summary); `/fork` and `/agents` manage a session's
agents. `python scripts/sessions_tui.py <vivy>` checks these on Linux/macOS.

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
