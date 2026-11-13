export function gradeBands(scores: number[]): string[] {
  return scores.map((s) => {
    if (s >= 90) return "A";
    if (s >= 80) return "B";
    if (s >= 70) return "C";
    if (s >= 60) return "D";
    return "F";
  });
}
