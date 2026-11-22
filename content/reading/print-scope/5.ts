function add(item: string, bag: string[] = []): string[] {
  bag.push(item);
  return bag;
}

console.log(add("a").length);
console.log(add("b").length);
console.log(add("c", []).length);
console.log(add("d").length);
