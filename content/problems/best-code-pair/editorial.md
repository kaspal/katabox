## Approach: a binary trie, greedy from the top bit

Write each code with 20 bits, highest first, and insert it into a trie whose nodes have a child for bit 0 and one for bit 1.

To find the best partner of `x`, start at the root and look at the bits of `x` from the top. At each level, prefer the child with the opposite bit: if it exists, that bit of the XOR is 1, and since a higher bit outweighs all lower bits together, taking it is always right. Otherwise follow the same bit (XOR bit 0).

Insert all codes, then query each one (or query each code against the codes inserted before it): the answer is the largest result.

**Prefix-set variant:** the same greedy, done with sets: build the answer bit by bit from the top; for each bit, collect the prefixes of all codes and check whether two of them XOR to the candidate answer with this bit set.

**Complexity:** O(n·B) with B = 20 bits, O(n·B) space for the trie. Trying every pair is O(n²): 5·10⁹ pairs for 10⁵ codes.
