A trail camera in a nature reserve takes `n` photos, and for each one the rangers log the id of the species in it. Later they study stretches of the log: how many different species appear in photos `l` to `r`? The log is finished and never changes, but the rangers ask a great many questions.

This is a **program problem**: read the log and the questions from standard input and print the answers to standard output.

## Input format

- The first line holds `n`, the number of photos.
- The second line holds the `n` species ids, separated by spaces, in the order the photos were taken (photo 1 first).
- The third line holds `Q`, the number of questions.
- Then `Q` lines, each `l r`: the stretch from photo `l` to photo `r`, both included.

## Output format

Print `Q` lines, one per question in order: the number of different species ids among photos `l` to `r`.

### Example 1

Input:

```
7
12 5 12 9 5 5 31
4
1 3
2 6
4 4
1 7
```

Output:

```
2
3
1
4
```

Photos 2 to 6 show species 5, 12, 9, 5 and 5: three different species.

### Example 2

Input:

```
1
1000000000
1
1 1
```

Output:

```
1
```

### Example 3

Input:

```
5
3 3 3 3 3
3
1 5
2 3
5 5
```

Output:

```
1
1
1
```

### Constraints

- 1 ≤ n ≤ 5·10⁴
- 1 ≤ id ≤ 10⁹, written without leading zeros
- 1 ≤ Q ≤ 5·10⁴
- 1 ≤ l ≤ r ≤ n
