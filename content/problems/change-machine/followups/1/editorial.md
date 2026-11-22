## Idea

Counting orderings would be easy, but here `1 + 2` and `2 + 1` are one combination. Fix
an order for the token values and build each combination in that order: `ways[a]` is the
number of ways to make `a` with the values processed so far, starting from
`ways[0] = 1`. Adding value `c` lets any amount `a ≥ c` end with one more `c`, so going
upwards through the amounts, `ways[a] += ways[a − c]`. Going upwards on purpose reuses the
already updated `ways[a − c]`, which is what allows several tokens of the same value.

The loop order matters: values outside and amounts inside counts combinations; amounts
outside and values inside would count ordered sequences instead.

## Complexity

O(coins.length · amount) time, at most 30 · 2·10⁴ = 6·10⁵ steps, and O(amount) extra
memory.
