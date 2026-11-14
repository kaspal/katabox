WHITE, GREY, BLACK = 0, 1, 2


def has_cycle(n, adj):
    color = [WHITE] * n

    def visit(u):
        color[u] = GREY  # @step grey
        for v, _ in adj[u]:
            if color[v] == GREY:
                return True  # @step back-edge
            if color[v] == WHITE and visit(v):
                return True
        color[u] = BLACK  # @step black
        return False

    for u in range(n):
        if color[u] == WHITE and visit(u):
            return 1
    return 0
