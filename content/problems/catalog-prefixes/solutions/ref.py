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

    def count(self, prefix: str) -> int:
        node = 0
        for c in prefix:
            node = self.child.get((node, c), -1)
            if node < 0:
                return 0
        return self.passes[node]


def prefix_counts(titles: list[str], typed: list[str]) -> list[int]:
    trie = Trie()
    for t in titles:
        trie.insert(t)
    return [trie.count(p) for p in typed]
