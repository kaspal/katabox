import { Tooltip, TooltipContent, TooltipTrigger } from "@/components/ui/tooltip";

/** Shorten in the middle, keeping both ends (`C:\Users\…\toolchains\python.exe`). */
export function middleEllipsis(text: string, max: number): string {
  if (text.length <= max) return text;
  const keep = max - 1;
  const head = Math.ceil(keep * 0.4);
  return `${text.slice(0, head)}…${text.slice(text.length - (keep - head))}`;
}

/** A long path shortened in the middle; the full text is in a tooltip and is what gets copied. */
export function MiddleEllipsis({ text, max = 56, className }: { text: string; max?: number; className?: string }) {
  const short = middleEllipsis(text, max);
  if (short === text) return <span className={className}>{text}</span>;
  return (
    <Tooltip>
      <TooltipTrigger asChild>
        <span
          tabIndex={0}
          className={className}
          aria-label={text}
          data-full={text}
          onCopy={(e) => {
            e.preventDefault();
            e.clipboardData.setData("text/plain", text);
          }}
        >
          {short}
        </span>
      </TooltipTrigger>
      <TooltipContent className="max-w-xl font-mono text-xs break-all">{text}</TooltipContent>
    </Tooltip>
  );
}
