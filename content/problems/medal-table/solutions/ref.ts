export function medalTable(countries: string[], medals: number[][]): string[] {
  const order = countries.map((_, i) => i);
  order.sort((a, b) => {
    for (let k = 0; k < 3; k++) {
      if (medals[a][k] !== medals[b][k]) return medals[b][k] - medals[a][k];
    }
    return countries[a] < countries[b] ? -1 : countries[a] > countries[b] ? 1 : 0;
  });
  return order.map((i) => countries[i]);
}
