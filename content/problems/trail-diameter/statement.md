A nature park is planning its yearly endurance walk. The park's junctions are numbered from 1 to `N`, and `N − 1` two-way trails connect them so that there is **exactly one route** between any two junctions (the trails never form a loop). The organisers want the walk to follow the longest such route, so they ask: what is the largest number of trails on the route between two junctions?

This is a **program problem**: read the park maps from standard input and print the answers to standard output.

## Input format

- The first line holds `T`, the number of parks.
- Each park starts with a line holding `N`, the number of junctions, followed by `N − 1` lines `a b`: a trail between junctions `a` and `b`. A park with one junction has no trail lines.

## Output format

Print `T` lines, one per park in order: the number of trails on the longest route between two of its junctions.

### Example 1

Input:

```
2
5
1 2
2 3
3 4
2 5
1
```

Output:

```
3
0
```

In the first park, the routes 1-2-3-4 and 5-2-3-4 both use 3 trails. The second park is a single junction.

### Example 2

Input:

```
1
6
4 1
4 2
4 3
4 5
4 6
```

Output:

```
2
```

Every trail meets junction 4, so the longest route goes from one end, through 4, to another end.

### Example 3

Input:

```
1
7
1 2
1 3
2 4
2 5
3 6
6 7
```

Output:

```
5
```

The longest route is 4-2-1-3-6-7 (or 5-2-1-3-6-7).

### Constraints

- 1 ≤ T ≤ 1000
- 1 ≤ N ≤ 20 000, and the `N` of all the parks add up to at most 20 000.
- 1 ≤ a, b ≤ N and a ≠ b; the trails of each park connect all its junctions without loops.
