def kth_in_window(scores: list[int], w: int, k: int) -> list[int]:
    # Scores are 0..300: a Fenwick tree over the values counts each window.
    size = 301
    tree = [0] * (size + 1)

    def add(v: int, d: int) -> None:
        i = v + 1
        while i <= size:
            tree[i] += d
            i += i & -i

    def kth_largest(kk: int) -> int:
        # The kk-th largest is the (w - kk + 1)-th smallest.
        need = w - kk + 1
        pos = 0
        step = 1 << 8
        while step:
            nxt = pos + step
            if nxt <= size and tree[nxt] < need:
                pos = nxt
                need -= tree[nxt]
            step >>= 1
        return pos  # value index (0-based)

    out = []
    for i, s in enumerate(scores):
        add(s, 1)
        if i >= w:
            add(scores[i - w], -1)
        if i >= w - 1:
            out.append(kth_largest(k))
    return out
