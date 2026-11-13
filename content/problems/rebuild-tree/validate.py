def validate(inp):
    pre, ino = inp["preorder"], inp["inorder"]
    if not isinstance(pre, list) or not isinstance(ino, list) or not all(type(v) is int for v in pre + ino):
        return "preorder and inorder are lists of integers"
    if len(pre) != len(ino) or len(pre) > 10**4:
        return "0 ≤ preorder.length = inorder.length ≤ 10⁴"
    if len(set(pre)) != len(pre) or set(pre) != set(ino) or not all(-(10**6) <= v <= 10**6 for v in pre):
        return "All numbers are different, between -10⁶ and 10⁶."
    # Rebuild iteratively, checking consistency and the height.
    pos = {v: i for i, v in enumerate(ino)}
    stack = [(0, len(pre), 0, 1)]  # preorder start, length, inorder start, depth
    while stack:
        ps, length, ins, depth = stack.pop()
        if length == 0:
            continue
        if depth > 1000:
            return "Both lists come from the same binary tree, which has at most 1000 levels."
        r = pos[pre[ps]]
        left = r - ins
        if not 0 <= left < length:
            return "Both lists come from the same binary tree, which has at most 1000 levels."
        stack.append((ps + 1, left, ins, depth + 1))
        stack.append((ps + 1 + left, length - left - 1, r + 1, depth + 1))
    return None
