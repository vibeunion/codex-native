# AgentConfig Distribution and Updates

## Current Follow-up - 2026-11-14

Parent: URL-import and PM-directed interaction work recorded below. Source:
the user's clarification, "通过 url 就能完成分发和同步更新". Reason: the prior
implementation and PM brief incorrectly reduced AgentConfig to selecting one
model. The historical implementation notes below are superseded by this section.

Scope: complete bundle retention, remembered HTTPS sources, manual update
checks, section diff/consent, optional model activation and non-destructive
source removal. Risk: medium; Native single writer, corrected read-only PM
review and one read-only security verifier. No commit, push, deployment,
release or actual default-handler change is authorized.

### Standard Versus Extension

Primary reference: `vibeunion/agentconfig/spec/SPEC.md`, Draft v1.5, read using
GitHub's API on 2026-11-14. The standard snapshot link remains:

```text
agentconfig://import?v=1&bundle=<base64url-envelope>
```

ACB explicitly excludes server storage and cross-device conflict resolution.
Its `src` identifies the producing client, not an update URL.

Volt's application-owned source layer accepts a public HTTPS URL returning a
raw ACB JSON envelope. Publishers replace that file at the same URL; recipients
check for updates from the source list. Copied source distribution links use
this **Volt extension**, not a standard ACB route:

```text
agentconfig://subscribe?v=1&url=<percent-encoded-https-source>
```

Other AgentConfig clients are not assumed to support it. The source list
distinguishes snapshot bundles from updatable sources (Volt extension).
There is no scheduled background application, cloud account, automatic merge,
OAuth refresh or source-signature verification.

### Import and Update Semantics

- Preview inventory and section changes, decrypt when required, then confirm.
- Retain the entire original envelope, all models, all sections, unknown fields
  and the original encrypted payload in the private per-workspace state.
- Accept model-free and multi-model bundles without requiring model activation.
- Keep current runtime settings and credentials by default. Activating a model
  is an optional explicit choice with model/endpoint/credential preview.
- Reopen saved bundles and sources after restarting.
- Fetching a candidate does not change the confirmed snapshot. Cancel, network
  failure, wrong password and failed persistence do not advance the baseline.
- Ignore formatting and `created` alone when deciding whether an update exists.
  Unknown envelope/public fields participate in comparison. Ciphertext changes
  conservatively count as updates. Payload contents stay hidden in the diff.
- Reject confirmation if the source was removed or its retained bundle changed
  after preview. Runtime local overrides are preserved unless the user
  explicitly selects an incoming model; there is no silent runtime merge.
- Persist the source and optional model preferences in one atomic write before
  changing live settings. Check time is distinct from import time.
- All desktop-state saves use a cross-instance lock plus a disk revision check;
  stale instances must reload rather than replace newer state. Legacy state
  without a revision is accepted as revision zero.
- Confirm source removal without clearing active settings or credentials.

Retention is not runtime activation. MCP definitions, skill enable/order
references, prompts, agents, resources and unknown capabilities are preserved
and updated, but are clearly marked **not active in the runtime**. No binaries
are installed, MCP commands executed or resource URIs accessed by importing.

### Source Security

Sources require HTTPS with ordinary certificate validation and public resolved
addresses, pinned for the request. Redirects, ambient HTTP proxies and provider
authentication are disabled. A 20-second timeout covers DNS, TLS and streaming;
response bodies are bounded to 1,000,000 bytes. The library has a 32-bundle
limit. Cancellation aborts the worker and stale replies cannot stage a preview.
File imports also use a bounded read on a background executor and reject
non-regular files.

Full source URLs and download errors are not logged or displayed in the list,
which shows only the hostname. Query-bearing URLs are retained in private local
state for updates and included only in an explicitly copied distribution link;
prefer public credential-free URLs. Passwords and decrypted API keys are never
serialized. Encrypted bundles require a password again when reopened.

### Verification Boundary

