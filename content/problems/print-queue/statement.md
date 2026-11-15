An office has a single printer and a long list of print jobs. Jobs are numbered 1, 2, 3, … in the order they appear in the list; each one arrives at some second, has a priority and a number of pages, and the printer prints one page per second. Once a job starts, it prints to the end without interruption.

Whenever the printer is free, it picks among the jobs that have arrived and are still waiting: the one with the **highest priority**; among those, the one that **arrived earliest**; among those, the one with the **smallest number**. A job that arrives at the very second the printer becomes free is already waiting. If nothing is waiting, the printer stays idle until the next job arrives.

This is a **program problem**: read the jobs from standard input and print the order in which they are printed to standard output.

## Input format

- The first line holds `n`, the number of jobs.
- Then `n` lines, each `arrival priority pages`: the job's arrival second, its priority (larger is more urgent) and its number of pages. Line `i` describes job `i`. The lines are **not** sorted by arrival.

## Output format

Print one line with the `n` job numbers in the order they are printed, separated by spaces.

### Example 1

Input:

```
4
0 2 3
1 5 2
1 9 1
2 5 2
```

Output:

```
1 3 2 4
```

At second 0 only job 1 has arrived, so it prints from 0 to 3. By then jobs 2, 3 and 4 are waiting, and job 3 has the highest priority (9); it prints from 3 to 4. Jobs 2 and 4 both have priority 5, and job 2 arrived first.

### Example 2

Input:

```
2
10 1 5
0 1 5
```

Output:

```
2 1
```

Job 2 arrives first and prints from 0 to 5; the printer then waits until job 1 arrives at second 10.

### Example 3

Input:

```
3
5 7 2
5 7 2
5 7 2
```

Output:

```
1 2 3
```

### Constraints

- 1 ≤ n ≤ 5·10⁴
- 0 ≤ arrival ≤ 10⁹
- 1 ≤ priority ≤ 10⁹
- 1 ≤ pages ≤ 10⁴
