let count = 10;

function reset(): void {
  let count = 0;
}

function bump(): void {
  count += 1;
}

reset();
bump();
console.log(count);
