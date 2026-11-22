export function longestBalancedPart(text: string): number {
  const pairs: Record<string, string> = { ")": "(", "]": "[", "}": "{" };
  let stack = [-1];
  let best = 0;
  for (let i = 0; i < text.length; i++) {
    const c = text[i];
    if (c === "(" || c === "[" || c === "{") stack.push(i);
    else if (stack.length > 1 && text[stack[stack.length - 1]] === pairs[c]) {
      stack.pop();
      best = Math.max(best, i - stack[stack.length - 1]);
    } else stack = [i];
  }
  return best;
}
