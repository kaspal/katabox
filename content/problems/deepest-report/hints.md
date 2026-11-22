The answers on small charts are right. Think about the tallest chart the constraints allow: 5·10⁴ people in a single line, where person `i` reports to person `i + 1` and the last person is the boss.
---
The memo in `depth` is fine and the levels it computes are correct. The trouble is how `level` reaches the boss: how many calls are waiting on the stack when it starts from person 0 in that chart?
---
`level` recurses once per manager in the chain, so a chain of 5·10⁴ people needs 5·10⁴ nested calls and overflows the call stack (raising the recursion limit is not a reliable fix). Walk up iteratively instead: follow `manager` from `v` until you reach the boss or a person whose depth is already known, keeping the path in a list, then go back down the path assigning depths.