Local environment remains macOS Apple Silicon, `pi-rust`, base HEAD
`72e3afb66b9b611a56f1a55f033674435edef891` plus unrelated dirty changes.
Focused tests cover source routes, public-address policy, bounded downloads,
redirect/error/non-JSON rejection, whole model-free bundle retention, unknown
fields, stale baselines and encrypted persistence.

Final local evidence, using `/tmp/codex-native-target`:

- Desktop binary tests: 232 passed twice consecutively, including 100 repeated
  atomic saves while 25 child processes start.
- Core AgentConfig tests: 8 passed.
- Desktop debug binary build: passed.
- Desktop all-target Clippy with `-D warnings`: passed. Cargo still reports the
  pre-existing third-party future-compatibility warning for `block v0.1.6`.
- Touched-source rustfmt check and `git diff --check`: passed.
- The corrected PM review and scoped security review were read-only. The two
  reported security P2s (multi-instance overwrite and unbounded UI-thread file
  reading) were addressed; the reviewer statically closed both.

During verification, one earlier full run hit a concurrently edited Git-review
test failure; that file was not changed by this task. A subsequent repeat found
a save-lock contention case, addressed with explicit unlock on scope exit and
the child-spawn stress test above. The final repeated runs passed.

This redesign has not passed native visual acceptance: the previous GUI
session was interrupted by a locked Mac. Earlier model-only screenshots do
not validate the new source UI. Actual public HTTPS hosting, browser dispatch
and Windows runtime remain separate external acceptance checks.

Status: **PARTIAL**. Local implementation and deterministic checks are complete.
Native GUI/browser acceptance is **blocked** by the locked Mac; real public
HTTPS-source and Windows runtime checks remain **unverified**. Full workspace
tests, production changes and release/deployment are not claimed.

## Historical Implementation (Superseded)

Volt supports the VibeUnion AgentConfig Bundle draft as an import/export
boundary. The implementation follows the wire format from Draft v1.4:

- JSON envelope with `schema: "agentconfig-bundle"` and `v: 1`
- `.acfg` file payloads
- `agentconfig://import?v=1&bundle=...` deep links
- `shared`, `self`, and `managed` trust modes
- public preview metadata under `pub`
- plain payloads for credential-free sharing
- PBKDF2-HMAC-SHA256 plus AES-256-GCM for encrypted personal backups
- unknown public and envelope fields are accepted and ignored by the adapter

The adapter lives in
`crates/pi-agent-core/src/agent_config.rs`. It maps an explicitly selected public model to
the existing `ModelConfig`, exposes enabled skill IDs for registry matching,
and leaves unknown MCP servers, prompts, resources, and provider-specific
parameters available to a future feature without pretending they are already
executable.

The desktop settings page exposes:

- Preview configuration: accepts an `agentconfig://import?v=1&bundle=...` link and creates
  a preview. Enter in the URL field also opens the preview.
- External links: command-line URLs and native open-URL events open Settings >
  AgentConfig with the same pending preview, including on a cold launch.
- Import file: opens a native file picker and creates the same preview.
- Confirm and apply: applies the explicitly selected model and endpoint after
  confirmation. Multi-model bundles require a selection. Cancel discards the
  pending bundle and password without changing provider settings.
- Decrypt and preview: shown only for encrypted configuration, before confirmation,
  displaying the endpoint change and whether the API key will be replaced or
  cleared, never the key itself.
- Copy sharing link: copies a credential-free link for the current model/endpoint.
- Use for AgentConfig links: explicitly selects Volt as the OS link handler.
  Startup does not silently replace another application's handler.
- Export shared: writes a credential-free `.acfg` bundle.
- Personal backup: collapsed by default, with an independent backup password.
  Writes an encrypted `.acfg` bundle containing the configured provider key.

Passwords are held only in the input control for the current operation. API
keys are not written to the persisted desktop JSON state. A shared bundle
cannot contain credentials, and a personal backup cannot contain credentials
without encryption. Import replaces rather than reuses the previous provider
key, so a new endpoint never inherits an unrelated credential. This isolation
is persisted: restarting does not combine an imported endpoint with process
environment credentials. A missing endpoint blocks application instead of
silently retaining the previous endpoint or installing an empty URL.

