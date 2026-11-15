A company keeps its chart as a list of bosses. Employee `1` is the director and has no boss; every other employee has exactly one boss, and following bosses upwards always ends at the director. Employee `a` is **above** employee `b` when `b` reports to `a`, either directly or through a chain of bosses. Nobody is above themselves.

The office receives many questions of the form "is `a` above `b`?". Answer them all.

This is a **program problem**: read the chart and the questions from standard input and print the answers to standard output.

## Input format

- The first line holds `n`, the number of employees, numbered `1` to `n`.
- The second line holds `n − 1` numbers, separated by spaces: the bosses of employees `2`, `3`, …, `n`, in that order. A boss may have a larger number than the employee. When `n` is 1 this line is empty.
- The third line holds `Q`, the number of questions.
- Then `Q` lines, each `a b`: is `a` above `b`?

## Output format

Print `Q` lines, one per question in order: `YES` if `a` is above `b`, otherwise `NO`. When `a` equals `b` the answer is `NO`.

### Example 1

Input:

```
6
1 1 2 2 4
5
1 6
2 6
3 6
6 2
4 4
```

Output:

```
YES
YES
NO
NO
NO
```

Employee `6` reports to `4`, who reports to `2`, who reports to the director `1`: so `1` and `2` are above `6`, but `3` is not. `6` is below `2`, not above, and `4` is not above themselves.

### Example 2

Input:

```
1

1
1 1
```

Output:

```
NO
```

The director works alone, so the line of bosses is empty.

### Example 3

Input:

```
5
4 1 5 3
4
3 2
4 2
2 3
1 5
```

Output:

```
YES
YES
NO
YES
```

The chart is a single line: `2` reports to `4`, `4` to `5`, `5` to `3` and `3` to `1`.

### Constraints

- 1 ≤ n ≤ 10⁵
- 1 ≤ boss ≤ n, and nobody is their own boss
- Everyone reports to employee `1` in the end: the chart has no loops
- 1 ≤ Q ≤ 10⁵
- 1 ≤ a, b ≤ n
