//! The bridge's message policy (ADR 0014): whatever the UI sends, a language server only
//! ever sees its virtual project, the bridge's safe settings, and an allow-listed set of
//! methods. Applied in Rust, between the UI and the server process.

use serde_json::{Value, json};

/// JSON-RPC "method not found", used for methods the policy blocks.
const BLOCKED: i64 = -32601;

/// What to do with a message from the UI.
#[derive(Debug, Clone, PartialEq)]
pub enum FromClient {
    /// Send this (possibly rewritten) message to the server.
    Forward(Value),
    /// Answer the UI directly with this message; the server never sees the request.
    Reply(Value),
    /// Drop it (a blocked notification).
    Drop,
}

/// What to do with a message from the server.
#[derive(Debug, Clone, PartialEq)]
pub enum FromServer {
    /// Relay it to the UI.
    Forward(Value),
    /// Answer the server directly with this message.
    Reply(Value),
    /// Drop it.
    Drop,
}

#[derive(Debug, Clone)]
pub struct Policy {
    /// `file:///…/` URI of the virtual project folder (trailing slash).
    pub root_uri: String,
    /// Settings handed to the server (initializationOptions and workspace/configuration).
    pub settings: Value,
}

const CLIENT_METHODS: &[&str] = &[
    "initialize",
    "initialized",
    "shutdown",
    "exit",
    "$/cancelRequest",
    "$/setTrace",
    "workspace/didChangeConfiguration",
];

fn error_reply(id: &Value, message: &str) -> Value {
    json!({ "jsonrpc": "2.0", "id": id, "error": { "code": BLOCKED, "message": message } })
}

fn result_reply(id: &Value, result: Value) -> Value {
    json!({ "jsonrpc": "2.0", "id": id, "result": result })
}

impl Policy {
    fn inside(&self, uri: &str) -> bool {
        uri.starts_with(&self.root_uri)
    }

    /// The settings object at a dotted `section` (`"basedpyright.analysis"`), or all of
    /// them; `null` when absent.
    pub fn section(&self, section: Option<&str>) -> Value {
        let mut v = &self.settings;
        if let Some(path) = section.filter(|s| !s.is_empty()) {
            for part in path.split('.') {
                match v.get(part) {
                    Some(next) => v = next,
                    None => return Value::Null,
                }
            }
        }
        v.clone()
    }

    /// Filter and rewrite a message from the UI.
    pub fn from_client(&self, mut msg: Value) -> FromClient {
        let id = msg.get("id").cloned();
        let Some(method) = msg.get("method").and_then(Value::as_str).map(str::to_owned) else {
            // A response to a server request.
            return if id.is_some() {
                FromClient::Forward(msg)
            } else {
                FromClient::Drop
            };
        };
        let refuse = |why: &str| match &id {
            Some(id) => FromClient::Reply(error_reply(id, why)),
            None => FromClient::Drop,
        };
        let document = method.starts_with("textDocument/");
        if !document && !CLIENT_METHODS.contains(&method.as_str()) {
            return refuse(&format!("{method} is blocked by KataBox (ADR 0014)"));
        }
        if document {
            let uri = msg
                .pointer("/params/textDocument/uri")
                .and_then(Value::as_str)
                .unwrap_or_default();
            if !self.inside(uri) {
                return refuse("documents outside the problem's project are blocked");
            }
        }
        match method.as_str() {
            "initialize" => {
                let params = json!({
                    "processId": std::process::id(),
                    "rootUri": self.root_uri,
                    "rootPath": Value::Null,
                    "workspaceFolders": [{ "uri": self.root_uri, "name": "katabox" }],
                    "initializationOptions": self.settings,
                    "capabilities": msg.pointer("/params/capabilities").cloned().unwrap_or(json!({})),
                    "clientInfo": { "name": "KataBox" },
                    "locale": msg.pointer("/params/locale").cloned().unwrap_or(Value::Null),
                });
                msg["params"] = params;
            }
            "workspace/didChangeConfiguration" => {
                msg["params"] = json!({ "settings": self.settings });
            }
            _ => {}
        }
        FromClient::Forward(msg)
    }

