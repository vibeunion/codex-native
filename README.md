# Codex Native (Native AI Coding Agent)

A blazing-fast, lightweight native desktop AI coding assistant with a configurable application identity.

---

## ⚡ Tech Stack Architecture

* **Desktop UI & Rendering**: Native GPU rendering engine (120 FPS hardware acceleration).
* **Agent Core**: Native autonomous runtime (`pi-agent-core`):
  * Full autonomous tool execution loop (`read_file`, `write_file`, `edit_file`, `bash`, `grep`, `find`, `office`, `calc`).
  * Built-in **10+ Native High-Performance Skills**: Office spreadsheet/table analytics, Word (.docx) unpacker & text extractor, presentation slide decks (Marp/Slidev), ASCII charting, scientific math evaluator, descriptive statistics, linear regression, linear algebra/matrix math, loan PMT / financial modeling, unit conversion, and calendar business days.
  * Streaming providers through an OpenAI-compatible Chat Completions endpoint.
  * Session history management and serialization.
* **Built-in Standalone CLIs (Pure Rust)**:
  * `office-cli`: High-speed tabular filtering, statistics, CSV/TSV <-> Markdown table conversion, docx text extraction, slides generation, and ASCII charts.
  * `calc-cli`: Scientific expression solver, sample statistics, bivariate linear regression, matrix operations, mortgage PMT & compound interest, unit converter, and business days calculation.
* **Ultra-Low Overhead**: Zero Electron, zero Node/V8 runtime overhead, native process startup without a bundled browser runtime. Performance depends on the workload and platform; no benchmark claims are made here.
* **No External Codex CLI**: Standalone native agent runtime without dependencies on npm or proprietary wrappers.
* **AgentConfig Bundle compatibility**: Import/export of VibeUnion `.acfg` bundles and `agentconfig://` deep-link payloads, with shared credential-free exchange and encrypted self backups.
* **Codex Harness compatibility**: Codex Harness is the default desktop runtime, with the same CodexStrict session contract plus a stdio JSON-RPC harness for external hosts; sessions support import/export/fork and Pi-native runtime forks.
* **Authentication**: API keys, ChatGPT OAuth, ZCode OAuth account authorization, and OAuth 2.0 PKCE for configured providers.

