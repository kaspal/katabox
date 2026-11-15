Adding minutes and seconds separately means carrying every 60 seconds into a minute. Is there a single unit that makes adding and comparing easy?
---
Turn every lap into seconds: `m:ss` is `m · 60 + ss`. The total is a plain sum and the fastest lap a plain minimum. Do not compare the lap strings themselves: as text, `10:00` comes before `9:59`.
---
For each runner, read the name, `k` and the next `k` laps; convert them to seconds, then take the sum and the minimum. Write a time back as `total / 60` (whole division) for the minutes, a colon, and `total % 60` padded to two digits for the seconds.
