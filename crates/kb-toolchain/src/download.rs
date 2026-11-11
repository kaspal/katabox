//! The downloader (ADR 0015): HTTPS only, to allow-listed hosts only (redirects too),
//! the system proxy only, resumable `.part` files, and size + SHA-256 verified before a
//! file is accepted.

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use futures_util::StreamExt;
use reqwest::{StatusCode, Url, header};
use sha2::{Digest, Sha256};
use tokio::io::AsyncWriteExt;
use tokio_util::sync::CancellationToken;

use crate::manifest::{Artifact, Manifest, hex};
use crate::{Error, Result};

/// Hosts the downloader may contact, for the first request and every redirect.
#[derive(Debug, Clone)]
pub struct AllowList {
    hosts: Vec<String>,
    any_port: bool,
}

impl AllowList {
    /// The manifest's hosts, https on port 443 only. The only list the app uses.
    pub fn builtin() -> AllowList {
        AllowList {
            hosts: Manifest::builtin().hosts.clone(),
            any_port: false,
        }
    }

    /// A custom list. `any_port` exists for the local test server; the app never sets it.
    pub fn new(hosts: Vec<String>, any_port: bool) -> AllowList {
        AllowList { hosts, any_port }
    }

    fn check(&self, url: &Url) -> std::result::Result<(), String> {
        if url.scheme() != "https" {
            return Err("not https".into());
        }
        if !url.username().is_empty() || url.password().is_some() {
            return Err("user info in the URL".into());
        }
        if !self.any_port && url.port().is_some_and(|p| p != 443) {
            return Err("only port 443".into());
        }
        match url.host_str() {
            Some(h) if self.hosts.iter().any(|a| a.eq_ignore_ascii_case(h)) => Ok(()),
            _ => Err("host not on the allow-list".into()),
        }
    }
}

/// A redirect the policy refused (found again in the error's source chain).
#[derive(Debug)]
struct Refused(String);
impl std::fmt::Display for Refused {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}
impl std::error::Error for Refused {}

const MAX_REDIRECTS: usize = 5;

#[derive(Clone)]
pub struct Downloader {
    client: reqwest::Client,
    allow: Arc<AllowList>,
}

impl std::fmt::Debug for Downloader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Downloader")
            .field("allow", &self.allow)
            .finish_non_exhaustive()
    }
}

impl Downloader {
    /// The app's downloader: the OS certificate store (rustls-platform-verifier), the
    /// compiled-in allow-list, the system proxy.
    pub fn system() -> Result<Downloader> {
        use rustls_platform_verifier::BuilderVerifierExt;
        let provider = Arc::new(rustls::crypto::ring::default_provider());
        let tls = rustls::ClientConfig::builder_with_provider(provider)
            .with_safe_default_protocol_versions()
            .and_then(|b| b.with_platform_verifier())
            .map_err(|e| Error::Manifest(format!("TLS setup: {e}")))?
            .with_no_client_auth();
        Downloader::new(tls, AllowList::builtin())
    }

    /// A downloader with a given TLS configuration and allow-list (tests use their own
    /// certificate authority; the app only uses [`Downloader::system`]).
    pub fn new(tls: rustls::ClientConfig, allow: AllowList) -> Result<Downloader> {
        let allow = Arc::new(allow);
        let policy_allow = allow.clone();
        let policy = reqwest::redirect::Policy::custom(move |attempt| {
            if attempt.previous().len() >= MAX_REDIRECTS {
                return attempt.error(Refused(format!("more than {MAX_REDIRECTS} redirects")));
            }
            match policy_allow.check(attempt.url()) {
                Ok(()) => attempt.follow(),
                Err(why) => {
                    let msg = format!("redirect to {} refused: {why}", attempt.url());
                    attempt.error(Refused(msg))
                }
            }
        });
        let client = reqwest::Client::builder()
            .tls_backend_preconfigured(tls)
            .https_only(true)
            .redirect(policy)
            .connect_timeout(Duration::from_secs(30))
            // Idle time between reads, not a total: large downloads on slow links are fine.
            .read_timeout(Duration::from_secs(60))
            .user_agent(concat!("KataBox/", env!("CARGO_PKG_VERSION")))
            .build()
            .map_err(|e| Error::Manifest(format!("HTTP client: {e}")))?;
        Ok(Downloader { client, allow })
    }

