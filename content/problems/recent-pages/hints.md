Keeping the pages in a list ordered by last use works, but finding and moving a page costs O(capacity). Which two things must be fast?
---
Finding a page by number (a hash map), and moving a page to the "most recent" end or removing the "least recent" one (a doubly linked list does both in O(1)).
---
Map each page to its node in a doubly linked list ordered by use. "get" and "put" move the node to the front; evicting removes the node at the back and its map entry. Many languages have an ordered map that does this for you (insertion-ordered maps where you delete and re-insert to refresh).
