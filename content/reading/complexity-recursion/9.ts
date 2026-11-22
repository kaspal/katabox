function f(n: number): number[] {
  const nums: number[] = [];
  for (let i = 0; i < n; i++) {
    nums.push((i * 7919) % n);
  }
  return mergeSort(nums);
}

function mergeSort(a: number[]): number[] {
  if (a.length <= 1) {
    return a;
  }
  const mid = Math.floor(a.length / 2);
  const left = mergeSort(a.slice(0, mid));
  const right = mergeSort(a.slice(mid));
  const merged: number[] = [];
  let i = 0;
  let j = 0;
  while (i < left.length && j < right.length) {
    if (left[i] <= right[j]) {
      merged.push(left[i++]);
    } else {
      merged.push(right[j++]);
    }
  }
  while (i < left.length) merged.push(left[i++]);
  while (j < right.length) merged.push(right[j++]);
  return merged;
}
