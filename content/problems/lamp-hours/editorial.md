## Approach: sort by start, then merge

After sorting the lamps by start time, lamps whose light overlaps or touches appear next to each other. Sweep them while keeping the current lit stretch `[s, e)`:

```
sort lamps by start
s, e = lamps[0]
total = 0
for start, end in lamps[1:]:
    if start <= e:            # overlaps or touches the stretch: extend it
        e = max(e, end)
    else:                     # a dark gap: close the stretch
        total += e - s
        s, e = start, end
total += e - s
```

Every lit second belongs to exactly one closed stretch, so nothing is counted twice. Touching lamps (`start == e`) are merged, which does not change the total but keeps the stretches maximal.

### Walkthrough

`[[1, 4], [2, 6], [8, 10]]` is already sorted. `[1, 4)` and `[2, 6)` merge into `[1, 6)`; `[8, 10)` starts after 6, so 5 is added and the new stretch adds 2. Total 7.

**Complexity:** O(n log n) for the sort and O(n) for the sweep. Marking every second costs O(n + T) with T up to 10⁹.
