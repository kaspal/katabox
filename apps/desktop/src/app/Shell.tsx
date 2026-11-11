import { useEffect, useState } from "react";
import { Link, Outlet, useNavigate, useRouterState } from "@tanstack/react-router";
import { useTranslation } from "react-i18next";
import { Boxes, Command as CommandIcon } from "lucide-react";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Tooltip, TooltipContent, TooltipTrigger } from "@/components/ui/tooltip";
import { CommandPalette } from "@/app/CommandPalette";
import { DEV_NAV, NAV } from "@/app/nav";
import { useFocusMode } from "@/lib/focus";
import { useAppInfo, useLanguages } from "@/lib/queries";
import { FirstRun } from "@/languages/FirstRun";

/**
 * The first-run screen (ADR 0015): decided once, when the language list first loads, and
 * kept until the user finishes or skips it (so an install's success state stays visible).
 */
function useFirstRun() {
  const langs = useLanguages();
  const [show, setShow] = useState<boolean | null>(null);
  useEffect(() => {
    if (show !== null || !langs.data) return;
    setShow(langs.data.platform_supported && !langs.data.languages.some((l) => l.runtime_installed));
  }, [langs.data, show]);
  return { show: show === true, hide: () => setShow(false) };
}

/** Left navigation rail, top bar (DEV DATA badge, palette) and the page. */
export function Shell() {
  const { t } = useTranslation();
  const [paletteOpen, setPaletteOpen] = useState(false);
  const info = useAppInfo();
  const { focus } = useFocusMode();
  const firstRun = useFirstRun();
  const navigate = useNavigate();
  const pathname = useRouterState({ select: (s) => s.location.pathname });
  // Settings and the dev screens stay reachable from the rail.
  const gated = firstRun.show && !pathname.startsWith("/settings") && !pathname.startsWith("/dev");

  useEffect(() => {
    const onKey = (e: KeyboardEvent) => {
      if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === "k") {
        e.preventDefault();
        setPaletteOpen((open) => !open);
      }
    };
    window.addEventListener("keydown", onKey);
    return () => window.removeEventListener("keydown", onKey);
  }, []);

  return (
    <div
      className={
        focus ? "grid h-full grid-cols-[1fr] grid-rows-[1fr]" : "grid h-full grid-cols-[56px_1fr] grid-rows-[44px_1fr]"
      }
      data-focus={focus || undefined}
    >
      <nav
        hidden={focus}
        aria-label={t("nav.label")}
        className="row-span-2 flex flex-col items-center gap-1 border-r bg-rail py-2"
      >
        <span className="mb-2 flex size-9 items-center justify-center text-primary" aria-hidden>
          <Boxes className="size-6" />
        </span>
        {[...NAV, ...DEV_NAV].map((item) => (
          <Tooltip key={item.to}>
            <TooltipTrigger asChild>
              <Link
                to={item.to}
                aria-label={t(item.labelKey)}
                activeOptions={{ exact: item.to === "/" }}
                className="flex size-10 items-center justify-center rounded-md text-muted-foreground transition-colors hover:bg-accent hover:text-foreground aria-[current=page]:bg-accent aria-[current=page]:text-foreground"
              >
                <item.icon aria-hidden className="size-5" />
              </Link>
            </TooltipTrigger>
            <TooltipContent side="right">{t(item.labelKey)}</TooltipContent>
          </Tooltip>
        ))}
      </nav>

      <header hidden={focus} className="flex items-center justify-between gap-3 border-b px-4">
        <span className="font-semibold">{t("app.name")}</span>
        <div className="flex items-center gap-3">
          {info.data?.dev_data && (
            <Tooltip>
              <TooltipTrigger asChild>
                <Badge
                  tabIndex={0}
                  className="bg-dev-badge font-mono text-dev-badge-foreground"
                  data-testid="dev-data-badge"
                >
                  {t("app.devData")}
                </Badge>
              </TooltipTrigger>
              <TooltipContent>{t("app.devDataHint")}</TooltipContent>
            </Tooltip>
          )}
          <Button
            variant="outline"
            size="sm"
            className="gap-2 text-muted-foreground"
            onClick={() => setPaletteOpen(true)}
          >
            <CommandIcon aria-hidden className="size-3.5" />
            {t("palette.open")}
            <kbd className="rounded border bg-muted px-1.5 font-mono text-xs">Ctrl K</kbd>
          </Button>
        </div>
      </header>

      <main className="min-h-0 overflow-auto">
        {gated ? (
          <FirstRun
            onDone={() => {
              firstRun.hide();
              void navigate({ to: "/" });
            }}
            onSkip={firstRun.hide}
          />
        ) : (
          <Outlet />
        )}
      </main>

      <CommandPalette open={paletteOpen} onOpenChange={setPaletteOpen} />
    </div>
  );
}
