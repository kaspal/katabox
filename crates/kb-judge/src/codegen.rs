//! Code generation from a problem's typed signature: the starter stub shown in the
//! editor and the harness that runs a batch of tests (templates in `languages/<id>/`).

use kb_core::Language;
use minijinja::syntax::SyntaxConfig;
use minijinja::{AutoEscape, Environment, UndefinedBehavior, Value, context};

use crate::JudgeError;
use crate::languages::{LanguageDef, definition};
use crate::signature::Signature;

/// Starter code for the editor: the function with its typed signature and an empty body,
/// after the `ListNode` / `TreeNode` definitions it uses (ADR 0025).
pub fn starter_code(lang: Language, sig: &Signature) -> Result<String, JudgeError> {
    let def = definition(lang)?;
    render(def.stub_template, template_context(&def, sig))
}

/// The function's declaration line in the starter code (`def two_sum(...) -> ...:`), which
/// every reference solution must contain; the node definitions above it do not count.
pub fn signature_line(lang: Language, sig: &Signature) -> Result<String, JudgeError> {
    let stub = starter_code(lang, sig)?;
    let function = sig.function_name(lang);
    stub.lines()
        .find(|l| {
            ["def ", "export function ", "function ", "pub fn "]
                .iter()
                .any(|k| l.starts_with(&format!("{k}{function}(")))
        })
        .map(|l| l.trim().to_string())
        .ok_or_else(|| {
            JudgeError::Internal(format!("no declaration of {function} in the {lang} stub"))
        })
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

/// Files for a debugger replay run (ADR 0022): the debug harness and the solution.
pub(crate) fn debug_files(
    def: &LanguageDef,
    sig: &Signature,
    source: &str,
) -> Result<Vec<(String, String)>, JudgeError> {
    let (_, template) = def
        .debug
        .as_ref()
        .ok_or(JudgeError::Unsupported(def.language))?;
    let mut files = vec![
        (
            def.main_file(),
            render(template, template_context(def, sig))?,
        ),
        (def.solution_file(), source.to_string()),
    ];
    if def.language == Language::TypeScript {
        files.push((
            "package.json".into(),
            "{\"type\": \"module\"}
"
            .into(),
        ));
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
    // Node types (ADR 0025): which ones the signature uses, and each value's shape
    // ("list", "tree" or "plain") with its array depth, for the harness conversions.
    let types = || sig.params.iter().map(|p| &p.ty).chain([&sig.returns]);
    let uses_list = types().any(|t| *t.element() == crate::signature::Type::ListNode);
    let uses_tree = types().any(|t| *t.element() == crate::signature::Type::TreeNode);
    let shape = |t: &crate::signature::Type| (t.shape(), t.array_depth());
    let param_shapes: Vec<(&str, usize)> = sig.params.iter().map(|p| shape(&p.ty)).collect();
    context! {
        function,
        aliases,
        params,
        param_keys,
        returns => sig.returns.spelled_in(lang),
        returns_raw => sig.returns.to_string(),
        return_depth => sig.returns.array_depth(),
        solution_file => def.solution_file(),
        uses_list,
        uses_tree,
        uses_nodes => uses_list || uses_tree,
        param_shapes,
        return_shape => shape(&sig.returns),
        nodes_code => def.nodes_code,
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
    fn typescript_debug_files() {
        let def = crate::languages::definition(Language::TypeScript).unwrap();
        let files = debug_files(&def, &two_sum(), "export function twoSum() {}").unwrap();
        let names: Vec<&str> = files.iter().map(|(n, _)| n.as_str()).collect();
        assert_eq!(names, ["main.ts", "solution.ts", "package.json"]);
        let harness = &files[0].1;
        assert!(harness.contains("const FUNCTION: string = \"twoSum\";"));
        assert!(harness.contains("const SOLUTION: string = \"solution.ts\";"));
        assert!(harness.contains("connectToMainThread"));
        assert!(!harness.contains("{{") && !harness.contains("{%"));
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
            "pub fn two_sum(nums: Vec<i64>, target: i64) -> Vec<i64> {\n    \
             // Do not write `fn main`: KataBox provides it and calls this function.\n    \
             todo!()\n}\n"
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

    fn node_sig() -> Signature {
        Signature::from_json(&serde_json::json!({
            "name": "mergeAll",
            "params": [{"name": "lists", "type": "ListNode[]"}, {"name": "k", "type": "int"}],
            "returns": "TreeNode"
        }))
        .unwrap()
    }

    #[test]
    fn node_stubs_define_their_classes_before_the_function() {
        let sig = node_sig();
        let py = starter_code(Language::Python, &sig).unwrap();
        assert!(
            py.contains("class ListNode:") && py.contains("class TreeNode:"),
            "{py}"
        );
        assert!(
            py.contains("def merge_all(lists: list[ListNode | None], k: int) -> TreeNode | None:")
        );
        let ts = starter_code(Language::TypeScript, &sig).unwrap();
        assert!(ts.contains("export class ListNode {") && ts.contains("export class TreeNode {"));
        let php = starter_code(Language::Php, &sig).unwrap();
        assert!(
            php.starts_with(
                "<?php
"
            ) && php.contains("public ?ListNode $next = null")
        );
        let rs = starter_code(Language::Rust, &sig).unwrap();
        assert!(rs.contains("use std::rc::Rc;") && rs.contains("pub struct TreeNode {"));
        assert!(rs.contains("pub fn new(val: i64) -> Self"));
        // The declaration line every reference must contain.
        assert_eq!(
            signature_line(Language::Rust, &sig).unwrap(),
            "pub fn merge_all(lists: Vec<Option<Box<ListNode>>>, k: i64) -> Option<Rc<RefCell<TreeNode>>> {"
        );
        assert_eq!(
            signature_line(Language::TypeScript, &sig).unwrap(),
            "export function mergeAll(lists: (ListNode | null)[], k: number): TreeNode | null {"
        );
        // A signature without nodes keeps its plain stub.
        assert!(
            !starter_code(Language::Python, &two_sum())
                .unwrap()
                .contains("class")
        );
        assert_eq!(
            signature_line(Language::Php, &two_sum()).unwrap(),
            "function twoSum(array $nums, int $target): array {"
        );
    }

    #[test]
    fn node_harnesses_carry_shapes_and_conversions_only_when_used() {
        let sig = node_sig();
        let py = harness(Language::Python, &sig).unwrap();
        assert!(
            py.contains(r#"PARAM_SHAPES = [["list", 1], ["plain", 0]]"#),
            "{py}"
        );
        assert!(py.contains(r#"RETURN_SHAPE = ["tree", 0]"#));
        assert!(py.contains("def tree_from_json"));
        let rs = harness(Language::Rust, &sig).unwrap();
        assert!(rs.contains("impl FromJson for Option<Box<ListNode>>"));
        assert!(rs.contains("impl ToJson for Option<Rc<RefCell<TreeNode>>>"));
        // Plain signatures get none of it.
        for lang in [
            Language::Python,
            Language::TypeScript,
            Language::Php,
            Language::Rust,
        ] {
            let h = harness(lang, &two_sum()).unwrap();
            assert!(
                !h.contains("ListNode") && !h.contains("node_cycle"),
                "{lang}"
            );
        }
    }
}
