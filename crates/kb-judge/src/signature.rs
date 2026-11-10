//! The typed function signature from `[signature]` in problem.toml, and how each
//! language spells its names and types.
//!
//! Type grammar (deliberately small; grows with the problem set):
//! `int | float | string | bool | <type>[]`, e.g. `int[]`, `string[][]`.

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
            _ => Err(format!(
                "unknown type {s:?} (expected int | float | string | bool, optionally followed by [])"
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
                Type::Array(t) => format!("list[{}]", t.spelled_in(lang)),
            },
            // JavaScript has no annotations; the TypeScript spelling is used in JSDoc later.
            Language::TypeScript | Language::JavaScript => match self {
                Type::Int | Type::Float => "number".into(),
                Type::String => "string".into(),
                Type::Bool => "boolean".into(),
                Type::Array(t) => format!("{}[]", t.spelled_in(lang)),
            },
            Language::Php | Language::Rust => unimplemented!("{lang} types are not mapped yet"),
        }
    }
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
        Language::Python | Language::Rust | Language::Php => snake_case(name),
        Language::TypeScript | Language::JavaScript => name.to_string(),
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
        Language::Php | Language::Rust => &[],
    };
    if reserved.contains(&name.as_str()) {
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
