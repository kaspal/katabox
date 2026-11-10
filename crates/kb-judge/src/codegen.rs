//! Code generation from a problem's typed signature: the starter stub shown in the
//! editor and the harness that runs a batch of tests (templates in `languages/<id>/`).

use kb_core::Language;
use minijinja::syntax::SyntaxConfig;
use minijinja::{AutoEscape, Environment, UndefinedBehavior, Value, context};

use crate::JudgeError;
use crate::languages::{LanguageDef, definition};
use crate::signature::Signature;

/// Starter code for the editor: the function with its typed signature and an empty body.
pub fn starter_code(lang: Language, sig: &Signature) -> Result<String, JudgeError> {
    let def = definition(lang)?;
    render(def.stub_template, template_context(&def, sig))
}

/// The harness program for `lang`: reads the tests from stdin, calls the user's
/// function once per test, and prints one JSON record per test (ADR 0008).
pub fn harness(lang: Language, sig: &Signature) -> Result<String, JudgeError> {
    let def = definition(lang)?;
    render(def.harness_template, template_context(&def, sig))
}

/// Files to place in the work folder for one run: harness, solution and extras.
pub(crate) fn workspace_files(
    def: &LanguageDef,
    sig: &Signature,
    source: &str,
) -> Result<Vec<(String, String)>, JudgeError> {
    let mut files = vec![
        (
            def.main_file(),
            render(def.harness_template, template_context(def, sig))?,
        ),
        (def.solution_file(), source.to_string()),
    ];
    if def.language == Language::TypeScript {
        // Makes `.ts` files ES modules without Node guessing from the syntax.
        files.push(("package.json".into(), "{\"type\": \"module\"}\n".into()));
    }
    Ok(files)
}

fn template_context(def: &LanguageDef, sig: &Signature) -> Value {
    let lang = def.language;
    let function = sig.function_name(lang);
    // Python also accepts the camelCase name from problem.toml.
    let mut aliases = vec![function.clone()];
    if sig.name != function {
        aliases.push(sig.name.clone());
    }
    let params: Vec<Value> = sig
        .params
        .iter()
        .map(|p| context! { name => sig.param_name(p, lang), type => p.ty.spelled_in(lang) })
        .collect();
    let param_keys: Vec<String> = sig.params.iter().map(|p| p.name.clone()).collect();
    context! {
        function,
        aliases,
        params,
        param_keys,
        returns => sig.returns.spelled_in(lang),
        return_depth => sig.returns.array_depth(),
        solution_file => def.solution_file(),
    }
}

fn render(template: &str, ctx: Value) -> Result<String, JudgeError> {
    let mut env = Environment::new();
    env.set_auto_escape_callback(|_| AutoEscape::None);
    env.set_undefined_behavior(UndefinedBehavior::Strict);
    env.set_syntax(
        SyntaxConfig::builder()
            .trim_blocks(true)
            .lstrip_blocks(true)
            .keep_trailing_newline(true)
            .build()
            .map_err(|e| JudgeError::Internal(e.to_string()))?,
    );
    env.template_from_str(template)
        .and_then(|t| t.render(ctx))
        .map_err(|e| JudgeError::Internal(format!("template: {e:#}")))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::problem::Problem;
    use std::path::Path;

    fn two_sum() -> Signature {
        Problem::load(&Path::new(env!("CARGO_MANIFEST_DIR")).join("../../content/problems/two-sum"))
            .unwrap()
            .signature
    }

    #[test]
    fn python_stub() {
        assert_eq!(
            starter_code(Language::Python, &two_sum()).unwrap(),
            "def two_sum(nums: list[int], target: int) -> list[int]:\n    pass\n"
        );
    }

    #[test]
    fn typescript_stub() {
        assert_eq!(
            starter_code(Language::TypeScript, &two_sum()).unwrap(),
            "export function twoSum(nums: number[], target: number): number[] {\n\n}\n"
        );
    }

    #[test]
    fn php_stub() {
        assert_eq!(
            starter_code(Language::Php, &two_sum()).unwrap(),
            "<?php
function twoSum(array $nums, int $target): array {

}
"
        );
    }

    #[test]
    fn rust_stub() {
        assert_eq!(
            starter_code(Language::Rust, &two_sum()).unwrap(),
            "pub fn two_sum(nums: Vec<i64>, target: i64) -> Vec<i64> {
    todo!()
}
"
        );
    }

    #[test]
    fn php_and_rust_harnesses_embed_the_signature() {
        let php = harness(Language::Php, &two_sum()).unwrap();
        assert!(php.contains(r#"const KB_FUNCTION = "twoSum";"#), "{php}");
        assert!(php.contains("const KB_RETURN_DEPTH = 1;"));
        let rs = harness(Language::Rust, &two_sum()).unwrap();
        assert!(rs.contains(r#"include!("solution.rs");"#), "{rs}");
        assert!(rs.contains("type Args = (Vec<i64>,i64,);"), "{rs}");
        assert!(rs.contains("fn call(args: Args) -> Vec<i64>"));
        assert!(rs.contains("crate::two_sum(a0, a1)"));
        for src in [&php, &rs] {
            assert!(!src.contains("{{") && !src.contains("{%"));
        }
    }

    #[test]
    fn harnesses_embed_the_signature() {
        let py = harness(Language::Python, &two_sum()).unwrap();
        assert!(py.contains(r#"ALIASES = ["two_sum", "twoSum"]"#), "{py}");
        assert!(py.contains(r#"PARAMS = ["nums", "target"]"#));
        assert!(py.contains(r#"SOLUTION = "solution.py""#));
        let ts = harness(Language::TypeScript, &two_sum()).unwrap();
        assert!(ts.contains(r#"const FUNCTION: string = "twoSum";"#), "{ts}");
        assert!(ts.contains(r#"const PARAMS: string[] = ["nums", "target"];"#));
        for src in [&py, &ts] {
            assert!(!src.contains("{{") && !src.contains("{%"));
        }
    }

    #[test]
    fn unsupported_language_is_an_error() {
        assert!(matches!(
            starter_code(Language::JavaScript, &two_sum()),
            Err(JudgeError::Unsupported(Language::JavaScript))
        ));
    }
}
