#!/usr/bin/env bun

/**
 * Upload release artifacts to CTFile and render a Markdown download table.
 *
 * This script intentionally depends only on Bun's built-in fetch/FormData
 * implementation. CTFile credentials are read from the environment and are
 * never printed.
 */

import { basename, resolve } from "node:path";
import { createHash } from "node:crypto";

const API_BASE = "https://rest.ctfile.com/v1";
const PUBLIC_HOST = "url88.ctfile.com";
const RETRYABLE_STATUS = new Set([408, 425, 429, 500, 502, 503, 504]);

class CTFileError extends Error {}

type JsonObject = Record<string, any>;
type Metadata = {
  path: string;
  size: number;
  md5: string;
  sha256: string;
};

function isCtfileHost(hostname: string): boolean {
  const host = hostname.toLowerCase().replace(/\.$/, "");
  return host === "ctfile.com" || host.endsWith(".ctfile.com");
}

function canonicalUrl(value: unknown, kind: "file" | "folder"): string | undefined {
  if (typeof value !== "string" || !value) return undefined;
  let url: URL;
  try {
    url = new URL(value, `https://${PUBLIC_HOST}`);
  } catch {
    return undefined;
  }
  if (
    url.protocol !== "https:" ||
    url.username ||
    url.password ||
    url.port ||
    !isCtfileHost(url.hostname)
  ) return undefined;
  const pathPattern = kind === "file"
    ? /^\/(?:f|file)\/[A-Za-z0-9_-]+$/
    : /^\/dir\/[A-Za-z0-9_-]+$/;
  if (!pathPattern.test(url.pathname)) return undefined;
  const passcode = url.searchParams.get("p");
  url.search = passcode ? `?p=${encodeURIComponent(passcode)}` : "";
  url.hash = "";
  return url.href;
}

function uploadUrl(value: unknown): string {
  if (typeof value !== "string" || !value) {
    throw new CTFileError("CTFile did not return an upload URL");
  }
  let url: URL;
  try {
    url = new URL(value);
  } catch {
    throw new CTFileError("CTFile returned an invalid upload URL");
  }
  if (url.protocol !== "https:" || url.username || url.password || !isCtfileHost(url.hostname)) {
    throw new CTFileError("CTFile returned an unsafe upload URL");
  }
  return value;
}

async function digest(path: string, algorithm: "md5" | "sha256"): Promise<string> {
  const hash = createHash(algorithm);
  const file = Bun.file(path);
  const reader = file.stream().getReader();
  try {
    while (true) {
      const { done, value } = await reader.read();
      if (done) break;
      hash.update(value);
    }
  } finally {
    reader.releaseLock();
  }
  return hash.digest("hex");
}

async function metadata(path: string): Promise<Metadata> {
  const file = Bun.file(path);
  if (!await file.exists()) throw new CTFileError(`release asset not found: ${path}`);
  const size = file.size;
  if (size < 100) throw new CTFileError(`release asset is too small for CTFile: ${basename(path)}`);
  return {
    path,
    size,
    md5: await digest(path, "md5"),
    sha256: await digest(path, "sha256"),
  };
}

function delay(ms: number): Promise<void> {
  return new Promise((resolvePromise) => setTimeout(resolvePromise, ms));
}

class CTFileClient {
  constructor(private readonly session: string, private readonly retries = 3) {
    if (!session) throw new CTFileError("CTFILE_SESSION is required");
  }

  private apiFolderId(folderId: string): string {
    return folderId === "0" || folderId.startsWith("d") ? folderId : `d${folderId}`;
  }

  private parentFolderId(folderId: string): string {
    return folderId === "0" ? "0" : folderId.replace(/^d/, "");
  }

