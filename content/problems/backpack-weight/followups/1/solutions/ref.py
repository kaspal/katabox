def best_pack_copies(weights: list[int], values: list[int], counts: list[int], capacity: int) -> int:
    best = [0] * (capacity + 1)
    for w, v, c in zip(weights, values, counts):
        # Split c copies into bundles 1, 2, 4, ... and a remainder: any count up to c is a
        # sum of bundles, each bundle packed once (0/1 knapsack).
        k = 1
        while c > 0:
            take = min(k, c)
            bw, bv = w * take, v * take
            for cap in range(capacity, bw - 1, -1):
                if best[cap - bw] + bv > best[cap]:
                    best[cap] = best[cap - bw] + bv
            c -= take
            k *= 2
    return best[capacity]
