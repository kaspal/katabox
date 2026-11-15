A freelance translator is offered `n` small jobs. Each job takes exactly one day, has a deadline (it must be done on that day or earlier) and pays a fee. Days are numbered 1, 2, 3, … from today, and she does at most one job per day. She may turn any job down.

This is a **program problem**: read the offers from standard input and print the largest total fee she can earn to standard output.

## Input format

- The first line holds `n`, the number of jobs.
- Then `n` lines, each `deadline fee`: the last day on which the job may be done, and what it pays.

## Output format

Print one line: the largest total fee of a set of jobs that can all be done by their deadlines, one per day.

### Example 1

Input:

```
5
2 50
1 10
2 40
3 20
1 30
```

Output:

```
110
```

Do the 40 job on day 1, the 50 job on day 2 and the 20 job on day 3. Days 1 and 2 can hold only two of the four jobs due by day 2, and 50 and 40 are the best two.

### Example 2

Input:

```
1
1000000000 7
```

Output:

```
7
```

### Example 3

Input:

```
4
1 5
1 9
1 2
1 9
```

Output:

```
9
```

Every job is due on day 1, so only one can be done.

### Constraints

- 1 ≤ n ≤ 10⁵
- 1 ≤ deadline ≤ 10⁹
- 1 ≤ fee ≤ 10⁹
