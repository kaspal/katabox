function makePair(): [() => void, () => number] {
  let n = 0;
  const inc = () => {
    n += 10;
  };
  const get = () => n;
  return [inc, get];
}

const [inc, get] = makePair();
inc();
inc();
console.log(get());
