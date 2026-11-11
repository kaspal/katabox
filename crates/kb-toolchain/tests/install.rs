//! Installs against a local HTTPS server (self-signed CA, scripted faults) with archives
//! built here: the download rules, the extraction rules, receipts, purge, resume, cancel,
//! uninstall and locking (ADR 0015).

use std::collections::HashMap;
use std::io::Write;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use kb_core::Language;
use kb_toolchain::manifest::{Manifest, Platform};
use kb_toolchain::{
    AllowList, ComponentPhase, Downloader, ErrorCode, InstallEvent, InstallOptions, Manager,
};
use sha2::{Digest, Sha256};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

// --- The test server -------------------------------------------------------------------

#[derive(Clone)]
enum Mode {
    Ok,
    /// 302 to this URL.
    Redirect(String),
    /// Promise the whole body, send this many bytes, close.
    CloseAfter(usize),
    /// Answer ranges with a Content-Range that does not match the request.
    BadRange,
    /// Send slowly (for cancel).
    Slow,
}

#[derive(Clone)]
struct Route {
    body: Arc<Vec<u8>>,
    mode: Mode,
}

#[derive(Default)]
struct State {
    routes: HashMap<String, Route>,
    /// (path, Range header) per request.
    hits: Vec<(String, Option<String>)>,
}

struct Server {
    port: u16,
    state: Arc<Mutex<State>>,
    client_tls: rustls::ClientConfig,
}

impl Server {
    async fn start() -> Server {
        let ck = rcgen::generate_simple_self_signed(vec!["localhost".to_string()]).unwrap();
        let cert_der = ck.cert.der().clone();
        let key = rustls::pki_types::PrivateKeyDer::Pkcs8(ck.signing_key.serialize_der().into());
        let provider = Arc::new(rustls::crypto::ring::default_provider());
        let server_cfg = rustls::ServerConfig::builder_with_provider(provider.clone())
            .with_safe_default_protocol_versions()
            .unwrap()
            .with_no_client_auth()
            .with_single_cert(vec![cert_der.clone()], key)
            .unwrap();
        let mut roots = rustls::RootCertStore::empty();
        roots.add(cert_der).unwrap();
        let client_tls = rustls::ClientConfig::builder_with_provider(provider)
            .with_safe_default_protocol_versions()
            .unwrap()
            .with_root_certificates(roots)
            .with_no_client_auth();
        let acceptor = tokio_rustls::TlsAcceptor::from(Arc::new(server_cfg));
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();
        let state: Arc<Mutex<State>> = Arc::default();
        let st = state.clone();
        tokio::spawn(async move {
            loop {
                let Ok((tcp, _)) = listener.accept().await else {
                    return;
                };
                let acceptor = acceptor.clone();
                let st = st.clone();
                tokio::spawn(async move {
                    if let Ok(tls) = acceptor.accept(tcp).await {
                        let _ = handle(tls, st).await;
                    }
                });
            }
        });
        Server {
            port,
            state,
            client_tls,
        }
    }

    fn url(&self, path: &str) -> String {
        format!("https://localhost:{}/{path}", self.port)
    }

    fn route(&self, path: &str, body: Vec<u8>, mode: Mode) {
        self.state.lock().unwrap().routes.insert(
            path.into(),
            Route {
                body: Arc::new(body),
                mode,
            },
        );
    }

    fn set_mode(&self, path: &str, mode: Mode) {
        self.state
            .lock()
            .unwrap()
            .routes
            .get_mut(path)
            .unwrap()
            .mode = mode;
    }

    fn hits(&self) -> Vec<(String, Option<String>)> {
        self.state.lock().unwrap().hits.clone()
    }

    fn downloader(&self) -> Downloader {
        Downloader::new(
            self.client_tls.clone(),
            AllowList::new(vec!["localhost".into()], true),
        )
        .unwrap()
    }
}

