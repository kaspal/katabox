## The idea

A **trie** (or prefix tree) stores words letter by letter. Each node stands for a prefix, and each edge out of it is one letter: from the root, follow `c`, then `a`, then `r`, and you reach the node for "car". Words that start the same way share the same path, so "car", "cart", "care" and "cat" all go through the nodes for "c" and "ca".

Each node here keeps two things:

- a **count**: how many inserted words pass through it, that is, how many start with its prefix;
- an **end mark**: whether some word ends exactly here.

### Insert and count

To **insert** a word, start at the root and follow one letter at a time. When an edge is missing, create a new node for it. Add 1 to the count of every node you enter, and mark the last one as an end.

To **count the words with a prefix**, walk the prefix the same way. If an edge is missing, no word starts with it: the answer is 0. Otherwise the node you reach already holds the answer in its count, and its end mark tells whether the prefix is itself a word.

### The default input

The words car, cart, care, cat and dog are inserted, then the prefix "ca" is counted. Inserting cart after car follows three existing edges and creates one node, for `t`. When all five are in, the trie has 9 nodes below the root, the node for "car" has count 3 (car, cart, care), and the node for "ca" has count 4. "ca" is not a word, so the output is `[4, 0]`.

## Steps

1. Start with an empty root node.
2. Insert each word: for each letter, create the child if it is missing, move to it and add 1 to its count.
3. After the last letter, mark the node as the end of a word.
4. Count a prefix: follow its letters from the root. If an edge is missing, output `[0, 0]`.
5. Otherwise output the node's count, and 1 if it is marked as an end, else 0.

## Cost

| Operation | Cost |
|---|---|
| insert a word of length `L` | `O(L)` |
| count a prefix of length `L` | `O(L)`, however many words are stored |
| Extra space | `O(total letters)` in the worst case; shared prefixes are stored once |

## Pitfalls

- **Counting instead of marking.** The count says how many words *pass through* a node; it does not say whether one ends there. Keep both.
- **Inserting a word twice.** It is counted twice. Check the end mark first if words must be unique.
- **Memory.** Each node holds a map of children, which is much heavier than one character.

## When to use it

Use a trie when questions are about prefixes: autocomplete, counting or listing the words that start with something, spell checking, or matching the longest known prefix (routers match IP address prefixes this way).

## Related topics

- [Hash table: separate chaining](learn:hashing/separate-chaining): faster for whole-word lookups, useless for prefixes.
- [Pre-, in- and post-order](learn:trees/tree-traversals): walking a trie in pre-order lists its words in sorted order when children are visited in letter order.
