## The bug

The starter is a union-find, and the check `find(a) != find(b)` is correct. The join is not:

```python
if find(a) != find(b):
    parent[a] = b
    clubs -= 1
```

`parent[a] = b` rewires the person `a`, not the root of `a`'s club. If `a` already pointed somewhere, that link is lost: everyone `a` was connected through falls out of the merged club, and later friendships between them are counted as new merges. The count can even reach 0.

With `n = 3` and `friendships = [[0, 1], [0, 2], [1, 2]]`:

- `[0, 1]`: roots 0 and 1 differ, `parent[0] = 1`, 2 clubs.
- `[0, 2]`: roots 1 and 2 differ, `parent[0] = 2`, 1 club. Person 1 is now a root again, detached from 0.
- `[1, 2]`: roots 1 and 2 differ, so the code merges again: 0 clubs.

The code returns `[2, 1, 0]`; the answer is `[2, 1, 1]`.

## The fix

Link the two roots, so both whole clubs join, and halve the path in `find` so the trees stay shallow on long chains:

```python
def find(x: int) -> int:
    while parent[x] != x:
        parent[x] = parent[parent[x]]
        x = parent[x]
    return x

for a, b in friendships:
    ra, rb = find(a), find(b)
    if ra != rb:
        parent[ra] = rb
        clubs -= 1
    out.append(clubs)
```

In TypeScript, make the same change: store `find(a)` and `find(b)` in `ra` and `rb`, and set `parent[ra] = rb` when they differ. The reference also halves the path inside `find` (`parent[x] = parent[parent[x]]`); that is optional here (the tests pass without it) but keeps the trees shallow on long chains.

**Complexity:** O(n + m log n) time for m friendships (amortized, with path halving), O(n) extra space besides the output.
