This time the catalogue changes while readers search. It starts empty, and each `ops[i]`, together with `words[i]`, is one of:

- `"add"`: a copy of the title `words[i]` joins the catalogue (copies count separately, as before);
- `"remove"`: one copy of `words[i]` leaves the catalogue (it is always there);
- `"count"`: a reader typed `words[i]`; report how many titles in the catalogue **right now** begin with it.

Return the answers to the `"count"` operations in order (an empty list if there are none). A title starts with itself.

### Example 1

```
ops = ["add", "add", "count", "add", "remove", "count", "count"], words = ["moby", "moon", "mo", "mob", "moby", "mob", "z"]
Output: [2, 1, 0]
```

Once "moby" is removed, only "mob" begins with "mob".

### Example 2

```
ops = ["add", "remove", "add", "add", "count", "count"], words = ["b", "b", "aa", "ba", "a", "a"]
Output: [1, 1]
```

### Example 3

```
ops = ["count", "add", "add", "add", "count", "count"], words = ["a", "b", "a", "b", "b", "a"]
Output: [0, 2, 1]
```

The first count sees an empty catalogue.

### Constraints

- 1 ≤ ops.length ≤ 3·10⁴
- words.length = ops.length
- Each `ops[i]` is `"add"`, `"remove"` or `"count"`
- 1 ≤ words[i].length ≤ 20, lowercase English letters only
- `"remove"` only names a title that is in the catalogue at that moment
