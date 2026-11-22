## The bug

The running maximum is initialized in the wrong place:

```python
for w in words:
    best = 0
```

Because `best = 0` runs at the start of every iteration, each word wipes out what the previous words found, and the function ends up returning the vowel count of the **last** word only. With `words = ["audio", "sky"]` it returns `0` (the vowels of "sky") instead of `4`. In Example 1 it returns `2`, the count for "tree".

The TypeScript starter has the same mistake in another form: `let best = 0;` inside the loop declares a new variable that shadows the outer `best`, so the outer one stays `0` and the function always returns `0`.

## The fix

Initialize `best` once, before the loop:

```python
def most_vowels(words: list[str]) -> int:
    best = 0
    for w in words:
        count = sum(1 for c in w if c in "aeiou")
        best = max(best, count)
    return best
```

In TypeScript, delete the inner `let best = 0;` so the loop updates the outer `best`.

**Complexity:** O(L) time, where L is the total number of letters, and O(1) extra space.
