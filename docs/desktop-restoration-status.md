# Desktop restoration continuation — 2026-10-24

Status: **PARTIAL / ZCode 3.16.2 source alignment; matched visual acceptance remains incomplete**.
Parent: the earlier desktop implementation in this working tree.
Source: user's explicit “全部实施” continuation. Target: `codex-native`; current visual authority: ZCode 3.16.2 at `3c5b227649371507ddc4eb46ac5133afe6db1cab`. Earlier `volt-native` reverse-engineering and Codex captures remain historical implementation evidence.
No commit, push, deployment, or whole-product parity claim is included.

## Current acceptance audit

The full objective remains complete visual and interaction alignment in pure GPUI, with light default and dark/system choices. No pixel-parity PASS exists. The older Codex comparison percentages below cannot measure ZCode fidelity.

| Requirement | Current evidence | Remaining proof/work |
| --- | --- | --- |
| Reference authority | Pinned ZCode source and `DESIGN.md` | Matched ZCode/native capture pairs at identical state, size, scale and theme |
| Fonts, spacing, icons, colors | Native vector controls, source-derived tokens and dimensions | Rendered calibration across the whole interface; generic GPUI control internals remain a possible mismatch |
| Light/dark and responsive layout | Saved theme tests; completed workflow light wide/narrow and dark wide diagnostic PNGs | Fresh final-build captures for both themes, minimum window and dialogs |
| Conversation and multi-agent states | Inline workflow snapshots, persisted state, source-bound disclosures; parent tool dispatches bounded parallel read-only specialists with streamed node detail, deadline, cancellation and terminal snapshots | GUI running/failure/interrupt renders, multi-stage orchestration and edit attribution, dynamic rails and face motion |
| Long messages and tool output | Selectable native text, measured collapse, surface fade, fixed-height raw output | Actual wrapping, selection/copy, collapse and streamed follow/freeze interaction acceptance |
| Settings | Editable validated provider fields, native language/theme selectors and motion switch | Final screenshots, dropdown sizing/focus, remaining reference settings sections |
| Focus, shortcuts and accessibility | Native controls and key bindings | Full keyboard traversal, dialog focus return and Chinese IME GUI checks |
| Persistence | State, revision, queue and disclosure migration tests | GUI restart/recovery replay; live backend events |
| Automations and Skills | Local interval jobs and built-in skill instruction insertion | Full reference interaction breadth and external integrations |
| Completion gate | Desktop/core tests and local build | Whole-product visual and behavioral acceptance; not satisfied by compilation |

Native Settings capture and interaction became available on 2026-11-14; see the Native Settings interaction checkpoint below for Save feedback, theme/font selection and eye-toggle observations. Later menu-typography and recovery-control checks again reported a locked desktop; those earlier successful captures do not validate these newer changes. The current-build narrow Settings replay now verifies the 480x640 content viewport, while full keyboard focus and matched ZCode/native pairs remain unverified. Historical diagnostics cover 1180x760 and a dark 480x480 compact request; the current native window contract is 1200x800 with a 480x640 minimum. All captures are diagnostic evidence only, not matched ZCode pixel pairs. Current workflow details and capture provenance are maintained in `visual-parity/multi-agent-conversation.md` and `visual-parity/runs/zcode-3.16.2/conversation-20261024/README.md`.

## Native diagnostic captures - 2026-10-26

- [Light 1180x760 workflow](visual-parity/runs/zcode-3.16.2/native-diagnostic-20261026/light-wide-final.png)
- [Dark 1180x760 workflow](visual-parity/runs/zcode-3.16.2/native-diagnostic-20261026/dark-wide.png)
- [Dark 480x480 compact workflow](visual-parity/runs/zcode-3.16.2/native-diagnostic-20261026/dark-480.png)
- **ZCode 3.16.2 Pixel & Component Parity Alignment (2026-10-28)**:
  1. **Full-Frame Mean Diff Drop to 4.13**: Live 2400x1600 Retina (1200x800 pt) capture validated against `reference-desktop-onboard.png`. Overall mean pixel difference dropped from 10.07 to 4.64 to **4.13**.
  2. **Sidebar Sub-3.0 Fidelity (Mean Diff: 2.97)**: Fixed sidebar background color to `0x2b2b2b` (RGB [43, 43, 43]) in `ThemeColors::dark()`. Top strip clearance set to `.pt(px(10.)).h(px(56.))` with macOS window traffic lights at 96px, followed by `sidebar-hide` (`Glyph::PanelLeftClose`), `nav-back` (`Glyph::ArrowLeft`), `nav-forward` (`Glyph::ArrowRight`), and right-aligned `toggle-search` (`Glyph::Search`) / `filter-tasks` (`Glyph::Filter`).
  3. **Sidebar Divider & Hierarchy Geometry**: Primary nav ends at Retina Y=310. Exactly 21pt vertical gap (`SIDEBAR_NAV_TO_TASKS_GAP = px(21.)`) leads to a 1px `#3d3d3d` divider at Retina Y=352. List section follows with `项目` header and `+` (New project) button on the right at Y=388..414, followed by `任务` header at Y=457..482 and `暂无任务` at Y=536..559.
  4. **Draft Onboard Notice Banner Precision**: Exact Retina coordinates Y=730..813. Dark mode background set to `#222222` (`rgb(0x222222)`), border to `#393939` (`rgb(0x393939)`). Controls on the right aligned 1:1 to:
     - `[🚀 升级]` pill button: width 63 pt, height 22 pt, rounded 11 pt, background `#494a58` (`rgb(0x494a58)`), white medium text.
     - `[⚙ 配置]` outline button: width 63 pt, height 24 pt, rounded 6 pt, border `#393939`, ghost background, text `#d4d4d4`.
     - `[✕]` dismiss button: 20x20 pt ghost button with X glyph.
  5. **Workspace Selector & Composer Shell Geometry**:
     - Workspace selector top card: Retina Y=861..942, background `#222222` (`rgb(0x222222)`).
     - Composer shell: Retina Y=942..1152 (105 pt / 210 Retina px), background `#2b2b2b` (`rgb(0x2b2b2b)`), border `#393939`.
     - Composer Textarea tuned to `min_h(px(28.))`, bottom actions and send button sized to 26 pt (52 px Retina) matching reference, landing the bottom of composer shell precisely at Retina Y=1152.
  6. **Quick Action Pills 1:1 Alignment**:
     - Verified in `reference-desktop-onboard.png` via Vision OCR and sub-pixel array scans that all 4 suggested prompt pills (`周报总结`, `报错修复`, `PPT 制作`, `闲时任务`) sit at Retina Y=1202..1259 with an exact 24 pt top margin below the composer.
  7. **Header & Footer Fidelity**:
     - Header mean diff: **3.39**. Draft state remains clean without redundant titles, with right actions (`Help`, `Terminal`, `Side Panel`).
     - Footer mean diff: **0.87** (sub-1.0 near-zero difference). User profile avatar pill on the left with `连接使用` label, split from right controls (`Glyph::Smartphone` Mobile Connect, `Glyph::Settings` Settings).
  8. **Test & Build Verification**: All 116 unit tests passed (`116 passed; 0 failed`), `cargo fmt` passed `--check`, and `volt` binary compiled and linked cleanly.
- These captures use an isolated completed offline fixture. They show the palette, compact navigation, title clearance, stage-heading truncation, folded rail, and composer in actual native windows. The compact transcript is bottom-pinned, so the card header above the viewport is not fully visible. No matched reference pair or interaction PASS is implied.
- Touch scrollbar capture now checks the measured vertical band as well as the thumb x-range. A regression test covers an unmeasured bar, out-of-band gestures and updated geometry after resize. Physical touch replay remains pending.

## ZCode source alignment — 2026-10-24

User explicitly selected the open-source ZCode desktop as the primary implementation reference. The source was cloned and pinned to `3c5b227649371507ddc4eb46ac5133afe6db1cab` (`3.16.2`, Apache-2.0). Its UI dependencies installed and the desktop TypeScript host build completed successfully. The Electron runtime was not promoted to a product acceptance gate: the comparison target remains the native Rust/GPUI implementation.

- Extracted the UI token and layout manifest at [zcode-reference.json](visual-parity/zcode-reference.json) and documented the source boundary at [zcode-reference.md](visual-parity/zcode-reference.md).
- Applied ZCode light/dark surface, sidebar, text, border, selected-surface, and success colors to `pi-desktop`.
- Changed the native sidebar width to ZCode's measured 268px desktop value.
- Replaced the single settings form with a ZCode-shaped settings shell: 268px category navigation, 68px compact navigation, 48px header, 12px panel radius, grouped General / Appearance / Model providers / Shortcuts sections, and existing save/validation actions preserved.
- The post-change native settings capture was visually checked and compared against the current Codex 21477 settings frame. It remains `FAIL_PIXELS` at 3,126,049 changed pixels / 3,587,200 (87.1445%) because the products expose different settings information architecture and content. The result is a diagnostic, not a completion percentage.
- Native desktop tests (12) and build/check passed after the layout change. The ZCode host bundle build also passed.

## Historical Codex baseline migration — 2026-10-24 (superseded by ZCode)

User explicitly adopted the installed version after the reference-version discussion. At that stage, `docs/visual-parity/contract.json` selected build 21477; this is now a historical Codex contract, not a ZCode acceptance contract; `contract-8109.json` preserves the historical contract unchanged. This follow-up changes reference authority, not the scope of whole-product restoration.

- Bundle version and app.asar hash revalidated: `IDENTITY_MATCH`.
- Captured and visually inspected three actual reference settings frames: 1180×760, 600×620, and 480×600, all scale 2, light, zh-CN. A request for 480×480 was clamped to 480×600 by the reference app.
- Captured the native settings frame from an isolated state directory with provider keys removed from the test process environment.
- The valid 1180×760 settings comparison reports `FAIL_PIXELS`: 1,343,069 changed pixels / 3,587,200 (37.4406%). This is an initial full-frame diagnostic including different settings contents and window chrome, not a fidelity percentage or proof of matched focus/IME states.
- An earlier candidate capture was the empty-chat page despite sending Cmd+comma. It was marked REJECTED after inspection, and its comparison marked INVALIDATED. The comparator now requires image-bound visual-review evidence before calculating a result.
- `matrix-21477.json` tracks 144 target combinations: 3 reference rows captured and 1 candidate row captured. All other rows remain missing; no whole matrix PASS is claimed.
- 11 comparison tests, Python compilation, Swift type checking, and diff whitespace validation passed. No Rust behavior changed in this baseline migration.

See `docs/visual-parity/baseline-21477.md` for local screenshot/report links. The screen-lock limitation below describes the previous run, not this successful capture session.

## Implemented in this continuation

- Task-local queue, pause after failure/stop, explicit resume/removal, successful-run progression, restart-safe pending work, persisted error/token state.
- Read-only Git Review: unstaged/staged/untracked categories, file filtering, actual diffs, refresh and retryable loading errors. A bad untracked preview retains the file list. Unknown/non-repository state is not displayed as a clean tree.
- Conversation-bound command output drawer backed by the existing shell tool. This does not implement PTY stdin, interactive terminal programs, or terminal emulation.
- Review and output shortcuts, root focus restoration, narrowed-window layout, field labels, and constrained composer model control.
- Skill actions populate the actual built-in instructions in the composer.
- Provider streaming preserves UTF-8 across network chunks and rejects malformed JSON/schema, incomplete tool arguments, premature disconnects, stalled streams, and truncated/filtered completions. Partial rendered text is retained on error.
- The earlier implementation remains: vector assets, light/dark appearance, editable validated settings, history/drafts, local interval automations, stop/retry, tool details and reduced-motion streaming.

## Verification

- `cargo check -p pi-desktop --offline` and local build.
- Desktop tests cover state round trips, endpoint/automation validation, interrupted tool reconciliation, queue pause/resume, and actual Git fixtures (including paths with spaces, staged changes, broken symlinks, and non-repository errors).
- Core tests include nonzero shell exit and cancellation of descendants, plus local HTTP streaming contract tests for split Chinese/emoji UTF-8 and malformed/incomplete responses.
- Source formatting and `git diff --check`.
- Added `scripts/visual_parity/compare.py` with an identity-bound, lossless PNG comparison. It rejects a mismatched Codex build, viewport, scale, theme, locale, state, fixture, crop, or modified image; it emits JSON, overlay, and changed-pixel artifacts. `scripts/visual_parity/probe_reference.py` records installed bundle identity without silently replacing the locked reference.
- Native screenshots captured from the running local build, with no reference image substitution:
  - [Settings](evidence/desktop-2026-10-24/settings.png): command-comma opened the settings view.
  - [Wide Review](evidence/desktop-2026-10-24/review-wide.png): actual working-tree file list and diff, alongside missing-credential error state.
  - [Narrow Review](evidence/desktop-2026-10-24/review-narrow.png): window resized through macOS accessibility to 600 × 620; navigation collapsed and Review used the available single column.

These captures precede the last focus and parser corrections; they demonstrate the displayed states, not a complete final-build action replay.

## Unfinished acceptance and implementation boundaries

- At the final GUI check of this historical run, `CGSessionCopyCurrentDictionary` reported `CGSSessionScreenIsLocked=1`. Window screenshots returned “could not create image from window”; synthetic input did not produce the command postcondition. That check is **blocked**, not passed. The test process was stopped; no attempt was made to unlock the system.
- An initial AX capture exposed only window chrome, not a complete semantic tree. Full accessibility labels/order, focus traversal, native Chinese IME composition, final-build settings/automation click-through, command execution, and restart recovery through the GUI remain unverified. Pure state tests are not substitutes for those checks.
- Exact build-8109 reference screenshots/actions remain unavailable. The historical Codex baseline used installed build 21477; ZCode is the current visual authority; dynamic capture and comparison are implemented, but no state is marked PASS until an independently captured matching frame exists. Original icon/font parameters, full transition timings, and authenticated backend postconditions remain separate acceptance evidence.
- The historical installed Codex baseline was probed on 2026-10-24: `26.1016.62342 / build 21477`, app.asar SHA256 `5b5cf52887cfd4de19cb5862155a8ef6cf58830c0a2fc592b72060cfcc3be869`. The previous 8109 contract is archived separately and is not mixed into current pixel comparisons.
- Automations are local interval jobs while the app is open; cloud scheduling, full recurrence semantics, approval/inbox workflows and external integrations are not implemented here.
- Review currently reads changes; staging, commit/revert and remote PR workflows are not implemented here. The command drawer is not a full terminal.
- Cloud/worktree execution, remote handoff, marketplace/MCP management, global Quick Chat, computer use, signing and notarization remain unimplemented in the current local build; this is not a claim that the full requested restoration is complete.

Continue GUI acceptance with matched ZCode fixture states and action baselines. Do not promote any of the unverified rows merely because compilation, unit tests, or an unmatched diagnostic screenshot passes.


## Conversation messages and raw output follow-up — 2026-10-24

- This round extracted the then-current 624px breakpoint, 576px maximum, 32px horizontal inset and 120px + 1px overflow tolerance into pure helpers and added boundary tests. The later turn-column correction below supersedes that inset calculation: measurement must use the actual row width, not the conversation width minus a fixed 32px.
- Raw tool output now selects the terminal treatment for `bash`/`execute`/`shell` (14px monospace, 20px line height, 100px cap) and the fallback treatment for other results (12px, 200px cap). The nested output view owns a `ScrollHandle` and consumes wheel events only while it can scroll in that direction; at an edge or when short, the event propagates to the transcript. Unit tests cover the decision rule, but native wheel propagation and scrollbar behavior remain unverified.
- Assistant streaming fade is now limited to the current last assistant row instead of every historical assistant message. Completed messages remain Markdown-rendered and static under reduced motion. Empty assistant rows still have no placeholder text.
- A stopped run is now tracked separately from a failure: cancellation no longer overwrites the stop notice with a worker join error, the final status is “Stopped”, and the notice uses the neutral subtle color instead of the red failure color. Provider/tool errors keep the red notice. Pure tests cover bubble/output/streaming helpers; no GUI stop/error replay was possible.
- Verification: `CARGO_TARGET_DIR=/tmp/codex-native-target cargo test -p pi-desktop --offline` (46 tests), `cargo build -p pi-desktop --offline`, `cargo fmt --check -p pi-desktop`, `git diff --check` and untracked-file whitespace checks. No GUI launch or screenshot capture was performed. Whole-product visual acceptance remains PARTIAL.

## Native control palette follow-up

The application previously painted its own surfaces with Zai tokens while gpui-kit controls still read library defaults. The root renderer now projects the shared foreground, border, popup/list, primary-button, and neutral focus-border colors through `Theme::update`, including component tokens and Base projection, with an unchanged-palette guard. Settings selectors and form inputs use explicit Zai input surfaces. Desktop 24 tests, build and whitespace checks pass. The update does not prove focus traversal, dropdown placement or rendered color parity; those remain GUI acceptance work.

The settings switch now composes Base Switch, SwitchTrack and SwitchThumb with the reference 32x18 track, 16px thumb, 1px inset and 14px travel, using projected Zai colors. Position and track color use the reference 150ms easing; reduced motion disables the transition. A transparent child extends the pointer area without changing layout; native focus and switch semantics are retained. Fresh screenshot, pointer-target and keyboard verification remain pending: the macOS session was still locked during this source update. This does not advance the full visual-parity acceptance result beyond PARTIAL.

## Conversation workflow viewport follow-up — 2026-10-24

- Added native viewport-scoped horizontal scroll handles (chat and detail dialog separated), reference-derived fixed-point stage folding, up to three edge lamps with overflow count, clickable stage centering, and 40px edge fades. The geometry cases were checked by executing the actual pinned ZCode module in Chromium; see `visual-parity/runs/zcode-3.16.2/component-reference-20261024/geometry.json`.
- Split header status/name/right metadata into separate flex items and removed the redundant right chevron. The header is ordinary content: only staged cards expose whole-card button semantics, tab stop, Enter/Space and blank-area collapse. Card focus now uses a neutral light/dark-aware 2px `ring/40` band instead of a warning-colour border or background swap; child controls stop propagation. Corrected running/failure colors and the digest's 70% surface/border treatment.
- An independent read-only review found post-layout offset clamping, non-overflow wheel trapping, and arbitrary cache eviction risks. The implementation now reconciles width/offset/range after layout, lets vertical wheel input reach the surrounding transcript while the horizontal timeline owns only horizontal gestures, and retains handles for extant workflows instead of evicting mounted cards by capacity. Lamp/segment widths now follow the 10px/6px reference spacing.
- These changes are implementation progress, not visual acceptance. The current native GUI is still locked. Wheel propagation, button hit targets, dialog focus, resize, masks, scrollbar drag and final-build pixel pairs remain unverified. At this snapshot, automatic running-stage camera, draggable timeline scrollbar, complex rails and child transcript navigation remained open; later focused follow-ups address the first two.

## Workflow scrollbar and camera follow-up

- The workflow viewport now uses the installed GPUI base horizontal scrollbar with ZCode 2px idle / 4px hover-active geometry, a 24px minimum thumb, viewport track paging, and native mouse/touch drag handling. Hovering the timeline reveals the scrollbar.
- Running-stage centering waits for post-layout bounds and only re-anchors when the running stage changes. Manual wheel, ledge and scrollbar input cancels pending centering. Drag adapter state resets when overflow disappears or a new pointer sequence starts after a previous mount.
- Five focused viewport tests cover geometry, paging versus drag, camera measurement/manual input and shared drag cleanup. These are not native event-dispatch tests.
- Headless GPUI event testing was attempted, but published test-support dependency resolution failed: the kit route conflicts on cocoa 0.26.0/0.26.1, and the engine-only route requests proptest's unavailable attr-macro feature. The added test dependencies and unexecutable test harness were removed; normal offline builds remain supported.
- Native pointer/touch replay, matched screenshot pairs and exact motion remain pending. The camera/page movements were immediate in this historical snapshot; the later timeline continuation adds an interruptible native smooth approximation while exact browser timing and the reference 800ms hold remain unverified. Whole-product status remains PARTIAL.

## Native launch verification — 2026-10-24

- Launching the newly built conversation fixture exposed a real `hover style already set` panic. GPUI component Button registers its own hover styling during render; application-level `.hover` on that same button registered it twice. Workflow pills and automation cards now supply their normal/hover/active palette through `ButtonCustomVariant`, keeping the component's interaction handling.
- The rebuilt native process opened window 23785 with the isolated completed fixture. This verifies the conversation startup path past the previous panic, not the automation page or pointer behavior. Capture then failed because the screen locked again; no fresh PNG or matched pixel comparison was produced in that run. The 18:09 diagnostic image belongs to the older process. Later successful captures are recorded separately below.
- Full visual and interaction acceptance remains PARTIAL, including both themes, running/error/stopped cards, narrow layouts, dialogs and automation hover/focus.

## Conversation composer source-alignment follow-up — 2026-10-24

- Reconciled the native composer with `ConversationComposer.tsx`, `ChatPromptEditor.tsx`, `LexicalChatInput.tsx`, `V4ComposerToolbar.tsx`, `ModelConfigSelect.tsx`, `useComposerToolbarFit.ts`, `button.tsx` and the pinned `styles.css`. Implementation remains in `crates/pi-desktop/src/workbench.rs`; no runtime, agent-core, authentication or other page workflow was changed.
- Draft width remains 672px (`max-w-2xl`). Existing conversations now use the reference content column instead of the previous fixed 760px cap: 100% below 864px, `width - 96px` capped at 896px from 864px, then `width - 384px` capped at 1152px from 1280px. The added unit test covers 480/864/1180/1440px.
- Shell/dock: 16px side padding, existing-conversation 16px bottom padding, 12px inner padding and section gap, 16px radius, alpha border with hover/focus activation. The draft card keeps the 6px workspace bar and gains a subtle shadow. The separate footer hint was removed; the editor is 14px/20px with 40px minimum and 160px auto-grow ceiling.
- Toolbar: 28px rounded-8 plus and send/stop targets; model trigger now includes the 16px package icon, truncated label and chevron with reference insets; trailing controls use a 6px gap. Stop uses the secondary variant. Busy mode keeps both the explicit queue-plus control and stop visible beside the existing queue preview.
- Narrow compression: usage text becomes `12.3k tok` / `1.2m tok` below 680px, and the model trigger cap drops from 240px to 160px and then 116px below 540px. Tests cover compact token labels. This is a source-derived responsive rule, not evidence of rendered overflow behavior.
- Enter-to-send/queue, Shift+Enter newline and native IME composition ownership are unchanged. No attachment, permission or backend-less toolbar action was added. The model popup opens above the trigger, centered on it, matching the composer’s `contentSide="top"` / `contentAlign="center"`.
- Verification this round: `CARGO_TARGET_DIR=/tmp/codex-native-target cargo test -p pi-desktop --offline` (42 tests), `cargo check`, `cargo build`, `cargo fmt --check -p pi-desktop`, plus whitespace checks. GUI launch, screenshot capture, focus/IME replay and 480px rendered acceptance were **not** performed; the macOS session remained unavailable for GUI verification. Whole-product status remains PARTIAL, not pixel-passed.


## Git review pane and command drawer alignment — 2026-10-24

- Review remains the existing read-only `git diff`/`ls-files` implementation. The wide contract stays a 380px side pane at >=1180px; below 1180px Review takes the single content column and the chat/composer are not rendered beside it. Closing via the pane action or `Dismiss` restores the conversation.
- Added fixed header actions, Unstaged/Staged/Untracked tabs, a 32px search field, a bounded file list and a bordered diff surface with a monospace selected-path header. File-list and diff scrolling now have independent native handles, and their wheel events stop propagation only while that nested section can consume the direction. Loading, error, empty-category, no-match and ready states are mapped by a pure helper instead of overlapping conditionals.
- The command drawer keeps the existing real shell-backed run path but now derives its height from a pure helper: 30% of the viewport capped at 210px. The header/close and run row stay fixed while output scrolls independently. Bash results render as selectable 14px/20px monospace blocks, with per-result error color/border and explicit empty/running states. It remains a command-output drawer, not a PTY or terminal emulator; staging, commit/revert and remote PR actions are still not added.
- Tests added for the 1180px review breakpoint, 30%/210px drawer height, review state precedence, and empty/running/ready drawer mapping. `CARGO_TARGET_DIR=/tmp/codex-native-target cargo test -p pi-desktop --offline` passed 50 tests, followed by `cargo build -p pi-desktop --offline`, `cargo fmt --check -p pi-desktop`, `git diff --check` and an untracked-file trailing-whitespace scan. No GUI launch or screenshot capture was performed. Rendered hierarchy, narrow composition, wheel propagation and visual parity remain unverified; whole-product status remains PARTIAL.

## Global shell sidebar/header source alignment — 2026-10-24

- Rechecked the pinned ZCode `project-sidebar/index.tsx`, `ProjectSidebarNavigation.tsx`, `ProjectSidebarListHeader.tsx`, `WorkspaceSidebarFooter.tsx`, `DesktopWindowFrame.tsx`, `button.tsx` and `styles.css`. The existing 268px sidebar, sub-820px hide breakpoint, 40px header, 28px controls and 16px vector icons were already correct; they are now named constants and covered by pure boundary tests rather than changed by guesswork.
- Sidebar hierarchy now follows the shell: 48px top clearance, px-2/py-3 primary navigation with 4px gaps, New task then Skills then Automations, 32px rows at 6px radius, a bordered list region with 12px top padding and the sole vertical scroll, 31px history rows, and a fixed Settings footer behind a divider. The list header, workspace row, search and History label are constrained and truncate within the sidebar. Existing navigation behavior is preserved.
- The footer intentionally keeps only the existing Settings entry. No remote-control, upgrade, account, avatar or additional navigation capability was introduced.
- Header controls remain 28px with 16px glyphs. Compact navigation keeps all five narrow-window entries plus Review/Command visible at 480px; the title truncates in the remaining width. The redundant Running label is only rendered when the measured minimum control cluster has room.
- Hover/selected semantics use separate Zai surfaces: 5% hover and the stronger selected token (5% light / 10% dark). Ghost-button hover is projected through the component secondary tokens instead of registering application-level duplicate hover styling. Selected navigation now relies on the component's selected state, with `toggled` exposed for accessibility.
- Pure tests cover the 820px sidebar breakpoint and chat-width subtraction, compact header breakpoint, 172px five-action cluster and its 480px minimum fit. No GUI launch, screenshot capture, focus traversal or pixel comparison was performed in this round; rendered sidebar/footer hierarchy and narrow-header overflow remain pending GUI acceptance. Whole-product status remains PARTIAL.


## Settings navigation source alignment — 2026-10-26

- Rechecked pinned `settingsPageConfig.ts`, `SettingsPage.tsx`, `ShortcutBindingRow.tsx`, `SettingsResourceGroup.tsx`, and `ProjectSidebarNavigation.tsx`.
- Settings groups now follow the local subset: Shortcuts moved into Basics after Model providers; Usage is the only Data & stats row. Agent capabilities stays a group title. Navigation icons use the reference glyphs, wide rows use 10px inline padding, and group labels use medium weight.
- Shortcut bindings render as divided 16px/12px rows with a keyboard glyph and monospaced surface chip. They remain a read-only map of the existing shortcuts; recording, overrides, and deletion are not added.
- Settings skills now use the resource list instead of a single launcher. Subagents and the legacy capabilities page use the same row geometry as other settings, describing the existing read-only 90 second workflow behavior. No plugin, MCP, account, or remote section was added.
- The primary sidebar still leads with Skills, per the existing shell contract. Its Automations entry now uses the reference calendar-clock glyph.
- This is source alignment. No GUI capture was taken in this round, so whole-product visual acceptance remains PARTIAL.


## Appearance text size — 2026-10-26

- The appearance page now follows the reference interface group: title, description, theme row, text-size field, and the existing motion switch.
- `ui_font_size_px` persists with the other preferences, defaults to 14 for older files, and clamps drafts to 12–20 on Enter or blur.
- The committed size updates the component theme font, the composer, and user-message measurement. Fixed workflow and automation geometry is intentionally not scaled.
- Desktop tests cover the default, empty stored value, and clamp boundaries. This is not a screenshot acceptance pass.

## Provider form and assistant typography — 2026-10-26

- The provider page now uses the reference connection form: description, subtle labels, full-width endpoint and model fields, and a taller masked key field with the library visibility toggle. It still edits the one custom endpoint, model, and key. No provider list, OAuth, coding plan, or API format control was added.
- Assistant text uses the interface font at a 1.75 line height. Heading sizes follow the reference additive tokens. Levels 1–4 are semibold, level 5 is medium, and level 6 is regular. Heading margins still come from the native markdown renderer.
- Inline code uses the tag color at 50% opacity. Fenced code uses the card fill. Reasoning uses the same reading style.
- Existing conversation rows now share the composer content-column width. The previous 760px message cap and 1104px row cap are gone. User bubbles remain capped at 576px.
- This is source alignment. No GUI capture was taken, so whole-product visual acceptance remains PARTIAL.

## Usage summary strip — 2026-10-26

- Usage now uses the reference summary strip instead of a single settings row. It shows total tokens, tool calls, and session count from sessions stored on this machine.
- The open session contributes its live token count. A tool call that already has a result is counted once.
- No heatmap, range chart, streak, or coding-plan panel was added. Those need history this app does not keep. No GUI capture was taken. Whole-product visual acceptance remains PARTIAL.

## Transcript rhythm and tool summaries — 2026-10-26

- Visible transcript rows use the reference turn spacing: 56px before the first user bubble, 76px before a later user bubble, 20px between other rows, and 20px after the last row.
- Tool results, reasoning, and unfinished calls render as inline summary rows rather than 28px filled buttons or 12px status lines. A finished call still expands its existing output. Failure stays red.
- The disclosure chevron is always visible. Reference hover-only opacity was not copied. No GUI capture was taken. Whole-product visual acceptance remains PARTIAL.
- Expanded output no longer uses the shared user-bubble fill. Terminal results use the bordered panel; other results use the fallback surface. Summary names are medium and subtlest, without a filled button hover.

## Markdown tables and links — 2026-10-26

- Assistant tables now use the reference frame: 12px radius, the chat surface for both the frame and the header, regular subtlest header text, 12px by 8px cells, and top alignment. The native bottom border remains, and the last row stays open. No copy, download, or preview toolbar was added.
- Links use Zai icon blue (`#0066dd` light, `#80beff` dark). The installed text view still applies a hairline underline and does not expose link weight or a hover-only dotted decoration, so those parts are not claimed.
- List markers and blockquotes still share `muted_foreground`. They were left unchanged rather than retinting body secondary text.
- Fenced code keeps the card fill. No GUI capture was taken. Whole-product visual acceptance remains PARTIAL.

## Draft greeting — 2026-10-26

- The empty draft no longer uses the office sentence. It follows the reference local-hour bands and both language strings, and arms one timer for the next boundary instead of polling.
- The title still measures at 30px and only shrinks, in 1px steps down to 20, when that line is wider than the row.
- A Z mark sits behind the title. Light mode draws the stroked path in the subtlest color at 70% opacity with a bottom fade. Dark mode uses the pinned `Z.svg` asset and does not add another opacity. There is no remote-control 20px mode and no office-mode override.
- No GUI capture was taken. Whole-product visual acceptance remains PARTIAL.

## Responsive measurement correction - 2026-10-26

- Corrected `@md/conversation` to Tailwind's 28rem (448px) container breakpoint; it is not the viewport `md` breakpoint of 768px. The 447/448px boundaries and shared row widths are covered by tests.
- User bubbles measure against the actual padded transcript row: 432px at a 480px conversation. The 624px query still uses the outer conversation width, so bubbles remain capped at 576px throughout the 624-671px interval as well as wider layouts.
- No GUI capture was taken in this round. Whole-product visual acceptance remains PARTIAL.

## Fenced code frame — 2026-10-26

- Markdown code blocks now use the reference 12px frame instead of the 6px theme radius, and 14px monospace instead of the renderer's 13px default. The card fill and border are unchanged.
- The floating language label and the renderer's 16px horizontal padding remain. Copy, wrap, and syntax-theme controls were not added.
- No GUI capture was taken. Whole-product visual acceptance remains PARTIAL.

## Markdown block spacing — 2026-10-26

- Assistant markdown blocks now use a 16px gap, matching Streamdown `space-y-4`, instead of the renderer's 12px default.
- Headings add 8px of top padding and 16px of bottom padding. After a normal block the visible gap is 24px, and the space below a heading is 16px. A heading at the start of a message still has the 8px top padding.
- List item spacing, blockquote color, and horizontal-rule spacing stay with the renderer. No GUI capture was taken. Whole-product visual acceptance remains PARTIAL.

## Tool summary row and reasoning — 2026-10-26

- Tool rows now follow `ToolSummaryRow`: a static subtlest tool icon, a medium kind label, and regular detail text. Bash shows its command, file tools show the path, and search shows the pattern. Success no longer appends a completion word or a middle dot. Failure uses 执行失败/Failed in the same subtlest color. Generic tools keep 执行中/已执行/已停止.
- The disclosure chevron stays visible. Reference hover-only opacity and the running gradient were not copied. Read and search still expand their raw output here, because this app has no file preview.
- Reasoning uses the brain trigger and the completed "a few seconds" duration. The open body is plain selectable text with the reference indent, left border, and 240px cap, not assistant Markdown.
- No GUI capture was taken. Whole-product visual acceptance remains PARTIAL.

## Message actions — 2026-10-26

- Completed user rows and completed assistant text now reserve the reference action row: 4px above a 24px ghost button, 14px copy icon, 6px radius, and 4px gaps. The row stays in the layout at zero opacity until the message is hovered or focus is inside the action row.
- Assistant rows also show the reference time label. Today is the clock, yesterday uses 昨天/Yesterday, a same-year date is month/day, and older dates include the year. Chinese stays 24-hour; English uses the en-US 12-hour form.
- Copy writes the message text to the clipboard and swaps in the green check for 1200ms. Like, dislike, fork, and edit stay absent because this app has no feedback, branch, or inline-edit command.
- No GUI capture was taken. Whole-product visual acceptance remains PARTIAL.

## Destructive color — 2026-10-26

- Storage, run, settings, and automation failures now use the Zai destructive token (`#e03131` light / `#ff5c5c` dark) instead of the fixed `#b42318`. Stopped runs remain on the subtle foreground.
- Review errors and failed command results use the same text, a 30% destructive border, and the theme destructive surface (10% light / 16% dark). The old review and command text passed a 6-digit hex to `rgba`, so the failure color was not actually red.
- No GUI capture was taken. Whole-product visual acceptance remains PARTIAL.

## Interface font field — 2026-10-26

- The appearance text-size control is the reference 112px field. The px suffix is inside the input and the value is right-aligned, instead of sitting beside the box.
- Enter and blur still commit the 12–20px range. No GUI capture was taken. Whole-product visual acceptance remains PARTIAL.

## Card border — 2026-10-26

- `--color-card-border` now follows the Zai alias of `--color-border`: black at 10% in light mode and white at 10% in dark mode. The review divider, command drawer, automation form, empty state, and task cards no longer use the opaque `#e6e6e6` / `#303030` line.
- Skill rows and automation cards hover with the 5% hover surface. Automation press still uses the stronger selected surface.
- No GUI capture was taken. Whole-product visual acceptance remains PARTIAL.

## Message action focus — 2026-10-26

- Completed user and assistant action rows now follow the reference `focus-within` rule as well as row hover. A non-tab-stop focus handle on the action row reveals it when the copy button inside is focused. Parent opacity is what changes, because a transparent parent would keep the focused button invisible.
- The expand control on a long user message is outside that row, so focusing it does not reveal the actions. Like, dislike, fork, and edit stay absent.
- No GUI capture was taken. Whole-product visual acceptance remains PARTIAL.

## Reasoning disclosure — 2026-10-26

- The reasoning chevron now follows the reference trigger: it stays in the row at zero opacity until the trigger is hovered or the body is open, then rotates open. Tool-summary chevrons stay visible.
- The streaming summary line and gradient sweep are still not replicated. No GUI capture was taken. Whole-product visual acceptance remains PARTIAL.

## Reasoning streaming summary — 2026-10-26

