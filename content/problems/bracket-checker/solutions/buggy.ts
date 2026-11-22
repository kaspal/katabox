export function bracketsOk(code: string): boolean {
  const pairs: Record<string, string> = { ")": "(", "]": "[", "}": "{" };
  const stack: string[] = [];
  for (const c of code) {
    if (c === "(" || c === "[" || c === "{") stack.push(c);
    else if (stack.pop() !== pairs[c]) return false;
  }
  return true;
}
