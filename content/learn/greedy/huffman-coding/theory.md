## The idea

A message uses a few symbols, some far more often than others. A **fixed-length code** gives every symbol the same number of bits: six symbols need 3 bits each. A **variable-length code** gives short codes to frequent symbols and long ones to rare symbols, so the whole message gets shorter.

Variable lengths raise a question: where does one code end and the next begin? The answer is a **prefix-free code**, where no code is the start of another. Put the codes on a binary tree, with 0 on one edge and 1 on the other, and the symbols only at the leaves; each code is the path from the root to its leaf, and reading bits from the root always stops at exactly one symbol. A symbol's code length is its leaf's **depth**.

**Huffman's algorithm** builds the tree from the bottom. Every symbol starts as a one-leaf tree weighted by its frequency. Repeatedly, take the **two lightest trees** and join them under a new parent whose weight is their sum. When one tree is left, it is the code. Ties go to the **older** tree: original symbols first, in input order, then merged trees in the order they were made. The output is the total number of bits for the message, then each symbol's code length. The app labels the symbols a, b, c… in input order.

Why the two lightest? The two rarest symbols can always sit together at the deepest level of some best tree: if a more frequent symbol were deeper, swapping it with a rarer one would not add bits. Joining them leaves the same problem with one symbol fewer, whose weight is their sum.

### The default input

Frequencies `4, 7, 11, 15, 18, 40` for a to f. Join a and b into 11. Two trees weigh 11 now; c is older, so c and the new tree join into 22. Then d and e make 33, then 22 and 33 make 55, and finally f and 55 make the root, 95.

The output is `[216, 4, 4, 3, 3, 3, 1]`: f, the most frequent, gets a 1-bit code; a and b get 4 bits. A fixed 3-bit code would use `95 · 3 = 285` bits. The total also equals the sum of every merge weight: `11 + 22 + 33 + 55 + 95 = 216`, because each merge adds one bit to every symbol beneath it.

## Steps

1. Put every symbol in a priority queue as `(frequency, id)`, ids 0 to `n - 1`.
2. Pop the two smallest, give them a new parent with the next id and their summed weight, and push it.
3. Repeat until one tree is left.
4. Each symbol's code length is the number of parent links up to the root.
5. Return the sum of `frequency · length`, then the lengths.

## Cost

| Case | Cost |
|---|---|
| Every case | `O(n log n)`: `n - 1` merges, each a few heap operations |
| Extra space | `O(n)` for the heap and the parent links |

## Pitfalls

- **Merging the heaviest trees.** That pushes the frequent symbols deep, the opposite of the goal.
- **Ties.** Different tie rules can give different trees and different code lengths, but the total bits are always the same. Sender and receiver must build the same tree, so fix the rule.
- **One symbol.** A single symbol would get a 0-bit code; real encoders give it 1 bit.

## When to use it

Huffman codes sit inside file compressors and image formats, usually after another step has made some symbols very common. The same "join the two smallest" rule also finds the cheapest order to merge many sorted files.

## Related topics

- [Binary heap and priority queue](learn:trees/binary-heap): the queue that hands out the two lightest trees.
- [Kruskal's minimum spanning tree](learn:graphs/kruskal): another greedy that joins pieces, cheapest first.
- [Fractional knapsack](learn:greedy/fractional-knapsack): a greedy rule with a swap argument behind it.