  private async request(endpoint: string, payload: JsonObject): Promise<JsonObject> {
    const body = JSON.stringify({ ...payload, session: this.session });
    for (let attempt = 0; attempt < this.retries; attempt += 1) {
      let response: Response;
      try {
        response = await fetch(`${API_BASE}${endpoint}`, {
          method: "POST",
          headers: { "content-type": "application/json" },
          body,
        });
      } catch (error) {
        if (attempt + 1 >= this.retries) {
          throw new CTFileError(`CTFile API request failed: ${error instanceof Error ? error.name : "network"}`);
        }
        await delay(2 ** attempt * 1000);
        continue;
      }
      let result: JsonObject;
      try {
        result = await response.json() as JsonObject;
      } catch {
        if (!response.ok && !RETRYABLE_STATUS.has(response.status)) {
          throw new CTFileError(`CTFile API request failed (${response.status})`);
        }
        if (attempt + 1 >= this.retries) throw new CTFileError("CTFile API returned invalid JSON");
        await delay(2 ** attempt * 1000);
        continue;
      }
      if (String(result.code) === "200") return result;
      const message = String(result.message ?? "");
      if (
        endpoint.includes("/folder/create")
        && (message.includes("already") || message.includes("存在"))
      ) return result;
      if (attempt + 1 < this.retries && RETRYABLE_STATUS.has(response.status)) {
        await delay(2 ** attempt * 1000);
        continue;
      }
      throw new CTFileError(`CTFile API rejected ${endpoint} (code ${result.code ?? "unknown"})`);
    }
    throw new CTFileError("CTFile API request failed");
  }

  async list(folderId: string, folders: boolean): Promise<JsonObject[]> {
    const endpoint = folders ? "/public/folder/list" : "/public/file/list";
    const entries: JsonObject[] = [];
    for (let page = 1; page <= 100; page += 1) {
      const result = await this.request(endpoint, {
        folder_id: this.apiFolderId(folderId),
        page,
        page_size: 100,
      });
      const batch = (result.results ?? result.data ?? []) as unknown;
      if (!Array.isArray(batch)) throw new CTFileError(`CTFile returned an invalid list for ${endpoint}`);
      for (const item of batch) {
        if (!item || typeof item !== "object") continue;
        const entry = item as JsonObject;
        const isFolder = entry.icon === "folder" || String(entry.key ?? "").startsWith("d");
        if (isFolder === folders) {
          entries.push({
            ...entry,
            default_passcode: entry.default_passcode ?? result.default_passcode,
          });
        }
      }
      if (batch.length < 100) return entries;
    }
    throw new CTFileError(`CTFile pagination limit exceeded for ${endpoint}`);
  }

  private static entryName(entry: JsonObject): string {
    return String(entry.name ?? entry.file_name ?? entry.folder_name ?? "");
  }

  private static entryId(entry: JsonObject): string {
    return String(entry.key ?? entry.file_id ?? entry.folder_id ?? entry.id ?? "")
      .replace(/^d/, "")
      .replace(/^f/, "");
  }

  async findOrCreateFolder(name: string, parentId: string): Promise<string> {
    const existing = (await this.list(parentId, true)).find((entry) => CTFileClient.entryName(entry) === name);
    if (existing) return CTFileClient.entryId(existing);
    try {
      const result = await this.request("/public/folder/create", {
        name,
        folder_id: this.parentFolderId(parentId),
      });
      const id = result.folder_id ?? result.id;
      if (id) return String(id).replace(/^d/, "");
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      if (!message.includes("already") && !message.includes("存在")) throw error;
    }
    const afterCreate = (await this.list(parentId, true))
      .find((entry) => CTFileClient.entryName(entry) === name);
    if (!afterCreate) throw new CTFileError(`CTFile did not create folder ${name}`);
    return CTFileClient.entryId(afterCreate);
  }