- A collapsed running reasoning row now shows the latest non-empty line beside 正在思考/Thinking, after a subtlest middle dot. The summary uses subtle text and keeps the newest character tail that fits the row. `\r` and `\n` both end a line.
- The row width is the transcript column: content width minus 32px, capped by the composer column. The fit reserves the 16px brain, the 16px chevron, four 8px gaps, and the measured label and dot. The trigger stays as wide as its content.
- Open rows, finished rows, and a tail that cannot fit omit the summary. The gradient sweep, CSS edge mask, and animated text are not replicated.
- No GUI capture was taken. Whole-product visual acceptance remains PARTIAL.

## Running label sweep — 2026-10-26

- 正在思考/Thinking and a running tool kind label now follow `animated-gradient-text`. The ink is #0d0d0d in light and white in dark. A 22% notch crosses the glyphs during the first 2s of each 4s cycle, then the label holds solid until the cycle repeats.
- Each glyph keeps its own alpha inside one medium text run, so the label width stays with the measured string. Reduced motion uses the solid ink and does not start the cycle. Icons, detail, status, and the reasoning summary line stay subtlest or subtle.
- The summary line still has no 16px edge mask and no queued roll. No GUI capture was taken. Whole-product visual acceptance remains PARTIAL.

## Reasoning summary mask — 2026-10-26

- An overflowing collapsed reasoning line now fades 16px at both ends, matching the reference mask. The fades are chat-surface overlays because GPUI has no text mask. A line that fits the row stays sharp.
- The visible string is still the newest fitting tail, not a scrolled full line. The queued roll animation is not replicated.
- No GUI capture was taken. Whole-product visual acceptance remains PARTIAL.

## Reasoning summary roll — 2026-10-26

- A collapsed reasoning summary now rolls when its source line index changes. The new line enters from 0.8em below and the previous line leaves upward, over 300ms with the reference cubic-bezier(0.4, 0, 0.2, 1). The first line does not roll in.
- Later tokens on that same line replace the text in place. Reduced motion swaps immediately and does not keep the outgoing line. Several line changes in a row restart the 300ms roll instead of queueing behind the reference 500ms hold.
- No GUI capture was taken. Whole-product visual acceptance remains PARTIAL.

## Reasoning summary queue — 2026-10-26

- A collapsed reasoning line now keeps the reference 800ms slot: 300ms of movement, then a 500ms hold. Another line change queues instead of restarting the roll.
- The queue keeps the next line and lets the newest arrival replace the slot after it. A hold that fires more than 250ms late, with two lines waiting, plays only the newest line. Tokens on the line that is showing still replace its text in place.
- Reduced motion clears the queue and shows the current line immediately. The visible tail is still the newest part that fits, not a scrolled full line.
- No GUI capture was taken. Whole-product visual acceptance remains PARTIAL.

## Tool disclosure — 2026-10-26

- A finished tool row now follows `ToolSummaryRow`: the 16px chevron is hidden until the row is hovered or the output is open. Opening rotates it 90 degrees.
- Opacity and rotation use the reference 200ms ease-out (`cubic-bezier(0, 0, 0.2, 1)`). Reduced motion snaps to the end state. Unfinished calls still have no chevron. The row still does not take a filled hover.
- No GUI capture was taken. Whole-product visual acceptance remains PARTIAL.

## Reasoning disclosure motion — 2026-10-26

- The reasoning chevron now fades and rotates with Tailwind's default transition: 150ms and cubic-bezier(0.4, 0, 0.2, 1). It is still hidden until the trigger is hovered or the body is open. Reduced motion snaps.
- Tool rows keep the separate 200ms ease-out from `ToolSummaryRow`.
- No GUI capture was taken. Whole-product visual acceptance remains PARTIAL.

## Assistant stream fade — 2026-10-26

- The current assistant row now fades new text with `zcode-stream-text-in`: 900ms and cubic-bezier(0.16, 1, 0.3, 1), instead of the previous 280ms ease-out with a 10ms stagger.
- The extracted stylesheet leaves `--zcode-stream-animation-delay` at 0, so one update fades as a single piece. Reduced motion still skips the fade. Completed rows stay static.
- No GUI capture was taken. Whole-product visual acceptance remains PARTIAL.

## Workflow scrollbar motion — 2026-10-26

- The timeline scrollbar is no longer the shared base scrollbar. That widget takes its fade and thickness timing from the global theme, and `ScrollbarMode::Always` skips the fade, so forcing it on for timeline hover made the bar snap.
- The bar now follows `.wf-sb` and `useTimelineViewport`: hidden at rest, shown for timeline hover, bar hover, dragging, or a scroll within the last 800ms. Opacity eases over 160ms and the 2px track thickens to 4px over 120ms, both with cubic-bezier(0.22, 0.61, 0.36, 1). Reduced motion snaps. The thumb length stays viewport squared over content, with a 24px minimum.
- Track clicks still page by one viewport immediately. Thumb drags use the reference `scrollWidth / clientWidth` ratio. Running-stage camera moves now use the same interruptible transition as ledge and keyboard moves.
- No GUI capture was taken. Whole-product visual acceptance remains PARTIAL.

## Reasoning scroll mask — 2026-10-26

- An open reasoning body now uses the reference scroll mask: 24px fades, shown at the top when scrollTop is past 1px and at the bottom until the scroll position is within 1px of the end. A body that cannot scroll has neither fade. GPUI still has no mask image, so the fades are chat-surface overlays that do not take hits.
- Opening the body, and streaming while it stays at the end, pins to the bottom. A wheel that would leave more than 2px of text below releases the pin. Returning to that 2px band pins it again. Collapsing the body rearms the pin for the next open.
- No GUI capture was taken. Whole-product visual acceptance remains PARTIAL.

## Reasoning summary scroll — 2026-10-26

- A collapsed reasoning summary now keeps the full latest line. The one-line viewport is content-sized until the remaining column width, then clips with overflow hidden. Its horizontal offset is the browser clamp of `scrollLeft = scrollWidth`, so the newest glyphs stay at the right edge.
- The 16px edge fade appears only when the full line is more than 1px wider than that viewport. A line that cannot fit is no longer omitted. The label and middle dot stay fixed; only the summary takes the remaining width.
- The 300ms roll and 500ms hold are unchanged. Same-line tokens still replace the text in place, and reduced motion still swaps immediately, but both the incoming and outgoing lines are the full scrolled lines.
- No GUI capture was taken. Whole-product visual acceptance remains PARTIAL.

## Message action fade — 2026-10-26

- Completed user and assistant action rows now fade with Tailwind `transition-opacity`: 150ms and cubic-bezier(0.4, 0, 0.2, 1). They still stay in the layout and appear for row hover or focus inside the action row. Reduced motion snaps.
- Like, dislike, fork, and edit stay absent. No GUI capture was taken. Whole-product visual acceptance remains PARTIAL.

## Work row spacing — 2026-10-26

- Consecutive reasoning and tool rows now use the reference work-group gap of 16px. Assistant prose still sits 20px below the previous row, including prose that follows reasoning inside the same message. A later user bubble still keeps the 56px turn opening plus the previous 20px.
- No GUI capture was taken. Whole-product visual acceptance remains PARTIAL.

## Turn column padding — 2026-10-26

- The shared content column is unchanged: full width below 864px, `calc(100% - 6rem)` capped at 896px, then `calc(100% - 24rem)` capped at 1152px from 1280px.
- Turn rows now keep that column's padding inside the column. It is 16px below Tailwind's 448px `@md` container breakpoint and 24px from there. User-message measurement uses the resulting inner row width, so narrow rows do not measure text against the outer conversation width or subtract the bubble padding twice. The existing-conversation composer still uses 16px inside the same column, so its shell is 8px wider than the message text on each side once the turn padding steps up. The draft shell is not padded a second time.
- No GUI capture was taken. Whole-product visual acceptance remains PARTIAL.

## Terminal command panel — 2026-10-26

- An expanded bash, execute, or shell row now hides the command from the summary and shows it inside the execute panel. The panel keeps the `$` prompt, interface-size monospace, 20px line height, a three-line collapse, and a ten-line open body. The toggle appears only after the collapsed command overflows by more than 1px. Closing the tool row clears that toggle.
- Output stays at a 100px cap. Empty output uses 没有输出。/No output. Fallback raw text now follows `text-ui-xs` instead of a fixed 12px size.
- The command preview keeps the `$` prompt and command in one scroll viewport, resets to the start when toggled, and accepts the same page/home/end and arrow navigation while focused. Wheel input is consumed only while that handle can move in the requested direction; at either edge it continues to the transcript. Disclosure and scroll keys include the session ID, so identical tool-call IDs in separate conversations do not share state.
- No GUI capture was taken. Whole-product visual acceptance remains PARTIAL.

## Timeline wheel and camera continuation - 2026-10-26

- The horizontal workflow scroller now restricts GPUI's axis remapping. Pure vertical wheel input reaches the surrounding transcript without also moving the timeline; horizontal gestures still cancel automatic camera intent and stay inside the overflowing timeline.
- Running-stage camera, ledge selection, left/right navigation and scrollbar track paging share an interruptible native transition. Wheel input and thumb drag cancel it at the displayed position; replacement moves start there. Reduced motion lands immediately, and live layout bounds clamp every sampled position.
- The active camera target is passed into the fixed-point ledge calculation, so the station being brought into view is not rendered as a folded lamp during its transition.
- Camera keep state is cleared when the transition lands, when reduced motion is enabled mid-flight, when overflow disappears, and when manual input takes over. Focused viewport regression coverage now includes 21 timeline tests.
- The pinned source uses browser `behavior: "smooth"` without a fixed duration. Native travel currently uses 300ms with cubic-bezier(0.22, 0.61, 0.36, 1); exact browser timing, focus retention and intermediate folded-stage rendering still need matched GUI replay.
- No GUI capture was taken. Whole-product visual acceptance remains PARTIAL.

## Draft suggested prompts, error banner, and queue panel — 2026-10-26

- Draft empty state now renders the 4 ZCode developer task chips (`ConversationDraftSuggestedPrompts.tsx`): 审查变更 (`Glyph::GitPullRequest`), 编写测试 (`Glyph::Check`), 解释架构 (`Glyph::FileText`), 排查问题 (`Glyph::Sparkles`). Placed 24px below the composer (`mt-6`), 32px height, 8px rounded corners, 12px padding, 6px icon-label gap, 14px text truncated at 256px max width. Clicking inserts the prompt, persists active draft, and focuses composer.
- Error banner now follows `ChatErrorBanner.tsx`: distinguishes `Stopped` (subtle border/text, `Glyph::Square`) vs `Failure` (destructive red, `Glyph::CircleAlert`, 30% border, surface fill) tones, with 12px radius, 12px/8px padding, max width aligned to composer column, and 28px outline retry action button with `Glyph::RefreshCw`.
- Queue panel now follows `ConversationQueuePanel.tsx`: card frame with 12px radius, 1px border, dock background, 12px padding. Paused state features `Glyph::Pause` banner and localized resume action (`Glyph::Play`). Active state displays `Glyph::ListOrdered` with item count. Queued items support pulling back into composer via edit action (`Glyph::SquarePen`) and removing via `Glyph::X`.
- No GUI capture was taken. Whole-product visual acceptance remains PARTIAL.

## User input epilogue, goal banner, and pending recovery — 2026-10-26

- User message bubbles now support engine-injected instruction epilogues (`ConversationUserInputEpilogue.tsx`): `split_user_input_epilogue` separates prompt body from trailing instructions, and `trim_epilogue_lead` removes leading markdown horizontal rules (`---`). The epilogue fold defaults to closed with a 12px chevron and localized "工作流引擎指令 / Workflow engine instructions", expanding into a monospaced scrollable preview. Messages consisting entirely of engine instructions (such as turn nudges) render a bubble containing only the epilogue fold.
- Conversation goal banner now follows `ConversationGoalBanner.tsx`: supports `/goal <objective>`, `/target <objective>`, `/goal resume`, `/goal clear`, and `/compact` slash commands. Active goals render a top banner across conversation views with subtle label, truncated objective with tooltip, status and iteration badge (`running · #1`), and dismiss button. Turn completions advance the iteration counter and mark completion.
- Pending command recovery banner now follows `PendingCommandRecoveryBanner.tsx`: when an uncompleted prompt is retained across app restarts or interruptions, a recovery banner surfaces above the composer column with "重新发送 / Resend" (restoring text into composer) and "忽略 / Dismiss" actions.
- GPUI-Kit component rules: zero duplicate `.hover()` handler registrations on `gpui_kit::component::button::Button` across all newly aligned views to prevent runtime style registration panics.
- Verification: 99 tests passed, `cargo fmt --check` and `git diff --check` passed. Whole-product visual acceptance remains PARTIAL.

## Workflow narrow overflow and fold-lamp replay — 2026-10-26

- The five-stage timeline now gives its horizontal station row an explicit `168 + 4 * 192 = 936px` content width and uses a fixed 24px gap, independent of the 14px interface text scale. This keeps the reference 192px station pitch aligned with the fold and camera geometry.
- In a rebuilt light Chinese offline fixture at a 480x600 native window, the timeline measured a 402px viewport with a 534px maximum horizontal offset. The native accessibility tree exposed the folded lamps as 18x18 buttons.
- On the temporary diagnostic build, AXPress on the `独立审计：换人验收 · 已完成` folded lamp moved the viewport to offset 459px. The [follow-up capture](visual-parity/runs/zcode-3.16.2/native-diagnostic-20261026/light-fold-lamp-480x600.png) visibly showed later stages, the edge rail, and the horizontal scrollbar thumb. This proves the diagnostic narrow navigation path and scroll-range establishment for the completed fixture; it does not prove matched ZCode pixels, final-build mouse/keyboard parity, running/error states, or whole-product acceptance.
- After removing diagnostics, the [Skills capture](visual-parity/runs/zcode-3.16.2/native-diagnostic-20261026/light-skills-480x640.png) confirms that category wrapping, search, list text, and use actions remain within the 480px window. The then-current window configuration clamped the requested 600px height to 640px; this is not the same viewport as the earlier timeline replay. The fixture remained offline and no prompt was submitted.
- Verification after the cleanup: `cargo fmt -p pi-desktop -- --check`, `CARGO_TARGET_DIR=/tmp/codex-native-target cargo test -p pi-desktop --offline` (99 passed), `cargo build -p pi-desktop --offline`, and `git diff --check`. Whole-product visual acceptance remains PARTIAL.

## Window chrome, desktop button position, top overlay actions, and turn file summary panel — 2026-10-26

- Window chrome and bounds (`packages/desktop/src/main/desktopWindowSize.ts`, `desktopWindowButtonPosition.ts`, `desktopWindowChrome.ts`):
  - Updated window default bounds from `1180x760` to `1200x800` (`DEFAULT_DESKTOP_WINDOW_WIDTH` / `DEFAULT_DESKTOP_WINDOW_HEIGHT`).
  - Updated window minimum bounds from `480x480` to `480x640` (`MIN_DESKTOP_WINDOW_WIDTH` / `MIN_DESKTOP_WINDOW_HEIGHT`).
  - Updated macOS traffic light position to `point(px(22.), px(23.))` (`MACOS_TRAFFIC_LIGHT_BASE_POSITION`).
  - Updated macOS title bar and header clearance to `96px` (`MACOS_TRAFFIC_LIGHT_BASE_LEFT_PADDING_PX`).
  - App title, sidebar title, and key binding context updated to `"ZCode"`.
- Top overlay and sidebar actions (`packages/ui/src/DesktopTopOverlay.tsx`):
  - Sidebar collapse button icon uses `Glyph::PanelLeftClose`.
  - Sidebar expand button in collapsed header uses `Glyph::PanelLeftOpen`.
  - When sidebar is collapsed in wide mode, adjacent New Task button (`Glyph::SquarePen`, "新任务" / "New Task") is displayed matching `DesktopTopOverlay.tsx`.
- Turn file summary panel (`packages/ui/src/v4/ConversationFileSummaryPanel.tsx` and `packages/desktop/scripts/gen-ui-fixture/file-summary-checks.ts`):
  - Added `FileChangeItem` and `TurnFileChanges` in `task_state.rs` with `files_count`, `total_additions`, `total_deletions`, and backwards-compatible deserialization defaults.
  - Added `extract_message_file_changes` in `workbench.rs` detecting `write_file`, `write`, `edit_file`, `edit`, `patch`, `apply_patch`, calculating additions/deletions and aggregating by path.
  - Added `file_summary_panel` collapsible card at the end of assistant turns:
    - 40px height header row, 12px rounded, 1px border.
    - Rotating chevron (`Glyph::ChevronDown` / `Glyph::ChevronRight`), localized `{count} 个文件已修改` / `{count} files changed` label, green `+{additions}` / red `-{deletions}` tabular counts.
    - Disabled ghost "撤销 / Rewind" button with `Glyph::Undo2`.
    - Collapsible file list with `Glyph::FileText`, filename (medium) and path (subtle), diff counts, and outline "审查 / Review" button jumping to Review panel for that file.
- GPUI-Kit Component Compliance ("注意 gpuikit"):
  - Zero `.hover()` handlers chained on `gpui_kit::component::button::Button` to prevent style conflict runtime panics; hover effects are placed exclusively on enclosing `div()` surfaces.
- Verification: 101 tests passed offline in 0.14s; `cargo fmt -- --check` and `git diff --check` passed cleanly. Whole-product visual acceptance remains PARTIAL.

## Sidebar navigation, history items, header geometry, and composer brand controls — 2026-10-26

- **Workbench header**:
  - Aligned header height to 48px (`WORKBENCH_HEADER_HEIGHT: Pixels = px(48.)`, matching `h-12` in `packages/ui/src/WorkspaceHeader.tsx`).
  - Added subtle `border-b border-border/50` bottom border when in active conversations (`.border_b_1().border_color(rgba(c.card_border))`).
- **Sidebar items and navigation** (`packages/ui/src/NewTaskButtonGroup.tsx`, `packages/ui/src/TaskListItem.tsx`, `packages/ui/src/WorkspaceSidebarFooter.tsx`):
  - Updated `SIDEBAR_HISTORY_ROW_HEIGHT` to `32px` (`h-8`) and `SIDEBAR_NAV_RADIUS` to `8px` (`rounded-lg`).
  - New task button (`NewTaskButtonGroup.tsx`): 32px height, 8px radius, `SquarePen` (16px), localized label `"新建对话" / "New thread"`, and right-aligned shortcut badge displaying `⌘N` (or `Ctrl+N` on non-mac) in `foreground_subtlest`.
  - History entries (`TaskListItem.tsx`): 32px row height, 8px radius, 16px leading slot displaying `Loader` spinner for active executing sessions or subtle `MessageSquare` (14px) for idle sessions, 14px truncated title.
  - Sidebar footer: removed dividing border line, using standard `px-4 pt-2 pb-4` padding matching `WorkspaceSidebarFooter`.
- **Composer layout and buttons** (`packages/ui/src/prompt-editor/ChatPromptEditor.tsx`, `packages/ui/src/v4/composer/V4ComposerToolbar.tsx`, `packages/ui/src/v4/ConversationComposer.tsx`):
  - Toolbar reorganization into leading (left) and trailing (right) clusters:
    - Left leading cluster:
      - More actions (`+`) button: 32px × 32px (`icon-md`), 8px rounded, ghost button with 16px `Plus` icon.
      - Model picker trigger: 28px height, 8px rounded, ghost, 8px left and 6px right padding, 16px `Package` icon, 12px truncated model label, 14px down chevron. Placed directly adjacent to `+` on the left.
    - Right trailing cluster:
      - Real-time token usage counter (subtle 12px text).
      - Send button: 32px × 32px, 8px rounded, GPUI-Kit `ButtonCustomVariant` using Brand black in light mode (`0x000000`, text `0xffffff`, hover `0x000000cc`) and Brand white in dark mode (`0xffffff`, text `0x000000`, hover `0xffffffcc`), with 16px `ArrowUp` icon.
      - Stop button: 32px × 32px, 8px rounded, secondary surface (`0xe6e6e6` in light, `0x363636` in dark), 16px `Square` icon.
- **User message bubble**:
  - Aligned border color with semantic `card_border` (`rgba(13, 13, 13, 0.1)` in light / `rgba(255, 255, 255, 0.1)` in dark), surface background `card_user_bg` (`0x0d0d0d08` / `0xffffff0d`), line height 20px (1.428) for 14px text (`text-ui-base`), 12px border radius with `2px` top-right radius (`rounded-xl rounded-tr-xs` in ZCode `ConversationUserInputBubble.tsx`).
- **GPUI-Kit Component Compliance ("注意 gpuikit")**:
  - Strictly avoided calling `.hover()` on `Button`. All custom button styling used built-in `ButtonVariants::custom(ButtonCustomVariant)` or `ButtonVariants::secondary()`.
- **Verification**: 108 tests passed offline; `cargo fmt -- --check` and `git diff --check` passed cleanly. Whole-product visual acceptance remains PARTIAL.

## Message feedback actions, user message editing, and review side pane styling — 2026-10-26

- **Message Actions and Interactions** (`packages/ui/src/v4/ConversationRowView.tsx` and `packages/ui/src/components/ai-elements/message.tsx`):
  - **Assistant message row actions**:
    - Copy action: 24px × 24px ghost button (`Glyph::Copy` / `Glyph::Check`), 14px icon, tooltip "复制 / Copy".
    - Thumbs up reaction: 24px × 24px ghost button (`Glyph::ThumbsUp`), 14px icon, tooltip "赞同 / Helpful". Active state highlighted with success green (`rgb(c.accent_green)`). Toggles `MessageFeedback::Like`.
    - Thumbs down reaction: 24px × 24px ghost button (`Glyph::ThumbsDown`), 14px icon, tooltip "不满意 / Unhelpful". Active state highlighted with active orange/amber (`rgb(c.workflow_active)`). Toggles `MessageFeedback::Dislike`.
    - Derive time label: 12px text in `rgba(c.foreground_subtlest)` displaying localized message timestamp derived from `row.createdAt`.
  - **User message row actions**:
    - Copy action: 24px × 24px ghost button, 14px icon, tooltip "复制 / Copy".
    - Edit action: 24px × 24px ghost button (`Glyph::Pencil`), 14px icon, tooltip "编辑 / Edit". Clicking automatically populates the composer textarea with this message's text and focuses the composer.
- **Review Side Pane Shell and Borders** (`packages/ui/src/app-shell/AnimatedSidePanePanel.tsx` and `packages/ui/src/GitPane.tsx`):
  - Header: aligned to 48px height (`h-12`) with 14px semibold title (`text-ui-base font-semibold text-foreground`), `border-b border-border/50` bottom border, 28px ghost Refresh button (`Glyph::RefreshCw`), and 28px ghost Close button (`Glyph::X`).
  - Container: full height with `bg-background` (`rgb(colors.root_bg)`).
  - Borders: replaced all `settings_border` occurrences with semantic `card_border` (`rgba(13, 13, 13, 0.1)` in light mode, `rgba(255, 255, 255, 0.1)` in dark mode) for crisp and subtle 1px frame dividers in file list, diff viewer, and status surfaces.
  - Diff view header: 12px monospace text with subtle background (`rgba(colors.card_user_bg)`).
- **GPUI-Kit Component Compliance ("注意 gpuikit")**:
  - Maintained zero `.hover()` calls on `gpui_kit::component::button::Button`. Reaction active states set icon colors directly on inner `Icon` element.
- **Verification**: 109 tests passed offline in 0.54s; `cargo fmt -- --check` and `git diff --check` passed cleanly. Whole-product visual acceptance remains PARTIAL.

## Transcript row alignment, full-width markdown, sidebar list header, and search action — 2026-10-26

- **Transcript cross-axis alignment & turn spacing** (`packages/ui/src/v4/ConversationTimeline.tsx` and `ConversationTurnGroup.tsx`):
  - Fixed cross-axis centering bug in message row wrapper: replaced `.items_center()` with `.items_stretch()`. This enables user messages to hug the right edge via `.items_end()`, assistant turns to stretch and left-align across the full content column, and workflow cards and file summary panels to span the full width.
  - Ensured assistant markdown text wrapper has `.w_full().min_w_0()` so text and code blocks expand across the full content column width.
- **Sidebar primary navigation and conversation list spacing** (`Sidebar.tsx` / `MyAgentSidebarTree.tsx` / `ProjectSidebarNavigation.tsx`):
  - Aligned primary navigation order: `New Task` (`SquarePen` / `MessageCirclePlus`), `Automations` (`CalendarClock`), `Skills` (`Sparkles`), matching open-source desktop specifications.
  - Aligned sidebar top header with macOS traffic lights clearance (`96px`), window title ("ZCode"), and right-aligned action group containing Search toggle (`Glyph::Search`, `⌘K` / `Ctrl+K`) and Sidebar collapse (`Glyph::PanelLeftClose`).
  - Connected conversation session items directly below primary navigation with clean 8px–12px vertical rhythm (`pt_1().pb_1()` on nav container, `pt_1()` on list container), eliminating the artificial horizontal divider line and redundant "任务 / Tasks" section title that caused an exaggerated 60px+ vertical gap.
  - When search is active, an expandable search input appears at the head of the history list with `px_2().pb_2()`.
  - Added 16px top margin (`transcript_row_top(0, false, false)`) for initial non-user turns so workflow cards and assistant headers maintain clear breathing room below the workbench header bar.
  - History item horizontal padding aligned to `10px` (`px-2.5`) with 32px row height and 8px border radius.
- **Global Search Shortcut & Action** (`packages/ui/src/lib/keyboardShortcuts.ts`):
  - Added `ToggleSearch` action bound to `⌘K` (macOS) and `Ctrl+K` (other platforms).
  - Pressing `⌘K` toggles `search_open` and focuses the search input; pressing `Escape` dismisses search focus when open.
- **Verification**: 110 tests passed offline in 0.20s; `cargo fmt -- --check` and `git diff --check` passed cleanly. Native GUI app launched and verified via live Quartz window capture. Whole-product visual acceptance remains PARTIAL.

## File-summary and Review replay - 2026-10-26

- Rebuilt the `pi-desktop` diagnostic binary with the Command Line Tools SDK and replayed an isolated offline fixture between 23:25 and 23:33 (Asia/Shanghai) at the current window contract:
  - `1200x800` light summary focus, expanded summary, and Review capture.
  - `1200x800` dark summary capture.
  - `480x640` light and dark Review captures, plus expanded-summary captures.
- Keyboard replay evidence:
  - `Tab` reached the file-summary trigger after 37 focus steps.
  - `Enter` expanded the two-file summary exactly once.
  - `Space` collapsed it exactly once; a second `Enter` reopened it.
  - Long paths remained truncated inside the file rows.
- Review replay evidence:
  - The `审查 crates/pi-desktop/src/workbench.rs` action selected the matching unstaged file.
  - The pane rendered the actual `workbench.rs` diff, including the header and selectable monospace body.
  - The previous empty diff was fixed in `crates/pi-desktop/src/panels.rs` by using native selectable text rather than wrapping diff content in Markdown.
- Archived diagnostic captures and AX dumps are under [native-final-20261026](visual-parity/runs/zcode-3.16.2/native-final-20261026/).
- The captures belong to that replayed binary, not later edits to `main.rs` or `workbench.rs`. A subsequent current-source build/test pass does not retroactively validate those edits through the GUI. The `runs/` directory remains locally ignored by the repository's existing policy.
- Verification: `cargo test -p pi-desktop --offline` passed with **110 tests**, `cargo build -p pi-desktop --offline` passed, `cargo fmt -p pi-desktop -- --check` passed with `DEVELOPER_DIR=/Library/Developer/CommandLineTools` and `SDKROOT=/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk`, and `git diff --check` passed.
- This remains **PARTIAL**: no matched ZCode/native pixel pairs, full keyboard traversal, IME coverage, running/error workflow replay, or whole-product acceptance has been proven.

## Workflow comparison continuation - 2026-10-27

- Continued against ZCode 3.16.2 at `3c5b227649371507ddc4eb46ac5133afe6db1cab`, specifically `WorkflowAgentPill.tsx`, `roster-model.ts`, `WorkflowMoreRow.tsx`, `WorkflowTimelineLedge.tsx`, and `styles.css`.
- Reviewed the existing reference/native screenshots in [workflow-comparison-20261027](visual-parity/runs/zcode-3.16.2/workflow-comparison-20261027/). Its native `native-before-light.png` predates the idle-fill correction and this continuation. It is not evidence for the current source. The two workflow field snapshots in that directory agree, but matching data alone does not establish matching pixels.
- Nodes without nonblank detail now render as static surfaces: no button semantics, tab stop, hover action, or fabricated status-only detail. Their pointer clicks can still bubble to the digest, matching the reference span behavior. Nodes with real detail keep the existing local disclosure, explicitly labeled as details, and use a neutral 2px keyboard focus band. Both node and roster disclosures use GPUI-Kit Base Button with native Enter/Space activation and explicit `aria-expanded`; the styled component does not expose that state API.
- Stages with more than six nodes now pin five nodes, prioritizing running then failed nodes and preserving source order within each priority. The aggregate shows the remaining count, up to three faces in failed/running/waiting/completed attention order, and the hidden failure count. Expanded lists retain the pinned prefix and include every node once. Regression tests cover six/seven-node boundaries, blank details, priority, hidden failures, stable expansion, and chat-scoped roster keys.
- Capability difference remains explicit: this app has no reference run-sidebar host or child transcript navigation. The aggregate uses a reversible native inline disclosure with a chevron; it does not claim that the reference arrow/sidebar workflow has been reproduced. Grouped sidebar rosters and exact aggregate hover/motion are still open.
- Stage lamps now express the 10px core and outer ring separately. Running retains its 3px steady halo while the second ring expands; failed uses a 2px outer ring. This is source alignment, not a post-change pixel result.
- The current macOS session was rechecked with `CGSessionCopyCurrentDictionary`: `CGSSessionScreenIsLocked=1`. No attempt was made to unlock it, and no post-change native screenshot or pointer/keyboard replay was obtained. Light/dark, 480px, focus, running/error, and pixel acceptance remain **PARTIAL**.
- Final verification: **112 native tests passed**, optimized `cargo build -p pi-desktop --release --offline` passed, `cargo fmt -p pi-desktop -- --check` passed, and `git diff --check` passed, using the Command Line Tools SDK and `CARGO_TARGET_DIR=/tmp/codex-native-target`. The current optimized binary is `/tmp/codex-native-target/release/volt`; running diagnostic windows and older captures must not be attributed to it.
- One independent read-only verifier identified stable-order, steady-halo, and expanded-state issues during implementation; it confirmed all four corrections on the final source, with no new code blocker. This is a source review, not interaction or pixel acceptance.
- Additional strict Clippy checks did not pass: the dependency-inclusive command first stops at the pre-existing unnecessary lazy fallback in `pi-agent-core/src/delegation.rs:60`; `--no-deps` also reports existing desktop lints in settings, workbench, timeline, old workflow function signatures/test placement, and file-summary tests. These files and old ownership boundaries were not refactored to make an unrelated lint gate green. No whole-product acceptance, commit, push, or deployment is claimed.

## Run side pane and grouped roster follow-up - 2026-10-27

Parent: the workflow comparison continuation above.
Source: user's explicit "继续完成对照" continuation.
Reason: the previous card-only roster disclosure did not reproduce the reference run-side-pane navigation.
Scope: native workflow detail hosting, stage landing/disclosures, grouped remaining roster, shell sizing and focus restoration. Full-product visual and interaction alignment remains the original objective; this follow-up does not narrow its acceptance.
Risk/orchestration: medium; native single writer with one independent read-only verifier. No coordination database or agmesh deployment was added.

- Compared the same pinned source, especially `WorkflowRunPhaseList.tsx`, `WorkflowRunSidePaneSections.tsx`, `WorkflowMoreRow.tsx`, `WorkflowRoll.tsx`, and `WorkflowRosterParts.tsx`.
- Replaced the old horizontal-card detail dialog with a native right-side run pane backed by the same owning chat/run snapshot. It uses the current shell's 380px slot at >=1180px and a single content pane below that breakpoint. These native shell dimensions are not a measured copy of ZCode's resizable panel.
- The card now keeps five pinned participants plus the up-right-arrow aggregate. Selecting that aggregate opens the pane, expands its phase and roster, and requests post-layout top alignment. Repeating the selection requests the landing again without resetting other disclosures.
- The pane uses independent 36px stage headers, a 39px text-column inset, a vertical linear spine, observed progress, collapsed avatar clusters/44x3px meters, and a reversible 32px roster door. The remaining participants appear once in attention-ordered groups with two columns of 24px rows. Interrupted participants retain their native state rather than being counted as completed or waiting.
- Phase state is local to the open chat/run, opens all newly running phases, retains manual collapse across identical snapshots, and drops removed phase IDs. Node detail IDs are isolated between transcript and pane. Empty or whitespace-only details remain inert.
- Review/file-review opens close the run pane. Closing via X, Escape, or the header side-pane toggle restores the captured focus. Switching chats clears the pane; focusing the composer dismisses the single-column run pane so the editor is visible.
- Verification: `CARGO_TARGET_DIR=/tmp/codex-native-target cargo test -p pi-desktop --offline` passed with **116 tests**, `cargo check -p pi-desktop --offline`, `cargo build -p pi-desktop --release --offline`, `cargo fmt -p pi-desktop -- --check` and `git diff --check` passed using the Command Line Tools SDK. The final local binary is `/tmp/codex-native-target/release/volt`, SHA256 `99181e7bcbeab3852bf677acb8e9118b159e119a7c3316f1d8c78b1cfc66d909`. These are source/build gates only; they do not promote GUI interaction or pixel acceptance.
- One independent read-only verifier found a shortcut-driven Review transition that removed the pane without transferring focus. All Review toggle callers now pass the `Window`; toggles and file-review opens close the pane through the shared path and then focus the continuously mounted workbench root. The verifier confirmed that correction with no new code blocker; it did not claim GUI focus acceptance or rerun the owner tests.
- Remaining boundaries: the snapshot has no branch topology or child transcripts; no branch graph, actor conversation, configuration/resume capability, or remote state was fabricated. Landing uses immediate deferred native positioning; the reference smooth movement/1200ms flash, exact hover/face fan-out/arrival/meter transitions, rendered light/dark, 480px layout, keyboard focus, and matched pixels still need acceptance.

## Settings save button detail follow-up - 2026-10-29

Parent: the existing ZCode visual alignment objective.
Source: user's attached model-provider settings screenshot and request to refine button styling.
Reason: the save action inherited a navigation row's left-aligned, full-width inner content and a fixed 132px width.
Scope/risk/orchestration: low-risk native single writer; settings save button presentation only, without changing persistence, labels, icons, or success/error behavior.

- Replaced the navigation helper with a settings-owned GPUI-Kit primary Button: content-sized width, start-aligned self placement to prevent flex stretching, centered interface-size text, 32px height, 8px radius, 10px horizontal padding, 16px Check icon, and 4px content gap. Colors, hover, pressed and keyboard-focus behavior remain owned by the component; no duplicate hover registration was introduced.
- Added a builder-style regression test for Chinese/English labels at 12/14/20px, checking height, unspecified width, start alignment and padding. This test does not measure rendered labels, pointer states or pixel parity.
- Verification: `CARGO_TARGET_DIR=/tmp/codex-native-target cargo test -p pi-desktop --offline` passed **123 tests**; the matching release build passed. `rustfmt --edition 2021 --check crates/pi-desktop/src/settings.rs` and `git diff --check` passed. Package-wide `cargo fmt -p pi-desktop -- --check` remains blocked by existing formatting differences in `execution.rs`, `generative_ui.rs`, and `panels.rs`; those unrelated files were not reformatted.
- Status: **PARTIAL**. The current macOS window probe returned `Screen is locked`; no final-build capture or interaction replay was obtained. Still required: unlocked light/dark settings captures at 1200x800 and 480x640, Chinese/English and largest-text layout, hover/pressed/keyboard focus, save feedback, and matched-reference visual review. The local binary is `/tmp/codex-native-target/release/volt`; existing running windows and screenshots are not evidence for this build.

## Settings navigation button follow-up - 2026-10-29

Parent: the settings save button detail follow-up above.
Source: the user's ongoing ZCode comparison and button-detail feedback.
Reason: settings navigation reused the workbench action's small text and retained an empty label and gap in compact mode.
Scope/risk/orchestration: low-risk native single writer; settings navigation presentation and its local theme token only. Save behavior, section persistence and return-focus handling remain unchanged.

