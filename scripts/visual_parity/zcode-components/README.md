# Conversation workflow component reference

This is a development-only reference harness. It imports the pinned ZCode
`WorkflowCardHeader`, `WorkflowTimeline`, theme CSS and providers directly.
It is not bundled into the Rust/GPUI application.

Run `python3 scripts/visual_parity/zcode-components/prepare.py /tmp/codex-native-zcode-reference`
from the native repository, then start Vite using the printed command from the
reference checkout (its dependencies must already be installed). Open port 5188
with `?theme=light` or `?theme=dark`.
Use `state=completed` for the unchanged `multi-agent.json` fixture and `width=788`
to match a measured native card width (the default remains 960px).

The native `multi-agent-running.json` fixture is adapted into timeline display
data. This does not exercise the reference runtime or its graph projection.
The wrapper copies the digest surface classes, but is not `WorkflowRunDigest`;
header actions are diagnostic placeholders. Do not use this harness as proof
of working Stop, child transcript navigation, collapse or full desktop parity.

Capture with reduced motion, wait for `document.fonts.ready` and the
`workflow-card-header` test ID, then save at 1180×760 and 480×760 for both themes.
Record source revision, fixture hash, viewport, theme, and these limitations.
The 2026-10-24 captures are under
`docs/visual-parity/runs/zcode-3.16.2/component-reference-20261024/`.

Observed header heights: 28px wide, 57px narrow. In the narrow reference the
right information cluster wraps and offscreen stages become edge lamps.
Native screenshots must use the same fixture and content width before any pixel
error is reported. Previous full-window screenshots cannot supply that proof.

## Shared controls

Open `/controls.html?theme=light&locale=zh-CN&font=14` for the pinned shared
Button, Input and Select implementations. `font` accepts 12-20, `locale=en-US`
uses the English labels, and `theme=dark` selects the dark palette.

The labels and grouping are diagnostic fixtures, not the reference Settings
page. In particular, the primary Save action models the native command using
the reference large Button; it does not assert that ZCode has a global Save
action. Buttons other than Select have no product handlers. This page supports
control geometry, typography, palette, disabled and native browser focus checks
only, not Settings persistence, full-page layout or whole-product acceptance.

Open `/controls.html?view=draft-prompts&theme=dark&locale=en-US&font=20`
for the actual pinned `ConversationDraftSuggestedPrompts` component and locale
provider. The four labels/icons match the native static fixtures; their selection
updates a hidden diagnostic output rather than a real composer or plugin service.
`disabled=true` exercises the disabled component. The wrapper reproduces the
draft timeline's 16px gutters and 672px column cap, not the full desktop layout.

Open `/controls.html?view=draft-empty&theme=dark&locale=en-US&font=20`
for the actual pinned `ConversationDraftEmptyState`, including its own text
measurement and logo. The wrapper preserves the timeline's 16px gutters and
672px empty-state column cap. Its fixed top padding is diagnostic: it does not
model the timeline's viewport spacer, toolbar, composer or scroll behavior.
Use this view only for greeting width, text padding, responsive bottom margin,
font sizing and logo measurements, not full-page position or desktop acceptance.

Open `/controls.html?view=user-message&theme=dark&locale=en-US&font=20&width=425`
for the actual pinned `ConversationUserInputBody` with the shared
`fixtures/user-message.json` plain-text message. `width` sets the diagnostic
conversation column (default 672px, clamped to 1-1200px and the available
viewport). The wrapper copies `ConversationUserInputBubble` classes and its
named container breakpoint; it does not import the full bubble's mentions,
attachments or epilogue. The actual body owns text measurement, mask,
expanded state, button, tooltip and animation. This supports component-only
geometry/paint/interaction comparisons, not full transcript layout, native
selection, persistence or whole-product acceptance.