    /// The project URI spelling for `uri` when it names something inside the project,
    /// however the server spelled it (basedpyright and tsserver send `file:///c%3A/…` for
    /// `file:///C:/…`). The editor matches files by exact URI.
    fn canonical(&self, uri: &str) -> Option<String> {
        if !uri.starts_with("file:") || uri.starts_with(&self.root_uri) {
            return None;
        }
        let (got, root) = (percent_decode(uri), percent_decode(&self.root_uri));
        let n = root.len();
        let same = got.len() >= n
            && got.is_char_boundary(n)
            && if cfg!(windows) {
                got.as_bytes()[..n].eq_ignore_ascii_case(root.as_bytes())
            } else {
                got.as_bytes()[..n] == *root.as_bytes()
            };
        same.then(|| format!("{}{}", self.root_uri, percent_encode_path(&got[n..])))
    }

    /// Rewrite every project URI in a server message (values and object keys, such as
    /// `WorkspaceEdit.changes`) to the project's spelling.
    fn canonicalize(&self, v: &mut Value) {
        match v {
            Value::String(s) => {
                if let Some(c) = self.canonical(s) {
                    *s = c;
                }
            }
            Value::Array(items) => items.iter_mut().for_each(|i| self.canonicalize(i)),
            Value::Object(map) => {
                let renamed: Vec<(String, String)> = map
                    .keys()
                    .filter_map(|k| self.canonical(k).map(|c| (k.clone(), c)))
                    .collect();
                for (old, new) in renamed {
                    if let Some(val) = map.remove(&old) {
                        map.insert(new, val);
                    }
                }
                map.values_mut().for_each(|i| self.canonicalize(i));
            }
            _ => {}
        }
    }

    /// Filter a message from the server. Requests the UI need not see are answered here.
    pub fn from_server(&self, mut msg: Value) -> FromServer {
        self.canonicalize(&mut msg);
        let (Some(id), Some(method)) = (msg.get("id"), msg.get("method").and_then(Value::as_str))
        else {
            // A notification or a response.
            return match msg.get("method").and_then(Value::as_str) {
                Some("telemetry/event") => FromServer::Drop,
                _ => FromServer::Forward(msg),
            };
        };
        match method {
            "workspace/configuration" => {
                let items = msg
                    .pointer("/params/items")
                    .and_then(Value::as_array)
                    .cloned()
                    .unwrap_or_default();
                let result: Vec<Value> = items
                    .iter()
                    .map(|i| self.section(i.get("section").and_then(Value::as_str)))
                    .collect();
                FromServer::Reply(result_reply(id, Value::Array(result)))
            }
            "workspace/workspaceFolders" => FromServer::Reply(result_reply(
                id,
                json!([{ "uri": self.root_uri, "name": "katabox" }]),
            )),
            "client/registerCapability"
            | "client/unregisterCapability"
            | "window/workDoneProgress/create"
            | "workspace/diagnostic/refresh"
            | "workspace/semanticTokens/refresh"
            | "workspace/inlayHint/refresh"
            | "workspace/codeLens/refresh" => FromServer::Reply(result_reply(id, Value::Null)),
            "window/showDocument" => {
                FromServer::Reply(result_reply(id, json!({ "success": false })))
            }
            "workspace/applyEdit" => {
                if edit_uris(&msg).iter().all(|u| self.inside(u)) {
                    FromServer::Forward(msg)
                } else {
                    FromServer::Reply(result_reply(
                        id,
                        json!({ "applied": false, "failureReason": "outside the problem's project" }),
                    ))
                }
            }
            _ => FromServer::Forward(msg),
        }
    }
}

/// Decode `%XX` escapes (invalid ones are kept as is).
fn percent_decode(s: &str) -> String {
    let b = s.as_bytes();
    let mut out = Vec::with_capacity(b.len());
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'%'
            && let Some(hex) = s.get(i + 1..i + 3)
            && let Ok(byte) = u8::from_str_radix(hex, 16)
        {
            out.push(byte);
            i += 3;
        } else {
            out.push(b[i]);
            i += 1;
        }
    }
    String::from_utf8_lossy(&out).into_owned()
}

/// Percent-encode a path tail for a `file:` URI (unreserved characters and `/` stay).
fn percent_encode_path(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for &c in s.as_bytes() {
        if c.is_ascii_alphanumeric() || b"-._~/".contains(&c) {
            out.push(c as char);
        } else {
            out.push_str(&format!("%{c:02X}"));
        }
    }
    out
}