- Compared `SettingsSidebarButton` and Back in the pinned ZCode 3.16.2 source at `3c5b227649371507ddc4eb46ac5133afe6db1cab`.
- Settings-owned content now uses interface-sized 14px/20px text, 16px non-shrinking icons, and an 8px gap only when the visible label exists. Compact targets stay 40x40 with zero padding and centered icons; labels remain available through tooltip and accessibility name.
- Section icons/text use foreground and selection/hover use the reference surface-hover token (5% light / 10% dark). Back uses ArrowLeft, 6px wide inline padding, the idle surface and opaque remote-muted text, with foreground content on group hover. Its instance-level background now pins the idle surface after the component variant is applied, avoiding gpui-kit’s translucent custom-color fallback. Custom variants retain component activation/focus without registering a second button hover handler.
- Added Chinese/English, 12/14/20px wide/compact builder-style coverage, light/dark remote-muted token assertions, and a light/dark wide/compact Back-frame regression for the idle surface and geometry. These are source contracts, not rendered label measurements or interaction acceptance.
- Verification: **126 tests passed**, the matching offline release build passed, `rustfmt --edition 2021 --check` passed for `settings.rs`, `buttons.rs` and `theme.rs`, and `git diff --check` passed. Release binary: `/tmp/codex-native-target/release/volt`, SHA256 `4334c3f1f76292874413e0a4c548d98395e257fc5c404b1ee99d1ceb18315880`. Unrelated formatting-only changes made during checking were removed; `execution.rs`, `generative_ui.rs` and `panels.rs` have no diff.
- A temporary offscreen-render diagnostic did not produce screenshots or runtime evidence: application test-support resolution hit an optional Fast-backend Cocoa version conflict, and a separate diagnostic build exhausted system-disk space. The diagnostic manifest, lockfile and source were removed; application dependencies and lockfile remain unchanged.
- The macOS window probe again returned `Screen is locked`. Post-build light/dark and narrow captures, largest-text layout, pointer/keyboard states, save feedback and matched pixels remain **PARTIAL**. No screenshot from an older running binary is attributed to this change.

## Settings heading typography follow-up - 2026-10-29

Parent: the settings navigation button follow-up above and the full ZCode alignment objective.
Source: the user's ongoing comparison request and active goal continuation.
Reason: page titles and navigation group labels stayed at fixed sizes after the interface font preference changed; group labels used translucent foreground instead of the reference remote-muted color.
Scope/risk/orchestration: low-risk native single writer; settings page title and navigation group typography only. Button geometry, handlers, preference persistence, global text scaling and other page content remain unchanged.

- Rechecked the same pinned ZCode commit, `SettingsPage.tsx` and `styles.css`. Title tokens are additive: interface size +10px with a 4/3 line height compact, or +16px with a 1.2 line height wide. Default 14px rendering therefore retains 24px/32px and 30px/36px.
- Native titles now use those tokens, full available width, a shrinkable minimum width, no fixed height and natural wrapping. Navigation group labels use interface size -2px, 1.5 line height, medium weight and opaque light/dark remote-muted colors; compact navigation still omits them.
- Builder tests cover all eight section labels in Chinese/English at 12/14/20px in both layout modes, plus light/dark group styles. These assert style contracts, not actual glyph measurement, wrapping bounds or rendered contrast.
- Verification: **128 tests passed** with `CARGO_TARGET_DIR=/tmp/codex-native-target cargo test -p pi-desktop --offline -q`; the offline release build, focused `rustfmt --edition 2021 --check` and `git diff --check` passed. Binary: `/tmp/codex-native-target/release/volt`, SHA256 `2d38c6e56819f24a43ae932029ec5b143cba21bb82ca620c5549e4208b38b7d6`.
- The current macOS probe returned `Screen is locked`; no fresh native screenshots or interaction results were obtained. Whole-product parity remains **PARTIAL**, including maximum-font wrapping, light/dark wide/minimum layouts, focus/pointer states and matched-reference pixels. Remaining fixed-size settings body/caption text is outside this heading-only follow-up and still needs source comparison.

## Settings body typography follow-up - 2026-10-29

Parent: the settings heading typography follow-up above and the full ZCode alignment objective.
Source: the user's ongoing comparison request and active goal continuation.
Reason: settings body text still bypassed the interface-size preference in status values, skill rows, footer/provider notes and shortcut keycaps, leaving the page internally inconsistent at larger text sizes.
Scope/risk/orchestration: low-risk native single writer; settings typography inheritance only. The workbench root remains unchanged, and fixed controls, icons, spacing, persistence and handlers remain unchanged.

- Scoped `self.ui_text(14.)` inheritance to the settings frame. This keeps the main workbench's existing typography contract intact while making settings row labels and descriptions follow the interface preference.
- Converted settings captions to the reference base-minus-2px token and larger usage values to base-plus-2px. Provider notes, footer labels, status values, skill names/counts/descriptions and shortcut labels/keycaps now follow the same scale. Fixed 20px/24px line heights remain stable.
- Added builder coverage for the settings frame and caption styles across light/dark themes and 12/14/20px values. These tests verify source style contracts, not rendered glyph metrics or actual line wrapping.
- Verification: **130 tests passed** with `CARGO_TARGET_DIR=/tmp/codex-native-target cargo test -p pi-desktop --offline -q`; the offline release build, focused `rustfmt --edition 2021 --check` and `git diff --check` passed. Binary: `/tmp/codex-native-target/release/volt`, SHA256 `917685f9a71a44b5c0f7f924549063400e469111b87ee7363a4e41a10d4c64e2`.
- The macOS probe again returned `Screen is locked`; fresh light/dark, compact, maximum-size and interaction captures remain unavailable. Whole-product visual parity is **PARTIAL**.

## Settings form control follow-up - 2026-10-29

Parent: the settings body typography follow-up and the full ZCode alignment objective.
Source: the user's attached model-provider settings comparison and pinned ZCode 3.16.2 source.
Reason: single-line fields used `Input::h()`, although GPUI-Kit only applies that builder to multiline editors; the key visibility control retained the library's smaller geometry.
Scope/risk/orchestration: low-risk native single writer; provider input geometry, API-key visibility affordance and the shared font-size field frame. Persistence, IME handling, blank-key retention and session-only key semantics remain unchanged.

- Single-line height is now written to the style applied by the actual component renderer: 32px for endpoint/model/font-size and 36px for API key, with 12px horizontal inset, 8px radius and 20px line height. The native editor owns vertical centering rather than the library's default 8px vertical frame padding. The font-size suffix and right alignment remain intact.
- Replaced the internal key toggle with a 24px ghost button, 14px eye icon, 6px radius, 6px field inset and localized show/hide tooltip/accessibility label. Its native mask toggle notifies the parent so icon and label update with the actual state. The key field reserves 40px for this overlay.
- Added style-builder coverage for light/dark themes, 32/36px heights and 12/14/20px fonts, plus both eye-button states and languages. These are source contracts, not rendered geometry or interaction tests.
- Verification: **132 tests passed** with `CARGO_TARGET_DIR=/tmp/codex-native-target cargo test -p pi-desktop --offline -q`; offline release build, focused `rustfmt --edition 2021 --check` and `git diff --check` passed. Binary: `/tmp/codex-native-target/release/volt`, SHA256 `d3a8e8eb203be1d462e5dea0ec7b338d1da06cbe005284e76470d19c7368f50b`.
- The current macOS window probe returned `Screen is locked`. Fresh light/dark wide/narrow, largest-font, pointer/keyboard, mask-toggle/save-feedback and matched-reference acceptance remain **PARTIAL**. Older captures are not attributed to this binary.

## Settings Select trigger follow-up - 2026-10-29

Parent: the settings form control follow-up above and the full ZCode alignment objective.
Source: the user's ongoing ZCode comparison and the pinned ZCode 3.16.2 Select contract.
Reason: Select triggers retained the library's default vertical padding and horizontal insets, which made the 32px frame visually tighter than the reference control.
Scope/risk/orchestration: low-risk native single writer; settings Select trigger geometry only. Menu behavior, option state, focus handling, accessibility labels and existing width constraints remain unchanged.

- Compared the pinned `components/ui/select.tsx` large trigger: 32px height, 12px left inset, 8px right inset, 8px radius and no vertical frame padding.
- Added `settings_select` on top of the shared settings control frame and applied it to General language, Footer language, Footer theme and Appearance theme. The native GPUI-Kit menu and focus behavior remain the owner of interaction states.
- Added builder coverage for light/dark themes and 12/14/20px interface sizes, checking height, min-width, insets, typography, surface and border. These tests are source contracts, not rendered label measurements or pointer/focus acceptance.
- Verification: **133 tests passed** with `CARGO_TARGET_DIR=/tmp/codex-native-target cargo test -p pi-desktop --offline -q`; the matching offline release build passed. Focused `rustfmt --edition 2021 --check` and `git diff --check` passed. Binary: `/tmp/codex-native-target/release/volt`, SHA256 `a67468df0d379133222a1dbfe6821923d2ff511a78d3f90e72d836087a94cce5`.
- The current macOS window probe returned `Screen is locked`. Fresh light/dark 1200x800 and 480x640 captures, Chinese/English and largest-font layout, pointer/keyboard/focus interaction, and matched-reference pixels remain **PARTIAL**. No older running binary or screenshot is attributed to this change.

## Settings shortcut badge follow-up - 2026-10-29

Parent: the settings Select trigger follow-up above and the full ZCode alignment objective.
Source: user's ongoing button-detail comparison, pinned ZCode 3.16.2 `ShortcutBindingRow.tsx`, `kbd.tsx`, `ControlShortcutBadge.tsx`, `shortcuts/label.ts`, and the final Zai tooltip-tag tokens.
Reason: bindings used a merged cross-platform string and a static keyboard glyph, although the reference uses separate platform-specific keycaps; task search was missing from the displayed native bindings.
Scope/risk/orchestration: low-risk native single writer; shortcut settings presentation only. Key bindings, handlers, persistence, save behavior and the workbench root remain unchanged.

- Split bindings into platform-specific keycaps: Command and Shift symbols with the macOS system font, or Ctrl/Shift labels with the configured monospace font elsewhere. Each badge uses 16px height/minimum width, 6px inline padding/radius, base-minus-4px medium text, the reference tooltip-tag palette and a 4px inter-key gap.
- Removed the static Keyboard glyph, which the reference reserves for recording. Added the existing primary-K task-search binding so all eight native bindings are represented. No recording, rebinding or deletion controls were fabricated.
- Below a 640px viewport, rows stack label and binding with an 8px gap; wider rows retain the 12px horizontal gap. Key groups wrap without shrinking individual badges. Existing 16px/12px row padding and dividers remain.
- Added four focused tests for platform token ordering, actual keycap builder styles across both themes and 12/14/20px fonts, narrow/wide row styles, and group wrapping constraints. The theme palette test brings the suite to **138 passed** with `CARGO_TARGET_DIR=/tmp/codex-native-target cargo test -p pi-desktop --offline -q`. These assert source contracts, not rendered glyph bounds, key dispatch or pixel parity.
- The matching `cargo build -p pi-desktop --release --offline`, focused rustfmt checks and `git diff --check` passed. Binary: `/tmp/codex-native-target/release/volt`, SHA256 `754c537333dcf71c116adffa0faf5fabd18a9b78760b14db23d3ca31b2c92e9f`. The build reports the existing future-incompatibility warning for dependency `block 0.1.6`; no dependencies or lockfile changed.
- The fresh macOS window probe returned `Screen is locked`. No current-build screenshots or interaction replay were obtained. Light/dark 1200x800 and 480x640 captures, Chinese/English and largest-font layout, save/mask controls, hover/pressed/keyboard focus, and matched-reference pixels remain **PARTIAL**. The full visual-parity goal is not complete.

## Settings card clipping follow-up - 2026-10-29

Parent: the settings shortcut badge follow-up above and the full ZCode alignment objective.
Source: pinned ZCode `SettingsPageParts.tsx` `SettingsGroupCard`, plus the user's model-provider settings screenshot.
Reason: the native settings card matched the 12px rounded border and surface but lacked the reference card's overflow constraint.
Scope/risk/orchestration: low-risk native single writer; settings card frame presentation only. Child controls, persistence, handlers and card contents remain unchanged.

- Added `overflow_hidden()` to the shared settings card, used by General, Providers, Appearance, Agent capabilities, Subagents and Shortcuts. Skills and Usage use separate surfaces. GPUI's `Style::overflow_mask` constructs a rectangular content mask, so these flags do not establish rounded child masking or prevent all corner overpaint. Rounded-corner pixels and focus effects remain unverified.
- Added light/dark builder coverage for the overflow axes, frame geometry, radius, surface and border. This is source-level evidence, not a rendered corner pixel comparison.
- Verification: **139 tests passed** with `CARGO_TARGET_DIR=/tmp/codex-native-target cargo test -p pi-desktop --offline -q`; matching offline release build, focused rustfmt checks and `git diff --check` passed. Binary: `/tmp/codex-native-target/release/volt`, SHA256 `e7a826b591c112a999f87e8f72cdbc59028abe91bc796b795769f4cb9ec282f4`.
- The macOS window probe still returns `Screen is locked`; fresh current-build light/dark, compact, focus, interaction and pixel evidence remain **PARTIAL**.

## Settings secondary action detail follow-up - 2026-10-29

Parent: the settings button-detail follow-ups and the full ZCode alignment objective.
Source: user's attached button feedback and ongoing comparison request.
Reason: Use in task and Windows shortcut installation still reused workbench navigation content with full-width inner layout, small text and an 8px icon gap.
Scope/risk/orchestration: low-risk native single writer; settings command presentation only. Handlers, disabled state, persistence, navigation and workbench controls remain unchanged.

- Added settings-owned secondary commands using the pinned `button.tsx` default-size geometry: content width, 28px height, 6px radius, 8px inline padding, 14px icons, 4px gap and centered interface-size labels. The native ghost variant still owns interaction states. Windows installation sets start alignment at its column-layout call site; skill actions retain their row's vertical centering.
- Shared only the existing icon/label content between save and secondary commands; the save button retains its 32px/8px-radius/16px-icon large-button geometry.
- Added two builder tests covering Chinese/English labels and 12/14/20px fonts. They check content-width constraints, geometry, text and gap, not rendered glyph bounds or runtime interactions.
- Corrected the preceding clipping record after inspecting GPUI `Style::overflow_mask`: the mask is rectangular, not rounded, and shared `card()` does not host Skills or Usage. Rounded corner masking remains an open visual difference.
- Final-source verification: **141 tests passed** with `CARGO_TARGET_DIR=/tmp/codex-native-target cargo test -p pi-desktop --offline -q`; matching offline release build, focused rustfmt checks and `git diff --check` passed. Binary: `/tmp/codex-native-target/release/volt`, SHA256 `ba2abe8397ea63f5ad01467c2af6c977bb77c071ab24f7d5c648281dbc4bfb1b`. Dependencies and lockfile remain unchanged; the existing `block 0.1.6` future-incompatibility warning remains.
- The current macOS probe returned `Screen is locked`; no current-build capture or interaction replay was obtained. Matched light/dark 1200x800 and 480x640 captures, Chinese/English and maximum-font layouts, pointer/keyboard states and save feedback remain **PARTIAL**. The full visual-parity objective remains active; no commit, push or deployment was performed.

## Settings interface font control follow-up - 2026-10-29

Parent: the settings secondary action detail follow-up and the full ZCode alignment objective.
Source: pinned ZCode `settingsCodePreview.tsx` `FontSizeInput`.
Reason: the native interface-font control was 32px high and rendered `px` through the input's layout-participating suffix, while the reference uses a 28px compact input with an independently positioned unit label.
Scope/risk/orchestration: low-risk native single writer; Appearance interface-font control presentation only. Range validation, Enter/blur commit, persistence and inherited font scaling remain unchanged.

- Added a 112x28px relative frame. The input now uses a 6px radius, 8px left inset, 32px right reservation and right-aligned value; the `px` label is absolute and noninteractive at the right 8px inset, with interface-size-plus-2px text.
- Added builder coverage for light/dark themes and 12/14/20px interface sizes, including frame geometry, absolute unit placement, text alignment, reserved inset, surface and radius. These are source contracts, not rendered glyph or cursor measurements.
- Final-source verification: **142 tests passed** with `CARGO_TARGET_DIR=/tmp/codex-native-target cargo test -p pi-desktop --offline -q`; matching offline release build, focused rustfmt checks and `git diff --check` passed. Binary: `/tmp/codex-native-target/release/volt`, SHA256 `4d3910d72bdc6f170c1b7a805198fc3ee629bc6b4054053a6424c5172bf92fb4`.
- The current macOS probe remains `Screen is locked`; no current-build capture or interaction replay was obtained. Light/dark 1200x800 and 480x640 captures, Chinese/English and maximum-font layout, pointer/keyboard focus, save feedback and pixel comparison remain **PARTIAL**. No commit, push or deployment was performed.

## Workflow phase landing follow-up - 2026-10-29

Parent: the ongoing ZCode workflow run pane alignment and settings detail follow-ups.
Source: pinned ZCode `WorkflowRunPhaseList.tsx` and the native GPUI `ScrollHandle` implementation.
Reason: the previous ScrollHandle API already deferred positioning until prepaint, but jumped directly to the stage without smooth travel, explicit landing state or user cancellation.
Scope/risk/orchestration: low-risk native single writer; workflow pane phase landing and cancellation state only. Stage grouping, focus restoration, node disclosure and workflow persistence remain unchanged.

- Phase landing now waits for post-layout child bounds, starts from the current vertical offset, and uses a 300ms native cubic-bezier approximation to the reference smooth scroll. The destination is clamped to the live scroll range on every frame.
- Reopening the same stage replaces the current motion, reduced motion lands immediately, and stage removal or an unavailable workflow clears pending state. Wheel input on either axis, pointer and plain keyboard scrolling cancel the landing so the pane does not reclaim user control. An independent read-only verifier identified the horizontal-wheel case because GPUI can map it to vertical scrolling; regression coverage now includes both axes and zero-delta preservation.
- Added seven deterministic state tests covering deferred layout, repeated landing, cancellation on either wheel axis, reduced motion, resize/non-scrollable bounds and stage removal. These tests cover state and coordinate contracts, not native pointer replay or pixel output.
- Final-source verification: **149 tests passed** with `CARGO_TARGET_DIR=/tmp/codex-native-target cargo test -p pi-desktop --offline -q`; matching offline release build, focused `rustfmt --edition 2021 --check` and `git diff --check` passed. Final binary SHA256: `5994ef49150049488cf6c641a93de5e637748e751fbe01b1eee0df566cfe1ee7`. The build retains the existing `block 0.1.6` future-incompatibility warning.
- The reference's transient phase landing surface remains intentionally open after source inspection found duplicate `wf-land` declarations in the pinned stylesheet, with the later opacity keyframes overriding the earlier background-color keyframes. The screen briefly became available and the pre-review Release rendered successfully; see the limited capture evidence below. Stage landing itself was not replayed. Overall visual parity remains **PARTIAL** and no commit, push or deployment was performed.

## Settings button capture checkpoint - 2026-10-29

Parent: the user's attached save-button feedback and the settings detail follow-ups above.
Source: the isolated native Release process and direct inspection of its captured model-provider settings page.
Reason: source-style checks alone did not establish that the content-sized save action rendered correctly.
Scope/risk/orchestration: read-only native GUI checkpoint using an offline temporary state directory; no real credentials, requests or user-state changes.

- The isolated 1200x800 window successfully rendered the light-theme workflow and model-provider settings pages. AXPress navigation opened Preferences and Model providers. The save button visibly sizes to its Check icon and Chinese label with centered content, without the previous large right-side gap. Fields and the action remain visible without obvious overlap at this size.
- Preserved full-window captures outside the repository:
  - Workflow: `/Users/zhd/.codex/visualizations/2026/10/26/01a1764d-5a1f-74b2-8fa3-c0499328387b/workflow-light-20261029.png`
  - Settings: `/Users/zhd/.codex/visualizations/2026/10/26/01a1764d-5a1f-74b2-8fa3-c0499328387b/settings-save-light-20261029.png`
- These captures precede the final horizontal-wheel cancellation fix, which did not change settings presentation. They are not screenshots of the final binary hash and are not matched ZCode pixel comparisons. The settings capture contains a navigation tooltip.
- At 23:26 local time the selected-window probe returned `Screen is locked`; subsequent capture failed. Dark/480x640/maximum-font rendering, hover/pressed/keyboard focus, save feedback, key visibility and workflow landing replay remain unverified. The isolated test process was stopped; the user's existing application processes and real state were not touched.

## Workflow roster-door interaction follow-up - 2026-10-29

Parent: the ongoing full ZCode visual-alignment objective and workflow roster continuation.
Source: the user's explicit goal continuation, pinned `WorkflowMoreRow.tsx`, and the `.wf-more-deck`, `.wf-pill-name`, `.wf-pill-open` rules in `styles.css`.
Reason: the aggregate button kept a static -5px face overlap, changed its surface abruptly and did not promote its name to primary foreground on hover or keyboard-visible focus.
Scope/risk/orchestration: medium-risk native single writer with one independent read-only verifier, limited to the shared timeline/pane aggregate control. Roster ordering, counts, disclosure handlers, workflow persistence and the full objective remain unchanged.

- Added a local render component to own per-row hover, focus and transition state without widening the workflow renderer's signatures or adding application-global interaction maps. Its focus subscriptions leave with keyed element state. The native Base Button retains keyboard activation, accessibility label, aria-expanded, tooltip, ring and the existing click handler.
- Pointer hover and keyboard-visible focus animate the face overlap from -5px to -2px over 160ms with cubic-bezier(0.22, 0.61, 0.36, 1), matching the pinned stylesheet. A single interruptible engagement channel drives face spacing, alpha surface and label foreground; reduced motion uses zero duration. The open pane door remains emphasized independently from hover.
- Added four deterministic tests for the overlap bounds/timing, light/dark alpha surfaces and text, the open-door presentation, and all hover/focus/modality combinations. They do not prove hover delivery, focus traversal, actual truncation or rendered pixels.
- The independent verifier found that the first implementation selected hover or focus exclusively by input modality, and GPUI's default hover listener suppresses hover after keyboard input. Corrected engagement to hover OR keyboard-visible focus and selected `InputModalityIndependent` for the single hover listener. The regression includes Tab moving focus away while the mouse remains over the row. The same verifier confirmed the correction without new findings; real GUI replay remains pending.
- Re-inspected the landing keyframes rather than following the reference's stale background-flash comment: `.wf-landed` uses 1200ms, but the last `@keyframes wf-land` scales from 0.6 and fades from zero. Current GPUI exposes transforms for SVG/icon painting but no generic subtree scaling builder. Exact landing/pressed scale remains open; this follow-up does not replace it with a different visual effect or call it complete.
- Final-source verification, completed across midnight on 2026-10-29/30: **153 tests passed**, the matching offline Release build, focused rustfmt and `git diff --check` passed. Release SHA256: `37f5c2c728668833cfe1418ac23ad3892f9f0d73c77c2bd780212939c4c6682a`. Whole-package `cargo fmt --check` also reports pre-existing formatting differences in `execution.rs`, `generative_ui.rs` and `panels.rs`; those unrelated files were left unchanged. The existing dependency warning for `block 0.1.6` remains; manifests and lockfile did not change.
- At 00:00 on 2026-10-30, the selected-window probe again returned `Screen is locked`. No GUI was launched in this follow-up. This change has no new rendered or interaction evidence; prior light settings captures are not evidence for it. Overall visual parity remains **PARTIAL**, including full light/dark, 480x640, keyboard/pointer and matched-reference verification. No commit, push or deployment was performed.

## Workflow roster settlement and disclosure motion - 2026-10-30

Parent: the workflow roster-door interaction follow-up and the full ZCode alignment objective.
Source: pinned ZCode `WorkflowRunPhaseList.tsx`, `WorkflowRosterParts.tsx`, `WorkflowMoreRow.tsx`, and `.wf-motion` / `.wf-meter-seg` / chevron rules in `styles.css`.
Reason: pane meters changed segment weights immediately, and phase/roster disclosure icons swapped glyphs instead of preserving one icon and rotating it.
Scope/risk/orchestration: medium-risk native single writer with the existing read-only verifier reused; pane roster presentation and disclosure motion only. Workflow state, stage ordering, click handlers, focus ownership and persistence remain unchanged.

- Added a keyed native roster meter component with a fixed 44x3px frame, 1px inter-segment gap, 2px minimum segment width and observed node-count weights in the reference order: completed, failed, running, waiting, interrupted. Existing status colors remain semantic; interrupted remains distinct from waiting. Existing segments transition their flex weights over 320ms with the shared workflow easing, and reduced motion settles immediately.
- Added a shared native chevron component. Pane stage headers keep one 14px ChevronRight and rotate to 90 degrees over 150ms with the reference Tailwind easing; roster doors keep one 14px ChevronDown and rotate to 180 degrees over 160ms with the roster easing. The card's always-present up-right arrow remains unchanged, while its idle-to-engaged tail color now follows the same aggregate interaction channel.
- Added five deterministic tests for count/order semantics, fixed meter geometry, themed segment styles, chevron targets and reduced-motion timing. They verify source/state contracts, not rendered segment interpolation, pointer replay or pixels.
- The independent read-only verifier found no new code blocker in the scoped changes: fixed geometry, transition-key isolation, reduced-motion policy and existing Button handlers/focus/expanded semantics remain intact. This was source review, not runtime or visual acceptance.
- Verification after the source correction: **158 tests passed** with `CARGO_TARGET_DIR=/tmp/codex-native-target cargo test -p pi-desktop --offline -q`; matching offline Release build, focused rustfmt and `git diff --check` passed. Release SHA256: `0db50ddc4e52506374c19f28a68fcac028bab51331cc3dde6cd40263baf514f1`. The build retains the existing `block 0.1.6` future-incompatibility warning; dependencies and lockfile remain unchanged.
- The current macOS window probe returned `Screen is locked`; no current-build capture or interaction replay was obtained. Light/dark, compact, reduced-motion GUI replay, hover/focus states, exact meter interpolation and matched-reference pixels remain **PARTIAL**. No commit, push or deployment was performed.

## Workflow entrance and roster grid follow-up - 2026-10-30

Parent: the workflow roster settlement and disclosure motion follow-up and the full ZCode alignment objective.
Source: pinned ZCode `WorkflowTimeline.tsx`, `WorkflowRoll.tsx`, `WorkflowRunPhaseList.tsx`, and `wf-arrive`/`wf-unfold` rules in `styles.css`.
Reason: timeline and pane rows still appeared synchronously, while the pane roster needed the reference's shared two-column roll and row cadence.
Scope/risk/orchestration: medium-risk native single writer with one bounded read-only verifier; entrance presentation and roster layout only. Workflow ordering, disclosure state, handlers, focus ownership, persistence and backend semantics remain unchanged.

- Added a keyed `WorkflowEntrance` render wrapper. Timeline pills use the global 30ms ordinal delay, including the aggregate row; pane roster rows use 8ms per row capped at 400ms. The wrapper approximates `wf-arrive` from +4px and `wf-unfold` from -3px over 200ms with the reference cubic-bezier, while relative positioning preserves sibling layout. Reduced motion settles immediately and removes delay.
- Reshaped the expanded pane roster into one `grid-cols-2` frame with an 8px column gap and 2px top inset. Group headings span both columns at 22px height, use 4px left padding and 5px internal spacing, and add 4px only between groups. Existing attention ordering, source ordering, counts, node handlers and accessible disclosure semantics remain unchanged.
- Added deterministic coverage for delayed/reduced-motion timing, start offsets and stable layout dimensions, the two-column frame, heading spans and reference spacing. These are source/state contracts, not rendered interpolation, pointer replay or pixel parity.
- The independent read-only verifier found no new scoped blocker: mounted animation keys are stable and surface-scoped, reduced motion clears both timing components, timeline ordinals include the aggregate row, and the single roll grid preserves cross-group row delays. This is source review, not GUI lifecycle or interaction acceptance. The owner separately compiled and passed both new geometry tests as part of the 164-test run.
- Verification: **164 tests passed** with `CARGO_TARGET_DIR=/tmp/codex-native-target cargo test -p pi-desktop --offline -q`; matching offline Release build passed. Focused `rustfmt --edition 2021 --check` and `git diff --check` passed. Release binary SHA256: `bc127ee97e60209bc9acb891240ab1aa180aad12e16410ce4f476b1c3fa881bd`. The build retains the existing `block 0.1.6` future-incompatibility warning; dependencies and lockfile remain unchanged.
- The macOS window probe in this follow-up on `2026-10-30` returned `Screen is locked`. No current-build screenshot or interaction replay was obtained. Light/dark, compact, reduced-motion GUI replay, hover/focus states, exact entrance interpolation and matched-reference pixels remain **PARTIAL**. No commit, push or deployment was performed.

## Settings command hover token follow-up - 2026-10-30

Parent: the settings secondary action detail follow-up and the full ZCode alignment objective.
Source: pinned ZCode `button.tsx`, `select.tsx`, and `.theme-zai-light` / `.theme-zai-dark` tokens in `styles.css`.
Reason: GPUI-Kit's Ghost variant resolves hover through its accent palette, while ZCode settings commands use the neutral 5% hover surface in both themes.
Scope/risk/orchestration: low-risk native single writer; settings command color presentation only. Geometry, handlers, persistence, disabled behavior, focus ownership and save semantics remain unchanged.

- Added an explicit settings command variant with transparent idle, primary foreground, neutral hover/pressed surface (`hover` token) and native component focus treatment. Applied it to Windows shortcut installation, Skills “Use in task”, and the API-key visibility button. The global Ghost/navigation palette remains unchanged.
- This fixes the source-level color-token mismatch without inventing a new interaction model. GPUI-Kit's color state is still immediate rather than the reference CSS `transition-colors` timing, so rendered transition acceptance remains open.
- Verification: **164 tests passed** with `CARGO_TARGET_DIR=/tmp/codex-native-target cargo test -p pi-desktop --offline -q`; matching offline Release build passed. Focused rustfmt and `git diff --check` passed. Release binary SHA256: `e3714ca6eb2a0258cf3e20c8ecb5c01f640d757be6a434997ebdaef11d9aa250`. The existing `block 0.1.6` future-incompatibility warning remains; dependencies and lockfile are unchanged.
- The macOS window probe on `2026-10-30` returned `Screen is locked`. No current-build screenshot or interaction replay was obtained; light/dark, compact, pointer/keyboard focus, color-transition timing and matched-reference pixels remain **PARTIAL**. No commit, push or deployment was performed.

## Settings command color motion and tooltip follow-up - 2026-10-30

Parent: the settings command hover token follow-up and the full ZCode alignment objective.
Source: the user's save-button detail feedback, pinned ZCode `button.tsx` and its Tailwind color-transition defaults.
Reason: command colors still changed immediately, and the first transition implementation competed with the eye button's managed tooltip for GPUI's single hover listener.
Scope/risk/orchestration: medium-risk native single writer with one independent read-only verifier; settings save, shortcut-installation, skill-use and API-key visibility commands only. Button geometry, handlers, persistence, keyboard activation and native disabled semantics remain unchanged.

- Added keyed, interruptible background and foreground transitions over 150ms with cubic-bezier(0.4, 0, 0.2, 1). Save retains black/white ink and fades its primary fill to 80% alpha on hover. Neutral commands transition from transparent to the existing neutral hover token. Reduced motion and disabled state settle immediately; pointer tracking remains input-modality independent.
- The verifier identified a duplicate-hover registration in the initial implementation: the component's managed tooltip also calls `on_hover`. The eye button now keeps the styled Tooltip content through GPUI's separate native tooltip builder, preserving the localized accessibility label and fixed 24x24 geometry without a layout wrapper. The verifier confirmed the source-level conflict is closed with no new scoped finding. Native tooltip placement and event delivery remain unverified.
- Four deterministic color-policy tests cover light/dark paint, disabled targets, reference timing, reduced motion and bounded alpha interpolation. The final full desktop run passed **168 tests** with `CARGO_TARGET_DIR=/Volumes/Datas/tmp-codex-native-target TMPDIR=/Volumes/Datas/tmp-codex-native-build-temp cargo test -p pi-desktop --offline -q`. These tests do not render the tooltip or replay pointer/keyboard interactions.
- The matching offline Release build passed. Its binary is `/Volumes/Datas/tmp-codex-native-target/release/volt`, verified as a 33,127,056-byte arm64 Mach-O executable; SHA256 `28184d96ea6b56f685e5fae8b8602cfc5bff6e278204c9829b159717ce77d190`. This supersedes the failed system-disk build attempt, whose truncated output is not a valid artifact. The existing `block 0.1.6` future-incompatibility warning remains; dependencies and lockfile are unchanged.
- Focused rustfmt checks and `git diff --check` passed. The current `window_info.swift` probe returned `Screen is locked`; no final-build screenshot or interaction replay was obtained. Light/dark, 1200x800 and 480x640, maximum-font layouts, hover/pressed/focus behavior, save feedback, key visibility, actual color-transition timing and matched-reference pixels remain **PARTIAL**. The full objective remains active. No commit, push or deployment was performed.

## Settings command border geometry follow-up - 2026-10-30

Parent: the settings command color motion follow-up and full ZCode alignment objective.
Source: the user's save-button screenshot and the pinned `components/ui/button.tsx` base `border border-transparent` rule.
Reason: native Primary, Ghost and Custom buttons do not reserve a border by default, unlike the reference command frame.
Scope/risk/orchestration: low-risk native single writer; settings save, secondary commands and API-key visibility geometry only. Navigation, handlers, persistence and focus ownership remain unchanged.

- Added an explicit 1px transparent border to these command builders. Content-sized actions now include the reference border reservation; existing fixed heights, inline padding, radii and icon sizes remain unchanged. The eye control stays 24x24px.
- Added builder coverage for all four border edges and transparency across both languages, 12/14/20px interface sizes and both key-visibility states. This does not establish native rendering, focus-ring appearance or CSS `bg-clip-padding` pixel equivalence.
- Verification: **169 tests passed**; matching offline Release build, focused rustfmt and `git diff --check` passed. Binary: `/Volumes/Datas/tmp-codex-native-target/release/volt`, SHA256 `68b6bbcea5b2903cbbbf6a55e61ccc2b89b184084b7901f49dcab38984bbdcaf`. Whole-workspace format checking still reports the existing unrelated differences in `execution.rs`, `generative_ui.rs` and `panels.rs`; they were left unchanged. The dependency warning for `block 0.1.6` remains.
- The current macOS probe returned `Screen is locked`. No current-build screenshot or interaction replay was obtained; prior light captures are historical diagnostics only. Overall visual parity remains **PARTIAL** pending unlocked light/dark, compact, maximum-font, pointer/keyboard and matched-reference checks. No commit, push or deployment was performed.

## Settings navigation color motion follow-up - 2026-10-30

Parent: the settings command border geometry follow-up and full ZCode alignment objective.
Source: pinned ZCode `SettingsSidebarButton` and settings back-button classes, including `transition-colors`, `bg-surface`, `bg-surface-hover`, `text-remote-foreground-muted` and `text-foreground`.
Reason: settings navigation and Back used the correct idle/hover tokens but changed them immediately, unlike the reference color transition.
Scope/risk/orchestration: medium-risk native single writer; settings navigation and Back presentation only. Selection persistence, handlers, keyboard activation, focus semantics, tooltips and layout remain unchanged.

- Added keyed, interruptible 150ms transitions to navigation and Back backgrounds/foregrounds. Selected navigation remains emphasized after pointer exit; unselected rows fade to transparent when idle. Back transitions its idle surface and muted text together. Reduced motion and disabled states settle immediately.
- Reused the existing native tooltip builder for navigation rows so the color-motion hover listener does not compete with the managed tooltip listener. The eye button uses the same helper; no layout wrapper or duplicate hover registration was added.
- Added deterministic coverage for selected navigation, Back foreground/surface interpolation, disabled behavior and reduced-motion timing. This is source/state evidence, not rendered pointer, focus or tooltip placement evidence.
- One independent read-only reviewer confirmed tooltip/hover slot separation, unique stable navigation keys and preserved click/persistence/return-focus calls. The review was stopped before its complete selected/hover/focus override audit and is recorded as **incomplete**, not a clean review receipt.
- Verification: **171 tests passed**; matching offline Release build, focused rustfmt and `git diff --check` passed. Binary: `/Volumes/Datas/tmp-codex-native-target/release/volt`, SHA256 `2ee9a5c8c7aebaa28a9c932b69d2c0cdfc0372747dc378f2690ad9c8b003da73`. The existing `block 0.1.6` future-incompatibility warning remains.
- The current macOS probe returned `Screen is locked`. No current-build screenshot or interaction replay was obtained; light/dark, compact, hover/focus, tooltip placement and matched-reference pixels remain **PARTIAL**. No commit, push or deployment was performed.

