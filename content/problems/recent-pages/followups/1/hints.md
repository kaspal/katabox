Scanning every cached page to find the one to remove costs O(capacity) per eviction, too
slow for 2·10⁵ operations on a big cache. What would let you find the victim right away?
---
Group the pages by their number of uses. Inside one group, the base problem's trick (an
order by last use) tells you which page is oldest.
---
Keep, for each use count, the pages with that count in order of last use, plus the
smallest count that has any page. A use moves a page from group `f` to the end of group
`f + 1` (if group `f` empties and was the smallest, the smallest becomes `f + 1`); a new
page joins group 1, which then becomes the smallest; evict the oldest page of the
smallest group.
