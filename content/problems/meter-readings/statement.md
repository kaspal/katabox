A mountain hut gets its water from a spring, and a small flow meter on the pipe logs a reading every minute through the whole season. When the hut closes, the keeper downloads the log and wants three numbers from it: the sum of all the readings, how many times a reading was higher than the one just before it, and the length of the longest steady stretch, where the same reading repeats minute after minute.

This is a **program problem**: read the log from standard input and print the three numbers to standard output. The log can be long, so read it efficiently.

## Input format

The first line holds `N`, the number of readings. Each of the next `N` lines holds one reading, a whole number.

## Output format

Print one line with three numbers separated by spaces:

1. the sum of all the readings;
2. the number of readings strictly greater than the reading just before them;
3. the length of the longest run of equal consecutive readings (a single reading is a run of length 1).

### Example 1

Input:

```
6
5
7
7
7
3
8
```

Output:

```
37 2 3
```

The readings rise twice (5 to 7 and 3 to 8), and 7 repeats three times in a row.

### Example 2

Input:

```
1
1000000
```

Output:

```
1000000 0 1
```

### Example 3

Input:

```
5
4
3
2
1
0
```

Output:

```
10 0 1
```

### Example 4

Input:

```
4
0
0
1
1
```

Output:

```
2 1 2
```

### Constraints

- 1 ≤ N ≤ 6·10⁴
- 0 ≤ reading ≤ 10⁶
