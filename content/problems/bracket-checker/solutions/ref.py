def brackets_ok(code: str) -> bool:
    pairs = {")": "(", "]": "[", "}": "{"}
    stack = []
    for c in code:
        if c in "([{":
            stack.append(c)
        elif not stack or stack.pop() != pairs[c]:
            return False
    return not stack
