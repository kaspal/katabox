import { BarChart3, BookOpen, Code2, FlaskConical, Home, Settings } from "lucide-react";

export type NavItem = {
  to: "/" | "/library" | "/progress" | "/settings" | "/dev/smoke";
  labelKey: "nav.home" | "nav.library" | "nav.progress" | "nav.settings" | "nav.smoke";
  icon: typeof Home;
};

export const NAV: NavItem[] = [
  { to: "/", labelKey: "nav.home", icon: Home },
  { to: "/library", labelKey: "nav.library", icon: BookOpen },
  { to: "/progress", labelKey: "nav.progress", icon: BarChart3 },
  { to: "/settings", labelKey: "nav.settings", icon: Settings },
];

/** Development only: the temporary smoke screen. */
export const DEV_NAV: NavItem[] = import.meta.env.DEV
  ? [{ to: "/dev/smoke", labelKey: "nav.smoke", icon: FlaskConical }]
  : [];

export const WORKSPACE_ICON = Code2;
