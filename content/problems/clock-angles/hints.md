The dial has only 12 hours: 15:00 and 03:00 look the same. Measure where each hand points, as an angle from the 12, and compare the two.
---
From the 12, the minute hand is at `6 · mm` degrees and the hour hand at `30 · (hh mod 12) + 0.5 · mm` degrees. Their difference can be more than 180: the other way round the dial is then shorter, `360 − difference`.
---
Count in half-degrees to keep everything whole: the hour hand is at `60 · (hh mod 12) + mm`, the minute hand at `12 · mm`, so `gap = |60 · (hh mod 12) − 11 · mm|` and the answer is `min(gap, 720 − gap)` half-degrees. Print `answer / 2` (whole division), then `.5` if `answer` is odd and `.0` if it is even.
