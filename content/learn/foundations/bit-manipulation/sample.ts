function bitOps(x: number, y: number): number[] {
  const results = [x & y]; // @step and
  results.push(x | y); // @step or
  results.push(x ^ y); // @step xor
  results.push(x << 1); // @step left
  results.push(x >> 1); // @step right
  let count = 0;
  while (x !== 0) {
    x &= x - 1; // @step clear
    count++;
  }
  results.push(count); // @step count
  return results;
}
