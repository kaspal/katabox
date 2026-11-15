Collecting each stretch's species in a set costs up to `n` steps per question, and the stretches can be long. You do not have to answer the questions in the order they are asked: what if you sorted them by their right end `r`?
---
Read the log from left to right. Once you have reached photo `r`, put a mark only on the **latest** photo of each species seen so far. Every species in `l..r` then has exactly one mark inside `l..r`, and no other species does, so the answer is the number of marks in `l..r`.
---
When photo `r` shows a species last seen at photo `p`, remove the mark at `p` and add one at `r`. Keep the marks in a Fenwick tree (binary indexed tree) so that each change and each count of marks in `1..k` costs O(log n); the answer is `count(r) − count(l − 1)`. Use a hash map from species id to its latest photo (ids go up to 10⁹), and store each answer at its question's original position.
