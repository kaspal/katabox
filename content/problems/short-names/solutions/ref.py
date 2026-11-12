class Trie:
    """Nodes are numbers; edges live in one dict keyed by (node, letter)."""

    def __init__(self):
        self.child: dict[tuple[int, str], int] = {}
        self.passes = [0]

    def insert(self, word: str) -> None:
        node = 0
        for c in word:
            nxt = self.child.get((node, c))
            if nxt is None:
                nxt = len(self.passes)
                self.passes.append(0)
                self.child[(node, c)] = nxt
            node = nxt
            self.passes[node] += 1

    def label(self, word: str) -> str:
        node = 0
        for k, c in enumerate(word):
            node = self.child[(node, c)]
            if self.passes[node] == 1:
                return word[: k + 1]
        return word


def short_names(names: list[str]) -> list[str]:
    trie = Trie()
    for name in names:
        trie.insert(name)
    return [trie.label(name) for name in names]
