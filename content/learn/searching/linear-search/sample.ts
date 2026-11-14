function linearSearch(a: number[], target: number): number {
  for (let i = 0; i < a.length; i++) {
    if (a[i] === target) { // @step check
      return i; // @step found
    }
  }
  return -1; // @step missing
}
