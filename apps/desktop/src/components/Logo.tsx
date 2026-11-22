// The KataBox mark (ADR 0021, 0041): the app icon, "Kata Box". The full-colour mark uses
// the brand tokens (styles/tokens.css); `mono` draws the cube alone in the current text
// colour. The icon files are src-tauri/icons/katabox.svg (and katabox-small.svg, which
// this follows at small sizes: no braces below 32 px).
import { useId } from "react";
import { cn } from "@/lib/utils";

/** The braces and chevron (paths, no font), shared with the splash. */
export const LOGO_BRACES = [
  "M73 80 C66 80 64 83 64 89 L64 95 C64 98.5 62 100 55 101 C62 102 64 103.5 64 107 L64 113 C64 119 66 122 73 122",
  "M105 80 C112 80 114 83 114 89 L114 95 C114 98.5 116 100 123 101 C116 102 114 103.5 114 107 L114 113 C114 119 112 122 105 122",
];

export function Logo({ className, mono = false, braces = true }: { className?: string; mono?: boolean; braces?: boolean }) {
  const id = useId();
  if (mono) {
    return (
      <svg viewBox="40 30 176 198" aria-hidden className={cn("size-6", className)} data-testid="logo">
        <path d="M128 40 206 84 128 128 50 84Z" fill="currentColor" />
        <path d="M50 84 128 128 128 218 50 174Z" fill="currentColor" fillOpacity="0.62" />
        <path d="M206 84 128 128 128 218 206 174Z" fill="currentColor" fillOpacity="0.36" />
      </svg>
    );
  }
  return (
    <svg viewBox="0 0 256 256" aria-hidden className={cn("size-6", className)} data-testid="logo">
      <defs>
        <linearGradient id={`${id}bg`} x1="0" y1="0" x2="0" y2="1">
          <stop offset="0" style={{ stopColor: "var(--brand-bg-from)" }} />
          <stop offset="1" style={{ stopColor: "var(--brand-bg-to)" }} />
        </linearGradient>
      </defs>
      <rect width="256" height="256" rx="58" fill={`url(#${id}bg)`} />
      <path d="M128 40 206 84 128 128 50 84Z" style={{ fill: "var(--brand-top)" }} />
      <path d="M50 84 128 128 128 218 50 174Z" style={{ fill: "var(--brand-left)" }} />
      <path d="M206 84 128 128 128 218 206 174Z" style={{ fill: "var(--brand-right)" }} />
      {braces && (
        <g
          transform="matrix(1 0.564 0 1 0 0)"
          fill="none"
          strokeWidth="6"
          strokeLinecap="round"
          strokeLinejoin="round"
          style={{ stroke: "var(--brand-mark)" }}
        >
          {LOGO_BRACES.map((d) => (
            <path key={d} d={d} />
          ))}
        </g>
      )}
      <path
        d="M158 136 L178 151 L158 166"
        fill="none"
        strokeWidth={braces ? 10 : 16}
        strokeLinecap="round"
        strokeLinejoin="round"
        style={{ stroke: "var(--brand-mark)" }}
      />
    </svg>
  );
}
