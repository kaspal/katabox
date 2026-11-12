class Trie:
    """Nodes are numbers; edges live in one dict keyed by (node, letter)."""

    def __init__(self):
        self.child: dict[tuple[int, str], int] = {}
        self.best: list[str] = [""]

    def insert(self, word: str) -> None:
        node = 0
        for c in word:
            nxt = self.child.get((node, c))
            if nxt is None:
                nxt = len(self.best)
                self.best.append(word)  # words arrive sorted: the first is the smallest
                self.child[(node, c)] = nxt
            node = nxt

    def first(self, prefix: str) -> str:
        node = 0
        for c in prefix:
            node = self.child.get((node, c), -1)
            if node < 0:
                return ""
        return self.best[node]


def first_matches(words: list[str], prefixes: list[str]) -> list[str]:
    trie = Trie()
    for w in sorted(set(words)):
        trie.insert(w)
    return [trie.first(p) for p in prefixes]
