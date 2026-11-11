// The editor's language server (ADR 0014): connects through the bridge when the workspace
// opens a language, and yields a CodeMirror extension (completion, hover, signature help,
// diagnostics, go to definition, format). The editor works the same without one.
import { useCallback, useEffect, useRef, useState } from "react";
import type { Extension } from "@codemirror/state";
import { LSPClient, languageServerExtensions, type Transport } from "@codemirror/lsp-client";
import type { Language, LspConnection, LspEvent, LspStatus } from "@/bindings";
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

type Connected = { language: Language; conn: LspConnection };

/**
 * `unavailable`: the reason from app_info when this language has no server here (then
 * nothing is started). `text` is read once per connection to seed the solution file.
 *
 * Every connect gets its own bridge session (a server is initialized once per process);
 * a client is created per server process: again when the bridge restarts it after a crash.
 */
export function useLanguageServer(
  problem: string,
  language: Language | null,
  unavailable: string | null,
  text: () => string,
): LanguageServer {
  const [status, setStatus] = useState<ServerState>({ state: "off" });
  const [extension, setExtension] = useState<Extension | null>(null);
  const [connected, setConnected] = useState<Connected | null>(null);
  // Bumped to connect again: waking a server that stopped while idle.
  const [connectGen, setConnectGen] = useState(0);
  // Bumped for a new client on the same session: the server restarted after a crash.
  const [clientGen, setClientGen] = useState(0);
  const statusRef = useRef<ServerState>(status);
  const textRef = useRef(text);
  textRef.current = text;
  // The current client's message handlers (the transport subscribes them).
  const handlers = useRef(new Set<(m: string) => void>());

  // The session: connect, follow its status, disconnect when the editor leaves.
  useEffect(() => {
    setConnected(null);
    if (!language) {
      setStatus({ state: "off" });
      return;
    }
    if (unavailable !== null) {
      setStatus({ state: "unavailable", reason: unavailable });
      return;
    }
    let live = true;
    let wasReady = false;
    let session: number | null = null;
    const update = (s: ServerState) => {
      statusRef.current = s;
      if (live) setStatus(s);
    };
    const onEvent = (e: LspEvent) => {
      if (!live) return;
      if (e.kind === "message") {
        handlers.current.forEach((h) => h(e.message));
        return;
      }
      update(e.status);
      if (e.status.state === "ready") wasReady = true;
      if (e.status.state === "starting" && wasReady) setClientGen((g) => g + 1);
      if (e.status.state === "unavailable" || e.status.state === "stopped") setConnected(null);
    };

    update({ state: "connecting" });
    api
      .lspConnect(problem, language, textRef.current(), onEvent)
      .then((conn) => {
        session = conn.session;
        // Cleaned up before the bridge answered (React's dev double effects, quick
        // navigation): release the session right away.
        if (!live) return void api.lspDisconnect(language, conn.session).catch(() => {});
        setConnected({ language, conn });
      })
      .catch((e) => {
        if (!live) return;
        update({ state: "unavailable", reason: isAppError(e) ? e.message : String(e) });
      });

    return () => {
      live = false;
      if (session !== null) void api.lspDisconnect(language, session).catch(() => {});
    };
  }, [problem, language, unavailable, connectGen]);

  // The client: one per server process of the session.
  useEffect(() => {
    if (!connected) {
      setExtension(null);
      return;
    }
    const { language, conn } = connected;
    let live = true;
    // Requests this client is still waiting for (id → method), and its message handler.
    const pending = new Map<number | string, string>();
    let own: ((m: string) => void) | null = null;
    const track = (message: string) => {
      try {
        const m = JSON.parse(message) as { id?: number | string; method?: string };
        if (m.id !== undefined && m.method === undefined) pending.delete(m.id);
      } catch {
        // not JSON: nothing to track
      }
    };
    const transport: Transport = {
      send(message) {
        try {
          const m = JSON.parse(message) as { id?: number | string; method?: string };
          if (m.id !== undefined && m.method !== undefined) pending.set(m.id, m.method);
        } catch {
          // not JSON: nothing to track
        }
        api.lspSend(language, conn.session, message).catch((e) => {
          // The session ended (stopped while idle): show it; typing wakes it.
          if (live && isAppError(e) && e.code === "language_server_not_connected") {
            statusRef.current = { state: "stopped" };
            setStatus(statusRef.current);
          }
        });
      },
      subscribe: (h) => {
        own = (m) => {
          track(m);
          h(m);
        };
        handlers.current.add(own);
      },
      unsubscribe: () => {
        if (own) handlers.current.delete(own);
      },
    };
    const client = new LSPClient({
      rootUri: conn.root_uri,
      timeout: 5000,
      sanitizeHTML,
      extensions: languageServerExtensions(),
    }).connect(transport);
    setExtension(client.plugin(conn.file_uri, conn.language_id));
    return () => {
      live = false;
      // A replaced client's requests will never be answered (its session or process is
      // gone). Answer them here, so none times out later as an unhandled rejection:
      // initialize with no capabilities, everything else with "nothing".
      const deliver = own;
      for (const [id, method] of pending) {
        const result = method === "initialize" ? { capabilities: {} } : null;
        deliver?.(JSON.stringify({ jsonrpc: "2.0", id, result }));
      }
      pending.clear();
      client.disconnect();
      setExtension(null);
    };
  }, [connected, clientGen]);

  const wake = useCallback(() => {
    if (statusRef.current.state === "stopped") {
      statusRef.current = { state: "connecting" };
      setConnectGen((g) => g + 1);
    }
  }, []);

  return { status, extension, wake };
}
