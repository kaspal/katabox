def short_names(names: list[str]) -> list[str]:
    # Correct but O(n²·L): the label is one letter longer than the longest common
    # beginning with any other name (or the whole name).
    out = []
    for i, name in enumerate(names):
        longest = 0
        for j, other in enumerate(names):
            if i == j:
                continue
            k = 0
            while k < len(name) and k < len(other) and name[k] == other[k]:
                k += 1
            if k > longest:
                longest = k
        out.append(name[: longest + 1])
    return out
