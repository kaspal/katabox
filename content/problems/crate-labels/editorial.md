## Approach: split, add up in a map, sort the names

**Parsing.** Labels contain no spaces, so the whole input split on whitespace is `N` followed by the `N` labels, one token each. Take each label apart in two steps:

```
"pears:5;apples:1;figs:40"
  split on ";"  →  "pears:5", "apples:1", "figs:40"
  split on ":"  →  ("pears", 5), ("apples", 1), ("figs", 40)
```

**Adding up.** Keep a map `total` from name to quantity. For each entry, `total[name] = total.get(name, 0) + quantity`. A name repeated on the same label is simply added twice, like a name repeated on different labels.

**Ordering.** Sort the map's names and print `name total` for each. The names are lowercase letters only, so the plain string comparison of every language is the alphabetical order the statement asks for: it compares letter by letter, and when one name runs out first (`fig` against `figs`), the shorter one comes first.

**Why it is correct:** every entry of every label adds its quantity to exactly one total, the one of its name, so each total is the sum of all that product's quantities. Each product appears once in the map, so it is printed once, and the sort puts the lines in the required order.

**Complexity:** at most 1000 · 20 = 2·10⁴ entries, each O(1) on average in the map, then O(P log P) to sort the P distinct names: O(E + P log P) in all, for E entries. The largest total is 2·10⁴ · 1000 = 2·10⁷, which fits any integer type.

**Language traps:**

- PHP: `explode(";", $label)` and `explode(":", $entry)`. Build the totals in an array keyed by name and sort with `ksort($total, SORT_STRING)`, which says plainly that the keys are compared as strings.
- TypeScript: a `Map<string, number>`; `[...total.keys()].sort()` with no comparator is the right order here (it compares by character code).
- Rust: a `HashMap<&str, u32>` sorted afterwards, or a `BTreeMap`, which keeps the names in order as you insert them.

**Fast input and output:** read all of stdin once and write all the lines in one go.
