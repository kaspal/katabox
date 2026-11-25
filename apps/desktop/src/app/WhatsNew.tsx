import { useEffect, useState } from "react";
import { useQuery } from "@tanstack/react-query";
import { useTranslation } from "react-i18next";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";
import { api } from "@/lib/api";
import { settingKey, useSetting } from "@/lib/settings";

/** The app version the user last started (machine-local: not in an export). */
export const LAST_VERSION_KEY = "app.last_version";

/** The highlights shown after an upgrade (`whatsNew.items.<id>`), from CHANGELOG.md. */
export const WHATS_NEW_ITEMS = ["home", "brand", "content", "practice", "packs", "app"] as const;

/**
 * Whether to show "What's new" (decided once per start). A start on a new version
 * shows it when the user had already used KataBox: a version was recorded before, or,
 * for an upgrade from a version that recorded none, the welcome was already done. A
 * fresh install records the version silently (the welcome covers it).
 */
export function decideWhatsNew(last: string | null, current: string, onboardingDone: boolean | null): boolean {
  if (last === current) return false;
  if (last !== null) return true;
  return onboardingDone === true;
}

export function useWhatsNew(current: string | undefined, onboarding: { loaded: boolean; done: boolean | null }, welcomeShown: boolean) {
  const query = useQuery({
    queryKey: settingKey(LAST_VERSION_KEY),
    queryFn: () => api.getSetting(LAST_VERSION_KEY),
    staleTime: Infinity,
  });
  const [last, setLast] = useSetting<string | null>(LAST_VERSION_KEY, null, (v) =>
    typeof v === "string" ? v : undefined,
  );
  const [show, setShow] = useState<boolean | null>(null);
  useEffect(() => {
    if (show !== null || !current || !query.isSuccess || !onboarding.loaded) return;
    const decided = !welcomeShown && decideWhatsNew(last, current, onboarding.done);
    setShow(decided);
    // Recorded when decided, so it shows once even if the app closes with it open.
    if (last !== current) setLast(current);
  }, [show, current, query.isSuccess, onboarding.loaded, onboarding.done, welcomeShown, last, setLast]);
  return { show: show === true, hide: () => setShow(false) };
}

export function WhatsNewDialog({ open, version, onClose }: { open: boolean; version: string; onClose: () => void }) {
  const { t } = useTranslation();
  return (
    <Dialog open={open} onOpenChange={(o) => !o && onClose()}>
      <DialogContent className="sm:max-w-lg" data-testid="whats-new">
        <DialogHeader>
          <DialogTitle>{t("whatsNew.title", { version })}</DialogTitle>
          <DialogDescription>{t("whatsNew.intro")}</DialogDescription>
        </DialogHeader>
        <ul className="list-disc space-y-1.5 pl-5 text-sm">
          {WHATS_NEW_ITEMS.map((id) => (
            <li key={id}>{t(`whatsNew.items.${id}`)}</li>
          ))}
        </ul>
        <p className="text-xs text-muted-foreground">{t("whatsNew.more")}</p>
        <DialogFooter>
          <Button onClick={onClose}>{t("whatsNew.close")}</Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
