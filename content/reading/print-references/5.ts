function addItem(xs: string[]): void {
  xs.push("x");
}

function replace(xs: string[]): void {
  xs = ["new"];
  xs.push("y");
}

const items = ["a"];
addItem(items);
replace(items);
console.log(items.join(" "));
console.log(items.length);