async fn handle(
    mut s: tokio_rustls::server::TlsStream<tokio::net::TcpStream>,
    st: Arc<Mutex<State>>,
) -> std::io::Result<()> {
    let mut buf = Vec::new();
    let mut tmp = [0u8; 1024];
    while !buf.windows(4).any(|w| w == b"\r\n\r\n") {
        let n = s.read(&mut tmp).await?;
        if n == 0 {
            return Ok(());
        }
        buf.extend_from_slice(&tmp[..n]);
    }
    let text = String::from_utf8_lossy(&buf).into_owned();
    let path = text
        .split_whitespace()
        .nth(1)
        .unwrap_or("/")
        .trim_start_matches('/')
        .to_string();
    let range = text
        .lines()
        .find_map(|l| {
            l.strip_prefix("range: ")
                .or_else(|| l.strip_prefix("Range: "))
        })
        .map(str::to_owned);
    let route = {
        let mut st = st.lock().unwrap();
        st.hits.push((path.clone(), range.clone()));
        st.routes.get(&path).cloned()
    };
    let Some(route) = route else {
        s.write_all(b"HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n")
            .await?;
        return s.shutdown().await;
    };
    let body = &route.body;
    let len = body.len();
    let start = range
        .as_deref()
        .and_then(|r| r.strip_prefix("bytes="))
        .and_then(|r| r.strip_suffix('-'))
        .and_then(|n| n.parse::<usize>().ok())
        .filter(|&n| n < len);
    match &route.mode {
        Mode::Redirect(to) => {
            let h = format!(
                "HTTP/1.1 302 Found\r\nLocation: {to}\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
            );
            s.write_all(h.as_bytes()).await?;
        }
        Mode::BadRange if start.is_some() => {
            let h = format!(
                "HTTP/1.1 206 Partial Content\r\nContent-Range: bytes 0-{}/{len}\r\nContent-Length: {len}\r\nConnection: close\r\n\r\n",
                len - 1
            );
            s.write_all(h.as_bytes()).await?;
            s.write_all(body).await?;
        }
        mode => {
            let (status, from) = match start {
                Some(n) => (
                    format!(
                        "206 Partial Content\r\nContent-Range: bytes {n}-{}/{len}",
                        len - 1
                    ),
                    n,
                ),
                None => ("200 OK".to_string(), 0),
            };
            let h = format!(
                "HTTP/1.1 {status}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                len - from
            );
            s.write_all(h.as_bytes()).await?;
            let rest = &body[from..];
            match mode {
                Mode::CloseAfter(n) => {
                    s.write_all(&rest[..(*n).min(rest.len())]).await?;
                    s.flush().await?;
                    return Ok(()); // drop without finishing the body
                }
                Mode::Slow => {
                    for chunk in rest.chunks(4096) {
                        s.write_all(chunk).await?;
                        s.flush().await?;
                        tokio::time::sleep(Duration::from_millis(20)).await;
                    }
                }
                _ => s.write_all(rest).await?,
            }
        }
    }
    s.shutdown().await
}

// --- Archives ----------------------------------------------------------------------------

enum E<'a> {
    File(&'a str, &'a [u8]),
    Dir(&'a str),
    Symlink(&'a str, &'a str),
    Hardlink(&'a str, &'a str),
}

/// A tar.gz with raw names (no validation, so tests can build hostile archives).
fn tar_gz(entries: &[E]) -> Vec<u8> {
    let mut b = tar::Builder::new(Vec::new());
    for e in entries {
        let mut h = tar::Header::new_old();
        let (name, ty, data, link): (&str, tar::EntryType, &[u8], &str) = match e {
            E::File(n, d) => (n, tar::EntryType::Regular, d, ""),
            E::Dir(n) => (n, tar::EntryType::Directory, b"", ""),
            E::Symlink(n, t) => (n, tar::EntryType::Symlink, b"", t),
            E::Hardlink(n, t) => (n, tar::EntryType::Link, b"", t),
        };
        let old = h.as_old_mut();
        old.name[..name.len()].copy_from_slice(name.as_bytes());
        old.linkname[..link.len()].copy_from_slice(link.as_bytes());
        h.set_entry_type(ty);
        h.set_size(data.len() as u64);
        h.set_mode(0o755);
        h.set_cksum();
        b.append(&h, data).unwrap();
    }
    let tar = b.into_inner().unwrap();
    let mut gz = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::fast());
    gz.write_all(&tar).unwrap();
    gz.finish().unwrap()
}

fn zip_file(entries: &[E]) -> Vec<u8> {
    let mut w = zip::ZipWriter::new(std::io::Cursor::new(Vec::new()));
    let opts =
        zip::write::SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);
    for e in entries {
        match e {
            E::File(n, d) => {
                w.start_file(*n, opts).unwrap();
                w.write_all(d).unwrap();
            }
            E::Dir(n) => w.add_directory(*n, opts).unwrap(),
            E::Symlink(n, t) => w.add_symlink(*n, *t, opts).unwrap(),
            E::Hardlink(..) => unreachable!(),
        }
    }
    w.finish().unwrap().into_inner()
}

/// Incompressible bytes (so archives keep their size on the wire).
fn noise(n: usize) -> Vec<u8> {
    let mut x: u64 = 0x2545_F491_4F6C_DD1D;
    (0..n)
        .map(|_| {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            (x >> 24) as u8
        })
        .collect()
}

fn sha(b: &[u8]) -> String {
    let h = Sha256::digest(b);
    h.iter().map(|x| format!("{x:02x}")).collect()
}

// --- Test manifests ------------------------------------------------------------------------

