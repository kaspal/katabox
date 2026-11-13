Trying every pair is O(n²). XOR is decided bit by bit, and a high bit is worth more than all lower bits together. Can you choose bits greedily from the top?
---
For one code `x`, its best partner differs from it in the highest bit possible, then the next, and so on. Store the codes in a binary trie (one level per bit, highest first) to follow that choice.
---
Insert each code into the trie; to query `x`, walk from the top bit and go to the child with the **opposite** bit whenever it exists (that bit of the XOR becomes 1). Take the best query over all codes.
