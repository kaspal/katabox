// Small pieces shared by the practice views (ADR 0019).
import { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import { Flame, ListPlus } from "lucide-react";
import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import {
  DropdownMenu,
  DropdownMenuCheckboxItem,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { Input } from "@/components/ui/input";
import { ErrorNotice } from "@/components/ErrorNotice";
import { api } from "@/lib/api";
import { useListActions, useLists, useListsWith } from "@/lib/practice";

/** "2026-11-17" as a local date in the UI language. */
export function formatDay(day: string, lang: string): string {
  const [y, m, d] = day.split("-").map(Number);
  return new Intl.DateTimeFormat(lang, { dateStyle: "medium" }).format(new Date(y, m - 1, d));
}

export function StreakBadge({ count }: { count: number }) {
  const { t } = useTranslation();
  return (
    <span
      className="inline-flex items-center gap-1 text-sm font-medium tabular-nums"
      data-testid="streak"
      title={count === 0 ? t("practice.streak.none") : undefined}
    >
      <Flame aria-hidden className={count > 0 ? "size-4 text-orange-500" : "size-4 text-muted-foreground"} />
      {t("practice.streak.label", { count })}
    </span>
  );
}

/** Ask for a list name (create or rename). */
export function NameDialog({
  open,
  title,
  initial = "",
  submitLabel,
  onOpenChange,
  onSubmit,
}: {
  open: boolean;
  title: string;
  initial?: string;
  submitLabel: string;
  onOpenChange: (open: boolean) => void;
  onSubmit: (name: string) => Promise<unknown>;
}) {
  const { t } = useTranslation();
  const [name, setName] = useState(initial);
  const [error, setError] = useState<unknown>(null);
  const [busy, setBusy] = useState(false);
  useEffect(() => {
    if (open) {
      setName(initial);
      setError(null);
    }
  }, [open, initial]);
  const valid = name.trim().length > 0 && name.trim().length <= 80;
  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-sm">
        <form
          className="space-y-4"
          onSubmit={(e) => {
            e.preventDefault();
            if (!valid || busy) return;
            setBusy(true);
            onSubmit(name.trim())
              .then(() => onOpenChange(false))
              .catch(setError)
              .finally(() => setBusy(false));
          }}
        >
          <DialogHeader>
            <DialogTitle>{title}</DialogTitle>
          </DialogHeader>
          <Input
            aria-label={t("practice.lists.name")}
            placeholder={t("practice.lists.name")}
            value={name}
            maxLength={80}
            autoFocus
            onChange={(e) => setName(e.target.value)}
          />
          {error !== null && <ErrorNotice error={error} />}
          <DialogFooter>
            <Button type="button" variant="outline" onClick={() => onOpenChange(false)}>
              {t("practice.lists.cancel")}
            </Button>
            <Button type="submit" disabled={!valid || busy}>
              {submitLabel}
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
}

/** Confirm deleting a list. */
export function ConfirmDialog({
  open,
  title,
  description,
  confirmLabel,
  onOpenChange,
  onConfirm,
}: {
  open: boolean;
  title: string;
  description: string;
  confirmLabel: string;
  onOpenChange: (open: boolean) => void;
  onConfirm: () => void;
}) {
  const { t } = useTranslation();
  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-sm">
        <DialogHeader>
          <DialogTitle>{title}</DialogTitle>
          <DialogDescription>{description}</DialogDescription>
        </DialogHeader>
        <DialogFooter>
          <Button variant="outline" onClick={() => onOpenChange(false)}>
            {t("practice.lists.cancel")}
          </Button>
          <Button
            variant="destructive"
            onClick={() => {
              onConfirm();
              onOpenChange(false);
            }}
          >
            {confirmLabel}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}

/** "Add to list": a checkbox per list, plus "New list…". */
export function AddToListMenu({
  problem,
  size = "icon",
}: {
  problem: string;
  size?: "icon" | "sm";
}) {
  const { t } = useTranslation();
  const lists = useLists();
  const holding = useListsWith(problem);
  const actions = useListActions();
  const [creating, setCreating] = useState(false);
  const inList = new Set(holding.data ?? []);
  return (
    <>
      <DropdownMenu>
        <DropdownMenuTrigger asChild>
          {size === "icon" ? (
            <Button variant="ghost" size="icon" className="size-7" aria-label={t("practice.lists.addTo")}>
              <ListPlus aria-hidden />
            </Button>
          ) : (
            <Button variant="outline" size="sm" className="gap-1.5" aria-label={t("practice.lists.addTo")}>
              <ListPlus aria-hidden />
              <span className="hidden xl:inline">{t("practice.lists.addTo")}</span>
            </Button>
          )}
        </DropdownMenuTrigger>
        <DropdownMenuContent align="end" className="min-w-56" onClick={(e) => e.stopPropagation()}>
          <DropdownMenuLabel>{t("practice.lists.addTo")}</DropdownMenuLabel>
          {(lists.data ?? []).map((l) => (
            <DropdownMenuCheckboxItem
              key={l.id}
              checked={inList.has(l.id)}
              onSelect={(e) => e.preventDefault()}
              onCheckedChange={(on) =>
                on ? actions.add.mutate({ id: l.id, problem }) : actions.drop.mutate({ id: l.id, problem })
              }
            >
              {l.name}
            </DropdownMenuCheckboxItem>
          ))}
          {(lists.data?.length ?? 0) > 0 && <DropdownMenuSeparator />}
          <DropdownMenuItem onSelect={() => setCreating(true)}>{t("practice.lists.newEllipsis")}</DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>
      <NameDialog
        open={creating}
        title={t("practice.lists.new")}
        submitLabel={t("practice.lists.create")}
        onOpenChange={setCreating}
        onSubmit={async (name) => {
          const id = await api.createList(name);
          await actions.add.mutateAsync({ id, problem });
        }}
      />
    </>
  );
}
