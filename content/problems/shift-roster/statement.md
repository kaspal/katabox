A neighbourhood food bank runs on volunteers. At the end of every month the coordinator copies the shift roster into a short report: the total number of hours the warehouse was staffed, followed by the volunteers who put in more time than the average, so they can be thanked in the newsletter.

This is a **program problem**: read the roster from standard input and print the report to standard output.

## Input format

The first line holds `N`, the number of volunteers. Each of the next `N` lines holds a volunteer's name and the hours they worked this month, separated by one space: `name hours`.

## Output format

- The first line is the total of all the hours.
- Then, in the order of the input, one line `name hours` for every volunteer whose hours are **strictly greater** than the average (the total divided by `N`).
- If no volunteer is above the average, the second line is the single word `none`.

Lines are compared exactly, apart from trailing spaces: use one space between the name and the hours, and print no blank lines.

### Example 1

Input:

```
4
ana 8
bruno 6
chen 10
dora 4
```

Output:

```
28
ana 8
chen 10
```

The average is 28 / 4 = 7.

### Example 2

Input:

```
3
eli 5
fay 5
gus 5
```

Output:

```
15
none
```

Everyone worked exactly the average, so nobody is above it.

### Example 3

Input:

```
5
ivo 7
jun 10
kai 0
lea 8
mo 24
```

Output:

```
49
jun 10
mo 24
```

The average is 9.8: it does not have to be a whole number.

### Example 4

Input:

```
1
hal 0
```

Output:

```
0
none
```

### Constraints

- 1 ≤ N ≤ 2·10⁴
- Each name has 1 to 12 lowercase letters (`a`-`z`); the names are distinct.
- 0 ≤ hours ≤ 24
