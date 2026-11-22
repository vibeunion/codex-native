# Project review implementation — resumed 2026-11-16

## Current local verification receipt — 2026-11-17 continued

The current dirty checkout was revalidated without changing source files or
touching the existing Volt process. The full workspace test command on the
data volume entered a Cargo filesystem wait with no compiler child process and
was stopped; that interruption is not a source failure. Repeating the same
command with a private local `/tmp` target completed against the current
checkout:

- `cargo test --workspace --all-targets --offline --locked --quiet` passed
  **408 tests**: 86 in `pi-agent-core` and 322 in `pi-desktop`, with zero
  failures, ignored tests, measured tests or filtered tests.
- The provider transport suite passed **8 tests**, including response-header,
  idle-body, total-deadline, slow-error-body, credential-redaction and bounded
  diagnostic-body contracts.
- The Settings suite passed **55 tests**.
- `cargo clippy -p pi-desktop --tests --offline --locked -- -D warnings`
  passed. The only remaining output is the existing transitive
  `block 0.1.6` future-incompatibility warning.
- Focused rustfmt checks passed for the provider and Settings files touched by
  the current acceptance work. A full `cargo fmt --all -- --check` invocation
  again stalled in `rustfmt` while reading the data-volume checkout, so this
  turn does not claim a new full-format receipt.
- The macOS package script now preflights required release credentials and can
  sign embedded binaries, notarize the app, staple the ticket and verify the
  stapled bundle. The tag-only CI path imports the configured certificate and
  requires all signing/notarization secrets; a missing identity fails before
  creating a partial app. Local unsigned packaging and the missing-identity
  guard passed, and the full ad hoc signing path verified the three packaged
  binaries and app bundle. No real certificate or notarization submission was
  available in this checkout.

The CUA adapter found the existing Volt process but returned
`AXError.cannotComplete` when selecting it; no new GUI, IME or VoiceOver
acceptance is claimed. The current v4 Settings traversal and narrow-window
receipts remain the authoritative current-candidate evidence. Windows
installation, real-provider/HTTP/2 lifecycle, matched ZCode pixels,
VoiceOver announcements, Chinese IME composition, secret-backed
signing/notarization execution and authorized remote release remain **PARTIAL**.
No commit, push, release or deployment was performed.

## Current live-source validation — 2026-11-17 continued

The current dirty checkout was revalidated after the latest settings Select
changes. This is a live combined-source receipt; it supersedes older frozen
candidate counts for deterministic local gates, while preserving the
uncommitted and unrelated worktree changes.

- Fixed the settings menu label `ElementId` construction so the current
  Select delegate compiles. The package script now has its executable bit.
- Kept the production Git review timeout at 15 seconds while giving test
  builds a 60-second process-start window. This removes false failures caused
  by the hundreds of concurrent unit tests starting Git fixtures together.
- `cargo fmt --all -- --check`, `git diff --check`, and `actionlint
  .github/workflows/build.yml` passed.
- CI test jobs now use `cargo test --locked --workspace --all-targets` on both
  macOS and Windows, matching the local production validation scope.
- `cargo check --locked --offline --workspace --all-targets` and
  `cargo clippy --locked --offline --workspace --all-targets -- -D warnings`
  passed. The existing `block 0.1.6` future-compatibility warning remains.
- `cargo test --locked --offline --workspace` passed **408 tests**: 86 in
  `pi-agent-core` and 322 in `pi-desktop`, with zero failed, ignored, measured,
  or filtered tests. The desktop-only default-concurrency run also passed all
  322 tests.
- Release builds passed for `volt`, `calc-cli`, `office-cli`, and
  `sensenova-live-test`. The current `volt` SHA-256 is
  `57a21c98f175c2bbe1553514bdf16964dee3008cc0bd1b5f9711d0318a0a26c6`.
- `scripts/package-macos.sh` passed `Info.plist` validation and produced a
  local arm64 archive. The packaged app passed ad hoc strict codesign
  verification. The archive SHA-256 is
  `4be24773c0fde464d7ab483874e61511b06b7738265b9b12d501e3a1e4a0d226`.
  Release signing and notarization are not established by this local package
  check.
- Packaged CLI smoke coverage remains covered by the earlier local package
  receipt: scientific evaluation, statistics, matrix determinant, unit
  conversion, filtered table query, Markdown/CSV roundtrip, table statistics,
  and ASCII chart generation.

The first GUI probe in this continuation timed out while opening the fresh
packaged app; the preceding probe returned `Screen is locked`. A later v4
candidate produced a current-build narrow Settings screenshot and interaction
receipt: the content viewport was `480x640`, the selected Theme check was
visible, and Dark/20px Save feedback rendered. Matched ZCode pixel comparison,
complete keyboard traversal, IME and screen-reader receipt remain unverified.
Windows installation/browser behavior, authorized real-provider and HTTP/2
lifecycle evidence, signing/notarization, and remote CI/release remain
separate gates.
Overall production acceptance remains **PARTIAL**. No commit, push, release
publication, or deployment was performed.

## Current live-source continuation checkpoint — 2026-11-16

The current working tree remains intentionally dirty and uncommitted. The
seven strict-Clippy substitutions in the live `projects.rs`, `workbench.rs`
and `workbench/composer_toolbar.rs` files are now integrated in this checkout;
the Select menu row-height correction remains in `settings/select_motion.rs`.
This checkpoint does not claim a stable combined-source baseline while the
separate UI writer chat is still active.

The live checkout passed `cargo fmt --all -- --check`, `git diff --check`,
`actionlint .github/workflows/build.yml`,
`cargo check --locked --workspace --all-targets`,
`cargo clippy --locked --workspace --all-targets -- -D warnings`, and
`cargo test --locked --workspace --all-targets`. The workspace test run passed
**407 tests**: 86 core tests and 321 desktop tests, with no failures, ignored
tests, measured tests or filters. The existing `block 0.1.6`
future-compatibility warning remains.

The CI-equivalent Release build
`cargo build --locked --release --workspace --bins` passed for all four
arm64 binaries. The current `volt` SHA-256 is
`ce156a477672253261910e71c57f9c1a9d078ea19f67c713cd4f38557d93cd36`.
The macOS packaging script passed `Info.plist` validation, payload listing,
arm64 Mach-O checks and executable-permission checks; the temporary package
archive SHA-256 was
`5833cd7dd064eba9b90bdf89246bfbeb1cde991343e1e496b61881b6d1b98f84`.
Packaged CLI smoke checks passed CSV semantic roundtrip, table query/statistics,
scientific evaluation and unit conversion.

The native UI writer started a fresh Release instance, but the current
accessibility/window probe did not yield a usable frontmost interaction
receipt. No GUI PASS is claimed from that attempt. Stable combined-source
validation, Windows installation/browser behavior, authorized real-provider
and HTTP/2 lifecycle evidence, the complete keyboard/IME/screen-reader and
visual matrix, signing/notarization, and authorized remote CI/release remain
open. No commit, push, release publication or deployment was performed.

The current continuation remains **PARTIAL** and the goal is **active** after
the user resumed it. The previous `goal blocked` result below is a historical
v14 checkpoint; a fresh blocked audit is required for this resumed run. The
requirement-by-requirement acceptance status is recorded in
`production-completion-audit-v14.json`.
The latest validated candidate is **v14**, with 88 frozen inputs. Evidence is
indexed by `continuation-v14-evidence.json` under
`/Users/zhd/.codex/tmp/codex-native-tools-correction-20261115-o6b_29hq/`.
This is a derived local candidate; it does not validate subsequent concurrent
UI edits or establish production release acceptance.

The Mac is unlocked, Screen Recording is granted and Accessibility is trusted.
Although the tool catalog has no native input tool, a scoped Accessibility
helper now performs actual `AXPress` on uniquely identified, enabled buttons.
It checks the owned PID, executable path, bundle identity and candidate hash
before each action. This resolved the native button-control blocker without
interacting with the user's application state.

V14 passed formatting, actionlint, ordinary/default-profile workspace all-target
check, all-target strict Clippy and all **401 tests**: **86 core and 315 desktop**.
None failed, were ignored, were measured or were filtered out. Frozen inputs
remained unchanged throughout validation. Both local packages were cleaned
before all four Release binaries were rebuilt from the exact frozen source.
The desktop executable SHA-256 is
`133b83ef709fb69972bb8d1a81a2360ffcd2b4ffb61afec3805bb70ef3dff1f1`.

Actual native screenshots exposed a recovery defect: a paused queue without a
run-error notice hid its command row behind the composer. The review-owned
`panels.rs` now uses the existing standalone layout for paused queues as well
as run notices. A fresh v14 crash/restart screenshot confirms the complete
queued command, its controls, the paused banner and the composer are separately
visible. Explicit Resume then executes only the queued command.

Fresh-artifact native lifecycle receipts also prove durable checkpoints before
shell effects, no automatic replay after a controlled crash, persisted
unknown-outcome reconciliation on Resume, Stop terminating the shell and its
descendant, and no delayed descendant effect. Two native instances demonstrate
stale-write rejection before another side effect; after the primary's debounced
save settled, the entire persisted state remained byte-for-byte unchanged.
The bilingual conflict notice was visually checked. All owned fixture apps and
the loopback server have stopped; cleanup is recorded separately.

The native model fixture passed actual Send/Stop, two ordered real shell tools,
checkpoint and result ordering, delivery of both results in the next provider
request, SSE disconnection on Stop and an unexecuted paused queue. This uses an
owned HTTP/1.1 loopback server with an explicitly fake fixture credential.
It does not establish real-provider or HTTP/2 acceptance.

A further v14 keyboard fixture now has actual per-PID key-event evidence:
Shift+Enter inserts a durable newline without submitting; Tab shows a visible
focus ring on More actions; Shift+Tab returns to the composer and Enter executes
the exact local command once. Three Tabs from the composer reach Select model,
Enter opens its labelled native accessibility menu, and Escape closes it.
Empty Enter does not add messages or another tool result.

The first keyboard receipt remains **failed** because an additional assertion
could not prove the textarea's focus after closing the menu. The observed
Accessibility focus was the window; this does not establish a product defect
or a successful textarea focus transition. A separate narrow recheck proves
Tab/Shift+Tab returns to the editor through a second actual, durable newline,
with no submission or additional tool execution. Both receipts are retained
under `native-keyboard-v14/`; their scoped evidence is indexed separately.
This does not complete the full keyboard matrix, Chinese IME or live
screen-reader acceptance. The keyboard fixture has stopped.

The v14 macOS archive passed payload hashes, executable permissions, arm64 and
minimum-system metadata, and actual packaged CLI checks, including exact
CSV/Markdown roundtrip. It is an unsigned local candidate at
`/Volumes/Datas/Workspace/.codex-validation-cache/codex-native-review-evidence-20261116/release-package-check-v14/output with spaces/volt-macos-arm64.tar.gz`.
Its SHA-256 is
`de3f81bc74cc8658e0f0d26c84caa2ba2f2c46aaa39bc73de00f790e0547688d`.

The slow private-cache dependency links were replaced by 98 hash-verified
regular files, restoring local reads of approximately 1.252 GiB. The original
blobs remain intact. After this correction, clean v13 and v14 Release builds
completed in approximately 51 and 47 seconds respectively.

Seven Clippy substitutions in `projects.rs`, `workbench.rs` and
`workbench/composer_toolbar.rs` remain isolated in
`full-workspace-v14/pending-ui-clippy.patch`; they are included in v14's passing
checks. Their live files remain owned by the active UI chat. This task has not
edited those files or messaged that chat. A stable combined capture is required
after integration. The current live comparison is `live-comparison-v14.json`.
The latest readiness audit confirms the patch still applies. The UI chat ended
its preceding turn and immediately started another active turn, so this is not
evidence that its writer scope has been released. Cross-chat messaging remains
unauthorized; the prepared coordination note is `handoff-ui-clippy-v14.md`.

The v11 Clippy failure, v12 formatting interruption, interrupted v13 Release
attempt, original native fixture failures and conflict-baseline race are
preserved. Successful corrected receipts do not relabel those attempts.
Recovery reconciliation happens in memory on startup and becomes durable at
the next explicit save; the corrected crash fixture verifies that contract.
The conflict fixture now waits for the primary's 400 ms debounced save before
freezing and rechecking its baseline.

Remaining acceptance requirements are integration of the pending lint patch
and stable combined-source validation; actual Windows installation and browser
behavior; authorized real-provider and HTTP/2 lifecycle evidence; the complete
keyboard matrix, Chinese IME and live screen-reader behavior; the complete visual
reference matrix; and signing/notarization plus authorized remote CI/release.
The `block 0.1.6` future-compatibility warning remains tracked.
No commit, push, release publication or deployment was performed.

Continuation requires explicit authorization for cross-chat coordination or
clear release of the UI writer scope, followed by stable combined-source
validation. Windows, real-provider, full desktop acceptance and signing/release
requirements remain separate. This blocked checkpoint preserves the full
production-readiness goal and all passing local evidence.

## Historical v10 checkpoint — 2026-11-16

