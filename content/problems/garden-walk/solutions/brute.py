# Provided by KataBox: the judge builds the input trees with this class.
class TreeNode:
    def __init__(self, val: int = 0, left: TreeNode | None = None, right: TreeNode | None = None):
        self.val = val
        self.left = left
        self.right = right


from collections import deque


def longest_walk(root: TreeNode | None) -> int:
    # Obviously correct: build the graph and run a BFS from every node.
    adj, todo = {}, [root] if root is not None else []
    while todo:
        node = todo.pop()
        adj.setdefault(id(node), [])
        for child in (node.left, node.right):
            if child is not None:
                adj[id(node)].append(id(child))
                adj.setdefault(id(child), []).append(id(node))
                todo.append(child)
    best = 0
    for start in adj:
        dist = {start: 0}
        q = deque([start])
        while q:
            x = q.popleft()
            for y in adj[x]:
                if y not in dist:
                    dist[y] = dist[x] + 1
                    q.append(y)
        best = max(best, max(dist.values()))
    return best
