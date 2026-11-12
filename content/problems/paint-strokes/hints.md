Marking every centimetre is impossible: the fence is up to 10⁹ cm long. Think about whole strokes instead.
---
If the strokes are sorted by `start`, a stroke either overlaps (or touches) the painted run you are building, or starts a new one after a gap.
---
Sort by `start`. Keep the current run `[runStart, runEnd)`. For each stroke: if `start ≤ runEnd`, extend `runEnd = max(runEnd, end)`; otherwise add `runEnd − runStart` to the total and start a new run. Add the last run at the end.
