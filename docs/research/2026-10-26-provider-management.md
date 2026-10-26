# Provider management and plugin boundaries

Research checkpoint, 2026-10-26. This is an implementation recommendation for
owner review, not a replacement for DESIGN's gateway-first decision or a claim
that the proposed management commands exist. Provider management expands the
previously deferred direct-provider surface explicitly requested for research.

Keep Rig's execution path. Add management of configured provider instances,
their credentials and model catalogs over its existing Connector. Select
protocol and special-authentication code through the recipe; configure instances
of compiled adapters at runtime. A compatible gateway remains one such instance.

## Source revisions

| Project | Revision and inspected boundary |
| --- | --- |
| vivy-test | `737b5b6`, current gateway, recipe, model controls, launcher and process environment filtering |
| Rig | `55cd481a2421fd5da468b5bd6afc6ac876883599`, pinned catalog, Connector, provider registry, LoginPlugin, defaults and Connection |
| Pi | Current `d5b3de22d0e24a3e9c38a9410d3329b099717283`; the inspected Provider interface and model/custom-provider documents have no diff against the repository's Pi baseline `f321e5bdd26b3624bfb2ab182a334d0a5edb698a` |
| deepseek-harness | `22ddee3792402c3e70ec5f86a63c3e3e895a2f60`, current default branch: provider guide, LLM registry, pi-ai adapter, API-key and account routes |

Pi was cloned into ignored `.workspace/pi` and left detached at the handoff's
baseline. DSH files were read through GitHub; neither reference is a product
dependency. The evidence below is source inspection and the current generated
package's offline Cargo graph, not a new live-provider compatibility test.

## What the references establish

### Pi

The [model guide][pi-models] separates login, model selection, compatible
endpoints and executable provider extensions. `/login` and `/logout` manage
stored credentials; `/model` selects an available model. Session model choices
and startup defaults are separate. `models.json` supplies endpoint/model
configuration, and model overrides preserve the rest of a provider's catalog.

The [provider extension contract][pi-custom] supports both a complete native
Provider and a legacy configuration form. A [Provider][pi-provider] owns its
identity, authentication, synchronous model catalog, optional refresh and
credential-specific filtering, and request streaming. A new compatible endpoint
usually needs configuration; a new authentication flow or wire protocol needs
code. Registration can reuse an existing protocol rather than copying it.

Borrow the user's workflow, metadata overrides and registration model. Do not
import the Pi TypeScript extension runtime or pi-ai into the Python worker.

### deepseek-harness

The [LLM service][dsh-llm] is provider-neutral; adapters register named routes.
Its [source contract][dsh-registry] has separate adapter, configurable-provider
directory and discovery registrations. Duplicate routes are rejected, route
replacement is atomic, and registrations are disposed with their owning plugin.

The [pi-ai adapter][dsh-pi] serves many configured providers from one adapter
plugin. Built-in providers supply defaults; custom routes specify endpoint,
protocol and models. Configuration changes affect subsequent operations through
captured snapshots. An explicit credential reference that cannot resolve fails
instead of silently selecting an unrelated ambient key.

The [provider guide][dsh-guide] distinguishes third-party provider presets from
custom APIs and separates stored secrets from settings. Custom provider IDs are
stable because sessions and defaults reference them. Discovery assists editing;
manual model IDs remain available when an endpoint cannot list models.

The [API-key][dsh-key] and [account][dsh-account] plugins share a Messages
transport but own distinct authentication, model admission and account failures.
This demonstrates that transport, credential and commercial account semantics
need not be one large provider implementation. Their special account eligibility
and server-version gates are not general requirements for vivy-test.

### Rig and the current experiment

Rig already has [Catalog and Connector][rig-connector], with ModelSpec,
`Provide`, `SignIn`, `ProviderId::declare` and `Connector::provide`. Provide can
be a small closure returning an ErasedHandler. The existing GatewayPlugin uses
that seam, so a provider supplied by a recipe-selected Rust plugin is already
demonstrated by this repository's assembly and scripted-gateway checks.

The upstream [LoginPlugin][rig-login] includes provider API-key entry and
ChatGPT browser/device login with token refresh. The recipe deliberately omitted
it. Re-enabling it can recover useful built-in management, but does not create a
complete editor for multiple custom instances or arbitrary subscription logins.
Its saved-key helper does not replace a connector already registered by another
plugin, which also matters when preserving GatewayPlugin.

Current gaps are specific: one gateway endpoint/key, shared metadata across its
listed models, no provider-instance CRUD UI, no explicit provider-first picker,
no configuration reload for captured endpoint/key state, and no unified policy
for direct-provider credentials. `/model vendor/model` already exists.

