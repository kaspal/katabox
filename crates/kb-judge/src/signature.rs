//! The typed function signature from `[signature]` in problem.toml, and how each
//! language spells its names and types.
//!
//! Type grammar (deliberately small; grows with the problem set):
//! `int | float | string | bool | ListNode | TreeNode | <type>[]`, e.g. `int[]`,
//! `string[][]`, `ListNode[]`.
//!
//! Node types (ADR 0025) travel as JSON arrays: a `ListNode` is its values in order, a
//! `TreeNode` its level order with `null` for a missing child (children of a missing node
//! are not listed, trailing nulls are trimmed); `[]` is the empty list or tree. Values
//! are `int`.

use std::fmt;
use std::str::FromStr;

use kb_core::Language;
use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Int,
    Float,
    String,
    Bool,
    /// A singly linked list of `int` (ADR 0025).
    ListNode,
    /// A binary tree of `int` (ADR 0025).
    TreeNode,
    Array(Box<Type>),
}

impl FromStr for Type {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if let Some(inner) = s.strip_suffix("[]") {
            return Ok(Type::Array(Box::new(inner.parse()?)));
        }
        match s {
            "int" => Ok(Type::Int),
            "float" => Ok(Type::Float),
            "string" => Ok(Type::String),
            "bool" => Ok(Type::Bool),
            "ListNode" => Ok(Type::ListNode),
            "TreeNode" => Ok(Type::TreeNode),
            _ => Err(format!(
                "unknown type {s:?} (expected int | float | string | bool | ListNode | TreeNode, optionally followed by [])"
            )),
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Int => f.write_str("int"),
            Type::Float => f.write_str("float"),
            Type::String => f.write_str("string"),
            Type::Bool => f.write_str("bool"),
            Type::ListNode => f.write_str("ListNode"),
            Type::TreeNode => f.write_str("TreeNode"),
            Type::Array(t) => write!(f, "{t}[]"),
        }
    }
}

impl Type {
    /// The type as written in `lang`'s source code.
    pub fn spelled_in(&self, lang: Language) -> String {
        match lang {
            Language::Python => match self {
                Type::Int => "int".into(),
                Type::Float => "float".into(),
                Type::String => "str".into(),
                Type::Bool => "bool".into(),
                Type::ListNode => "ListNode | None".into(),
                Type::TreeNode => "TreeNode | None".into(),
                Type::Array(t) => format!("list[{}]", t.spelled_in(lang)),
            },
            // JavaScript has no annotations; the TypeScript spelling is used in JSDoc later.
            Language::TypeScript | Language::JavaScript => match self {
                Type::Int | Type::Float => "number".into(),
                Type::String => "string".into(),
                Type::Bool => "boolean".into(),
                Type::ListNode => "ListNode | null".into(),
                Type::TreeNode => "TreeNode | null".into(),
                // A union element needs parentheses: `(ListNode | null)[]`.
                Type::Array(t) if t.is_node() => format!("({})[]", t.spelled_in(lang)),
                Type::Array(t) => format!("{}[]", t.spelled_in(lang)),
            },
            // PHP has one array type; nesting is not expressible in a type declaration.
            Language::Php => match self {
                Type::Int => "int".into(),
                Type::Float => "float".into(),
                Type::String => "string".into(),
                Type::Bool => "bool".into(),
                Type::ListNode => "?ListNode".into(),
                Type::TreeNode => "?TreeNode".into(),
                Type::Array(_) => "array".into(),
            },
            Language::Rust => match self {
                Type::Int => "i64".into(),
                Type::Float => "f64".into(),
                Type::String => "String".into(),
                Type::Bool => "bool".into(),
                Type::ListNode => "Option<Box<ListNode>>".into(),
                Type::TreeNode => "Option<Rc<RefCell<TreeNode>>>".into(),
                Type::Array(t) => format!("Vec<{}>", t.spelled_in(lang)),
            },
        }
    }

    /// How many array levels wrap the element type (`int[][]` -> 2).
    pub fn array_depth(&self) -> usize {
        match self {
            Type::Array(t) => 1 + t.array_depth(),
            _ => 0,
        }
    }

    /// The element type under every array level (`ListNode[][]` -> `ListNode`).
    pub fn element(&self) -> &Type {
        match self {
            Type::Array(t) => t.element(),
            t => t,
        }
    }

