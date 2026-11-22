The small examples pass. Try values close to 10⁹, for example `nums = [999999999, 999999999]`, and compare with the exact answer.
---
The running product is reduced modulo 10⁹ + 7 at every step, so it stays below about 10⁹. But the multiplication itself happens before the reduction: how large can `acc * x` get, and what kind of number holds it?
---
`acc` is a floating-point number, and a double holds integers exactly only up to 2⁵³ (about 9·10¹⁵). `acc * x` can reach about 10¹⁸, so it gets rounded before `% MOD`. Keep `acc` an integer (`acc = 1`). In TypeScript, `number` has the same limit: do the multiplication with `BigInt` and convert back at the end.
