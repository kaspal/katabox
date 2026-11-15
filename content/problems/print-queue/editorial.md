## Approach: event simulation with a heap of waiting jobs

**Sort the arrivals first.** The jobs come in input order, not arrival order, so build `(arrival, priority, pages, number)` for each job and sort by arrival (then number). With a pointer `next` into that list, the jobs that have arrived by time `t` are exactly those before `next` once you have advanced it past every `arrival ≤ t`.

**The waiting room is a heap.** The printer always wants the smallest key

```
(−priority, arrival, number)
```

among the waiting jobs, which is exactly what a min-heap gives. The simulation:

```
t = 0
while jobs remain:
    if heap is empty and jobs[next].arrival > t: t = jobs[next].arrival   # idle
    while next < n and jobs[next].arrival ≤ t: push jobs[next]; next += 1
    pop the smallest key; print its number; t += its pages
```

Example 1: at `t = 0` only job 1 is in the heap; it prints until `t = 3`. Jobs 2, 3 and 4 (arrivals 1, 1, 2) are pushed. The smallest key is job 3's `(−9, 1, 3)`; `t = 4`. Then `(−5, 1, 2)` beats `(−5, 2, 4)`, so job 2 prints, then job 4: `1 3 2 4`.

**Why it is correct:** the printer only chooses at the moments it becomes free, and the loop runs exactly at those moments: right after a job ends, or, when nothing is waiting, at the next arrival. At each such `t`, the heap holds precisely the jobs with `arrival ≤ t` that have not been printed (all of them are pushed before the pop, and printed ones have been popped). Popping the smallest key applies the rule in full: highest priority first, then the earliest arrival, then the smallest number. Jobs never interrupt one another, so nothing happens between choices.

**Complexity:** O(n log n) for the sort, and each job is pushed and popped once at O(log n): O(n log n) in all.

**Why scanning is too slow:** keeping the waiting jobs in a plain list and scanning it for the best one costs O(waiting) per pick. In a busy office (three jobs arriving each second, about five pages each), the queue grows to tens of thousands of jobs; over 5·10⁴ picks that is around 10⁹ comparisons, far past the time limit, while the heap needs about 5·10⁴ · 2 · 16 ≈ 1.6·10⁶ steps.

**Practical notes:**

- The clock can reach 10⁹ + 5·10⁴ · 10⁴ = 1.5·10⁹: use 64-bit integers in Rust (`i64`/`u64`). PHP integers and TypeScript `Number` are fine.
- Python: `heapq` with tuples `(-p, a, i, g)` compares in exactly the printer's order. Rust: `BinaryHeap` is a max-heap, so push `Reverse((-p, a, i, g))` or `(p, Reverse(a), Reverse(i))`. TypeScript has no heap in its standard library: write a small binary heap with a comparator. PHP: `SplPriorityQueue` does not keep insertion order for equal priorities, so either pass the full key as the priority, `[$p, -$a, -$i]` (it is a max-queue, and an array compares element by element), or write a small heap.
- Read all of stdin at once and print the `n` numbers in one write.
