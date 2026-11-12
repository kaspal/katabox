Think of every frozen cell as a node, joined to the frozen cells right of it and below it. A pond is a group of connected nodes.
---
A disjoint-set union (union-find) keeps one representative per group: start with every cell alone, then join each frozen cell with its frozen right and bottom neighbours.
---
Number cell `(r, c)` as `r × width + c`. With path compression and union by size, each join costs almost O(1). At the end, count the cells per representative and sort the counts from largest to smallest.