    pub fn is_node(&self) -> bool {
        matches!(self, Type::ListNode | Type::TreeNode)
    }

    /// The harness shape of a value (ADR 0025): `"list"`, `"tree"` or `"plain"` for the
    /// element type; harnesses convert node elements under [`Type::array_depth`] arrays.
    pub fn shape(&self) -> &'static str {
        match self.element() {
            Type::ListNode => "list",
            Type::TreeNode => "tree",
            _ => "plain",
        }
    }

    /// Whether `value` is a well-formed value of this type (the JSON encoding; ADR 0025).
    /// The error says what was expected, for authors and custom inputs.
    pub fn check_value(&self, value: &serde_json::Value) -> Result<(), String> {
        use serde_json::Value;
        let int = |v: &Value| {
            v.as_i64().is_some() || v.as_u64().is_some_and(|u| i64::try_from(u).is_ok())
        };
        match self {
            Type::Int if int(value) => Ok(()),
            Type::Int => Err(format!("expected an int, got {value}")),
            Type::Float if value.is_number() => Ok(()),
            Type::Float => Err(format!("expected a float, got {value}")),
            Type::String if value.is_string() => Ok(()),
            Type::String => Err(format!("expected a string, got {value}")),
            Type::Bool if value.is_boolean() => Ok(()),
            Type::Bool => Err(format!("expected a bool, got {value}")),
            Type::ListNode => match value.as_array() {
                Some(items) if items.iter().all(int) => Ok(()),
                _ => Err(format!(
                    "expected a ListNode: a JSON array of ints, got {}",
                    short(value)
                )),
            },
            Type::TreeNode => match value.as_array() {
                Some(items) => check_level_order(items),
                None => Err(format!(
                    "expected a TreeNode: a level-order JSON array, got {}",
                    short(value)
                )),
            },
            Type::Array(t) => match value.as_array() {
                Some(items) => items
                    .iter()
                    .enumerate()
                    .try_for_each(|(i, v)| t.check_value(v).map_err(|e| format!("[{i}]: {e}"))),
                None => Err(format!("expected an array ({self}), got {}", short(value))),
            },
        }
    }
}

/// A value for an error message, cut to a readable length.
fn short(value: &serde_json::Value) -> String {
    let s = value.to_string();
    if s.chars().count() > 60 {
        format!("{}…", s.chars().take(60).collect::<String>())
    } else {
        s
    }
}