Outcome: **PARTIAL**, goal **blocked**. The latest frozen combined candidate is
**v10**. Its local validation passed, but native and production acceptance remain
incomplete. Evidence is indexed by `continuation-v10-evidence.json` under
`/Users/zhd/.codex/tmp/codex-native-tools-correction-20261115-o6b_29hq/`.

V10 includes the review corrections, keyboard focus reveal, greeting geometry,
composer spacing, and the UI owner's test compilation correction. Its 86 captured
files matched live source before and after copying. Formatting, actionlint,
Release-profile workspace all-target strict Clippy, the complete test inventory,
all **374 Release-profile tests** and all four freshly rebuilt Release binaries
passed. The tests comprise **86 core and 288 desktop** cases; none failed, were
ignored, were measured as benchmarks or were filtered out. These results are
explicitly Release-profile checks, not a new ordinary-profile test receipt.

The final build cleared only this task's two local workspace packages from its
private Cargo cache. Its build log then confirmed compilation of both packages
from the frozen v10 paths. The resulting desktop executable has SHA-256
`3b10f5cc8a7421c268a40cfddf371eb848d4d71247da9697c0c7690f02571188`.
The macOS archive passed payload hashes, permissions, platform metadata and actual
packaged CLI checks, including exact CSV/Markdown roundtrip. Its payload contains
regular executable files and was assembled on the data volume. Its SHA-256 is
`54663b58ecceb90e1cfc2c03f3309b4f16d00322d4bd9e3e5b3803917b0a67b3`.
This is an unsigned local candidate, not an authorized release.

The clean rebuild closes a real evidence defect. V8 initially reused the v7
desktop executable despite changed runtime source. Rebuilding the unchanged v8
source after cleaning the desktop package produced a different executable.
The original v8 receipt is marked `superseded-artifact-binding`; its logs and
original artifact remain preserved. New captures use fresh destination timestamps
and clear both local packages before the final binary build. The v9 all-target
Clippy failure also remains preserved: a UI test lacked `LengthExt` and expected
`Pixels` instead of `Option<Pixels>`. Its owner independently corrected both
before v10 was captured; this task did not edit that active owner's file.

The first v10 attempt stopped at its disk preflight before executing a command.
The successful retry retained every verification gate, used existing Release
dependencies and monitored a 2 GiB stop reserve after a 3 GiB preflight. This
budget was based on the preceding complete run's measured usage. Large immutable
dependency artifacts were individually copied to the data volume with matching
hashes, permissions and timestamps; Cargo metadata and workspace source stayed
local. The abandoned whole-cache migration remains quarantined and unused.

The post-validation live comparison found two further concurrent UI changes:
`crates/pi-desktop/src/workbench.rs` and
`crates/pi-desktop/src/workbench/recovery.rs`. They are **outside v10** and need a
stable combined capture before release acceptance. The four review correction
files remain separately bound to their independent review. The historical
workbench formatting patch is retained; v10 itself passes formatting without
requiring that old patch.

Native acceptance is still blocked by a current observation, not by another
chat's activity. After the fresh build, the UI tool again returned: “The Mac is
locked and automatic unlock could not unlock it.” No v10 fixture was launched
and no UI action was submitted. `queue-notice-layout-20261116/launch-native-v10.py`
is prepared for the private Stop/queue replay after manual unlock. Older native
receipts remain valid for their recorded executable hashes; they do not accept
the freshly built v10 artifact.

Remaining requirements are manual Mac unlock and fresh-artifact native workflow
and accessibility acceptance; a stable capture of the concurrent UI changes;
actual Windows installation and browser-opening tests; authorized real-provider
and HTTP/2 lifecycle evidence; signing/notarization and authorized remote
CI/release. The `block 0.1.6` future-compatibility warning remains tracked.
No commit, push, release publication or deployment was performed.

The **2026-11-16** read-only remote audit found local and remote `pi-rust` still
at `72e3afb66b9b611a56f1a55f033674435edef891`. The newest listed branch run,
`41166775791`, remains a completed failure from **2026-10-31**. These remote
results do not validate the uncommitted v10 candidate; no remote candidate build
was initiated by this task.

The same locked-Mac condition was verified in three consecutive goal turns.
The goal is now **blocked**, with the complete scope and passing local evidence
preserved. The next required action is a manual Mac unlock, followed by explicit
resume for native acceptance. Windows, real-provider configuration and release
authorization remain separate requirements. The blocking audit is recorded in
`goal-blocked-audit-20261116.json`; a resumed run starts a fresh blocker audit.

## Historical v6/v7 checkpoint

The following checkpoint predates v10; its pending states describe that earlier
candidate, not the current validation result.

Outcome: **PARTIAL**, goal **active**. The user's resolve-blockers instruction
removed an overly broad assumption: an active UI chat does not by itself reserve
every independent native application. Distinct fixture bundles, state directories
and workspaces allowed native lifecycle acceptance without editing the active
UI owner's files or interacting with that owner's previews.

All seven v6 source-bound gates passed: formatting, actionlint, all-target
check, strict Clippy, complete test inventory, all 359 tests and all four
Release binaries. No tests failed, were ignored or were filtered out; no frozen
source changed. The count comprises 80 core and 279 desktop tests.

All scoped core review findings remain implemented and independently reviewed.
Their sources are unchanged from v3, including the 14 data/CLI regressions and
two traversal-error tests. V6 macOS packaging passed payload hashes,
permissions, platform metadata and exact whole-record packaged CLI roundtrip.
The core review receipts and earlier failed attempts remain preserved.

The original live v5 capture failed formatting in `workbench.rs`. Its immutable
failure receipt is preserved. V6 differs from it only by rustfmt output in that
file; the exact two-hunk patch passed forward and reverse checks. It is pending
the active UI writer's release and has not been applied to live source. Later
keyboard-reveal edits in `workbench/draft_prompts.rs` and `workbench.rs` are
outside v6. No passing live-workspace claim is made.

The v6 Release artifact now has real native interaction receipts for checkpointed
command crash/restart without replay, paused-queue resume, Stop with child-process
termination, and two-instance conflict rejection before command execution. A
separate native model fixture also verified two ordered tools, durable checkpoints,
tool results in the next provider request, streaming cancellation and a paused,
unexecuted queue. Its server is controlled loopback HTTP/1.1; this is not real-service
or HTTP/2 evidence. The older v4 fixtures remain unlaunched and preserved.

The model replay exposed a visible queue/run-notice overlap: the queue's negative
composer margin also overlapped an intervening Stop notice. A bounded local fix in
`panels.rs` and `panels/queue.rs` separates the card when a run notice exists.
The four corrective files match live source and passed independent read-only
review. The immutable v7 candidate passed full formatting and all four Release
binary builds. It also passed macOS archive assembly, payload hashes, permissions,
platform metadata and packaged CLI checks. V7 retains the v6 desktop baseline;
the concurrent UI owner's later work remains outside this candidate.

Native acceptance of the queue correction is still **PARTIAL**. On **2026-11-16**,
the v7 private fixture launched, but the UI tool returned “The Mac is locked and
automatic unlock could not unlock it.” No UI command was submitted. The owned
fixture was stopped and its state preserved. The remaining check is to run the
private hold command, queue a prompt, click Stop, verify separate readable and
operable queue/notice controls, then dismiss the notice and resume the queue.
Unlocking the Mac is required before that check can continue; the earlier v6
native receipts remain valid for their own artifact.

The live-provider probe also returned success when credentials were absent and
could print PASSED without the requested tool result. A new subprocess regression
reproduced the missing-key false success against unchanged v6 probe source. The
local correction requires a successful bash result with the requested marker and
a successful model turn. Six loopback subprocess contracts, all **86 core tests**,
core all-target strict Clippy, formatting and a Release probe build passed against
the immutable correction source. No core test failed, was ignored or was filtered
out. The reviewer identified two test-evidence gaps, both addressed and closed by
the same independent reviewer:
zero network connections without credentials, and malformed model output after
an already successful tool. These checks use synthetic credentials and loopback
HTTP/1.1; they do not establish real-provider availability.

The internal disk blocker was resolved by pruning only this task's rebuildable,
unlocked debug cache. The preceding migration to the data volume was interrupted
because of slow filesystem I/O; its partial destination is preserved and is not
used as a build cache. The original Release cache and candidate hashes were
preserved. The successful corrective checks subsequently used the original private
cache and retained the 5 GiB preflight floor.

Windows, real-provider/HTTP2, live screen-reader, complete desktop workflow and
authorized remote CI/release remain outstanding. The `block 0.1.6`
future-compatibility warning remains. No commit, push, release publication or
deployment was performed. Current evidence and the exact pending format patch
are linked by `continuation-v6-evidence.json` in the corrective evidence
directory. The latest corrective evidence is indexed by
`continuation-v7-evidence.json`. New native receipts are in `native-lifecycle-current-v6/` and
`native-model-lifecycle-v6/`; correction plans are in
`live-probe-correction-20261116/` and `queue-notice-layout-20261116/`.
Overall acceptance is **PARTIAL**; the full scope is retained.

## Implemented locally

- File, search, and Office tool paths resolve inside the selected workspace.
  Writes reject escaping paths and broken symlinks. An unavailable workspace
  produces a tool error instead of panicking during registry construction.
- File line ranges reject invalid types, zero, and reversed bounds. Search
  limits reject invalid types and values outside 1–10,000.
- Generative UI validates input size before JSON deserialization and bounds
  text, collections, nesting, node counts, and numeric parameters. Invalid or
  oversized complete blocks remain intact as Markdown.
- Message parsing preserves Unicode byte offsets, distinguishes self-closing
  embeds, and respects attribute names and quoted `>` characters.
- Embedded file sources resolve inside the workspace and enforce the 256 KiB
  artifact limit while reading, as well as before reading.
- Browser actions accept HTTP(S) URLs without credentials and pass the
  normalized URL directly to the platform opener. Windows no longer passes
  these actions through a command interpreter.
- HTML previews use unique filenames, exclusive creation, and private Unix
  permissions. All three HTML launch controls share creation/opening logic.
  The current notification follow-up makes launch errors visible without
  changing model execution state; runtime acceptance is recorded separately.
- Formatting, compilation, and Clippy issues encountered in the current
  workspace were corrected while retaining concurrent dependency, settings,
  and AgentConfig work.

The earlier report-data, CSV/query/numeric validation, credential masking,
Windows installer, release asset, and CI gate fixes remain in the repository
and are covered by the current regression suite or workflow validation.

These path checks are application input validation. They do not provide an OS
sandbox or prevent concurrent filesystem replacement; the shell tool retains
its existing execution capabilities.

## Previously verified provider follow-up snapshot

Rust: `rustc 1.97.1`. Cargo target directory:
`/tmp/codex-native-target`. Cargo commands ran serially with the existing lockfile.

| Check | Result |
| --- | --- |
| `cargo fmt --all -- --check` | PASS |
| `actionlint .github/workflows/build.yml` | PASS |
| `cargo check --locked --workspace --all-targets` | PASS |
| `cargo clippy --locked --workspace --all-targets -- -D warnings` | PASS |
| `cargo test --locked --workspace` | PASS: 264 tests |
| `cargo build --locked --release --workspace --bins` | PASS |
| `git diff --check` | PASS |
| Final manual diff and clean-code-guard pass | PASS |

The latest tests comprise 23 core unit tests, 31 core integration tests, and 210 desktop
unit tests. No tests were ignored. The build still reports the third-party
`block 0.1.6` future-incompatibility warning.

This full-workspace receipt belongs to the provider follow-up snapshot. Later
concurrent AgentConfig, installer, macOS packaging, and UI changes require their
own validation before a combined candidate can use that receipt.

## Production reliability follow-up — 2026-11-14

Parent: the project review above. Source: the user's continuing production-readiness
goal. Reason: inspection found unbounded waits for model response headers and
heartbeats, and an executable regression demonstrated that malformed content
deltas could incorrectly complete successfully.

Scope: provider HTTP lifecycle, streaming protocol, tool-call assembly, error
reporting, and their tests. Risk: medium. Orchestration: Native, one writer and
one independent read-only verifier. Concurrent settings, file-summary, dependency,
and AgentConfig work remains with its existing owners.

- Response headers have a 30-second deadline, idle reads 60 seconds, and the
  complete request 300 seconds. Tests confirm the server sees its HTTP/1.1
  connection close after each deadline, including continuous heartbeats.
- Individual events are limited to 4 MiB; cumulative responses to 16 MiB.
  UTF-8 fragments, multiline SSE data, BOM, LF, CRLF, and CR are covered by
  actual loopback HTTP requests.
- Typed deltas reject malformed content and tool-call fields. Tool IDs must be
  unique, arguments must decode to an object, and call/field limits are enforced.
  Interleaved fragments retain their original index and usage receipt.
- Partial text survives later malformed events and incomplete finish reasons.
  Output after a terminal finish reason, truncated trailing events, and streams
  with no assistant choice fail without a `Completed` event.
- Invalid credential headers fail before sending a request. Headers are marked
  sensitive, error bodies are bounded, and both HTTP and parsing errors redact
  the configured credential. Tests use dummy credentials and loopback services.
