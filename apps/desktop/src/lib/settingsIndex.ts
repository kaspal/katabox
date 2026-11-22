// What "Search settings" and "Search everything" find in Settings: each section's row
// labels, as i18n keys (moved out of Settings.tsx in expansion M4).
import type { SettingsSection } from "@/lib/pageTabs";

/** What "Search settings" finds: each section's row labels (i18n keys). */
export const SETTINGS_SEARCH_INDEX: [SettingsSection, string[]][] = [
  [
    "appearance",
    ["theme.label", "language.label", "settings.contentLang", "settings.contrast", "settings.motion", "settings.window.label", "settings.window.reset"],
  ],
  [
    "editor",
    ["settings.keymap", "settings.ligatures", "settings.fontSize", "settings.tabWidth", "settings.editorTheme", "settings.preview", "snippets.title"],
  ],
  ["practice", ["practice.settings.daily", "practice.settings.review", "plan.setting", "kata.rusty.setting", "welcome.again.label"]],
  ["languages", ["settings.sections.languages", "settings.calibration.title", "settings.calibration.recalibrate"]],
  ["learn", ["practice.settings.learnReview", "settings.learnSpeed"]],
  ["data", ["practice.data.export", "practice.data.import", "backup.title"]],
  ["packs", ["packs.title", "packs.import", "packs.includeInPractice", "packs.showCreate"]],
  [
    "about",
    [
      "settings.version",
      "settings.contentPack",
      "settings.licenses.label",
      "settings.userSchema",
      "settings.dataFolder",
      "settings.sandboxRoot",
      "settings.notices",
      "settings.docs",
      "settings.changelog",
      "shortcuts.title",
      "settings.diagnostics.label",
    ],
  ],
];