/// A canonical level-order tree: ints and nulls, the root not null, every slot owned by
/// a node listed before it (each node owns two), and no trailing null.
fn check_level_order(items: &[serde_json::Value]) -> Result<(), String> {
    let int = |v: &serde_json::Value| v.as_i64().is_some();
    if let Some((i, v)) = items
        .iter()
        .enumerate()
        .find(|(_, v)| !v.is_null() && !int(v))
    {
        return Err(format!("TreeNode: [{i}] is {v}, expected an int or null"));
    }
    if items.first().is_some_and(|v| v.is_null()) {
        return Err("TreeNode: the root is null (write [] for an empty tree)".into());
    }
    if items.last().is_some_and(|v| v.is_null()) {
        return Err("TreeNode: trailing nulls are trimmed in level order".into());
    }
    // Slots available after reading index i: 1 (the root) + 2 per node so far.
    let mut slots = 1usize;
    for (i, v) in items.iter().enumerate() {
        if i >= slots {
            return Err(format!(
                "TreeNode: [{i}] has no parent (children of a missing node are not listed)"
            ));
        }
        if !v.is_null() {
            slots += 2;
        }
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Param {
    /// Name as written in problem.toml; also the key in each test's `input` object.
    pub name: String,
    pub ty: Type,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Signature {
    /// Function name in camelCase, as written in problem.toml.
    pub name: String,
    pub params: Vec<Param>,
    pub returns: Type,
}

/// `[signature]` exactly as it appears in problem.toml, before validation.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct RawSignature {
    name: String,
    params: Vec<RawParam>,
    returns: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawParam {
    name: String,
    #[serde(rename = "type")]
    ty: String,
}

impl TryFrom<RawSignature> for Signature {
    type Error = String;
    fn try_from(raw: RawSignature) -> Result<Self, String> {
        check_identifier(&raw.name, "function name")?;
        let mut params: Vec<Param> = Vec::with_capacity(raw.params.len());
        for p in raw.params {
            check_identifier(&p.name, "param name")?;
            if params.iter().any(|q| q.name == p.name) {
                return Err(format!("duplicate param {:?}", p.name));
            }
            let ty =
                p.ty.parse()
                    .map_err(|e| format!("param {:?}: {e}", p.name))?;
            params.push(Param { name: p.name, ty });
        }
        let returns = raw.returns.parse().map_err(|e| format!("returns: {e}"))?;
        Ok(Signature {
            name: raw.name,
            params,
            returns,
        })
    }
}

impl Signature {
    /// Parse and validate a signature stored as JSON (content.db): the same shape as
    /// `[signature]` in problem.toml, `{"name", "params": [{"name", "type"}], "returns"}`.
    pub fn from_json(value: &serde_json::Value) -> Result<Signature, String> {
        let raw: RawSignature =
            serde_json::from_value(value.clone()).map_err(|e| format!("signature: {e}"))?;
        raw.try_into()
    }

    /// Function name in `lang`'s convention (`twoSum` -> `two_sum` in Python).
    pub fn function_name(&self, lang: Language) -> String {
        identifier(&self.name, lang)
    }

    /// Param name in `lang`'s convention.
    pub fn param_name(&self, param: &Param, lang: Language) -> String {
        identifier(&param.name, lang)
    }
}

/// A problem.toml name (camelCase) in `lang`'s naming convention.
fn identifier(name: &str, lang: Language) -> String {
    let name = match lang {
        Language::Python | Language::Rust => snake_case(name),
        // PHP functions are conventionally camelCase, like the problem.toml name.
        Language::Php | Language::TypeScript | Language::JavaScript => name.to_string(),
    };
    avoid_keyword(name, lang)
}

/// ASCII identifiers only: names are pasted into generated source code.
fn check_identifier(s: &str, what: &str) -> Result<(), String> {
    let mut chars = s.chars();
    let ok = chars
        .next()
        .is_some_and(|c| c.is_ascii_alphabetic() || c == '_')
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '_');
    if ok {
        Ok(())
    } else {
        Err(format!("{what} {s:?} must be an ASCII identifier"))
    }
}

/// `twoSum` -> `two_sum`, `isValidBST` -> `is_valid_bst`, `max2D` -> `max2_d`.
pub fn snake_case(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let mut out = String::with_capacity(s.len() + 4);
    for (i, &c) in chars.iter().enumerate() {
        if c.is_ascii_uppercase() && i > 0 {
            let prev = chars[i - 1];
            let next_lower = chars.get(i + 1).is_some_and(|n| n.is_ascii_lowercase());
            if prev.is_ascii_lowercase()
                || prev.is_ascii_digit()
                || (prev.is_ascii_uppercase() && next_lower)
            {
                out.push('_');
            }
        }
        out.push(c.to_ascii_lowercase());
    }
    out
}

const PYTHON_KEYWORDS: &[&str] = &[
    "False", "None", "True", "and", "as", "assert", "async", "await", "break", "class", "continue",
    "def", "del", "elif", "else", "except", "finally", "for", "from", "global", "if", "import",
    "in", "is", "lambda", "nonlocal", "not", "or", "pass", "raise", "return", "try", "while",
    "with", "yield",
];

/// Names a PHP function may not take (keywords and language constructs).
const PHP_RESERVED: &[&str] = &[
    "abstract",
    "and",
    "array",
    "as",
    "break",
    "callable",
    "case",
    "catch",
    "class",
    "clone",
    "const",
    "continue",
    "declare",
    "default",
    "die",
    "do",
    "echo",
    "else",
    "elseif",
    "empty",
    "enddeclare",
    "endfor",
    "endforeach",
    "endif",
    "endswitch",
    "endwhile",
    "eval",
    "exit",
    "extends",
    "final",
    "finally",
    "fn",
    "for",
    "foreach",
    "function",
    "global",
    "goto",
    "if",
    "implements",
    "include",
    "include_once",
    "instanceof",
    "insteadof",
    "interface",
    "isset",
    "list",
    "match",
    "namespace",
    "new",
    "or",
    "print",
    "private",
    "protected",
    "public",
    "readonly",
    "require",
    "require_once",
    "return",
    "static",
    "switch",
    "throw",
    "trait",
    "try",
    "unset",
    "use",
    "var",
    "while",
    "xor",
    "yield",
];

const RUST_KEYWORDS: &[&str] = &[
    "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern",
    "false", "fn", "for", "gen", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut",
    "pub", "ref", "return", "self", "static", "struct", "super", "trait", "true", "type", "unsafe",
    "use", "where", "while", "abstract", "become", "box", "do", "final", "macro", "override",
    "priv", "try", "typeof", "unsized", "virtual", "yield",
];

const TS_RESERVED: &[&str] = &[
    "break",
    "case",
    "catch",
    "class",
    "const",
    "continue",
    "debugger",
    "default",
    "delete",
    "do",
    "else",
    "enum",
    "export",
    "extends",
    "false",
    "finally",
    "for",
    "function",
    "if",
    "import",
    "in",
    "instanceof",
    "new",
    "null",
    "return",
    "super",
    "switch",
    "this",
    "throw",
    "true",
    "try",
    "typeof",
    "var",
    "void",
    "while",
    "with",
    "let",
    "static",
    "yield",
    "await",
    "implements",
    "interface",
    "package",
    "private",
    "protected",
    "public",
];

fn avoid_keyword(name: String, lang: Language) -> String {
    let reserved = match lang {
        Language::Python => PYTHON_KEYWORDS,
        Language::TypeScript | Language::JavaScript => TS_RESERVED,
        Language::Php => PHP_RESERVED,
        Language::Rust => RUST_KEYWORDS,
    };
    // PHP keywords are case-insensitive.
    let lowered = name.to_ascii_lowercase();
    let probe = if lang == Language::Php {
        lowered.as_str()
    } else {
        name.as_str()
    };
    if reserved.contains(&probe) {
        name + "_"
    } else {
        name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sig(toml_src: &str) -> Result<Signature, String> {
        let raw: RawSignature = toml::from_str(toml_src).map_err(|e| e.to_string())?;
        raw.try_into()
    }

    #[test]
    fn parses_types() {
        assert_eq!("int".parse(), Ok(Type::Int));
        assert_eq!(
            "string[][]".parse(),
            Ok(Type::Array(Box::new(Type::Array(Box::new(Type::String)))))
        );
        assert!("map<int,int>".parse::<Type>().is_err());
        assert!("[]".parse::<Type>().is_err());
        assert_eq!("bool[]".parse::<Type>().unwrap().to_string(), "bool[]");
    }

    #[test]
    fn spells_types_per_language() {
        let t: Type = "int[][]".parse().unwrap();
        assert_eq!(t.spelled_in(Language::Python), "list[list[int]]");
        assert_eq!(t.spelled_in(Language::TypeScript), "number[][]");
        assert_eq!(Type::Bool.spelled_in(Language::TypeScript), "boolean");
        assert_eq!(Type::String.spelled_in(Language::Python), "str");
    }

    #[test]
    fn node_types_parse_and_spell() {
        let list: Type = "ListNode".parse().unwrap();
        let trees: Type = "TreeNode[]".parse().unwrap();
        assert_eq!(list, Type::ListNode);
        assert_eq!(trees.to_string(), "TreeNode[]");
        assert!("listnode".parse::<Type>().is_err());
        let spell = |t: &Type| ALL_LANGS.map(|l| t.spelled_in(l));
        assert_eq!(
            spell(&list),
            [
                "ListNode | None",
                "ListNode | null",
                "?ListNode",
                "Option<Box<ListNode>>"
            ]
        );
        assert_eq!(
            spell(&trees),
            [
                "list[TreeNode | None]",
                "(TreeNode | null)[]",
                "array",
                "Vec<Option<Rc<RefCell<TreeNode>>>>"
            ]
        );
        assert_eq!((trees.shape(), trees.array_depth()), ("tree", 1));
        assert_eq!(Type::Int.shape(), "plain");
    }

    const ALL_LANGS: [Language; 4] = [
        Language::Python,
        Language::TypeScript,
        Language::Php,
        Language::Rust,
    ];

    #[test]
    fn values_are_checked_against_their_types() {
        use serde_json::json;
        let ok = |t: &str, v: serde_json::Value| t.parse::<Type>().unwrap().check_value(&v);
        assert!(ok("int", json!(3)).is_ok());
        assert!(ok("int", json!(3.5)).is_err());
        assert!(ok("float", json!(3)).is_ok());
        assert!(ok("string[]", json!(["a", 1])).unwrap_err().contains("[1]"));
        assert!(ok("int[][]", json!([[1], [2, 3]])).is_ok());
        // Lists: arrays of ints, [] is the empty list.
        assert!(ok("ListNode", json!([])).is_ok());
        assert!(ok("ListNode", json!([1, 2])).is_ok());
        assert!(ok("ListNode", json!([1, null])).is_err());
        assert!(ok("ListNode[]", json!([[], [4]])).is_ok());
        // Trees: canonical level order.
        for good in [
            json!([]),
            json!([1]),
            json!([1, null, 2, 3]),
            json!([5, 3, 8, 1, null, 7, 9]),
        ] {
            assert!(ok("TreeNode", good.clone()).is_ok(), "{good}");
        }
        let bad = [
            (json!([null]), "root is null"),
            (json!([1, 2, null]), "trailing"),
            (json!([1, null, null, 3]), "no parent"),
            (json!([1, "x"]), "int or null"),
            (json!({"val": 1}), "level-order"),
        ];
        for (v, why) in bad {
            let e = ok("TreeNode", v.clone()).unwrap_err();
            assert!(e.contains(why), "{v}: {e}");
        }
    }

    #[test]
    fn spells_php_and_rust_types() {
        let t: Type = "int[][]".parse().unwrap();
        assert_eq!(t.spelled_in(Language::Rust), "Vec<Vec<i64>>");
        assert_eq!(t.spelled_in(Language::Php), "array");
        assert_eq!(t.array_depth(), 2);
        assert_eq!(Type::Float.spelled_in(Language::Rust), "f64");
        assert_eq!(Type::String.spelled_in(Language::Rust), "String");
        assert_eq!(Type::Bool.spelled_in(Language::Php), "bool");
        assert_eq!(Type::Int.array_depth(), 0);
    }

    #[test]
    fn php_keeps_camel_case_and_rust_snake_cases() {
        let s = sig(r#"name = "list"
params = [{ name = "type", type = "int" }, { name = "maxValue", type = "int" }]
returns = "int""#)
        .unwrap();
        assert_eq!(s.function_name(Language::Php), "list_");
        assert_eq!(s.function_name(Language::Rust), "list");
        assert_eq!(s.param_name(&s.params[0], Language::Rust), "type_");
        assert_eq!(s.param_name(&s.params[1], Language::Rust), "max_value");
        assert_eq!(s.param_name(&s.params[1], Language::Php), "maxValue");
    }

    #[test]
    fn snake_cases_names() {
        assert_eq!(snake_case("twoSum"), "two_sum");
        assert_eq!(snake_case("isValidBST"), "is_valid_bst");
        assert_eq!(snake_case("parseHTMLTag"), "parse_html_tag");
        assert_eq!(snake_case("nums"), "nums");
        assert_eq!(snake_case("max2D"), "max2_d");
    }

    #[test]
    fn keywords_get_a_suffix() {
        let s = sig(r#"name = "f"
params = [{ name = "from", type = "int" }, { name = "delete", type = "int" }]
returns = "int""#)
        .unwrap();
        assert_eq!(s.param_name(&s.params[0], Language::Python), "from_");
        assert_eq!(s.param_name(&s.params[1], Language::Python), "delete");
        assert_eq!(s.param_name(&s.params[1], Language::TypeScript), "delete_");
    }

    #[test]
    fn rejects_bad_signatures() {
        let bad_name = sig(r#"name = "two sum"
params = []
returns = "int""#);
        assert!(bad_name.unwrap_err().contains("ASCII identifier"));
        let dup = sig(r#"name = "f"
params = [{ name = "a", type = "int" }, { name = "a", type = "int" }]
returns = "int""#);
        assert!(dup.unwrap_err().contains("duplicate"));
        let bad_type = sig(r#"name = "f"
params = [{ name = "a", type = "integer" }]
returns = "int""#);
        assert!(bad_type.unwrap_err().contains("unknown type"));
    }
}