    /// Make `dir/<sha256>` a verified copy of `artifact`, resuming `dir/<sha256>.part`.
    /// `progress(done, total)` is called as bytes arrive.
    pub(crate) async fn fetch(
        &self,
        artifact: &Artifact,
        dir: &Path,
        cancel: &CancellationToken,
        progress: &mut (dyn FnMut(u64, u64) + Send),
    ) -> Result<PathBuf> {
        let done = dir.join(&artifact.sha256);
        let part = dir.join(format!("{}.part", artifact.sha256));
        if done.is_file() {
            if verify_file(&done, artifact).await.is_ok() {
                progress(artifact.size, artifact.size);
                return Ok(done);
            }
            remove(&done).await?;
        }
        let mut last: Option<Error> = None;
        for url in &artifact.urls {
            match self
                .fetch_from(url, artifact, &part, cancel, progress)
                .await
            {
                Ok(()) => {
                    last = None;
                    break;
                }
                // Try the next mirror only for transport problems; integrity, policy and
                // cancel errors stop here (no retry loops).
                Err(e @ (Error::Network { .. } | Error::Download { .. })) => last = Some(e),
                Err(e) => return Err(e),
            }
        }
        if let Some(e) = last {
            return Err(e);
        }
        if let Err(e) = verify_file(&part, artifact).await {
            remove(&part).await?;
            return Err(e);
        }
        tokio::fs::rename(&part, &done)
            .await
            .map_err(|e| Error::io(format!("keep {}", done.display()), e))?;
        Ok(done)
    }

    async fn fetch_from(
        &self,
        url: &str,
        artifact: &Artifact,
        part: &Path,
        cancel: &CancellationToken,
        progress: &mut (dyn FnMut(u64, u64) + Send),
    ) -> Result<()> {
        let parsed = Url::parse(url).map_err(|_| Error::HostNotAllowed { url: url.into() })?;
        self.allow
            .check(&parsed)
            .map_err(|_| Error::HostNotAllowed { url: url.into() })?;
        let size = artifact.size;
        let mut have = match tokio::fs::metadata(part).await {
            Ok(m) => m.len(),
            Err(_) => 0,
        };
        if have > size {
            remove(part).await?;
            have = 0;
        }
        if have == size {
            progress(size, size);
            return Ok(());
        }
        // At most one restart from zero when a server ignores or garbles the range.
        for _ in 0..2 {
            let mut req = self.client.get(parsed.clone());
            if have > 0 {
                req = req.header(header::RANGE, format!("bytes={have}-"));
            }
            let resp = tokio::select! {
                r = req.send() => r.map_err(|e| net_error(url, &artifact.id, e))?,
                _ = cancel.cancelled() => return Err(Error::Cancelled),
            };
            let status = resp.status();
            let append = if have > 0 && status == StatusCode::PARTIAL_CONTENT {
                let ok = resp
                    .headers()
                    .get(header::CONTENT_RANGE)
                    .and_then(|v| v.to_str().ok())
                    .is_some_and(|v| v == format!("bytes {have}-{}/{size}", size - 1));
                if !ok {
                    // A range we did not ask for: start over.
                    remove(part).await?;
                    have = 0;
                    continue;
                }
                true
            } else if status == StatusCode::OK {
                have = 0;
                false
            } else {
                return Err(Error::Download {
                    artifact: artifact.id.clone(),
                    detail: format!("HTTP {status} from {}", host_of(url)),
                });
            };
            let mut file = tokio::fs::OpenOptions::new()
                .create(true)
                .write(true)
                .append(append)
                .truncate(!append)
                .open(part)
                .await
                .map_err(|e| Error::io(format!("write {}", part.display()), e))?;
            let mut got = have;
            progress(got, size);
            let mut stream = resp.bytes_stream();
            loop {
                let chunk = tokio::select! {
                    c = stream.next() => c,
                    _ = cancel.cancelled() => {
                        let _ = file.flush().await;
                        return Err(Error::Cancelled);
                    }
                };
                let Some(chunk) = chunk else { break };
                let chunk = chunk.map_err(|e| net_error(url, &artifact.id, e))?;
                got += chunk.len() as u64;
                if got > size {
                    drop(file);
                    remove(part).await?;
                    return Err(Error::Size {
                        artifact: artifact.id.clone(),
                        expected: size,
                        actual: got,
                    });
                }
                file.write_all(&chunk)
                    .await
                    .map_err(|e| Error::io(format!("write {}", part.display()), e))?;
                progress(got, size);
            }
            file.flush()
                .await
                .map_err(|e| Error::io(format!("write {}", part.display()), e))?;
            if got < size {
                // The connection ended early; the .part file is kept for a resume.
                return Err(Error::Download {
                    artifact: artifact.id.clone(),
                    detail: format!("connection closed after {got} of {size} bytes"),
                });
            }
            return Ok(());
        }
        Err(Error::Download {
            artifact: artifact.id.clone(),
            detail: format!("{} sent an invalid range twice", host_of(url)),
        })
    }
}

