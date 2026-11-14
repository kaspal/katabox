## The idea

Comparing the pattern with every window of the text letter by letter costs up to `m` comparisons per window. **Rabin-Karp** compares numbers first: it hashes the pattern once, keeps a [rolling hash](learn:hashing/rolling-hash) of the current window, and looks at the letters only when the two hashes are equal.

The hash reads a word as a number in base 26, with `a = 0` up to `z = 25`, kept small with `mod 101`. The leading letter of an `m`-letter window weighs `high = 26^(m - 1) mod 101`. Moving one place to the right takes `O(1)`: subtract the leaving letter's code times `high`, multiply by 26, add the arriving letter's code, and take the remainder again.

Equal words always get equal hashes, but only 101 hashes exist, so different words can share one. Equal hashes mean "maybe": the letters decide. A real match is recorded; a false alarm is a **collision**, and the sample counts those separately so you can see them. The output is the number of matches, the number of collisions, then the match positions. The text and the pattern are typed as two words, letters a to z.

### The default input

Text `bcaxaavbcab`, pattern `bca`. The pattern's hash is `(1 · 26 + 2) · 26 + 0 = 728`, and `728 mod 101 = 21`. The 9 windows hash to 21, 62, 93, 95, 21, 42, 84, 21, 40.

Three windows hash to 21. At position 0 the letters are `bca`: a match. At position 4 the window is `aav`: `0 · 676 + 0 · 26 + 21 = 21` too, but the letters differ, so it is a collision. At position 7 the letters are `bca` again. The output is `[2, 1, 0, 7]`.

## Steps

1. If the pattern is longer than the text, return `[0, 0]`.
2. Compute `high`, the pattern's hash and the first window's hash.
3. For each window: if the hashes are equal, compare the letters and count a match or a collision.
4. If a letter follows the window, roll: `((window - code(leaving) · high) · 26 + code(arriving)) mod 101`.
5. Return matches, collisions, then the positions.

## Cost

| Case | Cost |
|---|---|
| Best | `O(n + m)`: hashes rarely agree |
| Average | `O(n + m)` expected, with a good modulus |
| Worst | `O(n · m)` when many windows collide or match, since each costs `m` comparisons |
| Extra space | `O(1)` besides the output |

## Pitfalls

- **Trusting the hash.** Skipping the letter check turns every collision into a wrong answer.
- **Negative remainders.** In TypeScript, `%` can be negative after the subtraction; the sample adds `MOD` before taking `%` again.
- **A tiny modulus.** 101 keeps the numbers readable here and makes collisions visible. A large prime modulus (around 10⁹) makes a false alarm on random text very unlikely, and **double hashing**, two hashes with different moduli that must both agree, makes it rarer still. Fewer collisions mean fewer wasted comparisons, so the expected time stays linear.

## When to use it

Use it when hashes of windows are useful in their own right: comparing many windows of the same length, searching several patterns of one length at once, or finding a chunk that appears twice. The linked problem, Repeated Chunk, looks for a repeated window this way.

## Related topics

- [Rolling hash](learn:hashing/rolling-hash): the window hash on its own.
- [Knuth-Morris-Pratt](learn:strings/kmp): linear search with no hashing and no bad cases.
- [Z-function](learn:strings/z-function): another linear search.
- [Sliding window](learn:searching/sliding-window): the same "drop one, add one" update.
