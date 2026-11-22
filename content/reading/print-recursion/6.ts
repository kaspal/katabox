function spell(n: number, acc: string): string {
  if (n === 0) {
    return acc;
  }
  return spell(Math.floor(n / 10), acc + String(n % 10));
}

console.log(spell(305, ""));
console.log(spell(40, ""));
