A tide gauge at the harbour mouth has logged the water level `N` times, at regular intervals, in centimetres above or below the chart datum. Pilots who plan to bring a deep-hulled ship in study old stretches of the log: for readings `l` to `r`, how low did the water get? The log is finished and never changes, but the pilots ask a great many questions.

This is a **program problem**: read the log and the questions from standard input and print the answers to standard output.

## Input format

- The first line holds `N`, the number of readings.
- The second line holds the `N` levels, separated by spaces, in the order they were logged (reading 1 first).
- The third line holds `Q`, the number of questions.
- Then `Q` lines, each `l r`: the stretch from reading `l` to reading `r`, both included.

## Output format

Print `Q` lines, one per question in order: the lowest level among readings `l` to `r`.

### Example 1

Input:

```
6
12 -3 7 0 -8 5
4
1 3
4 4
4 6
1 6
```

Output:

```
-3
0
-8
-8
```

Readings 4 to 6 are 0, -8 and 5; the lowest is -8.

### Example 2

Input:

```
1
-1000000
1
1 1
```

Output:

```
-1000000
```

### Example 3

Input:

```
5
4 4 2 2 9
3
1 2
2 4
5 5
```

Output:

```
4
2
9
```

### Constraints

- 1 ≤ N ≤ 10⁵
- -10⁶ ≤ level ≤ 10⁶
- 1 ≤ Q ≤ 5·10⁴
- 1 ≤ l ≤ r ≤ N
