---
id: gateway
type: module-design
status: draft
title: Provider instances and model catalog (`plugins/vivy-gateway`)
parent: architecture
depends-on: [process]
---

## Responsibility

Make configured endpoints usable as Rig models. `ProvidersPlugin` installs one
catalog and one Connector over every provider instance the user added: OpenAI
(Chat Completions) and Anthropic (Messages) protocols behind Cargo features, an
instance's `protocol` choosing which Rig wire serves it.

The instances are user-owned data in `RIG_HOME/settings.toml`: `config` parses and
maps them onto Rig's `ModelSpec`, so a model's context window, output limit, image
support, reasoning levels and prices are facts Rig acts on (compaction, images,
`/effort`) rather than display text. Presets reuse Rig's built-in metadata and
option mappings. `store` performs atomic settings edits and keeps saved API keys
in a separate file store, outside settings. `ui` edits instances and models on
Rig's picker through an explicit draft, so an external file edit conflicts instead
of being silently overwritten. `redact` wraps the HTTP client for one reason only:
to strip the resolved credential from a provider's error cause while keeping the
cause.

The legacy `[gateway]` table remains readable as `gateway/<model>` and is migrated
into the instance collection on save.

## Boundary

Depends on Rig's provider/catalog crates and on `process` (it registers the
credential variable names other components filter). It does not implement a
protocol: Rig's vendor wirings do the requests, and this crate supplies
configuration, selection, storage and error hygiene.

Decisions that are not visible in the code:

- Only the **user's** settings are read, never a project's `.rig/settings.toml`, so
  a cloned repository cannot redirect the credential.
- A model ID is sent to the endpoint unchanged; there is no invented rewriting and
  no provider catalog beyond what the user configured.
- An explicit credential source never falls back to a different account.

Invariants: no credential value appears in settings, a session, a log or an error
payload; a removed or invalid instance is reported rather than guessed around; an
invalid file retains the last valid catalog.