- HTTP failures retain their known status even if a slowly arriving diagnostic
  body reaches the total deadline.

Independent verifier `Zeno` reviewed the four provider implementation/test files
without modifying them. It identified three issues: slow error bodies could hide
the HTTP status; an earlier terminal marker could admit new incomplete output;
and truncated finish events could lose their last text delta. All three were
addressed with regression coverage. The verifier did not issue a second review
of the fixes; the owner reran deterministic validation after applying them.

The final targeted rerun also confirmed that the slow-error-body regression
preserves both HTTP 429 and its error event. Formatting, workflow validation,
and whitespace checks passed after that assertion was added.

The default `gpt-5.3-codex` preference was unavailable in the exposed agent model
list, so this verifier inherited the current Codex model. No host runtime switch
was used.

One global formatting attempt encountered a concurrently edited, temporarily
unparseable `workbench.rs`. The UI owner corrected it; no changes were made to
that owner's work by this follow-up. The final global format, check, strict
Clippy, test, and release gates passed afterwards. The test count includes the
UI owner's latest tests; it is not a count of this follow-up's additions alone.

The provider transport remains Chat Completions compatible. Native Anthropic
Messages and Ollama Chat adapters are not implemented or accepted by this work.
The README now states the actual transport instead of implying native support.

## Remaining evidence

- Independent read-only verifier `Goodall` did not return findings: the platform
  rejected its run with “This content was flagged for possible cybersecurity
  risk.” This is an interrupted verification run, not an approval or a clean
  review. No alternate runtime was used.
- Windows installation and browser-opening behavior require an actual Windows
  environment.
- A fresh read of GitHub Actions found run `41166775791`, created on
  **2026-10-31**, for current HEAD `72e3afb66b9b611a56f1a55f033674435edef891`.
  It failed at **Run Rustfmt Check**; the macOS build, Windows build, and release
  jobs were skipped. The uncommitted fixes above have local proof but no new
  remote CI or release receipt. The first API read returned EOF; a retry
  returned the completed run and job metadata.
- Browser artifact rendering and the complete desktop workflow remain
  unaccepted. The scoped native settings interactions below now have GUI
  evidence. Starting the platform opener alone does not prove browser rendering.
- An earlier native-app inventory returned **The Mac is locked**. That
  historical observation is superseded for the scoped settings checks by the
  successful native launch below; no manual unlock or system-setting change
  was performed.
- Deadline cancellation tests prove HTTP/1.1 loopback behavior. HTTP/2 and real
  provider connection lifecycle have not been accepted by these tests.

## Continuation acceptance audit — 2026-11-14

The preceding goal turn made implementation and verification progress. This
continuation rechecked the remaining external gates and found no new acceptance
receipt; it is not a verified wait for an active test or CI job.

- The current host is macOS, has no `pwsh`, and only has the
  `aarch64-apple-darwin` Rust target installed.
- Neither `SENSENOVA_API_KEY` nor `OPENAI_API_KEY` is present in the test process
  environment. Only their presence was checked; no credential was printed and no
  live provider request was sent.
- GitHub still reports completed, failed run `41166775791` for the current HEAD.
  There is no running workflow to wait for and no new remote validation receipt.
- The GUI inventory returned, but did not establish that the desktop is unlocked.
  Native access to the Codex host was denied by the tool's safety policy, and
  the tool did not recognize the bare `volt` executable as an application. No
  new Volt screenshot or interaction receipt was obtained. The earlier locked
  desktop observation must not be treated as a fresh lock-state measurement.

## Native settings acceptance follow-up — 2026-11-14

Parent: the continuation acceptance audit. Source: the continuing user goal.
Reason: a bare executable was not a supported native-tool application target;
that tooling limitation did not establish that GUI testing was impossible.
Scope: an isolated temporary application bundle and settings/preview acceptance
of the existing Release binary. No production source or release packaging was
changed by this follow-up.

The Release binary was copied into a uniquely identified temporary macOS app
bundle. Its launcher uses a new empty workspace and private state directory,
removes provider credentials from its environment, and selects only a loopback
endpoint. Binary SHA256:
`3367d9495ba9e8c7bb472e32214a65a8608b246c9b2673a23edc0827754c8517`.
The copied binary and current Release artifact still matched after testing.

Observed through native accessibility and screenshots:

- The startup page and provider settings opened successfully.
- Entering a `file:` provider endpoint and activating Save displayed the
  validation error asking for a valid HTTP(S) address and model name.
- A non-sensitive dummy credential was visible only after using Show API Key.
  Hiding it removed its value from the accessibility tree. After saving, the
  input was cleared and the configured-credential summary displayed a mask.
- Reading the isolated state file confirmed the saved loopback endpoint and
  the absence of both an API-key field and the dummy credential.
- The theme menu opened, and Down/Return changed its value to dark. The
  resulting screenshot showed the dark settings surface.

These checks do not establish complete settings layout, focus/IME, minimum
window size, transient Save feedback, or matched-reference pixel acceptance.
The captured provider form's right edge was not fully visible, so field suffix
visibility and viewport sizing remain unaccepted.

For preview testing, the same private state was seeded with Unicode text, a
valid native card, malformed JSON to retain as text, and a self-contained HTML
artifact. Its session appeared after launch, but the subsequent screenshot
request timed out. The same still-running app was reselected without restarting
it because of that timeout; its current view then contained additional
navigation and a new session. No HTML rendering or interaction PASS is claimed.

Only the identified fixture process was stopped. Temporary state, the manifest,
and a structured acceptance receipt remain outside the repository for
inspection. Real configuration and existing application processes were not
changed. The goal remains active/PARTIAL with Windows, real-provider,
browser-preview, complete desktop interaction, and remote CI/release proof
outstanding.

## macOS packaging verification follow-up — 2026-11-14

Parent: the original release/install review and the native acceptance follow-up.
Source: the continuing production-readiness goal. Reason: concurrent work added
`scripts/package-macos.sh`; its archive assembly had not been covered by this
review's earlier workflow syntax check. Scope: temporary packaging artifacts
and a proposed narrow script fix. Orchestration: Native; no shared source edit
while its existing writer is active.

The inspected script snapshot had SHA256
`44af6a4558684f7e23d42c60536031849ffb7f442c2093c9a8738852378f33ea`.
It was copied with its Info.plist into a private test directory and exercised
against the existing Release binaries:

- Absolute input/output paths passed. The archive contained `Volt.app`, its
  plist and three executables, plus the three standalone executables. The
  bundle version was `0.1.0`.
- Relative paths failed with exit code 1: `tar: could not chdir to 'release'`.
  The second relative `-C` was resolved after entering the output directory.
  The current CI invocation uses absolute paths, so this reproduces a local
  script invocation defect, not a demonstrated failure of that CI step.
- A one-line private patch resolves the binary directory to an absolute path
  before passing it to the second `tar -C`. The patched snapshot passed actual
  archive creation for relative paths, absolute paths, and an output path
  containing spaces. Each archive contained all 10 expected entries.
- The patched script passed `bash -n`; the current workflow passed
  `actionlint`. The source script still matched the failing snapshot after
  these checks.

The active chat “Add agentconfig URL import” is still editing and validating
this packaging scope. Its live turn was polled, not assumed finished after an
observation timeout. The patch and both packaging receipts remain outside the
repository; this review has not applied the patch to the shared script or
claimed the defect fixed there. The active chat “继续按 zcode 样式复刻” is also
performing native interaction work, so this follow-up did not start another
native UI driver.

The packaging tests establish local archive assembly only. They do not accept
the latest URL-import runtime, signing/notarization, downloaded installation,
Windows behavior, or remote CI/release publication. The previously validated
Release binaries were the packaging inputs; the new concurrent source was not
rebuilt by this follow-up.

No commit, push, release, or deployment was performed. Changes remain local
alongside the existing concurrent work.

## Combined-workspace validation follow-up — 2026-11-14

Parent: the provider and packaging follow-ups. Source: the continuing user goal.
Reason: concurrent AgentConfig, installer, packaging, and interface edits changed
the inputs covered by the previous complete validation receipt.

The serial validation in
`/Volumes/Datas/codex-native-integration-tpqniygq/validation-receipt.json` passed
Rustfmt and actionlint, then failed
`cargo check --locked --workspace --all-targets`. The test module in
`crates/pi-desktop/src/settings.rs` calls `provider_field_label` without importing
it, producing E0425 at line 2096 in the checked snapshot. Consequently that run
did not execute all-target Clippy or the full test suite. No existing input hash
changed during that run; its inventory did not independently track new files
created after the run started.

The settings file remains owned by an active interface-editing chat. This
follow-up has not changed that owner's file or treated its active turn as
finished after a wait timeout. The earlier 264-test PASS is historical evidence
for its recorded snapshot, not a PASS for this combined workspace.

An additional serial run against the shared tree passed:

- `cargo check --locked --workspace --bins`
- `cargo clippy --locked --workspace --bins -- -D warnings`
- `cargo test --locked -p pi-agent-core --quiet`: 57 tests passed, none ignored.

Its receipt and log are in
`/Volumes/Datas/codex-native-integration-followup-7n0y9l2a/`. The source inventory
was collected again at completion, including newly added matching inputs; it
found no changes. These results cover the program compilation and core tests
only and do not clear the desktop test-target failure.

## Current packaging recheck — 2026-11-14

The packaging owner independently added normalization of `binary_dir` before
archive assembly. This supersedes the earlier finding that the shared script
still contained the relative-path defect. The rechecked script SHA256 is
`005b4e9bc850aef5328190272b9fe40aca8287747b98050b13eba0d4b77b0647`;
this review did not apply its older private patch over that owner's edit.

A private copy of the current script and plist passed `bash -n` and actual
archive creation with relative paths, absolute paths, and paths containing
spaces. Each archive had all 10 expected payload entries, valid version
`0.1.0`, the `agentconfig` scheme declaration, matching executable SHA256
values, and executable permission bits. macOS also generated 10 AppleDouble
metadata entries on this volume; the first test's exact-entry-count assertion
rejected these. The completed check separately identified those entries and
verified the payload without silently treating metadata as program content.

The source script and plist stayed unchanged during the checks. All three test
output directories were removed; scripts, logs, and the structured receipt
remain in `/Volumes/Datas/codex-native-packaging-current-daxipv2y/`.

This is archive-assembly evidence using the previously validated Release
binaries, not a rebuild or runtime acceptance of the new concurrent source.
No native UI driver was started while the interface owner was testing.

## Latest combined validation and consolidation — 2026-11-14

The interface owner corrected the missing `provider_field_label` test import in
the shared source. The private candidate and patch were not applied over that
owner's changes. Its redundant Cargo process was interrupted after confirming
that the shared import was fixed; that interrupted candidate run is not a
product compilation failure or an acceptance PASS.

The shared-workspace run in
`/Volumes/Datas/codex-native-combined-validation-w6ez4p61/` passed:

| Check | Result |
| --- | --- |
| Rustfmt and actionlint | PASS |
| Locked workspace check, all targets | PASS |
| Locked workspace Clippy, all targets, warnings denied | PASS |
| Workspace tests | PASS: 273, none ignored |
| Locked Release build, all workspace binaries | PASS |
| `git diff --check` | PASS |

Three desktop inputs changed during that run's Release build:
`agentconfig_links.rs`, `interactions.rs`, and `settings/agentconfig.rs`.
Consequently this first run alone does not establish a single stable candidate.

A targeted consolidation in
`/Volumes/Datas/codex-native-desktop-consolidation-pn4savga/` passed formatting,
desktop all-target Clippy, desktop tests, and a new Release build of `volt`.
The links file changed again during its test step; its subsequent Release
build had no input changes.

The final check in
`/Volumes/Datas/codex-native-final-desktop-check-w9yo4ut3/` then passed formatting,
desktop all-target Clippy with warnings denied, and all 216 desktop tests.
Its starting inputs exactly matched the preceding stable Release build, and no
inputs changed during the final check. The core, dependency manifests/lockfile,
and workflow hashes also matched the earlier complete workspace run, retaining
the 57-test core receipt. These linked receipts close the identified
concurrent-edit validation gaps without treating an earlier snapshot as current.

The resulting `volt` Release SHA256 is
`10211c40eb3dac6c5b9ae6dcea9942635dd97282aba8dfad13190d6dbeb5e33f`.
The `block 0.1.6` future-incompatibility warning remains. Checks used the
existing dependency cache with Cargo networking disabled.

This closes the local compilation failure and packaging-path finding.
Windows runtime, real-provider behavior, complete desktop/browser acceptance,
the interrupted workspace-tool independent review, and remote CI/release
evidence remain as documented above. The latest runtime checks in the other
active chats were not represented as this review's own GUI acceptance.

Changes remain local; no commit, push, release, or deployment was performed.

## Acceptance continuation checkpoint — 2026-11-14

The preceding goal turn made progress by closing compilation and packaging
findings and linking the stable build/test receipts. This checkpoint initially
recorded a **verified wait**. The Review follow-up below subsequently made
implementation progress in this same goal turn.

