// The welcome flow (features-content M2), shown once on the very first start: choose and
// install a language, then "new to this" or "practised" (a short placement), then a
// 30-second tour of the workspace. Skippable at every step; never shown again unless
// Settings → Practice asks for it.
import { useState } from "react";
import { useNavigate } from "@tanstack/react-router";
import { useTranslation } from "react-i18next";
import { GraduationCap, Sprout } from "lucide-react";
import { Button } from "@/components/ui/button";
import { ToggleGroup, ToggleGroupItem } from "@/components/ui/toggle-group";
import { ErrorNotice } from "@/components/ErrorNotice";
import { UI_LANGS, type UiLangCode } from "@/i18n";
import { api } from "@/lib/api";
import { usePrefs } from "@/lib/prefs";
import { FirstRun } from "@/languages/FirstRun";
import { useOnboardingDone, usePlacement, useTourPending } from "@/onboarding/state";

type Step = "languages" | "level";

export function Welcome({ onClose }: { onClose: () => void }) {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const { lang, uiLang, setUiLang } = usePrefs();
  const { setDone } = useOnboardingDone();
  const [, setPlacement] = usePlacement();
  const [, setTour] = useTourPending();
  const [step, setStep] = useState<Step>("languages");
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<unknown>(null);

  /** "New to this" (with the tour) or "Skip the welcome" (without): start at fundamentals. */
  const finishNew = async (tour: boolean) => {
    setBusy(true);
    setError(null);
    try {
      const start = await api.finishOnboarding(null, lang);
      setDone(true);
      setPlacement(null);
      setTour(tour);
      onClose();
      if (tour && start) await navigate({ to: "/problems/$problemId", params: { problemId: start.id } });
      else await navigate({ to: "/" });
    } catch (e) {
      setError(e);
    } finally {
      setBusy(false);
    }
  };

  const startPlacement = async () => {
    setBusy(true);
    setError(null);
    try {
      const problems = (await api.placementProblems(lang)).map((p) => p.id);
      if (problems.length === 0) return await finishNew(true);
      setPlacement({ problems, index: 0 });
      setDone(true);
      onClose();
      await navigate({ to: "/problems/$problemId", params: { problemId: problems[0] } });
    } catch (e) {
      setError(e);
    } finally {
      setBusy(false);
    }
  };

  const header = (
    <div className="mx-auto flex max-w-3xl flex-wrap items-center justify-between gap-3 px-8 pt-6">
      <ToggleGroup
        type="single"
        variant="outline"
        size="sm"
        aria-label={t("language.label")}
        value={uiLang}
        onValueChange={(v) => v && setUiLang(v as UiLangCode)}
      >
        {UI_LANGS.map((l) => (
          <ToggleGroupItem key={l} value={l} lang={l} className="px-3">
            {t(`language.${l}`)}
          </ToggleGroupItem>
        ))}
      </ToggleGroup>
      <Button variant="link" size="sm" disabled={busy} onClick={() => void finishNew(false)}>
        {t("welcome.skip")}
      </Button>
    </div>
  );

  if (step === "languages") {
    return (
      <div data-testid="welcome" data-step="languages">
        {header}
        <FirstRun onDone={() => setStep("level")} onSkip={() => setStep("level")} />
      </div>
    );
  }

  return (
    <div data-testid="welcome" data-step="level">
      {header}
      <div className="mx-auto max-w-3xl space-y-6 p-8">
        <div className="space-y-2">
          <h1 className="text-2xl font-semibold">{t("welcome.levelTitle")}</h1>
          <p className="text-sm text-muted-foreground">{t("welcome.levelIntro")}</p>
        </div>
        <div className="grid gap-4 sm:grid-cols-2">
          <button
            type="button"
            disabled={busy}
            onClick={() => void finishNew(true)}
            className="flex flex-col items-start gap-2 rounded-lg border bg-card p-5 text-left transition-colors hover:bg-accent disabled:opacity-60"
          >
            <Sprout aria-hidden className="size-6 text-primary" />
            <span className="text-base font-semibold">{t("welcome.newTitle")}</span>
            <span className="text-sm text-muted-foreground">{t("welcome.newText")}</span>
          </button>
          <button
            type="button"
            disabled={busy}
            onClick={() => void startPlacement()}
            className="flex flex-col items-start gap-2 rounded-lg border bg-card p-5 text-left transition-colors hover:bg-accent disabled:opacity-60"
          >
            <GraduationCap aria-hidden className="size-6 text-primary" />
            <span className="text-base font-semibold">{t("welcome.practisedTitle")}</span>
            <span className="text-sm text-muted-foreground">{t("welcome.practisedText")}</span>
          </button>
        </div>
        {error !== null && <ErrorNotice error={error} />}
      </div>
    </div>
  );
}
