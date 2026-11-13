Several bus lines start from the central station of a town. Each line first leaves at minute `r`, then every `m` minutes after that: at minutes `r`, `r + m`, `r + 2·m`, and so on. The town wants a photo of every bus pulling out at the same moment, and the planners have drafted several timetables. For each one, what is the first minute at which all of its lines leave together?

This is a **program problem**: read the timetables from standard input and print the answers to standard output.

## Input format

- The first line holds `T`, the number of timetables.
- Each timetable starts with a line holding `n`, its number of bus lines, followed by `n` lines `r m`: the line first leaves at minute `r` and then every `m` minutes.

## Output format

Print `T` lines, one per timetable in order: the first minute `t ≥ 0` at which every line of that timetable leaves, or `-1` if that never happens. Answers can be close to 9·10¹⁵, so use 64-bit integers (they still fit exactly in a JavaScript number).

### Example 1

Input:

```
2
2
2 3
1 4
3
0 4
2 6
3 5
```

Output:

```
5
8
```

In the second timetable the lines leave every 4, 6 and 5 minutes; minute 8 is the first they share.

### Example 2

Input:

```
3
2
0 4
1 6
1
7 9
2
3 10
3 10
```

Output:

```
-1
7
3
```

In the first timetable one line always leaves at even minutes and the other at odd minutes, so they never meet.

### Example 3

Input:

```
1
3
4 7
2 9
0 10
```

Output:

```
200
```

### Constraints

- 1 ≤ T ≤ 2000
- 1 ≤ n ≤ 6
- 1 ≤ m ≤ 10⁶
- 0 ≤ r < m
- In each timetable, the least common multiple of all its `m` is at most 9·10¹⁵.
