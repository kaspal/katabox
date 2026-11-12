def rotation_groups(codes: list[str]) -> int:
    seen = set()
    for code in codes:
        seen.add(min(code[i:] + code[:i] for i in range(len(code))))
    return len(seen)
