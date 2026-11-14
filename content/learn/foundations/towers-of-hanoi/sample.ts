function hanoi(n: number, source: number, target: number, spare: number, moves: number[][]): void {
  if (n === 0) return; // @step check
  hanoi(n - 1, source, spare, target, moves);
  moves.push([source, target]); // @step move
  hanoi(n - 1, spare, target, source, moves);
}

function solve(n: number): number[][] {
  const moves: number[][] = [];
  hanoi(n, 1, 3, 2, moves);
  return moves;
}
