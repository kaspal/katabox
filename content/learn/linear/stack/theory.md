## The idea

A stack is a pile where you only ever touch the top. **push** puts a value on top, **pop** takes the top value off and returns it, and **peek** reads the top without removing it. The last value in is always the first one out (LIFO), like plates stacked in a cupboard.

That one rule is what makes stacks useful. Whenever a task opens something and must later close it in reverse order (brackets, nested function calls, folders entered and left, edits to undo), the thing to close next is always the most recent one still open: the top of the stack.

### Built on a dynamic array

The sample keeps the stack in a dynamic array whose **end** is the top. Pushing appends, popping removes the last value, and peeking reads it; none of them shifts anything. The bottom of the stack is position 0 and never moves.

pop and peek on an empty stack have no value to give. Here they return `−1`.

## Steps

1. Start with an empty array `stack` and an empty output row.
2. push `x`: append `x` to the end of `stack`.
3. pop: if `stack` is empty, output `−1`; otherwise remove the last value and output it.
4. peek: if `stack` is empty, output `−1`; otherwise output the last value and leave it in place.
5. After all the operations, the output row holds every value that pop and peek returned, in order.

## Cost

| Operation | Cost |
|---|---|
| push | `O(1)` amortised (the array may double) |
| pop | `O(1)` |
| peek | `O(1)` |
| Extra space | `O(n)` for `n` values on the stack |

## Pitfalls

- **Popping an empty stack.** In Python `[].pop()` raises an error and in JavaScript it returns `undefined`. Check for empty before every pop and peek.
- **Using the front as the top.** Pushing and popping at position 0 of an array shifts every value, so each operation costs `O(n)`. Keep the top at the end.
- **Confusing pop with peek.** peek leaves the value on the stack; calling pop "just to look" loses it.

## When to use it

Use a stack whenever the most recent unfinished item is the next one to deal with: matching brackets, evaluating expressions, undo history, walking a folder path with `..`, and depth-first search done without recursion. A monotonic stack, which pops values that can no longer matter, answers "next greater value" questions in one pass.

## Related topics

- [Recursion and the call stack](learn:foundations/recursion-call-stack): the stack every function call already uses.
- [Monotonic stack](learn:searching/monotonic-stack): a stack that keeps its values in order.
- [Amortised cost: the dynamic array](learn:foundations/dynamic-array): why push stays `O(1)` on average.
- [Queue and circular buffer](learn:linear/queue-ring-buffer): the first-in, first-out counterpart.
- Depth-first search (coming soon): explores a graph with a stack.
