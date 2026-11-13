A mountain rescue team still carries an old field radio for places with no phone signal. It sends each letter as its number in the alphabet, `A` = 1, `B` = 2, …, `Z` = 26, but it sends no separators, so a received message is just a run of digits and can often be read in several ways. For example `25` could be `BE` (2, 5) or `Y` (25).

The receiver logs each message on its own line as a `#` sign followed by the digits. Before decoding, the team wants to know how ambiguous each message is: in how many ways can it be split into numbers from 1 to 26? A piece never has a leading zero, so `0` on its own and `05` are not letters, while `10` and `20` are.

This is a **program problem**: read the log from standard input and print the counts to standard output.

## Input format

- The first line holds `T`, the number of messages.
- Each of the next `T` lines holds one message: a `#` followed by its digits.

## Output format

Print `T` lines, one per message in order: the number of ways to read it as a sequence of letters, **modulo 1 000 003**. A message that cannot be read at all prints `0`.

### Example 1

Input:

```
3
#118
#2025
#9
```

Output:

```
3
2
1
```

`118` reads as 1 1 8, 11 8 or 1 18, and `2025` as 20 2 5 or 20 25.

### Example 2

Input:

```
4
#10
#2101
#30
#0
```

Output:

```
1
1
0
0
```

`2101` only reads as 2 10 1: the `0` must join the `1` before it. In `30` the `0` cannot join the `3`, since 30 is not a letter.

### Example 3

Input:

```
2
#111111111111111111111111111111
#2626
```

Output:

```
346266
4
```

The thirty 1s can be read in 1 346 269 ways, which is 346 266 modulo 1 000 003.

### Constraints

- 1 ≤ T ≤ 1000
- Each message has between 1 and 10⁵ digits (`0`-`9`).
- At most 2·10⁵ digits in all the messages together.
