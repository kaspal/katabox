// The splash handover (ADR 0041): once the shell has committed its first render and the
// first app-info request has settled (either way), tell Rust the main window can show.
// Once per page load; Rust ignores repeats and calls from other windows.
import { useEffect } from "react";
import { api } from "@/lib/api";

let sent = false;

export function useAppReady(settled: boolean) {
  useEffect(() => {
    if (!settled || sent) return;
    sent = true;
    void api.appReady().catch(() => {
      // The failsafe shows the window anyway.
    });
  }, [settled]);
}

/** Tests only: allow another call. */
export function resetAppReadyForTests() {
  sent = false;
}