- The recorded source hashes still matched the final validation receipt at
  the beginning of this continuation.
- A fresh GitHub query for the current HEAD again returned only completed,
  failed run `41166775791`, created on 2026-10-31. There is no remote CI job
  currently running for this HEAD to await.
- The host remains macOS without `pwsh`. Neither `OPENAI_API_KEY` nor
  `SENSENOVA_API_KEY` is present in this task process; only presence was
  inspected.
- The actual turns `01a1da65-da59-7de1-8f20-b0299bea25f9` in
  “Add agentconfig URL import” and
  `01a1da5e-14cf-7542-ae1d-48dd57c92c5f` in
  “继续按 zcode 样式复刻” were polled twice and remained `inProgress`.
  The latter reported measured browser reference controls; this is not this
  review's native GUI acceptance. No source ownership was taken from either
  active turn, and no parallel native UI driver was started.

This is the first external-gate checkpoint after the latest local consolidation.
The goal remains active/PARTIAL. Windows, real-provider, complete GUI/browser,
independent-review, and remote-publication requirements are unchanged; the
ongoing URL-source feature is not silently added as a new remediation project.

At the end of this checkpoint, a fresh inventory including added and deleted
paths found a change to `crates/pi-agent-core/src/agent_config.rs`. This is in
the active URL-source owner's scope. The prior 273-test and Release receipts
remain valid only for their recorded inputs, not for the now-changing combined
tree. No new test PASS is claimed, and the active owner's file was not modified.

## Review resource-bound follow-up — 2026-11-14

Parent: the production-readiness review and acceptance continuation above.
Source: the continuing user goal. Reason: the active settings chat reported two
Review tests timing out in Git; inspecting the unchanged `review.rs` additionally
found that Git output was captured without a memory bound before preview
truncation, and untracked-file reads were bounded only by a prior metadata check.
The timeout report does not yet establish the cause of those test failures.

Scope: `crates/pi-desktop/src/review.rs`, its regression tests, and this report.
Risk: medium. Orchestration: Native, one writer and one independent read-only
verifier. Other active writers retain their current scopes. The existing
15-second Git deadline is retained; this follow-up will not fix a failed test
by silently increasing it.

Module validation uses a temporary crate importing the actual Review source,
so the unrelated AgentConfig rewrite does not have to compile before these
resource contracts can be tested. This temporary fixture is not a new shipped
acceptance framework or a substitute for the final desktop build.

The initial regression failed against the previous implementation: a real Git
command returned and retained **16,777,217 bytes** instead of rejecting output
above the file-list limit. The corrected implementation:

- Concurrently drains stdout and stderr while retaining bounded prefixes.
  Successful file-list output over 16 MiB fails explicitly instead of producing
  a partial list. Diff previews retain at most 1 MiB plus the truncation sentinel
  byte; stderr retains at most 64 KiB.
- Still waits for Git's exit status. A nonzero status remains an error even when
  diagnostic text is truncated, and the diagnostic reports its truncation.
- Keeps null stdin, the 15-second deadline, and `kill_on_drop`. Pipes continue
  draining after the retained prefix fills, avoiding a full-pipe deadlock.
- Bounds untracked-file reads while reading, including file growth after a
  metadata check. Existing regular-file/path checks and preview markers remain.

The temporary fixture's strict Clippy, formatting, all **8 module tests**, and
scoped diff checks passed. Three tests are new: oversized complete Git output,
large Git diff drainage/truncation, and file growth after metadata inspection.
The reviewed source hash stayed
`df37c27ed7c08971c8f1d21885a1f8682e68c23daef471c3c63bda1fa12b5d8d`
throughout validation. Baseline and corrected receipts are in
`/Volumes/Datas/Workspace/codex-native-review-limits-zga39d_y/`.

The existing Review Git tests also passed in this module execution. That does
not establish the cause of the two Git timeouts reported by the settings chat,
or replace its failed full-desktop receipt. No deadline was increased.

Independent read-only verifier `Helmholtz` completed its review and found one P2:
a NUL byte discarded after the retained Git prefix could lose the old binary
preview classification. An actual Git regression reproduced that finding
before correction. Capture now scans every drained chunk for NUL bytes without
retaining the discarded bytes, and the preview uses that flag.

After correction, formatting, strict module Clippy, all **9 module tests**, and
the scoped diff check passed with unchanged source hash
`83fca3fbe7172b91783f918e99fc2e173d6cea1e52bc4ee0b20b99eac38e9ac0`.
The verifier reported no other issue in the requested scope, did not run Cargo,
and did not issue a second review after this correction. It inherited the host
model because the requested default was absent from the available model list;
no alternate host runtime was used.

`cargo check --locked -p pi-desktop --all-targets` also passed against the shared
workspace with no input changes during the check. The subsequent integration
receipts below define the acceptance scope of this new Review change, rather
than the older 273-test snapshot.

The first integration attempt stopped at Rustfmt while the active AgentConfig
owner changed `agentconfig_sync.rs`, `desktop_state.rs`, and
`settings/agentconfig.rs`. Those files were not reformatted by this review.
After the owner reported a complete desktop test run, a second workspace run
passed all seven gates, including **288 tests** and all Release binaries.
However, `settings.rs` changed during its test step, so this run alone was not
a stable-candidate receipt.

A desktop consolidation then passed formatting and strict Clippy but failed
with **230 passed, 1 failed**:
`desktop_state::tests::round_trip_history_preferences_drafts` could not acquire
the state-file lock on repeated save. No inputs changed during that run. Its
failure is retained in `desktop-final-validation.log` and its receipt.

The AgentConfig owner subsequently changed `desktop_state.rs`, adding an
explicit unlock guard and a regression that performs 100 saves while 25 child
processes start. That owner reported 232 passing desktop tests. This review
confirmed the source changed after its failed run and retained ownership with
that chat.

Direct validation of the corrected state then passed formatting, desktop
all-target Clippy with warnings denied, **232 desktop tests**, a matching Release
build, and `git diff --check`. No inputs changed during any check or across the
run. The earlier full-workspace run's core, dependency/lockfile, and workflow
hashes still matched, retaining its **57 core tests** and the unchanged CLI
Release builds. These linked receipts cover **289 tests**; the 9 Review tests
are included in the desktop count.

The final direct receipt is `state-fix-validation-receipt.json` in this
follow-up's evidence directory. The resulting `volt` Release SHA256 is
`7b4ba1bece1c391278328cba1d84fde979df66e68f7efdd3c0c1d060430b38d5`.
The failed intermediate receipts remain available instead of being overwritten
by later passing results. The third-party `block 0.1.6` warning remains.

The remaining runtime environment was also checked. No PowerShell or common
Parallels, Fusion, VirtualBox, UTM, or QEMU command runtime was found on PATH or
the checked installation paths. Read-only requests to the usual local Ollama
and LM Studio model inventories returned no usable inventory; no model
completion request was sent. This does not supply Windows or real-provider
acceptance evidence.

This goal turn made implementation and validation progress. Complete native
GUI/browser acceptance, Windows execution, real-provider behavior, the earlier
interrupted workspace-tool review, and remote CI/release publication remain
unaccepted. The goal remains active/PARTIAL. Changes remain local; no commit,
push, release, or deployment was performed.

## Remaining HTML launch entry follow-up — 2026-11-14

Parent: the original HTML-preview remediation and production-readiness review.
Source: the continuing user goal. Reason: a final call-site inspection found
that the lower artifact-preview button still used a fixed temporary filename,
an overwriting write, and a separate platform opener that ignored failures.
The earlier statement that both preview entry points were unified missed this
third launch control.

Scope: the HTML launch controls and their descriptions in
`crates/pi-desktop/src/panels.rs`, plus this report. Risk: medium.
Orchestration: Native, one writer and one independent read-only verifier.
The active interface owner continues to own the settings Select changes;
this follow-up did not edit those files or start another native UI driver.

The lower control now calls the same `open_html_artifact` helper as the header
control and message action. It therefore uses the existing size limit, unique
exclusive file creation, private Unix permissions, and opener-error handling.
Both panel controls notify the interface after success or failure. Wording now
describes a request to open in the default browser; it no longer claims that an
independent GPU window or sandbox is ready.

Independent read-only verifier `Boole` compared the saved pre-edit panel file
with the current callback and reviewed the shared helper and its tests. It
reported a scoped PASS with no new findings. It did not run Cargo, operate the
GUI, or accept actual browser rendering or Windows behavior. This review is
separate from the earlier interrupted workspace-tool review.

Evidence is retained in
`/Volumes/Datas/Workspace/codex-native-html-entry-pfzq9nht/`.
The previous 289-test receipt did not cover the concurrent settings changes
identified at the start of this continuation.

### Final linked validation — 2026-11-15

The first attempt passed panel formatting and desktop all-target strict Clippy
with unchanged inputs, but its desktop test run did not pass. That attempt set
`TMPDIR` to a private directory on the data volume. Five Review tests emitted
failures, and the save-lock child-spawn stress test ran for over a minute.
The test process remained in an uninterruptible wait and was explicitly
terminated; Cargo returned 101 after approximately 600 seconds. The source
inputs stayed unchanged. Its log and receipt are retained, and no test deadline
was increased.

A second attempt restored `TMPDIR=/tmp`, then stopped at compilation: the
active interface owner's new recovery module lacked the trait providing
`h_flex`. That owner independently corrected its module. This follow-up did not
edit the recovery or settings files. The failing compile receipt is retained.
A private copy of the test binary available after that correction then passed
all 237 tests in 1.41 seconds, including the new recovery tests; this copied
binary run alone was not used to establish the current source mapping.

The final direct Cargo run passed:

| Check | Result |
| --- | --- |
| Workspace Rustfmt | PASS |
| Desktop all-target Clippy, warnings denied | PASS |
| Desktop tests | PASS: 237, none ignored; 2.71 seconds in the test harness |
| Release `volt` build | PASS |
| `git diff --check` | PASS |

No validation input, including newly added or removed matching files,
changed during any check or across the complete final run. Core sources,
dependency manifests/lockfile, and workflow inputs still matched the earlier
57-test workspace receipt. These linked receipts cover **294 tests** and the
unchanged CLI Release builds. The data-volume test attempt remains a failed,
interrupted run; the successful system-temp run does not prove its exact cause
or accept that data-volume test environment.

The final receipt is `final-validation-receipt.json`, with source inventories
in `final-source-before.json` and `final-source-after.json`. Release `volt`
SHA256:
`e89e2a169c6bf379e1971ab5660f94badaf9b5aa29d96531b177df7d23355c67`.
A matching private executable is preserved as `volt-validated` in this evidence
directory for later runtime acceptance. The closing inventory found no source
drift after validation. The temporary test-binary copy was removed after its
hash and results were recorded.
The third-party `block 0.1.6` future-incompatibility warning remains.

The interface owner's latest acceptance attempt reported a locked Mac.
No competing native UI driver was started, and actual clicks, error feedback,
and browser rendering of these controls remain unaccepted. Windows,
real-provider, interrupted independent-review, and remote CI/release evidence
are still outstanding. This goal turn made implementation and validation
progress; the goal remains active/PARTIAL. No commit, push, release, or
deployment was performed.

## External acceptance checkpoint 1 — 2026-11-15

The preceding goal turn made progress by correcting the remaining HTML launch
control and completing the stable 294-test linked validation. This continuation
is an external-gate audit, not a new implementation or acceptance PASS.

- The initial dynamic input inventory found no changes from the final
  validation snapshot. The preserved `volt-validated` executable still matched
  SHA256 `e89e2a169c6bf379e1971ab5660f94badaf9b5aa29d96531b177df7d23355c67`.
- A fresh native-tool inventory returned: “The Mac is locked and automatic
  unlock could not unlock it. Ask the user to unlock the Mac manually before
  continuing.” No native interaction or system-setting change was attempted.
- Neither `OPENAI_API_KEY` nor `SENSENOVA_API_KEY` is present in the test
  process. Only presence was checked; no credential or real-model response
  was obtained.
- No Windows test target was supplied; this macOS process still has no
  PowerShell runtime. Earlier packaging and offline tests do not establish
  Windows runtime acceptance.
- The interface owner's new queue-panel turn is active. That is a separate
  local implementation scope, not a running Windows, real-model, or browser
  acceptance job. Its files and native UI were not modified by this audit.

The genuine remaining gates are unchanged: actual native/browser interaction,
Windows execution, a real-provider request, the interrupted workspace-tool
independent review, and remote CI/release acceptance. No authorized action in
this audit supplied those missing environments or receipts. The goal remains
active/PARTIAL at the first external-gate checkpoint after implementation
progress; no commit, push, release, or deployment was performed.

## External acceptance checkpoint 2 — 2026-11-15

The preceding continuation produced no new implementation or acceptance
receipt. A second live audit found the same external conditions:

- Native inventory again explicitly reported that the Mac is locked and
  requires manual unlock.
- The dynamic source inventory still matched the validated snapshot, and the
  preserved Release candidate retained the recorded SHA256.
- The test process still has no nonempty OpenAI/SenseNova credential, no
  PowerShell runtime, and no supplied Windows test target.
