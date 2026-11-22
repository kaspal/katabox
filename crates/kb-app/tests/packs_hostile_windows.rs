//! T8 of ADR 0038 in the Windows sandbox: code that comes with a pack runs only through the
//! ordinary judge path. A pack whose reference solution tries to list the user's profile,
//! reach the network, write to TEMP and to the registry is installed like any pack; a Run
//! with a custom test makes the app execute that reference in the sandbox. The reference
//! returns the right answer only when every attempt failed, so Accepted means contained.
//!
//! ```text
//! pwsh scripts/fetch-toolchains.ps1
//! cargo test -p kb-app --test packs_hostile_windows -- --nocapture
//! ```
#![cfg(windows)]
use std::path::{Path, PathBuf};

use kb_app::{App, AppPaths};
use kb_core::{Language, Verdict};

fn repo() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn temp(name: &str) -> PathBuf {
    let d = std::env::temp_dir().join(format!(
        "kb-app-packs-hostile-{name}-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&d);
    std::fs::create_dir_all(&d).unwrap();
    d
}

fn copy(src: &Path, dst: &Path) {
    std::fs::create_dir_all(dst).unwrap();
    for e in std::fs::read_dir(src).unwrap().flatten() {
        let p = e.path();
        if p.is_dir() {
            copy(&p, &dst.join(p.file_name().unwrap()));
        } else {
            std::fs::copy(&p, dst.join(p.file_name().unwrap())).unwrap();
        }
    }
}

#[tokio::test]
async fn pack_code_runs_contained() {
    let root = std::path::absolute(repo().join("target/katabox")).unwrap();
    assert!(
        root.join(kb_runner::ROOT_MARKER).is_file(),
        "run scripts/fetch-toolchains.ps1 first"
    );
    let work = temp("work");
    let problem = work.join("budget-pair");
    copy(&repo().join("content/problems/budget-pair"), &problem);
    let _ = std::fs::remove_dir_all(problem.join("profile/inputs"));
    let profile = std::env::var("USERPROFILE").unwrap();
    let temp_dir = std::env::var("TEMP").unwrap();
    let probe = Path::new(&temp_dir).join(format!("kb-pack-escape-{}.txt", std::process::id()));
    let hostile = format!(
        r#"def best_pair(prices: list[int], budget: int) -> int:
    escaped = []
    def attempt(name, f):
        try:
            f()
            escaped.append(name)
        except Exception:
            pass
    import os, socket
    attempt("list-profile", lambda: os.listdir({profile:?}))
    attempt("net", lambda: socket.create_connection(("1.1.1.1", 443), timeout=2).close())
    attempt("write-temp", lambda: open({probe:?}, "w").write("x"))
    import winreg
    attempt("registry", lambda: winreg.CreateKey(winreg.HKEY_CURRENT_USER, r"Software\KataBoxPackEscape"))
    if escaped:
        return -999
    lo, hi = 0, len(prices) - 1
    best = -1
    while lo < hi:
        total = prices[lo] + prices[hi]
        if total > budget:
            hi -= 1
        else:
            best = max(best, total)
            lo += 1
    return best
"#,
        probe = probe.display().to_string(),
    );
    std::fs::write(problem.join("solutions/ref.py"), hostile).unwrap();
    let manifest = work.join("pack.toml");
    std::fs::write(
        &manifest,
        "id = \"hostile\"\nname = \"Hostile\"\nauthor = \"Test\"\nversion = \"1.0.0\"\nlicence = \"MIT\"\nmin_app = \"1.0.0-beta.1\"\n",
    )
    .unwrap();
    let file = work.join("hostile.kbpack");
    kb_pack::kbpack::write_pack(&[problem], &manifest, &file).unwrap();

    let content = work.join("content.db");
    let c = content.clone();
    std::thread::spawn(move || {
        kb_pack::build_pack(&repo().join("content/problems"), &c, "test", "1").unwrap()
    })
    .join()
    .unwrap();
    let data = temp("data");
    let paths = AppPaths {
        content_db: content,
        user_db: data.join("user.db"),
        local_data_dir: data.join("local"),
        dev_root: Some(root),
        dev_data: true,
    };
    let app = App::open(&paths).await.unwrap();
    let preview = app.import_pack_preview(&file).await.unwrap();
    app.import_pack_apply(preview.token).await.unwrap();
    app.close().await;
    let app = App::open(&paths).await.unwrap();

    // A correct solution, judged on a custom test whose expected output the pack's
    // (hostile) reference computes in the sandbox.
    let solution =
        std::fs::read_to_string(repo().join("content/problems/budget-pair/solutions/ref.py"))
            .unwrap();
    let input = serde_json::json!({"prices": [1, 2, 3, 4], "budget": 6});
    let report = app
        .run(
            "hostile--budget-pair",
            Language::Python,
            &solution,
            vec![input.as_object().unwrap().clone()],
            |_| {},
        )
        .await
        .unwrap();
    let custom = report.tests.last().unwrap();
    println!(
        "custom test: {:?} actual {:?}",
        custom.result.verdict, custom.actual
    );
    assert_eq!(
        custom.result.verdict,
        Verdict::Accepted,
        "an escape attempt succeeded: {report:?}"
    );
    assert_eq!(custom.actual, Some(serde_json::json!(6)));
    assert!(!probe.exists(), "the pack's code wrote to the host's TEMP");
}
