## Approach: sort starts and ends, then sweep

At any moment, the meetings in progress need different rooms, and as soon as a room frees up it can be reused. So the answer is the maximum number of meetings in progress at once.

Sort the start times and the end times separately. Walk through both lists in time order: when the next event is a start, one more room is in use; when it is an end, one room is freed. If a start and an end happen at the same minute, process the end first (the room is free from `end` on). Track the maximum.

### Walkthrough

`meetings = [[1, 5], [2, 6], [3, 7], [6, 8]]`: starts `[1, 2, 3, 6]`, ends `[5, 6, 7, 8]`.

| event | rooms in use | max |
| --- | --- | --- |
| start 1 | 1 | 1 |
| start 2 | 2 | 2 |
| start 3 | 3 | 3 |
| end 5 | 2 | 3 |
| end 6 (before start 6) | 1 | 3 |
| start 6 | 2 | 3 |

The rest are ends; the answer is 3.

**Complexity:** O(n log n) time for the sorting, O(n) extra space. Counting, for each meeting, how many others are running when it starts is O(n²).
