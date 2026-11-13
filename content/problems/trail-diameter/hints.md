One breadth-first search from a junction gives the distance (in trails) to every other junction. Running it from every junction finds the answer, but costs O(N²): 4·10⁸ steps for 20 000 junctions.
---
Start a BFS anywhere and take the junction `u` farthest from the start. In a tree, `u` is always one end of some longest route.
---
Run a second BFS from `u`: the largest distance it finds is the answer. Build the adjacency lists once and use a queue (an array with a head index), not recursion, so a long chain of 20 000 junctions is no problem.
