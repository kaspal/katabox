## Approach: track the farthest reachable stone

The reachable stones always form a prefix `0..reach`: if stone `i` is reachable, so is every stone before it on the way (a jump from `j` can land on any stone up to `j + jumps[j]`).

Walk left to right with `reach = 0`. At each stone `i`, if `i > reach` the prefix has ended before `i` and the last stone cannot be reached. Otherwise extend `reach = max(reach, i + jumps[i])`. If the walk passes the last stone, the answer is true.

### Walkthrough

Example 1, `[2, 1, 0, 3, 1]`: stone 0 gives reach 2, stone 1 gives reach 2, stone 2 adds nothing, and stone 3 is beyond reach 2: false. Example 2, `[2, 0, 3, 0, 0, 1]`: reach 2, 2, then 5 at stone 2, which is the last stone.

**Complexity:** O(n) time, O(1) space. A search that tries every jump length from every reachable stone is O(n · max jump), O(n²) with long jumps.
