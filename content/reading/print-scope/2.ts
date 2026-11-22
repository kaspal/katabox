const x = "outer";

function show(): void {
  const x = "inner";
  console.log(x);
}

show();
console.log(x);
