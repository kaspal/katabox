def short_names(names: list[str]) -> list[str]:
    # Obviously correct: try every beginning, shortest first, against every other name.
    out = []
    for i, name in enumerate(names):
        label = name
        for k in range(1, len(name) + 1):
            p = name[:k]
            if not any(j != i and other.startswith(p) for j, other in enumerate(names)):
                label = p
                break
        out.append(label)
    return out
