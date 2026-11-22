def span_sums(rain: list[int], queries: list[list[int]]) -> list[int]:
    prefix = [0]
    for x in rain:
        prefix.append(prefix[-1] + x)
    return [prefix[r] - prefix[l] for l, r in queries]
