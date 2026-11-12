def two_lonely(ids: list[int]) -> list[int]:
    # Correct but O(n²): look for each sock's partner elsewhere in the list.
    n = len(ids)
    out = []
    for i in range(n):
        if not any(j != i and ids[j] == ids[i] for j in range(n)):
            out.append(ids[i])
    return sorted(out)
