# Codex Harness and OAuth

Volt now exposes two runtime engines from one desktop client. The desktop
application starts new and legacy sessions in the Codex Harness-compatible
runtime by default:

- `codex_compatible`: the default session engine, with Codex-style thread/turn
  lifecycle and a JSON-RPC stdio harness. The embedded desktop path uses the
  same `CodexStrict` policy and session contract without requiring a child
  process.
- `pi_native`: a forkable Pi runtime with the existing native tools and a
  balanced policy profile.

The desktop setting is persisted as a session preference. Engine changes are
blocked while a turn is running. A Pi session created from a Codex session is a
fork and retains `parent_session_id`; it does not mutate the original Codex
thread.

## Build-time runtime selection

The default desktop build includes both runtime features:

```bash
cargo build -p pi-desktop
```

Single-runtime builds are available for smaller or policy-specific packages:

```bash
cargo build -p pi-desktop --no-default-features --features codex-harness
cargo build -p pi-desktop --no-default-features --features pi-native
```

With only one feature enabled, the desktop hides the runtime selector,
normalizes restored/imported sessions to the compiled runtime, and rejects
Harness forks targeting a runtime that was not compiled.

## Harness

Build and run the stdio server:

```bash
cargo run -p pi-agent-core --bin codex-harness
```

The server accepts newline-delimited JSON-RPC requests:

- `initialize`
- `thread/start`
- `thread/read`
- `thread/import`
- `thread/export`
- `session/fork`
- `turn/start`
- `turn/interrupt`
- `auth/providers`
- `shutdown`

Streaming execution is emitted as `turn/event` notifications followed by a
`turn/completed` notification. The harness never accepts an API key in a
thread/turn request; credentials are supplied through the process environment
or the local credential store.

## Session import

`thread/import` accepts either a JSON object, a JSON string, or JSONL. The
importer understands common Codex `messages` and `items` records, preserves the
original items in `raw_events`, and exports a normalized Codex-compatible
document with `thread/export`.

Imported sessions are kept as Codex-compatible sessions. Use `session/fork` with
`"engine": "pi_native"` when continuing the work with Pi semantics.

## OAuth PKCE

API keys remain supported. ChatGPT uses the public OAuth client configuration
used by the official Codex client by default. Generic OAuth uses
authorization-code + PKCE with a local `127.0.0.1` callback and stores tokens
outside `desktop.json` in a file with user-only permissions.

Select a provider in **Settings → Model providers → Authentication** and then
choose **OAuth login**:

```bash
export VOLT_DEEPSEEK_OAUTH_AUTH_URL='https://official.example/authorize'
export VOLT_DEEPSEEK_OAUTH_TOKEN_URL='https://official.example/token'
export VOLT_DEEPSEEK_OAUTH_CLIENT_ID='official-public-client-id'
export VOLT_DEEPSEEK_OAUTH_SCOPES='openid profile'
```

The same variables exist for `CHATGPT`, `DEEPSEEK`, `PI`, and `CUSTOM`:

```text
VOLT_<PROVIDER>_OAUTH_AUTH_URL
VOLT_<PROVIDER>_OAUTH_TOKEN_URL
VOLT_<PROVIDER>_OAUTH_CLIENT_ID
VOLT_<PROVIDER>_OAUTH_SCOPES
```

ZCode uses the provider's official CLI OAuth device/polling flow. It defaults to
`https://zcode.z.ai/api/v1`; override it with
`VOLT_ZCODE_OAUTH_BASE_URL` for an official test environment. The browser
authorization URL is obtained from the provider and is never guessed.

The desktop login flow does not read browser cookies, private desktop
databases, API keys from web storage, or undocumented client secrets. After a
successful login, set `VOLT_AUTH_PROVIDER=chatgpt` (or `deepseek`, `zcode`,
`custom`) when starting the standalone harness so it can reuse the stored
token.

The Settings page also exposes Codex session import/export. Imported JSON and
JSONL sessions preserve the original events; continuing in Pi mode creates a
fork with `parent_session_id` instead of mutating the imported Codex thread.

If a provider does not publish a public OAuth/OIDC client contract, the app
will report that the official endpoint configuration is missing instead of
pretending to support a private login flow.
