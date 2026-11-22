import { beforeAll, describe, expect, it, vi } from "vitest";
import { render, screen, waitFor, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import i18n from "@/i18n";
import { renderApp } from "@/test/renderApp";
import { setFakeApi, useFakeApi } from "@/test/mockApi";
import { PROBLEMS } from "@/test/fakeApi";
import { Markdown } from "@/workspace/problem/Markdown";

vi.mock("@/lib/api", async () => (await import("@/test/mockApi")).apiModule);

const HOSTILE = [
  "Read [the rules](https://example.invalid/track) first.",
  "![a tracking pixel](https://example.invalid/pixel.png)",
  "[click](javascript:alert(1))",
  "<img src=x onerror=alert(1)>",
  "<script>alert(1)</script>",
  "$\\href{javascript:alert(1)}{x}$",
].join("\n\n");

describe("problem packs (expansion M11)", () => {
  useFakeApi();
  beforeAll(() => void i18n.changeLanguage("en"));

  it("T7: pack text renders without links, images, scripts or URLs", () => {
    const { container } = render(<Markdown text={HOSTILE} trust="pack" />);
    expect(container.querySelector("a, img, script, iframe")).toBeNull();
    // No element points anywhere: raw HTML and KaTeX's \href stay inert text.
    expect(container.querySelector("[href], [src], [onerror], [onclick]")).toBeNull();
    expect(container).toHaveTextContent("<img src=x onerror=alert(1)>");
    expect(container).toHaveTextContent("Read the rules first.");
    expect(container).toHaveTextContent("a tracking pixel");
    // Built-in text keeps its links.
    const app = render(<Markdown text="Read [the rules](https://example.invalid/track)." />);
    expect(app.container.querySelector("a")).not.toBeNull();
  });

  it("previews an import with what the pack is, installs it, and offers a restart", async () => {
    const user = userEvent.setup();
    const fake = setFakeApi();
    fake.state.packPreview = {
      token: 7,
      id: "drills",
      name: "Graph drills",
      author: "Ana",
      version: "1.0.0",
      licence: "CC-BY-SA-4.0",
      sha256: "ab".repeat(32),
      problems: 12,
      installed: null,
      action: "install",
      assignment: "Week 3",
    };
    renderApp("/settings?section=packs");
    const box = await screen.findByTestId("packs");
    expect(box).toHaveTextContent("No packs installed.");
    await user.click(within(box).getByTestId("pack-import"));
    const dialog = await screen.findByTestId("pack-preview");
    expect(dialog).toHaveTextContent("Unsigned pack. Author as stated: Ana.");
    expect(within(dialog).getByTestId("pack-sha")).toHaveTextContent("ab".repeat(32));
    expect(dialog).toHaveTextContent("CC-BY-SA-4.0");
    expect(within(dialog).getByTestId("pack-assignment")).toHaveTextContent("Week 3");
    await user.click(within(dialog).getByTestId("pack-apply"));
    expect(await within(box).findByTestId("pack-restart")).toHaveTextContent("restarts");
    expect(await within(box).findByTestId("pack-list")).toHaveTextContent("Graph drills");
    await user.click(within(box).getByRole("button", { name: "Restart now" }));
    expect(fake.api.restartApp).toHaveBeenCalled();
    await user.click(within(box).getByRole("button", { name: "Uninstall Graph drills" }));
    await waitFor(() => expect(within(box).getByText("No packs installed.")).toBeInTheDocument());
  });

  it("refuses to replace an installed pack with an older one", async () => {
    const user = userEvent.setup();
    const fake = setFakeApi();
    fake.state.packPreview = {
      token: 8,
      id: "drills",
      name: "Graph drills",
      author: "Ana",
      version: "0.9.0",
      licence: "MIT",
      sha256: "cd".repeat(32),
      problems: 3,
      installed: "1.0.0",
      action: "not_newer",
      assignment: null,
    };
    renderApp("/settings?section=packs");
    await user.click(await screen.findByTestId("pack-import"));
    const dialog = await screen.findByTestId("pack-preview");
    expect(dialog).toHaveTextContent("installed: 1.0.0");
    expect(within(dialog).getByTestId("pack-action")).toHaveTextContent("not newer");
    expect(within(dialog).queryByTestId("pack-apply")).toBeNull();
  });

  it("marks pack problems and filters the library by source", async () => {
    PROBLEMS[2].pack = "drills";
    try {
      setFakeApi();
      renderApp("/library?source=packs");
      await screen.findByText("Longest Window");
      expect(screen.queryByText("Two Sum")).toBeNull();
      expect(screen.getByTestId("pack-label")).toHaveTextContent("drills");
      expect(screen.getByTestId("source-filter")).toBeInTheDocument();
    } finally {
      PROBLEMS[2].pack = null;
    }
  });
});
