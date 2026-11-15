## Approach: work in seconds, format at the end

A time like `4:05` has two units, and adding such times by hand means carrying seconds into minutes. Converting every lap to **seconds** first removes the problem: then the total is an ordinary sum and the fastest lap an ordinary minimum.

```
seconds("4:05") = 4 · 60 + 5 = 245
```

For Ana in Example 1 the laps are 245, 238 and 252 seconds: the total is 735 and the fastest 238. Back to `m:ss`:

```
735 / 60 = 12 (whole division)    735 % 60 = 15    →  12:15
238 / 60 = 3                      238 % 60 = 58    →  3:58
```

The minutes are printed as they come, with no limit at 59 (`119:58` in Example 2), and the seconds always with two digits (`0:10`, not `0:1`).

**Why it is correct:** `m · 60 + ss` gives every time its exact length in seconds, so sums and comparisons of those numbers are sums and comparisons of the real times. Division by 60 and the remainder undo the conversion, so the printed text is the right time in the required format.

**A trap:** comparing the lap strings directly looks tempting but is wrong: as text, `"10:00" < "9:59"`, because `1` comes before `9`. Always compare numbers.

**Complexity:** O(total input) time; each runner needs only a running sum and a running minimum.

**Reading and writing:** read the whole input at once, split it into tokens and walk them with a position (name, `k`, then `k` laps), as in Ticket Tally; collect the output lines and print them once. Padding in each language: `f"{s // 60}:{s % 60:02d}"` (Python), `` `${Math.floor(s / 60)}:${String(s % 60).padStart(2, "0")}` `` (TypeScript; `/` alone gives a fraction), `sprintf("%d:%02d", intdiv($s, 60), $s % 60)` (PHP), `format!("{}:{:02}", s / 60, s % 60)` (Rust).
