P = 1_000_003


def team_picks(queries: list[list[int]]) -> list[int]:
    # Correct but O(k) per query: multiply the k factors of each answer.
    out = []
    for n, k in queries:
        if k > n:
            out.append(0)
            continue
        k = min(k, n - k)
        num = den = 1
        for i in range(k):
            num = num * (n - i) % P
            den = den * (i + 1) % P
        out.append(num * pow(den, P - 2, P) % P)
    return out
