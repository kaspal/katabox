function f(n: number): number[] {
  const nums: number[] = [];
  for (let v = n; v > 0; v--) {
    nums.push(v);
  }
  for (let i = 1; i < n; i++) {
    const key = nums[i];
    let j = i - 1;
    while (j >= 0 && nums[j] > key) {
      nums[j + 1] = nums[j];
      j--;
    }
    nums[j + 1] = key;
  }
  return nums;
}
