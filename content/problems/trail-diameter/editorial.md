## Approach: two breadth-first searches

The longest route between two junctions of a tree is its **diameter**. A breadth-first search (BFS) from one junction gives the number of trails to every other junction in O(N), so running one from every junction and keeping the largest distance is correct, but O(N²) overall.

Two searches are enough:

1. BFS from any junction `s` (say 1); let `u` be a junction at the largest distance.
2. BFS from `u`; the largest distance found is the diameter.

**Why it works.** In a tree, a junction farthest from any start is always an end of some longest route. Sketch: take a longest route `x … y`; the route from `s` to `u` meets it at some junction `p` (or comes closest to it at `p`). Since no junction is farther from `s` than `u`, `u` is at least as far from `p` as `x` and `y` are, so replacing `x` (or `y`) with `u` gives a route at least as long, ending at `u`. The second BFS measures the longest route from `u`, so it finds the diameter.

**Implementation:** read every token at once; for each park build adjacency lists (or a compact array of neighbours with offsets) from the `N − 1` edges. Use an **iterative** BFS with an array as the queue: a park can be a single chain of 20 000 junctions, too deep for recursion in most languages. A park with `N = 1` has no edge lines and answer 0.

**Complexity:** O(N) time and memory per park, O(total N) overall.

**Fast input in each language:** `sys.stdin.buffer.read().split()` (Python), `readFileSync(0, "utf8").split(/\s+/)` with typed arrays (TypeScript), `stream_get_contents(STDIN)` with `preg_split` (PHP), `read_to_string` with `split_ascii_whitespace` (Rust).
