function pairWithSum(a: number[], target: number): number[] {
  let i = 0;
  let j = a.length - 1;
  while (i < j) {
    const total = a[i] + a[j]; // @step sum
    if (total === target) {
      return [i, j]; // @step found
    }
    if (total < target) { // @step move
      i++;
    } else {
      j--;
    }
  }
  return [-1, -1]; // @step missing
}
