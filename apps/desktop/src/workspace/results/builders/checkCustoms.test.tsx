import { describe, expect, it, vi } from "vitest";
import { screen, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { renderApp } from "@/test/renderApp";
import { setFakeApi, useFakeApi } from "@/test/mockApi";

vi.mock("@/lib/api", async () => (await import("@/test/mockApi")).apiModule);

describe("custom tests: builders and Check (expansion M9)", () => {
  useFakeApi();

  it("offers a builder per parameter and checks the inputs without running code", async () => {
    const user = userEvent.setup();
    const fake = setFakeApi({ lastLanguage: "python" });
    renderApp("/problems/two-sum");
    await user.click(await screen.findByRole("button", { name: "Add custom test" }));
    const card = await screen.findByTestId("custom-1");
    expect(within(card).getByTestId("build-nums")).toBeInTheDocument();
    expect(within(card).queryByTestId("build-target")).toBeNull();
    const nums = within(card).getByRole("textbox", { name: /nums/ });
    await user.clear(nums);
    await user.type(nums, "[[4]");
    await user.click(screen.getByTestId("check-customs"));
    const results = await screen.findByTestId("check-results");
    expect(results).toHaveTextContent("Custom 1: 2 ≤ nums.length ≤ 10⁵");
    expect(fake.api.checkCustom).toHaveBeenCalledWith("two-sum", [{ nums: [4], target: 15 }]);
    expect(fake.api.run).not.toHaveBeenCalled();
  });
});
