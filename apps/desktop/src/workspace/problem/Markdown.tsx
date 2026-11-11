// Problem text: Markdown + GFM + math (KaTeX), code fences highlighted with the same
// highlighter as the editor (ADR 0013). Raw HTML in Markdown is not rendered.
import ReactMarkdown from "react-markdown";
import remarkGfm from "remark-gfm";
import remarkMath from "remark-math";
import rehypeKatex from "rehype-katex";
import "katex/dist/katex.min.css";
import { useEffect, useState } from "react";
import { highlightWith, type Highlighted } from "@/workspace/editor/syntax";
import { loadFenceParser } from "@/workspace/editor/languages";
import { cn } from "@/lib/utils";

/** Highlight runs once the fence language's grammar has loaded (plain text until then). */
function useHighlighted(code: string, fence: string | undefined): Highlighted | null {
  const [runs, setRuns] = useState<Highlighted | null>(null);
  useEffect(() => {
    let live = true;
    setRuns(null);
    void loadFenceParser(fence).then((parser) => {
      if (live && parser) setRuns(highlightWith(parser, code));
    });
    return () => {
      live = false;
    };
  }, [code, fence]);
  return runs;
}

function Code({ className, children }: { className?: string; children?: React.ReactNode }) {
  const fence = /language-(\w+)/.exec(className ?? "")?.[1];
  const text = String(children ?? "");
  const block = text.includes("\n") || fence !== undefined;
  const runs = useHighlighted(text.replace(/\n$/, ""), block ? fence : undefined);
  if (!block) {
    return <code className="rounded bg-muted px-1 py-0.5 font-mono text-[0.85em]">{children}</code>;
  }
  return (
    <code className="font-mono text-xs" data-fence={fence}>
      {runs
        ? runs.map((r, i) =>
            r.className ? (
              <span key={i} className={r.className}>
                {r.text}
              </span>
            ) : (
              r.text
            ),
          )
        : text.replace(/\n$/, "")}
    </code>
  );
}

export function Markdown({ text, className }: { text: string; className?: string }) {
  return (
    <div className={cn("kb-prose", className)}>
      <ReactMarkdown
        remarkPlugins={[remarkGfm, remarkMath]}
        rehypePlugins={[rehypeKatex]}
        components={{
          code: ({ className: cls, children }) => <Code className={cls}>{children}</Code>,
          pre: ({ children }) => (
            <pre className="my-3 overflow-x-auto rounded-md border bg-muted/50 p-3 leading-relaxed">{children}</pre>
          ),
          a: ({ children, href }) => (
            <a href={href} className="text-primary underline underline-offset-2" target="_blank" rel="noreferrer">
              {children}
            </a>
          ),
        }}
      >
        {text}
      </ReactMarkdown>
    </div>
  );
}
