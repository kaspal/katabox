const settings = { theme: "dark", size: 12 };
const backup = settings;
const current = { ...settings };
settings.size = 14;
current.theme = "light";
console.log(backup.size);
console.log(settings.theme);
console.log(current.size);
