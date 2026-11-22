In the base problem the count for length `i` only needed the counts for shorter lengths.
Now, before laying a long slab, you must know what the paving so far ends with.
---
Split the count for each length in two: pavings that end in a long slab and pavings that
do not (count the empty paving among the second kind).
---
A paving of length `i` ending in a long slab is a paving of length `i − 3` that does not
end in one, plus that slab. A paving not ending in a long slab is any paving of length
`i − 1` plus a square, or any paving of length `i − 2` plus a short slab. The answer is
the sum of both kinds for length `n`.