- No native/browser runtime acceptance, real-model response, Windows result,
  replacement independent-review receipt, or authorized remote CI/release
  receipt was obtained.

No own validation process remains running to await. Existing local tests were
not repeated because their inputs did not change. This is the second
consecutive external-gate audit after implementation progress; the goal remains
active/PARTIAL. No code, credential, system setting, or remote state was changed.

## External acceptance checkpoint 3 and blocked audit — 2026-11-15

The preceding continuation made no implementation or acceptance progress.
The third audit independently obtained the same native-tool result: the Mac is
locked and requires manual unlock. The test environment still has no nonempty
OpenAI/SenseNova credential, no PowerShell runtime, and no supplied Windows
test target. No real-provider request or Windows execution receipt was obtained.

The source inventory now differs from the preserved candidate in
`crates/pi-desktop/src/panels.rs` and the newly added
`crates/pi-desktop/src/panels/queue.rs`. The interface owner's live turn is
actively correcting and testing those queue-panel changes. Its observation
timeout was not treated as completion, and this review did not modify its
files or compete for native UI control. Inspection confirmed that both panel
HTML launch callbacks still call `open_html_artifact`; the fixed filename
path has not returned.

The preserved `volt-validated` executable still matches
`e89e2a169c6bf379e1971ab5660f94badaf9b5aa29d96531b177df7d23355c67`.
Its linked 294-test, Clippy, formatting, and Release receipts remain valid for
that snapshot. They are not a new compilation or test PASS for the active
queue-panel changes.

| Remaining requirement | Evidence boundary and blocker |
| --- | --- |
| Native workflow and actual browser preview | Three live inventories explicitly report a locked Mac; manual unlock is required. |
| Windows installation and browser behavior | No Windows test target is supplied; macOS/offline checks cannot prove this behavior. |
| Real-model response and provider lifecycle | The test process has no usable model credential or supplied live test endpoint. |
| Workspace-tool independent review | The earlier platform-rejected run remains interrupted; scoped reviews of other changes do not replace it. |
| Remote CI/release acceptance | The local candidate remains uncommitted/unpublished; remote writes have not been authorized. |
| Newly active queue-panel work | Owned by the interface chat; its local build/test activity does not supply any of the missing external receipts above. |

No validation process owned by this review remains live to await. All safe local
remediation within the frozen review scope has been implemented and has the
documented snapshot evidence. Repeating those tests or adding surrogate
acceptance fixtures would not establish the remaining actual-runtime gates.
The same external condition has persisted across checkpoints 1, 2, and 3,
without an available authorized action that supplies the missing environment.

The production-readiness goal is **blocked** and acceptance remains **PARTIAL**,
not complete. Continue when the Mac is manually unlocked and the necessary
Windows/model test access or authorized external verification becomes
available. Changes and evidence are preserved; no commit, push, release,
deployment, credential, or system-setting change was made by these audits.

## Resumed native/browser acceptance — 2026-11-15

Parent: the production-readiness review and remaining HTML launch follow-up.
Source: the user's explicit goal resume. Reason: a fresh native inventory now
returns applications without a lock error, so the previous locked-desktop
blocker no longer describes the current environment. The blocked audit starts
fresh for this resumed run; the production acceptance requirements are unchanged.

The interface owner is currently performing native button verification in its
own preview. This review first prepares its isolated fixture and current-source
validation without competing for native input or altering that owner's app.
AgentConfig's source-management owner is idle; its runtime scope is not expanded.

The initial integration attempt stopped at workspace Rustfmt. It reported
three formatting differences in the artifact pane, while
`workbench/run_notice.rs` changed during that check. Its failed receipt is
preserved in
`/Volumes/Datas/Workspace/codex-native-resumed-acceptance-u6h9n0ln/`.
It did not establish a new compilation/test/build PASS.

The follow-up changes only those artifact-pane formatting expressions and the
message card's “independent Web window” label. That control already opens the
default browser through the shared helper; its label now says browser preview.
Risk: low for these text/format changes. Orchestration: Native, external=0.
No queue/run-notice behavior or active writer's implementation was changed.

Prepared private normal and missing-opener app bundles, each with its own
workspace, state, artifact directory, loopback endpoint, and synthetic saved
conversation. Fixtures cover Unicode text, a native card, malformed JSON retained
as text, and an HTML counter. The missing-opener case uses a private empty PATH
to test actual launch-failure feedback. No fixture app has been launched by this
review yet. Real preferences, credentials, and existing app instances are not
test inputs. The subsequent validation and observed runtime receipts will define
the new acceptance boundary.

## HTML preview notification follow-up — 2026-11-15

Parent: the resumed native/browser acceptance. Source: the continuing user goal
and inspection of the three HTML launch controls. Reason: `status_message` has
no rendered consumer, so the two panel buttons silently lost error feedback.
The message-card entry also assigned browser errors to `run_error`, conflating
preview failures with model failures.

Scope: the shared preview UI action in `execution.rs`, its two panel callers,
and the already pending message-card label/format correction. Risk: medium.
Orchestration: Native, one writer and one independent read-only verifier.
This scope does not include queue/run-notice changes or the earlier interrupted
workspace-tool review.

All three controls now use `open_html_preview`. A launch error uses the existing
GPUI notification layer, remains visible until dismissed or replaced, and has
a shared identity to prevent duplicate notices. A successful open request clears
the previous preview error. The method borrows app state immutably and does not
assign model execution fields. Existing size, path, permission, and opener
validation stays in `open_html_artifact`.

Implementation is pending the current-source deterministic and actual-browser
receipts. The prior test snapshot does not cover this change.

Independent read-only verifier `Lagrange` returned no findings for this follow-up.
It verified notification mounting, persistent deduplication, success removal,
all three callers, and separation from model execution state against the installed
GPUI 0.7.3 sources. This was source verification only, with no Cargo or native
interaction, and does not replace the earlier interrupted workspace-tool review.
The preferred `gpt-5.3-codex` was unavailable in the exposed model inventory;
the verifier inherited the current Codex model without changing host runtime.

The follow-up's final serial validation passed:

| Check | Current follow-up result |
| --- | --- |
| Workspace Rustfmt | PASS |
| Strict desktop all-target Clippy | PASS |
| Desktop tests | PASS: 249 passed, 0 failed, 0 ignored |
| Release `volt` build | PASS |
| Tracked diff and report whitespace | PASS |
| Independent notification source review | No findings |

The dynamic source inventory did not change during any check or across the
validation run. Core, dependency, and workflow inputs still match the linked
57-test core receipt; the new desktop receipt plus that earlier core receipt
cover 306 linked tests, rather than a freshly rerun full-workspace suite.
The existing third-party `block 0.1.6` future-incompatibility warning remains.

Evidence: `notification-validation-receipt.json`, `notification-validation.log`,
the before/after inventories, and `notification-independent-review.json` under
`/Volumes/Datas/Workspace/codex-native-resumed-acceptance-u6h9n0ln/`.
Preserved candidate: `volt-notification-validated`, SHA256
`1cbc35b22f11b2f34d8391fbd299ecd65b2fdb636a2f554716184da2dc8c98fa`.
Both private browser-test bundles now contain that exact binary. They have not
been changed since preparation. The native/browser acceptance below used both
bundles after the interface owner's turn ended. These local results do not
establish Windows, real-provider, full desktop workflow, or remote CI/release
acceptance.

## Current candidate native/browser acceptance — 2026-11-15

The interface owner's turn completed and released the desktop before these
checks started. Both fixture apps used the preserved Release SHA256 above,
private canonical workspaces/state directories, and a loopback model endpoint.
Provider credentials were removed from the fixture launch environment.

Observed using native accessibility and screenshots:

- Unicode text and the uppercase `JSON-UI` native card rendered correctly.
  Malformed JSON remained visible as text.
- The panel's lower launch button, panel header button, and message-card button
  each opened a different HTML file in the actual default browser, Microsoft
  Edge. Each page showed the expected Unicode content, and its inline counter
  changed from 0 to 1 after a real button click.
- With an empty private PATH, all three entries displayed the visible
  “产物打开失败: 无法启动默认浏览器” notification. Repeated failures retained one
  notification, and no model Retry appeared in the observed app state.
- The notification remained visible across navigation and could be dismissed.
  Two coordinate attempts at the header failed with native-tool window-position
  errors; a fresh AX tree successfully targeted that control. Those failed
  attempts are not counted as application launch failures.
- The private artifact directory contained no files after failed launches.
  Adding a symlink to the actual `/usr/bin/open` inside the fixture's private
  PATH allowed a new request to open in Edge and clear the old notification.
  The recovered page's counter also changed from 0 to 1.

All four successful artifact files have different names, identical expected
content hashes, injected theme tokens, and Unix `0600` permissions. Both fixture
processes were stopped through their own launchers and reached terminal state.
Only the four observed fixture browser tabs were closed.

Receipt: `gui-browser-acceptance-receipt.json` and the two process receipts under
`/Volumes/Datas/Workspace/codex-native-resumed-acceptance-u6h9n0ln/`.
Screenshots and accessibility observations are retained in this chat's tool
record. These are light-theme browser checks; they do not prove keyboard,
screen-reader, dark-theme browser, or complete desktop-workflow acceptance.

The existing theme injector also adds a remote Tailwind script. The synthetic
saved conversation needs no live model request, but this test did not isolate
browser networking and does not establish network-free offline rendering.

The HTML notification follow-up has passed its scoped implementation, source
review, deterministic, and native/browser gates. Overall production acceptance
remains **PARTIAL**. The resumed goal remains active after this new progress;
the earlier three-checkpoint blocked audit does not apply to this resumed run.

## Artifact accessibility follow-up — 2026-11-15

Parent: HTML preview notification follow-up. Source: the user implement-all
instruction and continuing production goal. Reason: the observed native
accessibility tree exposed unnamed artifact controls despite visible child text.
Scope: seven artifact control names in `generative_ui.rs` and `panels.rs`, plus
scoped keyboard and dark-theme native/browser acceptance. Risk: low.
Orchestration: Native, one writer, external=0. Existing settings and other
owners remain outside this scope.

The previous candidate remains preserved. The follow-up requires a new
source-bound build and actual AX/keyboard evidence before claiming acceptance.
Evidence directory: `/Volumes/Datas/Workspace/codex-native-artifact-accessibility-f08f511b/`.

All seven controls now expose explicit accessibility names while retaining their
visible child content and existing handlers. The mode button shares its current
action text with its accessibility name. Panel names use the existing Chinese
and English translation path. No dependency or production test fixture was added.

The current-source serial validation passed:

| Check | Result |
| --- | --- |
| Workspace Rustfmt | PASS |
| Strict workspace all-target Clippy | PASS |
| Full workspace tests | PASS: 306 passed, 0 failed, 0 ignored |
| Release desktop `volt` build | PASS |
| Tracked diff and report whitespace | PASS |
| Owner clean-code-guard pass | Clean |

This is a freshly rerun full-workspace receipt: 57 core and 249 desktop tests.
No production, dependency, packaging, or workflow input changed during validation
or the subsequent GUI checks. The third-party `block 0.1.6` future-compatibility
warning remains. Preserved binary: `volt-accessibility-validated`, SHA256
`996411c08cae87487937360bee440d93a43661ea02ce6865e3e747f3e6e3f9d1`.

Two new private fixtures used this exact binary with isolated state/workspaces,
credentials removed from their launch environment, and a loopback model endpoint.
Observed through native accessibility, screenshots, and real keyboard input:

- The three message-card actions and four panel actions have names in the native
  accessibility tree. Chinese/light and English/dark panel labels match their
  visible actions; Source/Preview names change with the view.
- Tab moved a visible native focus ring. Return switched to source, Space
  restored preview, Tab/Return opened the header browser action, and Tab/Return
  closed the light fixture's artifact pane.
- The light browser page's counter changed from 0 to 1 with Tab/Return; the dark
  page's counter changed from 0 to 1 with Tab/Space. Both themes rendered in the
  actual default browser. The dark panel's named close action also worked.
- Both artifact files preserve the original payload, carry the expected light
  or dark background token, and have Unix `0600` permissions. Only the two
  observed fixture browser tabs were closed, and both fixture processes reached
  terminal state through their own launchers.

One dark launch attempt used an accessibility index invalidated by switching
views. The tool rejected it; retrying the fresh observed index succeeded.
This is a tooling error, not a failed application launch.

Native AX continued to report window focus even when screenshots showed moving
control focus rings. This scoped keyboard evidence is not live VoiceOver or
screen-reader acceptance. The existing remote Tailwind injection remains, so
these browser tests do not establish network-isolated offline rendering.

Receipts: `accessibility-validation-receipt.json`,
`gui-accessibility-acceptance-receipt.json`, and `guard-review.json` in the
evidence directory above. The earlier notification/browser receipt remains
preserved separately. The earlier interrupted workspace-tool independent review
also remains unresolved; this low-risk owner guard pass does not replace it.
Overall production acceptance remains **PARTIAL**, and the resumed goal remains
active following this implementation and acceptance progress. No commit, push,
release, or production deployment was performed.

