export function trappedRain(columns: number[]): number {
  let i = 0;
  let j = columns.length - 1;
  let leftMax = 0;
  let rightMax = 0;
  let water = 0;
  while (i <= j) {
    leftMax = Math.max(leftMax, columns[i]);
    rightMax = Math.max(rightMax, columns[j]);
    if (leftMax <= rightMax) {
      water += leftMax - columns[i];
      i++;
    } else {
      water += rightMax - columns[j];
      j--;
    }
  }
  return water;
}
