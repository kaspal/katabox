def is_balanced(text: str) -> bool:
    # Obviously correct: drop letters, then delete adjacent matching pairs until none is left.
    s = "".join(ch for ch in text if ch in "()[]{}")
    while True:
        t = s.replace("()", "").replace("[]", "").replace("{}", "")
        if t == s:
            return s == ""
        s = t