## Desktop execution durability follow-up — 2026-11-15

Parent: artifact accessibility follow-up. Source: the continuing production goal.
Reason: `start_local_command` appends the pending tool call after the last
synchronous save, and dispatch continues even if saving the prompt fails. A
child can therefore act before its tool record is durable, or execute while
another instance owns a newer state revision.

Scope: execution checkpoints and local-command dispatch in `execution.rs`, plus
isolated success, failure, cancellation, queue and restart acceptance. Risk:
medium. Orchestration: Native, one writer and one independent read-only verifier.
Other owners' settings and interface changes remain outside this scope.

The preceding candidate is preserved for old/new runtime comparison. Current
implementation and acceptance are pending; the prior 306-test receipt does not
cover these edits. Evidence directory: `/Volumes/Datas/Workspace/codex-native-lifecycle-yz2twbny/`.

The implementation now checks the initial prompt save before either local or
provider dispatch, and checkpoints the pending local command before spawning it.
A failed local checkpoint gets an explicit matching “command was not started”
tool result. Both errors use the existing failure/queue-pause completion path.

Independent read-only verifier Kepler found no actionable issues in this scoped
diff. It confirmed that failed checkpoints return before execution worker
creation and do not consume a queued prompt. Existing read-only review refresh
can still run; the guard is for execution dispatch, not every background task.
This does not replace the earlier interrupted workspace-tool review.

The source-bound serial checks passed: workspace formatting, strict workspace
all-target Clippy, all 306 workspace tests, desktop Release build, and tracked
diff checks. No inputs changed during validation. Preserved candidate:
`volt-lifecycle-validated`, SHA256
`c42013713d219551a02bccf255effa2eea2f287fdfbc27731d384312044e397f`.
The existing third-party `block 0.1.6` warning remains. Runtime old/new crash,
queue, and conflict acceptance is pending until the other active UI owner's
desktop checks finish. No acceptance fixture has been launched by this follow-up.

The later settings owner's `interactions.rs` edit is now included in a combined
source snapshot. Its formatting and strict workspace Clippy passed. The first
combined test command passed all 306 ordinary tests but failed at the doctest
stage because rustdoc could not locate a shared `reqwest` artifact. That file was
readable when inspected afterwards; the cause is unproven. The failed receipt
and log are retained, and no shared cache was removed or invalidated.

A full workspace test retry, including doctests, passed, followed by the Release
build and diff gate. The source inventories for the earlier format/Clippy pass
and retry are identical. Combined receipt: `combined-gate-receipt.json`.
Preserved combined candidate: `volt-lifecycle-combined-validated`, SHA256
`4bc6ccefce2c9d8b41cf79179eed1040cfb7c7335de2a81e3b35408edc482c46`. The private after-fix fixture now contains
this exact candidate; no native lifecycle fixture has been launched yet.

## Model-tool history checkpoint follow-up — 2026-11-15

Parent: desktop execution durability follow-up. Source: the continuing production
goal. Reason: a model tool previously started after sending `ToolCallStarted`,
while the desktop only scheduled a debounced history save. Sending an event did
not establish that the pending call had been saved before its side effects.

Scope: `agent.rs`, the specialist construction in `delegation.rs`, desktop
`execution.rs`, and `tests/tool_checkpoints.rs`. Risk: medium. Orchestration:
Native, one writer and one independent read-only verifier.

- The desktop supplies an acknowledgement channel for parent model tools. Each
  tool waits for the matching history checkpoint before execution.
- The desktop applies the already queued start event and prior results in order,
  then saves synchronously before acknowledging. A failed save, closed receiver,
  lost acknowledgement, or 30-second acknowledgement timeout prevents execution.
  Checkpoint rejection emits an explicit known-not-started tool result.
- Cancellation drops the pending execution future, so a late acknowledgement
  cannot start the tool. Losing the desktop entity aborts the worker and closes
  both receivers before joining it.
- Read-only specialists keep their existing workflow reporting and restricted
  registry; their calls do not enter the parent's chat checkpoint channel.
  Other core hosts retain their existing behavior unless they supply a channel.

Independent verifier Raman found one P2 issue: a stopped checkpoint could drain
but discard a previous tool's completed result. The generation check now runs
first, buffered events are applied next, and the stopped check follows them.
Raman rechecked the correction and the final source hashes with no remaining
findings. This is source review, not GPUI stop-order acceptance, and does not
replace the earlier interrupted workspace-tool review.

Seven loopback integration tests use real temporary filesystem writes to verify
per-tool acknowledgement, rejected checkpoints, closed checkpoint and event
receivers, lost acknowledgements, cancellation with late acknowledgement, and
acknowledgement timeout. They use no real provider credentials.

Evidence directory:
`/Volumes/Datas/Workspace/codex-native-lifecycle-yz2twbny/model-tool-checkpoints/`.
The immutable `source-snapshot-v3` includes Rust sources, manifests, assets,
fixtures, and the existing ignored `.cargo/config.toml`. Validation explicitly
uses the repository's Rust 1.97.1 toolchain and runs Cargo serially.

| Check | Result |
| --- | --- |
| Formatting of the four scoped files | PASS |
| Strict workspace all-target Clippy | PASS |
| Full workspace tests, including doctest stage | PASS: 315, zero failed or ignored |
| Desktop Release build | PASS |
| Current workspace formatting and tracked diff whitespace | PASS |
| Independent scoped source review and owner guard | PASS |
| Native crash, Stop, queue, conflict, and model checkpoint interaction | PENDING |

The test count comprises 64 core tests and 251 desktop tests. The candidate is
`volt-model-checkpoints-validated`, SHA256
`cbf1abe9ffc17ab90079d222907299510df061c8e8b22cb6117897dc9bbd5fed`.
Receipt: `frozen-v3-validation.json`; source manifest:
`frozen-source-v3.json`. No frozen inputs changed during any successful check.
The existing third-party `block 0.1.6` future-incompatibility warning remains.

The live settings owner subsequently wrapped one import in `select_motion.rs`.
It is the only source difference from the preserved candidate; formatting the
captured file produces the current file exactly. The current workspace format
check passes. The preserved binary still belongs to its exact frozen snapshot.

Failed attempts are retained: the first snapshot captured the concurrent
settings import before formatting; the next attempt omitted ignored registry
configuration and failed offline before compilation; a subsequent attempt used
the snapshot directory's default Rust 1.94.0 instead of the project's 1.97.1 and
failed compiling `gpui-pre`; strict Clippy then identified one redundant
`anyhow::Error` conversion in the checkpoint callback. The final snapshot
includes the registry configuration and conversion fix and pins the toolchain.
No dependency cache was deleted and no failed receipt was rewritten.

The active settings task still owns native desktop interactions, and no
lifecycle fixture has been launched. Windows runtime, real-provider credentials,
live screen-reader and complete desktop acceptance, the interrupted
workspace-tool review, and authorized remote CI/release remain outstanding.
Environment checks found neither provider key environment variable and no local
Windows runtime. All changes remain local and uncommitted; overall acceptance remains
**PARTIAL** and the production goal remains active after this implementation and
verification progress.

## Recovery result scoping follow-up — 2026-11-15

Parent: model-tool history checkpoint follow-up. Source: the continuing
production-readiness goal. Reason: recovery previously collected completed tool
IDs across the entire history. A result belonging to another assistant response
could therefore hide an interrupted call when its ID was reused.

Scope: recovery and its tests in `execution.rs`. Risk: medium. Orchestration:
Native, one writer and one independent read-only verifier. The implementation
matches results only within the contiguous Tool records following each message.
It preserves original records and result order and inserts explicit unknown
outcomes for missing calls. The scan is expected linear in the history size.

A regression saves and reloads a real session containing a completed call and a
later pending call with the same ID. The original matcher failed: three messages
were retained instead of the expected four, omitting the interruption result.
The failed test and source hash are preserved in `regression-before-fix.json`
and its log.

After the fix, all nine execution tests passed, including the new persisted-ID,
response-boundary and result-order cases. Subsequent test-only additions cover
System separation, empty history, leading orphan results and typed Tool records
without a `tool_result`. Original records are preserved and repeated recovery is
idempotent. This does not establish arbitrary malformed-record repair.

The ten final execution tests subsequently passed using the settings owner's
desktop test artifact, SHA256
`5afbc1b723512372f5ad47938ca41aeb144711dbfdffdeced5b1dccd11b98d60`.
The execution module in its source copy matches the frozen module hash below;
the binary hash was checked before and after running the tests. The new empty
history/orphan-record test ran and passed. The broader artifact includes later
settings changes, so this is scoped execution proof. Full frozen workspace
Cargo tests and Release remain required. Receipt: `final-scoped-runtime.json`.

Raman reviewed the final scoped source and added boundary cases with no
actionable findings. The current source and frozen snapshot match SHA256
`1c58ce86ae6b4baa8437e1846ef9f4a014fc65be27dfd8e8b416797dd7e5002a`.
This remains source review, separate from native recovery and the previously
interrupted workspace-tool review.

Evidence:
`/Volumes/Datas/Workspace/codex-native-lifecycle-yz2twbny/recovery-result-scoping/`.
The immutable snapshot includes the existing registry configuration and uses
Rust 1.97.1 explicitly. Workspace formatting has passed. The serial frozen
strict workspace Clippy check passed without any source changes. It waited for
an existing Cargo build lock before checking the snapshot's desktop crate.
The full test command later waited for the settings owner's test compilation,
then compiled its own dependencies, desktop and core integration test binaries.
It reached its 1,800-second deadline without producing any test result or source
error. This is a timeout, not a passing suite or a demonstrated source failure.
No owned compiler remained afterwards, and the original receipt and log are
retained. A serial test/Release retry then reached its 3,600-second command
deadline during test compilation, again with no test results or source error.
No owned compiler remained after that timeout. Its source inputs were unchanged,
and `frozen-validation-retry.json` and its log are preserved. Completed format
and Clippy checks are not being repeated.

At the first receipt timestamp, the live source differed from the snapshot only
in the concurrent settings owner's `settings.rs`. The execution file still
matched the independently reviewed hash above. That owner subsequently began
a button interaction follow-up. The retry does not claim coverage of later
settings changes.

The retry session `78842` is terminal. Resource observations found limited internal
volume headroom and existing swap use, but do not establish why compilation is
slow. A completed Cargo sample showed its job queue waiting for a compiler
subprocess. The optional compiler sampler could not finish symbol resolution
and was stopped; the compiler and test command were left running. Kernel
sampling required root and was unavailable. No shared cache was removed.

A local-volume copy is prepared at `/tmp/volt-recovery-source-7ozutgyu`.
All 78 files match the frozen manifest byte for byte (1,671,740 bytes; no
missing or extra files). This removes the external source volume from a future
diagnostic attempt. No validation command has run against this copy yet.
The copy must retain the explicit Rust 1.97.1 override.

Further path inspection found that the shared target's debug `deps`,
`incremental`, and `build` directories are symlinks to the external volume.
Its release cache remains on the internal volume. This identifies external
inputs still used by the timed-out attempts; it does not by itself prove the
cause of either timeout.

Independent debug cache preparation uses same-filesystem clonefile copies of
the settings owner's local dependency files. Workspace library seeds and
workspace/build-script fingerprints are excluded so Cargo recompiles the
workspace and generates native build outputs in the private target. Shared
cache files are preserved. An initial conservative 3 GiB space guard stopped
before copying. The retry uses measured executable sizes and copy-on-write
seeding, with a planned 1 GiB runtime disk floor. Source, Rust version, complete
test scope and debug profile are unchanged; incremental cache storage is
disabled for the private attempt.

The current source is not yet represented by the previous Release receipt.
Native crash, queue, conflict and stop-order replay remain pending shared
desktop availability. Windows, real-provider, live screen-reader and remote
release evidence remain outstanding. Overall acceptance remains **PARTIAL**.

## Post-restart current-source validation — 2026-11-15

Parent: recovery result-scoping follow-up. Source: the user reactivated the
production-readiness goal. Reason: the host restarted at 13:01:49 (Asia/Shanghai)
and the project volume became unavailable. Three successive goal turns recorded
that blocker; the goal was marked blocked with the full acceptance scope intact.
Project-volume access was confirmed restored at 15:33.

The source tree and existing dirty work remain present at HEAD
`72e3afb66b9b611a56f1a55f033674435edef891`. The reviewed execution module still
matches SHA256
`1c58ce86ae6b4baa8437e1846ef9f4a014fc65be27dfd8e8b416797dd7e5002a`.
Earlier receipts and failed attempts are preserved. The previous `/tmp` source
copy and build targets no longer exist; their preparation is historical
evidence, not a completed test run.

A new immutable snapshot contains 82 inputs and 1,700,576 bytes, including
current Rust sources, fixtures, registry configuration, workflow, and packaging
scripts. Inventory and hashes matched before and after capture. It includes
later settings and run-notice changes, so its receipt is separate from the old
78-file frozen snapshot. Source and target are on the internal disk under:
`/Users/zhd/.codex/tmp/codex-native-production-resume-20261115-pyjzi4xp/`.

