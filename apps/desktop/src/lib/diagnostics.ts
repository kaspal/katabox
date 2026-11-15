// "Copy diagnostics" (Settings → About): the recent error codes this window saw, and the
// plain-text report. Codes only: error messages can name files or users, so they are
// never kept. The report is in English on purpose; it is a bug-report format, like a log.
import type { Diagnostics } from "@/bindings";

export type RecentError = { at: string; code: string };

const MAX = 50;
const recent: RecentError[] = [];

/** Remember an error code (anything that is not a plain code becomes `unknown`). */
export function recordError(code: unknown): void {
  const c = typeof code === "string" && /^[a-z_]{1,40}$/.test(code) ? code : "unknown";
  recent.push({ at: new Date().toISOString().slice(0, 19) + "Z", code: c });
  if (recent.length > MAX) recent.splice(0, recent.length - MAX);
}

/** The recorded errors, oldest first (at most 50). */
export function recentErrors(): RecentError[] {
  return [...recent];
}

/** For tests. */
export function clearRecentErrors(): void {
  recent.length = 0;
}

/** Record uncaught errors in the window: the error code if it carries one, else `ui_exception`. */
export function watchWindowErrors(): void {
  const codeOf = (e: unknown) =>
    e && typeof e === "object" && "code" in e && typeof (e as { code: unknown }).code === "string"
      ? (e as { code: string }).code
      : "ui_exception";
  window.addEventListener("error", (e) => recordError(codeOf(e.error)));
  window.addEventListener("unhandledrejection", (e) => recordError(codeOf(e.reason)));
}

/** The text that is shown, then copied. */
export function diagnosticsText(d: Diagnostics, errors: RecentError[]): string {
  const lines = [
    "KataBox diagnostics",
    `app: ${d.app_version}${d.dev_data ? " (dev data)" : ""}`,
    `os: ${d.os} ${d.arch}`,
    `content: ${d.content_pack}`,
    `user schema: ${d.user_schema_version}`,
    `sandbox root: ${d.sandbox_root_source}`,
    `time scale: ${d.time_scale === null ? "not measured" : d.time_scale.toFixed(2)}`,
    `features: ${d.features.length ? d.features.join(", ") : "none"}`,
    "runtimes:",
    ...d.runtimes.map(
      (r) =>
        `  ${r.language}: ${r.installed ? "installed" : "not installed"}${r.version ? ` (${r.version})` : ""}, language server ${r.language_server ? "available" : "unavailable"}`,
    ),
    `recent errors (${errors.length}):`,
    ...(errors.length ? errors.map((e) => `  ${e.at} ${e.code}`) : ["  none"]),
  ];
  return lines.join("\n") + "\n";
}

/** Copy `text`: the clipboard API, or a hidden textarea and the copy command. */
export async function copyText(text: string): Promise<boolean> {
  try {
    await navigator.clipboard.writeText(text);
    return true;
  } catch {
    const area = document.createElement("textarea");
    area.value = text;
    area.setAttribute("readonly", "");
    area.style.position = "fixed";
    area.style.opacity = "0";
    document.body.appendChild(area);
    area.select();
    const ok = document.execCommand("copy");
    area.remove();
    return ok;
  }
}