struct Art {
    id: &'static str,
    bytes: Vec<u8>,
    format: &'static str,
    size: Option<u64>,
    sha: Option<String>,
    urls: Option<Vec<String>>,
}

impl Art {
    fn new(id: &'static str, bytes: Vec<u8>, format: &'static str) -> Art {
        Art {
            id,
            bytes,
            format,
            size: None,
            sha: None,
            urls: None,
        }
    }
}

/// Serve the artifacts and build a manifest. `comps`: (id, dest, check, installed_size,
/// sources TOML); `langs`: (id, runtime, servers).
fn manifest(
    server: &Server,
    arts: &[Art],
    comps: &[(&str, &str, &str, u64, String)],
    langs: &[(&str, &[&str], &[&str])],
) -> &'static Manifest {
    let mut t = String::from("schema = 1\nhosts = [\"localhost\"]\n");
    for a in arts {
        server.route(a.id, a.bytes.clone(), Mode::Ok);
        let urls = a.urls.clone().unwrap_or_else(|| vec![server.url(a.id)]);
        let urls: Vec<String> = urls.iter().map(|u| format!("{u:?}")).collect();
        t += &format!(
            "[[artifact]]\nid = \"{}\"\nplatform = \"any\"\nurls = [{}]\nsize = {}\nsha256 = \"{}\"\nformat = \"{}\"\n",
            a.id,
            urls.join(", "),
            a.size.unwrap_or(a.bytes.len() as u64),
            a.sha.clone().unwrap_or_else(|| sha(&a.bytes)),
            a.format
        );
    }
    for (id, dest, check, size, sources) in comps {
        t += &format!(
            "[[component]]\nid = \"{id}\"\nplatform = \"any\"\ndest = \"{dest}\"\ncheck = \"{check}\"\ninstalled_size = {size}\n{sources}\n"
        );
    }
    for (id, rt, srv) in langs {
        let q = |v: &[&str]| {
            v.iter()
                .map(|x| format!("\"{x}\""))
                .collect::<Vec<_>>()
                .join(", ")
        };
        t += &format!(
            "[[language]]\nid = \"{id}\"\nruntime = [{}]\nservers = [{}]\n",
            q(rt),
            q(srv)
        );
    }
    Box::leak(Box::new(
        Manifest::parse_allowing_any_port(&t).unwrap_or_else(|e| panic!("{e}\n{t}")),
    ))
}

fn src(artifact: &str, subtree: &str) -> String {
    format!("[[component.source]]\nartifact = \"{artifact}\"\nsubtree = \"{subtree}\"\n")
}

fn root() -> tempfile::TempDir {
    let d = tempfile::tempdir().unwrap();
    std::fs::write(d.path().join(".katabox-sandbox-root"), b"").unwrap();
    d
}

fn platform() -> Option<Platform> {
    Some(Platform::current().unwrap_or(Platform::WindowsX86_64))
}

type Events = Arc<Mutex<Vec<InstallEvent>>>;

async fn install(
    m: &Manager,
    s: &Server,
    langs: &[Language],
) -> (kb_toolchain::Result<()>, Vec<InstallEvent>) {
    let ev: Events = Arc::default();
    let e2 = ev.clone();
    let r = m
        .install(
            langs,
            InstallOptions::default(),
            &s.downloader(),
            &move |e| e2.lock().unwrap().push(e),
        )
        .await;
    let v = ev.lock().unwrap().clone();
    (r, v)
}

fn count_files(p: &Path) -> usize {
    match std::fs::read_dir(p) {
        Ok(rd) => rd.count(),
        Err(_) => 0,
    }
}

/// The standard two-language setup: Python = py + node + lsp; TypeScript = node + tsls.
async fn two_languages(s: &Server) -> &'static Manifest {
    let py = tar_gz(&[
        E::Dir("py"),
        E::File("py/py.exe", b"python!"),
        E::File("py/lib/os.py", b"import x"),
    ]);
    let node = zip_file(&[E::Dir("node/"), E::File("node/node.exe", b"node!")]);
    let lsp = tar_gz(&[E::File("package/index.js", b"lsp")]);
    let tsls = tar_gz(&[E::File("package/cli.js", b"tsls")]);
    manifest(
        s,
        &[
            Art::new("py.tgz", py, "tar.gz"),
            Art::new("node.zip", node, "zip"),
            Art::new("lsp.tgz", lsp, "tar.gz"),
            Art::new("tsls.tgz", tsls, "tar.gz"),
        ],
        &[
            ("py", "toolchains/py", "py.exe", 100, src("py.tgz", "py")),
            (
                "node",
                "toolchains/node",
                "node.exe",
                100,
                src("node.zip", "node"),
            ),
            (
                "lsp",
                "tools/language-servers/node_modules/lsp",
                "index.js",
                100,
                src("lsp.tgz", "package"),
            ),
            (
                "tsls",
                "tools/language-servers/node_modules/tsls",
                "cli.js",
                100,
                src("tsls.tgz", "package"),
            ),
        ],
        &[
            ("python", &["py"], &["node", "lsp"]),
            ("typescript", &["node"], &["tsls"]),
        ],
    )
}

