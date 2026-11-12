export function evaluate(expression: string): number {
  let pos = 0;
  const peek = (): string => {
    while (pos < expression.length && expression[pos] === " ") pos++;
    return pos < expression.length ? expression[pos] : "";
  };
  const factor = (): number => {
    const c = peek();
    if (c === "-") {
      pos++;
      return -factor();
    }
    if (c === "(") {
      pos++;
      const v = expr();
      peek();
      pos++; // ")"
      return v;
    }
    const start = pos;
    while (pos < expression.length && expression[pos] >= "0" && expression[pos] <= "9") pos++;
    return Number(expression.slice(start, pos));
  };
  const term = (): number => {
    let v = factor();
    while (peek() === "*") {
      pos++;
      v *= factor();
    }
    return v;
  };
  const expr = (): number => {
    let v = term();
    for (let c = peek(); c === "+" || c === "-"; c = peek()) {
      pos++;
      const w = term();
      v = c === "+" ? v + w : v - w;
    }
    return v;
  };
  const value = expr();
  return value === 0 ? 0 : value; // never -0
}
