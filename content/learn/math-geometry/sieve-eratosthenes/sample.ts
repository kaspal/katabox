function sieve(n: number): number[] {
  const isPrime = new Array<boolean>(n + 1).fill(true);
  isPrime[0] = false;
  isPrime[1] = false;
  for (let p = 2; p * p <= n; p++) {
    if (isPrime[p]) {
      const start = p * p; // @step prime
      for (let multiple = start; multiple <= n; multiple += p) {
        isPrime[multiple] = false; // @step cross
      }
    }
  }
  const primes: number[] = [];
  for (let k = 0; k <= n; k++) if (isPrime[k]) primes.push(k);
  return [primes.length, ...primes];
}
