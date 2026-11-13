## Approach: sort by a composite key

Pair every country with its medals and sort with a comparison that looks at the parts in order:

1. more gold first;
2. if equal, more silver first;
3. if equal, more bronze first;
4. if equal, the name in alphabetical order.

With a key-based sort, the key `(−gold, −silver, −bronze, name)` does all four steps at once: tuples compare element by element, and negating the counts turns "more first" into ascending order.

The names are lowercase letters only, so plain character-by-character comparison is alphabetical order (no locale rules needed).

**Complexity:** O(n log n) comparisons.
