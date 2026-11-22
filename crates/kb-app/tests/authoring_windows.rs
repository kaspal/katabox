//! The in-app editor's "Check in sandbox" (expansion M13, ADR 0040): the draft's
//! reference runs on its tests through the ordinary judge path.
//!
//! ```text
//! pwsh scripts/fetch-toolchains.ps1
//! cargo test -p kb-app --test authoring_windows -- --nocapture
//! ```
#![cfg(windows)]
use std::path::{Path, PathBuf};

use kb_app::{App, AppPaths};
use kb_core::{Language, Verdict};
use kb_pack::authoring::{DraftTest, ProblemDraft};

fn repo() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn temp(name: &str) -> PathBuf {
    let d = std::env::temp_dir().join(format!("kb-app-authoring-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&d);
    std::fs::create_dir_all(&d).unwrap();
    d
}

fn draft(language: Language, reference: &str) -> ProblemDraft {
    let t = |nums: &[i64], out: i64| DraftTest {
        input: serde_json::json!({ "nums": nums })
            .as_object()
            .unwrap()
            .clone(),
        output: out.into(),
    };
    ProblemDraft {
        title: "Tallest Tower".into(),
        tags: vec!["array".into()],
        function: "tallest".into(),
        statement: "Return the tallest of `nums`.".into(),
        statement_es: "Devuelve la mayor de `nums`.".into(),
        hints: vec!["One pass.".into()],
        editorial: "Track the best value.".into(),
        public_tests: vec![t(&[1, 5, 3], 5)],
        hidden_tests: vec![t(&[-4, -2, -9], -2), t(&[2, 9, 2], 9)],
        reference_language: language,
        reference: reference.into(),
        ..ProblemDraft::new("tallest-tower")
    }
}

#[tokio::test]
async fn check_in_sandbox_judges_the_reference() {
    let root = std::path::absolute(repo().join("target/katabox")).unwrap();
    assert!(
        root.join(kb_runner::ROOT_MARKER).is_file(),
        "run scripts/fetch-toolchains.ps1 first"
    );
    let work = temp("work");
    let content = work.join("content.db");
    let c = content.clone();
    std::thread::spawn(move || {
        kb_pack::build_pack(&repo().join("content/problems"), &c, "test", "1").unwrap()
    })
    .join()
    .unwrap();
    let app = App::open(&AppPaths {
        content_db: content,
        user_db: work.join("user.db"),
        local_data_dir: work.join("local"),
        dev_root: Some(root),
        dev_data: true,
    })
    .await
    .unwrap();

    let good = draft(
        Language::TypeScript,
        "export function tallest(nums: number[]): number {\n  return Math.max(...nums);\n}\n",
    );
    assert_eq!(app.authoring_save(&good).await.unwrap(), vec![]);
    let report = app.authoring_check("tallest-tower").await.unwrap();
    println!("good: {:?} ({} tests)", report.verdict, report.tests.len());
    assert_eq!(report.verdict, Verdict::Accepted);
    assert_eq!(report.tests.len(), 3);

    // A reference that is wrong on a hidden test: the author sees it before exporting.
    let wrong = draft(
        Language::Python,
        "def tallest(nums: list[int]) -> int:\n    return nums[0] if nums[0] > 0 else 0\n",
    );
    assert_eq!(app.authoring_save(&wrong).await.unwrap(), vec![]);
    let report = app.authoring_check("tallest-tower").await.unwrap();
    println!("wrong: {:?}", report.verdict);
    assert_eq!(report.verdict, Verdict::WrongAnswer);
}
