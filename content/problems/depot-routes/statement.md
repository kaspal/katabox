A city's ambulances all wait at one base, next to junction 1. The street map has `N` junctions, numbered 1 to `N`, linked by `M` two-way streets, each with the seconds an ambulance needs to drive it. The emergency line rings all night, and for every call the dispatcher needs one number at once: how many seconds will the ambulance from the base need to reach the caller's junction?

This is a **program problem**: read the street map and the calls from standard input and print the answers to standard output.

## Input format

- The first line holds `N` and `M`.
- Then `M` lines, each `a b w`: a street between junctions `a` and `b` that takes `w` seconds in either direction. Two junctions may be linked by more than one street.
- Then a line with `Q`, the number of calls.
- Then `Q` lines, each with the junction `t` of one call.

## Output format

Print `Q` lines, one per call in order: the shortest driving time in seconds from junction 1 to junction `t`, or `-1` when no street route reaches `t`. The base's own junction is 0 seconds away.

### Example 1

Input:

```
4 4
1 2 5
2 3 2
1 3 9
3 4 1
3
3
4
2
```

Output:

```
7
8
5
```

Junction 3 is 7 seconds away through junction 2 (5 + 2), faster than the direct 9-second street.

### Example 2

Input:

```
5 2
1 2 3
4 5 1
3
5
1
2
```

Output:

```
-1
0
3
```

Junctions 4 and 5 are only linked to each other, so the ambulance cannot reach them.

### Example 3

Input:

```
3 3
2 1 10
1 2 4
2 3 6
2
3
3
```

Output:

```
10
10
```

Two streets link junctions 1 and 2; the faster one counts. Several calls can come from the same junction.

### Constraints

- 1 ≤ N ≤ 10⁴
- 0 ≤ M ≤ 2·10⁴
- 1 ≤ a, b ≤ N, a ≠ b
- 1 ≤ w ≤ 10⁴
- 1 ≤ Q ≤ 10⁴
- 1 ≤ t ≤ N
