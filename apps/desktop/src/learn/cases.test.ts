// cases.json is generated from the tracers and committed; this test fails when it is
// stale. `pnpm learn:cases` (vitest -u on this file) rewrites it.
import { describe, expect, it } from "vitest";
import { REFERENCE } from "@/test/learnReference";
import { buildCases, casesJson } from "./engine/cases";
import { TRACERS } from "./tracers";

describe("learn cases.json", () => {
  for (const def of Object.values(TRACERS)) {
    it(`${def.id} matches the tracer`, async () => {
      const cases = buildCases(def);
      for (const c of cases) {
        expect(c.output, c.name).toEqual(REFERENCE[def.id]!(c.input));
      }
      await expect(casesJson(cases)).toMatchFileSnapshot(
        `../../../../content/learn/${def.subject}/${def.id}/cases.json`,
      );
    });
  }
});
