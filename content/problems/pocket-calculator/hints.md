Reading left to right, you cannot apply `+` as soon as you see it: a later `*` may bind tighter. Something has to wait, which is what a stack is for.
---
Describe the expression with three rules: an *expression* is terms joined by `+`/`-`; a *term* is factors joined by `*`; a *factor* is a number, a `-` followed by a factor, or an expression in parentheses. Each rule becomes a function that calls the others (recursive descent; the call stack does the waiting).
---
Keep a position in the string and skip spaces before reading each symbol. `term()` reads a factor and keeps multiplying while it sees `*`; `expr()` reads a term and keeps adding or subtracting while it sees `+` or `-`, which makes same-level operators apply from left to right. `factor()` handles the number, the unary minus and `(` expr `)`.