// --- Tests ------------------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread")]
async fn installs_a_language_with_its_server_and_writes_receipts() {
    let s = Server::start().await;
    let m = two_languages(&s).await;
    let r = root();
    let mgr = Manager::with_manifest(r.path(), m, platform()).unwrap();
    assert!(!mgr.runtime_installed(Language::Python));
    let (res, ev) = install(&mgr, &s, &[Language::Python]).await;
    res.unwrap();
    assert!(mgr.runtime_installed(Language::Python) && mgr.servers_installed(Language::Python));
    // TypeScript's runtime (node) came along; its server did not.
    assert!(
        mgr.runtime_installed(Language::TypeScript) && !mgr.servers_installed(Language::TypeScript)
    );
    assert_eq!(
        std::fs::read(r.path().join("toolchains/py/py.exe")).unwrap(),
        b"python!"
    );
    assert_eq!(
        std::fs::read(r.path().join("toolchains/py/lib/os.py")).unwrap(),
        b"import x"
    );
    for c in ["py", "node", "lsp"] {
        assert!(
            r.path()
                .join("receipts")
                .join(format!("{c}.json"))
                .is_file()
        );
    }
    // Events: the plan first, then each component reaches Installed.
    let InstallEvent::Plan { items } = &ev[0] else {
        panic!("{ev:?}")
    };
    assert_eq!(
        items
            .iter()
            .map(|i| i.component.as_str())
            .collect::<Vec<_>>(),
        ["py", "node", "lsp"]
    );
    for c in ["py", "node", "lsp"] {
        assert!(ev.iter().any(|e| matches!(e, InstallEvent::Progress { component, phase: ComponentPhase::Installed, .. } if component == c)));
    }
    // Nothing left behind.
    assert_eq!(count_files(&r.path().join("staging")), 0);
    assert_eq!(count_files(&r.path().join("downloads")), 0);
    // Installing again downloads nothing.
    let before = s.hits().len();
    let (res, ev) = install(&mgr, &s, &[Language::Python]).await;
    res.unwrap();
    assert_eq!(s.hits().len(), before);
    assert_eq!(ev, [InstallEvent::Plan { items: vec![] }]);
}

/// One component from one artifact, installed for `python`.
async fn single(
    s: &Server,
    art: Art,
    sources: String,
    check: &str,
) -> (tempfile::TempDir, Manager, kb_toolchain::Result<()>) {
    let m = manifest(
        s,
        &[art],
        &[("py", "toolchains/py", check, 1000, sources)],
        &[("python", &["py"], &[])],
    );
    let r = root();
    let mgr = Manager::with_manifest(r.path(), m, platform()).unwrap();
    let (res, _) = install(&mgr, s, &[Language::Python]).await;
    (r, mgr, res)
}

fn assert_nothing_installed(r: &Path) {
    assert!(!r.join("toolchains/py").exists(), "component folder exists");
    assert_eq!(count_files(&r.join("receipts")), 0, "receipt written");
    assert_eq!(count_files(&r.join("staging")), 0, "staging not cleaned");
}

fn code(r: &kb_toolchain::Result<()>) -> ErrorCode {
    r.as_ref().expect_err("should fail").code()
}

#[tokio::test(flavor = "multi_thread")]
async fn checksum_and_size_mismatches_are_refused_before_extraction() {
    let s = Server::start().await;
    let good = tar_gz(&[E::File("py/py.exe", b"x")]);

    let mut a = Art::new("bad-sha", good.clone(), "tar.gz");
    a.sha = Some("0".repeat(64));
    let (r, _, res) = single(&s, a, src("bad-sha", "py"), "py.exe").await;
    assert_eq!(code(&res), ErrorCode::ChecksumMismatch);
    assert_nothing_installed(r.path());
    assert_eq!(
        count_files(&r.path().join("downloads")),
        0,
        "a bad file is not kept"
    );

    // The server sends more than the manifest says: stopped as soon as it is exceeded.
    let mut a = Art::new("too-big", good.clone(), "tar.gz");
    a.size = Some(good.len() as u64 - 5);
    let (r, _, res) = single(&s, a, src("too-big", "py"), "py.exe").await;
    assert_eq!(code(&res), ErrorCode::SizeMismatch);
    assert_nothing_installed(r.path());
    assert_eq!(count_files(&r.path().join("downloads")), 0);

    // Less than the manifest says: an incomplete download (kept for a resume), not installed.
    let mut a = Art::new("too-small", good.clone(), "tar.gz");
    a.size = Some(good.len() as u64 + 5);
    let (r, _, res) = single(&s, a, src("too-small", "py"), "py.exe").await;
    assert_eq!(code(&res), ErrorCode::DownloadFailed);
    assert_nothing_installed(r.path());
}