Validation pins Rust 1.97.1, the existing lockfile, offline dependencies, one
Cargo job, and a fresh private target. The normal debug and Release profiles
are retained; incremental cache storage is disabled. A 5 GiB runtime disk floor
and command deadlines preserve resource interruptions as such.

Formatting and actionlint passed. The all-target workspace check failed with
exit 101 after 325 seconds: two E0277 errors in `run_notice.rs` used
`(&str, &str)` where GPUI requires a typed parent `ElementId`. No source drift
occurred. Strict Clippy, the test inventory, full tests and Release stages did
not run. The UI owner subsequently added `action_key`, which constructs
`(ElementId::from(id), phase)` in live source. This later source is outside the
failed snapshot and requires a separate validation. Receipts:
`capture.json`, `source-manifest.json`, and `validation.json` in the directory
above. No current-source PASS is claimed for an unfinished gate.

Independent read-only reviewer Peirce completed the file/search/Office
functional source review with nine findings. It used the inherited Codex runtime;
the preferred `gpt-5.3-codex` override is not offered by the host. The prior
platform-interrupted review remains interrupted and is not being relabeled.
This new review is separate from the completed execution-module review. Its
result is preserved in `workspace-tool-review-result.json`.

The native settings owner remains active, so no lifecycle fixture is launched.
Crash/queue/conflict/stop-order, Windows, real-provider, live screen-reader,
complete desktop, and authorized remote CI/release acceptance remain pending.
No commit, push, release, or deployment was performed. Overall acceptance remains
**PARTIAL**, and the reactivated goal is **active**.

## Workspace-tool data integrity follow-up — 2026-11-15

Parent: the post-restart independent workspace-tool review. Source: the user's
implement-all instruction and continuing production-readiness goal. Reason:
Peirce identified nine functional defects in the immutable source. Risk: medium.
Orchestration: Native, one writer and one independent read-only Codex verifier.
Scope: `office.rs`, `search.rs`, the Office CLI count caller, and new regression
tests and fixture. Desktop files remain under their existing owner's control.

The findings cover a pipe-only Markdown panic; CSV whitespace and empty-record
loss; Markdown pipe/newline corruption; suppressed search read/traversal errors;
invalid grep paths defaulting to the whole workspace; incorrect TSV defaults;
invalid slide counts defaulting silently; one extra generated slide; and DOCX
text retaining XML entity spellings.

Ten new regressions ran against a separate copy of the reviewed baseline.
All ten failed with concrete assertions or the documented panic, with zero
passing, ignored or filtered tests. This includes explicit delimiter validation
alongside the nine reviewed defects. The original failed workspace snapshot was
preserved unchanged. Evidence:
`/Users/zhd/.codex/tmp/codex-native-tools-correction-20261115-o6b_29hq/`,
including `plan.json`, `regression-before.json` and `regression-before.log`.

The initial correction preserves CSV field text and empty records, escapes
Markdown cell content and validates table structure, reports search errors,
rejects invalid explicit paths, resolves TSV defaults, validates total slide
counts in the tool and CLI, and decodes DOCX XML text references once. Accepted
slide counts are 3–100, including the title, agenda and Q&A. Recursive grep
continues to skip non-UTF-8 files; explicitly selected unreadable text and
incomplete traversal return errors. Targeted/full validation and independent
recheck remain pending; no passing current-source result is claimed yet.


### Parser refinement and complete workspace candidates

Peirce's correction-v1 recheck found three remaining P2 boundaries: Unicode
boundary whitespace, balanced malformed CSV quotes, and a terminal escaped pipe
mistaken for a Markdown border. The Unicode issue was also reproduced by
roundtripping a real CSV file through the corrected Office CLI. Its receipt is
`unicode-boundary-probe/receipt.json` in the corrective evidence directory.

The new 13-test suite ran against immutable correction-v1 production source:
10 original regressions passed and all three new regressions failed, with zero
ignored or filtered tests. Evidence: `refinement-before.json`, its log and
manifest. The follow-up plan is `parser-refinement-plan.json`, linked to the
original corrective plan with its source and reason.

The final source now protects the same Unicode whitespace set that Markdown
parsing trims, rejects invalid CSV quote transitions and requires an unescaped
closing Markdown border. Valid escaped CSV quotes, multiline fields and even
backslash parity are covered alongside the error cases. DOCX numeric character
references retain their XML-valid character restriction; Markdown conversion
can preserve its separate whitespace character set.

The complete correction-v1 workspace snapshot passed format, actionlint,
all-target check, strict Clippy and all 339 tests, with exact inventory matching
and zero failed/ignored/filtered tests. It comprises 76 core and 263 desktop
tests. The Release stage was intentionally stopped with SIGTERM after the
independent review findings; no Release PASS is claimed. Its unchanged source,
logs, `validation.json` and `owned-stage-stop.json` remain in `full-workspace-v1/`.

A fresh 84-file snapshot in `full-workspace-v2/` includes the final parser fixes,
13 regressions and current UI source. It preserves the independently reviewed
execution module and reuses only the owned, terminal internal Cargo cache.
Source inventory and hashes matched during capture. The complete seven-stage
validation and final independent source recheck are running. All subsequent
claims must refer to v2's own receipt and manifest, not the v1 test count.


V2 formatting, actionlint, all-target workspace check and strict workspace Clippy
have passed. The full inventory lists 342 tests: 79 core and 263 desktop. All
342 ran and passed, with zero failed, ignored, measured or filtered tests. This
includes the 13 data-integrity regressions, two search traversal-error cases and
all ten execution tests. V2 source hashes did not change during these stages.
The all-binary Release stage is still running; no unfinished build is a PASS.

Peirce completed the final v2 source recheck with no actionable findings. All
nine original findings and the three correction-review boundaries are closed
at source-review level. The verified hashes match v2's manifest. Receipt:
`independent-review-final.json`. The owner inspected the final scoped diff and
recorded a clean guard pass, matching live scoped hashes and passing diff
whitespace checks in `guard-review-final.json`.

The settings owner has started a subsequent error-banner copy-action follow-up.
Its later edits and native interactions remain outside v2's frozen receipt.
No lifecycle app was launched while that owner was active. A new isolated
before-fix fixture is prepared under `native-lifecycle-resume/`, preserving the
old manifests. Its binary hash matches the historical control, state/workspace
are private, real credential variables are removed, the endpoint is loopback,
and the private bundle has no URL-scheme registration. The after-fix fixture
requires v2's completed Release artifact. Fixture preparation is not native
acceptance.


## Final CLI record-count follow-up and v3 validation — 2026-11-15

Parent: parser refinement. Source: the continuing implement-all production goal.
Reason: preserving valid empty CSV records exposed an extra newline added by
Office CLI `println!` after `TableData::to_csv`. This is a low-risk single print
statement correction plus a meaningful exact-output regression. Plan:
`cli-record-count-plan.json` in the corrective evidence directory.

The original packaged-CLI probe compared only the first three parsed records,
so its CSV roundtrip assertion was insufficient. Its original receipt remains
unchanged, with the coverage gap recorded separately in
`release-package-check/coverage-gap.json`. This is not accepted as proof of
complete CSV record-count preservation.

The new regression ran against v2 production source: thirteen existing tests
passed and the exact stdout test failed on the additional final newline. There
were zero ignored or filtered tests. Receipt and unchanged source inventory:
`cli-record-count-before.json` and `cli-record-count-before-manifest.json`.
The CLI now uses `print!` for its already newline-terminated CSV output.

V3 derives from the fully validated immutable v2 source with exactly two
changes: the CLI print correction and its new regression. It does not absorb
concurrent UI-owner edits. Peirce compared the full tree and verified all v3
manifest hashes, returning no actionable findings. The original nine findings,
three parser recheck boundaries and CLI output finding are closed at source
level. Receipt: `independent-review-v3.json`.

| Final v3 gate | Result |
| --- | --- |
| Workspace formatting and actionlint | PASS |
| Workspace all-target check and strict Clippy | PASS |
| Complete test inventory and full test run | PASS: 343, zero failed/ignored/filtered |
| Test composition | 80 core, 263 frozen desktop; all ten execution tests included |
| All workspace Release binaries | PASS: `volt`, `calc-cli`, `office-cli`, `sensenova-live-test` |
| Candidate source integrity throughout every stage | PASS: no changes |
| Final independent source review and owner guard | PASS |
| Final canonical macOS package and exact packaged CLI roundtrip | PASS |
| Later UI-owner source, native and external acceptance | PENDING |

The v3 package probe initially retained a literal input directory pointing at
v2 binaries. Its payload-hash assertion correctly rejected the older Office CLI.
The failed attempt is preserved in `release-package-check-v3/failure.json`.
The corrected probe derives its input directory from the v3 validation receipt.
It passed relative input paths, an output path containing spaces, all ten
expected payload entries, executable hashes and permissions, bundle version,
arm64 architecture and deployment-target metadata. AppleDouble metadata entries
are explicitly inventoried separately from payload entries.

The packaged Office CLI preserves all CSV records exactly, including Unicode
boundary whitespace, multiline/escaped content and the empty final record. The
packaged four-slide request returns four slides; invalid count input fails
without stdout. Packaged `volt --help` returns before GUI initialization and the
calculator statistics smoke check passes. None of these checks launches a model
or registers the private app with LaunchServices. This is local package/CLI
proof, not GUI installation, signing/notarization, Windows or release proof.

Final evidence directory:
`/Users/zhd/.codex/tmp/codex-native-tools-correction-20261115-o6b_29hq/`.
Key receipts are `full-workspace-v3/validation.json`, its `source-manifest.json`,
`independent-review-v3.json`, `guard-review-v3.json`, `final-integrity-v3.json`
and `release-package-check-v3-retry/receipt.json`. The final archive SHA256 is
`5d18b45fdf7dcd5b5360a603d6658dc5e58e86ecb685d1286683d5742007260d`.
The final desktop artifact SHA256 is
`89925b5aeb9e3820bf1babc75f929c03e94308be95e883a92f68644cbc3158d0`.

Both new lifecycle fixtures are prepared and unlaunched. The after fixture's
binary hash also matches final v3, recorded in
`native-lifecycle-resume/v3-candidate-binding.json`. The current UI owner's
active follow-up prevents shared native interaction. Presence-only environment
checks find neither real provider-key variable and no `pwsh`; no secret values
were inspected or logged. Current full production acceptance remains **PARTIAL**.
All scoped local follow-up plans are complete; the broader production goal
remains active for the distinct pending native and external gates.


## Current UI integration and native handoff gate — 2026-11-15

Parent: final v3 validation and the continuing production goal. Reason: the UI
owner's completed run-notice copy-action, content spacing and multiline-summary
work changed the sole source input outside v3. A read-only complete capture can
be validated independently while native interaction remains owned by that task.
Risk: low. Orchestration: Native, one owner, external=0. No production source was
edited by this continuation.

V4 contains 84 inputs and 1,726,877 bytes. Its inventory and hashes matched
before and after capture; only `workbench/run_notice.rs` differs from v3. All
core sources and the independently reviewed recovery implementation are
unchanged. The private terminal Cargo cache was reused serially, with Rust
1.97.1, locked offline dependencies, one job and the normal profiles.

All seven v4 validation gates passed. The complete inventory and actual test
run match exactly: 347 passed, zero failed/ignored/measured/filtered. This covers
80 core tests and all 267 desktop tests in the captured source, including all
ten execution tests. Every stage retained unchanged frozen inputs. All four
workspace Release binaries built and were copied with hashes.

Current canonical macOS packaging also passed using binaries derived from v4's
own receipt. It verified all ten payload entries, hashes, executable permission
bits, bundle version, arm64 architecture and deployment-target metadata, plus
packaged CLI execution. The CSV check compares every record exactly. The known
AppleDouble metadata entries are separately inventoried. This remains local
archive/CLI proof, separate from native installation or publication.

The new `native-lifecycle-current-v4/` fixtures preserve the older unlaunched
fixtures. Their initial private states remain unchanged, binary hashes match,
no fixture process is running, real credential variables are removed by the
launcher, and the endpoint is loopback. The current after binary matches v4's
Release artifact. No shared GUI interaction occurred while the UI owner was
active. Its prior turn completed and immediately advanced to another active UI
turn; a new turn does not release the shared resource while it remains active.

Evidence is under the existing corrective root:
`/Users/zhd/.codex/tmp/codex-native-tools-correction-20261115-o6b_29hq/`.
Receipts: `full-workspace-v4/validation.json`, `full-workspace-v4/capture.json`,
`full-workspace-v4/source-manifest.json`, `release-package-check-v4/receipt.json`,
`native-lifecycle-current-v4/candidate-binding.json` and
`continuation-v4-evidence.json`.

The v4 desktop artifact SHA256 is
`20b45233d9649bd386b22bd7ef8cfd815dcf883e773d3463140cfbc899f0de37`.
The current archive SHA256 is
`b8d40c94c678bce3696a91b2a8dd0d559a67fa8aa6729a6b81a1fe89f946e6a3`.
Windows, real provider credentials/HTTP2, screen-reader/full desktop and
authorized remote release remain separate pending gates. Overall acceptance
remains **PARTIAL**; current-source deterministic proof does not replace the
unperformed native replay.

