## Approach: sort, then sweep the runs

Sort the strokes by `start`. Walking through them in that order, the painted fence is a series of disjoint **runs**:

- if a stroke starts at or before the end of the current run, it belongs to that run, which may grow: `runEnd = max(runEnd, end)`;
- otherwise there is a gap: the current run is complete, so add its length and start a new run with this stroke.

After the last stroke, add the final run.

### Walkthrough

`strokes = [[2, 6], [10, 12], [4, 8]]`, sorted: `[2, 6], [4, 8], [10, 12]`:

| stroke | run before | action | run after | total |
| --- | --- | --- | --- | --- |
| [2, 6] | — | start | [2, 6) | 0 |
| [4, 8] | [2, 6) | 4 ≤ 6: extend | [2, 8) | 0 |
| [10, 12] | [2, 8) | gap: close (6) | [10, 12) | 6 |
| end | | close (2) | | 8 |

**Complexity:** O(n log n) time for sorting and O(n) for the sweep. Merging each stroke into a list of runs by scanning the whole list is O(n²) when the strokes do not overlap.
