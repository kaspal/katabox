## The idea

A traveller starts at city 0, visits every other city exactly once, and comes back to city 0. Given the distance between every pair of cities, what is the shortest round trip? Trying every order of the other `n - 1` cities means `(n - 1)!` tours, which explodes quickly.

The key observation (the **Held-Karp** method): once you are partway through a trip, the order you came in no longer matters, only **which cities** you have visited and **where you are now**. So `best[mask][last]` is the shortest path that starts at city 0, visits exactly the cities in `mask`, and ends at `last`.

The `mask` is a **bitmask**: a set stored as the bits of a number, where bit `c` is 1 when city `c` is in the set. With four cities, `mask = 11` is `1011` in binary: cities 0, 1 and 3. Adding city `c` is `mask | (1 << c)`, and `mask & (1 << c)` tests whether `c` is already there.

Only `best[1][0] = 0` is known at first: just city 0, standing at city 0. Everything else starts at a huge number. The sample visits the masks in **increasing** order, odd ones only (they all contain city 0). Adding a city always makes the number larger, so every state is final before it is extended. From each reachable `(mask, last)`, it tries every unvisited `city` and improves `best[mask | (1 << city)][city]` with `best[mask][last] + dist[last][city]`. At the end, the full mask holds the shortest path through all cities ending at each `last`; adding `dist[last][0]` closes the loop, and the smallest total is the output.

The distances are typed as a symmetric matrix, up to 5 cities.

### The default input

Four cities; city 0 is 2, 9 and 10 away from cities 1, 2 and 3; 1 to 2 is 6, 1 to 3 is 4, 2 to 3 is 3. From mask `0001`, the first moves give `best[0011][1] = 2`, `best[0101][2] = 9` and `best[1001][3] = 10`. The full mask `1111` ends with 16 at city 1 (0, 2, 3, 1), 9 at city 2 (0, 1, 3, 2) and 11 at city 3 (0, 1, 2, 3). Closing the loop gives `16 + 2 = 18`, `9 + 9 = 18` and `11 + 10 = 21`.

The output is `[18]`: 0, 1, 3, 2, 0, or the same loop backwards.

## Steps

1. Make a table of `2ⁿ` masks by `n` cities, all "unreached"; set `best[1][0] = 0`.
2. For each odd mask in increasing order, each reached `last`, and each `city` not in the mask: improve `best[mask | (1 << city)][city]` with `best[mask][last] + dist[last][city]`.
3. Return the smallest `best[full][last] + dist[last][0]` over `last ≠ 0`.

## Cost

| Case | Cost |
|---|---|
| Every case | `O(2ⁿ · n²)`: each mask and last city tries every next city |
| Extra space | `O(2ⁿ · n)` for the table |

## Pitfalls

- **Forgetting the way home.** The full mask holds open paths; the closing `dist[last][0]` must be added.
- **Expecting it to scale.** `2ⁿ · n²` is far better than `(n - 1)!` (for 15 cities, about 7 million steps against 87 billion tours), but it still doubles with each city; around 20 is the practical limit.
- **Bit precedence.** In TypeScript and other C-like languages, `mask & 1 << c === 0` compares before it masks; add parentheses.

## When to use it

Use it when you must visit every item of a small set, in the best order: delivery rounds, ordering tasks with changeover costs, covering a few checkpoints. The linked problem, Delivery Round, is this method on a matrix where the two directions may differ.

## Related topics

- [Bit manipulation basics](learn:foundations/bit-manipulation): the shifts and masks used here.
- [0/1 knapsack](learn:dynamic-programming/knapsack-01): another table over subsets, one item at a time.
- [Floyd-Warshall](learn:graphs/floyd-warshall): shortest paths between all pairs, when detours are allowed.
- [Dijkstra's shortest paths](learn:graphs/dijkstra): shortest paths from one city, with no need to visit them all.
