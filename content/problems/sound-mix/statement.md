A field recorder left in a forest notes, every second, which sound source was the loudest: a woodpecker, a stream, a passing lorry, each with its own id number. A stretch of consecutive seconds is a **K-mix** when exactly `K` different sources appear in it. The sound designer wants to know how many K-mix stretches the recording holds.

This is a **program problem**: read the recording from standard input and print the count to standard output.

## Input format

- The first line holds `n K`: the number of seconds recorded and the number of different sources a K-mix must have.
- The second line holds the `n` source ids, separated by spaces, one per second in order.

## Output format

Print one line: the number of stretches (from second `i` to second `j`, with `i ≤ j`) that contain exactly `K` different ids.

### Example 1

Input:

```
6 2
4 4 7 4 9 9
```

Output:

```
7
```

The 2-mixes are `4 4 7`, `4 4 7 4`, `4 7`, `4 7 4`, `7 4`, `4 9` and `4 9 9`. The stretch `7 4 9` is not one: it has three sources.

### Example 2

Input:

```
3 3
1000000000 5 1000000000
```

Output:

```
0
```

Only two different sources appear in the whole recording, so no stretch has three.

### Example 3

Input:

```
5 1
8 8 8 3 3
```

Output:

```
9
```

### Constraints

- 1 ≤ K ≤ n ≤ 10⁵
- 1 ≤ id ≤ 10⁹, written without leading zeros
