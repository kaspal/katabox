// The 30-second workspace tour (features-content M2): five coach marks (Run, Submit,
// Hints, the results and debugger, Learn), each next to its element (`data-tour`).
// Keyboard: → or Enter next, ← back, Esc ends it. Focus moves into the mark; nothing
// else on the page is blocked, so there is no focus trap.
import { useEffect, useLayoutEffect, useRef, useState } from "react";
import { useTranslation } from "react-i18next";
import { Button } from "@/components/ui/button";
import { useTourPending } from "@/onboarding/state";

const STEPS = ["run", "submit", "hints", "results", "learn"] as const;
type StepId = (typeof STEPS)[number];

const CARD_W = 300;

function placeNear(id: StepId): { top: number; left: number } {
  const el = document.querySelector(`[data-tour="${id}"]`);
  const vw = window.innerWidth;
  const vh = window.innerHeight;
  if (!el) return { top: vh / 2 - 80, left: vw / 2 - CARD_W / 2 };
  const r = el.getBoundingClientRect();
  const below = r.bottom + 10;
  const top = below + 170 < vh ? below : Math.max(8, r.top - 180);
  const left = Math.min(Math.max(8, r.left), vw - CARD_W - 8);
  return { top, left };
}

export function Tour() {
  const { t } = useTranslation();
  const [pending, setPending] = useTourPending();
  const [k, setK] = useState(0);
  const [pos, setPos] = useState({ top: 0, left: 0 });
  const card = useRef<HTMLDivElement>(null);
  const step = STEPS[k];

  useLayoutEffect(() => {
    if (!pending) return;
    const update = () => setPos(placeNear(step));
    update();
    window.addEventListener("resize", update);
    return () => window.removeEventListener("resize", update);
  }, [pending, step]);

  useEffect(() => {
    if (pending) card.current?.focus();
  }, [pending, k]);

  // The workspace may still be rendering its target: look again for a few seconds, then
  // highlight it and move the mark next to it.
  useEffect(() => {
    if (!pending) return;
    let el: Element | null = null;
    let tries = 0;
    let timer: number | undefined;
    const look = () => {
      el = document.querySelector(`[data-tour="${step}"]`);
      if (el) {
        el.setAttribute("data-tour-active", "");
        setPos(placeNear(step));
      } else if (tries++ < 25) {
        timer = window.setTimeout(look, 200);
      }
    };
    look();
    return () => {
      window.clearTimeout(timer);
      el?.removeAttribute("data-tour-active");
    };
  }, [pending, step]);

  if (!pending) return null;
  const end = () => setPending(false);
  const next = () => (k + 1 < STEPS.length ? setK(k + 1) : end());
  const back = () => setK(Math.max(0, k - 1));

  return (
    <div
      ref={card}
      role="dialog"
      aria-labelledby="tour-title"
      aria-describedby="tour-text"
      tabIndex={-1}
      onKeyDown={(e) => {
        if (e.key === "Escape") {
          e.preventDefault();
          end();
        } else if (e.key === "ArrowRight" || (e.key === "Enter" && e.target === card.current)) {
          e.preventDefault();
          next();
        } else if (e.key === "ArrowLeft") {
          e.preventDefault();
          back();
        }
      }}
      style={{ top: pos.top, left: pos.left, width: CARD_W }}
      className="fixed z-50 space-y-2 rounded-lg border bg-popover p-4 text-popover-foreground shadow-lg outline-none focus-visible:ring-2 focus-visible:ring-ring"
      data-testid="tour"
      data-step={step}
    >
      <p className="text-xs text-muted-foreground">{t("welcome.tour.count", { k: k + 1, total: STEPS.length })}</p>
      <h2 id="tour-title" className="text-sm font-semibold">
        {t(`welcome.tour.${step}.title`)}
      </h2>
      <p id="tour-text" className="text-sm text-muted-foreground">
        {t(`welcome.tour.${step}.text`)}
      </p>
      <div className="flex items-center justify-between gap-2 pt-1">
        <Button variant="ghost" size="sm" onClick={end}>
          {t("welcome.tour.skip")}
        </Button>
        <div className="flex gap-2">
          {k > 0 && (
            <Button variant="outline" size="sm" onClick={back}>
              {t("welcome.tour.back")}
            </Button>
          )}
          <Button size="sm" onClick={next}>
            {k + 1 < STEPS.length ? t("welcome.tour.next") : t("welcome.tour.done")}
          </Button>
        </div>
      </div>
    </div>
  );
}
