def rotation_groups(codes: list[str]) -> int:
    # Obviously correct: group codes one by one, comparing with a member of each group
    # by trying every rotation.
    groups = []
    for code in codes:
        for g in groups:
            if len(g) == len(code) and any(g[i:] + g[:i] == code for i in range(len(g))):
                break
        else:
            groups.append(code)
    return len(groups)