ZCode OAuth uses the browser login documented in the [ZCode CLI source](https://github.com/zai-org/ZCode/blob/main/apps/zcode-cli/docs/design/v2/auth-login.md). It authorizes a Z.AI account but does not configure model access in this app. For GLM model requests, configure a separate Coding Plan API key and its matching endpoint. Z.AI documents this API-key route for [OpenCode](https://docs.z.ai/devpack/tool/opencode) and [Pi](https://docs.z.ai/devpack/tool/pi).

### One-click brand configuration

The default display name is `Codex Native`. Override the identity without changing
source files:

```bash
APP_BRAND_NAME="Example Labs" cargo run -p pi-desktop
```

`APP_AGENT_NAME` defaults to `<brand> Agent`, and `APP_DESCRIPTION` defaults to
`Native AI coding desktop`. Both can be set explicitly at startup. These settings
apply to the window title, interface prompts, agent identity, and generated office
artifacts. The executable and internal module names remain stable for compatibility.

### Runtime build features

The desktop build enables both runtimes by default:

```bash
cargo build -p pi-desktop
```

For a single-runtime build, disable defaults and select one feature:

```bash
cargo build -p pi-desktop --no-default-features --features codex-harness
cargo build -p pi-desktop --no-default-features --features pi-native
```

When only one runtime is compiled, the Settings page removes the runtime
selector and persisted sessions/preferences are normalized to that runtime.

---

## 📁 Workspace Layout

```text
.
├── Cargo.toml                  # Cargo workspace definition
├── crates/
│   ├── pi-agent-core/          # Core agent runtime, tools & skills (headless)
│   │   ├── src/
│   │   │   ├── agent.rs        # Autonomous agent turn loop & event streaming
│   │   │   ├── bin/
│   │   │   │   ├── office_cli.rs # Standalone office-cli binary
│   │   │   │   └── calc_cli.rs   # Standalone calc-cli binary
│   │   │   ├── provider.rs     # Streaming LLM client
│   │   │   ├── session.rs      # Session state & persistence
│   │   │   ├── skills/         # Native skills registry & system prompt injection
│   │   │   ├── tools/          # Native tools: bash, fs, search, office, calc
│   │   │   └── types.rs        # Message, ToolCall, AgentEvent, ModelConfig
│   │   └── tests/              # End-to-end tool & skill integration tests
│   └── pi-desktop/             # Native desktop GUI application
│       └── src/
│           ├── i18n.rs         # Bilingual text (ZH/EN) & skill categories
│           └── main.rs         # Native desktop window, chat interface & skills dashboard
└── .github/workflows/
    └── build.yml               # Multi-platform CI pipeline (macOS + Windows)
```

---

## 🚀 Quick Start

### Prerequisites
* Rust toolchain (1.80+ or latest stable)
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Run Tests
```bash
cargo test --workspace
```

### Run Desktop App
```bash
cargo run -p pi-desktop
# 或者直接运行编译出的原生二进制文件：
./volt
```

---

## 🪟 Windows 自安装与系统集成 (Self-Installation)

Codex Native 提供了现代跨平台桌面工具标准的**免提权用户级自安装系统**（参考 VS Code User Setup、Rustup 与 Ollama 方案）：

### 1. 单文件自安装（二进制原生支持）
直接下载或分发单文件 `volt.exe`（技术兼容名称），无需外部安装包即可实现自部署：
```powershell
# 一键自安装至用户程序目录，配置桌面/开始菜单快捷方式与全局环境变量 PATH
.\volt.exe --install

# 纯便携运行模式（不写注册表与系统路径）
.\volt.exe --portable

# 查看当前安装状态
.\volt.exe --install-status

# 干净卸载（清理快捷方式、环境变量与文件）
.\volt.exe --uninstall
```

### 2. PowerShell 一键远程/本地快速安装
通过一行 PowerShell 命令自动安装并启动：
```powershell
# 本地脚本安装
.\scripts\install.ps1

# 远程一键部署
irm https://raw.githubusercontent.com/vibeunion/codex-native/main/scripts/install.ps1 | iex
```

### 3. 图形界面内一键修复与集成
打开 Codex Native 桌面端后，点击左下角 **设置 (Settings) -> Windows 系统集成与自安装**，即可随时一键执行自安装或修复快捷方式。

---

## 📜 License
GNU Affero General Public License v3.0 only (AGPL-3.0-only). See [LICENSE](LICENSE)
for the license notice and the complete terms at
https://www.gnu.org/licenses/agpl-3.0.html.

## Reproducible builds and releases

`Cargo.lock` is part of this application source. CI runs `cargo fmt --all -- --check`,
`cargo clippy --locked --workspace --all-targets -- -D warnings`, and
`cargo test --locked --workspace`. Use `--locked` when building release artifacts.

Pushing a `v*` tag runs the checks and builds, then publishes the Windows executable,
Windows bundle and macOS arm64 archive as GitHub Release assets. The PowerShell installer
resolves `latest` to one tag and checks the executable SHA256 before installation.
`-Version 0.1.0` selects tag `v0.1.0`. A remote installation requires a successfully
published release; local builds remain usable via `-LocalExePath`.

### 国内下载（CTFile）

每个正式版本都会在 GitHub Release 中同时发布 CTFile 国内下载表：

- [最新 Release（含 CTFile 下载地址）](https://github.com/vibeunion/codex-native/releases/latest)
- Release 资产中的 `ctfile-links.md` 保存每个二进制文件的 canonical CTFile 地址、大小和 SHA-256。
- CTFile 公共目录由仓库变量 `CTFILE_PUBLIC_ROOT_URL` 配置，形如
  `https://url88.ctfile.com/dir/d123456`；未设置时，流水线会根据
  `CTFILE_FOLDER_ID` 生成目录地址。

仓库管理员需要在 GitHub Actions Secrets 中配置 `CTFILE_SESSION` 和
`CTFILE_FOLDER_ID`。CTFile 上传或回读 canonical 分享地址失败时，该 tag 的
Release job 会失败，避免出现没有国内下载入口的半成品版本。当前仓库没有凭空写入
具体 CTFile 文件夹 ID；配置真实公开目录后，Release notes 和 `ctfile-links.md`
会自动显示可直接复制的文件地址。

下载量用于支持 macOS 代码签名、Apple notarization 和 App Store 开发者费用，
但 CTFile 下载次数不等于独立用户数。统计口径、预算和月度复盘见
[`docs/release-download-plan.md`](docs/release-download-plan.md)。

Tag builds require the repository secrets `MACOS_CERTIFICATE_P12_BASE64`,
`MACOS_CERTIFICATE_PASSWORD`, `MACOS_SIGNING_IDENTITY`, `APPLE_ID`,
`APPLE_TEAM_ID`, and `APPLE_APP_PASSWORD`. CI imports the Developer ID
certificate, signs the app and embedded binaries, submits the app to Apple's
notary service, staples the ticket, and verifies the stapled bundle before
publishing the archive. Local packaging stays unsigned unless
`MACOS_SIGNING_IDENTITY` is provided.

## Reports require source data

`office-cli report gen --file report.json --out report.md` renders supplied data without
inventing measurements, dates, target assessments or recommendations. The Agent's
`office` tool accepts the same JSON as its `report` property with `action: "report_gen"`.
Required input shape (illustrative data, replace with your actual data):

```json
{
  "title": "Delivery report",
  "author": "Engineering",
  "date": "2026-10-24",
  "summary": "Two completed deliveries recorded in the supplied dataset.",
  "kpis": [["Completed deliveries", "2"]],
  "table": {"headers": ["Project", "Deliveries"], "rows": [["Example", "2"]]}
}
```

Missing report fields, unknown query operators/columns, and invalid numeric array
members return errors. Statistical CLI input such as `1,invalid,3` is rejected in full.

## Provider request limits

The client allows 30 seconds for response headers, 60 seconds without stream data,
and 300 seconds for the whole request. Heartbeats do not extend the total deadline.
Timed-out or incomplete responses retain their received text and return an error.

SSE events support multiline data and UTF-8 split across network packets. Individual
events are limited to 4 MiB and the complete response to 16 MiB. Tool calls require
unique IDs and JSON object arguments, with at most 128 calls and 1 MiB of arguments
per call. HTTP error diagnostics are bounded and redact the configured API credential.

Provider names in configuration use this same Chat Completions transport; a native
Anthropic Messages or Ollama Chat adapter is not implemented. See
[`docs/project-review-status.md`](docs/project-review-status.md) for current acceptance evidence.
