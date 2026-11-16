import { Fragment } from "react";
import { useTranslation } from "react-i18next";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { NAMED_KEYS, SHORTCUTS } from "@/app/shortcuts";

function Key({ name }: { name: string }) {
  const { t } = useTranslation();
  const named = (NAMED_KEYS as readonly string[]).includes(name);
  return (
    <kbd className="rounded border bg-muted px-1.5 py-0.5 font-mono text-xs">
      {named ? t(`shortcuts.key.${name}` as "shortcuts.key.Space") : name}
    </kbd>
  );
}

/** Every keyboard shortcut, by area (F1, the command palette, the top bar, About). */
export function ShortcutsDialog({ open, onOpenChange }: { open: boolean; onOpenChange: (open: boolean) => void }) {
  const { t } = useTranslation();
  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-2xl" data-testid="shortcuts">
        <DialogHeader>
          <DialogTitle>{t("shortcuts.title")}</DialogTitle>
          <DialogDescription>{t("shortcuts.intro")}</DialogDescription>
        </DialogHeader>
        <div className="max-h-[60vh] space-y-4 overflow-auto pr-1" tabIndex={0} aria-label={t("shortcuts.title")}>
          {SHORTCUTS.map((group) => (
            <section key={group.id}>
              <h3 className="mb-1 text-sm font-semibold">
                {t(`shortcuts.group.${group.id}` as "shortcuts.group.general")}
              </h3>
              <table className="w-full text-sm">
                <tbody>
                  {group.rows.map((row) => (
                    <tr key={row.id} className="border-b last:border-b-0">
                      <td className="py-1.5 pr-4">
                        {t(`shortcuts.${group.id}.${row.id}` as "shortcuts.general.palette")}
                      </td>
                      <td className="py-1.5 text-right whitespace-nowrap">
                        {row.combos.map((combo, i) => (
                          <Fragment key={i}>
                            {i > 0 && <span className="px-1 text-muted-foreground">/</span>}
                            {combo.map((k, j) => (
                              <Fragment key={j}>
                                {j > 0 && <span className="px-0.5 text-muted-foreground">+</span>}
                                <Key name={k} />
                              </Fragment>
                            ))}
                          </Fragment>
                        ))}
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </section>
          ))}
          <p className="text-xs text-muted-foreground">{t("shortcuts.keymapNote")}</p>
        </div>
      </DialogContent>
    </Dialog>
  );
}
