## Approach: dynamic programming over amounts

Let `best[x]` be the fewest tokens that pay exactly `x` (∞ if impossible). The last token of an optimal payment of `x` is some `c`, and the tokens before it must be an optimal payment of `x − c`. So:

- `best[0] = 0`
- `best[x] = 1 + min over c in coins with c ≤ x of best[x − c]`

Compute `best` from 0 up to `amount`; each entry looks at every token once.

### Walkthrough

`coins = [1, 6, 10]`, `amount = 12`:

| x | 0 | 1 | … | 5 | 6 | 7 | … | 10 | 11 | 12 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| best | 0 | 1 | … | 5 | 1 | 2 | … | 1 | 2 | 2 |

`best[12] = 1 + min(best[11], best[6], best[2]) = 1 + min(2, 1, 2) = 2`.

**Complexity:** O(amount · k) time and O(amount) space, with k token values. Trying every count of every token is exponential, and the greedy "largest token first" is not always optimal.
