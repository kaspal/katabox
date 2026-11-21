import { BarChart3, BookOpen, Code2, Dumbbell, FlaskConical, GraduationCap, Home, Settings, Timer } from "lucide-react";

export type NavItem = {
  to: "/" | "/library" | "/learn" | "/kata" | "/interview" | "/progress" | "/settings" | "/dev/smoke";
  labelKey:
    | "nav.home"
    | "nav.library"
    | "nav.learn"
    | "nav.kata"
    | "nav.interview"
    | "nav.progress"
    | "nav.settings"
    | "nav.smoke";
  icon: typeof Home;
};

export const NAV: NavItem[] = [
  { to: "/", labelKey: "nav.home", icon: Home },
  { to: "/library", labelKey: "nav.library", icon: BookOpen },
  { to: "/learn", labelKey: "nav.learn", icon: GraduationCap },
  { to: "/kata", labelKey: "nav.kata", icon: Dumbbell },
  { to: "/interview", labelKey: "nav.interview", icon: Timer },
  { to: "/progress", labelKey: "nav.progress", icon: BarChart3 },
  { to: "/settings", labelKey: "nav.settings", icon: Settings },
];

/** Development only: the temporary smoke screen. */
export const DEV_NAV: NavItem[] = import.meta.env.DEV
  ? [{ to: "/dev/smoke", labelKey: "nav.smoke", icon: FlaskConical }]
  : [];

export const WORKSPACE_ICON = Code2;
