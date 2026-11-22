With many labels, the work you do once per shelf matters less than the work you do per
label. Is there something about the shelf worth finding just once?
---
Find where the original order starts: the position of the smallest number. Comparing
the middle number with the last one tells you which half it is in.
---
Once you know the start `s`, the shelf read from `s` to the end and then from 0 to
`s − 1` is fully sorted. Binary search each label in that sorted view, and turn a found
index `k` back into a shelf position with `(k + s) mod n`.
