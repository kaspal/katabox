const isWord = (c: string): boolean => /[A-Za-z0-9]/.test(c);

export function isMirror(text: string): boolean {
  let i = 0;
  let j = text.length - 1;
  while (i < j) {
    if (!isWord(text[i])) i++;
    else if (!isWord(text[j])) j--;
    else if (text[i].toLowerCase() !== text[j].toLowerCase()) return false;
    else {
      i++;
      j--;
    }
  }
  return true;
}
