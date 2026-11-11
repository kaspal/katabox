import React from "react";
import ReactDOM from "react-dom/client";
import App from "@/App";
import { initI18n } from "@/i18n";
import { applyTheme, initialPrefs } from "@/lib/prefs";
import "@/index.css";

// First paint in the remembered theme and language; user.db confirms them right after.
const initial = initialPrefs();
applyTheme(initial.theme);
initI18n(initial.lang);

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
