function ping(n: number): string {
  if (n === 0) {
    return "ping";
  }
  return pong(n - 1);
}

function pong(n: number): string {
  if (n === 0) {
    return "pong";
  }
  return ping(n - 1);
}

console.log(ping(3));
console.log(ping(4));
