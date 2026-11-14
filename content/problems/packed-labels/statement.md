A sign workshop prints lettering on floor-marking tape for airports and warehouses, and some tapes run for kilometres. The printer could never store such a tape letter by letter, so it keeps each label **packed**: a sequence of runs, each written as a count followed by a lowercase letter. For example `3a2b10c` stands for `aaabbcccccccccc`. A run's count is always written, even when it is 1.

Before a print job, the operator checks single letters: "which letter sits at position `k` of label `i`?"

This is a **program problem**: read the labels and the checks from standard input and print the letters to standard output.

## Input format

- The first line holds `L`, the number of labels.
- Each of the next `L` lines holds one packed label. Labels are numbered from 1 in this order.
- The next line holds `Q`, the number of checks.
- Each of the next `Q` lines holds two numbers `i k`: label `i`, position `k`, counted from 1 in the unpacked label.

## Output format

Print `Q` lines, one per check in order: the letter at position `k` of the unpacked label `i`.

### Example 1

Input:

```
2
3a2b10c
1z
4
1 1
1 4
1 6
2 1
```

Output:

```
a
b
c
z
```

Label 1 unpacks to `aaabbcccccccccc`: positions 1-3 are `a`, 4-5 are `b` and 6-15 are `c`.

### Example 2

Input:

```
1
1000000000x5y999999999z
3
1 1000000000
1 1000000001
1 2000000004
```

Output:

```
x
y
z
```

This label is 2 000 000 004 letters long: far too long to unpack.

### Example 3

Input:

```
3
1a1b1c1d
2k1k3m
7q
3
1 3
2 4
3 7
```

Output:

```
c
m
q
```

Two runs in a row may use the same letter: `2k1k3m` unpacks to `kkkmmm`.

### Constraints

- 1 ≤ L ≤ 1000
- Every label has at least one run, and all the labels together have at most 60 000 runs.
- 1 ≤ count ≤ 10⁹, written without leading zeros; letters are lowercase `a`-`z`.
- 1 ≤ Q ≤ 20 000
- 1 ≤ i ≤ L, and 1 ≤ k ≤ the unpacked length of label `i`.
