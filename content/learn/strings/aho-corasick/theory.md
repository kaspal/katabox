## The idea

To count several patterns in one text, running a single-pattern search once per pattern reads the text again and again. **Aho-Corasick** reads it once, for all of them at the same time.

It starts with a [trie](learn:trees/trie) of the patterns: each node is a prefix of some pattern, and a node where a pattern ends lists that pattern in its **output**. Walking down the trie letter by letter follows the text as long as some pattern continues that way.

When no child fits the next letter, the walk takes a **failure link**: from a node, it goes to the longest proper suffix of that node's prefix that is also in the trie. It is the same move as falling back to a border in [Knuth-Morris-Pratt](learn:strings/kmp), but across many patterns. Links are built **breadth-first**, so a node's link is always ready before its children need it: for a child reached by letter `ch`, follow its parent's links until a node has a `ch` child, and link there (or to the root). A node also inherits the output of the node its link points to, since a pattern that ends at that suffix ends here too.

The scan then moves one letter at a time: follow failure links while the current node has no child for the letter, step down if one exists, and add one to every pattern in the node's output. Overlapping matches are all counted. The output is one count per pattern, in the order the patterns were given. The text and the patterns are typed as words, letters a to z. The app draws the trie with the failure links dashed.

### The default input

Text `abcabcab`, patterns `ab`, `bc`, `abc` and `cab`. The node for `abc` links to `bc`, so its output is `abc` and `bc`; the node for `cab` links to `ab`, so its output is `cab` and `ab`. The scan reaches `ab`, then `abc`. At the next `a`, it falls from `abc` to `bc`, then to `c`, and steps down to `ca`, then `cab`. Then `c` falls to `ab` and steps to `abc` again; the last two letters reach `cab` once more. The output is `[3, 2, 2, 2]`.

## Steps

1. Insert every pattern into the trie and add it to its last node's output.
2. Visit nodes breadth-first; link each child to the longest suffix in the trie and merge that node's output into the child's.
3. For each letter of the text, follow failure links until the letter fits or the walk is at the root, then step down if possible.
4. Add one to each pattern in the current node's output.

## Cost

| Case | Cost |
|---|---|
| Every case | `O(n + total pattern length + matches)` |
| Extra space | `O(total pattern length)` for the trie and links |

As in KMP, the depth grows by at most one per letter and every failure link lowers it, so the falls add up to `O(n)`.

## Pitfalls

- **Forgetting to merge outputs.** Without it, a pattern hidden inside a longer one (here `bc` inside `abc`) is missed.
- **Building links depth-first.** A link needs the links of shorter prefixes; only breadth-first order guarantees them.
- **Resetting to the root after a match.** That loses overlaps; the walk simply continues.

## When to use it

Use it to look for a whole dictionary of words in a text at once: filtering banned words, scanning for known signatures, tagging keywords.

## Related topics

- [Trie](learn:trees/trie): the tree this automaton is built on.
- [Knuth-Morris-Pratt](learn:strings/kmp): the single-pattern version of failure links.
- [Breadth-first search](learn:graphs/bfs): the order the links are built in.
- [Rabin-Karp](learn:strings/rabin-karp): many patterns of one length, by hashing.
