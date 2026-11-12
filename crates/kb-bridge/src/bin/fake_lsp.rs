//! Test-only stand-in for a language server (kb-bridge lifecycle tests). Never shipped.
//!
//! - `initialize` → minimal capabilities; remembers the params.
//! - `textDocument/didOpen` → a diagnostic for that document (its `data` reports the
//!   environment, root and settings this process received), then asks the client for
//!   `workspace/configuration` and reports the answer as a `fake/configured` notification.
//! - `textDocument/didChange` whose text is `crash` → exits with code 9 (a "bug").
//! - `shutdown` / `exit` as in LSP. `--die-at-start` exits at once with code 2.

use std::io::{BufRead, BufReader, Write};

use serde_json::{Value, json};

fn read(r: &mut impl BufRead) -> Option<Value> {
    let mut len = None;
    loop {
        let mut line = String::new();
        if r.read_line(&mut line).ok()? == 0 {
            return None;
        }
        let line = line.trim_end();
        if line.is_empty() {
            break;
        }
        if let Some((k, v)) = line.split_once(':')
            && k.eq_ignore_ascii_case("content-length")
        {
            len = v.trim().parse::<usize>().ok();
        }
    }
    let mut body = vec![0; len?];
    r.read_exact(&mut body).ok()?;
    serde_json::from_slice(&body).ok()
}

fn write(v: &Value) {
    let body = v.to_string();
    let mut out = std::io::stdout().lock();
    let _ = write!(out, "Content-Length: {}\r\n\r\n{body}", body.len());
    let _ = out.flush();
}

fn reply(id: &Value, result: Value) {
    write(&json!({ "jsonrpc": "2.0", "id": id, "result": result }));
}

fn main() {
    if std::env::args().any(|a| a == "--die-at-start") {
        eprintln!("fake-lsp: told to die at start");
        std::process::exit(2);
    }
    let mut input = BufReader::new(std::io::stdin().lock());
    let mut init_params = Value::Null;
    while let Some(msg) = read(&mut input) {
        let method = msg
            .get("method")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_string();
        let id = msg.get("id").cloned();
        match (method.as_str(), id) {
            ("initialize", Some(id)) => {
                init_params = msg["params"].clone();
                reply(
                    &id,
                    json!({ "capabilities": { "hoverProvider": true }, "serverInfo": { "name": "fake" } }),
                );
            }
            ("textDocument/didOpen", None) => {
                let uri = msg["params"]["textDocument"]["uri"].clone();
                write(&json!({
                    "jsonrpc": "2.0", "method": "textDocument/publishDiagnostics",
                    "params": { "uri": uri, "diagnostics": [{
                        "range": { "start": { "line": 0, "character": 0 }, "end": { "line": 0, "character": 1 } },
                        "severity": 1, "message": "fake diagnostic",
                        // What this process actually received, for the tests.
                        "data": {
                            "env": std::env::vars().map(|(k, _)| k).collect::<Vec<_>>(),
                            "rootUri": init_params["rootUri"],
                            "workspaceFolders": init_params["workspaceFolders"],
                            "initializationOptions": init_params["initializationOptions"],
                            "cwd": std::env::current_dir().map(|d| d.display().to_string()).unwrap_or_default()
                        }
                    }] }
                }));
                write(&json!({
                    "jsonrpc": "2.0", "id": "cfg", "method": "workspace/configuration",
                    "params": { "items": [{ "section": "fake" }] }
                }));
            }
            ("", Some(id)) if id == json!("cfg") => {
                write(
                    &json!({ "jsonrpc": "2.0", "method": "fake/configured", "params": msg["result"] }),
                );
            }
            ("textDocument/didChange", None)
                if msg.pointer("/params/contentChanges/0/text") == Some(&json!("crash")) =>
            {
                std::process::exit(9)
            }
            ("shutdown", Some(id)) => reply(&id, Value::Null),
            ("exit", None) => std::process::exit(0),
            (_, Some(id)) if !method.is_empty() => reply(&id, Value::Null),
            _ => {}
        }
    }
}