  private async uploadFile(folderId: string, path: string, size: number): Promise<string> {
    const current = (await this.list(folderId, false)).find(
      (entry) => CTFileClient.entryName(entry) === basename(path),
    );
    if (current) {
      const currentSize = Number(current.file_size ?? current.filesize ?? current.size);
      if (currentSize !== size) throw new CTFileError(`CTFile already contains a different ${basename(path)}`);
      const existingUrl = this.fileUrl(current);
      if (existingUrl) return existingUrl;
    }

    const ticket = await this.request("/public/file/upload", {
      folder_id: this.apiFolderId(folderId),
      checksum: await digest(path, "md5"),
      size: String(size),
      name: basename(path),
    });
    const form = new FormData();
    form.append("name", basename(path));
    form.append("filesize", String(size));
    form.append("file", Bun.file(path), basename(path));
    const controller = new AbortController();
    const timeout = setTimeout(() => controller.abort(), 60 * 60 * 1000);
    let response: Response;
    try {
      response = await fetch(uploadUrl(ticket.upload_url), {
        method: "POST",
        body: form,
        signal: controller.signal,
      });
    } catch (error) {
      throw new CTFileError(
        error instanceof Error && error.name === "AbortError"
          ? "CTFile upload timed out after 60 minutes"
          : `CTFile upload failed: ${error instanceof Error ? error.name : "network"}`,
      );
    } finally {
      clearTimeout(timeout);
    }
    if (!response.ok) throw new CTFileError(`CTFile upload failed (${response.status})`);
    let uploaded: JsonObject;
    try {
      uploaded = await response.json() as JsonObject;
    } catch {
      throw new CTFileError("CTFile upload returned invalid JSON");
    }
    const fileId = String(uploaded.id ?? uploaded.file_id ?? "");
    if (!fileId) throw new CTFileError("CTFile upload did not return a file ID");

    for (let attempt = 0; attempt < 5; attempt += 1) {
      if (attempt) await delay(attempt * 2000);
      const entries = await this.list(folderId, false);
      const entry = entries.find(
        (item) => CTFileClient.entryId(item) === fileId || CTFileClient.entryName(item) === basename(path),
      );
      if (!entry) continue;
      const remoteSize = Number(entry.file_size ?? entry.filesize ?? entry.size);
      if (remoteSize !== size) throw new CTFileError(`CTFile size verification failed for ${basename(path)}`);
      const url = this.fileUrl(entry);
      if (url) return url;
    }
    throw new CTFileError(`CTFile did not expose a canonical share URL for ${basename(path)}`);
  }

  private fileUrl(entry: JsonObject): string | undefined {
    const value = ["weblink", "share_url", "download_url", "url", "short_url"]
      .map((key) => canonicalUrl(entry[key], "file"))
      .find(Boolean);
    if (!value) return undefined;
    const passcode = entry.default_passcode;
    return passcode && !value.includes("?p=")
      ? `${value}?p=${encodeURIComponent(String(passcode))}`
      : value;
  }

  async publish(folderId: string, item: Metadata): Promise<string> {
    return this.uploadFile(folderId, item.path, item.size);
  }
}

function folderUrl(folderId: string, configured?: string): string {
  if (configured) {
    const checked = canonicalUrl(configured, "folder");
    if (!checked) throw new CTFileError("CTFILE_PUBLIC_ROOT_URL must be an HTTPS CTFile folder URL");
    return checked;
  }
  const normalized = folderId.startsWith("d") ? folderId : `d${folderId}`;
  return `https://${PUBLIC_HOST}/dir/${normalized}`;
}

function platformLabel(path: string): string {
  const name = basename(path).toLowerCase();
  if (name.includes("macos")) return "macOS arm64";
  if (name.includes("windows") && name.endsWith(".exe")) return "Windows x64 executable";
  if (name.includes("windows")) return "Windows x64 bundle";
  if (name.endsWith(".sha256") || name === "sha256sums") return "Checksums";
  return "Release asset";
}

function cell(value: string): string {
  return value.replaceAll("|", "\\|").replaceAll("\n", " ");
}

