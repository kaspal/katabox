import { readFileSync } from "node:fs";
import path from "node:path";
import { describe, expect, it, vi } from "vitest";
import { screen, waitFor, within } from "@testing-library/react";
import { renderApp } from "@/test/renderApp";
import { setFakeApi, useFakeApi } from "@/test/mockApi";
import { resetAppReadyForTests } from "@/app/appReady";
import { SPLASH_LANGS, SPLASH_TEXT, pickLang, prefersReducedMotion } from "@/splash/text";

vi.mock("@/lib/api", async () => (await import("@/test/mockApi")).apiModule);

describe("Startup splash (ADR 0041)", () => {
  useFakeApi();

  it("has every line in the three UI languages", () => {
    for (const lang of SPLASH_LANGS) {
      for (const key of ["tag", "s1", "s2", "s3"] as const) expect(SPLASH_TEXT[lang][key]).not.toBe("");
    }
  });

  it("speaks the app's language, else the system's, else English", () => {
    expect(pickLang("es", "en-US")).toBe("es");
    expect(pickLang("pt-BR", "en-US")).toBe("pt-BR");
    expect(pickLang(null, "es-MX")).toBe("es");
    expect(pickLang(null, "pt-PT")).toBe("pt-BR");
    expect(pickLang("fr", "de-DE")).toBe("en");
    expect(pickLang(null, undefined)).toBe("en");
  });

  it("reduces motion for the app's setting or the system's", () => {
    expect(prefersReducedMotion("reduced", false)).toBe(true);
    expect(prefersReducedMotion("system", true)).toBe(true);
    expect(prefersReducedMotion("system", false)).toBe(false);
    expect(prefersReducedMotion(null, undefined)).toBe(false);
  });

  it("is a static page: no Tauri API, no inline script, no IPC", () => {
    const root = path.resolve(process.cwd());
    const html = readFileSync(path.join(root, "splash.html"), "utf8");
    const ts = readFileSync(path.join(root, "src/splash/splash.ts"), "utf8");
    expect(html).not.toMatch(/<script(?![^>]*\bsrc=)/);
    expect(html).not.toMatch(/\son[a-z]+=/);
    const code = [html, ts, readFileSync(path.join(root, "src/splash/text.ts"), "utf8")].map((src) =>
      src.replace(/<!--[\s\S]*?-->/g, "").replace(/^\s*\/\/.*$/gm, ""),
    );
    for (const src of code) {
      expect(src).not.toMatch(/from\s+["'](react|i18next|@tauri-apps|@\/lib\/api)|__TAURI|invoke\(/);
    }
  });

  it("tells Rust once that the main window rendered", async () => {
    resetAppReadyForTests();
    const fake = setFakeApi();
    const first = renderApp("/");
    await waitFor(() => expect(fake.api.appReady).toHaveBeenCalledTimes(1));
    first.unmount();
    renderApp("/");
    await screen.findByTestId("home-greeting");
    expect(fake.api.appReady).toHaveBeenCalledTimes(1);
  });

  it("Settings → Appearance → Startup animation saves the mode", async () => {
    const fake = setFakeApi();
    const { user } = renderApp("/settings?section=appearance");
    const group = await screen.findByLabelText("Startup animation");
    expect(within(group).getByRole("radio", { name: "After an install or update" })).toHaveAttribute("aria-checked", "true");
    await user.click(within(group).getByRole("radio", { name: "Never" }));
    expect(fake.api.setSetting).toHaveBeenCalledWith("app.splash", "never");
    await user.click(within(group).getByRole("radio", { name: "Every time" }));
    expect(fake.api.setSetting).toHaveBeenCalledWith("app.splash", "always");
  });

  it("reads a stored mode", async () => {
    setFakeApi({ settings: { "app.splash": "never" } });
    renderApp("/settings?section=appearance");
    const group = await screen.findByLabelText("Startup animation");
    await waitFor(() => expect(within(group).getByRole("radio", { name: "Never" })).toHaveAttribute("aria-checked", "true"));
  });
});
