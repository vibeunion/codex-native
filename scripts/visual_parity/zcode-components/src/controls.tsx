import React from "react";
import { createRoot } from "react-dom/client";
import { Check, Eye, Undo2 } from "lucide-react";
import "./controls.css";
import { Button } from "../../packages/ui/src/components/ui/button.js";
import { Input } from "../../packages/ui/src/components/ui/input.js";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "../../packages/ui/src/components/ui/select.js";
import { ZCodeIntlProvider } from "../../packages/ui/src/i18n/IntlProvider.js";
import { ConversationDraftSuggestedPrompts } from "../../packages/ui/src/v4/ConversationDraftSuggestedPrompts.js";
import userMessage from "./user-message.json";

const query = new URLSearchParams(location.search);
const requestedSize = Number(query.get("font") ?? 14);
const fontSize = Number.isFinite(requestedSize)
  ? Math.max(12, Math.min(20, Math.round(requestedSize)))
  : 14;
const english = query.get("locale") === "en-US";
const dark = query.get("theme") === "dark";
document.documentElement.lang = english ? "en" : "zh-CN";
document.documentElement.className = dark ? "theme-zai-dark" : "theme-zai-light";
document.documentElement.style.setProperty("--ui-font-size", `${fontSize}px`);

const DraftEmptyState = React.lazy(async () => {
  const module = await import("../../packages/ui/src/v4/ConversationDraftEmptyState.js");
  return { default: module.ConversationDraftEmptyState };
});

const UserInputBody = React.lazy(async () => {
  const module = await import("../../packages/ui/src/v4/ConversationUserInputBody.js");
  return { default: module.ConversationUserInputBody };
});

function UserMessage() {
  const requestedWidth = Number(query.get("width") ?? 672);
  const width = Number.isFinite(requestedWidth)
    ? Math.max(1, Math.min(1200, requestedWidth))
    : 672;
  return (
    <ZCodeIntlProvider initialLocale={english ? "en-US" : "zh-CN"}>
      <main
        className="flex min-h-dvh justify-center bg-background p-4 text-foreground"
        data-reference-commit="3c5b227649371507ddc4eb46ac5133afe6db1cab"
      >
        <div
          className="@container/conversation flex max-w-full flex-col items-end"
          style={{ width }}
          data-testid="user-message-column"
        >
          <div
            className="flex max-w-full flex-col gap-2 rounded-xl rounded-tr-xs border border-border bg-surface px-4 py-3 text-ui-base text-foreground @min-[624px]/conversation:max-w-xl"
            data-testid="user-message-bubble"
          >
            <React.Suspense fallback={null}>
              <UserInputBody contentText={userMessage.content} rowId={userMessage.id}>
                {userMessage.content}
              </UserInputBody>
            </React.Suspense>
          </div>
        </div>
      </main>
    </ZCodeIntlProvider>
  );
}

function DraftEmpty() {
  return (
    <ZCodeIntlProvider initialLocale={english ? "en-US" : "zh-CN"}>
      <main
        className="flex min-h-dvh flex-col items-center bg-background px-4 py-8 text-foreground"
        data-reference-commit="3c5b227649371507ddc4eb46ac5133afe6db1cab"
      >
        <div
          className="flex w-full max-w-2xl shrink-0 items-center justify-center"
          data-testid="draft-empty-column"
        >
          <React.Suspense fallback={null}>
            <DraftEmptyState />
          </React.Suspense>
        </div>
      </main>
    </ZCodeIntlProvider>
  );
}

function DraftPrompts() {
  const [selected, setSelected] = React.useState("");
  const items = [
    ["weekly-report", "周报总结", "Weekly report", "alarm-clock"],
    ["bug-fix", "报错修复", "Fix errors", "bug"],
    ["make-ppt", "PPT 制作", "Make slides", "presentation"],
    ["idle-tasks", "闲时任务", "Idle tasks", "moon"],
  ].map(([id, cn, en, iconName]) => ({
    id: id!,
    iconName,
    label: { cn, en },
    prompt: { cn, en },
  }));
  return (
    <ZCodeIntlProvider initialLocale={english ? "en-US" : "zh-CN"}>
      <main
        className="flex min-h-dvh items-start justify-center bg-background px-4 py-8 text-foreground"
        data-reference-commit="3c5b227649371507ddc4eb46ac5133afe6db1cab"
      >
        <div className="w-full max-w-2xl" data-testid="draft-prompt-column">
          <ConversationDraftSuggestedPrompts
            items={items}
            disabled={query.get("disabled") === "true"}
            onSelect={(item) => setSelected(item.id)}
          />
          <output data-testid="selected-prompt" className="sr-only">{selected}</output>
        </div>
      </main>
    </ZCodeIntlProvider>
  );
}

function App() {
  if (query.get("view") === "user-message") return <UserMessage />;
  if (query.get("view") === "draft-empty") return <DraftEmpty />;
  if (query.get("view") === "draft-prompts") return <DraftPrompts />;
  return (
    <main
      className="flex min-h-dvh flex-col items-start gap-8 bg-background p-8 text-ui-base text-foreground"
      data-reference-commit="3c5b227649371507ddc4eb46ac5133afe6db1cab"
    >
      <div className="flex max-w-full flex-wrap items-center gap-3">
        <Button size="lg" data-testid="primary">
          <Check />
          {english ? "Save Preferences" : "保存配置"}
        </Button>
        <Button variant="ghost" data-testid="secondary">
          <Check />
          {english ? "Use in task" : "用于任务"}
        </Button>
        <Button variant="ghost" size="sm" disabled data-testid="rewind">
          <Undo2 />
          {english ? "Rewind" : "撤销"}
        </Button>
        <Button
          variant="outline"
          className="h-7 gap-1.5 rounded-lg bg-input px-2 text-ui-base"
          data-testid="review"
        >
          {english ? "Review" : "审查"}
        </Button>
        <Button
          variant="ghost"
          size="icon-sm"
          aria-label={english ? "Show API key" : "显示 API Key"}
          data-testid="visibility"
        >
          <Eye className="size-3.5" />
        </Button>
      </div>
      <div className="flex w-48 max-w-full flex-col gap-1" data-testid="provider-field">
        <label htmlFor="endpoint" className="text-ui-base text-foreground-subtle">
          Base URL
        </label>
        <Input id="endpoint" size="lg" defaultValue="http://127.0.0.1:9/v1" />
      </div>
      <Select defaultValue={dark ? "dark" : "light"}>
        <SelectTrigger size="lg" className="w-48" aria-label={english ? "Theme" : "主题"}>
          <SelectValue />
        </SelectTrigger>
        <SelectContent>
          <SelectItem value="light">{english ? "Light" : "明亮"}</SelectItem>
          <SelectItem value="dark">{english ? "Dark" : "暗黑"}</SelectItem>
          <SelectItem value="system">{english ? "System" : "跟随系统"}</SelectItem>
        </SelectContent>
      </Select>
    </main>
  );
}

createRoot(document.getElementById("root")!).render(<App />);
