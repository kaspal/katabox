// The Theory tab (ADR 0029): the topic's Markdown; `learn:<subject>/<topic>` links open
// other topics in the app.
import { Link } from "@tanstack/react-router";
import { Markdown } from "@/workspace/problem/Markdown";

export function TheoryTab({ theory }: { theory: string }) {
  return (
    <div className="p-5 text-sm leading-relaxed" data-testid="theory">
      <Markdown
        text={theory}
        internalLink={(href, children) => {
          const [subject, topic] = href.slice("learn:".length).split("/");
          return (
            <Link
              to="/learn/$subject/$topic"
              params={{ subject: subject ?? "", topic: topic ?? "" }}
              className="text-primary underline underline-offset-2"
            >
              {children}
            </Link>
          );
        }}
      />
    </div>
  );
}
