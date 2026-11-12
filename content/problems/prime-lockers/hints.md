Testing every number up to `L` for primality, separately for each question, repeats the same work thousands of times. Can you decide once which numbers up to the largest limit are prime?
---
The sieve of Eratosthenes: start with every number marked prime, then for each prime `p` cross out `p², p² + p, p² + 2p, …`. Starting at `p²` is enough: smaller multiples have a smaller prime factor and are already crossed out.
---
After the sieve, a running count `count[x]` = number of primes ≤ `x` answers each question in O(1).
