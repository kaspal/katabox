export function isBalanced(text: string): boolean {
  const match: Record<string, string> = { ")": "(", "]": "[", "}": "{" };
  const stack: string[] = [];
  for (const ch of text) {
    if (ch === "(" || ch === "[" || ch === "{") {
      stack.push(ch);
    } else if (ch in match) {
      if (stack.length === 0 || stack.pop() !== match[ch]) return false;
    }
  }
  return stack.length === 0;
}
