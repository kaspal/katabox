With at most 18 stamps there are at most 2¹⁸ ≈ 262 000 choices, so exploring them is feasible if each step is cheap. Decide for stamp 0, then stamp 1, and so on: take it or leave it.
---
Search recursively with the index of the next stamp and the amount still missing. Try **taking** a stamp before **leaving** it: then lists with the same number of stamps are found in lexicographic order, so keep a new answer only when it uses strictly fewer stamps.
---
Prune: stop when the missing amount is 0 (record the answer), when the remaining stamps cannot add up to it, when the next stamp is worth more than what is missing (skip taking it), and when you already use as many stamps as the best answer.
