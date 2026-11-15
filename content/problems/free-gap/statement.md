A small company shares one meeting room. For each day, the booking sheet lists when the room opens, when it closes and the slots people have booked. The slots were written down as they came in: they are not in order, and some of them overlap. A team wants the longest stretch of free time in each day.

This is a **program problem**: read the booking sheet from standard input and print the longest free stretch of each day to standard output.

## Input format

The first line holds `T`, the number of days. Then, for each day:

- a line `open close k`: the opening and closing times of the room and the number of booked slots;
- then `k` lines, each `start end`: one booked slot.

Every time is written `hh:mm` on a 24-hour clock (two digits each). A slot `start end` keeps the room busy from `start` up to `end`: the minute `start` is busy, the minute `end` is free again. So a slot ending at `10:00` and another starting at `10:00` leave no gap between them.

## Output format

Print `T` lines, one per day in order. Each line is the longest free stretch between `open` and `close`, written `hh:mm-hh:mm` (the first free minute, then the time it ends). If several stretches are equally long, print the earliest one. If the room has no free minute that day, print `BOOKED`.

### Example 1

Input:

```
2
09:00 17:00 3
10:00 11:30
13:00 14:00
10:30 12:00
08:00 12:00 0
```

Output:

```
14:00-17:00
08:00-12:00
```

On the first day the slots `10:00 11:30` and `10:30 12:00` overlap, so the room is busy from 10:00 to 12:00 and from 13:00 to 14:00. The free stretches are 09:00-10:00, 12:00-13:00 and 14:00-17:00, and the last is the longest. The second day has no bookings, so the whole day is free.

### Example 2

Input:

```
1
09:00 10:00 2
09:00 09:40
09:30 10:00
```

Output:

```
BOOKED
```

### Example 3

Input:

```
1
07:15 09:45 2
07:45 08:15
08:45 09:15
```

Output:

```
07:15-07:45
```

The three free stretches are 30 minutes each; the earliest one wins.

### Constraints

- 1 ≤ T ≤ 100
- Times are `hh:mm`, from `00:00` to `23:59`
- open < close
- 0 ≤ k ≤ 1000
- open ≤ start < end ≤ close
- Σk ≤ 2·10⁴ (all days together)
