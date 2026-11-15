## Approach: both hands in half-degrees

Measure each hand clockwise from the 12. The dial has 12 hours, so first fold the 24-hour time: `h = hh mod 12` (13:00 and 01:00 look the same, and 12:00 is 0).

- The minute hand turns 360 degrees in 60 minutes: `6 · mm` degrees.
- The hour hand turns 360 degrees in 12 hours, 30 degrees per hour, and moves on half a degree every minute: `30 · h + 0.5 · mm` degrees.

The only fraction is that half degree. **Doubling everything** removes it: in half-degrees the hour hand is at `60 · h + mm` and the minute hand at `12 · mm`, and a full turn is 720. So

```
gap    = |60 · h − 11 · mm|          (0 ≤ gap ≤ 660)
answer = min(gap, 720 − gap)         half-degrees, 0..360
```

and the angle is `answer / 2` degrees. It is always a whole number or a whole number and a half, so print `answer / 2` with whole division, then `.5` when `answer` is odd and `.0` when it is even.

For 09:45: `h = 9`, `gap = |540 − 495| = 45`, `min(45, 675) = 45` half-degrees, printed `22.5`. For 00:59: `gap = |0 − 649| = 649`, and the other way round is shorter: `720 − 649 = 71`, printed `35.5`.

**Why it is correct:** the two formulas are the hands' exact positions, and the gap between two directions on a circle is either their difference or the rest of the circle, whichever is smaller. Working in half-degrees only changes the unit, so nothing is rounded anywhere.

**Floating point would work too:** a multiple of 0.5 is stored exactly in a double, and `.1f` formatting (`toFixed(1)`, `sprintf("%.1f")`, `{:.1}`) prints it exactly. The whole-number version simply needs no such argument. Do not forget the `mod 12`: without it, 18:00 would give `|1080 − 0| = 1080` half-degrees and a negative "other way round".

**Complexity:** O(1) per time, O(Q) in all.

**Reading and writing:** read all input at once, split it into tokens and take the hours and minutes from each `hh:mm` (the characters before and after the colon; `"09"` reads as 9 in every language). Collect the output lines and print them once.