## Recommended ownership

| Concern | Implementation owner |
| --- | --- |
| Streaming, tools, cancellation, usage, compaction and child model calls | Existing Rig execution path through Models and Connector |
| Provider instance ID, display name, adapter, endpoint, credential reference and configured models | One small typed configuration collection in user settings |
| Native protocols and provider request dialects | Reuse Rig clients and mappings; recipe selects owned adapter plugins |
| Model facts | Rig ModelSpec, built-in defaults plus per-model overrides and disclosed discovery facts |
| Credentials | One host-owned store/resolver; settings contain references, never resolved secrets |
| Provider/model selectors and status | Projection of registered adapters, configured instances and their actual catalog/status |

Start with one owned provider-management plugin and small typed adapter
constructors. Use existing Rig types at the call boundary; add no standalone
provider SDK, second model engine, universal Port registry or independent usage
and cancellation path. There must not be another hand-maintained availability
list beside the assembled runtime.

Distinguish an upstream vendor from a configured instance. Two accounts/endpoints
for the same vendor need different instance IDs, such as `company` and `personal`.
The canonical selection `company/org/model-1` identifies instance `company` and
upstream model ID `org/model-1`; the request sends the latter unchanged.

### Proposed user experience

- `/provider` opens configured instances with their model counts and credential
  state. Choosing one opens that instance's model picker.
- `/model` retains global search and the canonical `instance/model-id` argument.
- Provider management offers add/edit/remove, credential entry/removal and model
  editing. An explicit model-list refresh is separate from a connection test.
- Keep the current CLI's `--model instance/model-id`. A separate `--provider`
  convenience option can be added after specifying its interaction with model
  IDs that already contain `/`; it is not needed for the first working slice.
- New-session defaults, a resumed session's recorded choice and an explicit CLI
  override remain distinct. A missing/deleted provider produces a visible repair
  choice, without silently routing the session to a different account.

The following is a proposed runtime configuration shape, not currently accepted
syntax. Keep it in user settings; the recipe still selects the adapter code.

```toml
[providers.company]
adapter = "openai-chat"
base_url = "https://gateway.example/v1"
credential_env = "RIG_SECRET_COMPANY"

[[providers.company.models]]
id = "org/model-1"
context_window = 131072
max_output_tokens = 8192
```

Preserve `[gateway]` as a legacy input during migration, mapping it into the same
management/execution path. Do not retain two independently editable stores of
the same endpoint. Update start.ps1, whose current credential-file bootstrap
writes `[gateway]`, with that migration rather than breaking existing starts.

## What plugin support means here

| Extension | Rebuild needed | Suitable mechanism |
| --- | --- | --- |
| Another endpoint/model list using an already compiled protocol | No | Add a provider-instance configuration |
| Another vendor preset over a compiled protocol | No if data-only; yes if it adds code | Configuration or a small recipe-selected preset plugin |
| A genuinely new wire protocol or special authentication implementation | Yes | Rust plugin crate registering through Rig's existing seams |
| Independently downloaded native plugin loaded while the program runs | A separate architecture | Stable FFI, WASM or a process boundary and explicit lifecycle contracts |

The last row is technically possible, but is not the current recipe model. The
[Rust ABI has no stability guarantee][rust-abi]; a Rust trait in a shared library
is not automatically a stable external plugin API. Introduce that boundary only
for a concrete hot-install requirement.

There is an important compilation limit: Rig's [ordinary provider modules][rig-modules]
are part of rig-core, and its pinned manifest has no per-vendor feature gates.
Omitting a local provider wrapper excludes that wrapper's crate/registration;
it does not prove that built-in provider source was excluded. Separate upstream
crates such as rig-bedrock and rig-vertexai can have a real crate boundary.
Claiming complete per-vendor source exclusion would require upstream changes or
a maintained fork, neither justified by this management request alone.

The current Connector defaults to the whole built-in catalog. For deliberate
recipe-selected availability, initialize an empty catalog and populate it from
selected adapters/instances using existing catalog rows. Merely hiding entries
in a picker would leave other selections available through the CLI. This bounds
registered capabilities; it does not strengthen the source-exclusion guarantee.

## Integration details that affect correctness

1. GatewayPlugin captures its key and endpoint during plugin construction, and
   Rig Connection retains its spec/handler. Changing settings or registering a
   new connector alone does not update existing connections. Capture one
   configuration generation per operation; reconnect affected idle agents or
   apply the change at the next model-request boundary. In-flight work keeps its
   original endpoint, credential and metadata.
