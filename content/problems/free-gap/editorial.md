## Approach: sort the slots, sweep with the end of the busy time

**Minutes, not clock times.** Convert every `hh:mm` to `hh · 60 + mm`. Now a slot is a half-open range `[start, end)` of whole numbers, and the length of a free stretch is a subtraction.

**The sweep.** Sort the day's slots by start. Keep `free_from`, the first minute after everything busy seen so far, starting at `open`, and the best stretch `(best_len, best_start)`, starting at length 0:

```
for (s, e) in slots sorted by start:
    if s − free_from > best_len: best_len, best_start = s − free_from, free_from
    free_from = max(free_from, e)
if close − free_from > best_len: best_len, best_start = close − free_from, free_from
```

The `max` handles overlaps: a slot that ends inside the busy time already seen does not move `free_from` back.

Example 1, first day, in minutes: open 540, close 1020, slots sorted `[600, 690)`, `[630, 720)`, `[780, 840)`.

| slot | gap before it | free_from after |
| --- | --- | --- |
| 600-690 | 540-600 (60): best | 690 |
| 630-720 | none (630 < 690) | 720 |
| 780-840 | 720-780 (60): not longer | 840 |
| close | 840-1020 (180): best | |

The answer is 840-1020, printed `14:00-17:00`.

**Why it is correct:** when a slot starting at `s` is reached, every slot that starts earlier has been seen, and they cover everything from `open` up to `free_from` that is busy. If `s > free_from`, the minutes `free_from .. s − 1` are covered by none of the earlier slots (they all end by `free_from`) and by none of the later ones (they start at `s` or after): they form a free stretch, and it cannot be extended on either side. Every free stretch is found this way, as the gap before some slot or before `close`, and they are found from earliest to latest. Replacing the best only on a **strictly** longer stretch therefore keeps the earliest of equally long ones. If no gap is ever positive, the room is fully booked: print `BOOKED`.

**Complexity:** O(k log k) per day for the sort, then O(k) for the sweep; O(Σk log k) in all.

**Another correct way:** mark each of the day's minutes busy or free in an array of 1440 and walk it. It also fits these limits, but its cost depends on the length of the slots rather than on their number; the sweep does not care how fine the clock is.

**Printing:** zero-pad both parts: `f"{m // 60:02d}:{m % 60:02d}"` (Python), `String(h).padStart(2, "0")` (TypeScript), `sprintf("%02d:%02d", ...)` (PHP), `format!("{:02}:{:02}", ...)` (Rust). Read all of stdin at once and write the `T` lines in one go.
