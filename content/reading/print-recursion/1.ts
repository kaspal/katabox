function down(n: number): void {
  if (n === 0) {
    return;
  }
  console.log(n);
  down(n - 1);
}

down(3);
