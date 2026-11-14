class TrieNode:
    def __init__(self):
        self.children = {}
        self.words = 0
        self.end = False


def insert(root, word):
    node = root
    for c in word:
        if c not in node.children:
            node.children[c] = TrieNode()  # @step new-node
        node = node.children[c]  # @step follow
        node.words += 1
    node.end = True  # @step mark-end


def count_prefix(root, prefix):
    node = root
    for c in prefix:
        if c not in node.children:
            return [0, 0]  # @step missing
        node = node.children[c]  # @step walk
    return [node.words, 1 if node.end else 0]  # @step count


def run(words):
    root = TrieNode()
    for word in words[:-1]:
        insert(root, word)
    return count_prefix(root, words[-1])
