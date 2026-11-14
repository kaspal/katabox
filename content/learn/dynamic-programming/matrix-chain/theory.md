## The idea

Multiplying a `p × q` matrix by a `q × r` matrix takes `p · q · r` scalar multiplications and gives a `p × r` matrix. A **chain** of matrices can be multiplied in any grouping, because matrix multiplication is associative: the result is the same, but the cost is not. Which grouping costs the least? The input is a list `dims`, where matrix `k` is `dims[k] × dims[k + 1]`.

Whatever the best grouping of matrices `i` to `j` is, it has a **last multiplication**: the left part `i..k` times the right part `k + 1..j`, for some **split** `k`. Each part should itself be multiplied as cheaply as possible, and the final product costs `dims[i] · dims[k + 1] · dims[j + 1]`. So:

`cost[i][j] = min over i ≤ k < j of cost[i][k] + cost[k + 1][j] + dims[i] · dims[k + 1] · dims[j + 1]`

A single matrix costs 0. This is **interval DP**: each cell is a range of the chain, and it needs the cells of shorter ranges inside it. So the sample fills by **span**, the number of matrices in the range: first every span of 2, then 3, up to the whole chain. In the table that is one **diagonal** at a time, moving up and to the right, ending in the top-right corner `cost[0][n - 1]`, which is the output.

### The default input

`dims = [8, 20, 4, 30]`: A is 8 × 20, B is 20 × 4, C is 4 × 30. Span 2: `cost[0][1] = 8 · 20 · 4 = 640` for AB, and `cost[1][2] = 20 · 4 · 30 = 2400` for BC. Span 3 tries both splits. Split after A, A(BC): `0 + 2400 + 8 · 20 · 30 = 7200`. Split after B, (AB)C: `640 + 0 + 8 · 4 · 30 = 1600`.

The output is `[1600]`, four and a half times cheaper than the other order.

## Steps

1. Let `n = len(dims) - 1`; make an `n × n` table of zeros (one matrix costs 0).
2. For each span from 2 to `n`, and each start `i` with `j = i + span - 1`: try every split `k` from `i` to `j - 1` and keep the smallest total.
3. Return `cost[0][n - 1]`.

## Cost

| Case | Cost |
|---|---|
| Every case | `O(n³)`: about `n²/2` ranges, each trying up to `n` splits |
| Extra space | `O(n²)` for the table |

## Pitfalls

- **Filling row by row.** `cost[0][2]` needs `cost[1][2]`, a row further down; going by span guarantees every shorter range is done.
- **The wrong dimensions.** The last product is `dims[i] · dims[k + 1] · dims[j + 1]`: rows of the first matrix, the shared size at the split, columns of the last one.
- **Greedy splits.** Doing the cheapest single product first is not always best; only comparing every split is safe.

## When to use it

Use the pattern when a range is built from two smaller ranges and the cost of joining them depends on where you split: grouping products, merging neighbouring piles, cutting a piece at several marks. The linked problem, Log Cutting, is interval DP too: a piece between two marks, a choice of which cut comes first.

## Related topics

- [Rod cutting (unbounded knapsack)](learn:dynamic-programming/rod-cutting): choosing cuts, with a one-dimensional table.
- [0/1 knapsack](learn:dynamic-programming/knapsack-01): another table filled in a careful order.
- [Floyd-Warshall](learn:graphs/floyd-warshall): another `O(n³)` table that tries every middle point.
- [Merge intervals](learn:greedy/merge-intervals): ranges handled greedily instead.
