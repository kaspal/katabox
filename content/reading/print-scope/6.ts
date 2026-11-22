function outer(): () => string {
  let msg = "hi";
  const say = () => msg;
  msg = "bye";
  return say;
}

console.log(outer()());
