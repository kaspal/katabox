Checking every room for every meeting can cost O(n²) when many rooms are open. You need
two quick answers at each start: which rooms have just become free, and which free room
has the smallest number.
---
Sort the meeting positions by (start, position) and keep the answers indexed by the
original position. A heap of busy rooms ordered by end time tells you which rooms free up
by the current start.
---
Use two min-heaps: busy rooms as (end, room) and free room numbers. At each meeting, move
every busy room with end ≤ start into the free heap, then take the smallest free number,
or open room `count + 1` if none is free, and push (end, room) into the busy heap.
