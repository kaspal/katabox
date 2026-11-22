function up(n: number): void {
  if (n === 0) {
    return;
  }
  up(n - 1);
  console.log(n);
}

up(3);
