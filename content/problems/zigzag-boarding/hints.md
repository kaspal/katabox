Fetching the last node again and again walks the whole list each time: O(n²). The back half is needed in reverse order. Can you prepare it once?
---
Find the middle with a slow and a fast pointer, cut the list there, and reverse the second half.
---
Now interleave: take one node from the front half, one from the reversed back half, and so on. The front half has the same number of nodes or one more.
