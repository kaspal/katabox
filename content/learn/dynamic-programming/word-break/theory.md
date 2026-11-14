## The idea

Given a text with no spaces and a list of dictionary words, can the text be cut into pieces that are **all** dictionary words? Words may be used any number of times. Trying every way to cut the text doubles the work with each letter, and grabbing the longest word that fits can walk into a dead end.

The table asks a yes-or-no question about every **prefix**: `ok[end]` is true when the first `end` letters can be split into words. `ok[0]` is true: the empty prefix needs no words. For each `end` from 1 to `n`, the sample tries the words **in the order given**. A word fits if it would start at `start = end - len(word) ≥ 0`, the prefix before it is splittable (`ok[start]`), and the letters from `start` to `end` spell the word. The **first** word that fits sets `ok[end]` to true and stops the search for that `end`. The output is `[1]` if `ok[n]` is true, else `[0]`.

The input is typed as the text, then the words, each in its own field, letters a to z.

### The default input

Text `sunflowerpot`, words `sun`, `flow`, `flower`, `pot`, `er`. `sun` makes `ok[3]` true. `flow` starts at 3, so `ok[7]` is true. At `end = 9`, `flower` (also from 3) fits before `er` is tried; `er` from 7 would have worked too. `pot` starts at 9, so `ok[12]` is true. The true cells are 0, 3, 7, 9 and 12, and the output is `[1]`.

### A dead end

The "dead end" preset is `goldfishy` with `gold`, `fish`, `goldfish`, `shy`. `ok[4]` is true (`gold`), and `ok[8]` is true (`fish` from 4, tried before `goldfish`). `shy` does fit at the end, but it starts at 6, and `ok[6]` is false. Every split runs out at the lone `y`: the output is `[0]`.

A greedy splitter that grabs the longest word takes `goldfish` and gets stuck on that same `y`. Here "no" happens to be right, but greedy cannot tell a real dead end from its own bad choice. With the words `fish`, `fishy`, `yard` and the text `fishyard`, it takes `fishy`, is left with `ard`, and says no, while `fish` + `yard` works. The table never commits: it keeps every reachable cut.

## Steps

1. Set `ok[0]` to true and the rest to false.
2. For each `end` from 1 to `n`, and each word in order: if `ok[end - len(word)]` is true and the text from there to `end` is the word, set `ok[end]` and stop.
3. Return `[1]` if `ok[n]`, else `[0]`.

## Cost

| Case | Cost |
|---|---|
| Every case | `O(n · words · L)`: each end tries each word, comparing up to `L` letters |
| Extra space | `O(n)` for `ok` |

## Pitfalls

- **Committing to one split.** Greedy cutting, longest or shortest first, can block a later word.
- **Reading before the start.** Check `start ≥ 0` before looking at `ok[start]`.
- **Expecting the pieces.** This table only says yes or no; to list a split, also store which word set each `ok[end]`.

## When to use it

Use it to segment text without spaces, check whether a code is made of known parts, or match a string against a set of tokens.

## Related topics

- [Coin change](learn:dynamic-programming/coin-change): reaching an exact total from reusable pieces.
- [Trie](learn:trees/trie): finding all words that start at a position in one walk.
- [Climbing stairs](learn:dynamic-programming/climbing-stairs): each cell reached from earlier cells by fixed jumps.
- [Jump game](learn:greedy/jump-game): a reachability question where greedy is safe.
