const team = { name: "A", members: ["ana", "luis"] };
const clone = { ...team };
clone.name = "B";
clone.members.push("eva");
console.log(team.name);
console.log(team.members.length);
