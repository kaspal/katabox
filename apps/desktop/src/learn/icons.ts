// Subject icons (content/learn/subjects.toml `icon`), with a tint per subject for the
// chip behind the icon. kb-pack checks every subject's icon is a key here; unknown names
// fall back to a book. Tints are decorative: the subject name is always shown.
import {
  BookOpenText,
  ChartColumn,
  ChartLine,
  Database,
  Grid3x3,
  Hash,
  Network,
  Rows3,
  Search,
  Sigma,
  Table,
  TrendingUp,
  Type,
  Undo2,
  Waypoints,
  type LucideIcon,
} from "lucide-react";

export type SubjectIcon = { icon: LucideIcon; tint: string };

export const SUBJECT_ICONS: Record<string, SubjectIcon> = {
  "chart-line": { icon: ChartLine, tint: "indigo" },
  "chart-column": { icon: ChartColumn, tint: "teal" },
  "search": { icon: Search, tint: "amber" },
  "rows": { icon: Rows3, tint: "slate" },
  "hash": { icon: Hash, tint: "pink" },
  "tree": { icon: Network, tint: "green" },
  "graph": { icon: Waypoints, tint: "indigo" },
  "grid": { icon: Grid3x3, tint: "yellow" },
  "table": { icon: Table, tint: "orange" },
  "trending-up": { icon: TrendingUp, tint: "teal" },
  "undo": { icon: Undo2, tint: "violet" },
  "type": { icon: Type, tint: "slate" },
  "sigma": { icon: Sigma, tint: "pink" },
  "database": { icon: Database, tint: "amber" },
};

export const subjectIcon = (name: string): SubjectIcon =>
  SUBJECT_ICONS[name] ?? { icon: BookOpenText, tint: "slate" };
