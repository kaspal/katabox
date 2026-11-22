let found = 0;
for (let i = 0; i < 3; i++) {
  for (let j = 0; j < 3; j++) {
    if (j === i) {
      break;
    }
    found++;
  }
}
console.log(found);
