## Approach: three pointers, one pass

Keep the belt in four regions:

| `0 .. low−1` | `low .. mid−1` | `mid .. high` | `high+1 .. n−1` |
| --- | --- | --- | --- |
| glass | paper | not looked at yet | plastic |

Start with `low = mid = 0` and `high = n − 1`, and repeat while `mid ≤ high`:

- `items[mid] = 0`: swap it with `items[low]` (which is paper, or the same item), then `low++`, `mid++`;
- `items[mid] = 1`: `mid++`;
- `items[mid] = 2`: swap it with `items[high]`, then `high--`. `mid` stays, because the item that arrived from `high` is new.

Each step shrinks the unknown region by one, so the loop ends after at most n steps with the belt sorted.

### Walkthrough

`[2, 0, 1, 2, 0]`: plastic at 0 swaps with the last item → `[0, 0, 1, 2, 2]`, high = 3; glass, glass, paper move `low` and `mid`; at mid = 3 the item is plastic: swap with itself, high = 2, and the loop ends.

**Complexity:** O(n) time, O(1) extra space.
