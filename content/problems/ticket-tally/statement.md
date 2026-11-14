A ferry office keeps a ledger of the tickets it sells, one day after another. For every day, the manager wants three numbers: how many tickets were sold, the total income, and the price of the most expensive ticket.

This is a **program problem**: read the ledger from standard input and print the answers to standard output.

## Input format

The first line holds `T`, the number of days. Then each day takes two lines:

- a line with `n`, the number of tickets sold that day;
- a line with the `n` ticket prices, separated by spaces. When `n` is 0 this line is empty.

## Output format

Print `T` lines, one per day in order: `count total highest`, separated by single spaces. A day with no tickets prints `0 0 0`.

### Example 1

Input:

```
2
3
12 30 8
1
45
```

Output:

```
3 50 30
1 45 45
```

### Example 2

Input:

```
3
0

2
7 7
4
5 1 9 2
```

Output:

```
0 0 0
2 14 7
4 17 9
```

The first day sold nothing: its price line is empty.

### Example 3

Input:

```
1
5
10000 10000 10000 10000 10000
```

Output:

```
5 50000 10000
```

### Constraints

- 1 ≤ T ≤ 1000
- 0 ≤ n ≤ 1000
- 1 ≤ price ≤ 10 000
- At most 50 000 prices in the whole input.
