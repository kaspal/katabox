def hanoi(n, source, target, spare, moves):
    if n == 0:  # @step check
        return
    hanoi(n - 1, source, spare, target, moves)
    moves.append((source, target))  # @step move
    hanoi(n - 1, spare, target, source, moves)


def solve(n):
    moves = []
    hanoi(n, 1, 3, 2, moves)
    return moves
