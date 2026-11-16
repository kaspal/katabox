import { useEffect, useState } from "react";
import { Link, Outlet, useRouterState } from "@tanstack/react-router";
import { useTranslation } from "react-i18next";
import { Command as CommandIcon, Keyboard } from "lucide-react";
import { Logo } from "@/components/Logo";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Tooltip, TooltipContent, TooltipTrigger } from "@/components/ui/tooltip";
import { CommandPalette } from "@/app/CommandPalette";
import { ShortcutsDialog } from "@/app/ShortcutsDialog";
import { WhatsNewDialog, useWhatsNew } from "@/app/WhatsNew";
import { onOpenShortcuts, openShortcuts } from "@/app/shortcuts";
import { DEV_NAV, NAV } from "@/app/nav";
import { useFocusMode } from "@/lib/focus";
import { useAppInfo, useLanguages } from "@/lib/queries";
import { Welcome } from "@/onboarding/Welcome";
import { useOnboardingDone } from "@/onboarding/state";
import { InterviewTimer } from "@/practice/interview";

/**
 * The welcome flow (features-content M2; it starts with ADR 0015's language install):
 * decided once, when the language list and `onboarding.done` first load, and kept until
 * the user finishes or skips it. Shown when asked again from Settings (`false`), or on a
 * first start (never decided) with no language installed. An install that already has a
 * language (an upgrade) is marked done without showing it.
 */
function useFirstRun() {
  const langs = useLanguages();
  const onboarding = useOnboardingDone();
  const [show, setShow] = useState<boolean | null>(null);
  useEffect(() => {
    if (!langs.data || !onboarding.loaded) return;
    if (onboarding.done === false) {
      setShow(true);
      return;
    }
    if (show !== null) return;
    const none = !langs.data.languages.some((l) => l.runtime_installed);
    if (onboarding.done === null && !none) onboarding.setDone(true);
    setShow(onboarding.done === null && langs.data.platform_supported && none);
  }, [langs.data, onboarding.loaded, onboarding.done, show]);
  return { show: show === true, hide: () => setShow(false), onboarding };
}

/** Left navigation rail, top bar (DEV DATA badge, shortcuts, palette) and the page. */
export function Shell() {
  const { t } = useTranslation();
  const [paletteOpen, setPaletteOpen] = useState(false);
  const [shortcutsOpen, setShortcutsOpen] = useState(false);
  const info = useAppInfo();
  const { focus } = useFocusMode();
  const firstRun = useFirstRun();
  const whatsNew = useWhatsNew(info.data?.app_version, firstRun.onboarding, firstRun.show);
  const pathname = useRouterState({ select: (s) => s.location.pathname });
  // Settings and the dev screens stay reachable from the rail.
  const gated = firstRun.show && !pathname.startsWith("/settings") && !pathname.startsWith("/dev");

  useEffect(() => {
    const onKey = (e: KeyboardEvent) => {
      if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === "k") {
        e.preventDefault();
        setPaletteOpen((open) => !open);
      } else if (e.key === "F1" && !e.ctrlKey && !e.altKey && !e.shiftKey && !e.metaKey) {
        e.preventDefault();
        setShortcutsOpen(true);
      }
    };
    window.addEventListener("keydown", onKey);
    const off = onOpenShortcuts(() => setShortcutsOpen(true));
    return () => {
      window.removeEventListener("keydown", onKey);
      off();
    };
  }, []);

  return (
    <div
      className={
        focus ? "grid h-full grid-cols-[1fr] grid-rows-[1fr]" : "grid h-full grid-cols-[56px_1fr] grid-rows-[44px_1fr]"
      }
      data-focus={focus || undefined}
    >
      <a
        href="#main"
        className="sr-only z-50 rounded-md bg-primary text-primary-foreground focus:not-sr-only focus:fixed focus:left-2 focus:top-2 focus:px-3 focus:py-2 focus:text-sm focus:font-medium"
        onClick={(e) => {
          // The router uses the hash: move focus instead of navigating.
          e.preventDefault();
          document.getElementById("main")?.focus();
        }}
        data-testid="skip-link"
      >
        {t("app.skipToContent")}
      </a>
      <nav
        hidden={focus}
        aria-label={t("nav.label")}
        className="row-span-2 flex flex-col items-center gap-1 border-r bg-rail py-2"
      >
        <span className="mb-2 flex size-9 items-center justify-center text-primary" aria-hidden>
          <Logo className="size-6" />
        </span>
        {[...NAV, ...DEV_NAV].map((item) => (
          <Tooltip key={item.to}>
            <TooltipTrigger asChild>
              <Link
                to={item.to}
                aria-label={t(item.labelKey)}
                data-tour={item.to === "/learn" ? "learn" : undefined}
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
          <InterviewTimer />
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
          <Tooltip>
            <TooltipTrigger asChild>
              <Button
                variant="ghost"
                size="icon"
                className="size-8 text-muted-foreground"
                aria-label={t("shortcuts.open")}
                aria-keyshortcuts="F1"
                onClick={openShortcuts}
                data-testid="shortcuts-button"
              >
                <Keyboard aria-hidden className="size-4" />
              </Button>
            </TooltipTrigger>
            <TooltipContent>{t("shortcuts.open")} (F1)</TooltipContent>
          </Tooltip>
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

      <main id="main" tabIndex={-1} className="min-h-0 overflow-auto outline-none">
        {gated ? (
          <Welcome onClose={firstRun.hide} />
        ) : (
          <Outlet />
        )}
      </main>

      <CommandPalette open={paletteOpen} onOpenChange={setPaletteOpen} />
      <ShortcutsDialog open={shortcutsOpen} onOpenChange={setShortcutsOpen} />
      {info.data && (
        <WhatsNewDialog open={whatsNew.show} version={info.data.app_version} onClose={whatsNew.hide} />
      )}
    </div>
  );
}