## Settings disabled-command palette follow-up - 2026-10-30

Parent: the settings navigation color motion follow-up and full ZCode alignment objective.
Source: the user's button-detail feedback, pinned `button.tsx` `disabled:opacity-50`, and installed GPUI Component/Base 0.7.3 state resolution.
Reason: disabled commands inherited the library's muted foreground and 15% fill instead of retaining idle colors with whole-control opacity.
Scope/risk/orchestration: medium-risk native single writer with one bounded independent read-only verifier; settings command presentation only. Handlers, persistence, native disabled semantics, focus, keyboard activation, geometry and dependencies remain unchanged.

- Pin both instance background and foreground for enabled and disabled commands, and apply 50% opacity only while disabled. The component replays these instance colors after selected/disabled palettes. Disabled targets remain idle even while the pointer is inside, and disabled/reduced-motion transitions settle immediately. Removed the redundant neutral variant helper.
- Added deterministic builder coverage for all command kinds in both themes, checking whole-control opacity, retained colors, content-sized width, height, padding and transparent border reservation. This does not establish rendered compositing, input delivery or pixel parity.
- The independent verifier completed the previously incomplete scoped selected/disabled/focus override audit without a new blocker. Native Base disabled activation, keyboard/focus behavior and the separate tooltip/hover slots remain intact in source; this is not GUI acceptance.
- Verification: **172 tests passed** with `CARGO_TARGET_DIR=/tmp/codex-native-target cargo test -p pi-desktop --offline`; focused rustfmt and `git diff --check` passed. The matching offline Release build used `CARGO_TARGET_DIR=/Volumes/Datas/tmp-codex-native-target TMPDIR=/Volumes/Datas/tmp-codex-native-build-temp`; binary SHA256: `429bde77c757f2cc906b270583a78f5a9e208413800d97c980d041b210ae5064`. The existing `block 0.1.6` warning and unrelated whole-workspace formatting differences in `execution.rs`, `generative_ui.rs` and `panels.rs` remain unchanged.
- The window-list helper initially returned a window, but the attempted capture failed with `could not create image from window`; a direct `CGSSessionScreenIsLocked` probe returned `1`, including the final probe. No screenshot was obtained. The isolated temporary-state Release process was stopped; no real configuration or existing application process was changed. Light/dark, compact, maximum-font, hover/pressed/focus, tooltip placement and matched-reference pixel acceptance remain **PARTIAL**. No commit, push or deployment was performed.

## Settings input border feedback follow-up - 2026-10-30

Parent: the settings disabled-command palette follow-up and full ZCode alignment objective.
Source: the user's settings/button detail feedback, explicit continuation, and pinned ZCode `components/ui/input.tsx` plus Zai input tokens at `3c5b227649371507ddc4eb46ac5133afe6db1cab`.
Reason: settings text inputs lacked the reference hover-border transition and still inherited the component's additional focus ring.
Scope/risk/orchestration: medium-risk native single writer with one bounded independent read-only verifier; provider and interface-font text-input presentation only. The full objective, native editor entities, callbacks, masking, IME, persistence, command behavior and dependencies remain unchanged.

- Added a keyed `SettingsField` observation shell around each existing native Input. Pointer hover or native editor focus targets the existing 15% neutral border; idle uses 10%. Border transitions use the reference 150ms cubic-bezier(0.4, 0, 0.2, 1), with zero duration under reduced motion. The input keeps its existing fill and uses this border instead of the library's extra focused border/ring.
- The shell adds no frame, padding, background, fixed height or tab stop. Original 28/32/36px inputs, the separate API-key eye button and px suffix keep their existing geometry. Select triggers and their menus remain outside this change.
- Added three deterministic tests covering light/dark hover/focus targets, timing/reduced motion, bounded alpha interpolation and the unframed shrinkable shell. These tests do not render the controls or exercise native pointer, focus traversal, selection, masking or IME.
- Verification: **175 tests passed** with `CARGO_TARGET_DIR=/Volumes/Datas/tmp-codex-native-target TMPDIR=/Volumes/Datas/tmp-codex-native-build-temp cargo test -p pi-desktop --offline -q`; matching offline Release build, focused rustfmt and `git diff --check` passed. Binary: `/Volumes/Datas/tmp-codex-native-target/release/volt`, arm64 Mach-O, SHA256 `62e9f55f3d0386df2139450235762867b7da252372ac3975e985e36ce813b1e5`. The existing `block 0.1.6` future-incompatibility warning remains.
- Independent source review completed with no scoped blocker. It confirmed keyed focus subscriptions and one hover listener, preserved native editor/IME/masking/callback paths, replacement border precedence after disabling the library focus ring, unchanged 28/32/36px input frames, and unchanged save-button content sizing and activation semantics. This is source review only. A direct `CGSSessionScreenIsLocked` probe returned `1` on 2026-10-30; no GUI process was launched or new screenshot obtained. Light/dark, compact, maximum-font, pointer/keyboard focus, actual border interpolation, eye/suffix hit testing and matched-reference pixels remain **PARTIAL**. Prior captures are historical diagnostics, not acceptance for this binary. No commit, push or deployment was performed.

## Settings Select hover and state ownership follow-up - 2026-10-30

Parent: the settings input border feedback follow-up and full ZCode alignment objective.
Source: the user's explicit continuation, pinned ZCode `components/ui/select.tsx` / Zai tokens, and installed GPUI Component 0.7.3 Select rendering and event delivery.
Reason: settings Select triggers lacked neutral hover-border feedback; adding independent hover state exposed existing reuse of one native SelectState at two simultaneously rendered locations. That state holds only one style, popup bounds, focus and open state.
Scope/risk/orchestration: medium-risk native single writer with the same bounded independent read-only verifier; settings Select presentation and per-trigger ownership plus language/theme value synchronization. The full visual objective and native menu, keyboard/activation, persistence and save semantics remain unchanged.

- Added a keyed, non-focusable 32px shell with one input-modality-independent hover listener. Hover transitions the neutral border from 10% to 15% over 150ms with cubic-bezier(0.4, 0, 0.2, 1); reduced motion settles immediately. The original Select retains focused/open border precedence. Its focus changes remain immediate, not reference-transition acceptance.
- Each General/Appearance/footer trigger now owns an independent native Entity. Existing language handlers synchronize both language values and refresh both theme menus' localized labels; theme confirmation synchronizes both theme values. This prevents style, focus, popup bounds and open state from being shared between different triggers. No custom menu engine or activation path was introduced.
- Content shells use explicit full width; the two footer shells use zero-basis flex and a zero minimum width. The shell adds no padding, paint, border or overflow clipping. Four tests cover exact light/dark hover colors, timing/reduced motion and bounded interpolation, stable unpainted geometry and footer/content width contracts. They do not exercise native layout or menu input.
- Independent source review confirmed distinct state ownership and synchronization, queued Confirm event delivery without source state/list reentry, and no recursive Confirm emission from `set_selected_index`. Native popup positioning, keyboard focus restoration and actual rendered states remain unverified.
- Final verification: **179 tests passed** with `CARGO_TARGET_DIR=/Volumes/Datas/tmp-codex-native-target TMPDIR=/Volumes/Datas/tmp-codex-native-build-temp cargo test -p pi-desktop --offline -q`; matching offline Release build passed, focused rustfmt and `git diff --check` passed. Binary: `/Volumes/Datas/tmp-codex-native-target/release/volt`, arm64 Mach-O, SHA256 `5807846140aec74f444b0910f61556bcf9ede4792fee86a5d12d671be4a8802f`. The existing `block 0.1.6` future-incompatibility warning remains. The direct screen-lock probe returned `1`; no native GUI process was launched or new screenshot obtained. Focus-transition timing, caret sizing/color, light/dark and compact/maximum-font layout, menu anchoring, pointer/keyboard and matched-reference acceptance remain **PARTIAL**. No commit, push or deployment was performed.

## Settings screenshot recheck and acceptance blocker - 2026-10-30

Parent: the settings Select ownership follow-up and full ZCode alignment objective.
Source: the user's attached Save-button screenshot and explicit request to continue comparing details.
Reason: recheck the accumulated button corrections before making further unobserved presentation changes.
Scope/risk/orchestration: low-risk native owner; source inspection and deterministic verification only. No application code, dependencies, configuration, or running application state was changed in this recheck.

- Reconfirmed the pinned reference commit and Save's existing content-sized 32px frame, 8px radius, transparent border, 10px horizontal padding, centered 16px Check/label content, 4px gap and themed hover/disabled policy. These are implementation contracts, not a new rendered acceptance receipt.
- The reference Select caret is explicitly 14px and subtle. The installed native default caret is 16px; its icon override forces 12px and muted ink. Changing the Select size would also change its list delegate size. No caret workaround was introduced; that difference remains open.
- Re-ran all **179 desktop tests**, focused settings-control rustfmt checks and `git diff --check` successfully. The matching offline Release build passed; `/Volumes/Datas/tmp-codex-native-target/release/volt` retains SHA256 `5807846140aec74f444b0910f61556bcf9ede4792fee86a5d12d671be4a8802f`. The existing `block 0.1.6` future-incompatibility warning remains.
- The direct `CGSSessionScreenIsLocked` probe again returned `1`. This repeats the same acceptance blocker across consecutive follow-ups. No current-build screenshot or pointer/keyboard replay was obtained. Further visual acceptance requires an unlocked desktop; the full objective remains **PARTIAL**, with goal execution **blocked** at this boundary rather than reported complete.
- After unlock, compare current-build light/dark screenshots at 1200x800 and 480x640, including 20px interface text, Save hover/focus/feedback, key visibility, and Select menu anchoring. Previously documented source differences and broader workflow/Markdown gaps remain open; this recheck does not reduce the full objective or add unrelated tooling work. No commit, push or deployment was performed.

## Settings Save button semantic variant follow-up - 2026-10-30

Parent: the settings screenshot recheck and acceptance blocker.
Source: the attached Save-button screenshot, pinned ZCode `button.tsx` large variant, and the installed GPUI Component 0.7.3 Button size semantics.
Reason: the Save button already matched the visible 32px frame and content sizing, but it used the native `small` variant as its base and then overrode the frame. Align the underlying size choice with the reference large variant. Its custom child already owns label/icon sizing, so this does not establish a visible correction or a focus-treatment difference.
Scope/risk/orchestration: low-risk native single writer; Save presentation semantics only. Click handling, persistence, feedback, disabled behavior, hover motion, geometry overrides, and dependencies remain unchanged.

- Changed the Save command to use the native large Button variant while preserving the explicit 32px height, 8px radius, 10px inline padding, transparent border reservation, Check glyph, 4px gap and interface-size label.
- Existing localized builder coverage still checks its geometry; updated the design contract. This verifies source geometry only; it does not establish rendered pixels or pointer/focus behavior.

## Settings Select indicator follow-up - 2026-10-30

Parent: the Save semantic variant follow-up and full ZCode alignment objective.
Source: explicit continuation of the user's settings-detail comparison and pinned ZCode `select.tsx` default `size-3.5` indicator.
Reason: the native default caret is 16px, while changing Select size to obtain a 14px caret would also change its menu delegate. Custom icons are adapted to 12px by the installed component.
Scope/risk/orchestration: low-risk native single writer; settings Select indicator constraints only. Trigger frames, native state ownership, menu sizing, handlers, focus, persistence and dependencies remain unchanged.

- Supply the existing ChevronDown asset with 14px minimum/maximum constraints on both axes, which survive the component's custom-icon xsmall adaptation. This leaves the Select's own size and native popup path intact. One focused builder test checks those constraints and the absence of extra paint/padding.
- Confirmed that `sync_component_theme` already maps `muted_foreground` directly to the reference `foreground_subtle`, so no palette change is needed. Rendered indicator bounds and menu interaction still require current-binary verification.
- Verification: **180 desktop tests passed**; matching offline Release build, focused `rustfmt --edition 2021 --check`, and `git diff --check` passed. Final binary: `/Volumes/Datas/tmp-codex-native-target/release/volt`, SHA256 `2794a5689d2ba4435005c06c47f9f11f90ce6a96a6f622ff42f1dd0cabbad599`. The existing `block 0.1.6` future-incompatibility warning remains. An initial format command incorrectly used edition 2024; its import-order differences were not applied, and the correct project-edition check passed.
- Capture is available again; a missing screen-lock dictionary key returned `unknown`, not confirmation of a locked desktop. The isolated pre-change binary (`580784...`) rendered General at 1200x800 in light mode. Its accessibility tree measured the Save button at **98x32px**, and the screenshot showed centered icon/label without the old trailing gap. Capture: `/var/folders/h7/9t4_q7bd6p9489dhdy_bdt400000gn/T/codex-shot-2026-10-30_08-12-39.png`. This is baseline layout evidence, not acceptance of the final indicator change.
- The final binary was launched with a separate temporary state containing only a local test endpoint, fixture model, dark theme and 20px interface preference. Its dark startup screen rendered, but Settings interaction was not completed: System Events returned `osascript is not allowed assistive access` (`-25211`), and the window bounds changed between probes. Stopped coordinate automation rather than applying further uncertain pointer input. Startup capture: `/var/folders/h7/9t4_q7bd6p9489dhdy_bdt400000gn/T/codex-shot-2026-10-30_08-26-23.png`. Final Settings light/dark, 480x640, maximum-font, Save hover/focus/feedback, eye toggle, rendered 14px indicator, menu anchoring and matched-reference acceptance remain **PARTIAL**. No commit, push or deployment was performed.

## Settings narrow-window row follow-up - 2026-10-30

Parent: the settings Select indicator follow-up and full ZCode alignment objective.
Source: the user's explicit continuation and button-detail screenshot, the native 480x640 minimum-window contract, and pinned `settings/SettingsPageParts.tsx`.
Reason: a fixed 192px control column leaves little room for labels/descriptions in narrow native windows, especially at 20px interface text.
Scope/risk/orchestration: low-risk native single writer; General, Appearance, capabilities and Subagents row layout only. Provider fields already use full-width vertical layout. Save geometry, control entities, menus, callbacks, persistence and dependencies remain unchanged.

- Rows now stack label/description above the full-width control frame below a 640px viewport width. Wide rows keep the 192px column. Both modes retain 16px gaps and 16px/12px padding, with no fixed row height. Select fills the narrow control row; fixed font/switch controls remain right-aligned.
- Reference clarification: the pinned default SettingsRow itself always uses `minmax(0,1fr) + 192px`; only its optional wide variant has a single-column breakpoint. This change is an explicit native minimum-window adaptation, not an asserted default-reference breakpoint.
- Added two deterministic builder tests for row direction, alignment, natural height, padding/gaps and control-column width. **182 desktop tests passed** with `CARGO_TARGET_DIR=/Volumes/Datas/tmp-codex-native-target TMPDIR=/Volumes/Datas/tmp-codex-native-build-temp cargo test -p pi-desktop --offline -q`. The matching offline Release build passed. Binary: `/Volumes/Datas/tmp-codex-native-target/release/volt`, SHA256 `2dec9fade5b271aca0cb683bc9d8a644dd11f5c32d352040952d2d9e0937d671`. Focused Rust 2021 rustfmt and `git diff --check` passed. The existing `block 0.1.6` warning and unrelated format differences in `execution.rs`, `generative_ui.rs` and `panels.rs` remain unchanged.
- GUI acceptance is **PARTIAL**: Screen Recording and Accessibility permission probes passed, but final-build window capture returned `could not create image from window`; the identity/geometry helper explicitly returned `Screen is locked`. No current-build screenshot, narrow resize, maximum-font layout, Save interaction or Select menu acceptance was obtained. The isolated dark/English/20px fixture process was stopped; real application state and existing processes were not changed. No commit, push or deployment was performed.

## Assistant Markdown table overflow follow-up - 2026-10-30

Parent: the ongoing ZCode alignment objective and the prior Markdown table/source comparison.
Source: pinned ZCode `markdown-table.tsx` and installed GPUI Base `TextView` table layout.
Reason: native Markdown tables used the adaptive wrapping path without horizontal overflow, so a column wider than the transcript could become clipped or unreachable instead of remaining scrollable like the reference table frame.
Scope/risk/orchestration: medium-risk native single writer with one bounded independent read-only verifier; assistant Markdown table frame presentation only. Markdown parsing, selectable text, table colors, cell padding, heading treatment and transcript scroll ownership remain unchanged.

- Added a dedicated assistant table-frame style with the existing 12px radius and chat-surface background, opting into GPUI Base's native horizontal-scroll layout. The renderer keeps its measured adaptive columns and wrapping behavior; it does not force `nowrap`, alter vertical overflow, or add a custom scrollbar/menu implementation.
- Added one deterministic style test for light/dark background, radius, horizontal overflow, unconstrained frame dimensions and preserved wrapping. This verifies the style contract, not actual column measurement, scrollbar painting, wheel bubbling, selection or corner pixels.
- Independent-review correction: the bounded read-only verifier was closed while still running and returned no review conclusion. This follow-up has no completed independent-review receipt; the source tests/build below do not replace it.
- Verification: **183 desktop tests passed** with `CARGO_TARGET_DIR=/Volumes/Datas/tmp-codex-native-target TMPDIR=/Volumes/Datas/tmp-codex-native-build-temp cargo test -p pi-desktop --offline -q`; focused Rust 2021 rustfmt and `git diff --check` passed. The matching offline Release build passed. Binary: `/Volumes/Datas/tmp-codex-native-target/release/volt`, arm64 Mach-O, SHA256 `85b9cb95d4ec83e06069f0867aae484afec4c2b51be39a8b64aa277d636df305`. The existing `block 0.1.6` warning remains; no dependency or lockfile changes were made.
- GUI acceptance remains **PARTIAL**. The current `window_info.swift` probe returned `Screen is locked`; no final-build Markdown table capture or wheel/selection replay was obtained. The reference's edge shadows, virtual scrollbar, toolbar actions and preview dialog remain outside this narrow follow-up. No commit, push or deployment was performed.

## Settings Save feedback layout follow-up - 2026-10-30

Parent: the settings Save button detail follow-ups and full ZCode alignment objective.
Source: the user's attached button screenshot, explicit continuation, and reinspection of the pinned `button.tsx` large-button geometry.
Reason: the Save frame is already content-sized, but its success text was a separate child of the page's 32px-gap column, visually disconnecting feedback from the action.
Scope/risk/orchestration: low-risk native single writer, external=0; Save action/feedback layout only. The complete parity objective remains unchanged. Existing button dimensions, Check glyph, localized labels, click handler, native focus/activation, errors, persistence and dependencies remain unchanged.

- Place the native command and existing success text together in an unframed, full-width wrapping row: 32px minimum height, centered cross-axis alignment, 12px horizontal gap and 8px wrap gap. The button keeps its content width and fixed 32px height. Feedback retains the interface font and success color, uses 1.5 line height, and can wrap naturally at narrow widths or 20px text instead of forcing horizontal overflow. A wrapped success state can still increase the row height.
- This is a native feedback-layout adaptation, not a claim that the reference settings page has the same global Save action. No speculative icon replacement, fixed label width, new state machine or toast implementation was introduced.
- Added two deterministic builder tests for row wrapping/unframed geometry and localized feedback typography in both themes at 12/14/20px. They do not establish rendered text measurement, actual success/error activation or pixel parity.
- Verification is **PARTIAL**, not a test/build pass. Focused Rust 2021 rustfmt and `git diff --check` passed. The offline desktop test command was interrupted after approximately ten minutes stalled in dependency metadata loading; the subsequent `CARGO_INCREMENTAL=0 cargo check -p pi-desktop --tests --offline` advanced through native dependencies but was also interrupted after exceeding the bounded verification window. Both returned 130, not a source-error or test-failure receipt. No desktop tests completed for this change, and no matching Release binary was built; the preceding 183-test result and binary hash belong to the previous table change. Confirmed that no Cargo or rustc processes remained after interruption.
- The macOS window probe explicitly returned `Screen is locked`; no final-build GUI capture or interaction replay was attempted. Rendered button/feedback alignment, narrow wrapping, maximum-font layout, hover/focus and actual save feedback remain unverified. Full visual acceptance remains **PARTIAL**. No commit, push or deployment was performed.

## Current HEAD verification checkpoint - 2026-11-14

Parent: the ongoing ZCode visual-parity objective and the settings Save feedback follow-up.
Source: current `pi-rust` HEAD `72e3afb66b9b611a56f1a55f033674435edef891`, current native source, and the existing visual-parity capture harness.
Reason: revalidate the accumulated native UI work after the previous interrupted verification and distinguish current-build evidence from historical captures.
Scope/risk/orchestration: read-only verification checkpoint; no application code, dependencies, configuration, user state or remote state was changed.

- The current HEAD test suite passed **190 desktop tests** with `CARGO_TARGET_DIR=/Volumes/Datas/tmp-codex-native-target TMPDIR=/Volumes/Datas/tmp-codex-native-build-temp cargo test -p pi-desktop --offline -q`. The matching offline Release build passed with the existing `block 0.1.6` future-incompatibility warning. Current binary: `/Volumes/Datas/tmp-codex-native-target/release/volt`, arm64 Mach-O, SHA256 `bbaa87e4797d99a6072416d096f90ea7b7df8e978bf5988f66d8f3a30bdc2c1b`.
- Focused Rust 2021 rustfmt checks for the settings module and `git diff --check` passed. The worktree was clean at the start of this checkpoint; unrelated uncommitted changes appeared afterward across Cargo, `pi-agent-core`, and desktop modules and were preserved. No commit, push or deployment was performed.
- An isolated `VOLT_STATE_DIR` launch of the current Release binary produced a visible 1200x800 light Chinese startup window. Candidate capture metadata was written under `/tmp/codex-native-capture-20261114-settings`; the image was visually inspected as the empty workbench, not Settings. The capture harness therefore remains **UNREVIEWED** for the declared Settings fixture and is not evidence for Save-button or Settings-page parity.
- macOS window capture is available in this session, but native keyboard/mouse navigation did not change the startup view and the GPUI accessibility tree exposed a recursive `AXApplication` structure. No reliable Settings-page, Save hover/focus/feedback, narrow layout or matched-reference interaction evidence was claimed. Overall visual acceptance remains **PARTIAL**.

## Settings action typography follow-up - 2026-11-14

Parent: the ongoing ZCode parity objective and settings button-detail follow-ups.
Source: pinned ZCode `3.16.2` commit `3c5b227649371507ddc4eb46ac5133afe6db1cab`, specifically `button.tsx`, `lib/utils.ts`, and the stylesheet/preflight typography contract.
Reason: native action content used a fixed 20px line height, while the reference default and large button classes resolve to interface-size text inheriting a relative 1.5 line height.
Scope/risk/orchestration: low-risk native single writer, external=0; settings action typography, focused test coverage and documentation only. Existing unrelated worktree changes are preserved.

- Save and secondary action content now uses relative 1.5 line height. Button frames, padding, radii, icons, gaps, hover behavior, handlers and persistence are unchanged. Navigation content retains its existing 20px line height.
- Expanded the existing action-content test across every integer interface font size from 12px through 20px and Chinese/English primary and secondary labels. It checks natural dimensions, centering, gap, normal weight, relative line height and nowrap; it does not measure rendered glyphs or establish clipping acceptance.
- Current dirty-worktree verification passed: **43 focused settings tests**, **203 desktop tests**, the matching offline locked Release build, focused Rust 2021 rustfmt and `git diff --check`. The existing `block 0.1.6` future-incompatibility warning remains. Binary: `/Volumes/Datas/tmp-codex-native-target/release/volt`, SHA256 `533c8b961ee1d40e42959a0e24141059e95587fbe1d89b4889d775f7b7aa0178`. These results supersede the preceding clean-HEAD binary identity for the current worktree, not its historical capture evidence.
- Native preview returned that the Mac is locked and automatic unlock failed. No current-build GUI screenshot, hover/focus replay or resize acceptance was obtained. Historical startup captures do not validate this change. Rendered alignment, clipping, maximum-font and narrow-window behavior remain **PARTIAL** pending an unlocked desktop. No commit, push or deployment was performed.

## File-summary action detail follow-up - 2026-11-14

Parent: the ongoing ZCode parity objective and settings action typography follow-up.
Source: pinned ZCode commit `3c5b227649371507ddc4eb46ac5133afe6db1cab`, `v4/ConversationFileSummaryPanel.tsx`, shared `components/ui/button.tsx` and interface typography tokens.
Reason: the native file-summary Rewind action incorrectly reused a 28px frame and 14px icon; both summary actions also inherited the library's small-button text instead of the reference interface text.
Scope/risk/orchestration: low-risk native single writer, external=0; file-summary action builders and their integration only. Receipt-derived file counts, disclosure state, review routing, unavailable rewind semantics and unrelated dirty changes are preserved.

- Rewind now uses the reference small-button geometry: 24px height, 6px radius, 8px inline padding, 12px Undo2 glyph and 4px content gap. Its interface-size text uses the small variant's relaxed 1.625 line height. It stays disabled; primary foreground and transparent fill are pinned before whole-control 50% opacity, avoiding the library's replacement disabled palette.
- Review retains 28px height and 8px radius, with explicit 8px inline padding, input surface, interface-size normal text and relative 1.5 line height. The existing file-specific accessible name, path tooltip and actual diff-opening handler remain on the same native Button; no placeholder review or rewind behavior was added.
- Added three builder tests covering Chinese/English labels, light/dark paint and every integer interface font size from 12px through 20px. The initial focused run exposed two assertions against deferred component radius fields; builders now write radius through `Styled` explicitly. The corrected complete desktop suite passed **206 tests**. Focused Rust 2021 rustfmt, `git diff --check` and the offline locked Release build passed. Binary: `/Volumes/Datas/tmp-codex-native-target/release/volt`, SHA256 `6b0bc9e142c0999a32af3d28038fb3a8c27835d621e50903814a68ab882d8a0b`. The existing `block 0.1.6` future-incompatibility warning remains.
- The current native-app inventory again reported that the Mac is locked and automatic unlock failed. No current-build screenshot or pointer/keyboard replay was obtained. Actual glyph bounds, maximum-font clipping, hover/focus, narrow-window composition and matched-reference acceptance remain **PARTIAL**. Full-product scope is unchanged. Other parallel worktree edits were preserved; no commit, push or deployment was performed.

## File-summary row and disclosure follow-up - 2026-11-14

Parent: the file-summary action detail follow-up and ongoing full ZCode parity objective.
Source: pinned ZCode commit `3c5b227649371507ddc4eb46ac5133afe6db1cab`, `v4/ConversationFileSummaryPanel.tsx`, and the installed native renderer/control APIs.
Reason: file-summary rows still forced 36px height and oversized horizontal padding; their path color, numeric font and disclosure glyph did not follow the reference presentation.
Scope/risk/orchestration: medium-risk native single writer with one independent read-only verifier; file-summary layout/paint/motion only. Receipt-derived file identities/counts, disclosure state, real Review routing, unavailable Rewind state, focus ownership and dependencies remain unchanged.

- Rows now have natural height, 8px padding on each side and a 4px gap before Review. Their separate background/50 wrapper remains behind the hover/30 surface; the file-label cluster is shrinkable with a 6px gap and subtlest path/icon color. Review and the text/count cluster remain children of the same horizontal row. Counts retain the interface font with OpenType `tnum`; per-file counts have an 8px gap, and unknown values are not fabricated.
- Header labels have an explicit shrinkable minimum and 1.5 line height. A single 14px ChevronRight uses subtlest ink and rotates 90 degrees over the reference 150ms easing, reusing existing scoped motion and reduced-motion handling. Reinspection of installed `gpui-pre 0.3.9` confirmed focused Div click listeners already receive Enter/Space activation on key release; no duplicate keyboard handler was introduced. This is source evidence, not a GUI key-replay receipt.
- Added four builder tests for natural row/surface dimensions, padding/gaps, 12-20px typography, background-only alpha and tabular digits without a forced font family. **210 desktop tests passed**, as did the offline locked Release build, focused Rust 2021 rustfmt and `git diff --check`. Binary: `/Volumes/Datas/tmp-codex-native-target/release/volt`, SHA256 `b182ffa921e5242ff69d49e7f751957fc453356d3073ac8591a0ed42558aa680`. The existing `block 0.1.6` future-incompatibility warning remains.
- The bounded independent verifier completed with no scoped source blocker, checking child ownership, flex sizing, background-only opacity, motion/state IDs and unchanged handlers/focus. Its review explicitly does not establish inherited glyph rendering, hover composition, animation timing or event dispatch.
- The Markdown audit confirmed that installed `gpui-base 0.7.3` exposes shared muted/border/paragraph roles, but no independent list, blockquote or horizontal-rule style refinements. Those earlier reference gaps remain open; no shared-text retint, custom replacement parser or dependency change was introduced as a substitute.
- Native-app inventory again reported a locked Mac with failed automatic unlock. No current-build screenshot, hover/focus/resize replay or matched-reference capture was obtained. Full visual and interaction acceptance remains **PARTIAL** pending an unlocked desktop; full-product scope is unchanged. Other dirty work was preserved. No commit, push or deployment was performed.

## Native Settings interaction checkpoint - 2026-11-14

Parent: the settings action typography and file-summary follow-ups, within the unchanged full ZCode parity objective.
Source: explicit continuation of the user's button-detail request and the current Release binary.
Reason: native UI access became available; the previous locked-desktop observations no longer describe this checkpoint.
Scope/risk/orchestration: local diagnostic verification, native single owner, external=0. No application source, dependencies, real preferences, credentials or remote state was changed in this checkpoint.

- Wrapped the existing Release executable in an ephemeral `/tmp/codex-native-ui-20261114.UYqkmi/VoltParity.app` bundle to obtain a usable native accessibility tree. The preview used a separate `VOLT_STATE_DIR`, unset provider keys and a loopback-only test endpoint. Rechecked the binary SHA256: `b182ffa921e5242ff69d49e7f751957fc453356d3073ac8591a0ed42558aa680`.
- Reached the actual General, Appearance and Model providers pages using their native controls. Unlike historical startup captures, the observed frames contain the relevant Settings actions. At 14px in light Chinese, Save is content-sized with centered icon/text, and clicking it visibly displays success feedback beside the button.
- Entered 20px in the Appearance field and committed with Enter; both the accessibility value and rendered text changed. Switched the theme through the native menu to dark and observed the matching selected values and paint. Save's Chinese label and icon remain contained at this size; clicking Save produces adjacent visible feedback. The dark English General frame also shows an unclipped `Save Preferences` label and adjacent `Settings Updated` text.
- At the observed intermediate window size (1902x1440 screenshot pixels), navigation switches to compact icons and the settings controls remain accessible. This is not a 480x640 receipt: resize attempts did not establish that exact viewport, and one returned `AXError.cannotComplete`. Some earlier frames had an uncaptured white right-hand region; they are not full-frame layout acceptance.
- Tested the eye control using only a non-secret local fixture string. Show exposes the fixture and changes the accessible button name to Hide; Hide restores masking, removes the value from the accessibility tree and restores the Show label. No model request or real credential was used.
- Screenshots were inspected inline through the native UI tool, not exported as local PNG artifacts. These are native diagnostic observations, not identity-bound matched ZCode/native captures. Tab did not establish a visible focus/keyboard-activation receipt. Exact minimum-window wrapping, focused activation, complete hover/motion checks and matched-reference visual acceptance remain **PARTIAL**. The isolated preview process was stopped; no commit, push or deployment was performed.
- Checkpoint verification: focused `rustfmt --edition 2021 --check` for settings/file-summary button and row builders passed, and `git diff --check` passed. The fresh `cargo test -p pi-desktop --offline --locked -q` attempt produced no output for approximately four minutes and was interrupted with SIGINT (exit 130); it is not a new test pass or a source-failure receipt. No Cargo/rustc child remained from this attempt. The preceding 210-test and matching Release-build receipts belong to the unchanged application-source follow-up above; this checkpoint adds GUI observations and documentation only.

## Settings field typography follow-up - 2026-11-14

Parent: the settings action typography follow-up and unchanged full ZCode parity objective.
Source: the user's continued button-detail request and pinned ZCode commit `3c5b227649371507ddc4eb46ac5133afe6db1cab`, shared large Input/Select classes, provider labels and interface text tokens.
Reason: adjacent settings inputs and provider labels still forced 20px line height while the reference uses interface text with inherited relative 1.5 line height.
Scope/risk/orchestration: low-risk native single writer, external=0; shared settings field typography, focused builder tests and a development-only reference control fixture. Existing frame heights, padding, colors, menus, callbacks, persistence and unrelated AgentConfig/worktree edits are preserved.

- Shared settings Input/Select styles now use relative 1.5 line height. Provider labels use the same relative line height without fixing their frame dimensions or preventing wrapping. The reference's explicitly 24px settings-row descriptions are unchanged.
- Expanded input/Select builder coverage to every integer interface size from 12px through 20px in both themes. Added provider-label checks for Chinese/English text, natural dimensions and relative typography. These are style-contract tests, not rendered glyph/clipping acceptance.
- Added `/controls.html` to the existing pinned-component reference harness, with light/dark, Chinese/English and 12-20px query settings. It imports the reference Button/Input/Select implementations; both fixture pages are explicit Vite build entries. Save/grouping are diagnostic fixtures, not the original Settings page, and non-Select commands have no product handlers. This harness is not bundled into the native app.
- Reference source was restored from the pinned codeload archive, SHA256 `48b28621a45735452d5629f5b6a57393826da1fdf5cb5d12e7cf3f068defb072`; its LICENSE hash matches the existing manifest. The extracted archive is not a Git checkout and was not presented as a successful `prepare.py` Git identity check. Reference dependency installation failed first with `ENOSPC`; a second attempt on the data volume was stopped with SIGINT (130) under resource pressure. No new reference browser render or Vite build has passed.
- Focused Rust 2021 rustfmt, `git diff --check`, Python syntax compilation of `prepare.py`, and a Bun syntax/transpile smoke check of `controls.tsx` plus `vite.config.ts` passed. The Bun check externalized every import; it does not validate dependency resolution, Tailwind output or browser rendering. The focused `cargo test -p pi-desktop settings:: --offline --locked -q` progressed through core compilation to the desktop test target, emitted an unrelated unused-variable warning in `settings/agentconfig.rs`, then remained unfinished after more than 20 minutes. It was stopped with SIGINT (130); all owned Cargo/rustc/install/profiler processes were confirmed exited. This is neither a test pass nor a source-failure receipt. No matching Release build or current-change native GUI acceptance is claimed. The preceding 210-test suite and native Settings observations belong to the previous binary. Full visual and interaction acceptance remains **PARTIAL**; no commit, push or deployment was performed.

## Rendered control reference checkpoint - 2026-11-14

Parent: the settings field typography follow-up and unchanged full ZCode parity objective.
Source: explicit continuation of the user's button-detail request, the pinned source archive and its actual shared Button/Input/Select renderers.
Reason: obtain browser-rendered control evidence and finish deterministic verification after the preceding interrupted installation/test attempts.
Scope/risk/orchestration: local diagnostic verification, native single owner, external=0. No additional application source, dependencies, real preferences, credentials or remote state was changed in this checkpoint.

