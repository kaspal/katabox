function canJump(jumps: number[]): number[] {
  let reach = 0;
  for (let i = 0; i < jumps.length; i++) {
    if (i > reach) {
      return [0, reach]; // @step stuck
    }
    if (i + jumps[i] > reach) {
      reach = i + jumps[i]; // @step farther
    }
  }
  return [1, Math.min(reach, jumps.length - 1)];
}
