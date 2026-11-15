The organiser of a puzzle hunt keeps a score balance for every team. Balances go up with solved puzzles and down with penalties, so they can be negative. Before the final round, two different teams may merge into one, but only if their combined balance is at most a quota `K`, so that merged teams do not start too far ahead. The organiser wants to know how many pairs of teams could merge. She runs several hunts and asks the same question for each one.

This is a **program problem**: read the test cases from standard input and print one count per case to standard output.

## Input format

- The first line holds `T`, the number of test cases (hunts).
- Then each case takes two lines:
  - a line with `n K`: the number of teams and the quota;
  - a line with the `n` balances, separated by spaces.

## Output format

Print `T` lines, one per case in order: the number of pairs of different teams whose balances add up to at most `K`. Each pair counts once, whatever its order, and two teams with the same balance are still different teams.

### Example 1

Input:

```
2
4 10
3 8 5 2
3 0
-4 4 1
```

Output:

```
4
2
```

In the first case the pairs that fit are 3 + 5, 3 + 2, 8 + 2 and 5 + 2; 3 + 8 and 8 + 5 go over 10. In the second, -4 + 4 = 0 and -4 + 1 fit, 4 + 1 does not.

### Example 2

Input:

```
1
1 5
7
```

Output:

```
0
```

A single team has nobody to merge with.

### Example 3

Input:

```
3
5 -2
-1 -1 -1 -1 -1
2 2000000000
1000000000 1000000000
3 -2000000000
-1000000000 -1000000000 -1000000000
```

Output:

```
10
1
3
```

### Constraints

- 1 ≤ T ≤ 10⁴
- 1 ≤ n, and the sum of `n` over all cases is at most 10⁵
- -2·10⁹ ≤ K ≤ 2·10⁹
- -10⁹ ≤ balance ≤ 10⁹
