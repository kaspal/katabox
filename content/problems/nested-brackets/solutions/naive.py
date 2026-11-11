def is_balanced(text: str) -> bool:
    # Correct, but each pass removes only the innermost pairs: O(n²) on deep nesting.
    s = "".join(ch for ch in text if ch in "()[]{}")
    while True:
        t = s.replace("()", "", 1)
        t = t.replace("[]", "", 1)
        t = t.replace("{}", "", 1)
        if t == s:
            return s == ""
        s = t
