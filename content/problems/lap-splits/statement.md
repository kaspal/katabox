The coach of a running club writes down every lap each runner completes in a training session. After the session she wants two numbers per runner: the total time on the track and the fastest lap.

Times are written as `m:ss`: the minutes with no leading zero, then the seconds, always with two digits (`0:59`, `4:05`, `12:30`). Totals use the same format and stay in minutes even past an hour: 119 minutes and 58 seconds is `119:58`.

This is a **program problem**: read the coach's sheet from standard input and print each runner's total and fastest lap to standard output.

## Input format

- The first line holds `T`, the number of runners.
- Then `T` lines, one per runner: the runner's `name`, then `k`, the number of laps, then the `k` lap times, all separated by single spaces.

## Output format

Print `T` lines, one per runner in input order: `name total fastest`, separated by single spaces, where `total` is the sum of the runner's laps and `fastest` the shortest lap, both in the `m:ss` format.

### Example 1

Input:

```
3
ana 3 4:05 3:58 4:12
bo 1 12:30
carmen 4 0:59 1:01 1:00 0:59
```

Output:

```
ana 12:15 3:58
bo 12:30 12:30
carmen 3:59 0:59
```

Ana ran 245 + 238 + 252 = 735 seconds, which is 12 minutes and 15 seconds; her fastest lap was `3:58`. A runner with one lap has the same total and fastest lap.

### Example 2

Input:

```
1
zoe 2 59:59 59:59
```

Output:

```
zoe 119:58 59:59
```

The total goes past an hour and is still written in minutes.

### Example 3

Input:

```
2
li 5 7:07 7:00 6:59 7:10 7:03
mo 2 0:01 0:09
```

Output:

```
li 35:19 6:59
mo 0:10 0:01
```

### Constraints

- 1 ≤ T ≤ 100
- A `name` has 1 to 12 lowercase letters (`a`-`z`).
- 1 ≤ k ≤ 50
- Every lap is written `m:ss`, with no leading zero in the minutes, and `0:01` ≤ lap ≤ `59:59`.
