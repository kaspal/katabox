// Command palette matching (ui-polish): a command is found by its label as shown, with or
// without punctuation and accents. "Theme: Dark", "theme dark" and "Configuración: Editor"
// / "configuracion editor" all match, for every command. cmdk's own fuzzy score then runs
// on the normalised text.
import { defaultFilter } from "cmdk";

/** Lower case, accents dropped, anything but letters and digits turned into one space. */
export function normalizeForSearch(text: string): string {
  return text
    .normalize("NFKD")
    .replace(/\p{M}+/gu, "")
    .toLowerCase()
    .replace(/[^\p{L}\p{N}]+/gu, " ")
    .trim();
}

export function paletteFilter(value: string, search: string, keywords?: string[]): number {
  const q = normalizeForSearch(search);
  if (q === "") return 1;
  return defaultFilter(normalizeForSearch(value), q, keywords?.map(normalizeForSearch));
}