#[tokio::test(flavor = "multi_thread")]
async fn truncated_download_resumes_with_a_range_request() {
    let s = Server::start().await;
    let body = tar_gz(&[E::File("py/py.exe", &noise(200_000))]);
    let half = body.len() / 2;
    let m = manifest(
        &s,
        &[Art::new("big", body.clone(), "tar.gz")],
        &[("py", "toolchains/py", "py.exe", 300_000, src("big", "py"))],
        &[("python", &["py"], &[])],
    );
    s.set_mode("big", Mode::CloseAfter(half));
    let r = root();
    let mgr = Manager::with_manifest(r.path(), m, platform()).unwrap();
    let (res, _) = install(&mgr, &s, &[Language::Python]).await;
    assert_eq!(code(&res), ErrorCode::DownloadFailed);
    let part = r
        .path()
        .join("downloads")
        .join(format!("{}.part", sha(&body)));
    assert_eq!(
        std::fs::metadata(&part).unwrap().len(),
        half as u64,
        "the partial file is kept"
    );
    assert_nothing_installed(r.path());

    // Retry: only the rest is requested.
    s.set_mode("big", Mode::Ok);
    let (res, _) = install(&mgr, &s, &[Language::Python]).await;
    res.unwrap();
    assert_eq!(
        s.hits().last().unwrap().1.as_deref(),
        Some(format!("bytes={half}-").as_str())
    );
    assert!(mgr.runtime_installed(Language::Python));
}

#[tokio::test(flavor = "multi_thread")]
async fn a_mismatched_content_range_restarts_from_zero() {
    let s = Server::start().await;
    let body = tar_gz(&[E::File("py/py.exe", &noise(50_000))]);
    let m = manifest(
        &s,
        &[Art::new("r", body.clone(), "tar.gz")],
        &[("py", "toolchains/py", "py.exe", 100_000, src("r", "py"))],
        &[("python", &["py"], &[])],
    );
    let r = root();
    std::fs::create_dir_all(r.path().join("downloads")).unwrap();
    std::fs::write(
        r.path()
            .join("downloads")
            .join(format!("{}.part", sha(&body))),
        &body[..1000],
    )
    .unwrap();
    s.set_mode("r", Mode::BadRange);
    let mgr = Manager::with_manifest(r.path(), m, platform()).unwrap();
    let (res, _) = install(&mgr, &s, &[Language::Python]).await;
    res.unwrap();
    let hits = s.hits();
    assert_eq!(hits[0].1.as_deref(), Some("bytes=1000-"));
    assert_eq!(hits[1].1, None, "restarted without a range");
}

#[tokio::test(flavor = "multi_thread")]
async fn redirects_must_stay_on_allowed_https_hosts() {
    let s = Server::start().await;
    let body = tar_gz(&[E::File("py/py.exe", b"x")]);
    for to in [
        format!("https://127.0.0.1:{}/real", s.port), // a host not on the list
        format!("http://localhost:{}/real", s.port),  // not https
    ] {
        let m = manifest(
            &s,
            &[
                Art::new("real", body.clone(), "tar.gz"),
                Art::new("hop", body.clone(), "tar.gz"),
            ],
            &[("py", "toolchains/py", "py.exe", 100, src("hop", "py"))],
            &[("python", &["py"], &[])],
        );
        s.set_mode("hop", Mode::Redirect(to.clone()));
        let r = root();
        let mgr = Manager::with_manifest(r.path(), m, platform()).unwrap();
        let (res, _) = install(&mgr, &s, &[Language::Python]).await;
        assert_eq!(code(&res), ErrorCode::HostNotAllowed, "{to}: {res:?}");
        assert_nothing_installed(r.path());
    }
    // An allowed redirect is followed.
    let m = manifest(
        &s,
        &[
            Art::new("real", body.clone(), "tar.gz"),
            Art::new("hop", body.clone(), "tar.gz"),
        ],
        &[("py", "toolchains/py", "py.exe", 100, src("hop", "py"))],
        &[("python", &["py"], &[])],
    );
    s.set_mode("hop", Mode::Redirect(s.url("real")));
    let r = root();
    let mgr = Manager::with_manifest(r.path(), m, platform()).unwrap();
    install(&mgr, &s, &[Language::Python]).await.0.unwrap();
}