- Installed a separate temporary reference runtime with direct versions from the pinned lockfile. The control-only Vite build passed with 1,849 transformed modules. This supersedes the preceding failed reference-install attempt for this diagnostic page only; it is not a frozen installation or build of the entire reference workspace. The extracted archive is still not a Git checkout and does not satisfy `prepare.py`'s Git identity gate.
- Measured actual light Chinese 14px and dark English 20px controls, primary hover, narrow fixture wrapping and the three-option menu. Save remains 32px high, with an 8px radius, 10px inline padding and a 16px icon; interface line height resolves to 21px/30px. At 20px, the reference menu options have 40.5px height, relaxed 32.5px line height and 2px row gaps. The fixture is not the original Settings page and does not prove native menu/layout parity.
- Saved three 1280x720 reference JPEGs, source hashes, screenshot hashes, rendered measurements and interaction boundaries in the locally ignored `docs/visual-parity/runs/zcode-3.16.2/control-reference-20261114/README.md`. Browser viewport overrides were reset. Reference screenshots are not Volt screenshots. The user's original temporary clipboard attachment is no longer present; no inspection or identification of that attachment is claimed.
- **44 focused settings tests passed.** The initial complete desktop run finished with 214 passes and two Review Git deadline failures. Running the same current test executable's five Review tests serially passed, including both failures; rerunning the complete executable with `--test-threads=1 --quiet` then passed **216 tests**, zero failures, in 11.34 seconds. The initial parallel failure remains part of the record; the serial pass does not prove its cause or change the 15-second Git deadline. Focused Rust 2021 rustfmt and `git diff --check` also passed.
- The matching offline locked Release build did not complete after approximately 20 minutes. A bounded compiler sample showed dependency-metadata loading; no compile error was emitted. The build was stopped with SIGINT (130), which is neither a build pass nor a source-failure receipt. Cargo, its rustc child and the local reference preview were confirmed exited. No replacement build was launched.
- No new native screenshot or interaction receipt was obtained for the field-typography change. The earlier native Settings checkpoint belongs to the previous binary. Current-build glyph/clipping checks, exact native minimum-window behavior, visible keyboard focus/activation, complete hover/motion and matched-reference screenshots remain **PARTIAL**. Full-product scope is unchanged; unrelated dirty work is preserved. No commit, push or deployment was performed.

## Settings inherited line-height follow-up - 2026-11-14

Parent: the settings field typography follow-up and rendered control reference checkpoint, within the unchanged full ZCode parity objective.
Source: the user's continued control-detail comparison, pinned ZCode commit `3c5b227649371507ddc4eb46ac5133afe6db1cab`, its actual Tailwind preflight and SettingsRow renderers, and installed GPUI typography inheritance.
Reason: setting only the interface font left ordinary settings text inheriting GPUI's default golden-ratio line height instead of the reference's relative 1.5.
Scope/risk/orchestration: medium-risk native single writer with one bounded independent read-only verifier; settings frame typography and its existing builder test only. Explicit title, navigation, description and control overrides, native state ownership, persistence, dependencies and unrelated dirty work are preserved.

- The Settings frame now supplies relative 1.5 line height alongside its existing interface font. The existing frame test covers every integer size from 12px through 20px in both themes, retaining the frame-geometry checks. This is not a global application typography change.
- The independent verifier found no scoped source defect and confirmed reference inheritance and retained explicit overrides. Its PASS is a source-review receipt only: descendant rendering, rounding, wrapping, clipping and sibling isolation are not established by stored-style assertions.
- Captured the prior Release binary's actual dark English 20px Theme menu in the local control-reference evidence folder. Its labels appear smaller than the interface and its row spacing differs from the reference. The screenshot's binary hash and pre-change identity are recorded in that folder's README; it does not validate the new field/frame changes. The native Select popup mismatch remains open. No global theme retint, compensating negative margins, custom dropdown replacement or dependency edit was introduced.
- **44 focused settings tests passed**, including the expanded frame test, and the offline locked debug build passed using the existing `/tmp/codex-native-target` cache. Executable: `/tmp/codex-native-target/debug/volt`, SHA256 `3eff21aef44b239d53ce5283a76cf93a430034aebcae67a62fd92c2c7786dd01`. The existing `block 0.1.6` future-incompatibility warning remains. These are debug-build receipts, not a new Release-build result.
- The earlier duplicate debug build in `/Volumes/Datas/tmp-codex-native-target` was stopped after approximately 25 minutes while compiling native dependencies. Cargo samples showed its process waiting for compiler output; it emitted no source error and returned 130 after SIGINT. This is not a build pass or a source-failure receipt. Its compiler exited before the cached targeted verification began.
- Launched the matching debug binary in the isolated preview and reached actual Appearance controls at 1200x800 logical size. Dark English at 20px and light Chinese at 14px both show contained Save icons/labels and adjacent visible success feedback after clicking Save. Language, theme and font changes were performed through the native controls. Saved current-build PNGs and hashes in the local control-reference evidence folder, separately from the old Release menu capture. The build identity was rechecked after capture.
- Replayed the focused run's same test executable, `/tmp/codex-native-target/debug/deps/volt-7b95628f8eecb9e5`, with `--test-threads=1 --quiet`: **232 tests passed**, zero failures, in 2.13 seconds. A separate Cargo-wide invocation re-entered `aws-lc-sys` dependency compilation and was stopped with SIGINT (130); that invocation is not a test pass. Cargo, its build-script/compiler descendants and the isolated GUI preview were confirmed exited. The 232-test receipt belongs to the compiled snapshot above, not a later worktree mutation.
- Focused Rust 2021 rustfmt and `git diff --check` passed. The earlier 216-test serial receipt predates this increment and subsequent unrelated worktree edits. Exact native 480x640 layout, visible keyboard activation, provider-field rendering, current menu measurements, complete motion and matched-reference screenshots remain unverified; full acceptance is **PARTIAL**. No commit, push or deployment was performed.

## Settings menu typography follow-up - 2026-11-14

Parent: the settings inherited line-height follow-up and prior native menu mismatch, within the unchanged full ZCode parity objective.
Source: explicit goal continuation, pinned ZCode commit `3c5b227649371507ddc4eb46ac5133afe6db1cab`, shared `select.tsx` item classes, and installed GPUI Component 0.7.3 item/adapter/size implementations.
Reason: native Medium Select rows force 0.875rem text even when the trigger uses the full interface size; the reference options use interface text with relaxed 1.625 line height.
Scope/risk/orchestration: medium-risk native single writer with one bounded independent read-only verifier; the four General/Appearance/footer language/theme menus and their item presentation only. Native Select entities, selection/check state, popup ownership, focus, string-valued events, callbacks, persistence, trigger geometry, dependencies and unrelated dirty work are preserved.

- Added a small settings-choice item implementing the native `SearchableListItem` presentation hook. Only its text child sets 1rem and relative 1.625 line height; the library still renders the row/check/selection wrapper. It does not use the delegate override whose implementation differs from its wrapper-suppression documentation.
- All four settings Select data/event types use the choice items, preserving the original plain localized titles and `SharedString` values. Language changes refresh the theme labels through the same adapter. Text follows the existing window rem size, so font changes do not require stale per-item pixel state or rebuilding choices. AgentConfig's separate Select is unchanged.
- Added focused tests for all displayed Chinese/English labels, 12-20px rem/line-height resolution, an unframed label with no duplicate padding/paint, stable titles/values and native selection lookup. Native row insets, radius, check glyph and missing 2px inter-row gaps remain separate unresolved parity differences; no compensating margins, global theme change, dropdown replacement or dependency fork was introduced.
- The first focused compilation failed on the test's `IndexPath` import and the shared shell's concrete delegate type, which also rejected AgentConfig's existing Select. The import now uses `component::IndexPath`; `SettingsSelect<D>` preserves its delegate through construction, styling and rendering. AgentConfig still uses `Vec<SharedString>` and its original item presentation. The same bounded verifier rechecked that actual call site after the correction and found no scoped source blocker; this is not rendered interaction evidence.
- **46 focused settings tests passed** after the correction. Replaying the same compiled test executable, `/tmp/codex-native-target/debug/deps/volt-7b95628f8eecb9e5`, with `--test-threads=1 --quiet` passed **234 desktop tests**, zero failures, in 2.65 seconds. These results identify that compiled snapshot, not future worktree changes. Focused Rust 2021 rustfmt and `git diff --check` passed.
- The offline locked debug build passed using `/tmp/codex-native-target`, one build job and the existing temporary build directory. Executable: `/tmp/codex-native-target/debug/volt`, SHA256 `cc5b4f336e2ebc44dcb6a247e4b3bf6a55bf0fcf51f2f36575e168eea0a5a644`. The existing `block 0.1.6` future-incompatibility warning remains. No new Release build is claimed.
- Attempted to bind the isolated preview for current-build menu checks, but native UI access reported a locked Mac and failed automatic unlock. No current menu screenshot, font-size/language replay, selection/dismissal or focus receipt was obtained. The owned preview process was stopped (143); its launch also logged an OS sandbox-extension warning, so a usable current-build window is not asserted. Prior native PNGs belong to the earlier frame-typography build.
- Verification remains **PARTIAL** pending an unlocked desktop and current-build menu rendering. Exact 480x640 layout, keyboard activation, complete motion and matched-reference/full-product acceptance remain open. The temporary clipboard attachment is still absent, so its specific button is not identified. Unrelated dirty work is preserved. No commit, push or deployment was performed.

## Restart recovery controls follow-up - 2026-11-15

Parent: the settings menu typography follow-up and unchanged full ZCode parity objective.
Source: explicit goal continuation, pinned ZCode commit `3c5b227649371507ddc4eb46ac5133afe6db1cab`, `v4/PendingCommandRecoveryBanner.tsx`, and installed native Button styling/interaction implementations.
Reason: the recovery prompt still truncated its message, grouped its actions into a non-wrapping cluster and used an icon-bearing outline Resend control instead of the reference's plain primary action.
Scope/risk/orchestration: medium-risk native single writer with one bounded independent read-only verifier; recovery-banner layout, command presentation and status role only. Both existing call sites, maximum width, localized local labels, native focus/activation, recovery state and handlers are preserved. Resend still restores the pending text to the composer for user confirmation; it never submits a model request automatically.

- Added a scoped `workbench/recovery.rs` presentation module. The surface uses 12px/8px padding, 8px gaps, 12px radius, the alpha surface/border and natural text at relative 1.5 line height. Removed the extra warning glyph, card shadow and text truncation. The message and both actions are independent wrapping siblings; the surface and outer wrapper resist flex shrinking.
- Resend uses the existing theme-projected primary black/white palette and no icon. Dismiss uses subtle foreground and the reference hover token through the native custom variant, without another hover registration. Both actions use natural height, 4px vertical padding, 10px/8px inline padding, 6px radius and no border. Their explicit text child bypasses the native library's smaller fixed variant typography. No global theme or dependency change was made.
- Added three builder tests for light/dark surface geometry, 12-20px typography, untruncated natural message layout and padded automatic button height. The first focused compilation failed on a missing `StyledExt` import; after correction all three tests passed. A final anti-shrink assertion was then added and included in the complete compiled suite.
- The first complete Cargo test invocation compiled successfully but spent several minutes in the existing AgentConfig save-stress test with `TMPDIR` on `/Volumes/Datas`. A bounded process sample showed the test thread mostly in `File::sync_all`/kernel file operations. That invocation was stopped with SIGINT (130), not recorded as a pass or source failure. Replaying the same compiled executable with `TMPDIR=/tmp --test-threads=1 --quiet` passed **237 desktop tests**, zero failures, in 1.81 seconds; no tests were removed or weakened.
- The bounded independent verifier found no scoped source blocker, including the final anti-shrink change and unchanged handlers. Focused Rust 2021 rustfmt and `git diff --check` passed. The matching offline locked debug build passed using `/tmp/codex-native-target` and one build job. Executable: `/tmp/codex-native-target/debug/volt`, SHA256 `4af28df826feaeef8147735820739d9b3f6d6ed39d4a246af50815b64cbc66ac`. The existing `block 0.1.6` future-incompatibility warning remains; no new Release build is claimed.
- Native-app inventory again reported a locked Mac and failed automatic unlock. No new GUI process or screenshot was produced in this follow-up. Native wrapping/height, primary and subtle-hover paint, focus traversal/activation, both draft and active-conversation placement, backdrop blur and matched-reference acceptance remain **PARTIAL**. Full-product scope is unchanged; unrelated dirty work is preserved. No commit, push or deployment was performed.

## Queue controls and panel follow-up - 2026-11-15

Parent: the restart recovery controls follow-up and unchanged full ZCode parity objective.
Source: continued control-detail comparison and pinned ZCode commit `3c5b227649371507ddc4eb46ac5133afe6db1cab`, `v4/ConversationQueuePanel.tsx`, shared Button classes and installed native control APIs.
Reason: queue Resume still carried a Play glyph; Edit/Remove used different glyphs, rows were separately framed cards and the queue did not share the composer's column/inset geometry.
Scope/risk/orchestration: medium-risk native single writer with one bounded independent read-only verifier; queue presentation and accessible labels only. Existing queue state, execution-time Resume lock, handlers, edit-to-draft persistence/focus, removal, both call sites and 140px scrolling cap are preserved. This is an explicitly authorized continuation, not a reduction of the full-product goal.

- Added a scoped `panels/queue.rs` presentation module. The surface uses alpha fill, 16px top corners, 4px padding and the reference 28px padded negative-margin tail. Draft/existing-conversation insets now follow their respective composer wrappers. Removed the extra active header and per-row card borders/shadows; the container exposes the queue label/count as an accessible Group.
- Paused notice keeps natural text, a 40px minimum height, 12px gap/insets and border/70. The pinned UI source defines no `surface-raised` token, so no ad hoc raised color was added. Resume is text-only, 24px high, rounded 6px, with 8px inline padding and interface-size text at 1.625 line height. Enabled subtle text becomes primary on hover; the existing disabled condition still blocks activation, with transparent fill, whole-control 50% opacity and no disabled group-hover style.
- Rows use 2px separation, 6px/4px horizontal and 4px vertical padding, 12px radius and hover/30 paint. Edit and Remove use explicit 16px Pencil/Trash assets in 28px rounded-8 native buttons, retaining localized tooltips/accessibility names and actual handlers. The existing Trash asset contains the reference two interior strokes. Truncated prompts now expose their full text through a native tooltip. No nonfunctional drag or Send-now controls were added.
- Initial compilation exposed stateful-scroll and container-label API differences; these were corrected to `Stateful<Div>` and `aria_label`. A later test attempted to inspect a private native hover field and was removed without changing production behavior. Final **5 focused queue tests** passed; the same compiled executable with `TMPDIR=/tmp --test-threads=1 --quiet` passed **241 desktop tests**, zero failures, in 4.32 seconds after final scoped formatting. Tests cover stored style contracts, not rendered glyphs, native event replay or layout acceptance.
- The bounded independent verifier found no scoped source defect, including final disabled-state paint and retained native state/focus ownership. New-module and queue-integration Rust 2021 formatting, tracked diff whitespace and untracked-module whitespace checks passed. The offline locked debug build passed with one build job and `TMPDIR=/tmp`. Executable: `/tmp/codex-native-target/debug/volt`, SHA256 `c43c81ad78f3d31384a66864fe895f8dd9d43950bbb2311cf4b65f9f65593902`. Source hashes: `panels.rs` `532e13d6f9cab7f1e0ae1b4340037f856f511e39767c933115de4821765b17ef`; `panels/queue.rs` `4f44847a77a894f6c374dec8ab6f9cfc0dc8cb40d7fa88a1b65e574dc7c7ab8a`. The existing `block 0.1.6` future-incompatibility warning remains; no Release result is claimed.
- Native-app inventory reported a locked Mac with failed automatic unlock. No current-build GUI process, screenshot or interaction receipt was produced. The original temporary clipboard attachment is absent, so its specific button is not identified. Backdrop blur, color transitions, actual queue/composer overlap, scroll/focus visibility, narrow layout, maximum-font clipping and matched-reference/full-product acceptance remain **PARTIAL**. Unrelated dirty work was preserved; no commit, push or deployment was performed.

## Run notice and Retry detail follow-up - 2026-11-15

Parent: the queue controls follow-up and unchanged full ZCode parity objective.
Source: continued control-detail comparison and pinned ZCode commit `3c5b227649371507ddc4eb46ac5133afe6db1cab`, `ChatErrorBanner.tsx`, shared Button classes and `v4/ConversationComposer.tsx`.
Reason: the existing audit incorrectly described a destructive banner and icon-bearing 28px Retry. The pinned source instead uses a neutral surface and text-only 24px outline action.
Scope/risk/orchestration: medium-risk native single writer with one bounded independent read-only verifier; run-notice presentation, composer placement and announcement role only. Existing retry dispatch, execution-time lock, local stopped state and unrelated dirty changes are preserved.

- Added `workbench/run_notice.rs`. The natural-height, nonshrinking surface uses alpha fill/border, 12px radius, 12px/8px padding and wrapping 8px gaps. Failure uses Info16 with primary text and Alert; stopped runs retain Square16, subtle text and Status. The medium-weight truncated summary exposes its full text through a native tooltip.
- Moved the notice into the composer before its card/context header, with the reference 24px bottom separation and matching draft/existing-conversation insets. Removed the old standalone rendering; recovery and queue handlers remain unchanged.
- Retry is a native, text-only outline Button: 24px height, 6px radius, 8px inline padding and interface-size normal text at relative 1.625 line height. Its normal fill is transparent; disabled paint retains primary ink and applies whole-control 50% opacity. The single `retry_run(cx)` callback and `is_executing` lock remain intact. No duplicate hover registration, global theme workaround or dependency change was introduced.
- Initial compilation exposed a missing `InteractiveElement` import, which was corrected. All four focused builder tests passed. The first full executable run had 229 passes and 16 failures explicitly reporting `ENOSPC`. After relocating task-owned caches, the same compiled executable passed **245 desktop tests**, zero failures; the final post-relocation replay with `TMPDIR=/tmp --test-threads=1 --quiet` passed again in 6.66 seconds. No tests were removed or weakened.
- Matching debug-build attempts initially failed while writing object files because the system disk was full. Preserved the task's incremental cache, archived object files and dependency directory on `/Volumes/Datas`, retaining the original incremental/deps paths through symlinks. The dependency move completed successfully with an empty error log; the existing test executable remained usable. No project source or user data was deleted. The final offline locked, single-job debug build passed in 57.50 seconds. Binary: `/tmp/codex-native-target/debug/volt`, arm64 Mach-O, SHA256 `a8947b9d87e7c994312c8c0fafdfedb262ee4fb0a329a08db2b020eb8444a9a8`. The existing `block 0.1.6` future-incompatibility warning remains; no Release result is claimed.
- The independent verifier found no scoped source defect, checking placement/insets, wrapping, stopped/failure semantics and unchanged native event ownership. New-module Rust 2021 rustfmt and tracked diff whitespace checks passed. Source SHA256: `workbench.rs` `721d134cdafb11d50dcfb14b4a31140af590e469a61edc93420917111046ab39`; `workbench/run_notice.rs` `faf32836951d1809c5484f7f33bf052f451839f32a7f43521195fa4f4c8accaa`. No Cargo/rustc process remained after verification.
- Native UI access repeatedly reports a locked Mac with failed automatic unlock, including this checkpoint. The original clipboard PNG is still absent; its specific button cannot be identified. No current-build window, screenshot or pointer/keyboard replay is claimed. Native outline hover border/fill, backdrop blur, diagnostic/provider-specific actions, actual wrapping/clipping, queue/error/composer overlap, tooltip, focus/activation and matched-reference acceptance remain open. Full visual acceptance is **PARTIAL**, with the screenshot/interaction step blocked pending manual unlock and a replacement attachment. No commit, push or deployment was performed.

## Retry outline interaction follow-up - 2026-11-15

Parent: the run-notice detail follow-up and unchanged full ZCode parity objective.
Source: the user's explicit continuation, pinned ZCode commit `3c5b227649371507ddc4eb46ac5133afe6db1cab`, shared Button outline classes, Zai light/dark tokens and installed GPUI Button/motion implementations.
Reason: the native outline variant did not implement the reference input/50 hover fill, border-hover token or color timing.
Scope/risk/orchestration: medium-risk native single writer with one bounded independent read-only verifier; Retry paint and pointer observation only. Its native callback, execution lock, focus, keyboard activation, 24px frame, 6px radius, 8px inline padding and text-only content remain unchanged. Unrelated AgentConfig/core/release/composer changes are preserved.

- Retry now transitions transparent fill to input/50 (white in light, `#2b2b2b` in dark) and its neutral border from 10% to 15%, over 150ms with cubic-bezier(0.4, 0, 0.2, 1). A scoped native Custom variant supplies the animated border without the library outline flag, while the instance retains its explicit 1px border and separate animated fill. Hover and active use the current animated fill; primary foreground stays unchanged.
- Disabled Retry targets idle paint and settles immediately, retaining whole-control 50% opacity. Reduced motion also settles immediately. Added four tests for the two-theme palette, disabled targets, timing policy and an unframed observation shell; these assertions are not rendered or event-replay acceptance.
- The verifier found that attaching the hover observer directly to the native Button could clear hover during held-press movement because that element owns pending mouse-down state. Moved observation to an unframed outer Div with no click/drag/focus handlers. The same verifier rechecked child hitbox behavior and inside/outside/re-entry source paths and reported no remaining scoped source defect. Native Button event ownership is retained; the geometry test does not replay those events.
- Initial compilation failed on an unsupported `ElementId` tuple for pointer state; corrected it to the stable string `run-notice-retry-pointer`. The separate transition-channel tuples are supported by `TransitionId`. A subsequent attempt failed before tests with `ENOSPC` while copying/linking a dependency build script. Moved only the task's build cache to `/Volumes/Datas/tmp-codex-native-build-20261115` and retained its old path through a symlink; the move completed with an empty error log. No project source or user data was deleted.
- The final offline locked test compilation passed in 16 minutes. A bounded compiler sample showed dependency-metadata loading during the quiet interval; no source error was emitted by that attempt. **8 focused run-notice tests passed**, then the same executable with `TMPDIR=/tmp --test-threads=1 --quiet` passed **249 desktop tests**, zero failures, in 1.77 seconds. Test executable SHA256: `67a021a8ec22156851cd6815b6af4322789bc727656759301bfc2829aee57684`. Scoped Rust 2021 rustfmt and diff whitespace checks passed.
- The matching offline locked single-job debug build passed in 1 minute 9 seconds. Executable: `/tmp/codex-native-target/debug/volt`, SHA256 `29f069feeda07e99a1043a06a724037ad5410e6e66dea87381976285aeed91f1`. Source hashes remained `82e6cb8e14af57f823ce15f2fa6f9d3adc10507a9d9cf0f675c4770e4b51a7d5` for `workbench/run_notice.rs` and `721d134cdafb11d50dcfb14b4a31140af590e469a61edc93420917111046ab39` for `workbench.rs`. The existing `block 0.1.6` future-incompatibility warning remains; no Release result is claimed.
- The Mac is now unlocked, superseding the prior checkpoint's lock report. An earlier isolated preview of the preceding debug binary (`a8947b9d87e7c994312c8c0fafdfedb262ee4fb0a329a08db2b020eb8444a9a8`) produced a startup screenshot and meaningful accessibility tree, but composer typing/click/Return attempts did not establish a changed UI state. Its process was stopped; no cause or successful interaction is asserted for that earlier attempt. It does not contain this hover change.
- Launched the matching new debug binary in `/tmp/codex-native-button-parity-20261115.B00mqi/Volt Button Parity.app`, with isolated state/workspace, an empty environment allowlist, no provider keys and a loopback-only endpoint. A temporary native-automation fixture exercised the existing missing-key branch without a model request. Its first state path did not match macOS's canonical `/private/tmp` working directory; corrected only the temporary fixture's workspace hash. The actual error notice then appeared with a native Retry accessibility control.
- Observed the light Chinese 14px notice and its text-only Retry in the current-build screenshot. Clicking Retry added the expected repeated local prompt and returned to the missing-key error. Through native Settings, committed 20px, selected Dark and English, returned to the conversation, and clicked Retry again; the error changed to English and the action remained visibly contained in its frame. Native menus opened and dismissed through their actual controls. These captures show the existing-conversation placement with no notice/composer overlap at the observed 2400x1600 screenshot size; they do not establish narrow-window or all-font acceptance.
- Pointer-over and pointer-away frames were observed, but no precise pixel/timing comparison, held-press exit/re-entry sequence, disabled-state replay or matched ZCode capture was obtained. Clicking the composer and sending Shift+Tab did not establish a visible Retry focus ring or keyboard activation. Screenshots were inspected inline through the native UI tool, not exported as identity-bound image artifacts. The temporary preview was stopped, leaving existing user app instances untouched. The original clipboard PNG remains absent, so its specific button is not identified.
- Full visual and interaction acceptance remains **PARTIAL**. Diagnostic/provider-specific actions, backdrop blur, narrow/max-font layout, focus/activation and matched-reference captures remain open. No commit, push or deployment was performed.

## Original attachment recovery - 2026-11-15

Parent: the user's settings-button detail request and unchanged full ZCode parity objective.
Source/reason: explicit instruction to recover the attachment locally. Earlier missing-file reports checked the temporary path but missed the complete PNG embedded in this chat's original user message.
Scope/risk/orchestration: low-risk native single owner; read-only history lookup, binary attachment extraction and this evidence correction. No application code or live history database was modified.

- Recovered the original `input_image` data from message `msg_01a1873b-5b73-76b1-bb44-160a04ae320d`, dated 2026-10-29. This is the user's actual attachment, not a replacement screenshot or reconstructed reference.
- Durable copy: `/Users/zhd/.codex/visualizations/2026/10/26/01a1764d-5a1f-74b2-8fa3-c0499328387b/recovered-button-5b7e45a7.png`. Restored the original `/var/folders/h7/9t4_q7bd6p9489dhdy_bdt400000gn/T/codex-clipboard-5b7e45a7-e400-4890-b0b4-a6934412c662.png` path without overwriting an existing file.
- Decoded and visually inspected the 1920x1280 PNG. SHA256: `d51835f5c078dfc28f4d351c85cac9f696f0dde6759c8a846821cf858648e341`. The screenshot shows dark Chinese Model providers settings and its white checkmark Save configuration action. The specific target is the settings Save button, not Retry.
- The existing Save builder is content-sized, with centered icon/text, 32px height, 8px radius, 10px inline padding, a 16px checkmark and a 4px gap. Existing Save refinements and their earlier rendering evidence remain applicable; recovering this old screenshot does not itself establish current-build visual parity.
- Verification: both recovered paths compare byte-for-byte equal; the existing compiled desktop test executable passed all **46 settings tests**, zero failures (203 filtered out). This was a replay of the previously compiled executable, not a new compilation. Repository diff whitespace checks passed.
- Attachment recovery is complete. A replacement upload is no longer required. Earlier absent-attachment statements are historical and superseded by this recovery; full matched-reference visual and interaction acceptance remains **PARTIAL**. No commit, push or deployment was performed.

## Provider Save feedback follow-up - 2026-11-15

Parent: the recovered original settings-button attachment and unchanged full ZCode parity objective.
Source: the user's explicit continuation, the recovered Model providers screenshot, current native replay and installed GPUI Base 0.7.3 input APIs.
Reason: saving Model providers cleared the key input with `clean()`, which emitted `InputEvent::Change`; the existing user-edit subscription then immediately revoked the successful save feedback. Appearance Save did not have that reset.
Scope/risk/orchestration: medium-risk native single writer with one bounded independent read-only verifier; provider Save's programmatic input reset only. Button geometry, native focus/activation, real-edit subscriptions, validation, persistence, runtime key retention and unrelated dirty work are preserved.

- Replaced only that `clean()` call with `set_value("")`. The established input API suppresses the change event and clears selection/undo state. A nonblank key is still read into runtime state before clearing; an empty field still retains the current key. AgentConfig's separate reset is outside this change.
- The independent verifier found no scoped source defect after checking real-edit invalidation, key retention, undo clearing and failed-save gates. This is source-review evidence, not full visual acceptance. No new dedicated unit test was added.
- Offline locked test compilation passed in 1m52s. The resulting executable passed **249 desktop tests**, zero failures, with `TMPDIR=/tmp --test-threads=1 --quiet` in 1.83s. Its SHA256 at that run was `71f829b52e3e8ef75e555bc1a381150ed10cdd41c2189c6a885d48188d7e8624`. A separate concurrent Cargo invocation subsequently replaced the shared test-executable path; that later artifact is not substituted for this receipt.
- The matching offline locked single-job debug build passed in 2m21s. Both `/tmp/codex-native-target/debug/volt` and the isolated preview copy were rehashed after GUI replay as `9ea133802f5cfd16e184ebbbf0241f5406e67f80047547e9838e3dfc8db3a356`. No Release build is claimed; the existing `block 0.1.6` future-incompatibility warning remains.
- Native replay used an isolated workspace/state directory, no environment credentials, a loopback-only endpoint and a non-secret key fixture. Dark English at 20px now keeps visible Settings Updated feedback after Save, including an empty key. Typing a fixture key removed that feedback; Tab from Model exposed the native Save focus ring and Return activated Save, cleared the input and retained success. Restoring a valid address after an invalid-address failure and using Tab plus Space also activated Save and restored success.
- Invalid-address Save showed the existing validation error without success feedback. Clicking the cleared key input and invoking Command-Z left its placeholder visible; this is a bounded undo replay, not an exhaustive editor-history test. Light Chinese at 14px also showed contained icon/text and adjacent success feedback. The native language/theme menus were used to select those states.
- Exported current-build 2400x1600 PNGs (1200x800 logical window) under `/Users/zhd/.codex/visualizations/2026/10/26/01a1764d-5a1f-74b2-8fa3-c0499328387b/`:

| Screenshot | SHA256 |
| --- | --- |
| `provider-save-dark-en-20-after.png` | `2e81514a4b0f3b507795bbc35e04f9e84277431c3cc87a238807cf142eb4e4f6` |
| `provider-save-light-zh-14-after.png` | `20426ba767534c614ad56ce09289f6cd8404d15f0952536749287a834dc9fa5b` |
| `provider-validation-dark-en-20-after.png` | `bf253bd1ecd10846edd346144e40d53df027c2a7831cffc10a42404864baf1ed` |

- Source hashes remained `813c9a2e2d7b206564e19073a82a00c8d7807e6bef9e90fb75582b5764fbc82c` (`interactions.rs`), `49dbdc699daa0c0872f6cadddaeb8e794ac584c23629705292df0c7e794ac7e7` (`main.rs`), `a18a70aec59a43e199888298e2c1c893c07bd10148e02be938abb0a61ed1ca08` (`settings.rs`) and `94763d9e7af5d0a1af153bb13488f7437b787aea4fe222337e374b4b60352dfc` (`settings/buttons.rs`) after replay. Scoped Rust 2021 rustfmt and diff whitespace checks passed.
- The owned preview exited with 143 after SIGTERM; user app instances were untouched. Attachment recovery and this provider-feedback regression are resolved. Exact 480x640 layout, complete pointer/motion checks, menu geometry and matched-reference/full-product parity remain **PARTIAL**; the overarching goal stays active. No commit, push or deployment was performed.

## Settings menu labels and language refresh - 2026-11-15

Parent: the provider Save follow-up and unchanged full ZCode parity objective.
Source/reason: explicit goal continuation, pinned Select source/rendered control fixture, current native accessibility tree and the stale English key placeholder observed after selecting Chinese.
Scope/risk/orchestration: medium-risk native single writer with one bounded independent read-only verifier; settings option text accessibility and provider key placeholder localization only. Native Select state, rows, checks, handlers, focus ownership, provider editor/value/masking and unrelated dirty work remain intact.

- The existing interface-sized menu text now has a stable row-scoped ID, native Label role and matching accessible name, without an extra focus handle/action. Actual native trees now identify the selected/selectable containers as 中文 / English and 明亮 / 暗黑 / 跟随系统, then Light / Dark / System after language selection. This is accessibility-tree evidence, not a VoiceOver audio or full assistive-technology audit.
- Centralized the two existing API key hints in `I18nText`, using the same lookup at input creation and in `change_language` via `set_placeholder`. No value reset or masking change was added. Native Chinese-English-Chinese replay updated the placeholder in both directions; revealing only a non-secret fixture verified its retained value. Remasking and Save cleared the field and kept visible localized success feedback.
- Added two tests: all eight displayed option labels expose their native name with no Focus action, and the language lookup returns the expected hint. Existing geometry/value tests remain. The first compilation failed because Role requires a stateful element and this AccessKit version has no `is_focusable`; corrected the label to `Stateful<Div>` with a stable ID and tested `supports_action(Action::Focus)` instead. The bounded verifier rechecked the final implementation and found no scoped source defect.
- Offline locked test compilation passed in 1m22s. The compiled executable passed **251 desktop tests**, zero failures, with `TMPDIR=/tmp --test-threads=1 --quiet` in 1.94s; SHA256 `27b22487bda13cc2143f6b545e2fb700a8a3d68af742ee8a966eb7f11e0bb8b3`. The matching single-job offline locked debug build passed in 54.59s. Binary and isolated preview copy both rehashed after replay as `bded56c88e4b584b1dc33c06746faa1fe8bddd8719a14b794a1541d07b33ec23`. Existing `block 0.1.6` future-incompatibility warning remains; no Release build is claimed.
- Direct native language Down/Return and Up/Return confirmed selection. Down/Escape kept the committed language. Theme cursor movement from Dark to System followed by Escape retained Dark and stayed in Settings; a second direct open/Escape also stayed there. One earlier Escape after screenshot capture instead returned to the workbench; subsequent bounded replay did not reproduce it, so its cause and unconditional cancellation acceptance remain open. No speculative keyboard-routing change was added.
- Reopened the unchanged pinned reference control fixture. Rendered 20px option measurements remained 40.5px height, 32.5px line height, 4px/8px vertical/horizontal padding, 6px radius, 2px gaps and a 16px selected check, with fonts loaded. The native library still fixes its row wrapper/check presentation and offers no complete public row-style hook; its delegate override does not suppress the wrapper despite that documentation. No dependency fork, compensating negative margins, custom replacement dropdown or global-theme workaround was introduced. These remaining visual differences are not accepted as parity.
- Current native screenshots are 2400x1600 pixels (1200x800 logical); the reference JPEG is 1280x720. Artifacts under `/Users/zhd/.codex/visualizations/2026/10/26/01a1764d-5a1f-74b2-8fa3-c0499328387b/`:

| Screenshot | SHA256 |
| --- | --- |
| `provider-language-roundtrip-zh-14-20261115.png` | `1276042b8b65cc7b4663f9d435bafbe54406c4ce7ca73da2f071049618043a45` |
| `native-menu-dark-en-20-20261115.png` | `2d506d14659ca6745bf8178abcb2d0462d10d404360405f4712a189935b5acd7` |
| `reference-menu-dark-en-20-20261115.jpg` | `b0dff5984a74ee101eae3399962d86111db42644445921f71c147ea4969e28e1` |

- Final source SHA256: `settings/select_motion.rs` `1eb574ce5a74c1af1850116fd6cbea5943ab0c4e623817af4aa6123fd1931cbf`; `i18n.rs` `233286d1b86e3e405917d5ce7ed1e521af1027deec83cabd8e5fc4ca297eac55`; `main.rs` `78d0a81534291bd26a96b948ca0a24839724f74f21a1366de5123f998a3210da`; `interactions.rs` `8cf796203a9da4f35ee506d3cddd39a701bd19680f75eb8f13856307cc918f3c`. The first final formatting check caught import wrapping in `select_motion.rs`; corrected only that whitespace after the build/replay above. Scoped Rust 2021 formatting and diff whitespace checks then passed. No rebuild is claimed for that whitespace-only correction. The debug binary still matches the recorded hash; the shared test-executable path was subsequently replaced (now `e1c329eb368251266f0be20f842dec483d44473dd570ad07b82b251e169c7048`), so it is not substituted for the original 251-test receipt.
- Two edge/corner drags did not establish a smaller native window; exact 480x640 remains unverified, not a source-layout failure or a completed gate. The isolated preview used no real credentials or provider requests and exited with 143; the temporary reference server exited with 130 after verification. Full matched-reference/menu geometry, minimum-window, motion and product acceptance remain **PARTIAL** and the full goal stays active. No commit, push or deployment was performed.

## Default settings columns and compact replay - 2026-11-15

Parent: the settings menu labels/language-refresh follow-up, within the unchanged full ZCode parity objective.
Source: explicit goal continuation, pinned `settings/SettingsPageParts.tsx`, General locale and Appearance theme/font call sites, and isolated native replay.
Reason: the native General/Appearance rows stacked below 640px even though the pinned default SettingsRow always retains its 192px control column; only the separate reference wide-control variant has a single-column breakpoint.
Scope/risk/orchestration: medium-risk native single writer with one bounded independent read-only verifier. General/Appearance row layout and two existing builder tests only; native control handlers, local informational rows, shortcuts, unrelated dirty work and full-product acceptance scope are preserved.

