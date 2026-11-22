If you already know how to merge the lines in order, the repeated numbers become easy to
spot. Where do equal ticket numbers end up in a sorted merge?
---
In the merged order, all copies of a number come out one right after another. You only
need to remember the last number you added to the result.
---
Merge with a min-heap holding the current front of each line (value, line index, node).
Pop the smallest, push its successor, and append its value only if it differs from the
last value appended.