/// Every document URI a `workspace/applyEdit` touches.
fn edit_uris(msg: &Value) -> Vec<String> {
    let mut out = Vec::new();
    if let Some(changes) = msg
        .pointer("/params/edit/changes")
        .and_then(Value::as_object)
    {
        out.extend(changes.keys().cloned());
    }
    if let Some(dc) = msg
        .pointer("/params/edit/documentChanges")
        .and_then(Value::as_array)
    {
        for c in dc {
            for p in ["/textDocument/uri", "/uri", "/oldUri", "/newUri"] {
                if let Some(u) = c.pointer(p).and_then(Value::as_str) {
                    out.push(u.to_string());
                }
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn policy() -> Policy {
        Policy {
            root_uri: "file:///C:/data/lsp/two-sum/python/".into(),
            settings: json!({ "basedpyright": { "analysis": { "typeCheckingMode": "standard" } } }),
        }
    }

    fn forwarded(a: FromClient) -> Value {
        match a {
            FromClient::Forward(v) => v,
            other => panic!("expected Forward, got {other:?}"),
        }
    }

    #[test]
    fn initialize_is_pinned_to_the_project_and_safe_settings() {
        let msg = json!({
            "jsonrpc": "2.0", "id": 1, "method": "initialize",
            "params": {
                "processId": 1, "rootUri": "file:///C:/Users/me/",
                "workspaceFolders": [{ "uri": "file:///C:/", "name": "everything" }],
                "initializationOptions": { "evil": true },
                "capabilities": { "textDocument": { "hover": {} } }
            }
        });
        let out = forwarded(policy().from_client(msg));
        assert_eq!(
            out["params"]["rootUri"],
            "file:///C:/data/lsp/two-sum/python/"
        );
        assert_eq!(
            out["params"]["workspaceFolders"][0]["uri"],
            "file:///C:/data/lsp/two-sum/python/"
        );
        assert_eq!(
            out["params"]["workspaceFolders"].as_array().unwrap().len(),
            1
        );
        assert_eq!(out["params"]["initializationOptions"], policy().settings);
        assert_eq!(out["params"]["processId"], std::process::id());
        assert_eq!(
            out["params"]["capabilities"]["textDocument"]["hover"],
            json!({})
        );
    }

    #[test]
    fn blocks_commands_and_unknown_methods() {
        let p = policy();
        match p.from_client(json!({ "jsonrpc": "2.0", "id": 7, "method": "workspace/executeCommand", "params": { "command": "x" } })) {
            FromClient::Reply(r) => {
                assert_eq!(r["id"], 7);
                assert_eq!(r["error"]["code"], BLOCKED);
            }
            other => panic!("{other:?}"),
        }
        assert_eq!(
            p.from_client(json!({ "jsonrpc": "2.0", "method": "workspace/didChangeWatchedFiles", "params": {} })),
            FromClient::Drop
        );
        // didChangeConfiguration is allowed, but always carries the bridge's settings.
        let out = forwarded(p.from_client(json!({
            "jsonrpc": "2.0", "method": "workspace/didChangeConfiguration", "params": { "settings": { "evil": 1 } }
        })));
        assert_eq!(out["params"]["settings"], p.settings);
    }

    #[test]
    fn documents_must_be_inside_the_project() {
        let p = policy();
        let open = |uri: &str| {
            json!({
                "jsonrpc": "2.0", "method": "textDocument/didOpen",
                "params": { "textDocument": { "uri": uri, "languageId": "python", "version": 1, "text": "" } }
            })
        };
        assert!(matches!(
            p.from_client(open("file:///C:/data/lsp/two-sum/python/solution.py")),
            FromClient::Forward(_)
        ));
        assert_eq!(
            p.from_client(open("file:///C:/Users/me/secrets.py")),
            FromClient::Drop
        );
        // A sibling folder sharing the prefix is not inside (the root ends with '/').
        assert_eq!(
            p.from_client(open("file:///C:/data/lsp/two-sum/python2/x.py")),
            FromClient::Drop
        );
        match p.from_client(json!({
            "jsonrpc": "2.0", "id": 3, "method": "textDocument/hover",
            "params": { "textDocument": { "uri": "file:///C:/Windows/win.ini" }, "position": { "line": 0, "character": 0 } }
        })) {
            FromClient::Reply(r) => assert_eq!(r["error"]["code"], BLOCKED),
            other => panic!("{other:?}"),
        }
        // Responses to server requests pass through.
        assert!(matches!(
            p.from_client(json!({ "jsonrpc": "2.0", "id": 9, "result": null })),
            FromClient::Forward(_)
        ));
    }

    #[test]
    fn answers_configuration_and_refuses_documents_and_edits_outside() {
        let p = policy();
        let reply = |a: FromServer| match a {
            FromServer::Reply(r) => r,
            other => panic!("{other:?}"),
        };
        let r = reply(p.from_server(json!({
            "jsonrpc": "2.0", "id": 1, "method": "workspace/configuration",
            "params": { "items": [{ "section": "basedpyright.analysis" }, { "section": "nope" }, {}] }
        })));
        assert_eq!(r["result"][0], json!({ "typeCheckingMode": "standard" }));
        assert_eq!(r["result"][1], Value::Null);
        assert_eq!(r["result"][2], p.settings);
        assert_eq!(
            reply(p.from_server(json!({ "jsonrpc": "2.0", "id": 2, "method": "window/showDocument", "params": { "uri": "https://x" } })))["result"]["success"],
            false
        );
        let edit = |uri: &str| {
            json!({
                "jsonrpc": "2.0", "id": 3, "method": "workspace/applyEdit",
                "params": { "edit": { "changes": { uri: [] } } }
            })
        };
        assert!(matches!(
            p.from_server(edit("file:///C:/data/lsp/two-sum/python/solution.py")),
            FromServer::Forward(_)
        ));
        assert_eq!(
            reply(p.from_server(edit("file:///C:/Users/me/x.py")))["result"]["applied"],
            false
        );
        assert_eq!(
            p.from_server(json!({ "jsonrpc": "2.0", "method": "telemetry/event", "params": {} })),
            FromServer::Drop
        );
        assert!(matches!(
            p.from_server(json!({ "jsonrpc": "2.0", "method": "textDocument/publishDiagnostics", "params": {} })),
            FromServer::Forward(_)
        ));
    }

    #[test]
    fn server_uris_come_back_in_the_project_spelling() {
        let p = Policy {
            root_uri: "file:///C:/Users/Jos%C3%A9%20P/lsp/two-sum/python/".into(),
            settings: json!({}),
        };
        let file = "file:///C:/Users/Jos%C3%A9%20P/lsp/two-sum/python/solution.py";
        let forwarded = |msg: Value| match p.from_server(msg) {
            FromServer::Forward(m) => m,
            other => panic!("{other:?}"),
        };
        // Same spelling: untouched.
        let out = forwarded(
            json!({ "jsonrpc": "2.0", "method": "textDocument/publishDiagnostics",
            "params": { "uri": file, "diagnostics": [] } }),
        );
        assert_eq!(out["params"]["uri"], file);
        // Decoded and differently encoded spellings map back (values and edit keys).
        let out = forwarded(json!({ "jsonrpc": "2.0", "id": 1, "result": {
            "uri": "file:///C:/Users/José P/lsp/two-sum/python/solution.py",
            "changes": { "file:///C:/Users/Jos%c3%a9%20P/lsp/two-sum/python/solution.py": [] },
            "other": "file:///C:/elsewhere/x.py", "text": "c%3A",
        } }));
        assert_eq!(out["result"]["uri"], file);
        assert!(out["result"]["changes"].get(file).is_some());
        assert_eq!(out["result"]["other"], "file:///C:/elsewhere/x.py");
        assert_eq!(out["result"]["text"], "c%3A");
        // Drive letter case and an encoded colon, as basedpyright and tsserver send them.
        if cfg!(windows) {
            let out = forwarded(
                json!({ "jsonrpc": "2.0", "method": "textDocument/publishDiagnostics",
                "params": { "uri": "file:///c%3A/Users/Jos%C3%A9%20P/lsp/two-sum/python/solution.py" } }),
            );
            assert_eq!(out["params"]["uri"], file);
        }
    }
}
