The base problem's table over amounts still works, with "count" in place of "fewest". But
if every amount tries every token as its last one, `1 + 2` and `2 + 1` are counted twice.
How can you make each combination appear in only one order?
---
Decide the tokens one value at a time: first how many of the first value, then how many
of the second, and so on. Then every combination is built in exactly one way.
---
Keep `ways[a]`, starting with `ways[0] = 1`. Loop over the token values on the outside;
for each value `c`, go through the amounts from `c` up to `amount` and add `ways[a − c]`
to `ways[a]`, modulo 1 000 000 007.
