// A small read-only editor showing the current editor preferences (Settings → Editor).
// Loaded lazily by the Settings page, so CodeMirror stays out of the main chunk.
import { useTranslation } from "react-i18next";
import { useEditorPrefs } from "@/lib/settings";
import { CodeEditor } from "@/workspace/editor/CodeEditor";

const SAMPLE = `# Two values that add up to the target
def pair_sum(values: list[int], target: int) -> list[int]:
    seen = {}  # value -> index
    for i, v in enumerate(values):
        if target - v in seen:
            return [seen[target - v], i]
        seen[v] = i
    return []
`;

export default function EditorPreview() {
  const { t } = useTranslation();
  const { keymap, fontSize, ligatures, tabWidth, theme } = useEditorPrefs();
  return (
    <div className="overflow-hidden rounded-md border" data-testid="editor-preview">
      <CodeEditor
        docKey="settings-preview"
        value={SAMPLE}
        lang="python"
        keymap={keymap}
        fontSize={fontSize}
        ligatures={ligatures}
        tabWidth={tabWidth}
        colorTheme={theme}
        readOnly
        ariaLabel={t("settings.previewLabel")}
        className="h-56 min-h-0 overflow-hidden"
      />
    </div>
  );
}