The URL field accepts the AgentConfig protocol, not arbitrary HTTP(S) pages or
remote downloads. Receiving a link never applies it automatically. Encrypted
links can be previewed before entering a password. Malformed routes, duplicate
version/payload parameters, unsupported versions, invalid base64url, and
oversized carriers are rejected.

macOS release archives include `Volt.app` with `CFBundleURLTypes` declaring the
protocol, as well as the existing standalone binaries. Launch the app bundle
for browser-to-app links; a standalone `cargo run` process cannot register a
macOS app handler. Windows uses a per-user registry handler pointing directly
at the executable with a quoted URL argument, without a command shell. The
registration action is explicit, and self-uninstall removes only Volt's own
association. Installation can opt in with `volt --install --register-agentconfig`
or `install.ps1 -RegisterAgentConfig`; omitting it preserves existing handlers.

This adapter does not add MCP execution, remote A2A transport, OAuth refresh,
secret storage, or importing MCP/agent/prompt/resource runtime definitions.
The current desktop applies model settings only; skill counts in the preview
do not mean skill enable states have been applied. Those are separate runtime
capabilities. The native workflow and delegation policy remain owned by
`pi-agent-core`; importing a bundle cannot change the workflow's maximum
parallel workers or recursively enable delegation.

The external Rust SDK was not used as a direct dependency because the current
VibeUnion repository revision contains a malformed literal in its Rust
`schema.rs`. The local adapter keeps the same wire format while using stable
crate dependencies already resolved by this workspace.

## URL import follow-up - 2026-11-14

Source: user clarification that URL one-click import is the primary workflow,
not just bundle-file import/export. Parent: the existing AgentConfig adapter.
Scope: protocol parsing, URL entry points, consent/model selection, shared-link
copying, explicit link registration, and macOS application packaging.
Risk: medium. Orchestration: Native, one writer and one read-only verifier.

No commit, push, release, deployment, or default-handler change is authorized
by this follow-up.

## PM-directed interaction follow-up - 2026-11-14

Parent: URL import follow-up above. Source: the user's explicit request to
inspect the interface, call a PM, and redo the configuration experience.
Reason: the previous page mixed import, sharing, backups and protocol setup
into one toolbar, with an unconditional password field.
Scope: native AgentConfig settings hierarchy, progressive disclosure, localized
feedback and import/backup password isolation. Risk: medium. Native single
writer; the explicitly requested PM returned read-only product recommendations.
The preceding security verifier's findings were addressed; no claim is made
that the verifier re-reviewed the fixes.

The idle page prioritizes URL preview and file import. A received bundle
replaces that area with a focused preview, hiding unrelated sharing, backup
and association actions. Single-model bundles use a read-only model label;
multiple models require an explicit selection. The password is shown only
while an encrypted bundle needs resolution. Only a resolved, valid model can
reach confirmation. The preview shows current and imported model/service URL
and the credential impact. Skills are explicitly excluded when present.
Applying or cancelling removes pending state and clears the import password.
Utility feedback cannot overwrite import feedback.

Local evidence on macOS Apple Silicon, branch `pi-rust`, base HEAD
`72e3afb66b9b611a56f1a55f033674435edef891` plus the dirty working tree:

- Desktop binary tests: 216 passed, including credential isolation,
  endpoint validation, decryption prerequisites and settings disabled states.
- Desktop debug build and strict all-target Clippy passed before the final
  button-alignment and localized-error corrections; final rechecks pending.
- Isolated native GUI: idle state, explicit multi-model selection, endpoint
  and credential preview, confirm/apply success, conditional encrypted
  password, wrong-password blocking and successful decryption followed by
  a separate confirmation were observed.
- Test state is isolated under `/tmp/volt-agentconfig-pm-gPbmMn/state`;
  no real configuration, credentials or default-handler association changed.
- Narrow-window visual checks were interrupted when the Mac locked.
  Windows runtime/installer and browser-to-system association acceptance
  remain unverified. Full workspace tests were not run for this follow-up.
