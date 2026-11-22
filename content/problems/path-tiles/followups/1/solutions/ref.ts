const MOD = 1000003;

function mul(a: number[][], b: number[][]): number[][] {
  const c = [
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
  ];
  // Each product is below 10^12 and a sum of three below 2^53: exact in a number.
  for (let i = 0; i < 3; i++)
    for (let j = 0; j < 3; j++) c[i][j] = (a[i][0] * b[0][j] + a[i][1] * b[1][j] + a[i][2] * b[2][j]) % MOD;
  return c;
}

export function tilingsLong(n: number): number {
  let step = [
    [1, 1, 1],
    [1, 0, 0],
    [0, 1, 0],
  ];
  let power = [
    [1, 0, 0],
    [0, 1, 0],
    [0, 0, 1],
  ];
  let e = n;
  while (e > 0) {
    if (e % 2 === 1) power = mul(power, step);
    step = mul(step, step);
    e = Math.floor(e / 2);
  }
  return power[0][0];
}
