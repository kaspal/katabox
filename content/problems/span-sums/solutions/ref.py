def span_sums(rain: list[int], queries: list[list[int]]) -> list[int]:
    # prefix[i] is the total of the first i days, so days l..r is prefix[r + 1] - prefix[l].
    prefix = [0]
    for x in rain:
        prefix.append(prefix[-1] + x)
    return [prefix[r + 1] - prefix[l] for l, r in queries]
