function appendAll(values: number[]): number[] {
  let capacity = 1;
  let data: number[] = new Array<number>(capacity);
  let size = 0;
  let copies = 0;
  for (const v of values) {
    if (size === capacity) { // @step check
      capacity *= 2;
      const bigger = new Array<number>(capacity);
      for (let i = 0; i < size; i++) {
        bigger[i] = data[i]; // @step copy
        copies++;
      }
      data = bigger;
    }
    data[size] = v; // @step write
    size++;
  }
  return [capacity, copies, ...data.slice(0, size)];
}
