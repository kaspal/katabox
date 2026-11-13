A language school runs a placement exam for all its new students. To keep things friendly, it does not rank students one by one: it ranks the **scores**. The best score anyone got is rank 1, the next lower score anyone got is rank 2, and so on; students with equal scores share the same rank.

Students then ask at the front desk: "what rank is a score of `x`?" If nobody got exactly `x`, the answer is `0`.

This is a **program problem**: read the scores and the questions from standard input and print the ranks to standard output.

## Input format

- The first line holds `N`, the number of students.
- The second line holds the `N` scores, separated by spaces.
- The third line holds `Q`, the number of questions.
- The fourth line holds the `Q` asked scores `x`, separated by spaces.

## Output format

Print **one line** with `Q` numbers separated by single spaces: for each asked score in order, its rank among the distinct scores (1 = the highest), or `0` if no student got exactly that score.

### Example 1

Input:

```
6
70 85 70 100 60 85
5
85 70 100 90 60
```

Output:

```
2 3 1 0 4
```

The distinct scores, highest first, are 100, 85, 70 and 60. Nobody got 90.

### Example 2

Input:

```
3
0 0 0
3
0 1 1000000000
```

Output:

```
1 0 0
```

### Example 3

Input:

```
5
5 4 3 2 1
4
1 5 3 6
```

Output:

```
5 1 3 0
```

### Constraints

- 1 ≤ N ≤ 20 000
- 0 ≤ score ≤ 10⁹
- 1 ≤ Q ≤ 20 000
- 0 ≤ x ≤ 10⁹
