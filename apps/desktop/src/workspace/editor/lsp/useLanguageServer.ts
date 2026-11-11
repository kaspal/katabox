// The editor's language server (ADR 0014): connects through the bridge when the workspace
// opens a language, and yields a CodeMirror extension (completion, hover, signature help,
// diagnostics, go to definition, format). The editor works the same without one.
import { useCallback, useEffect, useRef, useState } from "react";
import type { Extension } from "@codemirror/state";
import { LSPClient, languageServerExtensions, type Transport } from "@codemirror/lsp-client";
import type { Language, LspEvent, LspStatus } from "@/bindings";
import { api } from "@/lib/api";
import { isAppError } from "@/lib/errors";
import { sanitizeHTML } from "@/workspace/editor/lsp/sanitize";

/** What the status line shows. `connecting` covers the round trip before the bridge answers. */
export type ServerState = { state: "off" } | { state: "connecting" } | LspStatus;

export type LanguageServer = {
  status: ServerState;
  /** For CodeEditor's `lspExtension` (null while there is no server). */
  extension: Extension | null;
  /** Call on edits: restarts a server that stopped while idle. */
  wake: () => void;
};

/**
 * `unavailable`: the reason from app_info when this language has no server here (then
 * nothing is started). `text` is read once per connection to seed the solution file.
 */
export function useLanguageServer(
  problem: string,
  language: Language | null,
  unavailable: string | null,
  text: () => string,
): LanguageServer {
  const [status, setStatus] = useState<ServerState>({ state: "off" });
  const [extension, setExtension] = useState<Extension | null>(null);
  // Bumped to reconnect: after a crash restart, or when waking an idle server.
  const [generation, setGeneration] = useState(0);
  const statusRef = useRef<ServerState>(status);
  const textRef = useRef(text);
  textRef.current = text;

  useEffect(() => {
    if (!language) {
      setStatus({ state: "off" });
      setExtension(null);
      return;
    }
    if (unavailable !== null) {
      setStatus({ state: "unavailable", reason: unavailable });
      setExtension(null);
      return;
    }
    let live = true;
    let wasReady = false;
    const handlers = new Set<(m: string) => void>();
    const update = (s: ServerState) => {
      statusRef.current = s;
      if (live) setStatus(s);
    };
    const transport: Transport = {
      send(message) {
        api.lspSend(language, message).catch((e) => {
          if (isAppError(e) && e.code === "language_server_not_connected" && live) {
            update({ state: "stopped" });
          }
        });
      },
      subscribe: (h) => void handlers.add(h),
      unsubscribe: (h) => void handlers.delete(h),
    };
    const onEvent = (e: LspEvent) => {
      if (!live) return;
      if (e.kind === "message") {
        handlers.forEach((h) => h(e.message));
        return;
      }
      update(e.status);
      if (e.status.state === "ready") wasReady = true;
      // The server restarted after a crash: a fresh client re-initializes it.
      if (e.status.state === "starting" && wasReady) setGeneration((g) => g + 1);
      if (e.status.state === "unavailable" || e.status.state === "stopped") setExtension(null);
    };

    update({ state: "connecting" });
    let client: LSPClient | null = null;
    api
      .lspConnect(problem, language, textRef.current(), onEvent)
      .then((conn) => {
        if (!live) return;
        client = new LSPClient({
          rootUri: conn.root_uri,
          timeout: 5000,
          sanitizeHTML,
          extensions: languageServerExtensions(),
        }).connect(transport);
        setExtension(client.plugin(conn.file_uri, conn.language_id));
      })
      .catch((e) => {
        if (!live) return;
        const reason = isAppError(e) ? e.message : String(e);
        update({ state: "unavailable", reason });
        setExtension(null);
      });

    return () => {
      live = false;
      client?.disconnect();
      setExtension(null);
    };
  }, [problem, language, unavailable, generation]);

  const wake = useCallback(() => {
    if (statusRef.current.state === "stopped") {
      statusRef.current = { state: "connecting" };
      setGeneration((g) => g + 1);
    }
  }, []);

  return { status, extension, wake };
}
