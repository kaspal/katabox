import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { useTranslation } from "react-i18next";
import { Lightbulb, Lock } from "lucide-react";
import type { ProblemView } from "@/bindings";
import { Button } from "@/components/ui/button";
import { ErrorNotice } from "@/components/ErrorNotice";
import { api } from "@/lib/api";
import { Markdown } from "@/workspace/problem/Markdown";

/** Tiered hints, revealed one at a time and in order. */
export function HintsTab({ problem }: { problem: ProblemView }) {
  const { t } = useTranslation();
  const queryClient = useQueryClient();
  const key = ["hints", problem.id];
  const unlocked = useQuery({ queryKey: key, queryFn: () => api.unlockedHints(problem.id) });
  const reveal = useMutation({
    mutationFn: (tier: number) => api.unlockHint(problem.id, tier),
    onSuccess: (tiers) => queryClient.setQueryData(key, tiers),
  });

  if (problem.hints.length === 0) {
    return <p className="text-sm text-muted-foreground">{t("workspace.hints.none")}</p>;
  }
  if (unlocked.error) return <ErrorNotice error={unlocked.error} />;
  const open = new Set(unlocked.data ?? []);
  // Tiers unlock in order: the next one is the first not yet revealed.
  const next = problem.hints.findIndex((_, i) => !open.has(i + 1)) + 1;

  return (
    <ol className="space-y-3">
      {problem.hints.map((hint, i) => {
        const tier = i + 1;
        const isOpen = open.has(tier);
        return (
          <li key={tier} className="rounded-lg border p-3" data-testid={`hint-${tier}`}>
            <h3 className="mb-2 flex items-center gap-2 text-sm font-semibold">
              {isOpen ? (
                <Lightbulb aria-hidden className="size-4 text-difficulty-medium" />
              ) : (
                <Lock aria-hidden className="size-4 text-muted-foreground" />
              )}
              {t("workspace.hints.tier", { tier })}
            </h3>
            {isOpen ? (
              <Markdown text={hint} />
            ) : tier === next ? (
              <Button
                size="sm"
                variant="outline"
                disabled={reveal.isPending || unlocked.isPending}
                onClick={() => reveal.mutate(tier)}
              >
                {t("workspace.hints.reveal", { tier })}
              </Button>
            ) : (
              <p className="text-xs text-muted-foreground">{t("workspace.hints.lockedAfter", { tier: tier - 1 })}</p>
            )}
          </li>
        );
      })}
      {reveal.error && <ErrorNotice error={reveal.error} />}
    </ol>
  );
}
