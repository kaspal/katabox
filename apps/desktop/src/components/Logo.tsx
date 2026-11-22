// The KataBox mark (ADR 0021, 0041): the app icon, "Kata Box". The full-colour mark uses
// the brand tokens (styles/tokens.css); `mono` draws the cube alone in the current text
// colour. The icon files are src-tauri/icons/katabox.svg (and katabox-small.svg, which
// this follows at small sizes: no braces below 32 px).
import { useId } from "react";
import { cn } from "@/lib/utils";

/** The braces and chevron (paths, no font), shared with the splash. */
export const LOGO_BRACES = [
  "M70 80 C63 80 61 83 61 89 L61 95 C61 98.5 59 100 52 101 C59 102 61 103.5 61 107 L61 113 C61 119 63 122 70 122",
  "M108 80 C115 80 117 83 117 89 L117 95 C117 98.5 119 100 126 101 C119 102 117 103.5 117 107 L117 113 C117 119 115 122 108 122",
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
        d="M152 150 l20 14 -20 14"
        fill="none"
        strokeWidth={braces ? 10 : 16}
        strokeLinecap="round"
        strokeLinejoin="round"
        transform="matrix(1 -0.564 0 1 0 94)"
        style={{ stroke: "var(--brand-mark)" }}
      />
    </svg>
  );
}
