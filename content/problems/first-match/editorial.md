## Approach: a trie that remembers its smallest word

In a trie, the words starting with a prefix are exactly those whose path passes through the prefix's node. So the answer for a prefix only depends on that node: store in each node the **smallest word** that passes through it.

```
for word in sorted(words):          # in alphabetical order
    node = root
    for c in word:
        node = child(node, c)       # created if missing
        if node.best is None: node.best = word   # the first to arrive is the smallest
answer(prefix):
    walk down prefix; return node.best, or "" if a letter is missing
```

Inserting in sorted order means the first word to reach a node is the alphabetically smallest of all words through it. (Without sorting, compare each word with the stored one instead.) A shorter word sorts before the longer words it starts, which matches the statement.

### Walkthrough

Sorted: apple, peach, pear, plum. "peach" reaches the nodes p, pe, pea and peac first, so "pe", "p" and "peac" all answer "peach"; "a" answers "apple"; "b" has no node, so "".

**Complexity:** O(n log n · L) for the sort, O(total letters) to build the trie and O(L) per query. Scanning every word for every prefix is O(n · q · L).
