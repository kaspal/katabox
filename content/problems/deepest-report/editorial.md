## The bug

The starter computes each level with a memoized recursive helper:

```python
def level(v: int) -> int:
    if manager[v] == -1:
        return 1
    if depth[v] == 0:
        depth[v] = level(manager[v]) + 1
    return depth[v]
```

The logic is correct, and the memo makes the total work linear. The problem is the depth of the recursion: when `level` starts at a person whose ancestors are not known yet, it makes one nested call per manager on the way up. The constraints allow a single chain of 5·10⁴ people, and that many nested calls overflow the call stack, so the run crashes with a runtime error instead of returning an answer.

The order matters. With `manager = [-1, 0, 1, ...]` (each person reports to the one before), the loop visits the boss first and every later call stops after one step at a known depth. With the chain reversed, `manager = [1, 2, 3, ..., 49999, -1]` (5·10⁴ people, each reporting to the next one, the last is the boss), the very first call `level(0)` must go 5·10⁴ calls deep before anything is memoized. The answer is 50000, but the code crashes.

Raising the recursion limit is not a reliable fix: the real stack can still run out, and deeper charts would only move the crash.

## The fix

Replace the recursion with an explicit walk. Climb from `v` until you reach the boss or someone with a known depth, recording the path, then fill the depths back down the path:

```python
def deepest_report(manager: list[int]) -> int:
    n = len(manager)
    depth = [0] * n
    best = 0
    for v in range(n):
        path = []
        u = v
        while u != -1 and depth[u] == 0:
            path.append(u)
            u = manager[u]
        d = 0 if u == -1 else depth[u]
        for w in reversed(path):
            d += 1
            depth[w] = d
        best = max(best, depth[v])
    return best
```

The TypeScript version has the same recursive `level` helper; replace it with the same climb-then-fill loop using a plain array for the path.

**Complexity:** O(n) time, since each person is put on a path once, and O(n) extra space for `depth` and the path.
