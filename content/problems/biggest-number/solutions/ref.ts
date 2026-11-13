export function biggestNumber(cards: number[]): string {
  const texts = cards.map(String);
  texts.sort((a, b) => {
    const ab = a + b;
    const ba = b + a;
    return ab > ba ? -1 : ab < ba ? 1 : 0;
  });
  const joined = texts.join("");
  return joined[0] === "0" ? "0" : joined;
}