## Final v4 integrity audit — 2026-11-15 18:07 Asia/Shanghai

Parent: current UI integration and native handoff gate. Source: the continuing
implement-all production-readiness goal. This read-only audit made no production
code changes. It verified the complete frozen manifest, artifact/archive hashes,
live input inventory, reviewed core correction scope and `git diff --check`.
The scoped guard pass is clean; existing deterministic gates were not repeated.
Receipt: `final-v4-integrity-audit.json` in the corrective evidence directory.

A bounded 45-second wait confirmed the same UI turn remains active and is
preparing isolated native checks. The shared desktop was not released, and the
production-readiness fixtures remain unlaunched. No new native PASS is claimed.
The goal remains active with **PARTIAL** acceptance. The next useful step is a
stable UI handoff, followed by private lifecycle replay and validation of the
completed combined source. External acceptance requirements remain unchanged.

## Shared-native blocked audit — 2026-11-15

The previous goal turn made progress by validating and packaging v4. This
continuation confirmed the same UI task is still active through a bounded wait;
it is preparing wide and narrow native previews. Across three consecutive goal
turns, the required lifecycle replay has had no safe shared-desktop handoff.
No fixture process is running, and neither real-provider test-key variable nor
PowerShell is available in this process. No new native acceptance is claimed.

The goal tool confirmed status **blocked**. All v4 frozen inputs remain
unchanged. The two later UI source files remain outside that receipt. Windows,
real-provider/HTTP2, live screen-reader, complete desktop and authorized remote
release gates are preserved. The existing `block 0.1.6` future-compatibility
diagnostic was inspected; it remains unresolved, with no ABI patch or warning
suppression introduced.

Evidence and the exact three-turn audit:
`shared-native-blocked-audit-20261115.json` in the corrective evidence directory.
Resume after native ownership is released: reconcile the stable combined source,
run the prepared isolated lifecycle fixtures and finish the separate external
acceptance gates. No production code was changed by this continuation.

## Current all-targets verification receipt — 2026-11-17

The current dirty checkout was validated with a fresh temporary Cargo target
directory and an explicit temporary directory for macOS compiler files:

`CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR=/tmp/codex-native-alltargets-20261117 TMPDIR=/tmp/codex-native-alltargets-build-temp cargo test --workspace --all-targets --offline --locked`

The command passed **408 tests** with zero failures, ignored, measured or
filtered tests: 86 core tests and 322 desktop tests. This includes the
workspace integration suites and all binaries/targets. The separate default
concurrency desktop run, strict workspace Clippy, Release builds, package
creation, strict ad hoc codesign, Rustfmt, `git diff --check`, `actionlint` and
`shellcheck` receipts remain valid for the same current source state.

The GUI boundary is unchanged: the fresh probe timed out and the preceding
probe reported a locked screen, so current-build dropdown screenshots, pointer
replay, IME, screen-reader and matched ZCode/native pixel acceptance remain
**PARTIAL**. No commit, push, release publication or deployment was performed.

## Current GUI correction and replay — 2026-11-17

The first current-source GUI replay found a concrete issue that deterministic
builder tests could not see: the custom selected-menu check was rendered at
`right: -20px`, outside the `overflow_hidden` content slot created by
gpui-component 0.7.3. The settings Select delegate now uses `right: 0px`,
which keeps the 16px check visible while preserving the native Select
interaction and selection path. The test-only trailing-gap constant is now
test-scoped/removed from the production build, so the fresh Release build has
no related `dead_code` warning.

Focused Select tests passed (**9**), strict desktop all-target Clippy passed,
and the fresh current-source Release candidate has SHA-256
`1c44821c35fcf66accadeb994f64b54140efca377a802e5f4cb7884f4e532588`.
The isolated native candidate used private state/workspace roots and no
provider credentials. At the native 1200x800 logical viewport it accepted a
20px font change, switched to Dark, showed `配置已更新` after Save, displayed
the selected Dark check in the open menu, and completed API Key show/hide
masking with a local fixture value. No model request was made.

The remaining acceptance gates are narrower and external: exact 480x640
resize, complete keyboard/IME/VoiceOver coverage, matched ZCode pixels,
Windows installation, real-provider/HTTP2 lifecycle, signing/notarization
and remote release. The goal remains active and overall acceptance remains
**PARTIAL**. No commit, push, release publication or deployment was performed.

## Reactivated goal and v6 format candidate — 2026-11-15

Parent: the shared-native blocked audit. Source: the user explicitly reactivated
the full production-readiness goal. Reason: subsequent completed UI increments
require a fresh complete workspace candidate. Risk is low; Native, one owner,
external=0. No live UI-owned source was edited.

V5 captured 85 inputs with unchanged before/after inventory and hashes. The core
and reviewed execution sources match the earlier independently reviewed inputs.
The exact live capture failed the formatting gate, so compilation, tests and
Release were not run under that failed receipt. The separate v6 candidate
retains all v5 inputs except rustfmt output in `workbench.rs`.

The pending patch has two unified hunks covering three formatter diagnostics.
An initial preparation helper incorrectly required three hunks and stopped;
that helper failure is preserved. The correction checks the actual patch
forward against v5 and in reverse against v6. No production test was changed or
weakened. The exact formatting correction passed the scoped guard review.

V6 passed all seven gates with 359 tests, zero failures/ignored/filtered tests
and unchanged frozen source throughout. All four Release binaries were copied
with hashes. The canonical macOS package and exact complete-record packaged CLI
roundtrip passed. Archive SHA256:
`85d5be664fb4c7b15c05a057255a7a27400437fa2f41ab6323b72c7f4262ea87`.
This is candidate/package proof; live integration and native acceptance remain
pending.

A bounded wait confirmed UI turn `01a1defe-9000-7020-bda1-0a8a79303ee5` is active
and working on keyboard reveal. Its later two-file source changes are outside
v6. No private lifecycle fixture was launched. This is resumed audit turn one,
with implementation/evidence progress; the goal remains **active**.

Evidence under the corrective root:
`full-workspace-v5/validation.json`, `full-workspace-v6/validation.json`,
`full-workspace-v6/source-manifest.json`,
`full-workspace-v6/pending-workbench-format.patch`,
`full-workspace-v6/preparation-helper-failure.json`,
`release-package-check-v6/receipt.json` and `continuation-v6-evidence.json`.

## Resumed three-turn blocked audit — 2026-11-15

The user-reactivated run starts its own audit. Turn one prepared and validated
the isolated v6 formatting candidate while the UI writer remained active.
Turn two performed a bounded verified wait. Turn three confirmed the same exact
UI turn remains active after another 45-second wait; its focus-scroll probe
correction is still under development. The shared writer/native scopes have
not been released.

The goal tool confirmed **blocked** after those three consecutive observations.
The pending formatting patch still passes an applicability check but has not
been written into the active owner's file. Later changes to `workbench.rs` and
`workbench/draft_prompts.rs` remain outside v6's 359-test receipt. No new test,
build or native PASS is claimed. Missing Windows, real-provider/HTTP2,
screen-reader, full desktop and authorized release evidence remain unchanged.

The full scope and exact continuation actions are saved in
`resumed-native-blocked-turn3-20261115.json` under the corrective evidence root.
Resume when the UI writer and shared native resource are released, then
reconcile the current source, integrate the necessary format correction,
validate the stable combined candidate and bind the private lifecycle fixtures
to that artifact before replay.

## Narrow Settings replay — 2026-11-17

The current source was rebuilt in a private Release target with an explicit
writable temporary directory after the first isolated build exposed an
environment-only missing-`TMPDIR` failure. The current `volt` SHA-256 is
`2a77c04f4ff3161f081ca93fc84717f05e989d65eceeb8483f69109d5427ab53`.

The candidate bundle
`/tmp/codex-native-gui-20261117-v4/Volt.app` ran with private HOME, state and
workspace roots and no provider credentials. macOS Accessibility set the
content viewport to `480x640`; Core Graphics reported the full frame as
`480x680` because the titlebar contributes 40px. Settings Appearance rendered
without clipping at the minimum content size. The Theme menu showed all
three localized rows and the selected check. Dark theme plus 20px interface
text rendered successfully, and Save showed `配置已更新`.

The local capture is
`/tmp/codex-native-gui-20261117-v4/settings-dark-20-narrow.png` with SHA-256
`374067f06bde0c18d3df765bd75c9a7efde019f061bca00ba4a6c8ae5ee93bcb`.
This closes the current-build narrow Settings receipt. Complete keyboard,
IME, VoiceOver, matched-reference pixels, Windows installation,
real-provider/HTTP2 lifecycle, signing/notarization and authorized remote
release remain **PARTIAL**; no commit, push or deployment was performed.

## Settings keyboard control replay — 2026-11-17

The same isolated v4 bundle was relaunched for a focused keyboard replay.
Accessibility exposed Return, the settings navigation sections, both theme
controls, interface font size, reduced motion, Save and the footer language
control. From the font field, `Tab` reached reduced motion and the next
`Tab` reached Save; `Return` on Save visibly showed `配置已更新`.

The Appearance Theme menu opened with all three rows and the selected Dark
check. `Escape` dismissed it without changing the value, while reopening and
pressing `Return` confirmed the selected Dark row and closed the menu. The
font field accepted `⌘A`, `20`, and `Return`; AX reported `Value: 20` and the
page visibly reflowed at 20px. The value was restored to 14px after the
probe. The footer language menu exposed Chinese and English choices.

This closes the tested local keyboard paths, but not the complete Settings
Tab loop: the next Tab after Save moved to the native Window sharing control,
so continuous traversal through the sidebar and footer was not established.
IME, VoiceOver, matched-reference pixels, Windows installation,
real-provider/HTTP2 lifecycle, signing/notarization and authorized remote
release remain **PARTIAL**. No commit, push or deployment was performed.

## Settings focus-order follow-up audit — 2026-11-17

The source tree places the Settings navigation column before the selected
section content. In the wide layout that column contains Return, the section
buttons and the footer controls; the Appearance content then ends with the
reduced-motion switch and Save. The observed Tab from Save to the native
Window sharing control is therefore consistent with the existing window-wide
focus order and does not prove that the earlier Settings controls were
skipped.

The `gpui-component` Select trigger still registers a real Tab stop;
`focus_ring(false)` only suppresses its visual ring. No reproducible source
defect justified a focus-order patch. A fresh v4 candidate created its
window, but the screen capture was black and CUA returned
`cgWindowNotFound`, so a replay starting at Return could not be completed.
The complete Settings traversal remains an unresolved runtime acceptance
boundary and overall review status remains **PARTIAL**. No code, commit,
push or deployment was performed in this audit.

## Current-candidate focus replay continuation — 2026-11-17

The current v4 executable was relaunched from the recorded candidate bundle
and matched the recorded SHA-256
`2a77c04f4ff3161f081ca93fc84717f05e989d65eceeb8483f69109d5427ab53`.
The process stayed alive, but the window was not exposed to Core Graphics,
System Events or CUA, so the required replay from Return through the Settings
navigation, footer controls, content controls and Save could not be completed.
An older accessible package was deliberately excluded because its executable
hash did not match the current v4 candidate.

The current source passed the focused Settings suite (**55 passed**),
`cargo fmt --all -- --check`, and `git diff --check`. No speculative focus
change was added. The complete current-candidate traversal, IME, VoiceOver,
matched reference pixels, Windows installation, real-provider/HTTP2
lifecycle, signing/notarization and authorized remote release remain
**PARTIAL**. No code, commit, push or deployment was performed.

## Current v4 Settings traversal receipt — 2026-11-17

The current v4 candidate was verified through the native macOS accessibility
API after the CUA adapter failed to attach to the same live window. The
candidate executable remained at
`/tmp/codex-native-gui-20261117-v4/Volt.app/Contents/MacOS/volt` with SHA-256
`2a77c04f4ff3161f081ca93fc84717f05e989d65eceeb8483f69109d5427ab53`.

The real focused-state sequence from Return was:

`Return → 常规 → 外观 → 模型服务 → AgentConfig → 快捷键 → 子代理 → 技能 → 用量 → 界面语言 → 主题 → 主题 → 界面字号 → 减少动态效果 → 保存配置 → Return`

The font field accepted a real key event and was restored from `141` to `14`;
the reduced-motion switch toggled and was restored; Save produced visible
`配置已更新`; and the final Tab returned to Return. The current Theme popup
also rendered its selected-row check. The probe receipt is
`/tmp/codex-native-gui-20261117-v4/tab-probe-20261117.txt`, and the current
visual receipts are `theme-open-current.png` and `tab-probe-save.png` in the
same directory.

The previously unresolved current-candidate Settings traversal boundary is
closed. Native Chinese IME, VoiceOver announcements, matched reference
pixels, Windows installation, real-provider/HTTP2 lifecycle,
signing/notarization and authorized remote release remain **PARTIAL**. No
code, commit, push or deployment was performed.