- Removed the viewport-dependent stacking input from General and Appearance and explicitly selected horizontal layout for their four rows. Text remains shrinkable with natural height; the fixed control column, spacing, padding and right alignment remain unchanged. Renamed the shared helper's boolean to `stacked` to distinguish an explicit local layout choice from a default viewport rule. Capabilities/subagents still stack below 640px; shortcuts retain their separate responsive layout. Updated `DESIGN.md` to supersede the earlier intentional but non-matching compact adaptation.
- The independent read-only verifier confirmed the pinned default/call-site distinction and retained language/theme subscriptions, font Enter/Blur handling and reduced-motion callback. No scoped actionable defect was found. The two renamed builder tests cover explicit stacking geometry, not actual 480px rendering.
- Offline locked single-job test compilation passed in 3m05s after initially waiting for another test build's cache lock. The compiled executable was copied to the owned temporary preview directory before replay, avoiding later replacement of the shared Cargo output. `TMPDIR=/tmp` serial replay passed **254 desktop tests**, zero failures, in 2.19s. Retained executable: `/tmp/codex-native-button-parity-20261115.B00mqi/volt-compact-tests-20261115`, SHA256 `724391e9fcd35b9c26c463ad5baee8e46496a386e1a044647ef71d9b9596e2bf`. The existing `block 0.1.6` future-incompatibility warning remains.
- Before the source adjustment, replayed the prior verified debug binary/preview SHA256 `bded56c88e4b584b1dc33c06746faa1fe8bddd8719a14b794a1541d07b33ec23` with isolated state/workspace, no environment credentials and the loopback-only endpoint. One right-edge drag changed the captured window from 2400x1600 to 1920x1600 pixels (960x800 logical at the observed 2x scale), activating the compact settings icon rail. Further edge/corner and title-region drags did not establish 480x640; no cause is asserted. This is not minimum-window acceptance.
- In that 960x800 window, native language/theme controls selected dark English and the font field committed 20px. Provider labels, inputs and Save remained contained. Tab from Model visibly focused Save; Return activated it and left Settings Updated visible. No API key or provider request was used. The preview exited with 143 after SIGTERM. These screenshots belong to the pre-adjustment binary, not a rendered receipt for the new below-640px row rule.
- Saved native 1920x1600 PNGs under `/Users/zhd/.codex/visualizations/2026/10/26/01a1764d-5a1f-74b2-8fa3-c0499328387b/`: `provider-compact-en-20-focus-20261115.png`, SHA256 `3ad39440876c2743bcba68e1700f9c8e3c79f80f4c9126a718a853fdfab7dfad`; `provider-compact-en-20-saved-20261115.png`, SHA256 `8f1d4862d8fe2c353202512a8a7468aeabbb5d314fc8a3d4a3074629b4442fda`.
- Scoped source SHA256: `settings.rs` `980766c311266ccf81c5320038bf67c6a381caf3830790598db942dea9b9ee97`. Pinned `SettingsPageParts.tsx` SHA256 `087e102d95ed8c70480bcbdadf0182cbc16b7fd7bb07880c13d332d1a13a068b`. Scoped Rust 2021 formatting and diff whitespace checks passed.
- No new debug-build pass is claimed. The normal offline locked build stayed live for more than ten minutes without a source error; a short compiler sample showed incremental-session garbage collection and dependency-graph loading. Stopped only that owned Cargo/compiler attempt with SIGINT (130). A second diagnostic sample was interrupted and terminated (143), not recorded as a completed sample. Retried with `CARGO_INCREMENTAL=0` and the same dependencies; another workspace check held the shared build lock, so the owned queued retry was stopped with SIGINT (130) without interrupting the other check. No cache was deleted or source/dependency setting changed. Both owned build sessions and the preview are terminal; the 254-test receipt remains separate from a new application-binary or rendered-layout receipt.
- Exact 480px English/maximum-font wrapping, control reachability, matched-reference frames, menu geometry, motion and full-product parity remain **PARTIAL**. No commit, push or deployment was performed; the overarching goal stays active.

## Font-independent settings spacing - 2026-11-15

Parent: the default-settings-column follow-up and unchanged full ZCode parity objective.
Source/reason: explicit user continuation after recovering the original Save-button attachment. The pinned `lib/uiFontSize.ts` changes only `--ui-font-size`, while native GPUI Component's root binds rem to the theme font size.
Scope/risk/orchestration: medium-risk native single writer with one bounded read-only verifier; positive spacing shorthands in `settings.rs` only, plus focused tests and contract documentation. Preserve text scaling, handlers, library theme, AgentConfig's separate surface and unrelated dirty work.

- Replaced settings-owned rem spacing with explicit 4px-scale pixels across rows, navigation, footer, page separation, provider form, Appearance, Skills and Usage. Text size and natural wrapping are unchanged. Added a test resolving row/shortcut/Save/field gaps, row padding and group-label padding at every interface size from 12 through 20px, in both palettes; updated existing geometry assertions.
- The verifier confirmed the reference/native rem distinction and all spacing conversions. It caught the new test's unimported `gpui` alias; corrected the two references to the public `gpui_kit` re-exports. This source review is not compilation or rendered acceptance.
- Reopened the existing compiled pinned control fixture through a temporary loopback static server. At both 12px and 20px interface size, the root remained 16px, page gap/padding 32px, field gap 4px, and Save height/padding/gap 32px / 10px inline / 4px. These are rendered shared-control measurements, not a reference Settings-page or native-window comparison. The temporary browser tab and server were closed.
- Final-source compilation and native replay remain pending. The first test compilation waited for another Clippy run, then was interrupted (130) after the test-alias correction; its diagnostic sample was also stopped (143), not accepted as a finished profile. The final-source nonincremental retry remained queued behind another workspace test build and was stopped (130) without acquiring the cache lock. No other writer's process was stopped. No old executable is substituted for a final-source test receipt.
- Final `settings.rs` SHA256: `f8f5c436d187343bfdb4cb68792392e2656280a2a853c2f1fc6ab2269d6ada99`. Scoped Rust 2021 formatting and diff whitespace checks passed. Both owned Cargo attempts, the diagnostic sample and reference server are terminal; no new application binary or native screenshot is claimed.
- Full visual acceptance remains **PARTIAL**. No commit, push or deployment was performed.

## Held-press settings hover and final-source spacing replay - 2026-11-15

Parent: the font-independent spacing follow-up, recovered Save-button attachment and unchanged full ZCode parity objective.
Source/reason: explicit user continuation and independent review of GPUI's clickable-element hover handling. A held mouse move suppresses a clickable element's hover callback even with input-modality-independent observation.
Scope/risk/orchestration: medium-risk native single writer with one bounded independent read-only verifier. Settings command pointer observation and its layout tests only; preserve the native Button, AgentConfig callers, handlers, credentials, theme and unrelated dirty work.

- Added a stable, listener-only outer observer. It receives parent-facing width/min/max width, margins, placement, alignment and flex constraints; the child retains paint, padding/height, focus, click, accessibility and tooltip ownership. Content-sized Save, full/fixed navigation widths, Back margins and the absolute 24px key-visibility target are covered by three new builder tests. The independent verifier found no actionable scoped source issue.
- Final-source direct Rust test compilation and application linking passed using a private local copy of the already-built dependency artifacts. This is not a fresh Cargo/workspace dependency-validation receipt. The earlier external-volume compilation was stopped with SIGINT and exited 1 without a source diagnostic; its diagnostic sample and unsuccessful glob-based mirror attempt exited 143. None is a passing receipt. No shared cache or other owner's process was removed or stopped.
- The new owned executable ran **51 settings tests**, including all three observer tests and the previously pending fixed-pixel spacing test, with zero failures. Serial replay of the same executable passed **259 desktop tests**, zero failures, in 2.06s with `TMPDIR=/tmp`. Test executable: `/tmp/codex-native-button-parity-20261115.B00mqi/volt-settings-hover-tests-20261115`, SHA256 `5afbc1b723512372f5ad47938ca41aeb144711dbfdffdeced5b1dccd11b98d60`.
- Current application binary: `/tmp/codex-native-button-parity-20261115.B00mqi/Volt Spacing Parity.app/Contents/MacOS/volt`, SHA256 `1436486a0af0763a97484b6d558bf7fae2c3154a4cc988ca63fde7923ce560d2`. Launched with an empty environment allowlist, isolated HOME/state/workspace, no keys, disabled environment credentials and a loopback-only endpoint. Native Settings opened with contained provider fields and Save at dark English 20px; Tab from Model showed the native Save focus ring, and Return produced Settings Updated.
- Native Appearance committed 14px, the language menu selected Chinese, and reduced motion was turned off. Clicking Save produced localized success. Its pointer-over screenshot showed the dimmed primary fill; clicking Base URL restored the white idle fill without losing success. This bounded replay verifies ordinary hover/exit, native click and keyboard activation, not a held-press exit/re-entry sequence or precise animation timing.
- Saved 2400x1600 PNGs under `/Users/zhd/.codex/visualizations/2026/10/26/01a1764d-5a1f-74b2-8fa3-c0499328387b/`: `provider-spacing-hover-dark-en-20-20261115.png` (`ea999cde657e3c31dd76c68d0c003fdc62ef885823660b3bb805fe5ca80f97e6`), `provider-spacing-hover-dark-zh-14-20261115.png` (`358776a19713b8505bef615d0776b479570eceddcb63912ac9300fefc0b4dfaa`) and `provider-spacing-idle-dark-zh-14-20261115.png` (`e0cb752e77eb319e60f477d5d49c09f5f1777b6c50d0804e1d9af8de725bfda7`). The English image captures keyboard focus/success, not pointer hover despite its filename.
- Source SHA256 after replay: `settings/command_motion.rs` `8a925721e6db8fd48666a8e52d25c971d8e8273b64fcc79353a94cbf827bcd80`; `settings.rs` remains `f8f5c436d187343bfdb4cb68792392e2656280a2a853c2f1fc6ab2269d6ada99`. The binary also retained its recorded hash. Scoped Rust 2021 formatting and diff whitespace checks passed. The owned preview exited 143 after SIGTERM; existing user instances were untouched.
- Attachment recovery is complete; no replacement upload is needed. Exact 480x640, held-press replay, disabled-state replay, menu geometry, animation timing and matched-reference/full-product acceptance remain **PARTIAL**. No commit, push or deployment was performed; the overarching goal remains active.

## Minimum-window settings rendering replay - 2026-11-15

Parent: the held-press observer/fixed-spacing follow-up and unchanged full ZCode parity objective.
Source/reason: explicit continuation after original attachment recovery; complete the previously missing minimum-size rendering check.
Scope/risk/orchestration: low-risk native single owner, isolated rendering fixture and evidence only. No tracked product source, dependency, startup option or user state was changed in this follow-up.

- Relaunched the verified normal preview; three edge/corner drags did not establish a smaller window. Stopped only its owned process (143). No resize defect or successful live-resize acceptance is inferred.
- Built a private source fixture with exactly one difference from the verified product source: initial window bounds changed from 1200x800 to 480x640. The existing minimum size, native renderer, components, layout, handlers and dependencies were unchanged. This used the private local copy of already-built dependencies, not a fresh Cargo/workspace dependency-validation run. The fixture's successful compilation/link and plist lint are separate from the normal application receipt above.
- Fixture executable: `/tmp/codex-native-button-parity-20261115.B00mqi/Volt Narrow Parity.app/Contents/MacOS/volt`, SHA256 `b1e120de44f94a40631b9036a72494f68a0fd1d2694f368cc2e8f5f23070b6a3`. Launched with an empty environment allowlist, isolated HOME/state/workspace, no keys, environment credentials disabled and a loopback-only endpoint. All five saved PNGs are 960x1280 pixels, corresponding to 480x640 logical bounds at the observed 2x scale.
- At dark English 20px, provider labels, inputs and Save remained contained. Tab from Model visibly focused the native Save button; Return activated it and retained Settings Updated beside the button without overlap. General retained the reference's fixed control column with naturally wrapped text. Its actual language menu opened within the window; Escape dismissed it without changing language or leaving Settings.
- Appearance's longer rows scrolled to the font field, reduced-motion switch and Save. Clicking Save showed contained success feedback. Shortcuts retained its explicit compact stacked layout and scrolled between the title/first rows and final Back row. Switching from the scrolled Appearance page initially retained the shared scroll position; scrolling back exposed the Shortcuts title. This observation is recorded, not treated as reference parity or silently changed.
- Screenshots under `/Users/zhd/.codex/visualizations/2026/10/26/01a1764d-5a1f-74b2-8fa3-c0499328387b/`:

| Screenshot | SHA256 |
| --- | --- |
| `provider-480x640-dark-en-20-initial-20261115.png` | `232feb7fc318e619ae9951f107bd86f2ecc6b7de725789e355b66282edf7d347` |
| `provider-480x640-dark-en-20-saved-20261115.png` | `7c22fd40f13f72b7664ef2f376a3f3ec3696c721f58825c46baf457871f573e4` |
| `general-480x640-dark-en-20-20261115.png` | `3f4ceb5865322dbc5c842aec2091432ef44daebab8054b04cb27ec8513f055e4` |
| `appearance-480x640-dark-en-20-bottom-20261115.png` | `a04e005da1c3e786f62bcd3219c9234af47e49b1132b07c8525c1d65f3f2b4b6` |
| `shortcuts-480x640-dark-en-20-20261115.png` | `b97d18cc73cae94128dbda26531e0e0b6a9f7c9a42297e2a75d7edc38a87612d` |

- Replayed the retained final-source executable `volt-settings-hover-tests-20261115`: **259 desktop tests passed**, zero failures, in 2.21s with `TMPDIR=/tmp --test-threads=1 --quiet`. This is a rerun, not another fresh compilation. Product source hashes remain those recorded above. Stopped the owned narrow preview with SIGTERM (143); existing user instances were untouched.
- Minimum-window rendering now has bounded native-fixture evidence. Live resize, held-press exit/re-entry, disabled replay, precise animation timing, native Select row geometry and matched-reference/full-product acceptance remain **PARTIAL**. No commit, push or deployment was performed; the overarching goal remains active.

## Error-banner dismiss action - 2026-11-15

Parent: the minimum-window checkpoint and unchanged full ZCode parity objective.
Source/reason: explicit continuation of the button-detail comparison after attachment recovery.
Scope/risk/orchestration: medium-risk native single writer, one independent read-only verifier, targeted tests and isolated native previews. Existing unrelated dirty work was preserved.

- Followed the pinned `ChatErrorBanner.tsx` dismiss action and `button.tsx` outline/icon-sm variants: a trailing 24px square, 16px X, 6px radius, 1px border, localized accessible label and native tooltip. Retry and Dismiss share the existing 150ms outline color policy, including reduced-motion/disabled snapping and the listener-only held-press observer; no dependency or global-theme change was made.
- Both actions remain disabled while executing. The dismiss handler also short-circuits before taking `run_error` while busy, including cancellation cleanup. Once idle it clears only the displayed error and restores root focus. It does not change `run_stopped`, goal outcome/iteration, transcript or queued prompts; later session capture still leaves the interrupted queue paused.
- Compiling caught an invalid two-string GPUI element key. Pointer, observer, background and border now use explicit compound `ElementId`s. Four new tests cover the eight independent keys, icon-button geometry, busy dismissal protection and the paused-queue/failed-goal invariant.
- The restored core artifact was older than the current core source. Recompiled the current core library, current desktop tests and current application against private copies of the existing third-party artifacts. **263 desktop tests passed**, zero failures, in 2.48s, with an isolated HOME, empty credential environment and serial execution. This is current-source compilation/test evidence, not a fresh Cargo dependency-resolution or full locked-workspace build receipt.
- The normal offline locked Cargo test-compilation attempt was stopped with SIGINT (130) after more than six minutes in dependency fingerprint filesystem reads. The earlier direct desktop attempt also exposed the obsolete core artifact; neither attempt is counted as a pass. The independent static verifier found no scoped behavioral defect, but compilation and GUI evidence remain separate gates.
- Current source SHA256: `workbench/run_notice.rs` `984ed35d5b0d1a2dc4cc6348199126e994c572f7431ae0698d3d74d527d0d586`. Test executable `/tmp/codex-native-dismiss-20261115.F4j9qO/volt-dismiss-tests`: `aa1d71caa5b01cd5813999952a81e6b1c2fd0e704c31f9c972fc8636e5a3c886`. Native preview `/tmp/codex-native-dismiss-20261115.F4j9qO/Volt Notice Parity.app/Contents/MacOS/volt`: `bf43c1e3c6573bc8eb5abc81a2a60aaa1293c9a2d913eb24dbca9242a4b11253`, rechecked after capture.
- Native interaction tools were unavailable. Used the screenshot skill's read-only window listing and explicit-window-ID capture instead, without AppleScript activation or simulated input. Previews used isolated HOME/state/workspace, no keys, environment credentials disabled, a loopback-only endpoint and a seeded failed session; no provider request was needed.
- The light Chinese 14px capture shows the complete failure summary, Retry and the contained X action above the composer. Window bounds were 1200x800; the 2492x1692 PNG includes OS shadow margins. The dark 20px capture likewise shows a contained icon and separate actions; observed window bounds were 1200x752, with a 2492x1596 PNG. Despite English preferences/error text, other controls in that dark capture are Chinese: this is **not** an English-localization pass, and no cause is inferred. An earlier settings-page capture did not show the target banner and is not counted.
- Saved PNGs under `/Users/zhd/.codex/visualizations/2026/10/26/01a1764d-5a1f-74b2-8fa3-c0499328387b/`: `notice-dismiss-light-zh-14-20261115.png` (`5e02a94f3362a41ed0074776d3f3e11ab799177bf8b890730fe6605f1e5337ce`) and `notice-dismiss-dark-20-20261115.png` (`eded983817cbbf8b3fffa34aaf70b7788d9a86e825a1b13f1551af1801ea3b22`).
- Owned preview processes exited 143 after SIGTERM; other app instances were untouched. Formatting and whitespace checks cover the changed module and tracked diff. Real dismiss/keyboard/focus/tooltip replay, held-press and disabled rendering, English-localized replay, minimum-window rendering for this new action, precise timing, other outstanding error actions and matched-reference/full-product acceptance remain **PARTIAL**. The overarching goal stays active. No commit, push or deployment was performed.

## Error-banner copy action and single-line summary - 2026-11-15

Parent: the dismiss-action checkpoint and unchanged full ZCode parity objective.
Source/reason: continued visual-detail comparison against the pinned ZCode 3.16.2 source after attachment recovery.
Scope/risk/orchestration: medium-risk native single writer, one independent read-only verifier, targeted current-source tests and isolated native previews. No unrelated dirty work was reverted.

- Added the reference Copy action before Retry: 24px height, 6px radius, 1px outline, 8px horizontal padding, 14px Copy icon and a localized visible label. An explicit content row supplies the 6px icon/text gap. The verifier caught that setting gap on the button itself leaves GPUI's inner small-button gap unchanged; the corrected test checks the actual content row. Retry and Dismiss retain their geometry and execution guards through the shared outline builder.
- Copy remains available while executing and reads the current `run_error` and language at click time. It copies the entire available error string using the reference localized heading/summary template, retaining multiline content and whitespace; it does not fabricate unavailable structured detail or trace fields. A stable-ID, in-app, auto-hiding native notification supplies localized feedback without an explicit focus change. It does not mutate the error, stopped state, queue, goal or transcript. The native clipboard write API has no result value; actual clipboard delivery is not established by the unit tests.
- The multiline light fixture exposed a separate rendering mismatch: GPUI nowrap preserves hard line breaks, unlike the reference CSS nowrap. Only the displayed summary now collapses ASCII whitespace; NBSP/CJK content is preserved. Tooltip text and copy payload still use the original error. The three actions have twelve distinct pointer/observer/background/border keys and share the existing reduced-motion/disabled-aware outline transition.
- Recompiled the current core library, desktop tests and desktop application against the private cached third-party artifacts. Final **267 desktop tests passed**, zero failures, in 3.44s, with isolated HOME, empty credential environment and serial tests. Formatting, tracked `git diff --check` and the untracked notice module's whitespace check passed. This is not a fresh Cargo dependency-resolution or locked-workspace build receipt. The same independent verifier confirmed the gap and summary corrections with no further scoped finding.
- Final source SHA256: `workbench/run_notice.rs` `13c3a5756af57308310441090cd7108c6b829666e134967c3d859e425b3ec047`. Test executable `/tmp/codex-native-dismiss-20261115.F4j9qO/volt-dismiss-tests`: `80157f0df4a960ac6eed510baa5ef7130980e540384846908d1ff2a165a95b6f`. Native application `/tmp/codex-native-dismiss-20261115.F4j9qO/Volt Notice Parity.app/Contents/MacOS/volt`: `7d3102ff51f7b251fc46a0fc4779d8a01a9425f5a21fc3d9625f8581521a84fb`. Recompiled private core: `5e7fe6555380a7df200f3396ef00048326843a8765ca5b8375ad1cc6043c44a6`.
- Final native previews used fresh, separate light-Chinese-14px and dark-English-20px state roots, isolated HOME/workspace, environment credentials disabled, no keys and a loopback-only endpoint. The dark profile also enabled reduced motion. Read-only explicit-window-ID captures show single-line summaries and separate Copy/Retry/X controls at 1200x800 window bounds. English labels and sidebar rendered in English; the previous mixed-language capture was not reproduced, but no cause or language-switch fix is claimed. An earlier pre-correction multiline screenshot is not final acceptance evidence.
- Final PNGs under `/Users/zhd/.codex/visualizations/2026/10/26/01a1764d-5a1f-74b2-8fa3-c0499328387b/`: `notice-copy-light-zh-14-20261115.png` (2492x1692, SHA256 `34ea9d204852c7b647bc0ef6e49f40365198a5531f188b17f38fa4334f2db223`) and `notice-copy-dark-en-20-20261115.png` (2624x1824, SHA256 `338e65c982fcecefd8accc635bc01606b9d2846b2450e2148a25c0ed990a6f8b`). PNG dimensions include capture margins and are not the window bounds.
- All owned previews were stopped by exact PID and exited 143; no owned compiler or preview remained. Native interaction tools were unavailable, so real click/clipboard/notification-focus replay, tooltip and keyboard access, hover/held-press timing, disabled rendering, interactive language switching, minimum-window/live-resize coverage and matched-reference/full-product acceptance remain **PARTIAL**. Other reference error actions, including structured details and feedback, are still outstanding. The overarching goal stays active; no commit, push or deployment was performed.

## Model-setup notice actions - 2026-11-15

Parent: the Copy/single-line-summary checkpoint and unchanged full ZCode parity objective.
Source/reason: continued button-detail comparison against the pinned ZCode 3.16.2 `ChatErrorBanner.tsx`, `button.tsx`, locale and gradient definitions, after recovery of the original attachment.
Scope/risk/orchestration: medium-risk native single writer with one bounded independent read-only verifier; onboarding notice surface, actions and configuration routing only. Existing provider Save refinements and unrelated dirty work remain intact.

- Reused the neutral notice surface, single-line summary and outline motion for the missing-model notice. Configure now uses the reference localized labels (`配置` / `Set`), 24px height, 6px radius, 8px horizontal padding, 14px Settings icon and an explicit 6px gap in the actual content row. Its separate dismiss action uses a 24px square and 16px X. Five outline actions now have twenty distinct pointer/observer/background/border keys.
- Configure explicitly selects Model providers, saves that section preference, clears stale settings feedback, restores root focus and persists the session. It does not clear draft inputs, run errors or task state. The notice is hidden after dismissal or when the native provider already has a nonblank in-memory key.
- The preexisting Upgrade control had no purchase handler. It is now explicitly disabled, with 0.5 opacity rather than an enabled dead action. Its reference shape and white foreground are retained, using the light 135-degree `#191a1d` to `#747689` gradient and dark `#484a58` fill. Native purchase/inventory capability remains missing and is not counted as implemented.
- Recompiled current core, desktop tests and normal application against private copies of cached third-party artifacts. **270 desktop tests passed**, zero failures, initially in 1.63s and on final serial replay in 1.80s. Three new tests cover visibility/localization, Configure geometry and disabled Upgrade styling; existing action-key and Copy/Retry/Dismiss tests also pass. This is current-source compilation and testing, not fresh Cargo dependency-resolution or locked-workspace validation.
- The independent read-only verifier found no scoped source defect after checking routing, draft retention, key visibility, Auto-theme ordering, disabled styling and action keys. Its review is not click, keyboard or GUI acceptance. Scoped Rust 2021 formatting and tracked diff whitespace checks passed.
- Isolated native previews used separate dark-English-20px/reduced-motion and light-Chinese-14px state roots, no keys, environment credentials disabled and a loopback-only endpoint. Read-only window-ID captures show contained actions at 1200x800 logical bounds. A private narrow fixture changes only initial window bounds from 1200x800 to 480x640; product minimum size, layout, handlers and assets remain unchanged. Its dark-English-20px capture shows a truncated summary and separate Upgrade/Set/X controls without action overlap. The banner still spans the content width while the composer has gutters; no matched-reference layout acceptance is inferred. Suggested prompts extend below the narrow viewport; scroll reachability was not replayed.
- Saved screenshots under `/Users/zhd/.codex/visualizations/2026/10/26/01a1764d-5a1f-74b2-8fa3-c0499328387b/` (PNG dimensions include capture margins):

| Screenshot | Dimensions | SHA256 |
| --- | --- | --- |
| `model-setup-dark-en-20-20261115.png` | 2492x1692 | `f142b67efe4655b1de9e0af7406b0cece9f1537a76b38eef128153796c2ecefa` |
| `model-setup-light-zh-14-20261115.png` | 2492x1692 | `c5c9219515dc7ee96a904e02a3075ea66711d4affbf453073f7f20a60a55e42a` |
| `model-setup-480x640-dark-en-20-20261115.png` | 1184x1504 | `130c650ede81eaf8577ef5682b2f3f64cc2a803ca3243db3d479eec01386cd38` |

- Final SHA256: `workbench.rs` `1a6ac6bfbe4ed8dd30f0d9f5aa520ce5016646361d73ae917fb0f882a4856a42`; `workbench/run_notice.rs` `e3713d7574d58d89db4012a3196d94c79daff3821bc7fd9c8f4750dfc6f2204a`; private test executable `e80b29edc07c94e32ab842c6d90ef569c27870c82ad581fe238efec8c2ece24f`; normal preview `db587c2736d00c047bd8698be2569edd4dbe191a72fb6a4a7a6e02b190152908`; narrow fixture `633d0108d9ea3f7dee98e38234ec9ced446cd4c28d325b8d179175e0cf99fec2`. Artifacts are in `/tmp/codex-native-dismiss-20261115.F4j9qO/`; source/normal-preview/test hashes were rechecked after capture.
- All owned previews exited 143 after exact-PID SIGTERM; existing user app instances were untouched. Native interaction tools were unavailable. Configure click/return/draft preservation, persisted route replay, dismiss focus, keyboard/tooltip/hover/held-press behavior, Auto following a live system change, disabled Upgrade activation, live resize and matched-reference/full-product acceptance remain **PARTIAL**. The original attachment is recovered and needs no replacement upload. No commit, push or deployment was performed; the overarching goal stays active.

## Draft prompt strip and fixed composer insets - 2026-11-15

Parent: the model-setup checkpoint and unchanged full ZCode parity objective.
Source/reason: explicit goal continuation, pinned `ConversationDraftSuggestedPrompts.tsx` chips branch and `ConversationTimeline.tsx` centered-layout gutters. The previous turn made concrete source/test/capture progress; this follow-up addresses remaining visible draft-layout differences rather than declaring the full objective complete.
Scope/risk/orchestration: medium-risk native single writer with the same bounded independent read-only verifier. Draft prompt presentation and composer/setup insets only; prompt contents, draft persistence/focus, queue semantics, provider Save refinements and unrelated dirty work remain intact.

- Replaced the wrapping 16px-radius pills with one horizontally scrollable native strip: 32px action height, 8px radius, 12px inline padding, 16px inter-action gap, explicit 6px icon/text gap, 16px glyphs, 18px text line height and 256px label cap. Non-shrinking actions retain their widths; auto flex margins center a fitting strip and yield to left alignment when it overflows. The viewport retains the 672px cap and fixed 16px gutters. Native activation still inserts the existing full prompt, saves the draft and focuses the composer.
- Content uses primary ink at 70% opacity, rising to 100% on hover with the reference input/50 fill and border-hover token over 150ms. A listener-only shell observes held native presses without replacing Button semantics or registering a second button hover style. Disabled actions keep their execution lock and whole-control 50% opacity; disabled/reduced-motion colors settle immediately. Waterfall entrance is not implemented by this increment.
- Added fixed 16px gutters around the setup notice. Initial native 20px replay exposed that the composer still used rem-based `px_4()`, which resolves to 20px because the library root binds rem to the interface font. `composer_dock` / `composer_column` now use explicit 16px horizontal and existing-conversation bottom insets, preserving the mutually exclusive draft-outer versus conversation-inner padding. Final narrow capture shows aligned setup/composer boundaries. No global rem-size, theme or dependency change was made to the product.
- Added six regression tests: prompt scroll/strip layout, action geometry, text/content geometry, hover/disabled policy, setup gutters, and composer insets across 12-20px rem sizes. Final current-source test executable passed **276 desktop tests**, zero failures, in 1.82s. Normal and narrow application compilation/link also passed against private cached third-party artifacts. This is not a fresh locked Cargo/workspace dependency-validation receipt. Scoped module rustfmt and tracked diff whitespace checks passed.
- The verifier found a real diagnostic-fixture defect: PascalCase icon names would all fall back to SquareCode. Corrected them to `alarm-clock`, `bug`, `presentation`, `moon`; actual browser SVG classes then confirmed all four. Its final review found no additional scoped source defect, including after the rem/inset correction. Static review does not establish native click, keyboard or wheel behavior.
- Added `view=draft-prompts` to the existing reference-control harness, importing the actual reference component and locale provider. Selection changes only a hidden diagnostic output; there is no composer/plugin/backend request. The reference snapshot no longer has Git metadata, so `prepare.py` correctly refused staging; its revision guard was not weakened. Fetched the pinned commit's upstream Git tree and verified blob hashes for the prompt component, timeline, prompt-item types, locale provider, Button and stylesheet before manually staging the diagnostic entry. This verifies those six files, not the entire reference checkout or dependency lock.
- Restored missing dependencies only in the preexisting private reference runtime: Motion 12.38.0 and Zod 4.6.5, plus workspace-package symlinks. npm removed the prior extraneous shadcn artifact; restored the actual pinned `shadcn@4.1.1` package archive/CSS without executing its CLI or install scripts. The first reference navigations failed or timed out during startup/dependency loading; none counts as rendering evidence. A later snapshot and computed styles confirmed the loaded actual component.
- At a 480x640 CSS viewport, dark English 20px reference measurements were: viewport x=16 / width=448 / height=32, group width=628.140625, all actions height=32 / radius=8 / padding=12 / content gap=6 / text line height=18 / text opacity=0.7, with loaded fonts and the four intended glyphs. A real horizontal wheel moved reference scrollLeft from 0 to 180 and revealed the final action. These are browser-reference receipts, not native scroll acceptance or a matched full-desktop pixel comparison.
- Final native previews used the same isolated no-key, no-environment-credentials, loopback-only fixtures as the parent. Wide light Chinese 14px shows four centered rectangular actions; narrow dark English 20px shows a single overflowing row and aligned setup/composer gutters. The narrow fixture differs from product source only in initial bounds (1200x800 to 480x640), retaining the same minimum size, renderer and handlers. Pre-inset-correction captures are diagnostic only and are not substituted for final screenshots.

| Final screenshot | Dimensions | SHA256 |
| --- | --- | --- |
| `draft-prompts-light-zh-14-fixed-insets-20261115.png` | 2624x1824 | `3a47dbb75507469e82022f7f1dc9011953b5d4eec3d1aca20398973b436b9a54` |
| `draft-prompts-480x640-dark-en-20-fixed-insets-20261115.png` | 1184x1504 | `16029d7ae4d17eef82880a4720e902959a374904ae88a97013d19217a086cc91` |
| `reference-draft-prompts-480x640-dark-en-20-20261115.png` | 480x640 | `3f0b1a692d6ff2120eec74706396b3aa62d5c4704b26db477a270a017e9a57da` |

- Screenshots are under `/Users/zhd/.codex/visualizations/2026/10/26/01a1764d-5a1f-74b2-8fa3-c0499328387b/`. Native PNGs include window-capture margins and are not the browser CSS viewport scale. No pixel-error score is reported for these unmatched frames.
- Final SHA256: `workbench.rs` `74a28795617cdc9dc7a62b9ec6f1456dbaee90e9657e7947fba13e566d9f6440`; `run_notice.rs` `854aadd9da7de27d44404f9dd188c526c98a1b4573a088c1327c707a57a9bf0b`; `draft_prompts.rs` `62af8fba46ac6e34ed93a096c59fc995e6cd228f3190ec3aaac059a2ca6de5e6`. Private final tests `f3f24c35abe0c8610e044e995d24769aa78454e81110023cddf7b57a1743d607`; normal application `b51d37ab56bb2556c616cace06ea6cdec5d351a0d3cea0c76ac62f889af712d7`; narrow fixture `3b3a13521a13af906c81439dc756b674946748d32f267c3d33137ab6df97d03d`.
- Native pointer/keyboard/wheel/focus replay, offscreen-action keyboard reveal, held-press timing, waterfall entrance, live resize, remaining notice actions and matched-reference/full-product acceptance remain **PARTIAL**. The full goal stays active. No commit, push or deployment was performed.
- Final reference click replay selected `idle-tasks` in the diagnostic output and left that button focused; this is not native composer activation evidence. The named browser was closed, the owned Vite server and all native previews exited 143 after exact-PID SIGTERM, and no owned compiler remained. Final product-source/application hashes were unchanged after capture; final tracked diff and new prompt-module whitespace checks passed.

## Draft prompt waterfall - 2026-11-15

Parent: the draft-strip checkpoint and unchanged full ZCode parity objective.
Source/reason: explicit continuation after attachment recovery; the original attachment remains available and needs no replacement upload.
Scope/risk/orchestration: medium-risk native single writer, one bounded independent read-only verifier. Only prompt entrance presentation and ordinal propagation changed; activation, persistence, execution lock and unrelated dirty work were preserved.

- Rechecked the local prompt component and stylesheet against the previously fetched immutable commit tree: Git blob SHA1s still match `4ff52c3355e5d59cd9f1daf39eaf6382bba2fa9a` and `96a1767a768fb64155881c86f5484d1b8331e280`. This validates those files, not the entire reference checkout.
- Added the reference 260ms cubic-bezier(0.22, 1, 0.36, 1) entrance and 65ms per-item delay. Opacity, a -12px-to-zero relative offset and a moving top-down clipping shell share the same progress. The outer layout box and real Button stay 32px high, so the reveal does not resize the control or move adjacent content. Reduced motion removes delay and travel. Stable mounted keys retain the clock across redraws.
- Three new tests cover delayed/full-duration timing, the sampled easing, reduced motion, clip/translation coupling and fixed button height. Current-source desktop test compilation passed against private cached third-party artifacts; **279 tests passed**, zero failures, in 1.80s. Normal and narrow application compilation/link also passed. These are not fresh locked Cargo/workspace dependency-validation receipts.
- The independent verifier found no scoped source defect after checking layout, width, timing, native semantics and keyed animation lifecycle. Static review is separate from rendered or interactive acceptance.
- Current native previews used the existing isolated no-key, no-environment-credentials, loopback-only fixtures. Read-only explicit-window-ID captures show four contained actions in the 1200x800 light Chinese 14px layout and a single overflowing strip in the 480x640 dark English 20px/reduced-motion layout. The narrow fixture differs from product source only in initial window bounds. Its capture contains draft text; no automated selection or keyboard input was performed, and this is not prompt-activation evidence.
- Final screenshots under `/Users/zhd/.codex/visualizations/2026/10/26/01a1764d-5a1f-74b2-8fa3-c0499328387b/`: `draft-waterfall-light-zh-14-settled-20261115.png` and `draft-waterfall-480x640-dark-en-20-reduced-20261115.png`. These are settled-state captures with native window margins, not matched-scale pixel comparisons or frame-by-frame motion evidence.
- Source SHA256: `workbench.rs` `a597e1fa8145115a4ae701d259ce89deddecef313e8f770f25ff1b2e3c76e244`; `draft_prompts.rs` `57654ed0466ad654b54ecfd84e4ffcf5e185830bd41b818d42340e4be78306f2`. Normal preview SHA256 `34759842a92e1d957af1bea96d9b7f89f417883ac4730aceebb1dc56a85392ac`; narrow preview `2e2215c155f0c2841b9f6e088859da7bf7b8cd2740783b17845448143484c268`. All remain private local artifacts.
- Owned previews were stopped by exact PID; existing user app instances and isolated draft state were preserved. Native focus traversal/offscreen reveal, click/wheel replay, animated hit regions, precise frame timing, live resize and matched-reference/full-product acceptance remain **PARTIAL**. No keyboard defect was reproduced or repaired by this increment. The full goal remains active; no commit, push or deployment was performed.

