## The idea

To compare every window of `length` letters in a text, hashing each window from scratch costs `O(length)` per window. A **rolling hash** gets the next window's hash from the previous one in `O(1)`: take out the letter that leaves, shift, and add the letter that arrives.

The trick is a **polynomial hash**. Read the window as a number in base 26, with letters as digits (`a = 0`, `b = 1`, ..., `z = 25`), and keep it small with `mod 101`. For the window `abc`:

`h = (0 · 26² + 1 · 26 + 2) mod 101 = 28`

Each letter's **weight** depends on its place: the leading letter of a 3-letter window is worth `26² mod 101 = 70`.

### Rolling one step

To move from `abc` to `bca`:

1. **remove** the leading `a`: subtract its code times its weight, `0 · 70`. h stays 28.
2. **shift**: multiply by 26, so every remaining letter moves up one place.
3. **add** the new `a`: add its code. `h = (28 · 26 + 0) mod 101 = 21`.

Whatever `length` is, that is a constant number of operations. The default text `abcabcab` with `length` 3 has 6 windows, and their hashes are 28, 21, 40, 28, 21, 40: equal windows always get equal hashes.

### Collisions

The converse is not true. There are only 101 possible hashes but far more possible windows, so two different windows can share a hash. Equal hashes mean "maybe equal": compare the letters to be sure. Real code uses a large prime modulus (often around 10⁹) to make collisions rare. [Rabin–Karp](learn:strings/rabin-karp) builds a full substring search on this.

## Steps

1. Compute `power = 26^(length−1) mod 101`, the weight of a window's leading letter.
2. Hash the first window letter by letter: `h = (h · 26 + code) mod 101`.
3. For each next letter: subtract the leaving letter's `code · power`, keeping `h` in `0..100`.
4. Then `h = (h · 26 + code) mod 101` with the arriving letter's code, and output `h`.

## Cost

| Operation | Cost |
|---|---|
| First window | `O(length)` |
| Each next window | `O(1)` |
| All windows of an `n`-letter text | `O(n)` |
| Extra space | `O(1)` besides the output |

## Pitfalls

- **Negative remainders.** After subtracting, `h` can drop below 0. Python's `%` returns a non-negative result, but JavaScript's does not; the TypeScript sample adds `MOD` before taking `%` again.
- **Trusting equal hashes.** A collision makes two different windows look equal. Check the letters when hashes match.
- **Using the wrong power.** The leaving letter's weight is `base^(length−1)`, not `base^length`.

## When to use it

Use a rolling hash to compare many overlapping windows quickly: finding a pattern in a text, detecting a repeated substring, spotting duplicate chunks in files.

## Related topics

- [Sliding window](learn:searching/sliding-window): the same "drop one, add one" update.
- [Prefix sums](learn:foundations/prefix-sums): another way to get any range's value in `O(1)`.
- [Rabin–Karp](learn:strings/rabin-karp): substring search with a rolling hash.
