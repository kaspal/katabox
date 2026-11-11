import { cn } from "@/lib/utils";

/** A page with a heading; content scrolls inside the shell. */
export function Page({
  title,
  actions,
  children,
  className,
}: {
  title: string;
  actions?: React.ReactNode;
  children: React.ReactNode;
  className?: string;
}) {
  return (
    <section className={cn("flex h-full min-h-0 flex-col gap-4 p-6", className)}>
      <header className="flex items-center justify-between gap-4">
        <h1 className="text-xl font-semibold tracking-tight">{title}</h1>
        {actions}
      </header>
      {children}
    </section>
  );
}
