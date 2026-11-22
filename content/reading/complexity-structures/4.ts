function f(n: number): number[] {
  const vals: number[] = [];
  for (let i = 0; i < n; i++) {
    vals.push((i * 37) % 101);
  }
  const answer: number[] = new Array(n).fill(-1);
  const stack: number[] = [];
  for (let i = 0; i < n; i++) {
    while (stack.length > 0 && vals[stack[stack.length - 1]] < vals[i]) {
      answer[stack[stack.length - 1]] = i;
      stack.pop();
    }
    stack.push(i);
  }
  return answer;
}