function renderReport(
  version: string,
  rootUrl: string,
  versionUrl: string,
  rows: Array<{ item: Metadata; url?: string }>,
  dryRun: boolean,
): string {
  const lines = [
    `## CTFile 国内下载（${version}）`,
    "",
    `CTFile 根目录：[codex-native](${rootUrl})`,
    "",
    "| 文件 | 平台 | 大小 | SHA-256 | 下载 |",
    "| --- | --- | ---: | --- | --- |",
  ];
  for (const { item, url } of rows) {
    const size = `${(item.size / 1024 / 1024).toFixed(1)} MiB`;
    const link = dryRun ? "待上传（dry-run）" : `[CTFile](${url})`;
    lines.push(
      `| \`${cell(basename(item.path))}\` | ${platformLabel(item.path)} | ${size} | `
      + `\`${item.sha256}\` | ${link} |`,
    );
  }
  lines.push(
    "",
    `版本目录：[打开 CTFile 版本目录](${versionUrl})`,
    "",
    "> CTFile 链接由发布流水线从 API 返回的 canonical 分享地址生成，并与 GitHub Release 中的 SHA-256 校验文件配套。",
    "",
  );
  return lines.join("\n");
}

function args(): { version: string; output: string; files: string[]; dryRun: boolean } {
  const values = Bun.argv.slice(2);
  const files: string[] = [];
  let version = "";
  let output = "";
  let dryRun = false;
  for (let index = 0; index < values.length; index += 1) {
    const value = values[index];
    if (value === "--version") version = values[++index] ?? "";
    else if (value === "--output") output = values[++index] ?? "";
    else if (value === "--file") files.push(values[++index] ?? "");
    else if (value === "--dry-run") dryRun = true;
    else if (value === "--help") {
      console.log("Usage: bun scripts/publish-ctfile.ts --version v0.1.0 --output dist/ctfile-links.md --file dist/asset");
      process.exit(0);
    }
  }
  if (!version || !output || !files.length) {
    throw new CTFileError("usage requires --version, --output and at least one --file");
  }
  if (!/^v\d+\.\d+\.\d+(?:[-+][0-9A-Za-z.-]+)?$/.test(version)) {
    throw new CTFileError("version must be a semver tag such as v0.1.0");
  }
  return { version, output, files: files.map((file) => resolve(file)), dryRun };
}

async function main(): Promise<void> {
  const options = args();
  const items = await Promise.all(options.files.map(metadata));
  const rootId = process.env.CTFILE_FOLDER_ID ?? "";
  const rootUrl = folderUrl(rootId || "0", process.env.CTFILE_PUBLIC_ROOT_URL);

  if (options.dryRun) {
    const versionUrl = rootUrl;
    await Bun.write(options.output, renderReport(
      options.version,
      rootUrl,
      versionUrl,
      items.map((item) => ({ item })),
      true,
    ));
    console.log(`CTFile dry-run validated ${items.length} release assets`);
    return;
  }

  if (!rootId) throw new CTFileError("CTFILE_FOLDER_ID is required");
  const retries = Number.parseInt(process.env.CTFILE_RETRIES ?? "3", 10);
  const client = new CTFileClient(process.env.CTFILE_SESSION ?? "", Number.isFinite(retries) ? retries : 3);
  const productId = await client.findOrCreateFolder("codex-native", rootId);
  const versionId = await client.findOrCreateFolder(options.version, productId);
  const versionUrl = `https://${PUBLIC_HOST}/dir/d${versionId}`;
  const rows: Array<{ item: Metadata; url: string }> = [];
  for (const item of items) {
    console.log(`Uploading ${basename(item.path)} (${item.size} bytes)`);
    rows.push({ item, url: await client.publish(versionId, item) });
  }
  await Bun.write(options.output, renderReport(options.version, rootUrl, versionUrl, rows, false));
  console.log(`CTFile published ${rows.length} release assets`);
}

try {
  await main();
} catch (error) {
  console.error(`CTFile publish failed: ${error instanceof Error ? error.message : String(error)}`);
  process.exit(1);
}