#[tokio::test(flavor = "multi_thread")]
async fn hostile_archive_entries_are_rejected() {
    let s = Server::start().await;
    let cases: Vec<(&str, Vec<u8>, &str, &str)> = vec![
        (
            "tar-slip",
            tar_gz(&[
                E::File("py/py.exe", b"x"),
                E::File("py/../../evil.txt", b"x"),
            ]),
            "tar.gz",
            "",
        ),
        (
            "tar-abs",
            tar_gz(&[E::File("/tmp/evil.txt", b"x")]),
            "tar.gz",
            "",
        ),
        (
            "tar-backslash",
            tar_gz(&[E::File("py/..\\..\\evil.txt", b"x")]),
            "tar.gz",
            "",
        ),
        (
            "tar-drive",
            tar_gz(&[E::File("C:/evil.txt", b"x")]),
            "tar.gz",
            "",
        ),
        (
            "tar-device",
            tar_gz(&[E::File("py/NUL.txt", b"x")]),
            "tar.gz",
            "",
        ),
        (
            "tar-symlink",
            tar_gz(&[
                E::File("py/py.exe", b"x"),
                E::Symlink("py/link", "../../outside"),
            ]),
            "tar.gz",
            "",
        ),
        (
            "tar-hardlink",
            tar_gz(&[
                E::File("py/py.exe", b"x"),
                E::Hardlink("py/hard", "/etc/passwd"),
            ]),
            "tar.gz",
            "",
        ),
        (
            "tar-dup",
            tar_gz(&[E::File("py/py.exe", b"x"), E::File("py/py.exe", b"y")]),
            "tar.gz",
            "",
        ),
        (
            "zip-slip",
            zip_file(&[
                E::File("py/py.exe", b"x"),
                E::File("py/../../evil.txt", b"x"),
            ]),
            "zip",
            "",
        ),
        (
            "zip-abs",
            zip_file(&[E::File("/evil.txt", b"x")]),
            "zip",
            "",
        ),
        (
            "zip-symlink",
            zip_file(&[
                E::File("py/py.exe", b"x"),
                E::Symlink("py/link", "../../outside"),
            ]),
            "zip",
            "",
        ),
        // A listed link must stay inside and point to a regular file of the archive.
        (
            "link-escapes",
            tar_gz(&[
                E::File("py/py.exe", b"x"),
                E::Symlink("py/bin/python3", "../../../etc/passwd"),
            ]),
            "tar.gz",
            "bin/python3",
        ),
        (
            "link-dangling",
            tar_gz(&[
                E::File("py/py.exe", b"x"),
                E::Symlink("py/bin/python3", "nothing"),
            ]),
            "tar.gz",
            "bin/python3",
        ),
        (
            "link-to-dir",
            tar_gz(&[
                E::File("py/py.exe", b"x"),
                E::Dir("py/lib"),
                E::Symlink("py/bin/python3", "../lib"),
            ]),
            "tar.gz",
            "bin/python3",
        ),
    ];
    for (name, bytes, format, link) in cases {
        let mut sources = src(name, "py");
        if !link.is_empty() {
            sources += &format!("links = [\"{link}\"]\n");
        }
        let (r, _, res) = single(&s, Art::new(name, bytes, format), sources, "py.exe").await;
        assert_eq!(code(&res), ErrorCode::UnsafeArchive, "{name}: {res:?}");
        assert_nothing_installed(r.path());
        assert!(
            !r.path().join("evil.txt").exists()
                && !r.path().parent().unwrap().join("evil.txt").exists(),
            "{name}"
        );
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn listed_internal_links_are_copied_and_skipped_ones_left_out() {
    let s = Server::start().await;
    let a = tar_gz(&[
        E::File("py/bin/python3.14", b"the interpreter"),
        E::Symlink("py/bin/python3", "python3.14"),
        E::Symlink("py/share/terminfo/x", "../y"),
        E::File("py/install.sh", b"not ours"),
        E::File("README", b"outside the subtree"),
    ]);
    let mut sources = src("links", "py");
    sources += "links = [\"bin/python3\"]\nskip_links = [\"share/\"]\nexclude = [\"install.sh\"]\n";
    let (r, mgr, res) = single(&s, Art::new("links", a, "tar.gz"), sources, "bin/python3").await;
    res.unwrap();
    let p = r.path().join("toolchains/py/bin/python3");
    let meta = std::fs::symlink_metadata(&p).unwrap();
    assert!(
        meta.is_file() && !meta.file_type().is_symlink(),
        "copied, not linked"
    );
    assert_eq!(std::fs::read(p).unwrap(), b"the interpreter");
    assert!(!r.path().join("toolchains/py/share").exists());
    assert!(!r.path().join("toolchains/py/install.sh").exists());
    assert!(!r.path().join("toolchains/README").exists());
    assert!(mgr.runtime_installed(Language::Python));
}

#[tokio::test(flavor = "multi_thread")]
async fn an_archive_bigger_than_expected_is_stopped() {
    let s = Server::start().await;
    // 2 MB of zeros compresses to almost nothing: a small zip bomb against 1000 bytes.
    let a = tar_gz(&[E::File("py/py.exe", &vec![0u8; 3 << 20])]);
    let (r, _, res) = single(
        &s,
        Art::new("bomb", a, "tar.gz"),
        src("bomb", "py"),
        "py.exe",
    )
    .await;
    assert_eq!(code(&res), ErrorCode::UnsafeArchive);
    assert_nothing_installed(r.path());
}

#[tokio::test(flavor = "multi_thread")]
async fn cancel_mid_download_keeps_the_part_for_retry() {
    let s = Server::start().await;
    let body = tar_gz(&[E::File("py/py.exe", &noise(400_000))]);
    let m = manifest(
        &s,
        &[Art::new("slow", body.clone(), "tar.gz")],
        &[("py", "toolchains/py", "py.exe", 500_000, src("slow", "py"))],
        &[("python", &["py"], &[])],
    );
    s.set_mode("slow", Mode::Slow);
    let r = root();
    let mgr = Arc::new(Manager::with_manifest(r.path(), m, platform()).unwrap());
    let dl = s.downloader();
    let m2 = mgr.clone();
    let started = Arc::new(tokio::sync::Notify::new());
    let st2 = started.clone();
    let task = tokio::spawn(async move {
        m2.install(&[Language::Python], InstallOptions::default(), &dl, &move |e| {
            if matches!(e, InstallEvent::Progress { phase: ComponentPhase::Downloading, done, .. } if done > 0) {
                st2.notify_one();
            }
        })
        .await
    });
    started.notified().await;
    mgr.cancel();
    let res = task.await.unwrap();
    assert_eq!(code(&res), ErrorCode::InstallCancelled);
    let part = r
        .path()
        .join("downloads")
        .join(format!("{}.part", sha(&body)));
    let have = std::fs::metadata(&part).unwrap().len();
    assert!(have > 0 && have < body.len() as u64);
    assert_nothing_installed(r.path());
    // Retry resumes.
    s.set_mode("slow", Mode::Ok);
    install(&mgr, &s, &[Language::Python]).await.0.unwrap();
    assert_eq!(s.hits().last().unwrap().1, Some(format!("bytes={have}-")));
}

#[tokio::test(flavor = "multi_thread")]
async fn interrupted_installs_are_cleaned_up_and_never_count_as_installed() {
    let s = Server::start().await;
    let m = two_languages(&s).await;
    let r = root();
    let mgr = Manager::with_manifest(r.path(), m, platform()).unwrap();
    // A crash after the rename but before the receipt: files there, no receipt.
    std::fs::create_dir_all(r.path().join("toolchains/py")).unwrap();
    std::fs::write(r.path().join("toolchains/py/py.exe"), b"half").unwrap();
    // A crash mid-extraction, and an old trash folder.
    std::fs::create_dir_all(r.path().join("staging/abcd1234/0/py")).unwrap();
    std::fs::create_dir_all(r.path().join("staging/trash-1234")).unwrap();
    // A partial download longer than the artifact, and one of no current artifact.
    std::fs::create_dir_all(r.path().join("downloads")).unwrap();
    let py = &m.artifacts[0];
    std::fs::write(
        r.path()
            .join("downloads")
            .join(format!("{}.part", py.sha256)),
        vec![0u8; py.size as usize + 10],
    )
    .unwrap();
    std::fs::write(r.path().join("downloads").join("ffff.part"), b"old").unwrap();
    assert!(!mgr.runtime_installed(Language::Python));

    mgr.purge().unwrap();
    assert!(!r.path().join("toolchains/py").exists());
    assert_eq!(count_files(&r.path().join("staging")), 0);
    assert!(!r.path().join("downloads/ffff.part").exists());

    // The oversize .part is discarded at download time; the install succeeds.
    install(&mgr, &s, &[Language::Python]).await.0.unwrap();
    assert_eq!(
        std::fs::read(r.path().join("toolchains/py/py.exe")).unwrap(),
        b"python!"
    );

    // A receipt whose check file vanished (deleted by hand, quarantined): not installed.
    std::fs::remove_file(r.path().join("toolchains/py/py.exe")).unwrap();
    assert!(!mgr.runtime_installed(Language::Python));
    // A receipt from another manifest version (stale digest): not installed, replaced.
    install(&mgr, &s, &[Language::Python]).await.0.unwrap();
    let rp = r.path().join("receipts/node.json");
    let text = std::fs::read_to_string(&rp)
        .unwrap()
        .replace("\"digest\": \"", "\"digest\": \"0");
    std::fs::write(&rp, text).unwrap();
    assert!(!mgr.component_installed("node"));
    std::fs::write(r.path().join("toolchains/node/stale.txt"), b"old version").unwrap();
    install(&mgr, &s, &[Language::Python]).await.0.unwrap();
    assert!(mgr.component_installed("node"));
    assert!(
        !r.path().join("toolchains/node/stale.txt").exists(),
        "replaced as a whole folder"
    );
    // Garbage receipts are not installs.
    std::fs::write(r.path().join("receipts/lsp.json"), b"{not json").unwrap();
    assert!(!mgr.servers_installed(Language::Python));
}

#[tokio::test(flavor = "multi_thread")]
async fn uninstall_keeps_node_while_another_language_needs_it() {
    let s = Server::start().await;
    let m = two_languages(&s).await;
    let r = root();
    let mgr = Manager::with_manifest(r.path(), m, platform()).unwrap();
    install(&mgr, &s, &[Language::Python, Language::TypeScript])
        .await
        .0
        .unwrap();
    std::fs::create_dir_all(r.path().join("cache/rust")).unwrap();

    let removed = mgr.uninstall(Language::Python).unwrap();
    assert_eq!(removed, ["py", "lsp"]);
    assert!(!r.path().join("toolchains/py").exists());
    assert!(r.path().join("toolchains/node/node.exe").is_file());
    assert!(
        mgr.runtime_installed(Language::TypeScript) && mgr.servers_installed(Language::TypeScript)
    );

    // Two-step uninstall: after step 1 the language already counts as not installed.
    let u = mgr.uninstall_prepare(Language::TypeScript).unwrap();
    assert!(!mgr.runtime_installed(Language::TypeScript));
    assert!(
        r.path().join("toolchains/node").exists(),
        "folders go in step 2"
    );
    // Nothing else can install meanwhile.
    assert_eq!(
        install(&mgr, &s, &[Language::Python])
            .await
            .0
            .unwrap_err()
            .code(),
        ErrorCode::InstallBusy
    );
    mgr.uninstall_finish(u).unwrap();
    assert!(!r.path().join("toolchains/node").exists());
    assert_eq!(count_files(&r.path().join("receipts")), 0);
    assert_eq!(count_files(&r.path().join("staging")), 0);
}

#[tokio::test(flavor = "multi_thread")]
async fn not_enough_space_is_reported_before_downloading() {
    let s = Server::start().await;
    let a = tar_gz(&[E::File("py/py.exe", b"x")]);
    let m = manifest(
        &s,
        &[Art::new("huge", a, "tar.gz")],
        &[("py", "toolchains/py", "py.exe", 1 << 60, src("huge", "py"))],
        &[("python", &["py"], &[])],
    );
    let r = root();
    let mgr = Manager::with_manifest(r.path(), m, platform()).unwrap();
    let (res, ev) = install(&mgr, &s, &[Language::Python]).await;
    assert_eq!(code(&res), ErrorCode::NotEnoughSpace);
    assert!(
        matches!(ev[0], InstallEvent::Plan { .. }),
        "sizes are reported first"
    );
    assert!(s.hits().is_empty(), "nothing downloaded");
}

#[tokio::test(flavor = "multi_thread")]
async fn another_process_holding_the_lock_makes_installs_busy() {
    let s = Server::start().await;
    let m = two_languages(&s).await;
    let r = root();
    let mgr = Manager::with_manifest(r.path(), m, platform()).unwrap();
    let f = std::fs::OpenOptions::new()
        .create(true)
        .truncate(false)
        .write(true)
        .open(r.path().join(".install.lock"))
        .unwrap();
    fs4::FileExt::try_lock(&f).unwrap();
    // On Windows the lock is per handle; on Linux flock is per open file description.
    assert_eq!(
        install(&mgr, &s, &[Language::Python])
            .await
            .0
            .unwrap_err()
            .code(),
        ErrorCode::InstallBusy
    );
    assert_eq!(mgr.purge().unwrap_err().code(), ErrorCode::InstallBusy);
    fs4::FileExt::unlock(&f).unwrap();
    install(&mgr, &s, &[Language::Python]).await.0.unwrap();
}

#[tokio::test(flavor = "multi_thread")]
async fn roots_must_be_marked_and_platforms_supported() {
    let d = tempfile::tempdir().unwrap();
    assert!(Manager::open(d.path()).is_err(), "unmarked root");
    let s = Server::start().await;
    let m = two_languages(&s).await;
    let r = root();
    let mgr = Manager::with_manifest(r.path(), m, None).unwrap();
    let (res, _) = install(&mgr, &s, &[Language::Python]).await;
    assert_eq!(code(&res), ErrorCode::UnsupportedPlatform);
    assert!(mgr.status().iter().all(|l| !l.available));
}