/// Size, SHA-256 and (for npm) the SRI integrity of a file.
pub(crate) async fn verify_file(path: &Path, artifact: &Artifact) -> Result<()> {
    let path = path.to_path_buf();
    let artifact = artifact.clone();
    tokio::task::spawn_blocking(move || {
        let mut f = std::fs::File::open(&path)
            .map_err(|e| Error::io(format!("read {}", path.display()), e))?;
        verify_reader(&mut f, &artifact)
    })
    .await
    .map_err(|e| Error::Manifest(format!("verify task: {e}")))?
}

/// Hash everything `r` yields and compare it with the artifact's pins.
pub(crate) fn verify_reader(r: &mut impl std::io::Read, artifact: &Artifact) -> Result<()> {
    let mut sha256 = Sha256::new();
    let mut sha512 = artifact.integrity.as_ref().map(|_| sha2::Sha512::new());
    let mut buf = vec![0u8; 1 << 16];
    let mut total = 0u64;
    loop {
        let n = r
            .read(&mut buf)
            .map_err(|e| Error::io(format!("read {}", artifact.id), e))?;
        if n == 0 {
            break;
        }
        total += n as u64;
        sha256.update(&buf[..n]);
        if let Some(h) = sha512.as_mut() {
            h.update(&buf[..n]);
        }
    }
    if total != artifact.size {
        return Err(Error::Size {
            artifact: artifact.id.clone(),
            expected: artifact.size,
            actual: total,
        });
    }
    let actual = hex(&sha256.finalize());
    if actual != artifact.sha256 {
        return Err(Error::Checksum {
            artifact: artifact.id.clone(),
            expected: artifact.sha256.clone(),
            actual,
        });
    }
    if let (Some(want), Some(h)) = (&artifact.integrity, sha512) {
        use base64::Engine;
        let got = format!(
            "sha512-{}",
            base64::engine::general_purpose::STANDARD.encode(h.finalize())
        );
        if &got != want {
            return Err(Error::Checksum {
                artifact: artifact.id.clone(),
                expected: want.clone(),
                actual: got,
            });
        }
    }
    Ok(())
}

async fn remove(p: &Path) -> Result<()> {
    match tokio::fs::remove_file(p).await {
        Ok(()) => Ok(()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(e) => Err(Error::io(format!("remove {}", p.display()), e)),
    }
}

fn host_of(url: &str) -> String {
    Url::parse(url)
        .ok()
        .and_then(|u| u.host_str().map(str::to_owned))
        .unwrap_or_default()
}

fn net_error(url: &str, artifact: &str, e: reqwest::Error) -> Error {
    let mut src: Option<&dyn std::error::Error> = Some(&e);
    while let Some(s) = src {
        if s.downcast_ref::<Refused>().is_some() || e.is_redirect() {
            return Error::HostNotAllowed {
                url: e
                    .url()
                    .map(|u| u.to_string())
                    .unwrap_or_else(|| url.to_string()),
            };
        }
        src = s.source();
    }
    if e.is_connect() || e.is_timeout() {
        return Error::Network {
            host: host_of(url),
            detail: e.to_string(),
        };
    }
    Error::Download {
        artifact: artifact.to_string(),
        detail: format!("{e}"),
    }
}
