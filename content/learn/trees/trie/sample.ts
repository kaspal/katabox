class TrieNode {
  children = new Map<string, TrieNode>();
  words = 0;
  end = false;
}

function insert(root: TrieNode, word: string): void {
  let node = root;
  for (const c of word) {
    if (!node.children.has(c)) {
      node.children.set(c, new TrieNode()); // @step new-node
    }
    node = node.children.get(c)!; // @step follow
    node.words++;
  }
  node.end = true; // @step mark-end
}

function countPrefix(root: TrieNode, prefix: string): number[] {
  let node = root;
  for (const c of prefix) {
    if (!node.children.has(c)) {
      return [0, 0]; // @step missing
    }
    node = node.children.get(c)!; // @step walk
  }
  return [node.words, node.end ? 1 : 0]; // @step count
}

function run(words: string[]): number[] {
  const root = new TrieNode();
  for (const word of words.slice(0, -1)) insert(root, word);
  return countPrefix(root, words[words.length - 1]);
}
