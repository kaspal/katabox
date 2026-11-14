// The placement's banner in the workspace (features-content M2): "problem k of 3", with
// Next (or Finish). Submits here are ordinary attempts and solves; the result (how many
// of the three are solved) is decided in kb-app when the placement finishes.
import { useState } from "react";
import { useNavigate } from "@tanstack/react-router";
import { useTranslation } from "react-i18next";
import { Compass } from "lucide-react";
import { Button } from "@/components/ui/button";
import { api } from "@/lib/api";
import { usePrefs } from "@/lib/prefs";
import { usePlacement, useTourPending } from "@/onboarding/state";

export function PlacementBanner({ problemId }: { problemId: string }) {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const { lang } = usePrefs();
  const [placement, setPlacement] = usePlacement();
  const [, setTour] = useTourPending();
  const [busy, setBusy] = useState(false);
  if (!placement || placement.problems[placement.index] !== problemId) return null;
  const k = placement.index + 1;
  const total = placement.problems.length;
  const last = k === total;

  const next = async () => {
    setBusy(true);
    try {
      if (!last) {
        const index = placement.index + 1;
        setPlacement({ ...placement, index });
        await navigate({ to: "/problems/$problemId", params: { problemId: placement.problems[index] } });
        return;
      }
      const start = await api.finishOnboarding(placement.problems, lang);
      setPlacement(null);
      setTour(true);
      await navigate(
        start ? { to: "/problems/$problemId", params: { problemId: start.id } } : { to: "/" },
      );
    } finally {
      setBusy(false);
    }
  };

  return (
    <div
      role="region"
      aria-label={t("welcome.placement.label")}
      className="flex flex-wrap items-center gap-3 border-b bg-accent/60 px-4 py-2 text-sm"
      data-testid="placement-banner"
    >
      <Compass aria-hidden className="size-4 text-primary" />
      <span className="font-medium">{t("welcome.placement.progress", { k, total })}</span>
      <span className="text-muted-foreground">{t("welcome.placement.hint")}</span>
      <Button size="sm" className="ml-auto" disabled={busy} onClick={() => void next()}>
        {last ? t("welcome.placement.finish") : t("welcome.placement.next")}
      </Button>
    </div>
  );
}
