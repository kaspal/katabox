## Approach: recursive descent (the call stack is the stack)

Write the grammar with one rule per precedence level:

```
expr   := term   (("+" | "-") term)*
term   := factor ("*" factor)*
factor := "-" factor | number | "(" expr ")"
```

Each rule becomes a function that reads from a shared position in the string, skipping spaces:

```
expr():   v = term();   while next is + or -: op = read; w = term(); v = v + w or v − w
term():   v = factor(); while next is *: read; v = v * factor()
factor(): if next is "-": read; return −factor()
          if next is "(": read; v = expr(); read ")"; return v
          read the digits; return the number
```

**Why it is right.** `term` finishes every multiplication before `expr` sees the next `+` or `-`, so `*` binds tighter. The `while` loops fold from the left, so `10 − 2 − 3` is `(10 − 2) − 3`. A unary minus is a factor, so `2 * -3` and `-(…)` parse naturally, and `--1` is `−(−1)`.

The call stack waits for pending operators instead of an explicit operator stack (shunting-yard works too). With nesting at most 100 deep, the recursion stays small.

### Walkthrough

`-(7 - 10) * -2`: `expr` → `term` → `factor` sees `-`, so it negates the next factor: `(7 − 10)` = −3, giving 3. Back in `term`, `*` reads `factor` = −2. The result is −6.

**Complexity:** O(n) time, every character read once, and O(depth) stack.
