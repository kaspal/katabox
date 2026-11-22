function rev(s: string): string {
  if (s.length <= 1) {
    return s;
  }
  return rev(s.slice(1)) + s[0];
}

function total(xs: number[]): number {
  if (xs.length === 0) {
    return 0;
  }
  return xs[0] + total(xs.slice(1));
}

console.log(rev("abc"));
console.log(total([4, 5, 6]));
