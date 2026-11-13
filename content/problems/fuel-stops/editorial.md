## Approach: refuel in hindsight from the biggest passed station

Let `reach` be how far the van can get with the stops chosen so far (start with `fuel`). Walk the stations in order:

- while the station's position is beyond `reach`, the van must have stopped at some earlier station: take the one with the **most litres** among those passed and not yet used (a max-heap), add its litres to `reach`, count a stop; if there is none, return −1;
- then push this station's litres onto the heap.

At the end, do the same until `reach ≥ target`.

**Why the largest:** whenever more fuel is needed, using the largest available station extends the reach the most for one stop, and the choice of which earlier stations were used does not matter, only how many and how much they gave.

**Complexity:** O(n log n) time, O(n) space. The DP `far[t]` = farthest reach with t stops, updated for every station over every t, is O(n²).