2. Explicit credential references must fail when missing. Login, logout, API-key
   fallback and subscription authentication need one defined policy. Account
   token rejection must not delete a newer login or silently switch billing.
3. The current builder and shell/Python process helper filter RIG_SECRET_*
   variables. Standard names such as ANTHROPIC_API_KEY are not covered by that
   filter. Supporting those names requires filtering the resolved credential
   variables across every owned subprocess path, with tests using fake values.
   Rust plugins remain trusted process code, not a secret sandbox.
4. Model facts belong to individual models: context/output limits, tools,
   images, reasoning and prices can differ within a provider. Unknown facts
   stay unknown; a model-list endpoint is not proof of tool/image/reasoning
   compatibility. Preserve manual entries and user overrides when discovery
   fails. Any remote cache must distinguish endpoints and credentials.
5. Avoid two startup-default owners: the omitted Rig DefaultsPlugin and the
   current gateway initializer would need coordinated ordering and precedence.
   Session restore must retain its recorded provider/model choice.

## Suggested delivery order and acceptance

First deliver API-key and keyless-compatible instances, the provider-first TUI,
per-model facts, management persistence, migration from `[gateway]`, and reuse
of the required Rig protocols. Keep custom providers editable even while their
credentials are missing so they can be repaired. This is a recommended first
slice; the owner's OAuth scope preference remains to be settled.

Then add subscription authentication as provider-specific plugins, reusing
upstream implementations where they actually match. Verify browser/device
cancellation, refresh locking, stale-token rejection, logout and resumed calls
on native Windows. Other providers' OAuth support cannot be inferred from the
presence of one working ChatGPT authenticator.

Model discovery and metadata refresh can follow once manual configuration works;
preserve usable local data during network failure. A provider catalog alone is
not a reason to add a provider marketplace or automatic package installation.

Acceptance should cover two instances serving the same model ID with different
fake credentials, unchanged wire IDs, add/edit/remove and invalid configuration,
per-model reasoning/context differences, persistence and session restore,
configuration changes during streaming, tool calls and cancellation, failed
discovery, and absence of fake credentials in child environments and logs.
Recipe variants must demonstrate removal of an owned adapter dependency and
registration at its actual crate boundary. Actual selected gateways/providers
still need separate streaming/tool-call verification before compatibility claims.

[pi-models]: https://github.com/earendil-works/pi/blob/d5b3de22d0e24a3e9c38a9410d3329b099717283/packages/coding-agent/docs/models.md
[pi-custom]: https://github.com/earendil-works/pi/blob/d5b3de22d0e24a3e9c38a9410d3329b099717283/packages/coding-agent/docs/custom-provider.md
[pi-provider]: https://github.com/earendil-works/pi/blob/d5b3de22d0e24a3e9c38a9410d3329b099717283/packages/ai/src/models.ts#L158
[dsh-llm]: https://github.com/deepseek-ai/deepseek-harness/blob/22ddee3792402c3e70ec5f86a63c3e3e895a2f60/packages/llm/llm/README.md
[dsh-registry]: https://github.com/deepseek-ai/deepseek-harness/blob/22ddee3792402c3e70ec5f86a63c3e3e895a2f60/packages/llm/llm/src/index.ts#L208
[dsh-pi]: https://github.com/deepseek-ai/deepseek-harness/blob/22ddee3792402c3e70ec5f86a63c3e3e895a2f60/packages/llm/llm-pi-ai/README.md
[dsh-guide]: https://github.com/deepseek-ai/deepseek-harness/blob/22ddee3792402c3e70ec5f86a63c3e3e895a2f60/docs/user/guide/providers.md
[dsh-key]: https://github.com/deepseek-ai/deepseek-harness/blob/22ddee3792402c3e70ec5f86a63c3e3e895a2f60/packages/llm/llm-deepseek-api-key/README.md
[dsh-account]: https://github.com/deepseek-ai/deepseek-harness/blob/22ddee3792402c3e70ec5f86a63c3e3e895a2f60/packages/llm/llm-deepseek-account/README.md
[rig-connector]: https://github.com/0xPlaygrounds/rig/blob/55cd481a2421fd5da468b5bd6afc6ac876883599/crates/rig-core/src/catalog/connector.rs
[rig-login]: https://github.com/0xPlaygrounds/rig/blob/55cd481a2421fd5da468b5bd6afc6ac876883599/plugins/rig-basics/src/login.rs
[rig-modules]: https://github.com/0xPlaygrounds/rig/blob/55cd481a2421fd5da468b5bd6afc6ac876883599/crates/rig-core/src/providers/mod.rs
[rust-abi]: https://doc.rust-lang.org/reference/items/external-blocks.html#r-items.extern.abi.rust
