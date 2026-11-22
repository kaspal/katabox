## The bug

The starter is a standard backtracking search over the sorted weights, but its loop never backtracks:

```python
for j in range(i, len(items)):
    pick.append(items[j])
    go(j + 1, total + items[j])
```

Each jar is appended to `pick` and stays there after the recursive call returns. The next jar of the loop is then added on top of the previous one instead of replacing it. The total passed down is still right, but `pick` keeps growing: the only basket that ever gets checked with exactly `k` jars is the very first one (the `k` lightest jars), and after that `len(pick)` is always above `k`, so nothing else can match.

With `weights = [1, 2]`, `k = 1` and `target = 2`: the loop first appends 1 and checks the basket `[1]` (total 1, no match). Then it appends 2 without removing 1, so `pick = [1, 2]` with length 2, which is not `k`. The code returns `[]`; the answer is `[[2]]`.

## The fix

Undo each choice after exploring it, so `pick` is back to its previous state before the next jar is tried:

```python
for j in range(i, len(items)):
    pick.append(items[j])
    go(j + 1, total + items[j])
    pick.pop()
```

In TypeScript, add `pick.pop();` after the recursive `go(j + 1, total + items[j])` call in the same loop.

Because the weights are sorted and the loop tries them in increasing order, each basket comes out ascending and the baskets come out in lexicographic order with no extra sorting.

**Complexity:** O(2ⁿ · n) time in the worst case (at most 2ⁿ partial picks, copying a basket costs O(k)), O(n) extra space for the recursion and `pick`, besides the output.
