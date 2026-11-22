export function fewestInsertions(text: string): number {
  let open = 0;
  let added = 0;
  for (const c of text) {
    if (c === "(") open++;
    else if (c === ")") {
      if (open > 0) open--;
      else added++;
    }
  }
  return added + open;
}
