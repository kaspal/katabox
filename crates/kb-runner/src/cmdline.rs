//! Windows command-line quoting. Pure string logic so it is tested on every platform.

/// Append `arg` to `out`, quoted so that `CommandLineToArgvW` (and the MSVC C runtime)
/// parse it back as exactly one argument with the same content.
pub fn quote_windows_arg(arg: &str, out: &mut String) {
    let needs_quotes = arg.is_empty() || arg.contains([' ', '\t', '\n', '\u{0B}', '"']);
    if !needs_quotes {
        out.push_str(arg);
        return;
    }
    out.push('"');
    let mut backslashes = 0usize;
    for c in arg.chars() {
        match c {
            '\\' => backslashes += 1,
            '"' => {
                // Escape the preceding backslashes and the quote itself.
                out.extend(std::iter::repeat_n('\\', backslashes * 2 + 1));
                out.push('"');
                backslashes = 0;
            }
            _ => {
                out.extend(std::iter::repeat_n('\\', backslashes));
                out.push(c);
                backslashes = 0;
            }
        }
    }
    // Backslashes before the closing quote must be doubled.
    out.extend(std::iter::repeat_n('\\', backslashes * 2));
    out.push('"');
}

#[cfg(test)]
mod tests {
    use super::quote_windows_arg;

    fn q(s: &str) -> String {
        let mut out = String::new();
        quote_windows_arg(s, &mut out);
        out
    }

    #[test]
    fn plain_args_are_unchanged() {
        assert_eq!(q("-I"), "-I");
        assert_eq!(q(r"C:\a\b"), r"C:\a\b");
    }

    #[test]
    fn spaces_and_empty_are_quoted() {
        assert_eq!(q(""), r#""""#);
        assert_eq!(q("print(1) + 2"), r#""print(1) + 2""#);
    }

    #[test]
    fn quotes_and_backslashes_are_escaped() {
        assert_eq!(q(r#"say "hi""#), r#""say \"hi\"""#);
        assert_eq!(q(r#"a\"b"#), r#""a\\\"b""#);
        assert_eq!(q(r"dir with space\"), r#""dir with space\\""#);
        assert_eq!(q(r"a\\b c"), r#""a\\b c""#);
    }
}
