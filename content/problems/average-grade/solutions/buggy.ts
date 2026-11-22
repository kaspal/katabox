export function averageGrade(grades: number[]): number {
  let total = 0;
  for (const g of grades) total += g;
  return Math.floor(total / grades.length);
}
