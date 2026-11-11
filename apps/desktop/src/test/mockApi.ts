// Use in a test file:
//   vi.mock("@/lib/api", async () => (await import("@/test/mockApi")).apiModule);
//   const fake = useFakeApi();   // fresh fake backend per test
import { beforeEach } from "vitest";
import type { Api } from "@/lib/api";
import { createFakeApi } from "@/test/fakeApi";

let current: ReturnType<typeof createFakeApi> = createFakeApi();

export const apiModule = {
  api: new Proxy({} as Api, {
    get: (_, key: keyof Api) => current.api[key],
  }),
};

/** Replace the backend for one test (call inside the test, before rendering). */
export function setFakeApi(opts: Parameters<typeof createFakeApi>[0] = {}) {
  current = createFakeApi(opts);
  return current;
}

/** A fresh default backend before each test; returns a getter for the current one. */
export function useFakeApi() {
  beforeEach(() => {
    setFakeApi();
  });
  return () => current;
}
