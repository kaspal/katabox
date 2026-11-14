## The idea

A **graph** is a set of nodes joined by edges. Here the nodes are numbered `0` to `n − 1`, and each edge `u v w` joins `u` to `v` with a weight `w` (an unweighted graph simply uses `w = 1`). In a **directed** graph the edge goes one way only; in an **undirected** one it works both ways, so it is written twice: once from `u` to `v` and once from `v` to `u`.

Before any algorithm can walk a graph, it needs the edges stored in a shape it can query. Two shapes cover almost every case:

- **Adjacency list**: one list per node, holding the neighbours it has edges to, in the order the edges arrive.
- **Adjacency matrix**: an `n × n` grid where cell `[u][v]` holds the weight of the edge from `u` to `v`, and `0` means there is no edge.

The sample builds both in a single pass over the edge list, then writes each node's list (its length, then its neighbours) followed by the matrix row by row.

### The default input

Five nodes, undirected, with edges `0 1 4, 0 2 1, 1 3 2, 2 3 5, 3 4 3`. The lists come out as 0: [1, 2], 1: [0, 3], 2: [0, 3], 3: [1, 2, 4] and 4: [3]. Node 3 appears in three edges, so its list is the longest; node 4 has only one. The five edges fill 10 list entries, two per edge.

The matrix has 25 cells. Ten of them hold a weight, for example `[0][1] = [1][0] = 4` and `[3][4] = [4][3] = 3`, and the other 15 are `0`. The matrix is symmetric because the graph is undirected.

## Steps

1. Make `n` empty lists and an `n × n` matrix full of zeros.
2. For each edge `u v w`, append `v` to `u`'s list and set `matrix[u][v] = w`.
3. If the graph is undirected, also append `u` to `v`'s list and set `matrix[v][u] = w`.
4. When the edges run out, both structures describe the same graph.

## Cost

| Operation | Cost |
|---|---|
| Build the list | `O(n + m)` for `n` nodes and `m` edges |
| Is there an edge `u → v`? | `O(1)` in the matrix, `O(deg u)` in the list |
| Visit every neighbour of `u` | `O(deg u)` in the list, `O(n)` in the matrix |
| Space | `O(n + m)` for the list, `O(n²)` for the matrix |

## Pitfalls

- **Forgetting the reverse edge.** In an undirected graph, writing only `u → v` makes `v` unable to reach `u`, and traversals silently miss nodes.
- **Using `0` as "no edge" when weights can be `0`.** The matrix cannot tell a free edge from a missing one; use a separate marker such as `None` or infinity in that case.
- **A matrix for a big sparse graph.** A hundred thousand nodes means ten billion cells, even if there are only a few edges.

## When to use it

Pick the adjacency list by default: most graph algorithms visit every neighbour of a node, and the list does that in time proportional to the edges that exist. Pick the matrix when the graph is small or dense, or when you constantly ask "is `u` joined to `v`?", as in [Floyd-Warshall](learn:graphs/floyd-warshall).

## Related topics

- [Hash table: separate chaining](learn:hashing/separate-chaining): another array of lists.
- [Breadth-first search](learn:graphs/bfs): the first walk over an adjacency list.
- [Depth-first search](learn:graphs/dfs): the other basic walk.
- [Floyd-Warshall](learn:graphs/floyd-warshall): all-pairs shortest paths on a matrix.