## Draft Prompt Focus And Save Replay - 2026-11-16

Parent: the waterfall checkpoint and unchanged full ZCode parity objective.
Source/reason: explicit continuation after attachment recovery; no replacement upload was requested.
Scope/risk/orchestration: medium-risk native single writer with one bounded read-only verifier. Prompt focus visibility and isolated interaction verification only; unrelated dirty work was preserved.

- Added a keyed horizontal scroll handle and non-tab-stop focus observers without replacing native Button activation. Post-layout focus entry reveals entirely offscreen actions; partially visible actions remain stationary. A capture-phase `prevent_default` prevents the observing ancestor from taking pointer focus without stopping native click propagation.
- Rechecked reference prompt, stylesheet and Button Git blobs against the retained immutable-tree evidence: `4ff52c3355e5d59cd9f1daf39eaf6382bba2fa9a`, `96a1767a768fb64155881c86f5484d1b8331e280`, `89ca4a98780506f5e89a900bd7129d003921fd5b`. The preceding actual-component browser replay at 480x640/dark English 20px kept the third, partly visible action at scrollLeft 0 and moved the fourth to scrollLeft 180. No matched-scale full-product pixel comparison is claimed.
- Native computer-use interaction became available. Actual Tab/Return replay proved horizontal reveal and insertion of the complete Idle tasks prompt. Configure opened Model providers, Save Preferences showed Settings Updated beside the contained button, and the first return preserved the draft. Structured inspection of the dedicated no-key fixture confirmed the persisted provider route and draft.
- That replay also exposed a new vertical focus defect: the late-night English greeting and three-line draft pushed the strip entirely below the 480x640 viewport. Pointer wheel could reach it, but Tab left the focused action invisible. Added a separate empty-chat scroll handle and deferred vertical reveal. Requests cancel if focus leaves or either captured offset changes; retaining focus does not repeatedly override manual scrolling.
- The verifier found that measuring the animated clip could leave a button offscreen after its entrance finished. The observer now measures an unanimated 32px slot containing the waterfall clip. Two added regressions cover outer vertical bounds/range clamping and fixed measurement at entrance progress 0, 0.25, 0.5 and 1. The same verifier confirmed closure of its P2 with no other scoped finding.
- Final current-build native replay, dark English 20px/reduced motion: five Tabs from the composer revealed the off-bottom Weekly report action; the third stayed partly horizontally visible without a jump; the fourth scrolled into view. Manual upward wheel kept the strip offscreen until Shift+Tab entered another action, which revealed it again. Return on Make slides inserted the complete presentation prompt. Configure and Save were replayed successfully. The final Back action was rejected by the computer-use tool because the Mac had locked; that final return is not counted as passed.
- The fixture remains isolated under `/tmp/codex-native-dismiss-20261115.F4j9qO/focus-ui-state-20261116/`: no credentials, environment credentials disabled, loopback-only provider, zero messages and zero tokens. Its final persisted draft is `Generate a complete presentation outline and slide contents based on the project topic.` No Send action or provider request was performed.
- Project-volume compilation was interrupted with SIGINT after sampling showed blocked module-path `stat` calls; a narrow build was likewise interrupted during native search-directory reads. These are not passes. Copied the current desktop source to a private local snapshot, verified it with a recursive comparison, restored its relative JSON-fixture/asset layout, and omitted unnecessary native search paths because the cached rlibs already bundle their native objects. Initial relocated test attempts failed on missing relative fixture/assets, not product diagnostics. Final snapshot tests and both normal/narrow application builds succeeded: **283 tests passed, zero failed, in 1.91s**. This remains cached-dependency current-source evidence, not fresh locked Cargo/workspace validation.
- The final narrow snapshot differs from the current normal snapshot only in initial bounds (1200x800 to 480x640). Two older queue modules in the private fixture were refreshed from the current source; product queue code was not changed by this increment. Scoped rustfmt and whitespace checks passed. Owned native previews were stopped by exact PID and exited 143; other app instances were untouched.

Final source SHA256: `workbench.rs` `992c2c378b2949fc820f60b781153ae88e35280bd88138a23bbbf5ea55e38b97`; `draft_prompts.rs` `12bec7abbb093fba023abce855d55d05e5ed5d1e2d5bd48263b4fdafb322798e`. Private tests `36950e75180eb772d438ce3a85f8cf74139670fcdd7fb28bd39a18f1ee40018c`; normal application `91f100f0cda3d39e764060bddda82539015ed48f733952728bd0e9db905ff7b7`; narrow application `f8d810d9b85d7593236c53d8d160089c83cfb64791cc4300ceadcad5459f8c33`.

Captured and visually inspected PNGs (native capture margins included):

| Screenshot | SHA256 |
| --- | --- |
| `/tmp/prompt-focus-offscreen-before-20261116.png` | `06dbfef3efe17d819d89706bb3eeeda7b7bafae42581cdaa338adedd6e048ef9` |
| `/tmp/prompt-focus-dual-axis-after-20261116.png` | `3b939fe902279e90e2dba73a2317be53b343a30ed7c1503e5ff6e5850115deaa` |
| `/tmp/provider-save-feedback-480x640-20261116.png` | `c114bf5baea85eef33e1f42d8dcb60061d90e70d833bc9eeb8e9d35deec78aea` |

The vertical reveal policy is native containment, not separately measured reference-browser vertical behavior. Focus during rendered entrance, animated hit regions, precise timing, live resize, disabled activation and matched-reference/full-product acceptance remain **PARTIAL**. The final post-save return also remains unverified due to the new screen lock. The full goal stays active; no commit, push or deployment was performed.

## Draft Greeting Geometry - 2026-11-16

Parent: the focus/save checkpoint and unchanged full ZCode parity objective.
Source/reason: continued source comparison after original attachment recovery; no replacement upload is needed.
Scope/risk/orchestration: low-risk Native single writer, external=0. Greeting width, text insets and bottom margin only; composer, queue, prompt behavior and unrelated dirty work were preserved.

- Compared the retained pinned `ConversationDraftEmptyState` and `ConversationTimeline` sources. Their Git blob hashes matched the immutable commit-tree evidence before this increment: `b4b206c2f087767d733b9db1018cea2c405df639` and `7e8a75e6a0ae395e1712447ca5f03696d9473647`.
- Corrected the wide greeting margin from 49px to 32px; below the 640px viewport breakpoint it remains 40px. The breakpoint follows viewport width rather than the narrower sidebar/review-constrained conversation column.
- Greeting frame width is now `min(max(content_width - 32, 0), 672)`, preserving the timeline's fixed 16px gutters. Text rendering uses fixed 16px padding instead of rem-based `.px_4()`, and font measurement subtracts the same 32px. At 480px content width, the frame is 448px and available text width is 416px. Natural sizing, 20-30px limits, 1.2 line height, local-hour refresh and logo behavior are unchanged.
- Two new tests cover widths 480/639/640/wide, a constrained column within a wide viewport, very narrow bounds, both theme palettes and rem sizes 12-20. Current desktop source compiled against private cached dependency artifacts: **285 tests passed, zero failed, in 1.90s**. Both normal and 480x640 application compilation/link succeeded. These are not fresh locked Cargo/workspace-validation receipts.
- Added `view=draft-empty` to the diagnostic actual-component harness. Its wrapper supplies timeline gutters and column cap, not the full timeline spacer/composer. Restored only the reference's pinned private `zustand@5.0.12` package and verified its tarball SHA512 against the reference lockfile; no product manifest or lockfile was changed by this increment.
- The reference page remained blank while Vite's optimizer reported dependency bundling. HTTP 200 module transforms and an empty console-error result do not prove rendered parity. A later browser inspection timed out, so no reference geometry or screenshot acceptance is claimed. The temporary browser tab was closed and its viewport override reset.
- Native UI inventory confirmed that the Mac remained locked and automatic unlock was unavailable. No new native application was launched, and no screenshot or interaction acceptance is claimed for this increment. The preceding save-button replay remains separate historical evidence.

Source SHA256: `workbench.rs` `30e40dfa4b464b6c149cd16624f4e47fb5c23567f19e81e0296ed15d87aaf73e`. Private artifacts: tests `d9e962f2d77dc1a6a75bc1ecc94dd463d34df250babc908390643586ac0bb1e6`; normal app `1ad6aa30c7b01990b87b425e061ba06c99c2273ac37094e50bb7c1e4cab7df1b`; narrow app `7e070e88f6ea0e2676602432e91ba1c61c09689feabde9dd6fc25fcefa573550`.

Scoped rustfmt, tracked whitespace checks and comparison of the tested snapshot's `workbench.rs` with the workspace passed; the untracked harness check emitted no whitespace diagnostics. The owned Vite process did not exit promptly after exact-PID SIGTERM, then exited 137 after exact-PID SIGKILL; a process check confirmed it was gone. No owned compiler or native preview remained.

Matched-reference/native visual acceptance and the overall restoration remain **PARTIAL**. The full goal stays active; no commit, push or deployment was performed.

## Draft Dock And Composer Spacing - 2026-11-16

Parent: the greeting-geometry checkpoint and unchanged full ZCode parity objective.
Source/reason: continued actual-source comparison of the draft timeline and composer after original attachment recovery.
Scope/risk/orchestration: medium-risk Native single writer with one bounded independent read-only verifier. Draft grouping and composer surface geometry only; editor lifecycle, handlers, queue data and unrelated dirty work were preserved.

- Revalidated reference Git blobs against the retained immutable commit tree: `ConversationTimeline.tsx` `7e8a75e6a0ae395e1712447ca5f03696d9473647`; `ConversationComposer.tsx` `8b19443f9b8c7b63b0bcaf904690ea1d790e9f2f`; `ChatPromptEditor.tsx` `c9550fc16786b952543dc76f9e5493545998dbb0`; `SessionPane.tsx` `bcae9af6c03c6a1734285519a03d4e7509a52420`.
- The empty timeline now has one nonshrinking, natural-height bottom-dock column with the reference 12px top margin. Recovery/setup notices, queue, composer and suggested prompts retain their order and native handlers. Greeting and leading/trailing spacers remain outside it; the existing empty-conversation scroll handle continues to own overflow and prompt focus reveal.
- Composer dock and column no longer yield vertical height to flex shrinking. The input shell uses fixed 12px padding/gaps, 16px corners and clipped overflow, following the pinned editor shell instead of rem-dependent padding/gaps. Draft context-header padding is fixed 6px with wrapping, following the composer context wrapper rather than the former 12px inline inset. The existing editor entity, focus target, text sizing, hover/focus handlers and toolbar commands are retained.
- Three new tests cover natural-height dock geometry, fixed shell padding/gaps and wrapping context-header padding across rem sizes 12-20; shared composer-inset assertions now also check shrink resistance. Initial test compilation exposed a missing `LengthExt` test import and Option-valued margin conversion; these were corrected before rerunning.
- Final current-desktop-source tests compiled against private cached dependency artifacts: **288 passed, zero failed, in 1.68s**. Normal and 480x640 application compilation/link both succeeded. Recursive comparison confirmed that the normal test snapshot matches the workspace desktop source; the narrow snapshot differs only in initial window bounds. This is not fresh locked Cargo/workspace dependency-validation evidence.
- Independent source review found no actionable scoped defect. It confirmed that grouping preserves the existing outer scroll handle and that input-shell clipping does not contain the suggested prompts. It explicitly left composed short-window scrolling/focus with banners and a grown editor, plus narrow toolbar visibility under clipping, unproven. Property tests and source review are not rendered acceptance.
- Native UI inventory again reported a locked Mac with automatic unlock unavailable. No native app was launched and no screenshot, pointer, keyboard or resize replay is claimed for this increment. No reference browser/server was started. The original attachment remains recovered.

Final source SHA256: `workbench.rs` `f43dca7d3d89312078c84f1f2826ead7d7ef4f0bcbe28e896c4081e747ccffba`. Private artifacts: tests `4b24672a89b817dc6b8cdd3eca79dd2dd57fd38bfbd8951783479e70af13242f`; normal app `f470a56d16be4a2f1f6826299f2ad40830d83d3e769d1ca558eebc2e4ee8da53`; narrow app `e71165dc426f5b2351a8350f5ebbc067c721a3c28d1f314cc2f27a6da7d6ba79`.

Scoped formatting and whitespace checks passed. All owned compilers completed; no preview process was started. Whole-product source, visual and interaction alignment remains **PARTIAL**, with the original full objective active. No commit, push or deployment was performed.

## Recovery Content Column - 2026-11-16

Parent: the draft-dock checkpoint and unchanged full ZCode parity objective.
Source/reason: continued source comparison found that recovery still bypassed the draft cap and composer gutters.
Scope/risk/orchestration: low-risk Native single writer, external=0. Recovery placement only; status semantics, labels, button styling, draft restoration, dismissal and persistence handlers were preserved.

- Revalidated reference Git blobs against the retained immutable commit tree: `PendingCommandRecoveryBanner.tsx` `3a3ff5045cc97fdafe1e3ddfdf6dbc043b7e57a4`; `ConversationTimeline.tsx` `7e8a75e6a0ae395e1712447ca5f03696d9473647`; `SessionPane.tsx` `bcae9af6c03c6a1734285519a03d4e7509a52420`.
- Recovery now uses the existing composer column builder. Draft width caps at 672px after fixed outer 16px gutters; established-conversation gutters sit inside its responsive column. The wrapper and column each stay shrinkable horizontally and nonshrinking vertically, without clipping, new height limits or extra bottom spacing. The recovery surface retains its existing 12px bottom margin.
- Added a table-driven regression over 16 draft/conversation width cases and rem sizes 12-20. It checks exactly one 16px gutter per side and the resulting surface-width arithmetic at the minimum window, 639/640, 863/864, sidebar-constrained 932, 1279/1280 and wide content widths. For 480px content the surface width is 448px in both modes. These are builder/geometry assertions, not rendered flex-layout measurements.
- Current-desktop-source test compilation passed against private cached dependencies: **289 tests passed, zero failed, in 1.70s**. Both normal and narrow application compilation/link succeeded. Recursive comparison confirmed that the normal source snapshot matches the workspace; the narrow snapshot differs only in initial bounds (1200x800 to 480x640). This is not fresh locked Cargo/workspace dependency-validation evidence.
- Inspected the increment against private before-edit snapshots: only recovery placement and its tests changed. Scoped rustfmt and tracked whitespace checks passed. The new helper does not alter the native editor or the existing Resend behavior, which restores the pending text for user confirmation rather than submitting a provider request.
- Native UI inventory again reported the locked Mac and unavailable automatic unlock. No app/browser/server was launched. Actual wrapping, focus, button reachability and matched-reference screenshots remain unverified; previous captures do not validate this source increment.

Source SHA256: `workbench.rs` `c19ba8221eb37f38f1fddd3f70dafe16510e704510b558a6cc7d472bbbc25558`; `recovery.rs` `9533f99a34785ae58a33f16403344cc44dd2a12afc880435999ede000ea8b031`. Private artifacts: tests `a053b6c1bd9f09155ad3009d6b6770d43a72beaff997d6d42fc06cdc826687cf`; normal app `cbbeb55ce2c1fe36436d21fb725edc8508d874fca8dc4f4d21e131a7b0067dfa`; narrow app `5f53da7df4a6c6e26bead14813de6d610ef3de08a8a5506f50d5514cd23c60ce`.

All owned compilers completed, with no preview process started. The full restoration objective remains active and **PARTIAL**; no commit, push or deployment was performed.

## Composer Toolbar Geometry And Fit - 2026-11-16

Parent: the recovery-column checkpoint and unchanged full ZCode parity objective.
Source/reason: continued comparison of actual toolbar/button sources after original attachment recovery.
Scope/risk/orchestration: medium-risk Native single writer with one bounded independent read-only verifier. Toolbar geometry, typography and presentation fitting only; approval behavior, provider state, menu items, execution and persistence handlers were preserved.

- Revalidated the following reference Git blobs against the retained immutable tree: `button.tsx` `89ca4a98780506f5e89a900bd7129d003921fd5b`; `ChatPromptEditor.tsx` `c9550fc16786b952543dc76f9e5493545998dbb0`; `ChatPromptActionMenu.tsx` `63cfca932b435fa3676ec1e2ad1efe6ee87fbe1e`; `useComposerToolbarFit.ts` `6b61355165a94be9eb22432871446572a0947861`; `ConversationComposer.tsx` `8b19443f9b8c7b63b0bcaf904690ea1d790e9f2f`; `V4ComposerModeControls.tsx` `e3495e05452ffc870c8516f61be0d63b0821e02d`; `ModelConfigSelect.tsx` `7cce03e6eebfd0353322bdf94d1fb8106131a897`; `styles.css` `96a1767a768fb64155881c86f5484d1b8331e280`.
- Plus/send/stop now share the reference 28px frame, replacing 26px plus/send and 32px stop. Each icon is explicitly 16px with 8px corners and a transparent 1px border; the native queue action uses the same frame. Permission/model labels follow interface size and 1.5 line height; chevrons remain 14px. Usage text now follows the interface-relative small-text scale instead of fixed 12px. Existing light/dark send fills and disabled/executing conditions remain unchanged.
- The toolbar has a shrinkable leading allocation, nonshrinking control groups and a fixed 12px inter-group gap. Leading controls and the trailing submit-control contents each use fixed 4px gaps. The latter follows `ConversationComposer`'s `submitControlNode` gap, rather than incorrectly applying the enclosing editor trailing wrapper's 6px gap between its nested controls.
- Native shaping measures the same localized permission/model/usage strings at their rendered fonts. The fit budget subtracts column gutters and shell border/padding, including the draft cap; a full-width 480px viewport leaves 422px for the toolbar. Overflow first removes permission text/chevron, then uses the model package icon. Control IDs, focusable buttons, full-label tooltips and callbacks remain in place; fitting recomputes from full labels on each render rather than retaining a stale collapsed state. The existing native model-width caps and compact usage labels were deliberately retained, not claimed as reference-measured behavior.
- Five new tests cover rem-independent frame geometry, fixed group spacing, interface-size typography in both themes, exact-fit/collapse boundaries, expansion and usage/queue participation. Initial test compilation exposed an assertion type mismatch; the next run exposed a test attempting to inspect a private Button rounding property through base styles. Those test-only issues were corrected. Final current-desktop-source tests: **294 passed, zero failed, in 1.61s**, compiled against private cached dependencies. Normal and 480x640 app compilation/link both succeeded. This is not fresh locked Cargo/workspace dependency-validation evidence.
- Recursive comparison confirmed that the normal source snapshot matches the workspace desktop source. The narrow snapshot differs only in initial window bounds (1200x800 to 480x640). Scoped rustfmt and whitespace checks passed. Independent review found no actionable scoped defect, and confirmed the chrome arithmetic, explicit child typography and unchanged handlers.
- Native UI inventory still reported a locked Mac with automatic unlock unavailable. No application, browser or server was launched. Actual collapse thresholds, capped-label ellipsis at UI sizes 12-20, focus/menu continuity during resize and matched-reference screenshots remain unverified. Tests and independent source review do not establish rendered acceptance.

Source SHA256: `workbench.rs` `47ab2b337b95cd518eed2ec69058aa3bedf4fd4181ee0e67812f0bc6a9786bd1`; `composer_toolbar.rs` `568ef441658683321201b931a60b4c57beabc4bee29dcc24c3e651266caab731`. Private artifacts: tests `8fa4be05d68b4fce7a45e88d7647f35f7b9d5e897cf0898a0e29ae8029d3a741`; normal app `73669f637d1671012b57de874f61a4b0c4a6bc9f71a00f3e856029dd79b3d42c`; narrow app `61ce9101cee986d2826723f47bee1d73e865ef4f7403c52be672103daf23e189`.

All owned compilers completed; no preview process was started. The full restoration objective remains active and **PARTIAL**. No commit, push or deployment was performed.

## Natural Model Width And Project Trigger - 2026-11-16

Parent: the toolbar-geometry checkpoint and unchanged full ZCode parity objective.
Source/reason: continued comparison found that the retained native model caps differed from the reference's natural-width fit, and that the draft project trigger still used fixed 13px text and a non-pill frame.
Scope/risk/orchestration: medium-risk Native single writer with one bounded independent read-only verifier. Model fit and project-trigger presentation only; menus, execution, persistence, workspace selection and unrelated dirty work were preserved.

- Revalidated reference Git blobs against the retained immutable tree: `useComposerToolbarFit.ts` `6b61355165a94be9eb22432871446572a0947861`; `ModelConfigSelect.tsx` `7cce03e6eebfd0353322bdf94d1fb8106131a897`; `RollingToolbarLabel.tsx` `3bfaae95366131cfd4d76e982fb37668d52c5230`; `ComposerTriggerTransition.tsx` `c35ea96102258019d12be600ffec2b960e8d2372`; `ProjectComposerMenu.tsx` `a4c9065568d50432e1d5af82b6af6b1e40e4663d`; `WorkspaceShellLayout.tsx` `36ffea241837942f383577c5abe3dde58af3d066`; `SessionPane.tsx` `bcae9af6c03c6a1734285519a03d4e7509a52420`; `styles.css` `96a1767a768fb64155881c86f5484d1b8331e280`; `button.tsx` `89ca4a98780506f5e89a900bd7129d003921fd5b`.
- Removed the native 116/160/240px model caps. Fitting now starts with measured model text plus 34px chrome, preserves the whole label when it fits and uses the existing mode-first/model-second collapse sequence otherwise. The test case with a 334px model verifies full layout at 536px, mode-only compaction at 535px, and model compaction below 438px. Existing usage/queue accounting, button identities, native focus targets, full-label tooltip and callbacks are unchanged.
- The draft project trigger now has a fixed 28px height, 14px pill corners, transparent 1px border, 12px left / 8px right padding and reference 240px maximum width. The folder and chevron use fixed 16/14px sizes and subtle ink; primary label text follows interface size at 1.625 line height, with fixed 4px gaps and a shrinkable ellipsis label.
- Project hover/press is locally mapped to the existing surface-hover-equivalent token: 5% near-black in light mode and 10% white in dark mode. Ordinary ghost buttons retain their existing theme. The reference shell passes no trigger-style override, so the shared project-menu geometry is applicable to this draft header.
- Preserved the native path tooltip and existing `toggle_review` action. This increment does not implement the reference project directory/search/switch/detach menu, bound-project name or Git-branch control. Those functional differences remain open; matching a trigger's declared geometry is not proof of project-menu parity.
- Added three tests and updated existing fit tests: natural long-model sizing/compaction, project frame/insets/width limit, and localized primary text across UI sizes 12-20 in both themes. Final current-desktop-source tests compiled against private cached dependencies: **297 passed, zero failed, in 1.58s**. Both normal and 480x640 app compilation/link succeeded. This is not fresh locked Cargo/workspace dependency-validation evidence.
- Recursive comparison confirmed that the normal source snapshot matches the workspace desktop source. The narrow snapshot differs only in initial bounds (1200x800 to 480x640). Scoped rustfmt and whitespace checks passed; the increment was inspected against its private before-edit snapshots. Independent source review found no actionable scoped defect.
- Native inventory again confirmed a locked Mac with automatic unlock unavailable. No app, browser or server was launched. Actual natural-width/collapse thresholds, large-font clipping, hover/focus appearance, menu continuity during resize and matched-reference/native screenshots remain unverified. Source review and deterministic tests do not establish rendered acceptance.

Source SHA256: `workbench.rs` `a843957404db9965deef969155e0aa317c6e4d093eba1f35271a3267998c9156`; `composer_toolbar.rs` `35dbbec9a9275161f7cdec52a44c17241479846aae35cedaf2444cd025a26e12`. Private artifacts: tests `8300605f68908faa9522a450e067a2ff1c799a022483300e44497ad75a3ba23e`; normal app `fcbfa8f68b2270bd214d8fbf7a5ae0625f7181369b3c92722363d5fdfa577e6f`; narrow app `55f09029117dbeab01655f3774805ff83b16098a92a615b9726922fdfecb99d8`.

All owned compilers and validation commands completed; no preview process was started. The full restoration objective remains active and **PARTIAL**. No commit, push or deployment was performed.

## Local Project Menu And Workspace Ownership - 2026-11-16

Parent: the natural-model/project-trigger checkpoint and unchanged full ZCode parity objective.
Source/reason: direct continuation after attachment recovery; replace the project button's incorrect Review action with a real local project entry.
Scope/risk/orchestration: medium-risk Native single writer with one bounded independent read-only verifier. Project selection, workspace-owned entity replacement and directly related async/import guards only; unrelated dirty work was preserved.

- Reopened the recovered original `recovered-button-5b7e45a7.png` and confirmed its Save Preferences button; no replacement upload was requested. Revalidated pinned reference Git blobs against the retained immutable tree: `ProjectComposerMenu.tsx` `a4c9065568d50432e1d5af82b6af6b1e40e4663d`, `WorkspaceShellLayout.tsx` `36ffea241837942f383577c5abe3dde58af3d066`, `button.tsx` `89ca4a98780506f5e89a900bd7129d003921fd5b`, and `dropdown-menu.tsx` `de2ee49d003afc92b65766165d7c570e1ee7118c`.
- The trigger now displays the local folder name and opens a 288px above/start Popover with the existing searchable List, current-project indicator, full-path tooltips and native Open Folder action. Rows use fixed 8px inline/6px vertical padding, 6px corners, interface-sized 1.5 line-height text and a fixed 16px check area; their height accommodates UI sizes 12-20. Known roots are only the current folder and those visited in this window. Persistent named/multi-root projects, offline/remote availability, creation/edit/detach and branch controls remain open capabilities, not fabricated entries.
- A stable DesktopShell owns the active app and routes AgentConfig deep links to it. Selecting a different root reconstructs that root's Store, Agent, preferences, history and runtime in the same window and starts one new task without replacing its saved history. Selecting the current root also creates a new task. Process cwd is unchanged. Before leaving, the actual editor value is captured and the source store must save successfully; target read/save errors retain the source context. Known canonical aliases reuse the original path/store key.
- Source timers, pending debounce and subscriptions are stopped before releasing the old app. Only active-workspace timers run; a global cross-project scheduler is not implemented. Session-only API keys remain in a root-keyed, nonserialized in-memory cache, returning only to their own project. New roots retain the preexisting environment-credential policy. No credentials were logged or added to code, history, documentation or fixtures.
- Independent review found two P2s in the initial increment: A-to-B-to-A navigation or a completed run could revive a stale picker result, and the import guard missed local file reads/pending previews. Added navigation/run intent generations, checked at async completion and Shell commit, and extended both switch gates to importing and pending raw/preview state. Local file import now participates in the existing generation-aware fetching/cancellation lifecycle. The same verifier confirmed both fixes and found no direct cross-project credential issue in the final cache path. This is source review, not native event-order acceptance.
- Six new deterministic tests cover search/name/path matching, stale selection/run generations, read/import/preview gates, invalid directories, canonical aliases, new-task/history preservation and large-font row bounds. Final current-desktop-source compilation and tests passed against private cached dependencies: **303 passed, zero failed, in 1.83s**. Normal and 480x640 app compilation/link both succeeded. These are not fresh locked Cargo/workspace dependency-validation receipts.
- Recursive comparison confirmed the normal source snapshot matches the workspace desktop source; the narrow snapshot differs only in initial window bounds, 1200x800 to 480x640. Scoped rustfmt and whitespace checks passed, and the increment was checked against private before-edit snapshots.
- Native inventory again reported a locked Mac with automatic unlock unavailable. No app, browser, preview or server was launched. Picker cancellation, same/different-root switching, save-failure rollback, live background-event ordering, deep-link replay after switching, focus/keyboard/resize behavior and matched-reference screenshots remain unverified. No previous capture is reused as acceptance for this increment.

Source SHA256: `main.rs` `b7ef48e25f33eae5d486b31cc01dbc2ea2add1a23de077b36778bc1c948c3cf4`; `projects.rs` `cd7c1a7d60e89a7367d7566f931eb3df46e0098a66872e7fcd08e6003840e02b`; `interactions.rs` `3af9b972db0b0e74b6fd3e5924ed445c67bf87174c113c88a1bfbc1ecf1b1d25`; `workbench.rs` `b5a4375ae897d4e6731b03ab47d5cf17eeab164657e131d3eb654e3804edf212`. Private artifacts: tests `7abda1f7016c8f9afe7bc8e2ffeebb91d1962b354021287f6a05554532860192`; normal app `0a2ea35ff1042dc31c3012f59fcbde126807c2d9b133ef7a8e07038ef2c147a6`; narrow app `4247aad1db0f4cc819ba19ceb4c232f720cd0869e1e3fb5049a969bf66a065b0`.

All owned compilers completed; no preview process was started. The full restoration objective remains active and **PARTIAL**. No commit, push or deployment was performed.

## Interface Font Tokens And Message Measurement - 2026-11-16

Parent: the local-project-menu checkpoint and unchanged full ZCode parity objective.
Source/reason: direct continuation of the style comparison after recovering the original Save Preferences attachment; audit nondefault interface sizes without changing button frames or native handlers.
Scope/risk/orchestration: medium-risk Native single writer with one bounded independent read-only verifier. Font token arithmetic, composer leading, user-message height consistency and epilogue typography only; unrelated dirty work was preserved.

- Reopened the recovered original attachment without requesting another upload. Revalidated reference Git blobs against the retained immutable tree: `styles.css` `96a1767a768fb64155881c86f5484d1b8331e280`, `lib/mobileTextInput.ts` `2b2806651579c0f71d9a931c0e76e350c326238b`, and `v4/ConversationUserInputEpilogue.tsx` `c871c38be3d81da5a2daebc4373f4806e263dcf9`.
- Interface text now retains additive offsets from the selected 12-20px size, matching the reference tokens instead of scaling each font proportionally. Base-14 callers, including existing settings/button geometry, are unchanged. Intermediate native artifact-label sizes retain their original offsets; they are not claimed as named reference tokens.
- Audited every `ui_text` call and separated line-height uses from font-token arithmetic. Desktop composer leading is fixed at 20px, following `text-ui-base leading-5`. The user bubble retains its existing native 20/14 font-relative leading, now shared by shaping and rendering instead of measuring at 24/14. Six default 20px lines no longer falsely trigger the 120px-plus-1px overflow gate. Exact inherited reference body leading, font fallback and rendered wrapping remain unverified.
- Epilogue disclosure text now uses extra-small (interface minus 4px); its body uses small (interface minus 2px), the theme monospace family and relative 1.5 line height. The 12px chevron, 240px scroll limit, disclosure state and text splitting remain unchanged.
- Added three deterministic tests covering token offsets/default invariance, preference clamping, and line-height/collapse boundaries across sizes 12-20. Current-desktop-source tests compiled against private cached dependencies: **306 passed, zero failed, in 1.62s**. Normal and 480x640 app compilation/link both succeeded. These are not fresh locked Cargo/workspace dependency-validation receipts.
- Recursive comparison confirmed that the normal source snapshot matches the workspace desktop source. The narrow snapshot differs only in initial window bounds, 1200x800 to 480x640. Scoped rustfmt and whitespace checks passed; incremental diffs were inspected against private before-edit snapshots. Independent review found no actionable scoped defect and explicitly left real wrapping, fallback and clipping outside its evidence boundary.
- Native inventory still reported a locked Mac with automatic unlock unavailable. No app, browser or server was launched. Nondefault-font clipping, native focus/resize/disclosure replay and matched-reference screenshots remain unverified; tests and compilation do not establish visual acceptance.

Source SHA256: `desktop_state.rs` `b7b812f6826f80794cb6cccb28a4a1f45bb66ff2c4710f5dc7d0fc6946bed26f`; `workbench.rs` `d5300df8e7d7a06f331ac1aab63be1705c778cbf712311bd5b5c5971fab8dfd4`. Private artifacts: tests `c432de7a01c714eee4708c88dfa6c7c63977fe330ea83953500ee42bc283038d`; normal app `0e0c500973a698f4a2bf21dc108a2f64c7985ecae8a25b7cb40b30d2b5e4a33e`; narrow app `c11ef6ddb4f8d01d4b689a590cba84f2e978a74c12c39096fa87fa096369987d`.

All owned compilers completed; no preview process was started. The full restoration objective remains active and **PARTIAL**. No commit, push or deployment was performed.

## User Bubble Insets And Epilogue Control - 2026-11-16

Parent: the interface-font-token checkpoint and unchanged full ZCode parity objective.
Source/reason: continued comparison found rem-scaled bubble padding inconsistent with fixed-width shaping, and a pointer-only epilogue trigger.
Scope/risk/orchestration: medium-risk Native single writer with one bounded independent read-only verifier. User-message geometry/leading and epilogue trigger only; splitting, persistence, disclosure keys, copy/edit and unrelated dirty work were preserved.

- Revalidated pinned reference Git blobs against the retained immutable tree: `ConversationUserInputBubble.tsx` `eaa0f8aabd687609d77f801083237346b327c638`, `ConversationUserInputBody.tsx` `465fb28cfe286272664e5eb301c07faf97f9b009`, `ConversationUserInputEpilogue.tsx` `c871c38be3d81da5a2daebc4373f4806e263dcf9`, `button.tsx` `89ca4a98780506f5e89a900bd7129d003921fd5b`, `styles.css` `96a1767a768fb64155881c86f5484d1b8331e280`, `ConversationTimeline.tsx` `7e8a75e6a0ae395e1712447ca5f03696d9473647`, `SessionPane.tsx` `bcae9af6c03c6a1734285519a03d4e7509a52420`, `ConversationTurnGroup.tsx` `1aefe8150a0c41f4085f03e5e1de8552d512b0ab`, and `ConversationUserInputContent.tsx` `402b4a0c3a14bc6fe3175f29dd45d4441717d34b`.
- The bubble now uses fixed 16px inline/12px block padding and an 8px body/epilogue gap across interface sizes 12-20. Shaping derives its available width from those same padding and 1px border constants. Existing automatic bubble width, responsive 576px cap, alpha surface and asymmetric 12px/2px corners remain unchanged.
- Inspected the reference shell/session/timeline/turn/user-row chain and plain-text fragments; none adds body leading. The imported local Tailwind 4.2.2 preflight supplies unitless 1.5. Native plain user text and height measurement now both use 1.5, superseding the preceding retained 20/14 policy. At default 14px, five 21px lines fit and six exceed the unchanged 120px-plus-1px threshold. This is source-backed alignment, not rendered line-box measurement; mention-chip rendering, native masking and 300ms height-transition parity remain open.
- The existing native outline disclosure button now reserves 34x24px: fixed 16px icon plus 8px inline padding and 1px borders. Its expanded separation is fixed 8px. Existing tooltip, native focus target and click callback are retained.
- Replaced the epilogue's pointer-only div with an unstyled native Button, retaining its stable ID, localization, disclosure key and callback. It declares expanded accessibility state and uses the library's focus/keyboard activation behavior. Natural height, 4px gaps, 6px corners and foreground-only hover follow the reference; the icon inherits the foreground. Independent review found missing keyboard-visible focus in the initial version. Added primary ink plus the existing hover surface without geometry changes, explicitly documented as a native accessibility adaptation. The same verifier confirmed the source fix and found no direct regression.
- Added three tests for fixed insets/measurement agreement, disclosure frame and natural epilogue typography across themes, locales, states and UI sizes. Updated the paired line-height boundary regression. After the focus fix, final current-desktop-source tests compiled against private cached dependencies: **309 passed, zero failed, in 2.00s**. Normal and 480x640 app compilation/link both succeeded. These are not fresh locked Cargo/workspace dependency-validation receipts.
- Recursive comparison confirmed the normal source snapshot matches the workspace desktop source. The narrow snapshot differs only in initial bounds, 1200x800 to 480x640. Scoped rustfmt and whitespace checks passed; incremental source/design diffs were inspected against private before-edit snapshots.
- Native inventory again reported a locked Mac with automatic unlock unavailable. No native app, browser tab or server was launched. Actual line wrapping/clipping, disclosure pointer behavior, Tab focus visibility, Enter/Space single activation, expanded accessibility announcements and matched-reference screenshots remain unverified. Static style tests and source review do not replace those checks.

