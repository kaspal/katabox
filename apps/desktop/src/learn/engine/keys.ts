// Player shortcuts (ADR 0029): Space plays or pauses, ←/→ step, Home restarts. Keys that
// a focused control already handles are left alone, so nothing fires twice and typing
// is never stolen.

export type KeyAction = "toggle" | "back" | "forward" | "restart";

/** Controls that use these keys themselves (text entry, sliders, tabs, menus, lists). */
const OWNS_KEYS =
  'input, textarea, select, [contenteditable=""], [contenteditable="true"], [role="slider"], [role="tab"], [role="menuitem"], [role="option"], [role="listbox"], [role="combobox"], [role="radio"], [role="dialog"]';
/** Space and Enter already activate these. */
const ACTIVATES = 'button, a[href], [role="button"], summary';

export function playerKeyAction(e: Pick<KeyboardEvent, "key" | "ctrlKey" | "metaKey" | "altKey" | "shiftKey" | "defaultPrevented" | "target">): KeyAction | null {
  if (e.defaultPrevented || e.ctrlKey || e.metaKey || e.altKey || e.shiftKey) return null;
  const target = e.target instanceof Element ? e.target : null;
  if (target?.closest(OWNS_KEYS)) return null;
  switch (e.key) {
    case " ":
    case "Spacebar":
      return target?.closest(ACTIVATES) ? null : "toggle";
    case "ArrowLeft":
      return "back";
    case "ArrowRight":
      return "forward";
    case "Home":
      return "restart";
    default:
      return null;
  }
}
