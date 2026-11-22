def brackets_ok(code: str) -> bool:
    # Obviously correct: remove matched pairs until nothing changes.
    while any(p in code for p in ("()", "[]", "{}")):
        for p in ("()", "[]", "{}"):
            code = code.replace(p, "")
    return code == ""