Source SHA256: `workbench.rs` `691fde3c6426bba65ed877d2acb2103f0a1d44ff409c510fdfd6d5a898e4c3dc`. Private artifacts: tests `b80577e8564a18ae999349be499389d831841400380c35f2ca401f306030d72e`; normal app `5456e286d2b4dc636c3ef96f9b5f634c8ab8a8fc596126ba4f41b8256180a00a`; narrow app `ef9fb0440522148e29b2983229da486094df14c4f3b5e41d89e804c4d324fd2e`.

All owned compilers completed; no preview process was started. The full restoration objective remains active and **PARTIAL**. No commit, push or deployment was performed.

## User Message Disclosure Continuity - 2026-11-16

Parent: the user-bubble-insets checkpoint and unchanged full ZCode parity objective.
Source/reason: continued comparison after recovery of the original Save Preferences attachment; fix disappearing collapse controls after reflow and missing height motion.
Scope/risk/orchestration: medium-risk Native single writer with one bounded independent read-only verifier. User-message disclosure presentation only; message data, text selection, splitting, stored keys, callbacks and unrelated dirty work were preserved.

- Reopened the recovered original attachment without requesting another upload. Revalidated `ConversationUserInputBody.tsx` Git blob `465fb28cfe286272664e5eb301c07faf97f9b009` against the retained immutable reference tree; the inspected source remains pinned to ZCode 3.16.2. This does not verify the entire reference checkout.
- Expanded messages now retain the collapse control even when a wider window makes their content shorter than the overflow threshold. Collapsing a now-short message removes that control. Known content always uses clipped max-height: 120px collapsed, or the greater of measured height and 120px expanded. Unknown measurement remains uncapped and readable.
- Added the reference 300ms max-height transition with cubic-bezier(0, 0, 0.2, 1), using the existing library's keyed target transition. The key includes session, message and content. Library source inspection confirms sampling from the current value on retarget/reversal and immediate settlement under system reduced motion; the app preference also supplies zero duration. These are source contracts, not native animation replay results.
- The collapse fade now spans the final 30% of the current clipped height, settling at the existing 36px. It remains a composited surface-color overlay, not the reference's true alpha mask. Pointer routing, selection compositing and matching rendered masks remain open.
- Independent review identified that resetting the motion channel on measurement failure could recrop already-expanded text when measurement returned. Added keyed measurement availability so the first recovered measurement settles immediately, with a stateful Some/None/Some regression. Compilation and review also caught the new keyed-state tuple's required ElementId conversion; it was corrected. The same verifier confirmed both fixes and found no direct regression.
- Six new deterministic tests cover reflow/control persistence, targets, sampled-height clipping styles, duration/easing policy, consecutive measurement recovery and proportional fade. Final current-desktop-source tests compiled against private cached dependencies: **315 passed, zero failed, in 1.76s**. Normal and 480x640 application compilation/link both succeeded. These are not fresh locked Cargo/workspace dependency-validation receipts.
- Recursive comparison confirmed the normal source snapshot matches the workspace desktop source. The narrow snapshot differs only in initial bounds, 1200x800 to 480x640. Scoped rustfmt and whitespace checks passed; the increment was inspected against private before-edit snapshots.
- Native inventory again reported a locked Mac with automatic unlock unavailable. No app, browser or server was launched. Actual wrapping/clipping, rapid reversal, resize continuity, reduced-motion changes during playback, keyboard/focus/accessibility behavior and matched-reference screenshots remain unverified. Earlier captures do not validate this increment.

Source SHA256: `workbench.rs` `da2d206ac346dd727a9ee5d1f473f3708637ec272a31443fadf44b625e3e53e5`. Private artifacts: tests `a8e28e6684e4261071cefa4bad25bf3b40bfe8b73d694bb93930a2b5b58cc92e`; normal app `ae220749dbb3997033138ea146cd385403650fbfe4e314b17e7740fff8b97cb8`; narrow app `d9a87a923bec9b07ded76ca706f6766ec6bea72376a03c304a7fe108f1045a6a`.

All owned compilers completed; no preview process was started. The full restoration objective remains active and **PARTIAL**. No commit, push or deployment was performed.

## Native Interaction Evidence And Source Recovery - 2026-11-16

Parent: the user-message-disclosure-continuity checkpoint and unchanged full
ZCode parity objective. Source/reason: preserve real native acceptance obtained
before a temporary source-volume outage; the volume has since returned.

- During the earlier live run, the normal and 480x640 builds above rendered
  in isolated HOME/state directories with inherited environment cleared, an
  empty API key and a synthetic loopback provider endpoint. No provider request
  was submitted.
- Save accepted a synthetic model edit, rendered localized success feedback,
  and passed an isolated disk assertion. In the 480x640 build, Tab from the
  model field visibly focused Save; Space activated it and persisted the edit.
- Native theme/language menus applied Light and English. Chinese and English
  provider instructions wrapped at 20px; the Save action and success feedback
  fit without overlap at the minimum window.
- In the wide app, expanding a message while Review narrowed its content,
  then closing Review, retained the Collapse control after reflow. Collapsing
  removed the unnecessary control once the body fit.
- Two durable screenshots, their candidate hashes and precise acceptance limits
  are archived in
  [the native interaction record](visual-parity/runs/zcode-3.16.2/native-interaction-20261116/README.md).
  These are prior-run observations, not a fresh replay of later source changes.
- The temporary app/source/fixture/state directories disappeared during the
  outage. After the source volume returned, the before-edit workbench hash
  matched `da2d206ac346dd727a9ee5d1f473f3708637ec272a31443fadf44b625e3e53e5`,
  and both retained PNG hashes matched their recorded values.
- This limited evidence supersedes the earlier locked-desktop statements only
  for those observed candidate states and interactions. Other pages, full
  keyboard/accessibility replay, motion timing, masks and matched reference
  pixel pairs remain unaccepted. Full restoration remains **PARTIAL**.

## User Message Disclosure Paint States - 2026-11-16

Parent: the native-interaction/source-recovery checkpoint and unchanged full
ZCode parity objective. Source/reason: direct continuation of the button-detail
comparison after recovering the original Save Preferences attachment.
Scope/risk/orchestration: medium-risk Native single writer with one bounded
independent read-only verifier. Disclosure paint, expanded accessibility state
and their focused tests only; unrelated dirty work was preserved.

- Revalidated the retained pinned reference sources for the user-input body,
  shared Button and stylesheet. The native disclosure now declares expanded
  accessibility state and uses the root background at rest, input/50 when
  hovered or expanded, and the ordinary 10% border strengthened to 15% only
  while hovered. Primary foreground, fixed 34x24px frame, 16px icon, padding,
  shadow, stable ID, tooltip and click callback are retained.
- Background and border use independently keyed 150ms color transitions with
  cubic-bezier(0.4, 0, 0.2, 1). Reduced motion settles immediately. A
  non-clickable, non-tab-stop parent observes pointer engagement without
  replacing the Button's native hover, focus or activation handlers.
- Independent review found missing transition imports and the component
  library's additional outline border mixing. Added the local imports and
  removed the outline flag while retaining the explicit 1px border. The same
  verifier confirmed both fixes and found no directly introduced regression.
  Compilation additionally exposed the component Button's missing stateful
  ARIA builder. A narrowly scoped borrowed adapter now sets expanded metadata
  on the original Button interactivity, without another rendering layer or
  replacing any handlers. The verifier confirmed that source-level path.
- Added two deterministic tests: paint across both themes and all
  hover/expanded combinations, and reduced-motion duration. Updated the
  existing fixed-frame test for the expanded-state argument.
- The initial repository-volume Cargo attempt was interrupted with exit 130
  after prolonged artifact-write stalls; sampling observed rustc waiting in
  filesystem writes. That attempt is not a pass. A local default-toolchain
  retry was also interrupted rather than mixed with the acceptance candidate.
- Created a private internal-volume source snapshot and copy-on-write clone
  of third-party build caches. Cargo.toml and Cargo.lock matched the workspace;
  recursive comparison confirmed all crate sources match after the final
  adapter fix. Explicit Rust 1.97.1 locked/offline Cargo validation passed:
  **3 focused tests**, then **317 desktop tests**, zero failed, and the
  application dev build. The complete desktop suite finished in 0.83s; the
  final application build finished in 44.61s. This is normal cached-dependency
  Cargo validation, not a clean-from-zero dependency rebuild. Cargo emitted
  the existing future-incompatibility notice for third-party `block` 0.1.6.
- Scoped rustfmt, repository whitespace and incremental diff checks passed.
- The system screenshot helper confirmed Screen Recording permission and
  captured the existing `Volt Private Rendering V13` window. That window
  belongs to a separate older test candidate and was not operated or used to
  accept this increment.
- Launched the current executable in a uniquely named private app bundle with
  cleared inherited environment, isolated HOME/state/workspace, no API key,
  no automations and a synthetic loopback endpoint. No prompt was submitted.
  Native captures show collapsed rest, hover with the retained tooltip,
  AXPress expansion revealing all eight fixture lines, and a visible native
  focus ring after an AX resize request to 480x640. AX geometry reported the
  same 34x24px button. The test process was closed; other windows were untouched.
- Four current-candidate captures and precise limits are archived in
  [the disclosure-paint record](visual-parity/runs/zcode-3.16.2/disclosure-paint-20261116/README.md).
  Keyboard Space injection did not establish a state change and is not a
  keyboard-acceptance pass. The macOS AX expanded query returned no value;
  source-level expanded metadata is not proof of a read-screen announcement.
  Dark/large-font current-candidate replay, measured motion/reversal,
  full keyboard navigation and matched-reference pixels remain open.

## Disclosure Keyboard And Dark Large-Font Replay - 2026-11-16

Parent: the user-message-disclosure-paint checkpoint and unchanged full ZCode
parity goal. Source/reason: direct continuation after original attachment
recovery; replace the inconclusive keyboard injection with bounded native
evidence and inspect the same candidate at a larger interface size.
Scope/risk/orchestration: Native single owner, evidence-only follow-up.
No production source, dependency or user state was changed.

- Reopened the recovered original PNG and confirmed the Save Preferences
  control. No replacement upload was needed. Existing Save evidence remains
  associated with its original candidate, not this disclosure candidate.
- Reverified workbench SHA-256
  `c40b9eff7ce912415c3002c4639b6cd992119f1ed58341a8f058200be7667b2d`
  and executable SHA-256
  `04730291d4efb69327f79726486698f620cc33efaaf47caac85d5c3175c707a7`.
  The preceding 317-test/build receipts still describe this unchanged source.
  The explicit Rust 1.97.1 locked/offline desktop suite was rerun after replay:
  **317 passed, zero failed, in 0.86s**. Tracked diff whitespace checks passed.
- Improved only the private diagnostic keyboard driver: activate its exact
  owned PID, raise that window, confirm it is frontmost before each event and
  send modifier-free HID key-down/up pairs. It refuses input to another app.
  In Light/English/14px, native AX focus followed by Space, Space and Enter
  produced collapse, expansion and collapse. Accessible button names changed
  each time; isolated state ended collapsed at revision 5.
- Relaunched the same candidate with only isolated fixture preferences changed
  to Dark/English/20px and reduced motion. At 480x640, rest/hover captures
  retained the 34x24px AX button bounds and tooltip. Space expanded all eight
  lines with a visible focus ring and no composer overlap; Enter collapsed
  them. Persisted state ended collapsed at revision 7.
- Archived and inspected four additional PNGs, dimensions and SHA-256 hashes in
  [the disclosure-paint record](visual-parity/runs/zcode-3.16.2/disclosure-paint-20261116/README.md).
  Both owned preview processes were stopped by exact PID, exit 143. The other
  independent preview was untouched. No prompt, network request, commit, push
  or deployment was performed.
- This is progress, not a full parity receipt. Tab traversal, offscreen
  keyboard reveal, VoiceOver expanded-state announcements, live reduced-motion
  switching, motion timing/reversal, masks and matched-reference pixels remain
  unaccepted. The AX expanded query still returns no value. Full restoration
  remains **PARTIAL** and the overarching goal remains active.

## Disclosure Actual Reference And Shadow Calibration - 2026-11-16

Parent: disclosure-keyboard/dark-large-font replay. Source/reason: direct goal
continuation; move from native-only captures to the pinned actual reference
component and correct a confirmed renderer-unit mismatch.
Scope/risk/orchestration: medium-risk Native single writer with one bounded
independent read-only verifier. Local disclosure shadow, focused test and
development-only reference fixture. Full parity scope is unchanged.

- Added `view=user-message` to the existing control harness. It imports the
  actual `ConversationUserInputBody`, uses copied Bubble classes and a shared
  eight-line synthetic JSON fixture. It does not stand in for the full bubble
  parser, mentions, attachment/epilogue pipeline or transcript.
- Matched eight retained reference-source Git blobs against the upstream tree
  of the fixed `3c5b227` commit before staging the harness manually in the
  archive. No Git identity gate was relaxed; the archive still lacks `.git`.
- Confirmed actual reference control geometry, 120px collapse, 70%-100% text
  mask, hover/expanded paint and 8px backdrop blur. A first 1x browser output
  and a too-narrow 424px diagnostic column were excluded from comparison.
  Final reference captures use a configured 2x context and width 425; reference
  natural bubble width is 424.140625px, native raster width 425px. Expanded
  text/bubble heights agree at 240/298px; this is not exact pixel parity.
- Reference CSS shadow blur 3/2px maps to GPUI shader sigma 1.5/1px. Replaced
  only this control's generic shadow helper, retaining offset, spread and
  black/10 ink. New regression test covers both layers in both expansion
  states. The native before/after crop changes are confined to its halo:
  886 pixels, maximum channel delta 3. No reference-pixel score is claimed.
- Fixed missing test-only imports found by the first compile. Explicit Rust
  1.97.1 locked/offline validation then passed four focused tests, all
  **318 desktop tests** (0.85s) and application dev build (7.64s). Workspace
  crates matched the private build snapshot. Existing third-party `block`
  warning remains. Control-only reference Vite build passed (2601 modules).
  Independent review found no actionable scoped defect.
- Captured new native rest, hover/tooltip and AXPress-expanded states from the
  updated signed executable. Guarded Space/Enter replay after AX focus also
  passed; isolated state ended at revision 10, expanded. Candidate identities,
  fixture equality, measurements, nine inspected PNG hashes and limitations
  are in [the component record](visual-parity/runs/zcode-3.16.2/disclosure-component-20261116/README.md).
- This continuation made concrete source and rendered-evidence progress.
  Backdrop blur, tooltip differences, width rounding, true masks, motion,
  complete focus/accessibility/selection replay and full-window matched
  reference acceptance remain open. Owned native previews, reference browser
  and server were closed; unrelated app instances and dirty work were
  preserved. No commit, push or deployment. Full restoration remains
  **PARTIAL**, with the overarching goal active.

Current workbench SHA-256:
`c40b9eff7ce912415c3002c4639b6cd992119f1ed58341a8f058200be7667b2d`.
Test executable SHA-256:
`6ac0ec05a8dee5d626c4d48687087c4944f1ba37f00e065f68a96e6498777eaf`.
Application SHA-256:
`04730291d4efb69327f79726486698f620cc33efaaf47caac85d5c3175c707a7`.
Full restoration remains active and **PARTIAL**. No commit, push or deployment
was performed.

## Save Hover Border Follow-Up - 2026-11-16

Parent: attachment Save-button restoration and actual-component checkpoint.
Source/reason: direct user continuation; recovered attachment identifies the
provider-page Save action. Scope is settings command paint only, with Native
single writer and one bounded independent read-only review.

- Measured native and pinned-reference Chinese 14px Save at 98x32px; retained
  content width, 10px padding, 16px icon, 4px gap and 8px radius.
- Reproduced an unintended bright outline on native hover. The component
  library uses Custom `color` as hover/active border color. Keep it transparent
  while the existing command frame supplies animated background/foreground.
  No handlers, persistence, geometry or focus semantics changed.
- Updated signed app screenshots confirm removal of the bright edge; adjacent
  fill is unchanged. Guarded keyboard Enter still saves and displays success.
  English 20px remains complete in a 480x640 window, with native AX width 195px
  versus reference DOM 194.21875px.
- Rust 1.97.1 locked/offline settings tests: **51 passed**, zero failed.
  Application build, scoped Rustfmt, recursive build-source comparison and
  diff whitespace check passed. Independent review found no actionable defect.
- Seven inspected captures, candidate identities and precise limits are in
  [the Save hover record](visual-parity/runs/zcode-3.16.2/save-hover-20261116/README.md).
  Rounded background clipping, fractional width and focus-ring differences
  are not accepted as exact parity. Full restoration remains **PARTIAL** and
  the overarching goal active. No commit, push or deployment.

## Save Padding-Box Follow-Up - 2026-11-16

Parent: Save Hover Border Follow-Up. Source/reason: direct user continuation
of the recovered attachment; correct remaining transparent-border clipping.
Scope/risk/orchestration: medium-risk settings command paint, Native single
writer with one bounded independent read-only verifier. Full scope unchanged.

- Added a noninteractive padding-box background inside the native command's
  uniform border, preserving Button activation, contents, focus and geometry.
  Save remains 98x32px, with 8px outer / 7px inner radius. Secondary and eye
  use 6px outer / 5px inner radius. Borderless/incomplete/asymmetric frames
  retain the prior native paint path; disabled opacity applies only once.
- Actual pinned reference confirms padding-box clipping. Dark rest, Light
  rest and Light hover straight-edge pixel samples match at 2x. This is
  sampled paint evidence, not whole-button or whole-window pixel parity.
- Guarded pointer edge-click still saved; eye activation and navigation
  worked. Disabled preview remained visually inert, with identical
  before/after images. Native theme keyboard selection switched to Light.
  AX focus plus guarded Enter saved successfully, showed feedback and
  persisted Light at isolated store revision 16.
- Two new regression tests cover action inset/radii, original-frame
  preservation, nonsemantic paint, fallback and radius clamping. Explicit
  Rust 1.97.1 locked/offline checks passed: **53 settings tests**, all
  **320 desktop tests**, and app dev build. Scoped Rustfmt, recursive
  private-source comparison, strict codesign and diff checks passed.
  Existing third-party `block 0.1.6` warning remains.
- Independent review found no actionable scoped defect. Disabled preview
  still exposes AXEnabled=1; cause is unresolved, not an accessibility pass
  or a proven regression. Focus ring, fractional text width, complete
  state/font/theme matrix and full-window acceptance remain open.
- Candidate hashes, 14 inspected PNGs, exact samples and limitations are in
  [the padding-box record](visual-parity/runs/zcode-3.16.2/save-padding-20261116/README.md).
  Owned native/reference processes were closed; unrelated instances and
  dirty work preserved. No commit, push or deployment. Full restoration
  remains **PARTIAL** and the overarching goal stays active.

## Save Focus-Paint Follow-Up - 2026-11-16

Parent: Save Padding-Box Follow-Up. Source/reason: direct user continuation
of the recovered Save-button attachment; correct the reproduced focus halo.
Scope/risk/orchestration: medium-risk settings command paint, Native single
writer with one bounded independent read-only verifier. Full scope unchanged.

- Rechecked the durable original attachment and its SHA-256; the old temp
  path expired again, but no replacement upload is needed.
- Pinned global focus rules and actual browser Tab focus confirm no extra
  outline/shadow on the shared reference Button. Disable only the native
  library's extra settings-command ring, keeping focus handles, Tab traversal,
  activation, disabled behavior, paint and geometry otherwise unchanged.
- Native API-eye focus followed by two real guarded Tabs reaches Save.
  Updated Light Chinese 14px removes the reproduced halo and Space saves.
  Actual field/dropdown interaction selects Dark English 20px; at 480x640,
  the same traversal and Enter save with the full label and adjacent success
  feedback contained. Isolated revision 20 persists English/Dark/20px.
- Rust 1.97.1 locked/offline: **54 settings tests**, all **321 desktop tests**
  and application build passed. Regression coverage checks the ring flag and
  unchanged geometry across command kinds, themes and disabled states.
  Scoped Rustfmt, strict codesign, recursive source comparison and diff check
  passed. Independent review found no actionable scoped defect.
- Seven inspected captures, candidate identities and exact limits are in
  [the focus-paint record](visual-parity/runs/zcode-3.16.2/save-focus-20261116/README.md).
  This is not full accessibility or forced-colors acceptance. AX disabled
  announcements, fractional text width, motion and full-window matched
  reference acceptance remain open. Full restoration stays **PARTIAL** and
  the overarching goal active. No commit, push or deployment.

## Select Menu Row Follow-Up - 2026-11-16

Parent: Save Focus-Paint Follow-Up. Source/reason: direct continuation of
the attached settings-control comparison. Scope is native Select menu row
geometry, with the full ZCode visual-parity objective unchanged.

- The pinned ZCode menu uses fixed 4px vertical padding. At 20px interface
  text, GPUI Component 0.7.3's inherited `py_1` adds 1px per side, making
  each row 42.5px instead of the reference 40.5px.
- Added a local `-1px` vertical margin to the menu label only. The native
  Select still owns the trigger frame, popup, focus, selection and keyboard
  behavior. Added regression assertions for both compensating margins.
- Main-tree verification passed: all **321 desktop tests**, Release build,
  Rustfmt and `git diff --check`. Release binary SHA-256:
  `ec1b4b3c210bfb0ba51903bd08b7aa5ebb74a82d4d6b13269ff0e3146a87307c`.
- The final current-build GUI replay remains **PARTIAL**: the macOS session
  returned `CGSSessionScreenIsLocked=true`, so no current-build screenshot
  or menu interaction is claimed. Existing dirty work was preserved. No
  commit, push or deployment.

## Select Menu Responsive Padding Follow-Up - 2026-11-17

Parent: Select Menu Row Follow-Up. Source/reason: continue the attached
settings-control comparison and close the remaining interface-font-size edge
case without replacing the native Select.

- Replaced the fixed `-1px` menu-label margin with a calculated compensation:
  `4px - 0.25rem`, using the active interface rem size. This keeps the
  gpui-component 0.7.3 outer row at the ZCode fixed 4px vertical padding from
  12px through 20px, while preserving the outer selection background, hitbox,
  check icon, popup anchoring and keyboard ownership.
- Added coverage for every interface font size from 12px through 20px and
  confirmed the localized labels remain non-focusable content.
- Verification passed: the focused Select tests (**8 passed**), all **321
  desktop tests**, Release build, Rustfmt, `git diff --check`, and strict
  codesign. Current Release binary SHA-256:
  `b7d5df7b0adf270b6d7287e5bc776ba5bf8444ac23bae1cedf0f9039e2cba71e`.
- The screen remains locked, so current-build GUI replay and screenshot/menu
  interaction are still **PARTIAL**. No commit, push or deployment.

## Select Menu Check Alignment Follow-Up - 2026-11-17

Parent: Select Menu Responsive Padding Follow-Up. Source/reason: direct
continuation of the attached settings-control comparison; align the selected
row's trailing check with the pinned ZCode component while retaining the
native Select implementation.

- Changed only the settings Select delegate. `SettingsChoices` is now a
  dedicated delegate with a local `render_item` hook, so selected settings
  rows draw a 16px check icon in the native trailing slot. The outer row,
  selection background, hitbox, popup lifecycle, focus and keyboard handling
  remain owned by gpui-component 0.7.3. No dependency fork or Select rewrite
  was introduced.
- Added regression coverage for the dedicated delegate, localized value
  lookup, label accessibility role, and stable row geometry.
- Verification passed: focused Select tests (**9 passed**), all **322
  desktop tests**, Release build, Rustfmt, `git diff --check`, and strict
  codesign. Current Release binary SHA-256:
  `e9dc919cdea695ecd452d3bf332d61d2791402f2ece1a7b66757eb14f15f3326`.
- The macOS session remains locked, so current-build dropdown screenshots,
  pointer replay and matched ZCode/native pixel comparison remain
  **PARTIAL**. No commit, push or deployment.

## Select Menu Check Color Follow-Up - 2026-11-17

Parent: Select Menu Check Alignment Follow-Up. Source/reason: continue the
attached settings-control comparison against the pinned ZCode `SelectItem`.

- Matched the selected check's semantic color to the reference
  `foreground-subtle` token through the projected native theme. The 16px
  geometry, trailing-slot overlay, row wrapper, selection state and keyboard
  ownership are unchanged.
- Verification passed: focused Select tests (**9 passed**), all **322
  desktop tests**, Release build, Rustfmt, `git diff --check`, and strict
  codesign. Current Release binary SHA-256:
  `d7a215910eee53d2f82556fd2530b053c5ecfd4c526ca70ad4cc5647ec8038c4`.
- The current GUI acceptance boundary is unchanged: the screen is locked, so
  dropdown screenshots, pointer replay and matched ZCode/native pixel
  comparison remain **PARTIAL**. No commit, push or deployment.

## Select Menu Check Alignment Continuation - 2026-11-17

Parent: Select Menu Check Color Follow-Up. Source/reason: continue the
settings Select implementation after the final delegate compile check.

- Fixed the local menu-label `ElementId` to use a valid string identifier
  while retaining the native Select trigger, popup, selection and keyboard
  ownership.
- Kept the production Git review timeout at 15 seconds and made the test
  harness window 60 seconds so concurrent desktop tests do not create false
  review failures. The package script now has executable permissions.
- Verification passed: settings tests (**55 passed**), all **408
  workspace tests**, strict Clippy, Release builds for all four binaries,
  Rustfmt and `git diff --check`. Current `volt` SHA-256:
  `57a21c98f175c2bbe1553514bdf16964dee3008cc0bd1b5f9711d0318a0a26c6`.
- The locally packaged app passed ad hoc strict codesign verification. The
  current local archive SHA-256 is
  `4be24773c0fde464d7ab483874e61511b06b7738265b9b12d501e3a1e4a0d226`.
- The fresh GUI probe timed out and the preceding probe reported a locked
  screen. Current-build dropdown screenshots, pointer replay and matched
  ZCode/native pixel comparison remain **PARTIAL**. No commit, push or
  deployment.

## Current Select check and settings replay - 2026-11-17

Parent: Select Menu Check Alignment Continuation. Source/reason: the first
current-build dropdown replay exposed that the selected check was positioned
outside the clipped content slot even though the builder tests passed.

- `SearchableListDelegate::render_item` is wrapped by gpui-component 0.7.3
  inside a left content container with `overflow_hidden`. The previous
  `right: -20px` placement therefore clipped the custom 16px check. The
  settings delegate now keeps the check at the content slot's right edge
  (`right: 0px`), removes the production-only unused trailing-gap constant,
  and retains the native Select's popup, focus, keyboard and selection
  ownership.
- The current source passed `cargo fmt --all -- --check`, the focused Select
  suite (**9 passed**) and strict desktop all-target Clippy. The fresh Release
  candidate was built from the current worktree with SHA-256
  `1c44821c35fcf66accadeb994f64b54140efca377a802e5f4cb7884f4e532588`;
  the previous release warning for the test-only constant is gone. The
  third-party `block 0.1.6` future-incompatibility notice remains.
- The isolated bundle
  `/tmp/codex-native-gui-20261117-3/Volt Parity.app` used bundle ID
  `com.volt.native.parity.20261117.v3`, private HOME/state/workspace roots and
  no provider credentials. At the native 1200x800 logical viewport, Settings
  accepted a real 20px font change and Dark theme selection; Save visibly
  showed `配置已更新`. Opening the Theme menu showed the selected Dark row
  and its visible check. The API Key eye control showed a local fixture value,
  changed to `隐藏 API Key`, and restored masking/`显示 API Key` without a
  network request.
- This is current-build native interaction evidence for the tested Settings
  states. Exact 480x640 resizing, full keyboard traversal, IME, VoiceOver,
  matched reference pixels, Windows installation, real-provider/HTTP2
  lifecycle, signing/notarization and remote release remain outside this
  receipt. The owned preview was stopped after capture; no commit, push or
  deployment was performed.

## Narrow Settings replay - 2026-11-17

Parent: Current Select check and settings replay. Source/reason: the previous
current-build receipt had verified the Settings interaction at 1200x800 but
had not established the minimum content viewport.

- Rebuilt `pi-desktop` from the current worktree with
  `CARGO_BUILD_JOBS=2`, a private target directory, an explicit writable
  `TMPDIR`, `--offline` and `--locked`. The current `volt` executable SHA-256
  is `2a77c04f4ff3161f081ca93fc84717f05e989d65eceeb8483f69109d5427ab53`.
  The isolated bundle is
  `/tmp/codex-native-gui-20261117-v4/Volt.app` with bundle ID
  `com.volt.native.parity.20261117.v4`.
- macOS Accessibility set the candidate's content size to `480x640`. Core
  Graphics reported the complete window frame as `480x680`, which includes
  the 40px titlebar; the rendered client area is the declared
  `480x640` minimum. The Settings Appearance page rendered without clipped
  controls at that viewport.
- In the narrow Chinese replay, the Theme menu opened with all three choices
  visible and the selected row's check visible. Switching to Dark, entering
  `20` for the interface size and saving produced the dark large-font page
  with visible `配置已更新`. The capture was taken from isolated HOME,
  state and workspace roots with no provider credentials or model request.
  Window capture: `/tmp/codex-native-gui-20261117-v4/settings-dark-20-narrow.png`;
  SHA-256:
  `374067f06bde0c18d3df765bd75c9a7efde019f061bca00ba4a6c8ae5ee93bcb`.
- This closes the current-build minimum-content viewport and selected-check
  replay for Settings. Full keyboard traversal, native Chinese IME,
  VoiceOver, matched ZCode pixels, Windows installation, real-provider/HTTP2
  lifecycle, signing/notarization and remote release remain **PARTIAL**. No
  commit, push or deployment was performed.

## Settings keyboard control replay - 2026-11-17

Parent: Narrow Settings replay. Source/reason: continue the current-build
acceptance boundary after the minimum-content viewport and selected-check
receipt. This replay used the same isolated v4 bundle, private state roots and
no provider credentials.

- Accessibility exposed the Settings Appearance controls for Return,
  navigation sections, language, footer theme, appearance theme, interface
  font size, reduced motion and Save. Clicking the font field and pressing
  `Tab` reached the reduced-motion switch; a second `Tab` reached Save.
  Pressing `Return` on Save visibly produced `配置已更新`.
- The Appearance Theme popup opened with its three rows and selected Dark
  check. `Escape` closed it without changing the value; reopening it and
  pressing `Return` confirmed the selected Dark value and closed the popup.
  The footer language popup likewise exposed Chinese and English rows.
- The interface font field accepted `⌘A`, `20`, and `Return`; AX reported
  `Value: 20` and the narrow Settings page visibly reflowed to the larger
  text. The value was restored to `14` with the same keyboard path after the
  probe.
- This proves the local keyboard paths for the tested controls, but it does
  not prove a complete Settings Tab loop. After Save, the next Tab moved to
  the native Window sharing control, and the probe did not establish
  continuous traversal through the sidebar and footer controls. Native
  Chinese IME, VoiceOver announcements, matched reference pixels, Windows
  installation, real-provider/HTTP2 lifecycle, signing/notarization and
  remote release remain **PARTIAL**. No commit, push or deployment was
  performed.

## Settings focus-order follow-up audit - 2026-11-17

Parent: Settings keyboard control replay. Source/reason: direct continuation
after reviewing the reported post-Save Tab boundary.

- Source inspection confirms the Settings tree places the navigation column
  first (Return, sections and, at the wide layout, footer controls) and the
  selected section content second. The Appearance content ends with reduced
  motion and Save, so moving from Save to the next window-level control is
  consistent with the current native focus order. It does not by itself prove
  that the earlier navigation or footer controls were skipped.
- The pinned `gpui-component` Select implementation still registers its
  trigger with `track_focus(...tab_stop(true))`; `focus_ring(false)` only
  changes paint. No source defect was reproduced, so no speculative focus
  patch was added.
- A fresh v4 candidate created its 1200x800 window, but the current screen
  capture was entirely black and CUA returned `cgWindowNotFound`. The
  required replay from Return through the navigation, footer, content and
  Save could not be completed in this environment.

The reported post-Save transition is therefore reclassified as an
**unresolved acceptance boundary**, not a confirmed Settings focus defect.
The complete runtime traversal, IME, VoiceOver, matched reference pixels,
Windows installation, real-provider/HTTP2 lifecycle, signing/notarization
and remote release remain **PARTIAL**. No code, commit, push or deployment
was performed in this audit.

## Settings focus-order continuation — 2026-11-17

Parent: Settings focus-order follow-up audit. Source/reason: continue after
the desktop became partially accessible again and recheck the current
candidate without attributing evidence to an older package.

- The current v4 candidate remains
  `/tmp/codex-native-gui-20261117-v4/Volt.app/Contents/MacOS/volt` with
  SHA-256 `2a77c04f4ff3161f081ca93fc84717f05e989d65eceeb8483f69109d5427ab53`.
  Launching it produced a live process, but Core Graphics/System Events and
  CUA still exposed no window (`cgWindowNotFound` from CUA). The accessible
  `com.volt.native` window belonged to
  `/Volumes/Datas/tmp-codex-native-package-20261117/Volt.app` with a
  different executable hash, so its observations were excluded from current
  candidate acceptance.
- The source-level Settings tree remains navigation first, selected content
  second. No focus-order patch was justified by the available evidence.
- Current targeted verification passed: `cargo test -p pi-desktop settings::
  --offline --locked -q` (**55 passed**), `cargo fmt --all -- --check`, and
  `git diff --check`.

The complete current-candidate Tab traversal, IME, VoiceOver, matched
reference pixels, Windows installation, real-provider/HTTP2 lifecycle,
signing/notarization and remote release remain **PARTIAL**. No code, commit,
push or deployment was performed in this continuation.

## Current v4 Settings Tab traversal — 2026-11-17

Parent: Settings focus-order continuation. Source/reason: the CUA adapter
could not attach to the current v4 window even though Core Graphics exposed
it, so the same candidate was verified through macOS Accessibility
`AXUIElement` and Core Graphics keyboard events.

- The current v4 executable remained
  `/tmp/codex-native-gui-20261117-v4/Volt.app/Contents/MacOS/volt` with the
  recorded SHA-256
  `2a77c04f4ff3161f081ca93fc84717f05e989d65eceeb8483f69109d5427ab53`.
  Core Graphics exposed the live `ZCode` window, and the native accessibility
  tree exposed each Settings control and its internal focused state.
- Starting at Return, 15 Tab steps were observed in order:
  General, Appearance, Model providers, AgentConfig, Shortcuts, Subagents,
  Skills, Activity, footer language, footer theme, appearance theme, font
  size, reduced motion, Save, then Return. The loop therefore returns to
  Return after Save.
- The font field received a real key event (`14` became `141`) and was
  restored to `14`; reduced motion toggled on and back off; Return on Save
  produced the visible `配置已更新` feedback; the final values were restored.
- The current appearance Theme popup showed the selected `明亮` row and its
  visible check. The popup capture is
  `/tmp/codex-native-gui-20261117-v4/theme-open-current.png` with SHA-256
  `fffe95a600a0fe5aee55cbb20233264beb81e884df78f2297704e3595f096e79`.
  The Save capture is
  `/tmp/codex-native-gui-20261117-v4/tab-probe-save.png` with SHA-256
  `d337ae06f4b537ea6c914d49a048b2c9d81cdf1682ce4a9a3f8da6bcd5d550ce`.
  The textual probe receipt is
  `/tmp/codex-native-gui-20261117-v4/tab-probe-20261117.txt` with SHA-256
  `a936db557fb33089976efdf9ef037e9d48f2c7bdc76b59fe2f59c3d9b917a753`.

This closes the current-candidate Settings Tab traversal and selected-check
runtime boundary. Native Chinese IME, VoiceOver announcements, matched
reference pixels, Windows installation, real-provider/HTTP2 lifecycle,
signing/notarization and remote release remain **PARTIAL**. No code, commit,
push or deployment was performed.
