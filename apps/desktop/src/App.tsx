import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

type AppInfo = { version: string; languages: string[] };

const LABELS: Record<string, string> = {
  python: "Python",
  javascript: "JavaScript",
  typescript: "TypeScript",
  php: "PHP",
  rust: "Rust",
};

export default function App() {
  const [info, setInfo] = useState<AppInfo | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    invoke<AppInfo>("app_info").then(setInfo).catch((e) => setError(String(e)));
  }, []);

  return (
    <main className="shell">
      <header className="brand">
        <span className="logo" aria-hidden>
          ▣
        </span>
        <h1>KataBox</h1>
        {info && <span className="version">v{info.version}</span>}
      </header>
      <p className="tagline">Offline coding practice: problems, tests, editor and debugger.</p>
      {error && <p className="error">Could not reach the core: {error}</p>}
      {info && (
        <ul className="languages" aria-label="Supported languages">
          {info.languages.map((id) => (
            <li key={id}>{LABELS[id] ?? id}</li>
          ))}
        </ul>
      )}
      <p className="hint">Skeleton build. Next: design tokens, problem library, workspace.</p>
    </main>
  );
}
