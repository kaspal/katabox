function gcdSteps(a: number, b: number): number[] {
  let steps = 0;
  while (b !== 0) {
    [a, b] = [b, a % b]; // @step divide
    steps++;
  }
  return [a, steps];
}
