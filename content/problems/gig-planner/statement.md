A musician has received `n` gig offers for the season. Each offer has a start time, an end time and a pay. She can play only one gig at a time, so the gigs she accepts may not overlap; one gig may start at the very moment another ends, though, since the venue next door is a short walk away.

She wants the largest total pay. Among all the plans that reach it, she prefers the one with the fewest gigs, to save her voice.

This is a **program problem**: read the offers from standard input and print the best plan's total pay and number of gigs to standard output.

## Input format

- The first line holds `n`, the number of offers.
- Then `n` lines, each `start end pay`: the offer runs from time `start` to time `end` and pays `pay`. The offers are in no particular order.

Two offers overlap when one starts before the other ends. An offer that starts exactly at another's `end` does not overlap it.

## Output format

Print one line with two numbers separated by a space: the largest total pay, then the fewest gigs that reach that total.

### Example 1

Input:

```
4
1 4 50
4 6 30
1 6 80
6 9 20
```

Output:

```
100 2
```

Two plans pay 100: `50 + 30 + 20` with three back-to-back gigs, and `80 + 20` with two. The second one has fewer gigs.

### Example 2

Input:

```
1
0 1000000000 1000000000
```

Output:

```
1000000000 1
```

### Example 3

Input:

```
3
0 10 5
2 3 5
5 8 5
```

Output:

```
10 2
```

The long gig overlaps both short ones; the two short ones fit together and pay 10.

### Constraints

- 1 ≤ n ≤ 10⁵
- 0 ≤ start < end ≤ 10⁹
- 1 ≤ pay ≤ 10⁹
