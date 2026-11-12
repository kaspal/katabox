import { BarChart3, BookOpen, Code2, FlaskConical, Home, Settings, Timer } from "lucide-react";

export type NavItem = {
  to: "/" | "/library" | "/interview" | "/progress" | "/settings" | "/dev/smoke";
  labelKey: "nav.home" | "nav.library" | "nav.interview" | "nav.progress" | "nav.settings" | "nav.smoke";
  icon: typeof Home;
};

export const NAV: NavItem[] = [
  { to: "/", labelKey: "nav.home", icon: Home },
  { to: "/library", labelKey: "nav.library", icon: BookOpen },
  { to: "/interview", labelKey: "nav.interview", icon: Timer },
  { to: "/progress", labelKey: "nav.progress", icon: BarChart3 },
  { to: "/settings", labelKey: "nav.settings", icon: Settings },
];

/** Development only: the temporary smoke screen. */
export const DEV_NAV: NavItem[] = import.meta.env.DEV
  ? [{ to: "/dev/smoke", labelKey: "nav.smoke", icon: FlaskConical }]
  : [];

export const WORKSPACE_ICON = Code2;
