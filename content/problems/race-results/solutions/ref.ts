export function raceOrder(names: string[], times: number[]): string[] {
  const order = names.map((_, i) => i);
  order.sort((a, b) => times[a] - times[b] || (names[a] < names[b] ? -1 : names[a] > names[b] ? 1 : 0));
  return order.map((i) => names[i]);
}
