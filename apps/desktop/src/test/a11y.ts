// The axe-like naming check shared by the accessibility tests (features-content M11).
export function nameOf(el: Element): string {
  const labelledBy = el.getAttribute("aria-labelledby");
  if (labelledBy) {
    return labelledBy
      .split(/\s+/)
      .map((id) => document.getElementById(id)?.textContent ?? "")
      .join(" ")
      .trim();
  }
  const aria = el.getAttribute("aria-label")?.trim();
  if (aria) return aria;
  // <label for> names any labelable element, buttons included (Radix checkboxes are buttons).
  const forLabel = el.id ? document.querySelector(`label[for="${CSS.escape(el.id)}"]`)?.textContent?.trim() : "";
  if (forLabel) return forLabel;
  if (el instanceof HTMLInputElement || el instanceof HTMLSelectElement || el instanceof HTMLTextAreaElement) {
    const viaFor = el.id ? document.querySelector(`label[for="${CSS.escape(el.id)}"]`)?.textContent : null;
    const wrap = el.closest("label")?.textContent;
    return (viaFor ?? wrap ?? el.getAttribute("title") ?? el.getAttribute("placeholder") ?? "").trim();
  }
  return (el.textContent ?? el.getAttribute("title") ?? "").trim();
}

export function unnamed(root: ParentNode): string[] {
  const selector = 'button, a[href], [role="button"], [role="link"], [role="tab"], [role="radio"], input:not([type="hidden"]), select, textarea, [role="combobox"], [role="switch"], [role="checkbox"]';
  return [...root.querySelectorAll(selector)]
    .filter((el) => !el.closest("[hidden], [aria-hidden='true']"))
    .filter((el) => nameOf(el) === "")
    .map((el) => el.outerHTML.slice(0, 160));
}
