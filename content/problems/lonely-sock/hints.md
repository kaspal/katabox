Looking for each sock's partner works, but it compares every pair of socks. Is there a way to let the pairs cancel each other out?
---
Think about XOR (`^`): what is `x ^ x`? And `x ^ 0`? Does the order of the operations matter?
---
XOR all the numbers together. Each pair contributes `x ^ x = 0`, and XOR is commutative and associative, so only the lonely number is left.
