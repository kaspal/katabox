A warehouse receives mixed crates of fruit. Each crate carries a label listing what is inside, such as `apples:12;pears:3`: product names with their quantities, separated by semicolons. The stock keeper wants one total per product across all the crates.

This is a **program problem**: read the labels from standard input and print the totals to standard output.

## Input format

- The first line holds `N`, the number of crates.
- Then `N` lines, one label each. A label is one or more entries `name:quantity` separated by `;`, with no spaces. The same product may appear on many labels, and even twice on one label.

## Output format

Print one line per product that appears on any label: `name total`, separated by a single space, where `total` is the sum of all its quantities. List the products in alphabetical order, comparing letter by letter; a name that is the beginning of another comes first (`fig` before `figs`).

### Example 1

Input:

```
3
apples:12;pears:3
pears:5;apples:1;figs:40
kiwis:7
```

Output:

```
apples 13
figs 40
kiwis 7
pears 8
```

Apples come to 12 + 1 = 13 and pears to 3 + 5 = 8.

### Example 2

Input:

```
1
plums:1000
```

Output:

```
plums 1000
```

### Example 3

Input:

```
2
fig:2;figs:3;fig:4
figs:1
```

Output:

```
fig 6
figs 4
```

`fig` appears twice on the first label: 2 + 4 = 6.

### Constraints

- 1 ≤ N ≤ 1000
- A label has 1 to 20 entries
- A name is 1 to 10 lowercase letters (`a`-`z`)
- 1 ≤ quantity ≤ 1000
