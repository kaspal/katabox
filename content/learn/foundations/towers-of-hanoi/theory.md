## The idea

`n` disks sit on peg 1, biggest at the bottom. Move them all to peg 3, one disk at a time, never putting a bigger disk on a smaller one. The trick is to think only about the biggest disk: it can move only when every smaller disk is out of the way, on the spare peg. So `hanoi(n, source, target, spare, moves)` moves the `n - 1` smaller disks to the spare peg, moves the biggest disk, then moves the `n - 1` disks back on top of it. Moving `n - 1` disks is the same problem, one size smaller.

## Steps

1. If `n == 0`, there is nothing to move: return.
2. `hanoi(n - 1, source, spare, target, moves)`: clear the smaller disks onto the spare peg.
3. Move disk `n` from `source` to `target` and record the move.
4. `hanoi(n - 1, spare, target, source, moves)`: stack the smaller disks back on top of it.
5. `solve(n)` starts it all with `hanoi(n, 1, 3, 2, moves)`.

## Cost

| Measure | Cost |
|---|---|
| Moves | `2ⁿ − 1`, the fewest possible |
| Calls | `2ⁿ⁺¹ − 1`, counting the `n == 0` calls that return at once |
| Time | `O(2ⁿ)` |
| Stack depth | `O(n)`: at most `n + 1` frames |

## Pitfalls

- **Swapping the peg roles.** The first recursive call goes to `spare` (with `target` as its spare); the second comes from `spare` to `target`. Mixing them up still makes legal-looking calls but puts disks in the wrong place.
- **Expecting a shortcut.** The biggest disk must move at least once, and each time the `n - 1` disks above it must leave and come back, so `2ⁿ − 1` moves is the minimum. Every extra disk doubles the work: 20 disks need over a million moves.
- **The first move depends on `n`.** With an odd number of disks the smallest disk first goes to peg 3, with an even number to peg 2. Try `n = 2` and `n = 3`.

## When to use it

Towers of Hanoi is the classic exercise for trusting recursion: assume the smaller call works and only plan the biggest step. The same "solve `n - 1`, do one step, solve `n - 1` again" shape appears in recursive backtracking and in counting arguments. It is also a clear example of exponential growth that no clever coding can avoid.

## Related topics

- [Recursion and the call stack](learn:foundations/recursion-call-stack): frames, base cases and the recursion tree.
- [Big-O growth curves](learn:foundations/big-o-growth): what `O(2ⁿ)` means in practice.
- Backtracking (coming soon).
