## The idea

A row of squares holds numbers. Standing on square `i`, you may jump forward by any amount from 1 up to the number written there; a 0 means you cannot move from that square. Starting on square 0, can you reach the last square?

Trying every jump branches quickly. The greedy view needs only one number: the **farthest reach**, the highest index any square seen so far can jump to. Walk left to right. If the current index is beyond the reach, no earlier square can get here, so the walk is **stuck**. Otherwise, square `i` reaches `i + jump`; if that is farther, the reach grows. Every square up to the reach is reachable, because the jump that set it may stop anywhere along the way.

The output is `[1, reach]` when the walk gets through the whole row, with the reach capped at the last index, or `[0, reach]` when it gets stuck, the farthest index it could get to.

### The default input

Ten squares: `2, 3, 1, 1, 0, 2, 4, 0, 1, 1`. Square 0 reaches 2. Square 1 reaches `1 + 3 = 4`. Squares 2, 3 and 4 reach 3, 4 and 4: no gain. Square 5 is beyond the reach of 4: stuck.

The output is `[0, 4]`. The 0 on square 4 is a trap: every route ends there, and the 4 on square 6, which would have reached the last square, is never visited.

## Steps

1. Set `reach = 0`.
2. For each index `i` from left to right: if `i > reach`, return `[0, reach]`.
3. If `i + jumps[i] > reach`, set `reach = i + jumps[i]`.
4. After the loop, return `[1, min(reach, n - 1)]`.

## Cost

| Case | Cost |
|---|---|
| Every case | `O(n)`: one pass, constant work per square |
| Extra space | `O(1)`: only the reach |

## Pitfalls

- **Checking only the jump from square 0.** The reach comes from every square visited so far, not from the latest one.
- **Order of the checks.** Test `i > reach` before using square `i`; a square beyond the reach must not extend it.
- **Reading a 0 as "stuck".** A 0 traps you only if nothing earlier jumps past it. In `2, 0, 1` square 0 leaps over the 0.

## When to use it

Use it to decide whether a single forward pass of hops can cross a route, such as stones across a river or platforms in a level. The linked problem, Stepping Stones, is this pattern.

A related question asks for the **fewest jumps** to reach the end. It also walks left to right, but in layers: all squares reachable with one jump, then those reachable with two, counting a new jump each time the walk passes the end of the current layer.

## Related topics

- [Breadth-first search](learn:graphs/bfs): the fewest-jumps variant is a BFS on squares, done in place.
- [Climbing stairs](learn:dynamic-programming/climbing-stairs): counting the ways to climb instead of asking whether you can.
- [Maximum subarray (Kadane)](learn:dynamic-programming/maximum-subarray): another single pass that keeps one running best.
