const text = "the quick brown fox";
const words = text.split(" ");
console.log(words[1].split("").reverse().join(""));
console.log(words.map((w) => w[0]).join(" "));
