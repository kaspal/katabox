The groups are the strongly connected components of the directed graph. Checking reachability from every user separately is O(n·(n + m)).
---
Kosaraju's idea: a DFS on the graph gives the users an order by finishing time. Then a DFS on the **reversed** graph, taking start users in decreasing finishing time, collects exactly one group per start.
---
Do both passes iteratively (with explicit stacks): 10⁵ users in a chain would overflow a recursive DFS. Count the sizes, then sort them.
