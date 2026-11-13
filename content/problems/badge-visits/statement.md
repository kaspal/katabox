A science museum hands every member a badge with a numeric id. The entrance gate scans a badge each time someone walks in, and the same member may come back many times during the day. The day's scans are kept in order. The director keeps asking the same kind of question: "after the first `p` scans of the day, how many **different** members had come in?"

This is a **program problem**: read the scans and the questions from standard input and print the answers to standard output.

## Input format

- The first line holds `N`, the number of scans.
- The second line holds the `N` badge ids in the order they were scanned, separated by spaces.
- The third line holds `Q`, the number of questions.
- Each of the next `Q` lines holds one number `p`.

## Output format

Print `Q` lines, one per question in order: the number of distinct badge ids among the first `p` scans.

### Example 1

Input:

```
6
5 3 5 9 3 1
4
1
3
4
6
```

Output:

```
1
2
3
4
```

After 3 scans the gate has seen badges 5 and 3 (badge 5 twice), so the answer is 2.

### Example 2

Input:

```
4
7 7 7 7
3
4
2
4
```

Output:

```
1
1
1
```

### Example 3

Input:

```
5
1000000000 1 999999999 1 1000000000
5
5
2
3
1
5
```

Output:

```
3
2
3
1
3
```

The same question may be asked more than once, in any order.

### Constraints

- 1 ≤ N ≤ 30 000
- 1 ≤ badge id ≤ 10⁹
- 1 ≤ Q ≤ 30 000
- 1 ≤ p ≤ N
