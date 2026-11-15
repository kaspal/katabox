An expedition depot sells `n` ration packs, each at its own price. Before setting off, each of `Q` teams arrives with a budget and wants to take as many packs as possible. A team can buy each pack at most once. Every team's question is separate: the depot's stock is the same for every team, whatever the others bought.

This is a **program problem**: read the prices and the budgets from standard input and print the answers to standard output.

## Input format

- The first line holds `n Q`: the number of ration packs and the number of teams.
- The second line holds the `n` prices, separated by spaces.
- Then `Q` lines, each holding one team's budget.

## Output format

Print `Q` lines, one per team in order: the largest number of packs whose prices add up to at most that team's budget.

### Example 1

Input:

```
5 4
7 3 12 3 5
10
2
30
18
```

Output:

```
2
0
5
4
```

With a budget of 10 a team can buy the two packs at 3 (total 6); adding the pack at 5 would cost 11. A budget of 18 buys the packs at 3, 3, 5 and 7, exactly 18.

### Example 2

Input:

```
1 2
1000000000
999999999
1000000000
```

Output:

```
0
1
```

### Example 3

Input:

```
4 3
2 2 2 2
0
7
8
```

Output:

```
0
3
4
```

### Constraints

- 1 ≤ n ≤ 5·10⁴
- 1 ≤ Q ≤ 5·10⁴
- 1 ≤ price ≤ 10⁹
- 0 ≤ budget ≤ 5·10¹³
