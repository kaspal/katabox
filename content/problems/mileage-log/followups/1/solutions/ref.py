def long_days(miles: list[int], trips: list[list[int]]) -> list[int]:
    n = len(miles)
    tree = [0] * (n + 1)  # Fenwick tree over day positions, 1-based

    def add(i):
        i += 1
        while i <= n:
            tree[i] += 1
            i += i & -i

    def prefix(i):  # long days among positions 0..i-1
        s = 0
        while i > 0:
            s += tree[i]
            i -= i & -i
        return s

    days = sorted(range(n), key=lambda d: -miles[d])
    order = sorted(range(len(trips)), key=lambda j: -trips[j][2])
    out = [0] * len(trips)
    p = 0
    for j in order:
        a, b, x = trips[j]
        while p < n and miles[days[p]] >= x:
            add(days[p])
            p += 1
        out[j] = prefix(b + 1) - prefix(a)
    return out
