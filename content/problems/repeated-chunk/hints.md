If some piece of length L appears twice, what about pieces of length L − 1?
---
They appear twice as well (drop the last letter of both copies). So "is there a repeated piece of length L?" switches from yes to no at the answer: binary search on L.
---
To check one L in O(n), give every piece of length L a number with a **rolling hash**: from the hash of `text[i..i+L)` you get the hash of `text[i+1..i+L+1)` in O(1). Two equal hashes (with two independent moduli, so that false matches are practically impossible) mean a repeat.
