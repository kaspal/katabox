// The KataBox mark (ADR 0021): the app icon's isometric cube, drawn in the current text
// color so it follows the theme. The full icon (with its background) is
// src-tauri/icons/katabox.svg.
import { cn } from "@/lib/utils";

export function Logo({ className }: { className?: string }) {
  return (
    <svg viewBox="200 160 624 704" aria-hidden className={cn("size-6", className)} data-testid="logo">
      <g stroke="currentColor" strokeWidth="0" strokeLinejoin="round">
        <path d="M512 196 L786 354 L512 512 L238 354 Z" fill="currentColor" />
        <path d="M238 354 L512 512 L512 828 L238 670 Z" fill="currentColor" fillOpacity="0.62" />
        <path d="M512 512 L786 354 L786 670 L512 828 Z" fill="currentColor" fillOpacity="0.36" />
      </g>
    </svg>
  );
}
