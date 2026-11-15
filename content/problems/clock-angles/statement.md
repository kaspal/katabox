A clockmaker is testing a new analogue clock face: an ordinary 12-hour dial with an hour hand and a minute hand. For a list of times of day, written on the 24-hour clock, she wants to know the angle between the two hands, to check the hands against a drawing.

The minute hand turns 6 degrees every minute. The hour hand moves smoothly too: 30 degrees every hour, which is half a degree every minute, so at 09:45 it stands three quarters of the way from 9 to 10. The angle wanted is the smaller one between the hands, from 0 to 180 degrees.

This is a **program problem**: read the times from standard input and print the angles to standard output.

## Input format

- The first line holds `Q`, the number of times.
- Then `Q` lines, each a time `hh:mm` on the 24-hour clock, with two digits for the hours and two for the minutes.

## Output format

Print `Q` lines, one per time in order: the smaller angle between the hands, in degrees, with one digit after the decimal point.

### Example 1

Input:

```
4
03:00
09:45
12:00
06:30
```

Output:

```
90.0
22.5
0.0
15.0
```

At 09:45 the minute hand points at the 9, 270 degrees from the 12. The hour hand is at 9 · 30 + 45 · 0.5 = 292.5 degrees, so the hands are 22.5 degrees apart.

### Example 2

Input:

```
2
18:00
00:59
```

Output:

```
180.0
35.5
```

At 18:00 the dial shows 6 o'clock, and the hands point in opposite directions.

### Example 3

Input:

```
3
23:59
01:05
14:20
```

Output:

```
5.5
2.5
50.0
```

### Constraints

- 1 ≤ Q ≤ 1440
- 00 ≤ `hh` ≤ 23
- 00 ≤ `mm` ≤ 59
