## The bug

The loop deletes from the same list it is iterating over:

```python
for x in items:
    if x in bad:
        items.remove(x)
```

A `for` loop over a list walks it by position. When an element is removed, every later element moves one position to the left, and the loop then advances to the next position, so the element that just slid into the current slot is never looked at. Two banned values in a row are the classic victim. With `items = [7, 7]` and `banned = [7]`, the loop removes the first 7 (the list becomes `[7]`), moves on to position 1, finds nothing there, and returns `[7]` instead of `[]`. (`remove` also deletes the first matching value rather than the current one, which hides the problem further.)

## The fix

Leave `items` alone and collect the allowed values into a new list:

```python
def without_banned(items: list[int], banned: list[int]) -> list[int]:
    bad = set(banned)
    return [x for x in items if x not in bad]
```

The TypeScript starter has the same flaw with `splice` inside an index loop; return `items.filter((x) => !bad.has(x))` instead.

**Complexity:** O(n + m) time and O(n + m) extra space, for n items and m banned values. (Removing in place with `remove` or `splice` would also cost O(n) per removal.)
