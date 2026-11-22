Sorting every window costs O(n · w log w), far too slow when both are large. The windows
overlap almost completely: from one to the next, one game leaves and one game arrives.
---
A heap struggles here because old games must leave from the middle. Look at the range of
the scores instead: there are only 301 possible values.
---
Keep a count of how many games in the window have each score. Slide the window by adding
the new game and removing the old one, then walk the values from 300 down, summing the
counts until you reach `k`. A Fenwick tree over the values makes that walk logarithmic.
